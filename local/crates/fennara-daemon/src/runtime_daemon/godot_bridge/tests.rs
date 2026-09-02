use super::{
    BoundStatusRequest, Message, bound_status, call_tool_value_for_project,
    call_tool_value_for_session, handle_project_state_message, handle_tool_result_message,
};
use crate::runtime_daemon::{
    control_auth::{self, CONTROL_HEADER},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::State,
    middleware,
    routing::{get, post},
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Message as WebSocketMessage, client::IntoClientRequest},
};

static NEXT_FIXTURE_ID: AtomicU64 = AtomicU64::new(1);

struct ProjectFixture {
    root: PathBuf,
}

impl ProjectFixture {
    fn new(name: &str) -> Self {
        let unique = NEXT_FIXTURE_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "fennara-daemon-routing-{}-{unique}-{name}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("create project fixture");
        fs::write(root.join("project.godot"), "[application]\n").expect("write project.godot");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }

    fn protocol_path(&self) -> &str {
        self.root.to_str().expect("test root should be Unicode")
    }
}

impl Drop for ProjectFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

async fn connect_project(
    state: &AppState,
    session_id: &str,
    project_name: &str,
    project_path: &str,
) -> mpsc::UnboundedReceiver<Message> {
    let (sender, receiver) = mpsc::unbounded_channel::<Message>();
    let mut connected_session_id = None;
    assert!(
        handle_project_state_message(
            &json!({
                "type": "hello",
                "session_id": session_id,
                "project_name": project_name,
                "project_path": project_path,
                "editor_filesystem": {
                    "status": "ready",
                    "asset_tools_ready": true
                },
                "tools": []
            }),
            state,
            &mut connected_session_id,
            "fallback",
            &sender,
        )
        .await
    );
    assert_eq!(connected_session_id.as_deref(), Some(session_id));
    receiver
}

async fn answer_next_tool_call(
    state: &AppState,
    receiver: &mut mpsc::UnboundedReceiver<Message>,
    expected_session_id: &str,
    result_marker: &str,
) {
    let request = loop {
        let Message::Text(text) = receiver.recv().await.expect("receive routed tool call") else {
            continue;
        };
        let value: serde_json::Value = serde_json::from_str(&text).expect("parse bridge message");
        if value["type"] == "tool_call" {
            break value;
        }
    };
    assert_eq!(request["session_id"], expected_session_id);
    let request_id = request["request_id"]
        .as_str()
        .expect("tool call request ID");
    assert!(
        handle_tool_result_message(
            state,
            Some(expected_session_id),
            &json!({
                "type": "tool_result",
                "request_id": request_id,
                "ok": true,
                "marker": result_marker,
            }),
        )
        .await,
        "matching Godot session should satisfy its pending call"
    );
}

type TestWebSocket = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

