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

const ANTHROPIC_VERSION: &str = "2023-06-01";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const MAX_PRE_STREAM_RETRIES: usize = 2;
const PRE_STREAM_RETRY_DELAY: Duration = Duration::from_millis(500);
const DEFAULT_MAX_TOKENS: u32 = 4096;

pub(crate) async fn stream_chat<F, Fut>(
    request: &LlmRequest,
    trace: Option<TraceRecorder>,
    mut on_event: F,
) -> Result<ChatCompletion, LlmError>
where
    F: FnMut(StreamEvent) -> Fut + Send,
    Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
{
    if request.model.provider.adapter != AdapterKind::AnthropicCompatibleMessages {
        return Err(LlmError::ProviderInit {
            provider: request.model.provider.id.to_string(),
            message: format!(
                "{} does not use the Anthropic-compatible Messages adapter.",
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

    let request_url = format!("{base_url}/messages");
    let request_headers = headers(request)?;
    let request_body = body(request)?;
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
                    record_request_failure(
                        attempt_trace.as_ref(),
                        attempt,
                        Some(status.as_u16()),
                        request_started_at,
                        &error,
                    );
                    return Err(error);
                }
                record_retry(
                    attempt_trace.as_ref(),
                    attempt,
                    Some(status.as_u16()),
                    &error,
                );
            }
            Err(error) => {
                if attempt >= MAX_PRE_STREAM_RETRIES || !error.is_retryable() {
                    record_request_failure(
                        attempt_trace.as_ref(),
                        attempt,
                        None,
                        request_started_at,
                        &error,
                    );
                    return Err(error);
                }
                record_retry(attempt_trace.as_ref(), attempt, None, &error);
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

    parse_stream(
        response,
        stream_trace,
        request_started_at,
        &provider_id,
        &request.model.provider.name,
        on_event,
    )
    .await
}

async fn parse_stream<F, Fut>(
    response: reqwest::Response,
    trace: Option<TraceRecorder>,
    request_started_at: Instant,
    provider_id: &str,
    provider_name: &str,
    mut on_event: F,
) -> Result<ChatCompletion, LlmError>
where
    F: FnMut(StreamEvent) -> Fut + Send,
    Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
{
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut state = StreamState::default();
    let mut stream_chunk_count = 0usize;
    let mut stream_bytes = 0usize;
    let mut saw_first_byte = false;

    if !on_event(StreamEvent::StepStart { index: 0 }).await? {
        record_stream_end(
            trace.as_ref(),
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
                    provider_id,
                    &format!("{provider_name} stream failed"),
                    error,
                );
                record_stream_error(
                    trace.as_ref(),
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
            if let Some(trace) = &trace {
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
                            provider: provider_id.to_string(),
                            message: format!("{provider_name} sent invalid stream JSON: {error}"),
                            raw: Some(data.clone()),
                        };
                        record_stream_error(
                            trace.as_ref(),
                            request_started_at,
                            stream_chunk_count,
                            stream_bytes,
                            &error,
                        );
                        return Err(error);
                    }
                };
                if let Some(error) = chunk.get("error") {
                    let normalized = anthropic_stream_error(provider_id, error);
                    let _ = on_event(StreamEvent::ProviderError(normalized.clone())).await?;
                    record_stream_error(
                        trace.as_ref(),
                        request_started_at,
                        stream_chunk_count,
                        stream_bytes,
                        &normalized,
                    );
                    return Err(normalized);
                }

                if !state
                    .handle_chunk(&chunk, provider_id, &trace, &mut on_event)
                    .await?
                {
                    record_stream_end(
                        trace.as_ref(),
                        "cancelled",
                        request_started_at,
                        stream_chunk_count,
                        stream_bytes,
                        json!({ "reason": "consumer_cancelled" }),
                    );
                    return Ok(state.partial_completion(FinishReason::Cancelled));
                }
            }
        }
    }

    if !buffer.trim().is_empty() {
        let error = LlmError::InvalidProviderOutput {
            provider: provider_id.to_string(),
            message: format!("{provider_name} stream ended with an incomplete SSE event."),
            raw: Some(buffer),
        };
        record_stream_error(
            trace.as_ref(),
            request_started_at,
            stream_chunk_count,
            stream_bytes,
            &error,
        );
        return Err(error);
    }
    if !state.saw_message_stop {
        let error = LlmError::InvalidProviderOutput {
            provider: provider_id.to_string(),
            message: format!("{provider_name} stream ended before message_stop."),
            raw: None,
        };
        record_stream_error(
            trace.as_ref(),
            request_started_at,
            stream_chunk_count,
            stream_bytes,
            &error,
        );
        return Err(error);
    }

    if !on_event(StreamEvent::Finish {
        reason: state.finish_reason.clone(),
        usage: state.final_usage.clone(),
    })
    .await?
    {
        return Ok(state.partial_completion(FinishReason::Cancelled));
    }

    let completion = state.completion();
    record_stream_end(
        trace.as_ref(),
        "ok",
        request_started_at,
        stream_chunk_count,
        stream_bytes,
        json!({
            "finish_reason": trace::finish_reason_label(&completion.finish_reason),
            "text_chars": completion.content.chars().count(),
            "tool_call_count": completion.tool_calls.len(),
            "malformed_tool_call_count": completion.tool_call_observation.malformed.len()
        }),
    );
    Ok(completion)
}

