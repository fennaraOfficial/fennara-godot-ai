mod git;

use serde::Serialize;
use std::path::{Path, PathBuf};

use crate::runtime_daemon::util::fennara_app_dir;

use self::git::GitSnapshotStore;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CheckpointCoverage {
    Full,
    Partial,
    ConversationOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum SkippedPathReason {
    IgnoredPath,
    LargeUntrackedFile,
    NestedGitRepository,
    UntrackedByteBudgetExceeded,
    UnverifiedLfsObject,
    UnverifiedContentFilter,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct SkippedPath {
    pub(crate) path: String,
    pub(crate) reason: SkippedPathReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CaptureUnavailableReason {
    NonGitProject,
    GitUnavailable,
    TimedOut,
    CaptureFailed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
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
}

impl CheckpointStore {
    pub(crate) fn from_app_data() -> Result<Self, String> {
        Ok(Self::at(fennara_app_dir()?.join("chat-checkpoints")))
    }

    pub(crate) fn at(storage_root: PathBuf) -> Self {
        Self {
            git: GitSnapshotStore::new(storage_root),
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
}