async fn connect_project_over_websocket(
    address: std::net::SocketAddr,
    control_token: &str,
    session_id: &str,
    project_name: &str,
    project_path: &str,
) -> TestWebSocket {
    let mut request = format!("ws://{address}/godot/ws")
        .into_client_request()
        .expect("build Godot WebSocket request");
    request.headers_mut().insert(
        CONTROL_HEADER,
        control_token.parse().expect("valid control token header"),
    );
    let (mut socket, response) = connect_async(request)
        .await
        .expect("connect fake Godot WebSocket");
    assert_eq!(
        response.status(),
        axum::http::StatusCode::SWITCHING_PROTOCOLS
    );
    socket
        .send(WebSocketMessage::Text(
            json!({
                "type": "hello",
                "session_id": session_id,
                "project_name": project_name,
                "project_path": project_path,
                "editor_filesystem": {
                    "status": "ready",
                    "asset_tools_ready": true,
                },
                "tools": ["script_diagnostics"],
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send fake Godot hello");
    socket
}

async fn answer_websocket_tool_call(
    socket: &mut TestWebSocket,
    expected_session_id: &str,
    expected_probe: &str,
    result_marker: &str,
) {
    let request = loop {
        let message = socket
            .next()
            .await
            .expect("Godot WebSocket should remain connected")
            .expect("receive daemon WebSocket message");
        let WebSocketMessage::Text(text) = message else {
            continue;
        };
        let value: serde_json::Value =
            serde_json::from_str(text.as_ref()).expect("parse daemon WebSocket message");
        if value["type"] == "tool_call" {
            break value;
        }
    };
    assert_eq!(request["session_id"], expected_session_id);
    assert_eq!(request["tool"], "script_diagnostics");
    assert_eq!(request["args"]["probe"], expected_probe);
    socket
        .send(WebSocketMessage::Text(
            json!({
                "type": "tool_result",
                "request_id": request["request_id"],
                "ok": true,
                "marker": result_marker,
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("send fake Godot tool result");
}

async fn authenticated_status(
    client: &reqwest::Client,
    base_url: &str,
    control_token: &str,
) -> serde_json::Value {
    client
        .get(format!("{base_url}/status"))
        .header(CONTROL_HEADER, control_token)
        .send()
        .await
        .expect("request daemon status")
        .error_for_status()
        .expect("daemon status succeeds")
        .json()
        .await
        .expect("parse daemon status")
}

#[tokio::test]
async fn project_status_updates_only_the_matching_session_with_an_object_status() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let (outbound_tx, _outbound_rx) = mpsc::unbounded_channel::<Message>();
    let mut session_id = None;

    assert!(
        handle_project_state_message(
            &json!({
                "type": "hello",
                "session_id": "project-a",
                "project_name": "Project A",
                "editor_filesystem": { "status": "scanning" },
                "tools": []
            }),
            &state,
            &mut session_id,
            "fallback",
            &outbound_tx,
        )
        .await
    );
    assert_eq!(session_id.as_deref(), Some("project-a"));

    assert!(
        handle_project_state_message(
            &json!({
                "type": "project_status",
                "session_id": "project-a",
                "editor_filesystem": { "status": "ready", "asset_tools_ready": true }
            }),
            &state,
            &mut session_id,
            "fallback",
            &outbound_tx,
        )
        .await
    );
    let projects = state.projects.read().await;
    assert_eq!(
        projects["project-a"].editor_filesystem,
        Some(json!({ "status": "ready", "asset_tools_ready": true }))
    );
    drop(projects);

    handle_project_state_message(
        &json!({
            "type": "project_status",
            "session_id": "project-b",
            "editor_filesystem": { "status": "importing" }
        }),
        &state,
        &mut session_id,
        "fallback",
        &outbound_tx,
    )
    .await;
    assert_eq!(
        state.projects.read().await["project-a"].editor_filesystem,
        Some(json!({ "status": "ready", "asset_tools_ready": true }))
    );

    handle_project_state_message(
        &json!({
            "type": "project_status",
            "session_id": "project-a",
            "editor_filesystem": "ready"
        }),
        &state,
        &mut session_id,
        "fallback",
        &outbound_tx,
    )
    .await;
    assert_eq!(
        state.projects.read().await["project-a"].editor_filesystem,
        Some(json!({ "status": "ready", "asset_tools_ready": true }))
    );
}

#[tokio::test]
async fn bound_call_with_no_matching_editor_fails_closed_with_retryable_code() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let bound_project = ProjectFixture::new("bound");
    let unrelated_project = ProjectFixture::new("unrelated");
    let (unrelated_sender, _unrelated_receiver) = mpsc::unbounded_channel::<Message>();
    let mut unrelated_session = None;

    handle_project_state_message(
        &json!({
            "type": "hello",
            "session_id": "unrelated-session",
            "project_name": "Unrelated",
            "project_path": unrelated_project.protocol_path(),
            "tools": []
        }),
        &state,
        &mut unrelated_session,
        "unrelated-fallback",
        &unrelated_sender,
    )
    .await;

    assert_eq!(
        state.active_session_id.read().await.as_deref(),
        Some("unrelated-session")
    );
    let result = call_tool_value_for_project(
        &state,
        Some(bound_project.protocol_path()),
        "read_file",
        json!({ "path": "res://example.gd" }),
    )
    .await;

    assert_eq!(result["ok"], false);
    assert_eq!(result["code"], "bound_project_not_connected");
    assert_eq!(result["retryable"], true);
}

#[tokio::test]
async fn unavailable_bound_root_fails_closed_instead_of_using_the_legacy_target() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let bound_project = ProjectFixture::new("unavailable-bound");
    let bound_path = bound_project.protocol_path().to_string();
    let unrelated_project = ProjectFixture::new("unavailable-unrelated");
    let _receiver = connect_project(
        &state,
        "unrelated-session",
        "Unrelated",
        unrelated_project.protocol_path(),
    )
    .await;
    fs::remove_dir_all(bound_project.path()).expect("make bound root unavailable");

    let result = call_tool_value_for_project(
        &state,
        Some(&bound_path),
        "read_file",
        json!({ "path": "res://example.gd" }),
    )
    .await;

    assert_eq!(result["ok"], false);
    assert_eq!(result["code"], "bound_project_not_connected");
    assert_eq!(result["retryable"], true);
}

#[tokio::test]
async fn authenticated_http_routes_concurrent_bound_calls_to_their_godot_websockets() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let control_token: Arc<str> = Arc::from("routing-http-test-token");
    let app = Router::new()
        .route("/status", get(super::status))
        .route("/tools/call", post(super::call_tool))
        .route("/godot/ws", get(super::godot_ws))
        .route_layer(middleware::from_fn_with_state(
            control_token.clone(),
            control_auth::require_control_auth,
        ))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let client = reqwest::Client::new();
    let base_url = format!("http://{address}");
    let project_a = ProjectFixture::new("http-websocket-a");
    let project_b = ProjectFixture::new("http-websocket-b");

    let unauthorized = client
        .get(format!("{base_url}/status"))
        .send()
        .await
        .unwrap();
    assert_eq!(unauthorized.status(), axum::http::StatusCode::UNAUTHORIZED);

    let mut socket_a = connect_project_over_websocket(
        address,
        control_token.as_ref(),
        "session-a",
        "Agent A",
        project_a.protocol_path(),
    )
    .await;
    let mut socket_b = connect_project_over_websocket(
        address,
        control_token.as_ref(),
        "session-b",
        "Agent B",
        project_b.protocol_path(),
    )
    .await;

    let connected_status = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let status = authenticated_status(&client, &base_url, control_token.as_ref()).await;
            if status["connected_projects"]
                .as_array()
                .is_some_and(|projects| projects.len() == 2)
            {
                break status;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("both fake editors should connect");
    assert!(connected_status["active_session_id"].is_null());

    let request_a = client
        .post(format!("{base_url}/tools/call"))
        .header(CONTROL_HEADER, control_token.as_ref())
        .json(&json!({
            "project_path": project_a.protocol_path(),
            "tool": "script_diagnostics",
            "args": { "probe": "a" },
        }))
        .send();
    let request_b = client
        .post(format!("{base_url}/tools/call"))
        .header(CONTROL_HEADER, control_token.as_ref())
        .json(&json!({
            "project_path": project_b.protocol_path(),
            "tool": "script_diagnostics",
            "args": { "probe": "b" },
        }))
        .send();
    let answer_a = answer_websocket_tool_call(&mut socket_a, "session-a", "a", "result-from-a");
    let answer_b = answer_websocket_tool_call(&mut socket_b, "session-b", "b", "result-from-b");
    let (response_a, response_b, (), ()) = tokio::time::timeout(Duration::from_secs(2), async {
        let (response_a, response_b, (), ()) =
            tokio::join!(request_a, request_b, answer_a, answer_b);
        (
            response_a
                .unwrap()
                .error_for_status()
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap(),
            response_b
                .unwrap()
                .error_for_status()
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap(),
            (),
            (),
        )
    })
    .await
    .expect("concurrent routed calls should complete through HTTP and WebSockets");

    assert_eq!(response_a["marker"], "result-from-a");
    assert_eq!(response_b["marker"], "result-from-b");
    let final_status = authenticated_status(&client, &base_url, control_token.as_ref()).await;
    assert!(final_status["active_session_id"].is_null());

    socket_a.close(None).await.unwrap();
    socket_b.close(None).await.unwrap();
    server.abort();
    let _ = server.await;
}

#[tokio::test]
async fn a_different_editor_cannot_satisfy_another_projects_pending_call() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project_a = ProjectFixture::new("correlation-a");
    let project_b = ProjectFixture::new("correlation-b");
    let mut receiver_a =
        connect_project(&state, "session-a", "Agent A", project_a.protocol_path()).await;
    let _receiver_b =
        connect_project(&state, "session-b", "Agent B", project_b.protocol_path()).await;

    let call = call_tool_value_for_project(
        &state,
        Some(project_a.protocol_path()),
        "read_file",
        json!({ "path": "res://a.gd" }),
    );
    let answer = async {
        let request = loop {
            let Message::Text(text) = receiver_a.recv().await.expect("receive routed call") else {
                continue;
            };
            let value: serde_json::Value = serde_json::from_str(&text).unwrap();
            if value["type"] == "tool_call" {
                break value;
            }
        };
        let request_id = request["request_id"].as_str().unwrap();
        assert!(
            !handle_tool_result_message(
                &state,
                Some("session-b"),
                &json!({
                    "type": "tool_result",
                    "request_id": request_id,
                    "ok": true,
                    "marker": "wrong-editor",
                }),
            )
            .await
        );
        assert!(
            state
                .pending_tool_calls
                .read()
                .await
                .contains_key(request_id)
        );
        assert!(
            handle_tool_result_message(
                &state,
                Some("session-a"),
                &json!({
                    "type": "tool_result",
                    "request_id": request_id,
                    "ok": true,
                    "marker": "right-editor",
                }),
            )
            .await
        );
    };

    let (result, ()) =
        tokio::time::timeout(Duration::from_secs(2), async { tokio::join!(call, answer) })
            .await
            .expect("matching editor should complete the call");
    assert_eq!(result["marker"], "right-editor");
}

#[cfg(unix)]
#[tokio::test]
async fn alias_editors_for_one_live_project_are_an_ambiguous_binding() {
    use std::os::unix::fs::symlink;

    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project = ProjectFixture::new("alias-target");
    let alias_parent = ProjectFixture::new("alias-parent");
    let alias = alias_parent.path().join("project-alias");
    symlink(project.path(), &alias).expect("create project symlink");
    let alias_path = alias.to_str().expect("test alias should be Unicode");
    let _receiver_a = connect_project(&state, "session-a", "Target", project.protocol_path()).await;
    let _receiver_b = connect_project(&state, "session-b", "Alias", alias_path).await;

    let result = call_tool_value_for_project(
        &state,
        Some(project.protocol_path()),
        "read_file",
        json!({ "path": "res://example.gd" }),
    )
    .await;

    assert_eq!(result["ok"], false);
    assert_eq!(result["code"], "ambiguous_project_binding");
    assert_eq!(result["retryable"], false);
}

#[tokio::test]
async fn bound_call_recovers_after_editor_reconnects_with_a_new_session_id() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project = ProjectFixture::new("reconnect");
    let _old_receiver =
        connect_project(&state, "old-session", "Project", project.protocol_path()).await;
    state.godot_senders.write().await.remove("old-session");
    state.projects.write().await.remove("old-session");
    let mut new_receiver =
        connect_project(&state, "new-session", "Project", project.protocol_path()).await;

    let call = call_tool_value_for_project(
        &state,
        Some(project.protocol_path()),
        "read_file",
        json!({ "path": "res://example.gd" }),
    );
    let answer = answer_next_tool_call(&state, &mut new_receiver, "new-session", "reconnected");
    let (result, ()) =
        tokio::time::timeout(Duration::from_secs(2), async { tokio::join!(call, answer) })
            .await
            .expect("reconnected call should route promptly");

    assert_eq!(result["marker"], "reconnected");
}

#[tokio::test]
async fn explicit_editor_session_routing_ignores_the_legacy_mcp_target() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project_a = ProjectFixture::new("session-a");
    let project_b = ProjectFixture::new("session-b");
    let mut receiver_a =
        connect_project(&state, "session-a", "Project A", project_a.protocol_path()).await;
    let _receiver_b =
        connect_project(&state, "session-b", "Project B", project_b.protocol_path()).await;
    *state.active_session_id.write().await = Some("session-b".to_string());

    let call = call_tool_value_for_session(
        &state,
        Some("session-a"),
        "read_file",
        json!({ "path": "res://example.gd" }),
    );
    let answer = answer_next_tool_call(&state, &mut receiver_a, "session-a", "explicit-session");
    let (result, ()) =
        tokio::time::timeout(Duration::from_secs(2), async { tokio::join!(call, answer) })
            .await
            .expect("explicit session call should route promptly");

    assert_eq!(result["marker"], "explicit-session");
    assert_eq!(
        state.active_session_id.read().await.as_deref(),
        Some("session-b")
    );
}

#[tokio::test]
async fn legacy_mcp_call_keeps_using_the_dock_selected_target() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project_a = ProjectFixture::new("legacy-a");
    let project_b = ProjectFixture::new("legacy-b");
    let _receiver_a =
        connect_project(&state, "session-a", "Project A", project_a.protocol_path()).await;
    let mut receiver_b =
        connect_project(&state, "session-b", "Project B", project_b.protocol_path()).await;
    *state.active_session_id.write().await = Some("session-b".to_string());
    *state.active_project_explicit.write().await = true;

    let call = call_tool_value_for_project(
        &state,
        None,
        "read_file",
        json!({ "path": "res://example.gd" }),
    );
    let answer = answer_next_tool_call(&state, &mut receiver_b, "session-b", "legacy-target");
    let (result, ()) =
        tokio::time::timeout(Duration::from_secs(2), async { tokio::join!(call, answer) })
            .await
            .expect("legacy target call should route promptly");

    assert_eq!(result["marker"], "legacy-target");
}

#[tokio::test]
async fn legacy_mcp_call_keeps_using_the_sole_editor_fallback() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project = ProjectFixture::new("legacy-sole");
    let mut receiver =
        connect_project(&state, "sole-session", "Sole", project.protocol_path()).await;
    *state.active_session_id.write().await = None;

    let call = call_tool_value_for_project(
        &state,
        None,
        "read_file",
        json!({ "path": "res://example.gd" }),
    );
    let answer = answer_next_tool_call(&state, &mut receiver, "sole-session", "sole-fallback");
    let (result, ()) =
        tokio::time::timeout(Duration::from_secs(2), async { tokio::join!(call, answer) })
            .await
            .expect("sole-editor fallback should route promptly");

    assert_eq!(result["marker"], "sole-fallback");
}

