use super::{connected_shutdown_error, finish_deferred_shutdown};
use crate::runtime_daemon::state::{AppState, GodotProjectStatus};
use std::time::Duration;
use tokio::sync::oneshot;

#[test]
fn shutdown_is_allowed_without_connected_godot_projects() {
    assert!(connected_shutdown_error(0).is_none());
}

#[test]
fn shutdown_reports_connected_project_count() {
    let error = connected_shutdown_error(2).unwrap();
    assert_eq!(error["error"], "connected_godot_projects");
    assert_eq!(error["connected_project_count"], 2);
}

#[tokio::test]
async fn deferred_shutdown_is_cancelled_and_rearmed_when_a_project_connects() {
    let (sender, mut receiver) = oneshot::channel();
    let state = AppState::new(sender);
    let deferred = state.shutdown_sender.lock().await.take().unwrap();
    let mut projects = state.projects.write().await;
    let shutdown_state = state.clone();
    let shutdown = tokio::spawn(async move {
        finish_deferred_shutdown(shutdown_state, deferred, Duration::ZERO).await;
    });
    tokio::task::yield_now().await;
    projects.insert(
        "project".into(),
        GodotProjectStatus {
            session_id: "session".into(),
            project_name: Some("Project".into()),
            project_path: Some("/project".into()),
            godot_executable_path: None,
            godot_version: None,
            plugin_version: None,
            rendering_context: None,
            chat_token: None,
            tools: Vec::new(),
        },
    );
    drop(projects);

    shutdown.await.unwrap();

    assert!(receiver.try_recv().is_err());
    assert!(state.shutdown_sender.lock().await.is_some());
}

#[tokio::test]
async fn deferred_shutdown_fires_when_no_project_connects() {
    let (sender, receiver) = oneshot::channel();
    let state = AppState::new(sender);
    let deferred = state.shutdown_sender.lock().await.take().unwrap();

    finish_deferred_shutdown(state.clone(), deferred, Duration::ZERO).await;

    receiver.await.unwrap();
    assert!(state.shutdown_sender.lock().await.is_none());
}
