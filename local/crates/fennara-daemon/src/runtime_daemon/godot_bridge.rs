use axum::{
    Json,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};
use tokio::sync::{mpsc, oneshot};

use super::{
    DAEMON_VERSION,
    chat::{
        context::ChatContextSnippet,
        trace::{self, TraceRecorder},
    },
    docs_cache::handle_docs_warmup_request,
    state::{AppState, DaemonStatus, GodotProjectStatus, PendingToolCall},
    util::{optional_string, string_array},
};

#[derive(Debug, Deserialize)]
pub(crate) struct ToolCallRequest {
    tool: String,
    args: Value,
}

pub(crate) async fn status(State(state): State<AppState>) -> Json<DaemonStatus> {
    Json(current_status(&state).await)
}

pub(crate) async fn current_status_value(state: &AppState) -> Value {
    serde_json::to_value(current_status(state).await).unwrap_or_else(|_| {
        json!({
            "ok": false,
            "error": "Failed to serialize daemon status."
        })
    })
}

pub(crate) async fn set_active_project_session(
    state: &AppState,
    session_id: &str,
) -> Result<(), String> {
    if !state.projects.read().await.contains_key(session_id) {
        return Err("That Godot project is no longer connected.".to_string());
    }
    *state.active_session_id.write().await = Some(session_id.to_string());
    *state.active_project_explicit.write().await = true;
    broadcast_active_project_changed(state).await;
    Ok(())
}

pub(crate) async fn call_tool(
    State(state): State<AppState>,
    Json(request): Json<ToolCallRequest>,
) -> Json<Value> {
    Json(call_tool_value(&state, &request.tool, request.args).await)
}

pub(crate) async fn call_tool_value(state: &AppState, tool: &str, args: Value) -> Value {
    call_tool_value_for_session(state, None, tool, args).await
}

pub(crate) async fn call_tool_value_for_session(
    state: &AppState,
    session_id: Option<&str>,
    tool: &str,
    args: Value,
) -> Value {
    call_tool_value_for_session_traced(state, session_id, tool, args, None).await
}

pub(crate) async fn call_tool_value_for_session_traced(
    state: &AppState,
    session_id: Option<&str>,
    tool: &str,
    args: Value,
    trace: Option<&TraceRecorder>,
) -> Value {
    let started_at = Instant::now();
    let request_id = format!(
        "local-tool-{}",
        state.request_counter.fetch_add(1, Ordering::Relaxed) + 1
    );
    let (session_id, sender) = match select_session(state, session_id).await {
        Ok(target) => target,
        Err(error) => {
            if let Some(trace) = trace {
                trace.error(
                    "bridge.request.send",
                    "failed",
                    json!({
                        "tool": tool,
                        "args_bytes": trace::value_size(&args),
                        "message": error.as_str()
                    }),
                );
            }
            return json!({ "ok": false, "error": error });
        }
    };
    let bridge_trace =
        trace.map(|trace| trace.with_bridge_request(request_id.clone(), session_id.clone()));

    let (response_tx, response_rx) = oneshot::channel();
    state.pending_tool_calls.write().await.insert(
        request_id.clone(),
        PendingToolCall {
            session_id: session_id.clone(),
            sender: response_tx,
        },
    );

    let payload = json!({
        "type": "tool_call",
        "request_id": request_id,
        "session_id": session_id,
        "tool": tool,
        "args": args
    });

    if sender
        .send(Message::Text(payload.to_string().into()))
        .is_err()
    {
        state.pending_tool_calls.write().await.remove(&request_id);
        if let Some(trace) = &bridge_trace {
            trace.error(
                "bridge.request.send",
                "failed",
                json!({
                    "tool": tool,
                    "args_bytes": trace::value_size(&args),
                    "duration_ms": started_at.elapsed().as_millis() as i64,
                    "message": "websocket_send_failed"
                }),
            );
        }
        return json!({
            "ok": false,
            "error": "Failed to send tool call to the Godot plugin."
        });
    }
    if let Some(trace) = &bridge_trace {
        trace.event_status(
            "bridge.request.send",
            "ok",
            json!({
                "tool": tool,
                "args_bytes": trace::value_size(&args),
                "duration_ms": started_at.elapsed().as_millis() as i64
            }),
        );
    }

    match tokio::time::timeout(Duration::from_secs(295), response_rx).await {
        Ok(Ok(response)) => {
            if let Some(trace) = &bridge_trace {
                let ok = response.get("ok").and_then(Value::as_bool).unwrap_or(false);
                trace.event_status(
                    "bridge.response.received",
                    if ok { "ok" } else { "failed" },
                    json!({
                        "tool": tool,
                        "ok": ok,
                        "duration_ms": started_at.elapsed().as_millis() as i64,
                        "response_bytes": trace::value_size(&response)
                    }),
                );
            }
            response
        }
        Ok(Err(_)) => {
            if let Some(trace) = &bridge_trace {
                trace.error(
                    "bridge.disconnected",
                    "failed",
                    json!({
                        "tool": tool,
                        "duration_ms": started_at.elapsed().as_millis() as i64
                    }),
                );
            }
            json!({
                "ok": false,
                "error": "Godot plugin disconnected before returning a tool result."
            })
        }
        Err(_) => {
            state.pending_tool_calls.write().await.remove(&request_id);
            if let Some(trace) = &bridge_trace {
                trace.error(
                    "bridge.response.timeout",
                    "timed_out",
                    json!({
                        "tool": tool,
                        "duration_ms": started_at.elapsed().as_millis() as i64
                    }),
                );
            }
            json!({
                "ok": false,
                "error": "Timed out waiting for the Godot plugin tool result."
            })
        }
    }
}