#[tokio::test]
async fn legacy_mcp_call_still_reports_ambiguity_without_a_target() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let project_a = ProjectFixture::new("legacy-ambiguous-a");
    let project_b = ProjectFixture::new("legacy-ambiguous-b");
    let _receiver_a =
        connect_project(&state, "session-a", "Project A", project_a.protocol_path()).await;
    let _receiver_b =
        connect_project(&state, "session-b", "Project B", project_b.protocol_path()).await;
    assert!(state.active_session_id.read().await.is_none());

    let result = call_tool_value_for_project(
        &state,
        None,
        "read_file",
        json!({ "path": "res://example.gd" }),
    )
    .await;

    assert_eq!(result["ok"], false);
    assert_eq!(result["code"], "legacy_target_ambiguous");
    assert!(
        result["error"]
            .as_str()
            .is_some_and(|message| message.contains("Multiple Fennara projects are open"))
    );
}

#[tokio::test]
async fn bound_status_never_uses_unrelated_legacy_target_readiness() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let bound = ProjectFixture::new("status-bound");
    let unrelated = ProjectFixture::new("status-unrelated");
    let _receiver = connect_project(
        &state,
        "unrelated-session",
        "Unrelated",
        unrelated.protocol_path(),
    )
    .await;

    let Json(status) = bound_status(
        State(state),
        Json(BoundStatusRequest {
            project_path: bound.protocol_path().to_string(),
        }),
    )
    .await;

    assert_eq!(status["ok"], true);
    assert_eq!(status["routing_mode"], "bound");
    assert_eq!(status["bound_editor_state"], "not_connected");
    assert_eq!(status["code"], "bound_project_not_connected");
    assert_eq!(status["retryable"], true);
    assert!(status["selected_project"].is_null());
    assert!(status["editor_filesystem"].is_null());
    assert_eq!(status["legacy_active_session_id"], "unrelated-session");
}

