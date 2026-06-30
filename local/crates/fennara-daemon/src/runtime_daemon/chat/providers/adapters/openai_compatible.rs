use futures_util::StreamExt;
use reqwest::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use serde_json::{Map, Value, json};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::runtime_daemon::chat::{ids::new_id, trace};

use super::super::super::trace::TraceRecorder;
use super::super::error::LlmError;
use super::super::request::LlmRequest;
use super::super::sse::{data_lines, parse_sse_payloads};
use super::super::stream::{FinishReason, StreamEvent, Usage};
use super::super::types::{AdapterKind, ChatCompletion, MalformedToolCall, ToolCallObservation};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const MAX_PRE_STREAM_RETRIES: usize = 2;
const PRE_STREAM_RETRY_DELAY: Duration = Duration::from_millis(500);

pub(crate) async fn stream_chat<F, Fut>(
    request: &LlmRequest,
    trace: Option<TraceRecorder>,
    mut on_event: F,
) -> Result<ChatCompletion, LlmError>
where
    F: FnMut(StreamEvent) -> Fut + Send,
    Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
{
    if request.model.provider.adapter != AdapterKind::OpenAiCompatibleChat {
        return Err(LlmError::ProviderInit {
            provider: request.model.provider.id.to_string(),
            message: format!(
                "{} does not use the OpenAI-compatible chat adapter.",
                request.model.provider.name
            ),
        });
    }

    let provider_id = request.model.provider.id.to_string();
    let base_url = request
        .model
        .provider
        .base_url
        .as_deref()
        .ok_or_else(|| LlmError::Config {
            message: format!("{} is missing a base URL.", request.model.provider.name),
        })?
        .trim_end_matches('/')
        .to_string();
    let client = Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|error| LlmError::ProviderInit {
            provider: provider_id.clone(),
            message: format!("Failed to create HTTP client: {error}"),
        })?;

    let request_url = format!("{base_url}/chat/completions");
    let request_headers = headers(request)?;
    let request_body = body(request);
    let mut attempt = 0usize;
    let (response, stream_trace, request_started_at) = loop {
        let provider_attempt_id = new_id("provider_attempt");
        let attempt_trace = trace
            .as_ref()
            .map(|trace| trace.with_provider_attempt(provider_attempt_id.clone()));
        let request_started_at = Instant::now();
        if let Some(trace) = &attempt_trace {
            trace.event(
                "provider.request.start",
                json!({
                    "attempt": attempt + 1,
                    "provider_id": provider_id.as_str(),
                    "provider_name": request.model.provider.name.as_str(),
                    "model_id": request.model.model.id.as_str(),
                    "adapter_model_id": request.model.model.adapter_model_id.as_str(),
                    "url": request_url_summary(&request_url),
                    "message_count": request.messages.len(),
                    "tool_count": request.tools.len(),
                    "body_bytes": trace::value_size(&request_body)
                }),
            );
        }
        let result = client
            .post(&request_url)
            .headers(request_headers.clone())
            .json(&request_body)
            .send()
            .await
            .map_err(|error| {
                LlmError::from_reqwest(
                    &provider_id,
                    &format!("Failed to connect to {}", request.model.provider.name),
                    error,
                )
            });

        match result {
            Ok(response) if response.status().is_success() => {
                if let Some(trace) = &attempt_trace {
                    trace.event_status(
                        "provider.response.headers",
                        "ok",
                        json!({
                            "attempt": attempt + 1,
                            "status": response.status().as_u16(),
                            "provider_request_id": provider_request_id(response.headers()),
                            "duration_ms": request_started_at.elapsed().as_millis() as i64
                        }),
                    );
                }
                break (response, attempt_trace, request_started_at);
            }
            Ok(response) => {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                let error = LlmError::from_http_response(&provider_id, status, &text);
                if attempt >= MAX_PRE_STREAM_RETRIES || !error.is_retryable() {
                    if let Some(trace) = &attempt_trace {
                        trace.error(
                            "provider.request.failed",
                            "failed",
                            json!({
                                "attempt": attempt + 1,
                                "status": status.as_u16(),
                                "retryable": error.is_retryable(),
                                "duration_ms": request_started_at.elapsed().as_millis() as i64,
                                "error_code": error.code()
                            }),
                        );
                    }
                    return Err(error);
                }
                if let Some(trace) = &attempt_trace {
                    trace.warn(
                        "provider.request.retry",
                        "retrying",
                        json!({
                            "attempt": attempt + 1,
                            "status": status.as_u16(),
                            "retryable": true,
                            "sleep_ms": PRE_STREAM_RETRY_DELAY.as_millis() as i64,
                            "error_code": error.code()
                        }),
                    );
                }
            }
            Err(error) => {
                if attempt >= MAX_PRE_STREAM_RETRIES || !error.is_retryable() {
                    if let Some(trace) = &attempt_trace {
                        trace.error(
                            "provider.request.failed",
                            "failed",
                            json!({
                                "attempt": attempt + 1,
                                "retryable": error.is_retryable(),
                                "duration_ms": request_started_at.elapsed().as_millis() as i64,
                                "error_code": error.code()
                            }),
                        );
                    }
                    return Err(error);
                }
                if let Some(trace) = &attempt_trace {
                    trace.warn(
                        "provider.request.retry",
                        "retrying",
                        json!({
                            "attempt": attempt + 1,
                            "retryable": true,
                            "sleep_ms": PRE_STREAM_RETRY_DELAY.as_millis() as i64,
                            "error_code": error.code()
                        }),
                    );
                }
            }
        }

        attempt += 1;
        if !on_event(StreamEvent::Status {
            message: "Retrying request...".to_string(),
        })
        .await?
        {
            return Ok(empty_completion(FinishReason::Cancelled));
        }
        tokio::time::sleep(PRE_STREAM_RETRY_DELAY).await;
    };

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut text_parts: Vec<String> = Vec::new();
    let mut reasoning_details: Vec<Value> = Vec::new();
    let mut reasoning_text_parts: Vec<String> = Vec::new();
    let mut tool_entries: HashMap<usize, Value> = HashMap::new();
    let mut final_usage: Option<Usage> = None;
    let mut finish_reason = FinishReason::Stop;
    let mut tool_calls_finalized = false;
    let mut stream_chunk_count = 0usize;
    let mut stream_bytes = 0usize;
    let mut saw_first_byte = false;
    let mut saw_first_text = false;
    let mut saw_first_reasoning = false;
    let mut saw_first_tool_delta = false;

    if !on_event(StreamEvent::StepStart { index: 0 }).await? {
        record_stream_end(
            stream_trace.as_ref(),
            "cancelled",
            request_started_at,
            stream_chunk_count,
            stream_bytes,
            json!({ "reason": "cancelled_before_stream_start" }),
        );
        return Ok(empty_completion(FinishReason::Cancelled));
    }

    while let Some(next) = stream.next().await {
        let chunk = match next {
            Ok(chunk) => chunk,
            Err(error) => {
                let error = LlmError::from_reqwest(
                    &provider_id,
                    &format!("{} stream failed", request.model.provider.name),
                    error,
                );
                record_stream_error(
                    stream_trace.as_ref(),
                    request_started_at,
                    stream_chunk_count,
                    stream_bytes,
                    &error,
                );
                return Err(error);
            }
        };
        stream_chunk_count = stream_chunk_count.saturating_add(1);
        stream_bytes = stream_bytes.saturating_add(chunk.len());
        if !saw_first_byte {
            saw_first_byte = true;
            if let Some(trace) = &stream_trace {
                trace.event_status(
                    "provider.stream.first_byte",
                    "ok",
                    json!({
                        "duration_ms": request_started_at.elapsed().as_millis() as i64,
                        "chunk_bytes": chunk.len()
                    }),
                );
            }
        }
        buffer.push_str(&String::from_utf8_lossy(&chunk));
        let parsed = parse_sse_payloads(&buffer);
        buffer = parsed.rest;

        for event in parsed.events {
            for data in data_lines(&event) {
                if data.is_empty() || data == "[DONE]" {
                    continue;
                }
                let chunk: Value = match serde_json::from_str(&data) {
                    Ok(chunk) => chunk,
                    Err(error) => {
                        let error = LlmError::InvalidProviderOutput {
                            provider: provider_id.clone(),
                            message: format!(
                                "{} sent invalid stream JSON: {error}",
                                request.model.provider.name
                            ),
                            raw: Some(data.clone()),
                        };
                        record_stream_error(
                            stream_trace.as_ref(),
                            request_started_at,
                            stream_chunk_count,
                            stream_bytes,
                            &error,
                        );
                        return Err(error);
                    }
                };
                if let Some(error) = chunk.get("error") {
                    let normalized = LlmError::from_stream_error(&provider_id, error);
                    let _ = on_event(StreamEvent::ProviderError(normalized.clone())).await?;
                    record_stream_error(
                        stream_trace.as_ref(),
                        request_started_at,
                        stream_chunk_count,
                        stream_bytes,
                        &normalized,
                    );
                    return Err(normalized);
                }
                if let Some(usage) = chunk.get("usage").filter(|usage| usage.is_object()) {
                    let usage = Usage::from_provider_value(usage);
                    final_usage = Some(usage.clone());
                    if !on_event(StreamEvent::Usage(usage)).await? {
                        return Ok(partial_completion(
                            text_parts,
                            tool_entries,
                            FinishReason::Cancelled,
                        ));
                    }
                }

                let Some(choice) = chunk
                    .get("choices")
                    .and_then(Value::as_array)
                    .and_then(|choices| choices.first())
                else {
                    continue;
                };
                let finish_reason_seen =
                    if let Some(reason) = choice.get("finish_reason").and_then(Value::as_str) {
                        finish_reason = FinishReason::from_provider(Some(reason));
                        if let Some(trace) = &stream_trace {
                            let reason_label = trace::finish_reason_label(&finish_reason);
                            trace.event_status(
                                "provider.stream.finish_reason",
                                reason_label.as_str(),
                                json!({
                                    "reason": reason_label,
                                    "duration_ms": request_started_at.elapsed().as_millis() as i64
                                }),
                            );
                        }
                        true
                    } else {
                        false
                    };

                let delta = choice.get("delta");
                if let Some(delta) = delta {
                    if let Some(raw_reasoning) = delta.get("reasoning").and_then(Value::as_str) {
                        if !raw_reasoning.is_empty() {
                            if !saw_first_reasoning {
                                saw_first_reasoning = true;
                                if let Some(trace) = &stream_trace {
                                    trace.event_status(
                                        "provider.stream.first_reasoning",
                                        "ok",
                                        json!({
                                            "duration_ms": request_started_at.elapsed().as_millis() as i64,
                                            "delta_chars": raw_reasoning.chars().count()
                                        }),
                                    );
                                }
                            }
                            reasoning_text_parts.push(raw_reasoning.to_string());
                            if !on_event(StreamEvent::ReasoningDelta {
                                id: "reasoning".to_string(),
                                text: raw_reasoning.to_string(),
                            })
                            .await?
                            {
                                return Ok(partial_completion(
                                    text_parts,
                                    tool_entries,
                                    FinishReason::Cancelled,
                                ));
                            }
                        }
                    }
                    if let Some(raw_details) = delta.get("reasoning_details") {
                        if let Some(items) = raw_details.as_array() {
                            reasoning_details.extend(items.iter().cloned());
                        } else if raw_details.is_object() {
                            reasoning_details.push(raw_details.clone());
                        }
                    }
                    if !reasoning_details.is_empty() {
                        let reasoning = readable_reasoning_text(
                            &reasoning_details,
                            &reasoning_text_parts.join(""),
                        );
                        if !reasoning.is_empty()
                            && !on_event(StreamEvent::ReasoningDelta {
                                id: "reasoning".to_string(),
                                text: reasoning,
                            })
                            .await?
                        {
                            return Ok(partial_completion(
                                text_parts,
                                tool_entries,
                                FinishReason::Cancelled,
                            ));
                        }
                    }
                }

                if let Some(content) = delta
                    .and_then(|delta| delta.get("content"))
                    .and_then(Value::as_str)
                {
                    if !content.is_empty() {
                        if !saw_first_text {
                            saw_first_text = true;
                            if let Some(trace) = &stream_trace {
                                trace.event_status(
                                    "provider.stream.first_text",
                                    "ok",
                                    json!({
                                        "duration_ms": request_started_at.elapsed().as_millis() as i64,
                                        "delta_chars": content.chars().count()
                                    }),
                                );
                            }
                        }
                        text_parts.push(content.to_string());
                        if !on_event(StreamEvent::TextDelta {
                            id: "assistant".to_string(),
                            text: content.to_string(),
                        })
                        .await?
                        {
                            return Ok(partial_completion(
                                text_parts,
                                tool_entries,
                                FinishReason::Cancelled,
                            ));
                        }
                    }
                }

                if let Some(raw_tools) = delta
                    .and_then(|delta| delta.get("tool_calls"))
                    .and_then(Value::as_array)
                {
                    for raw_tool in raw_tools {
                        if !saw_first_tool_delta {
                            saw_first_tool_delta = true;
                            if let Some(trace) = &stream_trace {
                                trace.event_status(
                                    "provider.stream.first_tool_delta",
                                    "ok",
                                    json!({
                                        "duration_ms": request_started_at.elapsed().as_millis() as i64,
                                        "delta_count": raw_tools.len()
                                    }),
                                );
                            }
                        }
                        let index = raw_tool
                            .get("index")
                            .and_then(Value::as_u64)
                            .map(|value| value as usize)
                            .unwrap_or(tool_entries.len());
                        let entry = tool_entries
                            .entry(index)
                            .or_insert_with(|| new_tool_entry(index));
                        merge_tool_delta(entry, raw_tool);
                        let (id, name, arguments) = tool_parts(entry);
                        if !on_event(StreamEvent::ToolCallDelta {
                            id,
                            name,
                            arguments,
                        })
                        .await?
                        {
                            return Ok(partial_completion(
                                text_parts,
                                tool_entries,
                                FinishReason::Cancelled,
                            ));
                        }
                    }
                }

                if finish_reason_seen && !tool_calls_finalized && !tool_entries.is_empty() {
                    let classified = classify_tool_calls(&tool_entries);
                    record_tool_classification(stream_trace.as_ref(), &classified, &finish_reason);
                    if !emit_tool_call_events(&classified, &mut on_event).await? {
                        record_stream_end(
                            stream_trace.as_ref(),
                            "cancelled",
                            request_started_at,
                            stream_chunk_count,
                            stream_bytes,
                            json!({ "reason": "cancelled_during_tool_finalization" }),
                        );
                        return Ok(partial_completion(
                            text_parts,
                            tool_entries,
                            FinishReason::Cancelled,
                        ));
                    }
                    tool_calls_finalized = true;
                }
            }
        }
    }

    let classified = classify_tool_calls(&tool_entries);
    if !tool_calls_finalized {
        let effective_reason =
            effective_finish_reason(finish_reason.clone(), &classified.observation);
        record_tool_classification(stream_trace.as_ref(), &classified, &effective_reason);
        if !emit_tool_call_events(&classified, &mut on_event).await? {
            record_stream_end(
                stream_trace.as_ref(),
                "cancelled",
                request_started_at,
                stream_chunk_count,
                stream_bytes,
                json!({ "reason": "cancelled_during_tool_finalization" }),
            );
            return Ok(partial_completion(
                text_parts,
                tool_entries,
                FinishReason::Cancelled,
            ));
        }
    }
    let finish_reason = effective_finish_reason(finish_reason, &classified.observation);
    let continued = on_event(StreamEvent::Finish {
        reason: finish_reason.clone(),
        usage: final_usage.clone(),
    })
    .await?;
    record_stream_end(
        stream_trace.as_ref(),
        if continued { "done" } else { "cancelled" },
        request_started_at,
        stream_chunk_count,
        stream_bytes,
        json!({
            "finish_reason": trace::finish_reason_label(&finish_reason),
            "content_chars": text_parts.iter().map(|part| part.chars().count()).sum::<usize>(),
            "tool_entry_count": tool_entries.len(),
            "usage_present": final_usage.is_some()
        }),
    );

    Ok(ChatCompletion {
        content: text_parts.join(""),
        tool_calls: classified.tool_calls,
        finish_reason,
        tool_call_observation: classified.observation,
    })
}

