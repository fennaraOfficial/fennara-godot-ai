use axum::extract::ws::Message;
use futures_util::Sink;
use serde_json::{Value, json};
use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};
use tokio::sync::oneshot;

use crate::runtime_daemon::{
    permissions::{
        ApprovalMode, PendingToolApproval, PermissionDecision, PermissionPolicy,
        ToolApprovalRequest, ToolApprovalReview, ToolApprovalStatus, approval_request_payload,
    },
    state::AppState,
};

use super::super::{BoundChatProject, send_chat_list, send_error, send_json, store, tools, trace};
use super::is_chat_cancelled;

pub(super) enum ToolLoopResult {
    Completed { provider_messages: Vec<Value> },
    Stopped,
}

const TOOL_APPROVAL_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const TOOL_APPROVAL_CANCEL_CHECK: Duration = Duration::from_millis(250);

pub(super) async fn run_tool_calls<S>(
    sender: &mut S,
    request_id: Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    scope: &store::ProjectScope,
    chat_id: &str,
    assistant_message_id: &str,
    generation_id: &str,
    assistant_content: &str,
    approval_mode: ApprovalMode,
    tool_calls: Vec<Value>,
    recorder: trace::TraceRecorder,
) -> Result<ToolLoopResult, S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let mut provider_messages = Vec::new();
    for tool_call in tool_calls {
        if is_chat_cancelled(state, chat_id).await {
            let _ = store::finish_generation(generation_id, "cancelled", None);
            finish_cancelled_turn(
                sender,
                request_id,
                state,
                scope,
                chat_id,
                assistant_message_id,
                assistant_content,
            )
            .await?;
            return Ok(ToolLoopResult::Stopped);
        }

        let (tool_call_id, provider_tool_call_id, tool_name) = tool_identity(&tool_call);
        let tool_trace = recorder.with_tool_call(tool_call_id.clone());
        let arguments = match normalize_tool_arguments(&tool_call) {
            Ok(arguments) => arguments,
            Err(error) => {
                tool_trace.error(
                    "tool.args.parse_failed",
                    "failed",
                    json!({
                        "tool_name": tool_name.as_str(),
                        "message": error.as_str(),
                        "tool_call_bytes": trace::value_size(&tool_call)
                    }),
                );
                let arguments = json!({ "error": error });
                let markdown = format!("Tool: {tool_name}\nStatus: failed\nError: {error}");
                let metadata = json!({
                    "tool_name": tool_name,
                    "status": "failed",
                    "error": error
                });
                if let Err(store_error) = store::upsert_tool_call(
                    chat_id,
                    assistant_message_id,
                    Some(generation_id),
                    &tool_call_id,
                    provider_tool_call_id.as_deref(),
                    &tool_name,
                    &arguments,
                    "failed",
                )
                .and_then(|_| {
                    store::finish_tool_call_with_message(
                        chat_id,
                        &tool_call_id,
                        &tool_name,
                        "failed",
                        &json!({ "success": false, "error": error }),
                        &markdown,
                        &markdown,
                        &metadata,
                        &[],
                    )
                    .map(|_| ())
                }) {
                    let error_json = json!({ "message": store_error });
                    let _ = store::finish_generation(generation_id, "failed", Some(&error_json));
                    send_error(sender, request_id, "chat_store_failed", &store_error).await?;
                    return Ok(ToolLoopResult::Stopped);
                }
                send_json(
                    sender,
                    json!({
                        "type": "chat_item_update",
                        "request_id": request_id.clone(),
                        "item": {
                            "id": tool_call_id,
                            "type": "tool_result",
                            "name": tool_name,
                            "content": markdown,
                            "status": "failed"
                        }
                    }),
                )
                .await?;
                provider_messages.push(json!({
                    "role": "tool",
                    "tool_call_id": tool_call_id,
                    "name": tool_name,
                    "content": markdown
                }));
                continue;
            }
        };
        let policy = PermissionPolicy::new(approval_mode);
        let permission = policy.evaluate_tool(&tool_name, &arguments);
        let decision = policy.decide_tool(&tool_name, &arguments);
        tool_trace.event_status(
            "permission.evaluate",
            permission_decision_label(&decision),
            json!({
                "tool_name": tool_name.as_str(),
                "approval_mode": approval_mode.as_str(),
                "tool_kind": permission.kind.label(),
                "arguments_bytes": trace::value_size(&arguments)
            }),
        );
        let approval = if matches!(decision, PermissionDecision::AskUser { .. }) {
            Some(
                create_tool_approval(
                    state,
                    bound_project,
                    chat_id,
                    &tool_call_id,
                    &tool_name,
                    approval_mode,
                    permission.clone(),
                    &arguments,
                )
                .await,
            )
        } else {
            None
        };
        if let Some((approval_request, _)) = approval.as_ref() {
            tool_trace
                .with_approval(approval_request.id.clone())
                .event_status(
                    "permission.requested",
                    "pending",
                    json!({
                        "approval_id": approval_request.id.as_str(),
                        "tool_name": tool_name.as_str(),
                        "summary": approval_request.summary.as_str()
                    }),
                );
        }
        let persisted_tool_status = match decision {
            PermissionDecision::Allow => "in_progress",
            PermissionDecision::AskUser { .. } => ToolApprovalStatus::PendingApproval.as_str(),
            PermissionDecision::Deny { .. } => "denied",
        };
        let live_tool_status = match decision {
            PermissionDecision::Allow => "queued",
            PermissionDecision::AskUser { .. } => ToolApprovalStatus::PendingApproval.as_str(),
            PermissionDecision::Deny { .. } => "denied",
        };
        if let Err(error) = store::upsert_tool_call(
            chat_id,
            assistant_message_id,
            Some(generation_id),
            &tool_call_id,
            provider_tool_call_id.as_deref(),
            &tool_name,
            &arguments,
            persisted_tool_status,
        ) {
            let error_json = json!({ "message": error });
            let _ = store::finish_generation(generation_id, "failed", Some(&error_json));
            tool_trace.error(
                "tool.persisted",
                "failed",
                json!({
                    "tool_name": tool_name.as_str(),
                    "message": error.as_str()
                }),
            );
            send_error(sender, request_id, "chat_store_failed", &error).await?;
            return Ok(ToolLoopResult::Stopped);
        }
        tool_trace.event_status(
            "tool.persisted",
            persisted_tool_status,
            json!({
                "tool_name": tool_name.as_str(),
                "arguments_bytes": trace::value_size(&arguments)
            }),
        );
        send_tool_call_update(
            sender,
            request_id.clone(),
            &tool_call_id,
            &tool_name,
            &arguments,
            live_tool_status,
            approval.as_ref().map(|(request, _)| request),
        )
        .await?;

        match decision {
            PermissionDecision::Allow => {}
            PermissionDecision::Deny { reason } => {
                tool_trace.event_status(
                    "permission.resolved",
                    "denied",
                    json!({
                        "tool_name": tool_name.as_str(),
                        "reason": reason.as_str()
                    }),
                );
                let result = tools::denied_tool(&tool_name, reason);
                let Some(messages) = finish_tool_result(
                    sender,
                    request_id.clone(),
                    chat_id,
                    generation_id,
                    &tool_call_id,
                    &tool_name,
                    result,
                    &tool_trace,
                )
                .await?
                else {
                    return Ok(ToolLoopResult::Stopped);
                };
                provider_messages.extend(messages);
                continue;
            }
            PermissionDecision::AskUser { .. } => {
                let Some((approval_request, approval_rx)) = approval else {
                    let result = tools::failed_tool(
                        &tool_name,
                        "Tool approval could not be created.".to_string(),
                    );
                    let Some(messages) = finish_tool_result(
                        sender,
                        request_id.clone(),
                        chat_id,
                        generation_id,
                        &tool_call_id,
                        &tool_name,
                        result,
                        &tool_trace,
                    )
                    .await?
                    else {
                        return Ok(ToolLoopResult::Stopped);
                    };
                    provider_messages.extend(messages);
                    continue;
                };

                let approval_started_at = Instant::now();
                match wait_for_tool_approval(state, chat_id, &approval_request.id, approval_rx)
                    .await
                {
                    ToolApprovalReview::Approved => {
                        tool_trace
                            .with_approval(approval_request.id.clone())
                            .event_status(
                                "permission.resolved",
                                "approved",
                                json!({
                                    "approval_id": approval_request.id.as_str(),
                                    "wait_ms": approval_started_at.elapsed().as_millis() as i64
                                }),
                            );
                        let _ = store::upsert_tool_call(
                            chat_id,
                            assistant_message_id,
                            Some(generation_id),
                            &tool_call_id,
                            provider_tool_call_id.as_deref(),
                            &tool_name,
                            &arguments,
                            ToolApprovalStatus::Approved.as_str(),
                        );
                        send_tool_call_update(
                            sender,
                            request_id.clone(),
                            &tool_call_id,
                            &tool_name,
                            &arguments,
                            ToolApprovalStatus::Approved.as_str(),
                            Some(&ToolApprovalRequest {
                                status: ToolApprovalStatus::Approved,
                                ..approval_request.clone()
                            }),
                        )
                        .await?;
                        let _ = store::upsert_tool_call(
                            chat_id,
                            assistant_message_id,
                            Some(generation_id),
                            &tool_call_id,
                            provider_tool_call_id.as_deref(),
                            &tool_name,
                            &arguments,
                            "in_progress",
                        );
                        send_tool_call_update(
                            sender,
                            request_id.clone(),
                            &tool_call_id,
                            &tool_name,
                            &arguments,
                            "executing",
                            Some(&ToolApprovalRequest {
                                status: ToolApprovalStatus::Executing,
                                ..approval_request
                            }),
                        )
                        .await?;
                    }
                    ToolApprovalReview::Denied => {
                        tool_trace
                            .with_approval(approval_request.id.clone())
                            .event_status(
                                "permission.resolved",
                                "denied",
                                json!({
                                    "approval_id": approval_request.id.as_str(),
                                    "wait_ms": approval_started_at.elapsed().as_millis() as i64
                                }),
                            );
                        let result = tools::denied_tool(
                            &tool_name,
                            "Tool call denied by the user before execution.".to_string(),
                        );
                        let Some(messages) = finish_tool_result(
                            sender,
                            request_id.clone(),
                            chat_id,
                            generation_id,
                            &tool_call_id,
                            &tool_name,
                            result,
                            &tool_trace,
                        )
                        .await?
                        else {
                            return Ok(ToolLoopResult::Stopped);
                        };
                        provider_messages.extend(messages);
                        continue;
                    }
                    ToolApprovalReview::TimedOut => {
                        tool_trace
                            .with_approval(approval_request.id.clone())
                            .event_status(
                                "permission.resolved",
                                "timed_out",
                                json!({
                                    "approval_id": approval_request.id.as_str(),
                                    "wait_ms": approval_started_at.elapsed().as_millis() as i64
                                }),
                            );
                        let result = tools::timed_out_tool(
                            &tool_name,
                            "Tool approval timed out before execution.".to_string(),
                        );
                        let Some(messages) = finish_tool_result(
                            sender,
                            request_id.clone(),
                            chat_id,
                            generation_id,
                            &tool_call_id,
                            &tool_name,
                            result,
                            &tool_trace,
                        )
                        .await?
                        else {
                            return Ok(ToolLoopResult::Stopped);
                        };
                        provider_messages.extend(messages);
                        continue;
                    }
                    ToolApprovalReview::Cancelled => {
                        tool_trace
                            .with_approval(approval_request.id.clone())
                            .event_status(
                                "permission.resolved",
                                "cancelled",
                                json!({
                                    "approval_id": approval_request.id.as_str(),
                                    "wait_ms": approval_started_at.elapsed().as_millis() as i64
                                }),
                            );
                        let result = tools::cancelled_tool(
                            &tool_name,
                            "Tool call cancelled before execution.".to_string(),
                        );
                        let Some(_messages) = finish_tool_result(
                            sender,
                            request_id.clone(),
                            chat_id,
                            generation_id,
                            &tool_call_id,
                            &tool_name,
                            result,
                            &tool_trace,
                        )
                        .await?
                        else {
                            return Ok(ToolLoopResult::Stopped);
                        };
                        let _ = store::finish_generation(generation_id, "cancelled", None);
                        finish_cancelled_turn(
                            sender,
                            request_id,
                            state,
                            scope,
                            chat_id,
                            assistant_message_id,
                            assistant_content,
                        )
                        .await?;
                        return Ok(ToolLoopResult::Stopped);
                    }
                }
            }
        }

        send_tool_call_update(
            sender,
            request_id.clone(),
            &tool_call_id,
            &tool_name,
            &arguments,
            "executing",
            None,
        )
        .await?;
        let tool_exec_started_at = Instant::now();
        tool_trace.event_status(
            "tool.exec.start",
            "running",
            json!({
                "tool_name": tool_name.as_str(),
                "target": if tool_name == "exec_command" { "daemon" } else { "godot_bridge" },
                "arguments_bytes": trace::value_size(&arguments)
            }),
        );
        let result = tools::execute(
            state,
            chat_id,
            &bound_project.session_id,
            bound_project.scope.project_path.as_deref(),
            &tool_name,
            &arguments,
            Some(&tool_trace),
        )
        .await;
        let result_status = terminal_status_for_result(&result);
        tool_trace.event_status(
            "tool.exec.end",
            &result_status,
            json!({
                "tool_name": tool_name.as_str(),
                "ok": result.ok,
                "duration_ms": tool_exec_started_at.elapsed().as_millis() as i64,
                "raw_result_bytes": trace::value_size(&result.raw_result),
                "plugin_markdown_bytes": result.plugin_markdown.len(),
                "mcp_markdown_bytes": result.mcp_markdown.len()
            }),
        );
        let cancelled_after_execute = is_chat_cancelled(state, chat_id).await;

        let Some(messages) = finish_tool_result(
            sender,
            request_id.clone(),
            chat_id,
            generation_id,
            &tool_call_id,
            &tool_name,
            result,
            &tool_trace,
        )
        .await?
        else {
            return Ok(ToolLoopResult::Stopped);
        };
        if cancelled_after_execute {
            let _ = store::finish_generation(generation_id, "cancelled", None);
            finish_cancelled_turn(
                sender,
                request_id,
                state,
                scope,
                chat_id,
                assistant_message_id,
                assistant_content,
            )
            .await?;
            return Ok(ToolLoopResult::Stopped);
        }
        provider_messages.extend(messages);
    }

    Ok(ToolLoopResult::Completed { provider_messages })
}