#[tokio::test]
async fn bound_status_includes_readiness_only_for_the_unique_matching_editor() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let bound = ProjectFixture::new("status-connected");
    let _receiver = connect_project(&state, "bound-session", "Bound", bound.protocol_path()).await;

    let Json(status) = bound_status(
        State(state),
        Json(BoundStatusRequest {
            project_path: bound.protocol_path().to_string(),
        }),
    )
    .await;

    assert_eq!(status["bound_editor_state"], "connected");
    assert!(status["code"].is_null());
    assert_eq!(status["selected_project"]["session_id"], "bound-session");
    assert_eq!(status["editor_filesystem"]["status"], "ready");
}

#[tokio::test]
async fn ambiguous_bound_status_exposes_no_editor_or_filesystem_details() {
    let (shutdown_tx, _shutdown_rx) = oneshot::channel();
    let state = AppState::new(shutdown_tx);
    let bound = ProjectFixture::new("status-ambiguous");
    let _receiver_a = connect_project(&state, "session-a", "First", bound.protocol_path()).await;
    let _receiver_b = connect_project(&state, "session-b", "Second", bound.protocol_path()).await;

    let Json(status) = bound_status(
        State(state),
        Json(BoundStatusRequest {
            project_path: bound.protocol_path().to_string(),
        }),
    )
    .await;

    assert_eq!(status["bound_editor_state"], "ambiguous");
    assert_eq!(status["code"], "ambiguous_project_binding");
    assert_eq!(status["retryable"], false);
    assert!(status["selected_project"].is_null());
    assert!(status["editor_filesystem"].is_null());
}
