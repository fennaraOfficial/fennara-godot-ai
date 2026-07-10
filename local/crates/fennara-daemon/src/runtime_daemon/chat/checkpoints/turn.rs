use std::{collections::HashSet, fmt, path::Path, sync::OnceLock};

use serde_json::json;
use tokio::sync::{OnceCell, OwnedMutexGuard};

use super::{CaptureResult, CaptureUnavailableReason, CheckpointStore, git::ProjectIdentity};
use crate::runtime_daemon::chat::{ids, store, trace};

const RETAINED_TURN_CHECKPOINTS_PER_PROJECT: usize = 20;

static SHARED_STORE: OnceLock<Result<CheckpointStore, String>> = OnceLock::new();
static RECONCILED: OnceCell<()> = OnceCell::const_new();

#[derive(Debug)]
pub(crate) struct BeginProjectTurnError {
    message: String,
    recovery_incomplete: bool,
}

impl BeginProjectTurnError {
    fn checkpoint_unavailable(message: String) -> Self {
        Self {
            message,
            recovery_incomplete: false,
        }
    }

    fn recovery_incomplete(message: String) -> Self {
        Self {
            message,
            recovery_incomplete: true,
        }
    }

    pub(crate) fn is_recovery_incomplete(&self) -> bool {
        self.recovery_incomplete
    }
}

impl fmt::Display for BeginProjectTurnError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

pub(crate) struct TurnCheckpointIds<'a> {
    pub(crate) chat_id: &'a str,
    pub(crate) user_message_id: &'a str,
    pub(crate) assistant_message_id: &'a str,
    pub(crate) generation_id: &'a str,
    pub(crate) trace: &'a trace::TraceRecorder,
}

pub(crate) struct PendingTurnCheckpoint {
    inner: Option<PendingInner>,
    warning: Option<String>,
}

struct PendingInner {
    store: CheckpointStore,
    identity: ProjectIdentity,
    checkpoint_id: String,
    start_capture: CaptureResult,
    _lease: OwnedMutexGuard<()>,
}

impl PendingTurnCheckpoint {
    pub(crate) fn disabled() -> Self {
        Self {
            inner: None,
            warning: None,
        }
    }

    pub(crate) fn unavailable(warning: String) -> Self {
        Self {
            inner: None,
            warning: Some(warning),
        }
    }

    pub(crate) async fn attach(mut self, ids: TurnCheckpointIds<'_>) -> TurnCheckpoint {
        let Some(inner) = self.inner.take() else {
            return TurnCheckpoint {
                state: None,
                warning: self.warning.take(),
            };
        };
        attach_turn(inner, ids).await
    }
}

pub(crate) struct TurnCheckpoint {
    state: Option<TurnState>,
    warning: Option<String>,
}

enum TurnState {
    Active(Box<TurnInner>),
    Serialized(OwnedMutexGuard<()>),
}

struct TurnInner {
    store: CheckpointStore,
    identity: ProjectIdentity,
    checkpoint_id: String,
    start_capture: CaptureResult,
    trace: trace::TraceRecorder,
    _lease: OwnedMutexGuard<()>,
}

impl TurnCheckpoint {
    fn serialized(lease: OwnedMutexGuard<()>, warning: Option<String>) -> Self {
        Self {
            state: Some(TurnState::Serialized(lease)),
            warning,
        }
    }

    pub(crate) fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }

    pub(crate) async fn finish(mut self) -> Result<(), String> {
        match self.state.take() {
            Some(TurnState::Active(inner)) => finish_turn(*inner).await,
            Some(TurnState::Serialized(lease)) => {
                drop(lease);
                Ok(())
            }
            None => Ok(()),
        }
    }
}

impl Drop for TurnCheckpoint {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else {
            return;
        };
        if let TurnState::Active(inner) = state {
            let trace = inner.trace.clone();
            tokio::spawn(async move {
                if let Err(error) = finish_turn(*inner).await {
                    trace.warn(
                        "checkpoint.finish_failed",
                        "failed",
                        json!({ "message": error }),
                    );
                }
            });
        }
    }
}