fn request_url_summary(url: &str) -> Value {
    match reqwest::Url::parse(url) {
        Ok(url) => json!({
            "scheme": url.scheme(),
            "host": url.host_str(),
            "path": url.path()
        }),
        Err(_) => json!({
            "parse_error": true
        }),
    }
}

fn provider_request_id(headers: &HeaderMap) -> Option<String> {
    [
        "x-request-id",
        "x-openrouter-request-id",
        "cf-ray",
        "openai-request-id",
    ]
    .into_iter()
    .find_map(|name| {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn record_stream_end(
    trace: Option<&TraceRecorder>,
    status: &str,
    request_started_at: Instant,
    chunk_count: usize,
    bytes_received: usize,
    extra: Value,
) {
    let Some(trace) = trace else {
        return;
    };
    trace.event_status(
        "provider.stream.end",
        status,
        json!({
            "duration_ms": request_started_at.elapsed().as_millis() as i64,
            "chunk_count": chunk_count,
            "bytes_received": bytes_received,
            "extra": extra
        }),
    );
}

fn record_stream_error(
    trace: Option<&TraceRecorder>,
    request_started_at: Instant,
    chunk_count: usize,
    bytes_received: usize,
    error: &LlmError,
) {
    let Some(trace) = trace else {
        return;
    };
    trace.error(
        "provider.stream.error",
        "failed",
        json!({
            "duration_ms": request_started_at.elapsed().as_millis() as i64,
            "chunk_count": chunk_count,
            "bytes_received": bytes_received,
            "error_code": error.code()
        }),
    );
    record_stream_end(
        Some(trace),
        "failed",
        request_started_at,
        chunk_count,
        bytes_received,
        json!({ "error_code": error.code() }),
    );
}

fn record_tool_classification(
    trace: Option<&TraceRecorder>,
    classified: &ClassifiedToolCalls,
    finish_reason: &FinishReason,
) {
    let Some(trace) = trace else {
        return;
    };
    let valid_count = classified.tool_calls.len();
    let malformed_count = classified.observation.malformed.len();
    if classified.observation.observed == 0 && finish_reason != &FinishReason::ToolCalls {
        return;
    }
    let status =
        if malformed_count > 0 || (finish_reason == &FinishReason::ToolCalls && valid_count == 0) {
            "failed"
        } else {
            "ok"
        };
    trace.event_status(
        "tool.finalized",
        status,
        json!({
            "finish_reason": trace::finish_reason_label(finish_reason),
            "observed_preview_count": classified.observation.observed,
            "final_executable_call_count": valid_count,
            "malformed_call_count": malformed_count
        }),
    );

    for call in &classified.tool_calls {
        let (id, name, arguments) = tool_parts(call);
        trace.with_tool_call(id).event_status(
            "tool.finalized",
            "ok",
            json!({
                "tool_name": name,
                "arguments_bytes": arguments.len(),
                "arguments_empty": arguments.trim().is_empty()
            }),
        );
    }
    for malformed in &classified.observation.malformed {
        trace.with_tool_call(malformed.id.clone()).error(
            "tool.args.parse_failed",
            "failed",
            json!({
                "tool_name": malformed.name.as_deref(),
                "arguments_bytes": malformed.arguments.len(),
                "message": malformed.message.as_str()
            }),
        );
    }
}

fn headers(request: &LlmRequest) -> Result<HeaderMap, LlmError> {
    let provider_id = request.model.provider.id.to_string();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    super::apply_auth_headers(&mut headers, &request.model.provider.auth, &provider_id)?;
    for (key, value) in &request.model.request.headers {
        let name = HeaderName::from_bytes(key.as_bytes()).map_err(|error| LlmError::Config {
            message: format!("Invalid header name for {provider_id}: {key}: {error}"),
        })?;
        let value = HeaderValue::from_str(value).map_err(|error| LlmError::Config {
            message: format!("Invalid header value for {provider_id}: {key}: {error}"),
        })?;
        headers.insert(name, value);
    }
    Ok(headers)
}

fn body(request: &LlmRequest) -> Value {
    let mut body = request.model.request.body.clone();
    body.insert(
        "model".to_string(),
        Value::String(request.model.model.adapter_model_id.clone()),
    );
    body.insert(
        "messages".to_string(),
        Value::Array(request.messages.clone()),
    );
    body.insert("stream".to_string(), Value::Bool(true));
    body.insert(
        "stream_options".to_string(),
        json!({ "include_usage": true }),
    );
    if !request.tools.is_empty() && request.model.model.capabilities.tools {
        body.insert("tools".to_string(), Value::Array(request.tools.clone()));
    }
    if let Some(temperature) = request.model.request.generation.temperature {
        body.insert("temperature".to_string(), json!(temperature));
    }
    if let Some(max_tokens) = request.model.request.generation.max_output_tokens {
        body.insert("max_tokens".to_string(), json!(max_tokens));
    }
    if request.model.model.capabilities.reasoning {
        if let Some(reasoning_effort) = request
            .model
            .request
            .generation
            .reasoning_effort
            .as_deref()
            .filter(|effort| !effort.trim().is_empty())
        {
            body.insert(
                "reasoning".to_string(),
                json!({ "effort": reasoning_effort }),
            );
        }
    }
    Value::Object(body)
}

fn merge_tool_delta(entry: &mut Value, raw_tool: &Value) {
    if let Some(id) = raw_tool.get("id").and_then(Value::as_str) {
        entry["provider_tool_call_id"] = json!(id);
    }
    if let Some(tool_type) = raw_tool.get("type").and_then(Value::as_str) {
        entry["type"] = json!(tool_type);
    }
    let raw_function = raw_tool.get("function").unwrap_or(&Value::Null);
    if let Some(name) = raw_function.get("name").and_then(Value::as_str) {
        entry["function"]["name"] = json!(name);
    }
    if let Some(arguments) = raw_function.get("arguments").and_then(Value::as_str) {
        let current = entry["function"]["arguments"].as_str().unwrap_or_default();
        entry["function"]["arguments"] = json!(format!("{current}{arguments}"));
    } else if let Some(arguments) = raw_function.get("arguments") {
        entry["function"]["arguments"] = json!(arguments.to_string());
    }
}

fn new_tool_entry(index: usize) -> Value {
    json!({
        "id": new_id("call"),
        "provider_tool_call_id": format!("tool_call_{index}"),
        "type": "function",
        "function": { "name": "", "arguments": "" }
    })
}

fn tool_parts(value: &Value) -> (String, String, String) {
    let id = value
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("tool_call")
        .to_string();
    let function = value.get("function").unwrap_or(&Value::Null);
    let name = function
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let arguments = function
        .get("arguments")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    (id, name, arguments)
}

fn partial_completion(
    text_parts: Vec<String>,
    tool_entries: HashMap<usize, Value>,
    finish_reason: FinishReason,
) -> ChatCompletion {
    let classified = classify_tool_calls(&tool_entries);
    let finish_reason = effective_finish_reason(finish_reason, &classified.observation);
    ChatCompletion {
        content: text_parts.join(""),
        tool_calls: classified.tool_calls,
        finish_reason,
        tool_call_observation: classified.observation,
    }
}

fn empty_completion(finish_reason: FinishReason) -> ChatCompletion {
    ChatCompletion {
        content: String::new(),
        tool_calls: Vec::new(),
        finish_reason,
        tool_call_observation: ToolCallObservation::none(),
    }
}

#[derive(Clone, Debug)]
struct ClassifiedToolCalls {
    tool_calls: Vec<Value>,
    observation: ToolCallObservation,
}

fn classify_tool_calls(tool_entries: &HashMap<usize, Value>) -> ClassifiedToolCalls {
    let mut tool_calls = tool_entries
        .iter()
        .map(|(index, value)| (*index, value.clone()))
        .collect::<Vec<(usize, Value)>>();
    tool_calls.sort_by_key(|(index, _)| *index);

    let mut valid = Vec::new();
    let mut malformed = Vec::new();
    for (_, value) in tool_calls {
        match normalize_tool_call(value) {
            Ok(call) => valid.push(call),
            Err(error) => malformed.push(error),
        }
    }

    ClassifiedToolCalls {
        tool_calls: valid,
        observation: ToolCallObservation {
            observed: tool_entries.len(),
            malformed,
        },
    }
}

async fn emit_tool_call_events<F, Fut>(
    classified: &ClassifiedToolCalls,
    on_event: &mut F,
) -> Result<bool, LlmError>
where
    F: FnMut(StreamEvent) -> Fut + Send,
    Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
{
    for call in &classified.tool_calls {
        let (id, name, arguments) = tool_parts(call);
        if !on_event(StreamEvent::ToolCall {
            id,
            name,
            arguments,
            raw: call.clone(),
        })
        .await?
        {
            return Ok(false);
        }
    }
    for malformed in &classified.observation.malformed {
        if !on_event(StreamEvent::ToolCallMalformed {
            id: malformed.id.clone(),
            name: malformed.name.clone().unwrap_or_default(),
            arguments: malformed.arguments.clone(),
            message: malformed.message.clone(),
            raw: malformed.raw.clone(),
        })
        .await?
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn effective_finish_reason(
    finish_reason: FinishReason,
    observation: &ToolCallObservation,
) -> FinishReason {
    if finish_reason == FinishReason::Stop && observation.has_observed() {
        FinishReason::ToolCalls
    } else {
        finish_reason
    }
}

fn normalize_tool_call(mut value: Value) -> Result<Value, MalformedToolCall> {
    let raw = Some(value.to_string());
    let function = value.get("function").ok_or_else(|| MalformedToolCall {
        id: tool_id(&value),
        name: None,
        arguments: String::new(),
        message: "Tool call is missing its function payload.".to_string(),
        raw: raw.clone(),
    })?;
    let name = function
        .get("name")
        .and_then(Value::as_str)
        .map(str::trim)
        .unwrap_or_default();
    if name.is_empty() {
        return Err(MalformedToolCall {
            id: tool_id(&value),
            name: None,
            arguments: function
                .get("arguments")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string(),
            message: "Tool call is missing a function name.".to_string(),
            raw,
        });
    }
    let arguments = function
        .get("arguments")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let normalized_arguments = if arguments.trim().is_empty() {
        json!({})
    } else {
        match serde_json::from_str::<Value>(arguments) {
            Ok(parsed) if parsed.is_object() => parsed,
            Ok(_) => {
                return Err(MalformedToolCall {
                    id: tool_id(&value),
                    name: Some(name.to_string()),
                    arguments: arguments.to_string(),
                    message: "Tool call arguments must be a JSON object.".to_string(),
                    raw,
                });
            }
            Err(error) => {
                return Err(MalformedToolCall {
                    id: tool_id(&value),
                    name: Some(name.to_string()),
                    arguments: arguments.to_string(),
                    message: format!("Tool call arguments are not valid JSON: {error}"),
                    raw,
                });
            }
        }
    };

    if let Some(function) = value.get_mut("function").and_then(Value::as_object_mut) {
        function.insert(
            "arguments".to_string(),
            Value::String(normalized_arguments.to_string()),
        );
    }
    Ok(value)
}

fn tool_id(value: &Value) -> String {
    value
        .get("id")
        .and_then(Value::as_str)
        .filter(|id| !id.trim().is_empty())
        .unwrap_or("tool_call")
        .to_string()
}

fn readable_reasoning_text(reasoning_details: &[Value], reasoning_text: &str) -> String {
    let mut parts = Vec::new();
    if !reasoning_text.is_empty() {
        parts.push(reasoning_text.to_string());
    }
    for entry in reasoning_details {
        let Some(row) = entry.as_object() else {
            continue;
        };
        let text = row
            .get("summary")
            .or_else(|| row.get("text"))
            .or_else(|| row.get("content"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if text.is_empty() || text == "[REDACTED]" {
            continue;
        }
        if !reasoning_text.is_empty()
            && (text.contains(reasoning_text) || reasoning_text.contains(text))
        {
            continue;
        }
        parts.push(text.to_string());
    }
    parts.join("\n")
}

#[allow(dead_code)]
fn _assert_body_is_object(value: &Value) -> Option<&Map<String, Value>> {
    value.as_object()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::providers::request::LlmRequest;
    use crate::runtime_daemon::chat::providers::types::{
        Auth, Capabilities, GenerationDefaults, Limits, ModelDefinition, ModelId, ModelRef,
        ProviderDefinition, ProviderId, RequestDefaults, ResolvedModel,
    };
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use tokio::sync::Mutex;

    #[test]
    fn empty_tool_arguments_are_normalized_to_object() {
        let tool_entries = HashMap::from([(
            0,
            json!({
                "id": "call_1",
                "type": "function",
                "function": { "name": "read_file", "arguments": "" }
            }),
        )]);

        let classified = classify_tool_calls(&tool_entries);

        assert_eq!(classified.observation.observed, 1);
        assert!(classified.observation.malformed.is_empty());
        assert_eq!(
            classified.tool_calls[0]["function"]["arguments"],
            Value::String("{}".to_string())
        );
    }

    #[test]
    fn provider_tool_ids_do_not_replace_internal_tool_ids() {
        let mut first = new_tool_entry(0);
        merge_tool_delta(
            &mut first,
            &json!({
                "id": "tool_call_0",
                "function": { "name": "read_file", "arguments": "{}" }
            }),
        );
        let mut second = new_tool_entry(0);
        merge_tool_delta(
            &mut second,
            &json!({
                "id": "tool_call_0",
                "function": { "name": "read_file", "arguments": "{}" }
            }),
        );

        let first_id = first["id"].as_str().unwrap().to_string();
        let second_id = second["id"].as_str().unwrap().to_string();
        assert!(first_id.starts_with("call_"));
        assert!(second_id.starts_with("call_"));
        assert_ne!(first_id, second_id);
        assert_eq!(first["provider_tool_call_id"], "tool_call_0");
        assert_eq!(second["provider_tool_call_id"], "tool_call_0");

        let classified = classify_tool_calls(&HashMap::from([(0, first)]));
        assert_eq!(classified.tool_calls[0]["id"].as_str().unwrap(), first_id);
        assert_eq!(
            classified.tool_calls[0]["provider_tool_call_id"],
            "tool_call_0"
        );
    }

    #[test]
    fn invalid_tool_arguments_are_preserved_as_malformed() {
        let tool_entries = HashMap::from([(
            0,
            json!({
                "id": "call_1",
                "type": "function",
                "function": { "name": "read_file", "arguments": "{\"path\":" }
            }),
        )]);

        let classified = classify_tool_calls(&tool_entries);

        assert!(classified.tool_calls.is_empty());
        assert_eq!(classified.observation.observed, 1);
        assert_eq!(classified.observation.malformed.len(), 1);
        assert_eq!(
            classified.observation.malformed[0].name.as_deref(),
            Some("read_file")
        );
        assert!(
            classified.observation.malformed[0]
                .message
                .contains("not valid JSON")
        );
    }

    #[tokio::test]
    async fn malformed_tool_call_emits_terminal_malformed_event() {
        let classified = classify_tool_calls(&HashMap::from([(
            0,
            json!({
                "id": "call_1",
                "type": "function",
                "function": { "name": "read_file", "arguments": "{\"path\":" }
            }),
        )]));
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_for_callback = Arc::clone(&events);
        let mut on_event = move |event| {
            let events = Arc::clone(&events_for_callback);
            async move {
                events.lock().await.push(event);
                Ok(true)
            }
        };

        let emitted = emit_tool_call_events(&classified, &mut on_event)
            .await
            .unwrap();

        assert!(emitted);
        assert!(matches!(
            events.lock().await.as_slice(),
            [StreamEvent::ToolCallMalformed { id, .. }] if id == "call_1"
        ));
    }

    #[test]
    fn stop_finish_becomes_tool_calls_when_tool_delta_was_observed() {
        let observation = ToolCallObservation {
            observed: 1,
            malformed: Vec::new(),
        };

        assert_eq!(
            effective_finish_reason(FinishReason::Stop, &observation),
            FinishReason::ToolCalls
        );
    }

    #[tokio::test]
    async fn retryable_provider_error_emits_status_and_then_succeeds() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for attempt in 0..2 {
                let (mut socket, _) = listener.accept().await.unwrap();
                read_http_request(&mut socket).await;
                if attempt == 0 {
                    socket
                        .write_all(b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\n\r\n")
                        .await
                        .unwrap();
                } else {
                    let body = concat!(
                        "data: {\"choices\":[{\"delta\":{\"content\":\"ok\"},\"finish_reason\":null}]}\n\n",
                        "data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n\n"
                    );
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\n\r\n{}",
                        body.len(),
                        body
                    );
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
            }
        });

        let statuses = Arc::new(Mutex::new(Vec::new()));
        let completion = stream_chat(&test_request(format!("http://{addr}")), None, {
            let statuses = Arc::clone(&statuses);
            move |event| {
                let statuses = Arc::clone(&statuses);
                async move {
                    if let StreamEvent::Status { message } = event {
                        statuses.lock().await.push(message);
                    }
                    Ok(true)
                }
            }
        })
        .await
        .unwrap();

        server.await.unwrap();
        assert_eq!(completion.content, "ok");
        assert_eq!(statuses.lock().await.as_slice(), ["Retrying request..."]);
    }

    #[tokio::test]
    async fn official_openai_sends_bearer_auth_to_chat_completions() {
        let body = concat!(
            "data: {\"choices\":[{\"delta\":{\"content\":\"ok\"},\"finish_reason\":null}]}\n\n",
            "data: {\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n\n"
        );
        let (addr, server, captured) = serve_once(200, body).await;
        let mut request = test_request(format!("http://{addr}"));
        request.model.provider.id = ProviderId::unchecked(ProviderId::OPENAI);
        request.model.provider.name = "OpenAI".to_string();
        request.model.provider.auth = Auth::InlineBearer {
            value: "test-openai-key".to_string(),
        };

        let completion = stream_chat(&request, None, |_| async { Ok(true) })
            .await
            .unwrap();

        server.await.unwrap();
        assert_eq!(completion.content, "ok");
        let request = captured.lock().await.clone().unwrap();
        let request_lower = request.to_ascii_lowercase();
        assert!(request_lower.contains("authorization: bearer test-openai-key"));
        assert!(request.contains("POST /chat/completions HTTP/1.1"));
    }

    async fn serve_once(
        status: u16,
        body: &str,
    ) -> (
        std::net::SocketAddr,
        tokio::task::JoinHandle<()>,
        Arc<Mutex<Option<String>>>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let captured = Arc::new(Mutex::new(None));
        let captured_for_server = Arc::clone(&captured);
        let body = body.to_string();
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_http_request(&mut socket).await;
            *captured_for_server.lock().await = Some(request);
            let response = format!(
                "HTTP/1.1 {} OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\n\r\n{}",
                status,
                body.len(),
                body
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });
        (addr, server, captured)
    }

    async fn read_http_request(socket: &mut tokio::net::TcpStream) -> String {
        let mut buffer = Vec::new();
        let mut temp = [0u8; 1024];
        loop {
            let read = socket.read(&mut temp).await.unwrap();
            if read == 0 {
                break;
            }
            buffer.extend_from_slice(&temp[..read]);
            let Some(header_end) = buffer.windows(4).position(|window| window == b"\r\n\r\n")
            else {
                continue;
            };
            let headers = String::from_utf8_lossy(&buffer[..header_end]);
            let content_length = headers
                .lines()
                .find_map(|line| line.strip_prefix("Content-Length:"))
                .and_then(|value| value.trim().parse::<usize>().ok())
                .unwrap_or(0);
            if buffer.len() >= header_end + 4 + content_length {
                break;
            }
        }
        String::from_utf8_lossy(&buffer).to_string()
    }

    fn test_request(base_url: String) -> LlmRequest {
        let provider_id = ProviderId::unchecked(ProviderId::LOCAL);
        let model_id = ModelId::new("test-model").unwrap();
        let provider = ProviderDefinition {
            id: provider_id.clone(),
            name: "Test Provider".to_string(),
            adapter: AdapterKind::OpenAiCompatibleChat,
            base_url: Some(base_url),
            auth: Auth::None,
            request: RequestDefaults {
                generation: GenerationDefaults::default(),
                ..RequestDefaults::default()
            },
            disabled: false,
        };
        let model = ModelDefinition {
            id: model_id.clone(),
            provider: provider_id.clone(),
            display_name: "Test Model".to_string(),
            adapter_model_id: "test-model".to_string(),
            capabilities: Capabilities::text_tools(),
            limits: Limits::default(),
            request: RequestDefaults::default(),
            enabled: true,
        };
        LlmRequest {
            model: ResolvedModel {
                reference: ModelRef::new(provider_id, model_id),
                provider,
                model,
                request: RequestDefaults::default(),
            },
            messages: vec![json!({ "role": "user", "content": "hello" })],
            tools: Vec::new(),
        }
    }
}