#[derive(Clone, Debug)]
struct AnthropicToolBlock {
    internal_id: String,
    provider_tool_id: String,
    name: String,
    arguments: String,
}

#[derive(Clone, Debug)]
enum AnthropicContentBlock {
    Text,
    Tool(AnthropicToolBlock),
    Other,
}

struct StreamState {
    text_parts: Vec<String>,
    active_blocks: HashMap<usize, AnthropicContentBlock>,
    tool_calls: Vec<Value>,
    malformed_tool_calls: Vec<MalformedToolCall>,
    final_usage: Option<Usage>,
    finish_reason: FinishReason,
    saw_message_stop: bool,
}

impl Default for StreamState {
    fn default() -> Self {
        Self {
            text_parts: Vec::new(),
            active_blocks: HashMap::new(),
            tool_calls: Vec::new(),
            malformed_tool_calls: Vec::new(),
            final_usage: None,
            finish_reason: FinishReason::Stop,
            saw_message_stop: false,
        }
    }
}

impl StreamState {
    async fn handle_chunk<F, Fut>(
        &mut self,
        chunk: &Value,
        provider_id: &str,
        trace: &Option<TraceRecorder>,
        on_event: &mut F,
    ) -> Result<bool, LlmError>
    where
        F: FnMut(StreamEvent) -> Fut + Send,
        Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
    {
        match chunk
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default()
        {
            "message_start" => {
                if let Some(usage) = chunk
                    .get("message")
                    .and_then(|message| message.get("usage"))
                    .filter(|usage| usage.is_object())
                {
                    self.final_usage = Some(Usage::from_provider_value(usage));
                }
            }
            "content_block_start" => {
                let index = block_index(chunk);
                let block = chunk.get("content_block").unwrap_or(&Value::Null);
                match block
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                {
                    "text" => {
                        self.active_blocks
                            .insert(index, AnthropicContentBlock::Text);
                        if let Some(text) = block.get("text").and_then(Value::as_str) {
                            if !self.emit_text(text, on_event).await? {
                                return Ok(false);
                            }
                        }
                    }
                    "tool_use" => {
                        let provider_tool_id = block
                            .get("id")
                            .and_then(Value::as_str)
                            .filter(|id| !id.trim().is_empty())
                            .unwrap_or("tool_use")
                            .to_string();
                        let name = block
                            .get("name")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string();
                        let arguments = block
                            .get("input")
                            .filter(|input| {
                                input.as_object().is_none_or(|object| !object.is_empty())
                            })
                            .map(Value::to_string)
                            .unwrap_or_default();
                        let tool = AnthropicToolBlock {
                            internal_id: new_id("call"),
                            provider_tool_id,
                            name,
                            arguments,
                        };
                        if !on_event(StreamEvent::ToolCallDelta {
                            id: tool.internal_id.clone(),
                            name: tool.name.clone(),
                            arguments: tool.arguments.clone(),
                        })
                        .await?
                        {
                            return Ok(false);
                        }
                        self.active_blocks
                            .insert(index, AnthropicContentBlock::Tool(tool));
                    }
                    _ => {
                        self.active_blocks
                            .insert(index, AnthropicContentBlock::Other);
                    }
                }
            }
            "content_block_delta" => {
                let index = block_index(chunk);
                let delta = chunk.get("delta").unwrap_or(&Value::Null);
                match delta
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                {
                    "text_delta" => {
                        let text = delta
                            .get("text")
                            .and_then(Value::as_str)
                            .unwrap_or_default();
                        if !self.emit_text(text, on_event).await? {
                            return Ok(false);
                        }
                    }
                    "thinking_delta" => {
                        let text = delta
                            .get("thinking")
                            .or_else(|| delta.get("text"))
                            .and_then(Value::as_str)
                            .unwrap_or_default();
                        if !text.is_empty()
                            && !on_event(StreamEvent::ReasoningDelta {
                                id: "reasoning".to_string(),
                                text: text.to_string(),
                            })
                            .await?
                        {
                            return Ok(false);
                        }
                    }
                    "input_json_delta" => {
                        let partial = delta
                            .get("partial_json")
                            .and_then(Value::as_str)
                            .unwrap_or_default();
                        if let Some(AnthropicContentBlock::Tool(tool)) =
                            self.active_blocks.get_mut(&index)
                        {
                            tool.arguments.push_str(partial);
                            if !on_event(StreamEvent::ToolCallDelta {
                                id: tool.internal_id.clone(),
                                name: tool.name.clone(),
                                arguments: tool.arguments.clone(),
                            })
                            .await?
                            {
                                return Ok(false);
                            }
                        }
                    }
                    _ => {}
                }
            }
            "content_block_stop" => {
                let index = block_index(chunk);
                if let Some(AnthropicContentBlock::Tool(tool)) = self.active_blocks.remove(&index) {
                    if !self
                        .finish_tool_block(provider_id, tool, trace, on_event)
                        .await?
                    {
                        return Ok(false);
                    }
                }
            }
            "message_delta" => {
                if let Some(reason) = chunk
                    .get("delta")
                    .and_then(|delta| delta.get("stop_reason"))
                    .and_then(Value::as_str)
                {
                    self.finish_reason = anthropic_finish_reason(reason);
                }
                if let Some(usage) = chunk.get("usage").filter(|usage| usage.is_object()) {
                    let usage = Usage::from_provider_value(usage);
                    self.final_usage = Some(usage.clone());
                    if !on_event(StreamEvent::Usage(usage)).await? {
                        return Ok(false);
                    }
                }
            }
            "message_stop" => {
                self.saw_message_stop = true;
            }
            "ping" => {}
            _ => {}
        }
        Ok(true)
    }