async fn create_tool_approval(
    state: &AppState,
    bound_project: &BoundChatProject,
    chat_id: &str,
    tool_call_id: &str,
    tool_name: &str,
    approval_mode: ApprovalMode,
    permission: crate::runtime_daemon::permissions::ToolPermission,
    arguments: &Value,
) -> (ToolApprovalRequest, oneshot::Receiver<ToolApprovalReview>) {
    let approval_id = format!(
        "tool-approval-{}",
        state.request_counter.fetch_add(1, Ordering::Relaxed) + 1
    );
    let (approval_tx, approval_rx) = oneshot::channel();
    let request = ToolApprovalRequest {
        id: approval_id.clone(),
        chat_id: chat_id.to_string(),
        session_id: bound_project.session_id.clone(),
        tool_call_id: tool_call_id.to_string(),
        tool_name: tool_name.to_string(),
        tool_kind: permission.kind,
        tool_kind_label: permission.kind.label(),
        approval_mode,
        status: ToolApprovalStatus::PendingApproval,
        reason: permission.reason,
        summary: tool_summary(tool_name, arguments),
    };
    state.pending_tool_approvals.write().await.insert(
        approval_id,
        PendingToolApproval {
            request: request.clone(),
            responder: approval_tx,
        },
    );
    (request, approval_rx)
}