pub(crate) async fn begin_project_turn(
    project_path: Option<&str>,
) -> Result<PendingTurnCheckpoint, BeginProjectTurnError> {
    let Some(project_path) = project_path.filter(|path| !path.trim().is_empty()) else {
        return Ok(PendingTurnCheckpoint::disabled());
    };
    let store = shared_reconciled_store()
        .await
        .map_err(BeginProjectTurnError::checkpoint_unavailable)?;
    super::recovery::reconcile_pending_for_project_path(&store, Path::new(project_path))
        .await
        .map_err(BeginProjectTurnError::recovery_incomplete)?;
    begin_with_store(store, Path::new(project_path)).await
}

pub(super) async fn shared_reconciled_store() -> Result<CheckpointStore, String> {
    let store = shared_store()?;
    RECONCILED
        .get_or_try_init(|| reconcile(store.clone()))
        .await
        .map(|_| ())?;
    Ok(store)
}

fn shared_store() -> Result<CheckpointStore, String> {
    SHARED_STORE
        .get_or_init(CheckpointStore::from_app_data)
        .clone()
}

async fn begin_with_store(
    store: CheckpointStore,
    project_root: &Path,
) -> Result<PendingTurnCheckpoint, BeginProjectTurnError> {
    let identity = store
        .git
        .identify(project_root)
        .await
        .map_err(|error| BeginProjectTurnError::checkpoint_unavailable(error.to_string()))?;
    let lease = store.turn_lock(&identity.root).await.lock_owned().await;
    super::recovery::reconcile_pending_for_project_locked(&store, &identity)
        .await
        .map_err(BeginProjectTurnError::recovery_incomplete)?;
    let start_capture = store.capture(&identity.root).await;
    Ok(PendingTurnCheckpoint {
        inner: Some(PendingInner {
            store,
            identity,
            checkpoint_id: ids::new_id("checkpoint"),
            start_capture,
            _lease: lease,
        }),
        warning: None,
    })
}

async fn attach_turn(inner: PendingInner, ids: TurnCheckpointIds<'_>) -> TurnCheckpoint {
    let PendingInner {
        store,
        identity,
        checkpoint_id,
        start_capture,
        _lease: lease,
    } = inner;
    let project_path = identity.root.to_str().map(ToOwned::to_owned);
    let Some(project_path) = project_path else {
        return TurnCheckpoint::serialized(
            lease,
            Some("Project path is not valid UTF-8.".to_string()),
        );
    };
    if let Err(error) = store::insert_turn_checkpoint(store::NewTurnCheckpoint {
        id: &checkpoint_id,
        chat_id: ids.chat_id,
        user_message_id: ids.user_message_id,
        assistant_message_id: ids.assistant_message_id,
        generation_id: ids.generation_id,
        project_path: &project_path,
        storage_key: &identity.storage_key,
        start_capture: &start_capture,
    }) {
        return TurnCheckpoint::serialized(lease, Some(error));
    }
    if let Some(snapshot_id) = start_capture.snapshot_id.as_deref()
        && let Err(error) = store
            .git
            .pin_snapshot(&identity, &checkpoint_id, "start", snapshot_id)
            .await
    {
        let _ = store::mark_turn_checkpoint_interrupted(&checkpoint_id);
        let _ = prune_for_project(&store, &identity.storage_key).await;
        return TurnCheckpoint::serialized(lease, Some(error.to_string()));
    }
    if start_capture.snapshot_id.is_none() {
        let _ = prune_for_project(&store, &identity.storage_key).await;
        return TurnCheckpoint::serialized(lease, None);
    }
    TurnCheckpoint {
        state: Some(TurnState::Active(Box::new(TurnInner {
            store,
            identity,
            checkpoint_id,
            start_capture,
            trace: ids.trace.clone(),
            _lease: lease,
        }))),
        warning: None,
    }
}