    async fn emit_text<F, Fut>(&mut self, text: &str, on_event: &mut F) -> Result<bool, LlmError>
    where
        F: FnMut(StreamEvent) -> Fut + Send,
        Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
    {
        if text.is_empty() {
            return Ok(true);
        }
        self.text_parts.push(text.to_string());
        on_event(StreamEvent::TextDelta {
            id: "assistant".to_string(),
            text: text.to_string(),
        })
        .await
    }

    async fn finish_tool_block<F, Fut>(
        &mut self,
        provider_id: &str,
        tool: AnthropicToolBlock,
        trace: &Option<TraceRecorder>,
        on_event: &mut F,
    ) -> Result<bool, LlmError>
    where
        F: FnMut(StreamEvent) -> Fut + Send,
        Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
    {
        let raw = tool_call_value(&tool);
        match normalize_tool_arguments(&tool.arguments) {
            Ok(arguments) => {
                let call = json!({
                    "id": tool.internal_id,
                    "provider_tool_call_id": tool.provider_tool_id,
                    "type": "function",
                    "function": {
                        "name": tool.name,
                        "arguments": arguments
                    }
                });
                self.tool_calls.push(call.clone());
                on_event(StreamEvent::ToolCall {
                    id: call["id"].as_str().unwrap_or("tool_call").to_string(),
                    name: call["function"]["name"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    arguments,
                    raw: call,
                })
                .await
            }
            Err(message) => {
                let malformed = MalformedToolCall {
                    id: tool.internal_id,
                    name: Some(tool.name),
                    arguments: tool.arguments,
                    message: message.clone(),
                    raw: Some(raw.to_string()),
                };
                if let Some(trace) = trace {
                    trace.with_tool_call(malformed.id.clone()).error(
                        "tool.args.parse_failed",
                        "failed",
                        json!({
                            "provider": provider_id,
                            "tool_name": malformed.name.as_deref(),
                            "message": malformed.message.as_str(),
                            "arguments_bytes": malformed.arguments.len()
                        }),
                    );
                }
                self.malformed_tool_calls.push(malformed.clone());
                on_event(StreamEvent::ToolCallMalformed {
                    id: malformed.id,
                    name: malformed.name.unwrap_or_default(),
                    arguments: malformed.arguments,
                    message: malformed.message,
                    raw: malformed.raw,
                })
                .await
            }
        }
    }

    fn partial_completion(&self, finish_reason: FinishReason) -> ChatCompletion {
        ChatCompletion {
            content: self.text_parts.join(""),
            tool_calls: self.tool_calls.clone(),
            finish_reason,
            tool_call_observation: ToolCallObservation {
                observed: self.tool_calls.len() + self.malformed_tool_calls.len(),
                malformed: self.malformed_tool_calls.clone(),
            },
        }
    }

    fn completion(&self) -> ChatCompletion {
        let finish_reason =
            if self.finish_reason == FinishReason::Stop && self.tool_observation_count() > 0 {
                FinishReason::ToolCalls
            } else {
                self.finish_reason.clone()
            };
        ChatCompletion {
            content: self.text_parts.join(""),
            tool_calls: self.tool_calls.clone(),
            finish_reason,
            tool_call_observation: ToolCallObservation {
                observed: self.tool_observation_count(),
                malformed: self.malformed_tool_calls.clone(),
            },
        }
    }

    fn tool_observation_count(&self) -> usize {
        self.tool_calls.len() + self.malformed_tool_calls.len()
    }
}

fn headers(request: &LlmRequest) -> Result<HeaderMap, LlmError> {
    let provider_id = request.model.provider.id.to_string();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        HeaderName::from_static("anthropic-version"),
        HeaderValue::from_static(ANTHROPIC_VERSION),
    );
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

fn body(request: &LlmRequest) -> Result<Value, LlmError> {
    let mut body = request.model.request.body.clone();
    body.insert(
        "model".to_string(),
        Value::String(request.model.model.adapter_model_id.clone()),
    );
    body.insert("stream".to_string(), Value::Bool(true));
    body.insert(
        "max_tokens".to_string(),
        json!(
            request
                .model
                .request
                .generation
                .max_output_tokens
                .or(request.model.model.limits.output_tokens)
                .unwrap_or(DEFAULT_MAX_TOKENS)
        ),
    );
    if let Some(temperature) = request.model.request.generation.temperature {
        body.insert("temperature".to_string(), json!(temperature));
    }

    let (system, messages) = anthropic_messages(&request.messages, &request.model.provider.id)?;
    if !system.is_empty() {
        body.insert("system".to_string(), Value::String(system.join("\n\n")));
    }
    body.insert("messages".to_string(), Value::Array(messages));

    if !request.tools.is_empty() && request.model.model.capabilities.tools {
        let tools = request
            .tools
            .iter()
            .filter_map(anthropic_tool_definition)
            .collect::<Vec<_>>();
        if !tools.is_empty() {
            body.insert("tools".to_string(), Value::Array(tools));
        }
    }

    Ok(Value::Object(body))
}

fn anthropic_messages(
    messages: &[Value],
    provider: &super::super::types::ProviderId,
) -> Result<(Vec<String>, Vec<Value>), LlmError> {
    let mut system = Vec::new();
    let mut out = Vec::new();
    let mut provider_tool_ids_by_internal_id = HashMap::<String, String>::new();

    for message in messages {
        let role = message
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or_default();
        match role {
            "system" => {
                let text = content_to_text(message.get("content").unwrap_or(&Value::Null));
                if !text.trim().is_empty() {
                    system.push(text);
                }
            }
            "user" => {
                let content = anthropic_content_blocks(
                    message.get("content").unwrap_or(&Value::Null),
                    provider,
                    true,
                )?;
                if !content.is_empty() {
                    out.push(json!({ "role": "user", "content": content }));
                }
            }
            "assistant" => {
                let mut content = anthropic_content_blocks(
                    message.get("content").unwrap_or(&Value::Null),
                    provider,
                    false,
                )?;
                if let Some(tool_calls) = message.get("tool_calls").and_then(Value::as_array) {
                    for tool_call in tool_calls {
                        if let Some(tool_use) = anthropic_tool_use_block(tool_call) {
                            let internal_id = tool_call
                                .get("id")
                                .and_then(Value::as_str)
                                .unwrap_or_default()
                                .to_string();
                            let provider_tool_id = tool_use
                                .get("id")
                                .and_then(Value::as_str)
                                .unwrap_or_default()
                                .to_string();
                            if !internal_id.is_empty() && !provider_tool_id.is_empty() {
                                provider_tool_ids_by_internal_id
                                    .insert(internal_id, provider_tool_id);
                            }
                            content.push(tool_use);
                        }
                    }
                }
                if !content.is_empty() {
                    out.push(json!({ "role": "assistant", "content": content }));
                }
            }
            "tool" => {
                let internal_id = message
                    .get("tool_call_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if internal_id.is_empty() {
                    continue;
                }
                let provider_tool_id = provider_tool_ids_by_internal_id
                    .get(internal_id)
                    .map(String::as_str)
                    .unwrap_or(internal_id);
                out.push(json!({
                    "role": "user",
                    "content": [{
                        "type": "tool_result",
                        "tool_use_id": provider_tool_id,
                        "content": content_to_text(message.get("content").unwrap_or(&Value::Null))
                    }]
                }));
            }
            _ => {}
        }
    }

    Ok((system, out))
}

fn anthropic_content_blocks(
    content: &Value,
    provider: &super::super::types::ProviderId,
    allow_images: bool,
) -> Result<Vec<Value>, LlmError> {
    match content {
        Value::String(text) => Ok(text_block(text).into_iter().collect()),
        Value::Array(parts) => {
            let mut blocks = Vec::new();
            for part in parts {
                if let Some(text) = part
                    .get("text")
                    .and_then(Value::as_str)
                    .or_else(|| part.as_str())
                {
                    if let Some(block) = text_block(text) {
                        blocks.push(block);
                    }
                    continue;
                }
                if allow_images
                    && part.get("type").and_then(Value::as_str) == Some("image_url")
                    && let Some(url) = part
                        .get("image_url")
                        .and_then(|image| image.get("url"))
                        .and_then(Value::as_str)
                {
                    blocks.push(anthropic_image_block(url, provider)?);
                }
            }
            Ok(blocks)
        }
        Value::Null => Ok(Vec::new()),
        other => Ok(text_block(&other.to_string()).into_iter().collect()),
    }
}

fn anthropic_tool_definition(tool: &Value) -> Option<Value> {
    let function = tool.get("function")?;
    let name = function.get("name").and_then(Value::as_str)?.trim();
    if name.is_empty() {
        return None;
    }
    let description = function
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let input_schema = function
        .get("parameters")
        .cloned()
        .unwrap_or_else(|| json!({ "type": "object", "properties": {} }));
    Some(json!({
        "name": name,
        "description": description,
        "input_schema": input_schema
    }))
}

fn anthropic_tool_use_block(tool_call: &Value) -> Option<Value> {
    let function = tool_call.get("function")?;
    let name = function.get("name").and_then(Value::as_str)?.trim();
    if name.is_empty() {
        return None;
    }
    let internal_id = tool_call
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let provider_tool_id = tool_call
        .get("provider_tool_call_id")
        .and_then(Value::as_str)
        .unwrap_or(internal_id);
    if provider_tool_id.trim().is_empty() {
        return None;
    }
    let input = function
        .get("arguments")
        .and_then(Value::as_str)
        .and_then(|arguments| serde_json::from_str::<Value>(arguments).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({}));
    Some(json!({
        "type": "tool_use",
        "id": provider_tool_id,
        "name": name,
        "input": input
    }))
}

fn anthropic_image_block(
    url: &str,
    provider: &super::super::types::ProviderId,
) -> Result<Value, LlmError> {
    let clean = url.trim();
    let Some(rest) = clean.strip_prefix("data:") else {
        return Err(LlmError::Config {
            message: format!("{provider} image input must be a base64 data URL."),
        });
    };
    let Some((header, data)) = rest.split_once(',') else {
        return Err(LlmError::Config {
            message: format!("{provider} image data URL is malformed."),
        });
    };
    if !header.to_ascii_lowercase().contains(";base64") {
        return Err(LlmError::Config {
            message: format!("{provider} image data URL must be base64 encoded."),
        });
    }
    let media_type = header.split(';').next().unwrap_or("image/png").trim();
    if media_type.is_empty() || data.trim().is_empty() {
        return Err(LlmError::Config {
            message: format!("{provider} image data URL is missing image data."),
        });
    }
    Ok(json!({
        "type": "image",
        "source": {
            "type": "base64",
            "media_type": media_type,
            "data": data.trim()
        }
    }))
}

fn text_block(text: &str) -> Option<Value> {
    (!text.is_empty()).then(|| json!({ "type": "text", "text": text }))
}

fn content_to_text(content: &Value) -> String {
    match content {
        Value::String(text) => text.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter_map(|part| {
                part.get("text")
                    .and_then(Value::as_str)
                    .or_else(|| part.as_str())
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn normalize_tool_arguments(arguments: &str) -> Result<String, String> {
    if arguments.trim().is_empty() {
        return Ok(json!({}).to_string());
    }
    match serde_json::from_str::<Value>(arguments) {
        Ok(parsed) if parsed.is_object() => Ok(parsed.to_string()),
        Ok(_) => Err("Tool call arguments must be a JSON object.".to_string()),
        Err(error) => Err(format!("Tool call arguments are not valid JSON: {error}")),
    }
}

fn tool_call_value(tool: &AnthropicToolBlock) -> Value {
    json!({
        "id": tool.internal_id,
        "provider_tool_call_id": tool.provider_tool_id,
        "type": "function",
        "function": {
            "name": tool.name,
            "arguments": tool.arguments
        }
    })
}

fn anthropic_finish_reason(reason: &str) -> FinishReason {
    match reason {
        "tool_use" => FinishReason::ToolCalls,
        "end_turn" | "stop_sequence" => FinishReason::Stop,
        "max_tokens" => FinishReason::Length,
        "content_filter" | "refusal" | "safety" => FinishReason::ContentFilter,
        "" => FinishReason::Stop,
        other => FinishReason::Unknown(other.to_string()),
    }
}

fn anthropic_stream_error(provider: &str, error: &Value) -> LlmError {
    let message = error
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| error.as_str())
        .unwrap_or("Provider stream failed.")
        .to_string();
    let kind = error
        .get("type")
        .or_else(|| error.get("code"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();
    if kind.contains("auth") || kind.contains("permission") || kind.contains("forbidden") {
        return LlmError::Auth {
            provider: provider.to_string(),
            message,
        };
    }
    if kind.contains("rate") || kind.contains("quota") {
        return LlmError::RateLimit {
            provider: provider.to_string(),
            message,
            retry_after_ms: None,
        };
    }
    if kind.contains("context") || super::super::error::is_context_overflow_text(&message) {
        return LlmError::ContextOverflow {
            provider: provider.to_string(),
            message,
        };
    }
    LlmError::ProviderApi {
        provider: provider.to_string(),
        status: None,
        message,
        retryable: false,
    }
}

fn block_index(chunk: &Value) -> usize {
    chunk.get("index").and_then(Value::as_u64).unwrap_or(0) as usize
}

fn request_url_summary(url: &str) -> Value {
    let mut clean = url.to_string();
    if clean.len() > 160 {
        clean.truncate(160);
        clean.push_str("...");
    }
    json!(clean)
}

fn provider_request_id(headers: &HeaderMap) -> Option<String> {
    ["x-request-id", "request-id", "cf-ray"]
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

fn record_request_failure(
    trace: Option<&TraceRecorder>,
    attempt: usize,
    status: Option<u16>,
    request_started_at: Instant,
    error: &LlmError,
) {
    let Some(trace) = trace else {
        return;
    };
    trace.error(
        "provider.request.failed",
        "failed",
        json!({
            "attempt": attempt + 1,
            "status": status,
            "retryable": error.is_retryable(),
            "duration_ms": request_started_at.elapsed().as_millis() as i64,
            "error_code": error.code()
        }),
    );
}

fn record_retry(
    trace: Option<&TraceRecorder>,
    attempt: usize,
    status: Option<u16>,
    error: &LlmError,
) {
    let Some(trace) = trace else {
        return;
    };
    trace.warn(
        "provider.request.retry",
        "retrying",
        json!({
            "attempt": attempt + 1,
            "status": status,
            "retryable": true,
            "sleep_ms": PRE_STREAM_RETRY_DELAY.as_millis() as i64,
            "error_code": error.code()
        }),
    );
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

fn empty_completion(finish_reason: FinishReason) -> ChatCompletion {
    ChatCompletion {
        content: String::new(),
        tool_calls: Vec::new(),
        finish_reason,
        tool_call_observation: ToolCallObservation::none(),
    }
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

    #[tokio::test]
    async fn text_stream_emits_text_and_stop_finish() {
        let body = sse(&[
            json!({ "type": "message_start", "message": { "usage": { "input_tokens": 3, "output_tokens": 0 } } }),
            json!({ "type": "content_block_start", "index": 0, "content_block": { "type": "text", "text": "" } }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "text_delta", "text": "hel" } }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "text_delta", "text": "lo" } }),
            json!({ "type": "content_block_stop", "index": 0 }),
            json!({ "type": "message_delta", "delta": { "stop_reason": "end_turn" }, "usage": { "output_tokens": 2 } }),
            json!({ "type": "message_stop" }),
        ]);
        let (addr, server, _) = serve_once(200, &body).await;
        let events = Arc::new(Mutex::new(Vec::new()));

        let completion = stream_chat(&test_request(format!("http://{addr}")), None, {
            let events = Arc::clone(&events);
            move |event| {
                let events = Arc::clone(&events);
                async move {
                    events.lock().await.push(event);
                    Ok(true)
                }
            }
        })
        .await
        .unwrap();

        server.await.unwrap();
        assert_eq!(completion.content, "hello");
        assert_eq!(completion.finish_reason, FinishReason::Stop);
        assert!(events.lock().await.iter().any(|event| {
            matches!(event, StreamEvent::Finish { reason, .. } if reason == &FinishReason::Stop)
        }));
    }

    #[tokio::test]
    async fn tool_use_stream_emits_delta_then_final_tool_call() {
        let body = sse(&[
            json!({ "type": "message_start", "message": {} }),
            json!({
                "type": "content_block_start",
                "index": 0,
                "content_block": { "type": "tool_use", "id": "toolu_1", "name": "read_file", "input": {} }
            }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "input_json_delta", "partial_json": "{\"path\"" } }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "input_json_delta", "partial_json": ":\"res://main.gd\"}" } }),
            json!({ "type": "content_block_stop", "index": 0 }),
            json!({ "type": "message_delta", "delta": { "stop_reason": "tool_use" } }),
            json!({ "type": "message_stop" }),
        ]);
        let (addr, server, _) = serve_once(200, &body).await;
        let events = Arc::new(Mutex::new(Vec::new()));

        let completion = stream_chat(&test_request(format!("http://{addr}")), None, {
            let events = Arc::clone(&events);
            move |event| {
                let events = Arc::clone(&events);
                async move {
                    events.lock().await.push(event);
                    Ok(true)
                }
            }
        })
        .await
        .unwrap();

        server.await.unwrap();
        let events = events.lock().await;
        assert!(events.iter().any(|event| matches!(event, StreamEvent::ToolCallDelta { arguments, .. } if arguments.contains("res://main.gd"))));
        assert!(matches!(
            events.iter().find(|event| matches!(event, StreamEvent::ToolCall { .. })),
            Some(StreamEvent::ToolCall { id, name, arguments, raw }) if
                id.starts_with("call_")
                    && name == "read_file"
                    && arguments == "{\"path\":\"res://main.gd\"}"
                    && raw["provider_tool_call_id"] == json!("toolu_1")
        ));
        assert_eq!(completion.finish_reason, FinishReason::ToolCalls);
        assert_eq!(completion.tool_calls.len(), 1);
    }

