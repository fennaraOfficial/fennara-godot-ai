use std::{collections::HashSet, path::Path, sync::OnceLock};

use tokio::sync::{OnceCell, OwnedMutexGuard};

use super::{CaptureResult, CaptureUnavailableReason, CheckpointStore, git::ProjectIdentity};
use crate::runtime_daemon::chat::{ids, store};

const RETAINED_TURN_CHECKPOINTS_PER_PROJECT: usize = 20;

static SHARED_STORE: OnceLock<Result<CheckpointStore, String>> = OnceLock::new();
static RECONCILED: OnceCell<Result<(), String>> = OnceCell::const_new();

pub(crate) struct TurnCheckpointIds<'a> {
    pub(crate) chat_id: &'a str,
    pub(crate) user_message_id: &'a str,
    pub(crate) assistant_message_id: &'a str,
    pub(crate) generation_id: &'a str,
}

pub(crate) struct PendingTurnCheckpoint {
    inner: Option<PendingInner>,
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
        Self { inner: None }
    }

    pub(crate) async fn attach(
        mut self,
        ids: TurnCheckpointIds<'_>,
    ) -> Result<TurnCheckpoint, String> {
        let Some(inner) = self.inner.take() else {
            return Ok(TurnCheckpoint::disabled());
        };
        attach_turn(inner, ids).await
    }
}

pub(crate) struct TurnCheckpoint {
    inner: Option<TurnInner>,
}

struct TurnInner {
    store: CheckpointStore,
    identity: ProjectIdentity,
    checkpoint_id: String,
    start_capture: CaptureResult,
    _lease: OwnedMutexGuard<()>,
}

impl TurnCheckpoint {
    pub(crate) fn disabled() -> Self {
        Self { inner: None }
    }

    pub(crate) async fn finish(mut self) -> Result<(), String> {
        let Some(inner) = self.inner.take() else {
            return Ok(());
        };
        finish_turn(inner).await
    }
}

impl Drop for TurnCheckpoint {
    fn drop(&mut self) {
        let Some(inner) = self.inner.take() else {
            return;
        };
        tokio::spawn(async move {
            let _ = finish_turn(inner).await;
        });
    }
}

pub(crate) async fn begin_project_turn(
    project_path: Option<&str>,
) -> Result<PendingTurnCheckpoint, String> {
    let Some(project_path) = project_path.filter(|path| !path.trim().is_empty()) else {
        return Ok(PendingTurnCheckpoint::disabled());
    };
    let store = shared_store()?;
    RECONCILED
        .get_or_init(|| reconcile(store.clone()))
        .await
        .clone()?;
    begin_with_store(store, Path::new(project_path)).await
}

fn shared_store() -> Result<CheckpointStore, String> {
    SHARED_STORE
        .get_or_init(CheckpointStore::from_app_data)
        .clone()
}

async fn begin_with_store(
    store: CheckpointStore,
    project_root: &Path,
) -> Result<PendingTurnCheckpoint, String> {
    let identity = store
        .git
        .identify(project_root)
        .await
        .map_err(|error| error.to_string())?;
    let lease = store.turn_lock(&identity.root).await.lock_owned().await;
    let start_capture = store.capture(&identity.root).await;
    Ok(PendingTurnCheckpoint {
        inner: Some(PendingInner {
            store,
            identity,
            checkpoint_id: ids::new_id("checkpoint"),
            start_capture,
            _lease: lease,
        }),
    })
}

async fn attach_turn(
    inner: PendingInner,
    ids: TurnCheckpointIds<'_>,
) -> Result<TurnCheckpoint, String> {
    let project_path = inner
        .identity
        .root
        .to_str()
        .ok_or_else(|| "Project path is not valid UTF-8.".to_string())?;
    store::insert_turn_checkpoint(store::NewTurnCheckpoint {
        id: &inner.checkpoint_id,
        chat_id: ids.chat_id,
        user_message_id: ids.user_message_id,
        assistant_message_id: ids.assistant_message_id,
        generation_id: ids.generation_id,
        project_path,
        storage_key: &inner.identity.storage_key,
        start_capture: &inner.start_capture,
    })?;
    if let Some(snapshot_id) = inner.start_capture.snapshot_id.as_deref()
        && let Err(error) = inner
            .store
            .git
            .pin_snapshot(&inner.identity, &inner.checkpoint_id, "start", snapshot_id)
            .await
    {
        let _ = store::mark_turn_checkpoint_interrupted(&inner.checkpoint_id);
        let _ = prune_for_project(&inner.store, &inner.identity.storage_key).await;
        return Err(error.to_string());
    }
    if inner.start_capture.snapshot_id.is_none() {
        let _ = prune_for_project(&inner.store, &inner.identity.storage_key).await;
        return Ok(TurnCheckpoint::disabled());
    }
    Ok(TurnCheckpoint {
        inner: Some(TurnInner {
            store: inner.store,
            identity: inner.identity,
            checkpoint_id: inner.checkpoint_id,
            start_capture: inner.start_capture,
            _lease: inner._lease,
        }),
    })
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
    let checkpoints =
        store::claim_prunable_turn_checkpoints(storage_key, RETAINED_TURN_CHECKPOINTS_PER_PROJECT)?;
    if checkpoints.is_empty() {
        return Ok(());
    }
    for checkpoint in checkpoints {
        release_pruning_checkpoint(store, checkpoint).await?;
    }
    store
        .git
        .compact_storage(storage_key)
        .await
        .map_err(|error| error.to_string())
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
