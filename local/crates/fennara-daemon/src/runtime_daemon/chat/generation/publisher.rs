use axum::extract::ws::Message;
use futures_util::Sink;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use tokio::sync::{mpsc, oneshot};

use crate::runtime_daemon::state::AppState;

use super::super::{
    providers::{
        ChatCompletion, ChatRequest, FinishReason, LlmError, ProviderSettings, StreamItem,
        stream_chat,
    },
    send_json, tools, trace,
};
use super::is_chat_cancelled;

pub(super) struct StreamedAssistant {
    pub(super) completion: ChatCompletion,
    pub(super) usage: Option<Value>,
    pub(super) reasoning_content: Option<String>,
}

#[derive(Clone, Debug)]
struct ProvisionalTool {
    name: String,
    arguments: String,
    terminal: bool,
    delta_count: usize,
}

pub(super) async fn stream_one_assistant<S>(
    sender: &mut S,
    request_id: Option<String>,
    provider_settings: ProviderSettings,
    model: &str,
    reasoning_effort: &str,
    provider_messages: &[Value],
    assistant_message_id: &str,
    state: &AppState,
    chat_id: &str,
    trace: trace::TraceRecorder,
) -> Result<Result<StreamedAssistant, LlmError>, S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let (item_tx, mut item_rx) = mpsc::unbounded_channel::<StreamItem>();
    let (done_tx, done_rx) = oneshot::channel::<Result<ChatCompletion, LlmError>>();
    let model_for_task = model.to_string();
    let reasoning_effort_for_task = reasoning_effort.to_string();
    let messages_for_task = provider_messages.to_vec();
    let tools_for_task = tools::definitions();
    let state_for_task = state.clone();
    let chat_id_for_task = chat_id.to_string();
    let trace_for_task = trace.clone();

    tokio::spawn(async move {
        let result = stream_chat(
            &provider_settings,
            &ChatRequest {
                model: model_for_task,
                reasoning_effort: reasoning_effort_for_task,
                messages: messages_for_task,
                tools: tools_for_task,
                max_output_tokens: None,
            },
            Some(trace_for_task),
            |item| {
                let item_tx = item_tx.clone();
                let state_for_item = state_for_task.clone();
                let chat_id_for_item = chat_id_for_task.clone();
                async move {
                    if is_chat_cancelled(&state_for_item, &chat_id_for_item).await {
                        return Ok(false);
                    }
                    item_tx.send(item).map_err(|_| LlmError::Config {
                        message: "Chat websocket disconnected.".to_string(),
                    })?;
                    Ok(true)
                }
            },
        )
        .await;
        let _ = done_tx.send(result);
    });

    let mut usage: Option<Value> = None;
    let mut reasoning_content: Option<String> = None;
    let mut provisional_tools: HashMap<String, ProvisionalTool> = HashMap::new();
    let mut done_rx = done_rx;
    let completion = loop {
        tokio::select! {
            item = item_rx.recv() => {
                let Some(item) = item else {
                    continue;
                };
                match item {
                    StreamItem::Text { content, done } => {
                        send_json(
                            sender,
                            json!({
                                "type": "chat_item_update",
                                "request_id": request_id.clone(),
                                "item": {
                                    "id": assistant_message_id,
                                    "type": "message",
                                    "content": content,
                                    "status": if done { "done" } else { "in_progress" }
                                }
                            }),
                        )
                        .await?;
                    }
                    StreamItem::Reasoning { content, done } => {
                        let clean_content = content.trim();
                        if clean_content.is_empty() {
                            continue;
                        }
                        reasoning_content = Some(clean_content.to_string());
                        send_json(
                            sender,
                            json!({
                                "type": "chat_item_update",
                                "request_id": request_id.clone(),
                                "item": {
                                    "id": "reasoning",
                                    "type": "reasoning",
                                    "content": clean_content,
                                    "status": if done { "done" } else { "in_progress" }
                                }
                            }),
                        )
                        .await?;
                    }
                    StreamItem::FunctionCall { id, name, arguments, done } => {
                        let was_new = !provisional_tools.contains_key(&id);
                        let entry = provisional_tools.entry(id.clone()).or_insert_with(|| ProvisionalTool {
                            name: String::new(),
                            arguments: String::new(),
                            terminal: false,
                            delta_count: 0,
                        });
                        entry.delta_count = entry.delta_count.saturating_add(1);
                        if !name.is_empty() {
                            entry.name = name.clone();
                        }
                        entry.arguments = arguments.clone();
                        if done {
                            entry.terminal = true;
                        }
                        let preview_trace = trace.with_provisional_tool(id.clone());
                        preview_trace.event_status(
                            if was_new { "tool.preview.start" } else { "tool.preview.delta" },
                            if done { "queued" } else { "preparing" },
                            json!({
                                "name_present": !entry.name.is_empty(),
                                "tool_name": if entry.name.is_empty() { None } else { Some(entry.name.as_str()) },
                                "arguments_bytes": entry.arguments.len(),
                                "delta_count": entry.delta_count
                            }),
                        );
                        if done {
                            preview_trace
                                .with_tool_call(id.clone())
                                .event_status(
                                    "tool.preview.finalized",
                                    "ok",
                                    json!({
                                        "tool_name": if entry.name.is_empty() { None } else { Some(entry.name.as_str()) },
                                        "arguments_bytes": entry.arguments.len(),
                                        "delta_count": entry.delta_count
                                    }),
                                );
                        }
                        let status = if done { "queued" } else { "preparing" };
                        send_json(
                            sender,
                            json!({
                                "type": "chat_item_update",
                                "request_id": request_id.clone(),
                                "item": {
                                    "id": id,
                                    "type": "function_call",
                                    "name": name,
                                    "arguments": arguments,
                                    "status": status
                                }
                            }),
                        )
                        .await?;
                    }
                    StreamItem::FunctionCallError { id, name, arguments, message } => {
                        provisional_tools.insert(
                            id.clone(),
                            ProvisionalTool {
                                name: name.clone(),
                                arguments: arguments.clone(),
                                terminal: true,
                                delta_count: 1,
                            },
                        );
                        trace
                            .with_provisional_tool(id.clone())
                            .error(
                                "tool.preview.failed",
                                "failed",
                                json!({
                                    "tool_name": if name.is_empty() { None } else { Some(name.as_str()) },
                                    "arguments_bytes": arguments.len(),
                                    "message": message.as_str()
                                }),
                            );
                        send_json(
                            sender,
                            json!({
                                "type": "chat_item_update",
                                "request_id": request_id.clone(),
                                "item": {
                                    "id": id,
                                    "type": "function_call",
                                    "name": name,
                                    "arguments": arguments,
                                    "content": message,
                                    "status": "failed"
                                }
                            }),
                        )
                        .await?;
                    }
                    StreamItem::Status { message } => {
                        send_json(
                            sender,
                            json!({
                                "type": "chat_item_update",
                                "request_id": request_id.clone(),
                                "item": {
                                    "id": "status",
                                    "type": "reasoning",
                                    "content": message,
                                    "status": "in_progress"
                                }
                            }),
                        )
                        .await?;
                    }
                    StreamItem::Usage(next_usage) => {
                        usage = Some(next_usage);
                    }
                }
            }
            result = &mut done_rx => {
                break result.unwrap_or_else(|_| Err(LlmError::ProviderApi {
                    provider: "chat".to_string(),
                    status: None,
                    message: "Chat provider task ended unexpectedly.".to_string(),
                    retryable: false,
                }));
            }
        }
    };

    match completion {
        Ok(completion) => {
            trace.event_status(
                "assistant.finalized",
                "ok",
                json!({
                    "content_chars": completion.content.chars().count(),
                    "finish_reason": trace::finish_reason_label(&completion.finish_reason),
                    "final_tool_call_count": completion.tool_calls.len(),
                    "observed_tool_call_count": completion.tool_call_observation.observed,
                    "malformed_tool_call_count": completion.tool_call_observation.malformed.len()
                }),
            );
            let message = if completion.finish_reason == FinishReason::Cancelled {
                "Tool call cancelled before it finalized."
            } else {
                "Provider response ended before this tool call finalized."
            };
            finalize_matching_provisional_tools(
                sender,
                request_id.clone(),
                &mut provisional_tools,
                &completion.tool_calls,
                &trace,
            )
            .await?;
            fail_open_provisional_tools(
                sender,
                request_id.clone(),
                &mut provisional_tools,
                &trace,
                message,
            )
            .await?;
            Ok(Ok(StreamedAssistant {
                completion,
                usage,
                reasoning_content,
            }))
        }
        Err(error) => {
            trace.error(
                "assistant.finalized",
                "failed",
                json!({ "error_code": error.code() }),
            );
            fail_open_provisional_tools(
                sender,
                request_id.clone(),
                &mut provisional_tools,
                &trace,
                "Provider stream ended before this tool call finalized.",
            )
            .await?;
            Ok(Err(error))
        }
    }
}

