use super::{Message, handle_project_state_message};
use crate::runtime_daemon::state::AppState;
use serde_json::json;
use tokio::sync::{mpsc, oneshot};

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