    #[tokio::test]
    async fn multiple_tool_use_blocks_finalize_in_order() {
        let body = sse(&[
            json!({ "type": "message_start", "message": {} }),
            json!({ "type": "content_block_start", "index": 0, "content_block": { "type": "tool_use", "id": "toolu_a", "name": "first", "input": {} } }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "input_json_delta", "partial_json": "{}" } }),
            json!({ "type": "content_block_stop", "index": 0 }),
            json!({ "type": "content_block_start", "index": 1, "content_block": { "type": "tool_use", "id": "toolu_b", "name": "second", "input": {} } }),
            json!({ "type": "content_block_delta", "index": 1, "delta": { "type": "input_json_delta", "partial_json": "{}" } }),
            json!({ "type": "content_block_stop", "index": 1 }),
            json!({ "type": "message_delta", "delta": { "stop_reason": "tool_use" } }),
            json!({ "type": "message_stop" }),
        ]);
        let (addr, server, _) = serve_once(200, &body).await;

        let completion = stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap();

        server.await.unwrap();
        assert_eq!(completion.tool_calls.len(), 2);
        assert_eq!(completion.tool_calls[0]["function"]["name"], "first");
        assert_eq!(completion.tool_calls[1]["function"]["name"], "second");
    }

    #[tokio::test]
    async fn malformed_tool_json_emits_malformed_event() {
        let body = sse(&[
            json!({ "type": "message_start", "message": {} }),
            json!({ "type": "content_block_start", "index": 0, "content_block": { "type": "tool_use", "id": "toolu_1", "name": "read_file", "input": {} } }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "input_json_delta", "partial_json": "{\"path\":" } }),
            json!({ "type": "content_block_stop", "index": 0 }),
            json!({ "type": "message_delta", "delta": { "stop_reason": "tool_use" } }),
            json!({ "type": "message_stop" }),
        ]);
        let (addr, server, _) = serve_once(200, &body).await;
        let events = Arc::new(Mutex::new(Vec::new()));

        let completion = stream_chat(&test_request(format!("http://{addr}")), None, {
            let events = Arc::clone(&events);
            move |event| {
                let events = Arc::clone(&events);
                async move {
                    events.lock().await.push(event);
                    Ok(true)
                }
            }
        })
        .await
        .unwrap();

        server.await.unwrap();
        assert!(completion.tool_calls.is_empty());
        assert_eq!(completion.tool_call_observation.malformed.len(), 1);
        assert!(events.lock().await.iter().any(|event| {
            matches!(event, StreamEvent::ToolCallMalformed { message, .. } if message.contains("not valid JSON"))
        }));
    }

    #[test]
    fn request_shape_converts_system_tools_tool_results_and_images() {
        let mut request = test_request("http://127.0.0.1:1".to_string());
        request.messages = vec![
            json!({ "role": "system", "content": "system prompt" }),
            json!({ "role": "user", "content": [
                { "type": "text", "text": "see" },
                { "type": "image_url", "image_url": { "url": "data:image/png;base64,aGVsbG8=" } }
            ] }),
            json!({
                "role": "assistant",
                "content": "",
                "tool_calls": [{
                    "id": "call_internal",
                    "provider_tool_call_id": "toolu_provider",
                    "type": "function",
                    "function": { "name": "read_file", "arguments": "{\"path\":\"res://main.gd\"}" }
                }]
            }),
            json!({ "role": "tool", "tool_call_id": "call_internal", "name": "read_file", "content": "file contents" }),
        ];
        request.tools = vec![json!({
            "type": "function",
            "function": {
                "name": "read_file",
                "description": "Read a file",
                "parameters": { "type": "object", "properties": { "path": { "type": "string" } } }
            }
        })];

        let body = body(&request).unwrap();

        assert_eq!(body["system"], "system prompt");
        assert_eq!(body["messages"][0]["content"][1]["type"], "image");
        assert_eq!(
            body["messages"][0]["content"][1]["source"],
            json!({ "type": "base64", "media_type": "image/png", "data": "aGVsbG8=" })
        );
        assert_eq!(body["messages"][1]["content"][0]["type"], "tool_use");
        assert_eq!(body["messages"][1]["content"][0]["id"], "toolu_provider");
        assert_eq!(body["messages"][2]["content"][0]["type"], "tool_result");
        assert_eq!(
            body["messages"][2]["content"][0]["tool_use_id"],
            "toolu_provider"
        );
        assert_eq!(
            body["tools"][0]["input_schema"]["properties"]["path"]["type"],
            "string"
        );
    }

    #[tokio::test]
    async fn http_401_maps_to_auth_error() {
        let (addr, server, _) = serve_once(401, "{\"error\":{\"message\":\"bad key\"}}").await;

        let error = stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap_err();

        server.await.unwrap();
        assert!(matches!(error, LlmError::Auth { .. }));
    }

    #[tokio::test]
    async fn http_429_maps_to_rate_limit() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for _ in 0..=MAX_PRE_STREAM_RETRIES {
                let (mut socket, _) = listener.accept().await.unwrap();
                read_http_request(&mut socket).await;
                socket
                    .write_all(
                        b"HTTP/1.1 429 Too Many Requests\r\nContent-Length: 35\r\n\r\n{\"error\":{\"message\":\"slow down\"}}",
                    )
                    .await
                    .unwrap();
            }
        });

        let error = stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap_err();

        server.await.unwrap();
        assert!(matches!(error, LlmError::RateLimit { .. }));
    }

    #[tokio::test]
    async fn stream_error_event_maps_to_provider_error() {
        let body = sse(&[json!({
            "type": "error",
            "error": { "type": "rate_limit_error", "message": "rate limited" }
        })]);
        let (addr, server, _) = serve_once(200, &body).await;

        let error = stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap_err();

        server.await.unwrap();
        assert!(matches!(error, LlmError::RateLimit { .. }));
    }

    #[tokio::test]
    async fn malformed_stream_json_is_invalid_provider_output() {
        let body = "data: {not-json}\n\n";
        let (addr, server, _) = serve_once(200, body).await;

        let error = stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap_err();

        server.await.unwrap();
        assert!(matches!(error, LlmError::InvalidProviderOutput { .. }));
    }

    #[tokio::test]
    async fn early_stream_close_before_message_stop_is_invalid_provider_output() {
        let body = sse(&[
            json!({ "type": "message_start", "message": {} }),
            json!({ "type": "content_block_delta", "index": 0, "delta": { "type": "text_delta", "text": "partial" } }),
        ]);
        let (addr, server, _) = serve_once(200, &body).await;

        let error = stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap_err();

        server.await.unwrap();
        assert!(
            matches!(error, LlmError::InvalidProviderOutput { message, .. } if message.contains("message_stop"))
        );
    }

    #[tokio::test]
    async fn sends_bearer_auth_and_anthropic_version_header() {
        let body = sse(&[
            json!({ "type": "message_start", "message": {} }),
            json!({ "type": "message_delta", "delta": { "stop_reason": "end_turn" } }),
            json!({ "type": "message_stop" }),
        ]);
        let (addr, server, captured) = serve_once(200, &body).await;

        stream_chat(&test_request(format!("http://{addr}")), None, |_| async {
            Ok(true)
        })
        .await
        .unwrap();

        server.await.unwrap();
        let request = captured.lock().await.clone().unwrap();
        let request_lower = request.to_ascii_lowercase();
        assert!(request_lower.contains("authorization: bearer test-key"));
        assert!(request_lower.contains("anthropic-version: 2023-06-01"));
        assert!(request.contains("POST /messages HTTP/1.1"));
    }

    #[tokio::test]
    async fn official_anthropic_sends_x_api_key_without_bearer_auth() {
        let body = sse(&[
            json!({ "type": "message_start", "message": {} }),
            json!({ "type": "message_delta", "delta": { "stop_reason": "end_turn" } }),
            json!({ "type": "message_stop" }),
        ]);
        let (addr, server, captured) = serve_once(200, &body).await;
        let mut request = test_request(format!("http://{addr}"));
        request.model.provider.id = ProviderId::unchecked(ProviderId::ANTHROPIC);
        request.model.provider.name = "Anthropic".to_string();
        request.model.provider.auth = Auth::InlineHeader {
            name: "x-api-key".to_string(),
            value: "test-anthropic-key".to_string(),
        };

        stream_chat(&request, None, |_| async { Ok(true) })
            .await
            .unwrap();

        server.await.unwrap();
        let request = captured.lock().await.clone().unwrap();
        let request_lower = request.to_ascii_lowercase();
        assert!(request_lower.contains("x-api-key: test-anthropic-key"));
        assert!(request_lower.contains("anthropic-version: 2023-06-01"));
        assert!(!request_lower.contains("authorization: bearer"));
        assert!(request.contains("POST /messages HTTP/1.1"));
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

    fn sse(chunks: &[Value]) -> String {
        chunks
            .iter()
            .map(|chunk| format!("data: {chunk}\n\n"))
            .collect::<Vec<_>>()
            .join("")
    }

    fn test_request(base_url: String) -> LlmRequest {
        let provider_id = ProviderId::unchecked("test-anthropic");
        let model_id = ModelId::new("test-model").unwrap();
        let provider = ProviderDefinition {
            id: provider_id.clone(),
            name: "Test Anthropic".to_string(),
            adapter: AdapterKind::AnthropicCompatibleMessages,
            base_url: Some(base_url),
            auth: Auth::InlineBearer {
                value: "test-key".to_string(),
            },
            request: RequestDefaults::default(),
            disabled: false,
        };
        let model = ModelDefinition {
            id: model_id.clone(),
            provider: provider_id.clone(),
            display_name: "Test Model".to_string(),
            adapter_model_id: "test-model".to_string(),
            capabilities: Capabilities::text_image_tools_reasoning(),
            limits: Limits {
                context_tokens: Some(200_000),
                input_tokens: None,
                output_tokens: Some(4096),
            },
            request: RequestDefaults {
                generation: GenerationDefaults {
                    temperature: Some(0.7),
                    max_output_tokens: Some(1024),
                    reasoning_effort: None,
                },
                ..RequestDefaults::default()
            },
            enabled: true,
        };
        LlmRequest {
            model: ResolvedModel {
                reference: ModelRef::new(provider_id, model_id),
                provider,
                model,
                request: RequestDefaults::default(),
            },
            messages: vec![
                json!({ "role": "system", "content": "system" }),
                json!({ "role": "user", "content": "hello" }),
            ],
            tools: Vec::new(),
        }
    }
}