pub(crate) async fn begin_snapshot_turn_for_session_traced(
    state: &AppState,
    session_id: Option<&str>,
    chat_id: &str,
    user_message: &str,
    trace: Option<&TraceRecorder>,
) -> Value {
    call_plugin_request(
        state,
        session_id,
        json!({
            "type": "snapshot_begin_turn",
            "chat_id": chat_id,
            "user_message": user_message
        }),
        Duration::from_secs(10),
        trace,
    )
    .await
}

pub(crate) async fn revert_snapshot_turn_for_session(
    state: &AppState,
    session_id: Option<&str>,
    chat_id: &str,
) -> Value {
    call_plugin_request(
        state,
        session_id,
        json!({
            "type": "snapshot_revert",
            "chat_id": chat_id
        }),
        Duration::from_secs(30),
        None,
    )
    .await
}

pub(crate) async fn open_project_file_for_session(
    state: &AppState,
    session_id: Option<&str>,
    path: &str,
    start_line: Option<u32>,
    end_line: Option<u32>,
) -> Value {
    call_plugin_request(
        state,
        session_id,
        json!({
            "type": "open_project_file",
            "path": path,
            "start_line": start_line,
            "end_line": end_line
        }),
        Duration::from_secs(10),
        None,
    )
    .await
}

async fn call_plugin_request(
    state: &AppState,
    session_id: Option<&str>,
    mut payload: Value,
    timeout: Duration,
    trace: Option<&TraceRecorder>,
) -> Value {
    let started_at = Instant::now();
    let request_id = format!(
        "local-plugin-{}",
        state.request_counter.fetch_add(1, Ordering::Relaxed) + 1
    );
    let (session_id, sender) = match select_session(state, session_id).await {
        Ok(target) => target,
        Err(error) => {
            if let Some(trace) = trace {
                trace.error(
                    "bridge.request.send",
                    "failed",
                    json!({
                        "request_type": payload.get("type").and_then(Value::as_str),
                        "payload_bytes": trace::value_size(&payload),
                        "message": error.as_str()
                    }),
                );
            }
            return json!({ "ok": false, "error": error });
        }
    };
    let bridge_trace =
        trace.map(|trace| trace.with_bridge_request(request_id.clone(), session_id.clone()));

    let (response_tx, response_rx) = oneshot::channel();
    state.pending_tool_calls.write().await.insert(
        request_id.clone(),
        PendingToolCall {
            session_id: session_id.clone(),
            sender: response_tx,
        },
    );

    payload["request_id"] = json!(request_id);
    payload["session_id"] = json!(session_id);

    if sender
        .send(Message::Text(payload.to_string().into()))
        .is_err()
    {
        state.pending_tool_calls.write().await.remove(
            payload
                .get("request_id")
                .and_then(Value::as_str)
                .unwrap_or_default(),
        );
        if let Some(trace) = &bridge_trace {
            trace.error(
                "bridge.request.send",
                "failed",
                json!({
                    "request_type": payload.get("type").and_then(Value::as_str),
                    "payload_bytes": trace::value_size(&payload),
                    "duration_ms": started_at.elapsed().as_millis() as i64,
                    "message": "websocket_send_failed"
                }),
            );
        }
        return json!({
            "ok": false,
            "error": "Failed to send request to the Godot plugin."
        });
    }
    if let Some(trace) = &bridge_trace {
        trace.event_status(
            "bridge.request.send",
            "ok",
            json!({
                "request_type": payload.get("type").and_then(Value::as_str),
                "payload_bytes": trace::value_size(&payload),
                "duration_ms": started_at.elapsed().as_millis() as i64
            }),
        );
    }

    match tokio::time::timeout(timeout, response_rx).await {
        Ok(Ok(response)) => {
            if let Some(trace) = &bridge_trace {
                let ok = response.get("ok").and_then(Value::as_bool).unwrap_or(false);
                trace.event_status(
                    "bridge.response.received",
                    if ok { "ok" } else { "failed" },
                    json!({
                        "request_type": payload.get("type").and_then(Value::as_str),
                        "ok": ok,
                        "duration_ms": started_at.elapsed().as_millis() as i64,
                        "response_bytes": trace::value_size(&response)
                    }),
                );
            }
            response
        }
        Ok(Err(_)) => {
            if let Some(trace) = &bridge_trace {
                trace.error(
                    "bridge.disconnected",
                    "failed",
                    json!({
                        "request_type": payload.get("type").and_then(Value::as_str),
                        "duration_ms": started_at.elapsed().as_millis() as i64
                    }),
                );
            }
            json!({
                "ok": false,
                "error": "Godot plugin disconnected before returning a response."
            })
        }
        Err(_) => {
            state.pending_tool_calls.write().await.remove(
                payload
                    .get("request_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            );
            if let Some(trace) = &bridge_trace {
                trace.error(
                    "bridge.response.timeout",
                    "timed_out",
                    json!({
                        "request_type": payload.get("type").and_then(Value::as_str),
                        "duration_ms": started_at.elapsed().as_millis() as i64
                    }),
                );
            }
            json!({
                "ok": false,
                "error": "Timed out waiting for the Godot plugin response."
            })
        }
    }
}

pub(crate) async fn godot_ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_godot_socket(socket, state))
}