async fn finalize_matching_provisional_tools<S>(
    sender: &mut S,
    request_id: Option<String>,
    provisional_tools: &mut HashMap<String, ProvisionalTool>,
    final_tool_calls: &[Value],
    trace: &trace::TraceRecorder,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let mut finalized_ids = HashSet::new();
    for call in final_tool_calls {
        let Some(id) = tool_call_id(call) else {
            continue;
        };
        if !finalized_ids.insert(id.to_string()) {
            continue;
        }
        let Some(tool) = provisional_tools.get_mut(id) else {
            continue;
        };
        let was_terminal = tool.terminal;
        let (name, arguments) = finalize_provisional_tool(tool, call);
        if !was_terminal {
            trace
                .with_provisional_tool(id.to_string())
                .with_tool_call(id.to_string())
                .event_status(
                    "tool.preview.finalized",
                    "ok",
                    json!({
                        "tool_name": name.as_str(),
                        "arguments_bytes": arguments.len(),
                        "delta_count": tool.delta_count
                    }),
                );
        }
        send_json(
            sender,
            json!({
                "type": "chat_item_update",
                "request_id": request_id.clone(),
                "item": {
                    "id": id,
                    "type": "function_call",
                    "name": name,
                    "arguments": arguments,
                    "status": "queued"
                }
            }),
        )
        .await?;
    }
    Ok(())
}