async fn wait_for_tool_approval(
    state: &AppState,
    chat_id: &str,
    approval_id: &str,
    mut approval_rx: oneshot::Receiver<ToolApprovalReview>,
) -> ToolApprovalReview {
    wait_for_tool_approval_with_timeout(
        state,
        chat_id,
        approval_id,
        &mut approval_rx,
        TOOL_APPROVAL_TIMEOUT,
    )
    .await
}

async fn wait_for_tool_approval_with_timeout(
    state: &AppState,
    chat_id: &str,
    approval_id: &str,
    approval_rx: &mut oneshot::Receiver<ToolApprovalReview>,
    timeout: Duration,
) -> ToolApprovalReview {
    let timeout = tokio::time::sleep(timeout);
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            biased;

            review = &mut *approval_rx => {
                let review = review.unwrap_or(ToolApprovalReview::Cancelled);
                if matches!(review, ToolApprovalReview::Cancelled) {
                    remove_pending_tool_approval(state, approval_id).await;
                }
                return review;
            }
            _ = &mut timeout => {
                remove_pending_tool_approval(state, approval_id).await;
                return ToolApprovalReview::TimedOut;
            }
            _ = tokio::time::sleep(TOOL_APPROVAL_CANCEL_CHECK) => {
                if is_chat_cancelled(state, chat_id).await {
                    remove_pending_tool_approval(state, approval_id).await;
                    return ToolApprovalReview::Cancelled;
                }
            }
        }
    }
}

