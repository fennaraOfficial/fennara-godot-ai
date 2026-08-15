use super::{connected_shutdown_error, finish_deferred_shutdown, shutdown_error};
use crate::runtime_daemon::state::{AppState, GodotProjectStatus};
use fennara_project_identity::ProjectRoot;
use std::{fs, path::PathBuf, time::Duration};
use tokio::sync::oneshot::{self, error::TryRecvError};

struct ProjectFixture {
    root: PathBuf,
    owner: ProjectRoot,
}

impl ProjectFixture {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "fennara-shutdown-runtime-{}-{name}",
            std::process::id()
        ));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("project.godot"), b"[application]\n").unwrap();
        let owner = ProjectRoot::resolve_absolute(root.as_os_str()).unwrap();
        Self { root, owner }
    }
}

impl Drop for ProjectFixture {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.root)
            && error.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!(
                "Failed to remove shutdown test fixture {}: {error}",
                self.root.display()
            );
        }
    }
}

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

#[test]
fn shutdown_reports_anonymous_runtime_slot_occupancy() {
    let error = shutdown_error(0, true).unwrap();
    assert_eq!(error["error"], "runtime_slot_busy");
    assert!(error.get("session_id").is_none());
    assert!(error.get("project_path").is_none());
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
            editor_filesystem: None,
            chat_token: None,
            tools: Vec::new(),
        },
    );
    drop(projects);

    shutdown.await.unwrap();

    assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
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
    assert!(!state.runtime_slot.begin_shutdown());
}

#[tokio::test]
async fn runtime_start_that_wins_admission_cancels_and_rearms_shutdown() {
    let (sender, mut receiver) = oneshot::channel();
    let state = AppState::new(sender);
    let deferred = state.shutdown_sender.lock().await.take().unwrap();
    let project = ProjectFixture::new("admission");
    let claim = state
        .runtime_slot
        .try_claim(project.owner.clone(), 1_000)
        .unwrap();

    finish_deferred_shutdown(state.clone(), deferred, Duration::ZERO).await;

    assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));
    assert!(state.shutdown_sender.lock().await.is_some());
    drop(claim);
}
