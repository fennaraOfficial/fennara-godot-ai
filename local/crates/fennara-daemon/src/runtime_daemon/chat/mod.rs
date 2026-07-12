use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures_util::{Sink, SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use tokio::sync::broadcast;

use super::{DAEMON_HOST, DAEMON_PORT, permissions::ToolApprovalReview, state::AppState};
use crate::runtime_daemon::godot_bridge;

mod assets;
mod auth;
pub(crate) mod context;
mod context_compaction;
mod exec_command;
mod generation;
mod ids;
mod images;
mod models;
mod prompt;
mod providers;
mod schema;
mod settings;
mod store;
mod tools;
pub(crate) mod trace;

pub(crate) use assets::{chat_asset, chat_index, chat_index_redirect, chat_tool_media};
use settings::{SaveSettingsRequest, load_settings, save_settings};

#[derive(Debug, Deserialize)]
pub(crate) struct ChatWsQuery {
    chat_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClientRequest {
    #[serde(rename = "type")]
    request_type: String,
    request_id: Option<String>,
    chat_id: Option<String>,
    message: Option<String>,
    images: Option<Vec<images::ClientImage>>,
    context_snippets: Option<Vec<context::ClientContextSnippet>>,
    path: Option<String>,
    start_line: Option<u32>,
    end_line: Option<u32>,
    model: Option<String>,
    reasoning_effort: Option<String>,
    openrouter_api_key: Option<String>,
    ollama_cloud_api_key: Option<String>,
    provider_api_keys: Option<BTreeMap<String, String>>,
    ollama_base_url: Option<String>,
    provider_base_urls: Option<BTreeMap<String, String>>,
    approval_mode: Option<String>,
    local_model_context_lengths: Option<BTreeMap<String, u32>>,
    approval_id: Option<String>,
    decision: Option<String>,
    force: Option<bool>,
    refresh_local: Option<bool>,
    chat_surface: Option<String>,
}

#[derive(Clone)]
struct BoundChatProject {
    session_id: String,
    scope: store::ProjectScope,
}

pub(crate) async fn chat_ws(
    ws: WebSocketUpgrade,
    Query(query): Query<ChatWsQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Response {
    if !is_allowed_browser_origin(&headers) {
        return StatusCode::FORBIDDEN.into_response();
    }
    let Some(bound_project) = project_for_chat_token(&state, query.chat_token.as_deref()).await
    else {
        return StatusCode::FORBIDDEN.into_response();
    };
    ws.on_upgrade(move |socket| handle_chat_socket(socket, state, bound_project))
        .into_response()
}

pub(crate) async fn chat_traces(Query(query): Query<trace::TraceQuery>) -> axum::Json<Value> {
    if !query.has_filter() {
        return axum::Json(json!({
            "ok": false,
            "error": "Pass chat_id, trace_id, turn_id, or generation_id."
        }));
    }
    match trace::list_events(&query) {
        Ok(events) => axum::Json(json!({
            "ok": true,
            "events": events
        })),
        Err(error) => axum::Json(json!({
            "ok": false,
            "error": error
        })),
    }
}

fn is_allowed_browser_origin(headers: &HeaderMap) -> bool {
    let Some(origin) = headers.get("origin").and_then(|value| value.to_str().ok()) else {
        return true;
    };
    origin == "null" || origin.starts_with("file://") || is_local_daemon_origin(origin)
}

fn is_local_daemon_origin(origin: &str) -> bool {
    origin == format!("http://{DAEMON_HOST}:{DAEMON_PORT}")
        || origin == format!("https://{DAEMON_HOST}:{DAEMON_PORT}")
        || origin == format!("http://localhost:{DAEMON_PORT}")
        || origin == format!("https://localhost:{DAEMON_PORT}")
}

async fn project_for_chat_token(state: &AppState, token: Option<&str>) -> Option<BoundChatProject> {
    let Some(token) = token.filter(|value| !value.is_empty()) else {
        return None;
    };
    state
        .projects
        .read()
        .await
        .values()
        .find(|project| project.chat_token.as_deref() == Some(token))
        .map(|project| BoundChatProject {
            session_id: project.session_id.clone(),
            scope: store::ProjectScope {
                project_path: project.project_path.clone(),
                project_name: project.project_name.clone(),
            },
        })
}

async fn handle_chat_socket(socket: WebSocket, state: AppState, bound_project: BoundChatProject) {
    let (mut sender, mut receiver) = socket.split();
    let mut active_chat_id: Option<String> = None;
    let mut context_receiver = state.chat_context_sender.subscribe();

    if send_initial_state(&mut sender, &mut active_chat_id, &state, &bound_project)
        .await
        .is_err()
    {
        return;
    }

    loop {
        tokio::select! {
            message = receiver.next() => {
                let Some(message) = message else {
                    break;
                };
                match message {
                    Ok(Message::Text(text)) => {
                        let Ok(request) = serde_json::from_str::<ClientRequest>(&text) else {
                            let _ =
                                send_error(&mut sender, None, "bad_request", "Invalid chat request.").await;
                            continue;
                        };
                        if handle_request(
                            &mut sender,
                            &mut active_chat_id,
                            &state,
                            &bound_project,
                            request,
                        )
                        .await
                        .is_err()
                        {
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            received = context_receiver.recv() => {
                match received {
                    Ok(snippet) => {
                        if snippet.session_id == bound_project.session_id &&
                            send_json(&mut sender, snippet.to_client_message()).await.is_err()
                        {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {}
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
}

async fn send_initial_state<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let settings = load_settings();
    models::spawn_catalog_refresh_if_needed();
    send_json(
        sender,
        json!({
            "type": "settings",
            "request_id": null,
            "settings": settings.public()
        }),
    )
    .await?;
    send_project_status(sender, None, state, bound_project).await?;
    let scope = &bound_project.scope;
    match store::open_active_or_create(scope, &settings.model, &settings.reasoning_effort) {
        Ok(opened) => {
            *active_chat_id = Some(opened.chat.id.clone());
            send_json(
                sender,
                json!({
                    "type": "chat_opened",
                    "request_id": null,
                    "chat": opened.chat,
                    "messages": opened.messages,
                    "context_compactions": opened.context_compactions,
                    "can_revert": chat_can_revert(state, &opened.chat.id).await
                }),
            )
            .await?;
        }
        Err(error) => {
            send_error(sender, None, "chat_store_failed", &error).await?;
        }
    }
    send_chat_list(sender, None, scope).await
}

async fn handle_request<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    request: ClientRequest,
) -> Result<(), ()>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let request_id = request.request_id.clone();
    match request.request_type.as_str() {
        "get_settings" => {
            send_json(
                sender,
                json!({
                    "type": "settings",
                    "request_id": request_id,
                    "settings": load_settings().public()
                }),
            )
            .await
        }
        "get_project_status" => send_project_status(sender, request_id, state, bound_project).await,
        "prepare_fennara_update" => {
            match godot_bridge::request_fennara_update(state, &bound_project.session_id).await {
                Ok(()) => {
                    send_json(
                        sender,
                        json!({
                            "type": "fennara_update_requested",
                            "request_id": request_id,
                            "ok": true
                        }),
                    )
                    .await
                }
                Err(error) => send_error(sender, request_id, "update_start_failed", &error).await,
            }
        }
        "set_mcp_target" => {
            match godot_bridge::set_active_project_session(state, &bound_project.session_id).await {
                Ok(()) => send_project_status(sender, request_id, state, bound_project).await,
                Err(error) => send_error(sender, request_id, "target_set_failed", &error).await,
            }
        }
        "open_project_file" => {
            let Some(path) = request.path.as_deref() else {
                return send_error(sender, request_id, "bad_request", "path is required.")
                    .await
                    .map_err(|_| ());
            };
            let result = godot_bridge::open_project_file_for_session(
                state,
                Some(&bound_project.session_id),
                path,
                request.start_line,
                request.end_line,
            )
            .await;
            if result.get("ok").and_then(Value::as_bool) == Some(false) {
                let error = result
                    .get("error")
                    .and_then(Value::as_str)
                    .unwrap_or("Could not open that project file.");
                return send_error(sender, request_id, "project_file_open_failed", error)
                    .await
                    .map_err(|_| ());
            }
            send_json(sender, project_file_opened_message(request_id, result)).await
        }
        "list_chats" => {
            let scope = &bound_project.scope;
            send_chat_list(sender, request_id, scope).await
        }
        "list_models" => {
            let settings = load_settings();
            let catalog =
                models::list_models(&settings, request.refresh_local.unwrap_or(true)).await;
            send_json(
                sender,
                json!({
                    "type": "model_list",
                    "request_id": request_id,
                    "catalog": catalog
                }),
            )
            .await
        }
        "refresh_model_catalog" => {
            match models::refresh_model_catalog(request.force.unwrap_or(true)).await {
                Ok(status) => {
                    send_json(
                        sender,
                        json!({
                            "type": "catalog_refresh_result",
                            "request_id": request_id,
                            "ok": true,
                            "status": status
                        }),
                    )
                    .await
                }
                Err(error) => {
                    let status = models::list_models(&load_settings(), false)
                        .await
                        .catalog_status;
                    send_json(
                        sender,
                        json!({
                            "type": "catalog_refresh_result",
                            "request_id": request_id,
                            "ok": false,
                            "error": {
                                "code": "catalog_fetch_failed",
                                "message": "Could not refresh model catalog. Using cached models."
                            },
                            "detail": error,
                            "status": status
                        }),
                    )
                    .await
                }
            }
        }
        "open_chat" => {
            let Some(chat_id) = request.chat_id.as_deref() else {
                return send_error(sender, request_id, "bad_request", "chat_id is required.")
                    .await
                    .map_err(|_| ());
            };
            let scope = &bound_project.scope;
            match store::open_chat(scope, chat_id) {
                Ok(opened) => {
                    *active_chat_id = Some(opened.chat.id.clone());
                    send_json(
                        sender,
                        json!({
                            "type": "chat_opened",
                            "request_id": request_id,
                            "chat": opened.chat,
                            "messages": opened.messages,
                            "context_compactions": opened.context_compactions,
                            "can_revert": chat_can_revert(state, &opened.chat.id).await
                        }),
                    )
                    .await
                }
                Err(error) => send_error(sender, request_id, "chat_open_failed", &error).await,
            }
        }
        "save_settings" => {
            let update = SaveSettingsRequest {
                openrouter_api_key: request.openrouter_api_key,
                ollama_cloud_api_key: request.ollama_cloud_api_key,
                provider_api_keys: request.provider_api_keys,
                ollama_base_url: request.ollama_base_url,
                provider_base_urls: request.provider_base_urls,
                model: request.model,
                reasoning_effort: request.reasoning_effort,
                local_model_context_lengths: request.local_model_context_lengths,
                chat_surface: request.chat_surface,
                approval_mode: request.approval_mode,
            };
            match save_settings(update) {
                Ok(settings) => {
                    models::spawn_catalog_refresh_if_needed();
                    send_json(
                        sender,
                        json!({
                            "type": "settings_saved",
                            "request_id": request_id,
                            "settings": settings.public()
                        }),
                    )
                    .await
                }
                Err(error) => send_error(sender, request_id, "settings_save_failed", &error).await,
            }
        }
        "new_chat" => {
            let settings = load_settings();
            let model = request
                .model
                .as_deref()
                .and_then(settings::clean_model)
                .unwrap_or_else(|| settings.model.clone());
            let reasoning_effort = settings::clean_reasoning_effort(
                request
                    .reasoning_effort
                    .as_deref()
                    .unwrap_or(&settings.reasoning_effort),
            )
            .to_string();
            let scope = &bound_project.scope;
            match store::create_chat(scope, &model, &reasoning_effort) {
                Ok(opened) => {
                    *active_chat_id = Some(opened.chat.id.clone());
                    if send_json(
                        sender,
                        json!({
                            "type": "chat_created",
                            "request_id": request_id.clone(),
                            "chat": opened.chat
                        }),
                    )
                    .await
                    .is_err()
                    {
                        return Err(());
                    }
                    if send_json(
                        sender,
                        json!({
                            "type": "chat_opened",
                            "request_id": request_id,
                            "chat": opened.chat,
                            "messages": opened.messages,
                            "context_compactions": opened.context_compactions,
                            "can_revert": false
                        }),
                    )
                    .await
                    .is_err()
                    {
                        return Err(());
                    }
                    send_chat_list(sender, None, scope).await
                }
                Err(error) => send_error(sender, request_id, "chat_create_failed", &error).await,
            }
        }
        "delete_chat" => {
            let Some(chat_id) = request.chat_id.as_deref() else {
                return send_error(sender, request_id, "bad_request", "chat_id is required.")
                    .await
                    .map_err(|_| ());
            };
            let scope = &bound_project.scope;
            match store::archive_chat(scope, chat_id) {
                Ok(()) => {
                    if active_chat_id.as_deref() == Some(chat_id) {
                        *active_chat_id = None;
                    }
                    state.revertable_chats.write().await.remove(chat_id);
                    send_chat_list(sender, request_id, scope).await
                }
                Err(error) => send_error(sender, request_id, "chat_delete_failed", &error).await,
            }
        }
        "cancel_chat" => {
            let Some(chat_id) = request.chat_id.or_else(|| active_chat_id.clone()) else {
                return send_error(sender, request_id, "bad_request", "chat_id is required.")
                    .await
                    .map_err(|_| ());
            };
            state.cancelled_chats.write().await.insert(chat_id);
            send_json(
                sender,
                json!({
                    "type": "chat_cancel_requested",
                    "request_id": request_id
                }),
            )
            .await
        }
        "review_tool_approval" => {
            let Some(approval_id) = request.approval_id.as_deref() else {
                return send_error(
                    sender,
                    request_id,
                    "bad_request",
                    "approval_id is required.",
                )
                .await
                .map_err(|_| ());
            };
            let Some(decision) = parse_tool_approval_review(request.decision.as_deref()) else {
                return send_error(
                    sender,
                    request_id,
                    "bad_request",
                    "decision must be approved, denied, or cancelled.",
                )
                .await
                .map_err(|_| ());
            };
            match resolve_tool_approval(state, &bound_project.session_id, approval_id, decision)
                .await
            {
                Ok(()) => {
                    send_json(
                        sender,
                        json!({
                            "type": "tool_approval_reviewed",
                            "request_id": request_id,
                            "approval_id": approval_id,
                            "decision": decision
                        }),
                    )
                    .await
                }
                Err(error) => {
                    send_error(sender, request_id, "approval_review_failed", &error).await
                }
            }
        }
        "revert_chat" => {
            let Some(chat_id) = request.chat_id.or_else(|| active_chat_id.clone()) else {
                return send_error(sender, request_id, "bad_request", "chat_id is required.")
                    .await
                    .map_err(|_| ());
            };
            let scope = &bound_project.scope;
            if let Err(error) = store::ensure_chat_in_scope(scope, &chat_id) {
                return send_error(sender, request_id, "chat_scope_mismatch", &error)
                    .await
                    .map_err(|_| ());
            }
            if !chat_can_revert(state, &chat_id).await {
                return send_error(
                    sender,
                    request_id,
                    "revert_unavailable",
                    "No active live revert snapshot is available for this chat.",
                )
                .await
                .map_err(|_| ());
            }
            let fallback_restored_message = store::last_user_message_content(&chat_id)
                .ok()
                .flatten()
                .unwrap_or_default();
            let snapshot_result = godot_bridge::revert_snapshot_turn_for_session(
                state,
                Some(&bound_project.session_id),
                &chat_id,
            )
            .await;
            if snapshot_result.get("ok").and_then(Value::as_bool) == Some(false) {
                let error = snapshot_result
                    .get("error")
                    .and_then(Value::as_str)
                    .unwrap_or("Failed to revert the last file snapshot.");
                return send_error(sender, request_id, "revert_failed", error)
                    .await
                    .map_err(|_| ());
            }
            match store::revert_last_turn(scope, &chat_id) {
                Ok(opened) => {
                    state.revertable_chats.write().await.remove(&chat_id);
                    let restored_message = snapshot_result
                        .get("restored_message")
                        .and_then(Value::as_str)
                        .filter(|message| !message.is_empty())
                        .unwrap_or(&fallback_restored_message);
                    *active_chat_id = Some(opened.chat.id.clone());
                    if send_json(
                        sender,
                        json!({
                            "type": "chat_opened",
                            "request_id": request_id.clone(),
                            "chat": opened.chat,
                            "messages": opened.messages,
                            "context_compactions": opened.context_compactions,
                            "can_revert": false,
                            "reverted": true,
                            "restored_message": restored_message
                        }),
                    )
                    .await
                    .is_err()
                    {
                        return Err(());
                    }
                    send_chat_list(sender, None, scope).await
                }
                Err(error) => send_error(sender, request_id, "revert_failed", &error).await,
            }
        }
        "send_chat" => {
            generation::runner::run_chat(sender, active_chat_id, state, bound_project, request)
                .await
        }
        _ => {
            send_error(
                sender,
                request_id,
                "bad_request",
                "Unsupported chat request type.",
            )
            .await
        }
    }
    .map_err(|_| ())
}

fn parse_tool_approval_review(decision: Option<&str>) -> Option<ToolApprovalReview> {
    match decision?.trim().to_ascii_lowercase().as_str() {
        "approved" | "approve" => Some(ToolApprovalReview::Approved),
        "denied" | "deny" => Some(ToolApprovalReview::Denied),
        "cancelled" | "canceled" | "cancel" => Some(ToolApprovalReview::Cancelled),
        _ => None,
    }
}

async fn resolve_tool_approval(
    state: &AppState,
    session_id: &str,
    approval_id: &str,
    decision: ToolApprovalReview,
) -> Result<(), String> {
    let pending = state
        .pending_tool_approvals
        .write()
        .await
        .remove(approval_id)
        .ok_or_else(|| "That approval request is no longer pending.".to_string())?;
    if pending.request.session_id != session_id {
        state
            .pending_tool_approvals
            .write()
            .await
            .insert(approval_id.to_string(), pending);
        return Err("That approval request belongs to another Godot session.".to_string());
    }
    pending
        .responder
        .send(decision)
        .map_err(|_| "The waiting tool call is no longer active.".to_string())
}

fn project_file_opened_message(request_id: Option<String>, result: Value) -> Value {
    let mut payload = json!({
        "type": "project_file_opened",
        "request_id": request_id,
    });
    if let (Some(payload), Some(result)) = (payload.as_object_mut(), result.as_object()) {
        for (key, value) in result {
            if key != "type" && key != "request_id" {
                payload.insert(key.clone(), value.clone());
            }
        }
    }
    payload
}

async fn send_project_status<S>(
    sender: &mut S,
    request_id: Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let status = godot_bridge::current_status_value(state).await;
    let version_status = godot_bridge::call_tool_value_for_session(
        state,
        Some(&bound_project.session_id),
        "fennara_status",
        json!({}),
    )
    .await
    .get("result")
    .and_then(|result| result.get("version"))
    .cloned()
    .unwrap_or_else(|| json!({}));

    send_json(
        sender,
        json!({
            "type": "project_status",
            "request_id": request_id,
            "bound_session_id": bound_project.session_id,
            "daemon": status,
            "version": version_status
        }),
    )
    .await
}

async fn chat_can_revert(state: &AppState, chat_id: &str) -> bool {
    state.revertable_chats.read().await.contains(chat_id)
}

async fn send_chat_list<S>(
    sender: &mut S,
    request_id: Option<String>,
    scope: &store::ProjectScope,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    match store::list_chats(scope) {
        Ok(chats) => {
            send_json(
                sender,
                json!({
                    "type": "chat_list",
                    "request_id": request_id,
                    "chats": chats
                }),
            )
            .await
        }
        Err(error) => send_error(sender, request_id, "chat_list_failed", &error).await,
    }
}

async fn send_chat_updated<S>(
    sender: &mut S,
    request_id: Option<String>,
    chat_id: &str,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    match store::chat_summary(chat_id) {
        Ok(chat) => {
            send_json(
                sender,
                json!({
                    "type": "chat_updated",
                    "request_id": request_id,
                    "chat": chat
                }),
            )
            .await
        }
        Err(error) => send_error(sender, request_id, "chat_store_failed", &error).await,
    }
}

async fn send_json<S>(sender: &mut S, payload: Value) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
{
    sender.send(Message::Text(payload.to_string().into())).await
}

async fn send_error<S>(
    sender: &mut S,
    request_id: Option<String>,
    code: &str,
    message: &str,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
{
    send_json(
        sender,
        json!({
            "type": "error",
            "request_id": request_id,
            "code": code,
            "message": message
        }),
    )
    .await
}