async fn remove_pending_tool_approval(state: &AppState, approval_id: &str) {
    state
        .pending_tool_approvals
        .write()
        .await
        .remove(approval_id);
}

async fn send_tool_call_update<S>(
    sender: &mut S,
    request_id: Option<String>,
    tool_call_id: &str,
    tool_name: &str,
    arguments: &Value,
    status: &str,
    approval: Option<&ToolApprovalRequest>,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
{
    let mut item = json!({
        "id": tool_call_id,
        "type": "function_call",
        "name": tool_name,
        "arguments": arguments.to_string(),
        "status": status
    });
    if let Some(approval) = approval {
        item["approval"] = approval_request_payload(approval);
    }
    send_json(
        sender,
        json!({
            "type": "chat_item_update",
            "request_id": request_id,
            "item": item
        }),
    )
    .await
}

async fn finish_tool_result<S>(
    sender: &mut S,
    request_id: Option<String>,
    chat_id: &str,
    generation_id: &str,
    tool_call_id: &str,
    tool_name: &str,
    result: tools::ExecutedTool,
    recorder: &trace::TraceRecorder,
) -> Result<Option<Vec<Value>>, S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let status = terminal_status_for_result(&result);
    let persist_span = recorder.start_span(
        "db.write",
        json!({
            "action": "finish_tool_result",
            "tool_call_id": tool_call_id,
            "tool_name": tool_name,
            "status": status.as_str()
        }),
    );
    if let Err(error) = store::finish_tool_call_with_message(
        chat_id,
        tool_call_id,
        tool_name,
        &status,
        &result.raw_result,
        &result.mcp_markdown,
        &result.plugin_markdown,
        &result.metadata,
        &result.target_keys,
    ) {
        let error_json = json!({ "message": error });
        let _ = store::finish_generation(generation_id, "failed", Some(&error_json));
        persist_span.fail(json!({ "message": error.as_str() }));
        recorder.error(
            "tool.result.persisted",
            "failed",
            json!({
                "tool_name": tool_name,
                "status": status.as_str(),
                "message": error.as_str()
            }),
        );
        send_error(sender, request_id, "chat_store_failed", &error).await?;
        return Ok(None);
    }
    persist_span.finish(
        "ok",
        json!({
            "tool_name": tool_name,
            "status": status.as_str(),
            "plugin_markdown_bytes": result.plugin_markdown.len(),
            "mcp_markdown_bytes": result.mcp_markdown.len()
        }),
    );
    recorder.event_status(
        "tool.result.persisted",
        &status,
        json!({
            "tool_name": tool_name,
            "plugin_markdown_bytes": result.plugin_markdown.len(),
            "mcp_markdown_bytes": result.mcp_markdown.len()
        }),
    );
    send_json(
        sender,
        json!({
            "type": "chat_item_update",
            "request_id": request_id.clone(),
            "item": {
                "id": tool_call_id,
                "type": "tool_result",
                "name": tool_name,
                "content": result.plugin_markdown,
                "status": status
            }
        }),
    )
    .await?;

    let mut messages = vec![json!({
        "role": "tool",
        "tool_call_id": tool_call_id,
        "name": tool_name,
        "content": result.mcp_markdown
    })];
    messages.extend(result.model_followup_messages);
    Ok(Some(messages))
}