async fn handle_godot_socket(socket: WebSocket, state: AppState) {
    let connection_id = state.connection_counter.fetch_add(1, Ordering::Relaxed) + 1;
    let fallback_session_id = format!("connection-{connection_id}");
    let mut session_id: Option<String> = None;
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (outbound_tx, mut outbound_rx) = mpsc::unbounded_channel::<Message>();

    let writer = tokio::spawn(async move {
        while let Some(message) = outbound_rx.recv().await {
            if ws_sender.send(message).await.is_err() {
                break;
            }
        }
    });

    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(Message::Text(text)) => {
                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    if value.get("type").and_then(Value::as_str) == Some("hello") {
                        let next_session_id = optional_string(&value, "session_id")
                            .unwrap_or_else(|| fallback_session_id.clone());
                        let project = GodotProjectStatus {
                            session_id: next_session_id.clone(),
                            project_name: optional_string(&value, "project_name"),
                            project_path: optional_string(&value, "project_path"),
                            godot_executable_path: optional_string(&value, "godot_executable_path"),
                            godot_version: optional_string(&value, "godot_version"),
                            plugin_version: optional_string(&value, "plugin_version"),
                            rendering_context: value
                                .get("rendering_context")
                                .filter(|context| context.is_object())
                                .cloned(),
                            chat_token: optional_string(&value, "chat_token"),
                            tools: string_array(&value, "tools"),
                        };

                        session_id = Some(next_session_id.clone());
                        state
                            .godot_senders
                            .write()
                            .await
                            .insert(next_session_id.clone(), outbound_tx.clone());
                        state
                            .projects
                            .write()
                            .await
                            .insert(next_session_id.clone(), project);
                        ensure_active_project_after_connect(&state, &next_session_id).await;
                        broadcast_active_project_changed(&state).await;
                    } else if matches!(
                        value.get("type").and_then(Value::as_str),
                        Some("tool_result" | "snapshot_result" | "project_file_result")
                    ) {
                        if let Some(request_id) = value.get("request_id").and_then(Value::as_str) {
                            if let Some(pending) =
                                state.pending_tool_calls.write().await.remove(request_id)
                            {
                                let _ = pending.sender.send(value);
                            }
                        }
                    } else if value.get("type").and_then(Value::as_str)
                        == Some("set_active_project")
                    {
                        if let Some(next_session_id) = value
                            .get("session_id")
                            .and_then(Value::as_str)
                            .or(session_id.as_deref())
                        {
                            let _ = set_active_project_session(&state, next_session_id).await;
                        }
                    } else if value.get("type").and_then(Value::as_str)
                        == Some("chat_context_snippet")
                    {
                        if let Some(snippet) =
                            ChatContextSnippet::from_godot_message(&value, session_id.as_deref())
                            && session_id.as_deref() == Some(snippet.session_id.as_str())
                        {
                            let _ = state.chat_context_sender.send(snippet);
                        }
                    } else if value.get("type").and_then(Value::as_str)
                        == Some("warm_get_class_info_docs")
                    {
                        handle_docs_warmup_request(&state, &value).await;
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => {}
            Err(_) => break,
        }
    }

    writer.abort();
    if let Some(session_id) = session_id {
        state.godot_senders.write().await.remove(&session_id);
        state.projects.write().await.remove(&session_id);

        let mut active = state.active_session_id.write().await;
        if active.as_deref() == Some(session_id.as_str()) {
            *active = None;
        }
        drop(active);

        let pending = {
            let mut pending = state.pending_tool_calls.write().await;
            let ids: Vec<String> = pending
                .iter()
                .filter_map(|(request_id, call)| {
                    (call.session_id == session_id).then(|| request_id.clone())
                })
                .collect();
            ids.into_iter()
                .filter_map(|request_id| pending.remove(&request_id))
                .collect::<Vec<_>>()
        };
        for pending in pending {
            let _ = pending.sender.send(json!({
                "ok": false,
                "error": "Godot plugin disconnected."
            }));
        }

        normalize_active_project_after_disconnect(&state).await;
        broadcast_active_project_changed(&state).await;
        schedule_idle_shutdown_if_empty(state.clone()).await;
    }
}