fn finalize_provisional_tool(tool: &mut ProvisionalTool, call: &Value) -> (String, String) {
    let name = tool_call_name(call)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| {
            if tool.name.is_empty() {
                "tool".to_string()
            } else {
                tool.name.clone()
            }
        });
    let arguments = tool_call_arguments(call)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| tool.arguments.clone());
    tool.name = name.clone();
    tool.arguments = arguments.clone();
    tool.terminal = true;
    (name, arguments)
}

async fn fail_open_provisional_tools<S>(
    sender: &mut S,
    request_id: Option<String>,
    provisional_tools: &mut HashMap<String, ProvisionalTool>,
    trace: &trace::TraceRecorder,
    message: &str,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    for (id, tool) in provisional_tools.iter_mut() {
        if tool.terminal {
            continue;
        }
        tool.terminal = true;
        trace.with_provisional_tool(id.clone()).warn(
            "tool.preview.failed",
            "failed",
            json!({
                "tool_name": if tool.name.is_empty() { None } else { Some(tool.name.as_str()) },
                "arguments_bytes": tool.arguments.len(),
                "delta_count": tool.delta_count,
                "message": message
            }),
        );
        send_json(
            sender,
            json!({
                "type": "chat_item_update",
                "request_id": request_id.clone(),
                "item": {
                    "id": id,
                    "type": "function_call",
                    "name": if tool.name.is_empty() { "tool" } else { tool.name.as_str() },
                    "arguments": tool.arguments,
                    "content": message,
                    "status": "failed"
                }
            }),
        )
        .await?;
    }
    Ok(())
}

fn tool_call_id(call: &Value) -> Option<&str> {
    call.get("id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|id| !id.is_empty())
}

fn tool_call_name(call: &Value) -> Option<&str> {
    call.get("function")
        .and_then(|function| function.get("name"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
}

fn tool_call_arguments(call: &Value) -> Option<&str> {
    call.get("function")
        .and_then(|function| function.get("arguments"))
        .and_then(Value::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_tool_call_helpers_extract_normalized_parts() {
        let call = json!({
            "id": " call_1 ",
            "type": "function",
            "function": {
                "name": " exec_command ",
                "arguments": "{\"command\":\"pwd\"}"
            }
        });

        assert_eq!(tool_call_id(&call), Some("call_1"));
        assert_eq!(tool_call_name(&call), Some("exec_command"));
        assert_eq!(tool_call_arguments(&call), Some("{\"command\":\"pwd\"}"));
    }

    #[test]
    fn final_tool_call_reconciles_matching_preview() {
        let call = json!({
            "id": "call_1",
            "type": "function",
            "function": {
                "name": "project_settings",
                "arguments": "{\"action\":\"get\"}"
            }
        });
        let mut preview = ProvisionalTool {
            name: "project_settings".to_string(),
            arguments: "{\"action\":\"get\"".to_string(),
            terminal: false,
            delta_count: 2,
        };

        let (name, arguments) = finalize_provisional_tool(&mut preview, &call);

        assert_eq!(name, "project_settings");
        assert_eq!(arguments, "{\"action\":\"get\"}");
        assert!(preview.terminal);
        assert_eq!(preview.arguments, "{\"action\":\"get\"}");
    }

    #[test]
    fn final_tool_call_helpers_ignore_missing_or_blank_ids() {
        assert_eq!(tool_call_id(&json!({ "id": "  " })), None);
        assert_eq!(tool_call_id(&json!({})), None);
    }
}
