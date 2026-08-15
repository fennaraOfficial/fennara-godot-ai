use axum::{
    Json,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    ffi::OsStr,
    path::{Component, Path},
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

const MAX_RUNTIME_MODEL_IMAGE_COUNT: usize = 6;
const MAX_RUNTIME_MODEL_IMAGE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_RUNTIME_MODEL_IMAGE_TOTAL_BYTES: u64 = 24 * 1024 * 1024;

#[derive(Clone, Copy, Debug)]
enum TargetSelector<'a> {
    GodotEditorSession(&'a str),
    McpProject(&'a str),
    LegacyMcpTarget,
}

#[derive(Clone, Copy, Debug)]
enum BridgeRequestKind {
    ToolCall,
    PluginRequest,
}

#[derive(Clone, Copy, Debug)]
struct BridgeRequestProfile {
    request_id_prefix: &'static str,
    send_failure_message: &'static str,
    disconnected_message: &'static str,
    timeout_message: &'static str,
}

impl BridgeRequestKind {
    fn profile(self) -> BridgeRequestProfile {
        match self {
            Self::ToolCall => BridgeRequestProfile {
                request_id_prefix: "local-tool",
                send_failure_message: "Failed to send tool call to the Godot plugin.",
                disconnected_message: "Godot plugin disconnected before returning a tool result.",
                timeout_message: "Timed out waiting for the Godot plugin tool result.",
            },
            Self::PluginRequest => BridgeRequestProfile {
                request_id_prefix: "local-plugin",
                send_failure_message: "Failed to send request to the Godot plugin.",
                disconnected_message: "Godot plugin disconnected before returning a response.",
                timeout_message: "Timed out waiting for the Godot plugin response.",
            },
        }
    }
}

#[derive(Debug)]
struct BridgeRequest {
    kind: BridgeRequestKind,
    payload: Value,
    timeout: Duration,
    started_at: Instant,
}

impl BridgeRequest {
    fn tool_call(payload: Value) -> Self {
        Self {
            kind: BridgeRequestKind::ToolCall,
            payload,
            timeout: Duration::from_secs(295),
            started_at: Instant::now(),
        }
    }

    fn plugin_request(payload: Value, timeout: Duration) -> Self {
        Self {
            kind: BridgeRequestKind::PluginRequest,
            payload,
            timeout,
            started_at: Instant::now(),
        }
    }

    fn trace_details(&self) -> Value {
        match self.kind {
            BridgeRequestKind::ToolCall => json!({
                "tool": self.payload.get("tool").cloned().unwrap_or(Value::Null),
                "args_bytes": self
                    .payload
                    .get("args")
                    .map(trace::value_size)
                    .unwrap_or_default(),
            }),
            BridgeRequestKind::PluginRequest => json!({
                "request_type": self.payload.get("type").cloned().unwrap_or(Value::Null),
                "payload_bytes": trace::value_size(&self.payload),
            }),
        }
    }

    fn trace_details_with_duration(&self) -> Value {
        let mut details = self.trace_details();
        details["duration_ms"] = json!(self.started_at.elapsed().as_millis() as i64);
        details
    }
}

#[derive(Debug)]
struct RoutingError {
    message: String,
    code: Option<&'static str>,
    retryable: Option<bool>,
}

impl RoutingError {
    fn plain(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
            retryable: None,
        }
    }

    fn bound_project_not_connected() -> Self {
        Self {
            message: "The bound Godot project is not connected. Keep this MCP process running and retry after its editor reconnects.".to_string(),
            code: Some("bound_project_not_connected"),
            retryable: Some(true),
        }
    }

    fn ambiguous_project_binding() -> Self {
        Self {
            message: "More than one connected Godot editor resolves to the bound project. Close the duplicate editor instance before retrying.".to_string(),
            code: Some("ambiguous_project_binding"),
            retryable: Some(false),
        }
    }

    fn legacy_target_ambiguous() -> Self {
        Self {
            message: "Multiple Fennara projects are open. In the Fennara dock, choose Set as MCP target for the project you want to control.".to_string(),
            code: Some("legacy_target_ambiguous"),
            retryable: Some(false),
        }
    }

    fn into_value(self) -> Value {
        let mut value = json!({
            "ok": false,
            "error": self.message,
        });
        if let Some(code) = self.code {
            value["code"] = json!(code);
        }
        if let Some(retryable) = self.retryable {
            value["retryable"] = json!(retryable);
        }
        value
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ToolCallRequest {
    tool: String,
    args: Value,
    #[serde(default)]
    project_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BoundStatusRequest {
    project_path: String,
}

pub(crate) async fn status(State(state): State<AppState>) -> Json<DaemonStatus> {
    Json(current_status(&state).await)
}

pub(crate) async fn bound_status(
    State(state): State<AppState>,
    Json(request): Json<BoundStatusRequest>,
) -> Json<Value> {
    Json(bound_status_value(&state, &request.project_path).await)
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

pub(crate) async fn request_fennara_update(
    state: &AppState,
    session_id: &str,
) -> Result<(), String> {
    let (_, sender) = select_session(state, TargetSelector::GodotEditorSession(session_id))
        .await
        .map_err(|error| error.message)?;
    sender
        .send(Message::Text(
            json!({ "type": "prepare_fennara_update" })
                .to_string()
                .into(),
        ))
        .map_err(|_| {
            "The Godot project disconnected before update preparation started.".to_string()
        })
}

pub(crate) async fn call_tool(
    State(state): State<AppState>,
    Json(request): Json<ToolCallRequest>,
) -> Json<Value> {
    Json(
        call_tool_value_for_project(
            &state,
            request.project_path.as_deref(),
            &request.tool,
            request.args,
        )
        .await,
    )
}

pub(crate) async fn call_tool_value_for_project(
    state: &AppState,
    project_path: Option<&str>,
    tool: &str,
    args: Value,
) -> Value {
    let selector = project_path.map_or(TargetSelector::LegacyMcpTarget, TargetSelector::McpProject);
    call_tool_value_for_target(state, selector, tool, args, None).await
}

pub(crate) async fn call_tool_value_for_session(
    state: &AppState,
    session_id: Option<&str>,
    tool: &str,
    args: Value,
) -> Value {
    let selector = session_id.map_or(
        TargetSelector::LegacyMcpTarget,
        TargetSelector::GodotEditorSession,
    );
    call_tool_value_for_target(state, selector, tool, args, None).await
}

pub(crate) async fn call_tool_value_for_session_traced(
    state: &AppState,
    session_id: Option<&str>,
    tool: &str,
    args: Value,
    trace: Option<&TraceRecorder>,
) -> Value {
    let selector = session_id.map_or(
        TargetSelector::LegacyMcpTarget,
        TargetSelector::GodotEditorSession,
    );
    call_tool_value_for_target(state, selector, tool, args, trace).await
}

async fn call_tool_value_for_target(
    state: &AppState,
    selector: TargetSelector<'_>,
    tool: &str,
    args: Value,
    trace: Option<&TraceRecorder>,
) -> Value {
    let request = BridgeRequest::tool_call(json!({
        "type": "tool_call",
        "tool": tool,
        "args": args
    }));
    let mut response = dispatch_bridge_request(state, selector, request, trace).await;
    attach_runtime_model_images(tool, &mut response).await;
    response
}

async fn attach_runtime_model_images(tool: &str, response: &mut Value) {
    if !matches!(tool, "runtime_session" | "runtime_script")
        || response.get("model_images").is_some()
    {
        return;
    }

    let raw_result = response
        .get("raw_result")
        .cloned()
        .unwrap_or_else(|| response.clone());
    let captures = runtime_capture_candidates(tool, &raw_result);
    if captures.is_empty() {
        return;
    }

    let mut images = Vec::new();
    let mut total_bytes = 0u64;
    for (index, capture) in captures
        .iter()
        .take(MAX_RUNTIME_MODEL_IMAGE_COUNT)
        .enumerate()
    {
        if let Some(image) =
            runtime_capture_model_image(tool, capture, index, &mut total_bytes).await
        {
            images.push(image);
        }
    }

    if !images.is_empty() {
        response["model_images"] = Value::Array(images);
    }
}

fn runtime_capture_candidates(tool: &str, raw_result: &Value) -> Vec<Value> {
    match tool {
        "runtime_session" => {
            if raw_result.get("status").and_then(Value::as_str) != Some("started") {
                return Vec::new();
            }
            raw_result
                .get("startup_capture")
                .filter(|capture| capture.get("success").and_then(Value::as_bool) == Some(true))
                .cloned()
                .into_iter()
                .collect()
        }
        "runtime_script" => raw_result
            .get("captures")
            .or_else(|| raw_result.pointer("/result/captures"))
            .and_then(Value::as_array)
            .map(|captures| {
                captures
                    .iter()
                    .filter(|capture| capture.get("success").and_then(Value::as_bool) == Some(true))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default(),
        _ => Vec::new(),
    }
}

async fn runtime_capture_model_image(
    tool: &str,
    capture: &Value,
    index: usize,
    total_bytes: &mut u64,
) -> Option<Value> {
    let image_path = capture.get("image_path").and_then(Value::as_str)?.trim();
    if image_path.is_empty() {
        return None;
    }
    let path = Path::new(image_path);
    let canonical_path = tokio::fs::canonicalize(path).await.ok()?;
    if !canonical_path.is_file() || !is_fennara_media_path(&canonical_path) {
        return None;
    }
    let metadata = tokio::fs::metadata(&canonical_path).await.ok()?;
    let size = metadata.len();
    if size == 0 || size > MAX_RUNTIME_MODEL_IMAGE_BYTES {
        return None;
    }
    if total_bytes.saturating_add(size) > MAX_RUNTIME_MODEL_IMAGE_TOTAL_BYTES {
        return None;
    }

    let bytes = tokio::fs::read(&canonical_path).await.ok()?;
    if bytes.len() as u64 != size {
        return None;
    }
    let mime_type = detect_image_mime(&bytes)?;
    *total_bytes += size;

    let mut image = json!({
        "data": STANDARD.encode(bytes),
        "mime_type": mime_type,
        "label": runtime_capture_label(tool, capture, index),
        "image_path": image_path,
        "image_role": capture
            .get("image_role")
            .and_then(Value::as_str)
            .unwrap_or(if tool == "runtime_session" { "runtime_startup" } else { "runtime_capture" }),
        "size_bytes": size,
    });
    copy_if_present(capture, &mut image, "image_res_path");
    copy_if_present(capture, &mut image, "width");
    copy_if_present(capture, &mut image, "height");
    copy_if_present(capture, &mut image, "original_width");
    copy_if_present(capture, &mut image, "original_height");
    copy_if_present(capture, &mut image, "session_id");
    copy_if_present(capture, &mut image, "script_run_id");
    copy_if_present(capture, &mut image, "scene_path");
    Some(image)
}

fn runtime_capture_label(tool: &str, capture: &Value, index: usize) -> String {
    let label = capture
        .get("label")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("capture");
    match tool {
        "runtime_session" => "Runtime startup screenshot".to_string(),
        "runtime_script" => format!("Runtime script capture {}: {label}", index + 1),
        _ => format!("Runtime capture {}: {label}", index + 1),
    }
}

fn copy_if_present(source: &Value, target: &mut Value, key: &str) {
    if let Some(value) = source.get(key) {
        target[key] = value.clone();
    }
}

fn detect_image_mime(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("image/png");
    }
    if bytes.starts_with(b"\xff\xd8\xff") {
        return Some("image/jpeg");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    None
}

fn is_fennara_media_path(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(
            component,
            Component::Normal(name) if name.to_string_lossy().eq_ignore_ascii_case(".fennara")
        )
    })
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
    payload: Value,
    timeout: Duration,
    trace: Option<&TraceRecorder>,
) -> Value {
    let selector = session_id.map_or(
        TargetSelector::LegacyMcpTarget,
        TargetSelector::GodotEditorSession,
    );
    let request = BridgeRequest::plugin_request(payload, timeout);
    dispatch_bridge_request(state, selector, request, trace).await
}

async fn dispatch_bridge_request(
    state: &AppState,
    selector: TargetSelector<'_>,
    mut request: BridgeRequest,
    trace: Option<&TraceRecorder>,
) -> Value {
    let profile = request.kind.profile();
    let request_id = format!(
        "{}-{}",
        profile.request_id_prefix,
        state.request_counter.fetch_add(1, Ordering::Relaxed) + 1
    );
    let (session_id, sender) = match select_session(state, selector).await {
        Ok(target) => target,
        Err(error) => {
            if let Some(trace) = trace {
                let mut details = request.trace_details();
                details["message"] = json!(error.message.as_str());
                trace.error("bridge.request.send", "failed", details);
            }
            return error.into_value();
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

    request.payload["request_id"] = json!(request_id);
    request.payload["session_id"] = json!(session_id);

    if sender
        .send(Message::Text(request.payload.to_string().into()))
        .is_err()
    {
        state.pending_tool_calls.write().await.remove(&request_id);
        if let Some(trace) = &bridge_trace {
            let mut details = request.trace_details_with_duration();
            details["message"] = json!("websocket_send_failed");
            trace.error("bridge.request.send", "failed", details);
        }
        return json!({
            "ok": false,
            "error": profile.send_failure_message
        });
    }
    if let Some(trace) = &bridge_trace {
        trace.event_status(
            "bridge.request.send",
            "ok",
            request.trace_details_with_duration(),
        );
    }

    match tokio::time::timeout(request.timeout, response_rx).await {
        Ok(Ok(response)) => {
            if let Some(trace) = &bridge_trace {
                let ok = response.get("ok").and_then(Value::as_bool).unwrap_or(false);
                let mut details = request.trace_details_with_duration();
                details["ok"] = json!(ok);
                details["response_bytes"] = json!(trace::value_size(&response));
                trace.event_status(
                    "bridge.response.received",
                    if ok { "ok" } else { "failed" },
                    details,
                );
            }
            response
        }
        Ok(Err(_)) => {
            if let Some(trace) = &bridge_trace {
                trace.error(
                    "bridge.disconnected",
                    "failed",
                    request.trace_details_with_duration(),
                );
            }
            json!({
                "ok": false,
                "error": profile.disconnected_message
            })
        }
        Err(_) => {
            state.pending_tool_calls.write().await.remove(&request_id);
            if let Some(trace) = &bridge_trace {
                trace.error(
                    "bridge.response.timeout",
                    "timed_out",
                    request.trace_details_with_duration(),
                );
            }
            json!({
                "ok": false,
                "error": profile.timeout_message
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

async fn handle_project_state_message(
    value: &Value,
    state: &AppState,
    session_id: &mut Option<String>,
    fallback_session_id: &str,
    outbound_tx: &mpsc::UnboundedSender<Message>,
) -> bool {
    if value.get("type").and_then(Value::as_str) == Some("hello") {
        let next_session_id =
            optional_string(value, "session_id").unwrap_or_else(|| fallback_session_id.to_string());
        let project = GodotProjectStatus {
            session_id: next_session_id.clone(),
            project_name: optional_string(value, "project_name"),
            project_path: optional_string(value, "project_path"),
            godot_executable_path: optional_string(value, "godot_executable_path"),
            godot_version: optional_string(value, "godot_version"),
            plugin_version: optional_string(value, "plugin_version"),
            rendering_context: value
                .get("rendering_context")
                .filter(|context| context.is_object())
                .cloned(),
            editor_filesystem: value
                .get("editor_filesystem")
                .filter(|status| status.is_object())
                .cloned(),
            chat_token: optional_string(value, "chat_token"),
            tools: string_array(value, "tools"),
        };
        let telemetry_godot_version = project.godot_version.clone();

        *session_id = Some(next_session_id.clone());
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
        if let Some(godot_version) = telemetry_godot_version
            && let Some(telemetry) = state.telemetry.read().await.as_ref()
        {
            telemetry.record_activity(&godot_version);
        }
        ensure_active_project_after_connect(state, &next_session_id).await;
        broadcast_active_project_changed(state).await;
        return true;
    }

    if value.get("type").and_then(Value::as_str) == Some("project_status") {
        if let Some(current_session_id) = session_id.as_deref()
            && value
                .get("session_id")
                .and_then(Value::as_str)
                .is_none_or(|reported| reported == current_session_id)
            && let Some(editor_filesystem) = value
                .get("editor_filesystem")
                .filter(|status| status.is_object())
        {
            let mut projects = state.projects.write().await;
            if let Some(project) = projects.get_mut(current_session_id) {
                project.editor_filesystem = Some(editor_filesystem.clone());
            }
        }
        return true;
    }

    false
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
                    if handle_project_state_message(
                        &value,
                        &state,
                        &mut session_id,
                        &fallback_session_id,
                        &outbound_tx,
                    )
                    .await
                    {
                        continue;
                    }
                    if matches!(
                        value.get("type").and_then(Value::as_str),
                        Some("tool_result" | "snapshot_result" | "project_file_result")
                    ) {
                        handle_tool_result_message(&state, session_id.as_deref(), &value).await;
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

async fn handle_tool_result_message(
    state: &AppState,
    responding_session_id: Option<&str>,
    value: &Value,
) -> bool {
    let (Some(responding_session_id), Some(request_id)) = (
        responding_session_id,
        value.get("request_id").and_then(Value::as_str),
    ) else {
        return false;
    };

    let pending = {
        let mut pending_calls = state.pending_tool_calls.write().await;
        let response_matches_target = pending_calls
            .get(request_id)
            .is_some_and(|pending| pending.session_id == responding_session_id);
        response_matches_target
            .then(|| pending_calls.remove(request_id))
            .flatten()
    };
    let Some(pending) = pending else {
        return false;
    };

    let _ = pending.sender.send(value.clone());
    true
}

#[cfg(test)]
mod tests;

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

async fn bound_status_value(state: &AppState, project_path: &str) -> Value {
    let active_session_id = state.active_session_id.read().await.clone();
    let legacy_active_project = {
        let projects = state.projects.read().await;
        active_session_id
            .as_ref()
            .and_then(|session_id| projects.get(session_id))
            .map(|project| {
                json!({
                    "project_name": project.project_name.clone(),
                    "project_path": project.project_path.clone(),
                })
            })
    };

    let (canonical_project_path, matching_session_ids) =
        match matching_bound_session_ids(state, project_path).await {
            Ok((bound_root, session_ids)) => {
                (bound_root.as_protocol_str().to_string(), session_ids)
            }
            Err(_) => (project_path.to_string(), Vec::new()),
        };
    let selected_project = if let [session_id] = matching_session_ids.as_slice() {
        let sender_connected = state.godot_senders.read().await.contains_key(session_id);
        if sender_connected {
            state.projects.read().await.get(session_id).cloned()
        } else {
            None
        }
    } else {
        None
    };
    let bound_editor_state = if selected_project.is_some() {
        "connected"
    } else if matching_session_ids.len() > 1 {
        "ambiguous"
    } else {
        "not_connected"
    };
    let (routing_code, retryable) = match bound_editor_state {
        "not_connected" => (Some("bound_project_not_connected"), Some(true)),
        "ambiguous" => (Some("ambiguous_project_binding"), Some(false)),
        _ => (None, None),
    };
    let editor_filesystem = selected_project
        .as_ref()
        .and_then(|project| project.editor_filesystem.clone());

    json!({
        "ok": true,
        "daemon": "fennara-daemon",
        "version": DAEMON_VERSION,
        "godot_plugin_connected": !state.godot_senders.read().await.is_empty(),
        "routing_mode": "bound",
        "bound_editor_state": bound_editor_state,
        "bound_project_path": canonical_project_path,
        "code": routing_code,
        "retryable": retryable,
        "selected_project": selected_project,
        "editor_filesystem": editor_filesystem,
        "legacy_active_project": legacy_active_project,
        "legacy_active_session_id": active_session_id,
    })
}

async fn select_session(
    state: &AppState,
    selector: TargetSelector<'_>,
) -> Result<(String, mpsc::UnboundedSender<Message>), RoutingError> {
    match selector {
        TargetSelector::GodotEditorSession(session_id) => {
            let senders = state.godot_senders.read().await;
            senders
                .get(session_id)
                .cloned()
                .map(|sender| (session_id.to_string(), sender))
                .ok_or_else(|| {
                    RoutingError::plain(
                        "The Godot project that owns this chat is no longer connected.",
                    )
                })
        }
        TargetSelector::McpProject(project_path) => {
            select_bound_project_session(state, project_path).await
        }
        TargetSelector::LegacyMcpTarget => select_legacy_mcp_session(state).await,
    }
}

async fn select_legacy_mcp_session(
    state: &AppState,
) -> Result<(String, mpsc::UnboundedSender<Message>), RoutingError> {
    let active_session_id = state.active_session_id.read().await.clone();
    let senders = state.godot_senders.read().await;
    if senders.is_empty() {
        return Err(RoutingError::plain(
            "Open a Godot project with Fennara enabled.",
        ));
    }

    if let Some(active_session_id) = active_session_id
        && let Some(sender) = senders.get(&active_session_id)
    {
        return Ok((active_session_id, sender.clone()));
    }

    if senders.len() == 1 {
        let (session_id, sender) = senders.iter().next().expect("single sender should exist");
        return Ok((session_id.clone(), sender.clone()));
    }

    Err(RoutingError::legacy_target_ambiguous())
}

async fn select_bound_project_session(
    state: &AppState,
    project_path: &str,
) -> Result<(String, mpsc::UnboundedSender<Message>), RoutingError> {
    let (_, matching_session_ids) = matching_bound_session_ids(state, project_path).await?;
    let [session_id] = matching_session_ids.as_slice() else {
        return if matching_session_ids.is_empty() {
            Err(RoutingError::bound_project_not_connected())
        } else {
            Err(RoutingError::ambiguous_project_binding())
        };
    };

    let sender = state
        .godot_senders
        .read()
        .await
        .get(session_id)
        .cloned()
        .ok_or_else(RoutingError::bound_project_not_connected)?;
    Ok((session_id.clone(), sender))
}

async fn matching_bound_session_ids(
    state: &AppState,
    project_path: &str,
) -> Result<(fennara_project_identity::ProjectRoot, Vec<String>), RoutingError> {
    // Explicit configuration errors are rejected by the MCP process at startup.
    // At this seam, a previously valid canonical locator can also be temporarily
    // unavailable (for example, after an unmount), so keep the binding recoverable
    // while still failing closed instead of entering legacy target selection.
    let bound_root =
        fennara_project_identity::ProjectRoot::resolve_absolute(OsStr::new(project_path))
            .map_err(|_| RoutingError::bound_project_not_connected())?;
    let candidates = {
        let connected_session_ids = state
            .godot_senders
            .read()
            .await
            .keys()
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        let projects = state.projects.read().await;
        projects
            .values()
            .filter(|project| connected_session_ids.contains(&project.session_id))
            .filter_map(|project| {
                project
                    .project_path
                    .clone()
                    .map(|path| (project.session_id.clone(), path))
            })
            .collect::<Vec<_>>()
    };

    let matching_session_ids = candidates
        .into_iter()
        .filter_map(|(session_id, path)| {
            fennara_project_identity::ProjectRoot::resolve_absolute(OsStr::new(&path))
                .ok()
                .filter(|candidate| bound_root.same_project(candidate))
                .map(|_| session_id)
        })
        .collect();
    Ok((bound_root, matching_session_ids))
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