fn terminal_status_for_result(result: &tools::ExecutedTool) -> String {
    if result.ok {
        return "done".to_string();
    }

    let raw_status = result
        .metadata
        .get("status")
        .or_else(|| result.raw_result.get("status"))
        .and_then(Value::as_str)
        .unwrap_or("failed");
    match raw_status {
        "timed_out" | "timeout" | "approval_timed_out" => "timed_out",
        "cancelled" | "canceled" => "cancelled",
        "denied" | "permission_denied" => "denied",
        _ => "failed",
    }
    .to_string()
}

fn permission_decision_label(decision: &PermissionDecision) -> &'static str {
    match decision {
        PermissionDecision::Allow => "allowed",
        PermissionDecision::AskUser { .. } => "requested",
        PermissionDecision::Deny { .. } => "denied",
    }
}

fn tool_summary(tool_name: &str, arguments: &Value) -> String {
    let fields = match tool_name {
        "write_or_update_file" => &[("path", "path"), ("mode", "mode")][..],
        "run_scene_edit_script" => &[("scene_path", "scene"), ("script_path", "script")][..],
        "save_custom_resource" => &[("resource_path", "resource"), ("script_path", "script")][..],
        "project_settings" => &[("action", "action"), ("key", "key"), ("prefix", "prefix")][..],
        "runtime_session" => &[("action", "action"), ("scene_path", "scene")][..],
        "runtime_script" => &[("session_id", "session")][..],
        "exec_command" => &[("command", "command"), ("cwd", "cwd"), ("shell", "shell")][..],
        _ => &[][..],
    };
    let parts = fields
        .iter()
        .filter_map(|(key, label)| {
            arguments
                .get(*key)
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .map(|value| format!("{label}: {}", compact(value)))
        })
        .collect::<Vec<_>>();
    if parts.is_empty() {
        tool_name.to_string()
    } else {
        format!("{tool_name} ({})", parts.join(", "))
    }
}