async fn finish_turn(inner: TurnInner) -> Result<(), String> {
    let mut end_capture = inner.store.capture(&inner.identity.root).await;
    if let Some(snapshot_id) = end_capture.snapshot_id.as_deref()
        && inner
            .store
            .git
            .pin_snapshot(&inner.identity, &inner.checkpoint_id, "end", snapshot_id)
            .await
            .is_err()
    {
        end_capture = CaptureResult::unavailable(CaptureUnavailableReason::CaptureFailed);
    }
    let changed_paths = match (
        inner.start_capture.snapshot_id.as_deref(),
        end_capture.snapshot_id.as_deref(),
    ) {
        (Some(start), Some(end)) => match inner
            .store
            .changed_paths(&inner.identity.root, start, end)
            .await
        {
            Ok(paths) => paths,
            Err(_) => {
                end_capture = CaptureResult::unavailable(CaptureUnavailableReason::CaptureFailed);
                Vec::new()
            }
        },
        _ => Vec::new(),
    };
    store::complete_turn_checkpoint(store::CompletedTurnCheckpoint {
        id: &inner.checkpoint_id,
        end_capture: &end_capture,
        changed_paths: &changed_paths,
    })?;
    let _ = prune_for_project(&inner.store, &inner.identity.storage_key).await;
    Ok(())
}

async fn reconcile(store: CheckpointStore) -> Result<(), String> {
    store::mark_capturing_checkpoints_interrupted()?;
    let mut compact = HashSet::new();
    for checkpoint in store::pruning_turn_checkpoints()? {
        let storage_key = checkpoint.storage_key.clone();
        if release_pruning_checkpoint(&store, checkpoint).await.is_ok() {
            compact.insert(storage_key);
        }
    }
    for storage_key in compact {
        let _ = store.git.compact_storage(&storage_key).await;
    }
    Ok(())
}

async fn prune_for_project(store: &CheckpointStore, storage_key: &str) -> Result<(), String> {
    let mut checkpoints = store::pruning_turn_checkpoints_for_storage(storage_key)?;
    checkpoints.extend(store::claim_prunable_turn_checkpoints(
        storage_key,
        RETAINED_TURN_CHECKPOINTS_PER_PROJECT,
    )?);
    if checkpoints.is_empty() {
        return Ok(());
    }
    let mut first_error = None;
    let mut released = false;
    for checkpoint in checkpoints {
        match release_pruning_checkpoint(store, checkpoint).await {
            Ok(()) => released = true,
            Err(error) if first_error.is_none() => first_error = Some(error),
            Err(_) => {}
        }
    }
    if released
        && let Err(error) = store.git.compact_storage(storage_key).await
        && first_error.is_none()
    {
        first_error = Some(error.to_string());
    }
    first_error.map_or(Ok(()), Err)
}

async fn release_pruning_checkpoint(
    store: &CheckpointStore,
    checkpoint: store::PrunableTurnCheckpoint,
) -> Result<(), String> {
    store
        .git
        .release_checkpoint(&checkpoint.storage_key, &checkpoint.id)
        .await
        .map_err(|error| error.to_string())?;
    store::delete_pruning_turn_checkpoint(&checkpoint.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, time::Duration};

    #[tokio::test]
    async fn conversation_only_turn_keeps_the_project_lease() {
        let store = CheckpointStore::at(PathBuf::from("unused"));
        let project = PathBuf::from("project");
        let lease = store.turn_lock(&project).await.lock_owned().await;
        let checkpoint = TurnCheckpoint::serialized(lease, None);
        let waiting_store = store.clone();
        let waiting_project = project.clone();
        let (attempting_tx, attempting_rx) = tokio::sync::oneshot::channel();
        let mut waiting = tokio::spawn(async move {
            let _ = attempting_tx.send(());
            waiting_store
                .turn_lock(&waiting_project)
                .await
                .lock_owned()
                .await
        });

        attempting_rx.await.unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(20), &mut waiting)
                .await
                .is_err()
        );
        drop(checkpoint);
        tokio::time::timeout(Duration::from_secs(1), waiting)
            .await
            .unwrap()
            .unwrap();
    }
}
