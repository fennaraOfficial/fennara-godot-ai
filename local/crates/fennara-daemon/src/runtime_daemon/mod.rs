use axum::{
    Extension, Json, Router, middleware,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::{net::SocketAddr, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod chat;
pub(crate) mod control_auth;
pub(crate) mod docs_cache;
pub(crate) mod godot_bridge;
pub(crate) mod permissions;
pub(crate) mod process_helpers;
pub(crate) mod runtime_log;
pub(crate) mod runtime_sessions;
pub(crate) mod scene_runner;
pub(crate) mod state;
pub(crate) mod util;

use state::AppState;

pub(crate) const DAEMON_HOST: &str = "127.0.0.1";
pub(crate) const DAEMON_PORT: u16 = 41287;
pub(crate) const DAEMON_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn run() {
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let state = AppState::new(shutdown_tx);
    let control_token = control_auth::load_or_create()
        .expect("failed to initialize local daemon control authentication");

    let privileged = Router::new()
        .route("/status", get(godot_bridge::status))
        .route("/shutdown", post(shutdown))
        .route("/chat/traces", get(chat::chat_traces))
        .route("/tools/call", post(godot_bridge::call_tool))
        .route(
            "/runtime/run-godot-scene",
            post(scene_runner::run_godot_scene),
        )
        .route(
            "/runtime/run-godot-scenes-batch",
            post(scene_runner::run_godot_scenes_batch),
        )
        .route(
            "/runtime/session/start",
            post(runtime_sessions::runtime_session_start),
        )
        .route(
            "/runtime/session/status",
            post(runtime_sessions::runtime_session_status),
        )
        .route(
            "/runtime/session/stop",
            post(runtime_sessions::runtime_session_stop),
        )
        .route(
            "/runtime/session/script",
            post(runtime_sessions::runtime_session_script),
        )
        .route("/godot/ws", get(godot_bridge::godot_ws))
        .route_layer(middleware::from_fn_with_state(
            control_token.clone(),
            control_auth::require_control_auth,
        ));

    let app = Router::new()
        .route("/health", get(health))
        .route("/control/challenge", get(control_auth::challenge))
        .route("/chat", get(chat::chat_index_redirect))
        .route("/chat/", get(chat::chat_index))
        .route(
            "/chat/tool-media/{tool_call_id}/{index}",
            get(chat::chat_tool_media),
        )
        .route("/chat/{*path}", get(chat::chat_asset))
        .route("/chat/ws", get(chat::chat_ws))
        .merge(privileged)
        .layer(Extension(control_token))
        .with_state(state);

    let addr: SocketAddr = format!("{DAEMON_HOST}:{DAEMON_PORT}")
        .parse()
        .expect("daemon bind address should be valid");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind fennara daemon");

    eprintln!("fennara-daemon listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = shutdown_rx.await;
        })
        .await
        .expect("fennara daemon stopped unexpectedly");
}

async fn health() -> Json<Value> {
    Json(json!({
        "ok": true,
        "daemon": "fennara-daemon",
        "control_auth": true,
        "version": DAEMON_VERSION
    }))
}

async fn shutdown(axum::extract::State(state): axum::extract::State<AppState>) -> Json<Value> {
    if let Some(sender) = state.shutdown_sender.lock().await.take() {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let _ = sender.send(());
        });

        Json(json!({
            "ok": true,
            "message": "daemon shutdown requested"
        }))
    } else {
        Json(json!({
            "ok": true,
            "message": "daemon shutdown already requested"
        }))
    }
}
