use axum::extract::ws::Message;
use futures_util::Sink;
use serde_json::json;

use crate::runtime_daemon::{godot_bridge, state::AppState};

use super::{BoundChatProject, ClientRequest, checkpoints, send_error, send_json, store};

pub(super) async fn handle_undo<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    request: ClientRequest,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let request_id = request.request_id;
    let Some(chat_id) = request.chat_id.or_else(|| active_chat_id.clone()) else {
        return send_error(sender, request_id, "bad_request", "chat_id is required.").await;
    };
    let Some(user_message_id) = request.user_message_id else {
        return send_error(
            sender,
            request_id,
            "bad_request",
            "user_message_id is required.",
        )
        .await;
    };
    if let Err(error) = store::ensure_chat_in_scope(&bound_project.scope, &chat_id) {
        return send_error(sender, request_id, "chat_scope_mismatch", &error).await;
    }
    match checkpoints::undo_chat_turn(&chat_id, &user_message_id, request.force.unwrap_or(false))
        .await
    {
        Ok(result) => {
            send_result(
                sender,
                active_chat_id,
                state,
                bound_project,
                request_id,
                chat_id,
                result,
            )
            .await
        }
        Err(error) => send_error(sender, request_id, "turn_recovery_failed", &error).await,
    }
}

pub(super) async fn handle_redo<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    request: ClientRequest,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let request_id = request.request_id;
    let Some(chat_id) = request.chat_id.or_else(|| active_chat_id.clone()) else {
        return send_error(sender, request_id, "bad_request", "chat_id is required.").await;
    };
    if let Err(error) = store::ensure_chat_in_scope(&bound_project.scope, &chat_id) {
        return send_error(sender, request_id, "chat_scope_mismatch", &error).await;
    }
    match checkpoints::redo_chat_turn(&chat_id, request.force.unwrap_or(false)).await {
        Ok(result) => {
            send_result(
                sender,
                active_chat_id,
                state,
                bound_project,
                request_id,
                chat_id,
                result,
            )
            .await
        }
        Err(error) => send_error(sender, request_id, "turn_recovery_failed", &error).await,
    }
}

pub(super) async fn handle_resume<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    request: ClientRequest,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    let request_id = request.request_id;
    let Some(chat_id) = request.chat_id.or_else(|| active_chat_id.clone()) else {
        return send_error(sender, request_id, "bad_request", "chat_id is required.").await;
    };
    if let Err(error) = store::ensure_chat_in_scope(&bound_project.scope, &chat_id) {
        return send_error(sender, request_id, "chat_scope_mismatch", &error).await;
    }
    match checkpoints::resume_chat_turn(&chat_id, request.force.unwrap_or(false)).await {
        Ok(result) => {
            send_result(
                sender,
                active_chat_id,
                state,
                bound_project,
                request_id,
                chat_id,
                result,
            )
            .await
        }
        Err(error) => send_error(sender, request_id, "turn_recovery_failed", &error).await,
    }
}

async fn send_result<S>(
    sender: &mut S,
    active_chat_id: &mut Option<String>,
    state: &AppState,
    bound_project: &BoundChatProject,
    request_id: Option<String>,
    chat_id: String,
    result: checkpoints::TurnRecoveryResult,
) -> Result<(), S::Error>
where
    S: Sink<Message> + Unpin,
    S::Error: std::fmt::Debug,
{
    if result.confirmation_required {
        return send_json(
            sender,
            json!({
                "type": "turn_recovery_result",
                "request_id": request_id,
                "result": result
            }),
        )
        .await;
    }
    let opened = match store::open_chat(&bound_project.scope, &chat_id) {
        Ok(opened) => opened,
        Err(error) => return send_error(sender, request_id, "chat_open_failed", &error).await,
    };
    let editor_refresh = if result.project_restored {
        godot_bridge::refresh_project_files_for_session(
            state,
            Some(&bound_project.session_id),
            &result.changed_paths,
        )
        .await
    } else {
        json!({ "ok": true, "refreshed_count": 0 })
    };
    *active_chat_id = Some(chat_id);
    send_json(
        sender,
        json!({
            "type": "turn_recovery_result",
            "request_id": request_id,
            "result": result,
            "editor_refresh": editor_refresh,
            "chat": opened.chat,
            "messages": opened.messages,
            "context_compactions": opened.context_compactions,
            "turn_recovery": opened.turn_recovery
        }),
    )
    .await
}