fn compact(value: &str) -> String {
    const MAX_CHARS: usize = 160;
    let clean = value.trim().replace('\n', " ");
    if clean.chars().count() <= MAX_CHARS {
        return clean;
    }
    let mut truncated = clean.chars().take(MAX_CHARS).collect::<String>();
    truncated.push_str("...");
    truncated
}

pub(super) async fn finish_cancelled_turn<S>(
    sender: &mut S,
    request_id: Option<String>,
    state: &AppState,
    scope: &store::ProjectScope,
    chat_id: &str,
    assistant_message_id: &str,
    partial: &str,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let stored = match store::cancel_turn(chat_id, assistant_message_id, partial) {
        Ok(message) => message,
        Err(error) => {
            return send_error(sender, request_id, "chat_store_failed", &error).await;
        }
    };
    state.cancelled_chats.write().await.remove(chat_id);
    send_json(
        sender,
        json!({
            "type": "chat_cancelled",
            "request_id": request_id.clone(),
            "chat_id": chat_id,
            "message": stored,
            "response": partial
        }),
    )
    .await?;
    send_chat_list(sender, None, scope).await
}

fn tool_identity(tool_call: &Value) -> (String, Option<String>, String) {
    let tool_call_id = tool_call
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("tool_call")
        .to_string();
    let provider_tool_call_id = tool_call
        .get("provider_tool_call_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned);
    let function = tool_call.get("function").unwrap_or(&Value::Null);
    let tool_name = function
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    (tool_call_id, provider_tool_call_id, tool_name)
}

fn normalize_tool_arguments(tool_call: &Value) -> Result<Value, String> {
    let function = tool_call.get("function").unwrap_or(&Value::Null);
    let raw = function
        .get("arguments")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if raw.trim().is_empty() {
        return Ok(json!({}));
    }
    let value = serde_json::from_str::<Value>(raw)
        .map_err(|error| format!("Tool call arguments are not valid JSON: {error}"))?;
    if value.is_object() {
        Ok(value)
    } else {
        Err("Tool call arguments must be a JSON object.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::{permissions::ToolPermissionKind, state::AppState};

    #[test]
    fn malformed_tool_arguments_do_not_become_empty_object() {
        let tool_call = json!({
            "id": "call_1",
            "function": {
                "name": "read_file",
                "arguments": "{\"path\":"
            }
        });

        let error = normalize_tool_arguments(&tool_call).unwrap_err();

        assert!(error.contains("not valid JSON"));
    }

    #[test]
    fn empty_tool_arguments_remain_empty_object() {
        let tool_call = json!({
            "id": "call_1",
            "function": {
                "name": "read_file",
                "arguments": ""
            }
        });

        assert_eq!(normalize_tool_arguments(&tool_call).unwrap(), json!({}));
    }

    #[tokio::test]
    async fn approval_wait_times_out_and_removes_pending_request() {
        let (shutdown_tx, _shutdown_rx) = oneshot::channel();
        let state = AppState::new(shutdown_tx);
        let approval_id = "approval-timeout";
        let chat_id = "chat-timeout";
        let (approval_tx, mut approval_rx) = oneshot::channel();
        let request = ToolApprovalRequest {
            id: approval_id.to_string(),
            chat_id: chat_id.to_string(),
            session_id: "session-timeout".to_string(),
            tool_call_id: "call-timeout".to_string(),
            tool_name: "write_or_update_file".to_string(),
            tool_kind: ToolPermissionKind::MutatesProject,
            tool_kind_label: ToolPermissionKind::MutatesProject.label(),
            approval_mode: ApprovalMode::Ask,
            status: ToolApprovalStatus::PendingApproval,
            reason: "test".to_string(),
            summary: "test".to_string(),
        };
        state.pending_tool_approvals.write().await.insert(
            approval_id.to_string(),
            PendingToolApproval {
                request,
                responder: approval_tx,
            },
        );

        let review = wait_for_tool_approval_with_timeout(
            &state,
            chat_id,
            approval_id,
            &mut approval_rx,
            Duration::from_millis(1),
        )
        .await;

        assert_eq!(review, ToolApprovalReview::TimedOut);
        assert!(
            !state
                .pending_tool_approvals
                .read()
                .await
                .contains_key(approval_id)
        );
    }
}