async fn current_status(state: &AppState) -> DaemonStatus {
    let projects = state.projects.read().await;
    let active_session_id = state.active_session_id.read().await.clone();
    let mut connected_projects: Vec<GodotProjectStatus> = projects.values().cloned().collect();
    connected_projects.sort_by(|a, b| {
        a.project_name
            .clone()
            .unwrap_or_default()
            .cmp(&b.project_name.clone().unwrap_or_default())
    });
    let active_project = active_session_id
        .as_ref()
        .and_then(|session_id| projects.get(session_id))
        .cloned();

    DaemonStatus {
        ok: true,
        daemon: "fennara-daemon",
        version: DAEMON_VERSION,
        godot_plugin_connected: !projects.is_empty(),
        active_project,
        active_session_id,
        connected_projects,
    }
}

async fn select_session(
    state: &AppState,
    requested_session_id: Option<&str>,
) -> Result<(String, mpsc::UnboundedSender<Message>), String> {
    let senders = state.godot_senders.read().await;
    if senders.is_empty() {
        return Err("Open a Godot project with Fennara enabled.".to_string());
    }

    if let Some(session_id) = requested_session_id {
        if let Some(sender) = senders.get(session_id) {
            return Ok((session_id.to_string(), sender.clone()));
        }
        return Err("The Godot project that owns this chat is no longer connected.".to_string());
    }

    if let Some(active_session_id) = state.active_session_id.read().await.clone() {
        if let Some(sender) = senders.get(&active_session_id) {
            return Ok((active_session_id, sender.clone()));
        }
    }

    if senders.len() == 1 {
        let (session_id, sender) = senders.iter().next().expect("single sender should exist");
        return Ok((session_id.clone(), sender.clone()));
    }

    Err("Multiple Fennara projects are open. In the Fennara dock, choose Set as MCP target for the project you want to control.".to_string())
}

async fn ensure_active_project_after_connect(state: &AppState, session_id: &str) {
    let project_count = state.projects.read().await.len();
    let mut active = state.active_session_id.write().await;
    let mut explicit = state.active_project_explicit.write().await;

    if project_count == 1 {
        *active = Some(session_id.to_string());
        *explicit = false;
    } else if !*explicit {
        *active = None;
    } else if active.is_none() {
        *active = Some(session_id.to_string());
    }
}

async fn normalize_active_project_after_disconnect(state: &AppState) {
    let projects = state.projects.read().await;
    let mut active = state.active_session_id.write().await;
    let mut explicit = state.active_project_explicit.write().await;

    if projects.len() == 1 {
        *active = projects.keys().next().cloned();
        *explicit = false;
    } else if active
        .as_ref()
        .is_some_and(|session_id| !projects.contains_key(session_id))
    {
        *active = None;
        *explicit = false;
    }
}

async fn broadcast_active_project_changed(state: &AppState) {
    let active_session_id = state.active_session_id.read().await.clone();
    let active_project = {
        let projects = state.projects.read().await;
        active_session_id
            .as_ref()
            .and_then(|session_id| projects.get(session_id))
            .cloned()
    };
    let senders = state.godot_senders.read().await;
    for (session_id, sender) in senders.iter() {
        let payload = json!({
            "type": "active_project_changed",
            "active_session_id": active_session_id,
            "active_project_name": active_project.as_ref().and_then(|project| project.project_name.clone()),
            "active_project_path": active_project.as_ref().and_then(|project| project.project_path.clone()),
            "session_id": session_id,
            "is_active": active_session_id.as_deref() == Some(session_id.as_str())
        });
        let _ = sender.send(Message::Text(payload.to_string().into()));
    }
}

async fn schedule_idle_shutdown_if_empty(state: AppState) {
    if !state.projects.read().await.is_empty() {
        return;
    }

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(8)).await;
        if !state.projects.read().await.is_empty() {
            return;
        }

        if let Some(sender) = state.shutdown_sender.lock().await.take() {
            let _ = sender.send(());
        }
    });
}
