mod git;
mod recovery;
mod turn;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::runtime_daemon::util::fennara_app_dir;

use self::git::GitSnapshotStore;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CheckpointCoverage {
    Full,
    Partial,
    ConversationOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SkippedPathReason {
    IgnoredPath,
    LargeUntrackedFile,
    NestedGitRepository,
    UntrackedByteBudgetExceeded,
    UnverifiedLfsObject,
    UnverifiedContentFilter,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct SkippedPath {
    pub(crate) path: String,
    pub(crate) reason: SkippedPathReason,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CaptureUnavailableReason {
    NonGitProject,
    GitUnavailable,
    TimedOut,
    CaptureFailed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct CaptureResult {
    pub(crate) snapshot_id: Option<String>,
    pub(crate) coverage: CheckpointCoverage,
    pub(crate) skipped_paths: Vec<SkippedPath>,
    pub(crate) unavailable_reason: Option<CaptureUnavailableReason>,
}

impl CaptureResult {
    fn available(snapshot_id: String, skipped_paths: Vec<SkippedPath>) -> Self {
        let coverage = if skipped_paths.is_empty() {
            CheckpointCoverage::Full
        } else {
            CheckpointCoverage::Partial
        };
        Self {
            snapshot_id: Some(snapshot_id),
            coverage,
            skipped_paths,
            unavailable_reason: None,
        }
    }

    fn unavailable(reason: CaptureUnavailableReason) -> Self {
        Self {
            snapshot_id: None,
            coverage: CheckpointCoverage::ConversationOnly,
            skipped_paths: Vec::new(),
            unavailable_reason: Some(reason),
        }
    }
}

#[derive(Clone)]
pub(crate) struct CheckpointStore {
    git: GitSnapshotStore,
    turn_locks: Arc<Mutex<HashMap<PathBuf, Arc<Mutex<()>>>>>,
}

impl CheckpointStore {
    pub(crate) fn from_app_data() -> Result<Self, String> {
        Ok(Self::at(fennara_app_dir()?.join("chat-checkpoints")))
    }

    pub(crate) fn at(storage_root: PathBuf) -> Self {
        Self {
            git: GitSnapshotStore::new(storage_root),
            turn_locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) async fn capture(&self, project_root: &Path) -> CaptureResult {
        self.git.capture(project_root).await
    }

    pub(crate) async fn changed_paths(
        &self,
        project_root: &Path,
        from_snapshot: &str,
        to_snapshot: &str,
    ) -> Result<Vec<String>, String> {
        self.git
            .changed_paths(project_root, from_snapshot, to_snapshot)
            .await
            .map_err(|error| error.to_string())
    }

    async fn turn_lock(&self, canonical_project: &Path) -> Arc<Mutex<()>> {
        let mut locks = self.turn_locks.lock().await;
        locks.retain(|_, lock| Arc::strong_count(lock) > 1);
        locks
            .entry(canonical_project.to_path_buf())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }
}

pub(crate) use recovery::{TurnRecoveryResult, redo_chat_turn, undo_chat_turn};
pub(crate) use turn::{PendingTurnCheckpoint, TurnCheckpointIds, begin_project_turn};
