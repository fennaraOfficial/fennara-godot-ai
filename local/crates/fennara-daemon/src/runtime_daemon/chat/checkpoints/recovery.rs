use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use serde::Serialize;

use super::{
    CheckpointCoverage, CheckpointStore, SkippedPath, SkippedPathReason, git::ProjectIdentity, turn,
};
use crate::runtime_daemon::chat::{ids, store};

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TurnRecoveryResult {
    pub(crate) action: &'static str,
    pub(crate) chat_id: String,
    pub(crate) user_message_id: String,
    pub(crate) coverage: CheckpointCoverage,
    pub(crate) changed_paths: Vec<String>,
    pub(crate) skipped_paths: Vec<SkippedPath>,
    pub(crate) conflicts: Vec<String>,
    pub(crate) confirmation_required: bool,
    pub(crate) project_restored: bool,
    pub(crate) conversation_rewound: bool,
    pub(crate) rewind_boundary_sequence: i64,
}

pub(crate) async fn undo_chat_turn(
    chat_id: &str,
    user_message_id: &str,
    force: bool,
) -> Result<TurnRecoveryResult, String> {
    let store = turn::shared_reconciled_store().await?;
    reconcile_pending_for_chat(&store, Some(chat_id)).await?;
    let initial = store::recoverable_turn_checkpoint(chat_id, user_message_id)?;
    let (identity, _lease) = lock_project(&store, &initial.project_path).await?;
    let checkpoint = store::recoverable_turn_checkpoint(chat_id, user_message_id)?;
    verify_storage_identity(&identity, &checkpoint.storage_key)?;
    let conflicts = preflight_conflicts(
        &store,
        &identity,
        checkpoint.end_snapshot_id.as_deref(),
        &checkpoint.changed_paths,
    )
    .await?;
    if !conflicts.is_empty() && !force {
        return Ok(TurnRecoveryResult {
            action: "undo",
            chat_id: checkpoint.chat_id,
            user_message_id: checkpoint.user_message_id,
            coverage: checkpoint.capture.coverage,
            changed_paths: checkpoint.changed_paths,
            skipped_paths: checkpoint.capture.skipped_paths,
            conflicts,
            confirmation_required: true,
            project_restored: false,
            conversation_rewound: false,
            rewind_boundary_sequence: checkpoint.boundary_sequence,
        });
    }

    let operation_id = ids::new_id("recovery");
    let journal = store::begin_turn_undo(&checkpoint, &operation_id)?;
    apply_journal_target(&store, &identity, &journal, "undo").await?;
    store::finish_turn_undo(chat_id, &operation_id)?;
    Ok(TurnRecoveryResult {
        action: "undo",
        chat_id: checkpoint.chat_id,
        user_message_id: checkpoint.user_message_id,
        coverage: checkpoint.capture.coverage,
        changed_paths: checkpoint.changed_paths,
        skipped_paths: checkpoint.capture.skipped_paths,
        conflicts,
        confirmation_required: false,
        project_restored: journal.start_snapshot_id.is_some() && !journal.changed_paths.is_empty(),
        conversation_rewound: true,
        rewind_boundary_sequence: checkpoint.boundary_sequence,
    })
}

pub(crate) async fn redo_chat_turn(
    chat_id: &str,
    force: bool,
) -> Result<TurnRecoveryResult, String> {
    let store = turn::shared_reconciled_store().await?;
    reconcile_pending_for_chat(&store, Some(chat_id)).await?;
    let initial = store::recovery_journal(chat_id)?
        .ok_or_else(|| "No undone turn is available to redo.".to_string())?;
    if initial.state != "undone" {
        return Err("The previous turn recovery has not settled yet.".to_string());
    }
    let (identity, _lease) = lock_project(&store, &initial.project_path).await?;
    let journal = store::recovery_journal(chat_id)?
        .ok_or_else(|| "No undone turn is available to redo.".to_string())?;
    if journal.state != "undone" {
        return Err("The previous turn recovery has not settled yet.".to_string());
    }
    verify_storage_identity(&identity, &journal.storage_key)?;
    let conflicts = preflight_conflicts(
        &store,
        &identity,
        journal.start_snapshot_id.as_deref(),
        &journal.changed_paths,
    )
    .await?;
    if !conflicts.is_empty() && !force {
        return Ok(journal_result(&journal, "redo", conflicts, true, false));
    }

    let operation_id = ids::new_id("recovery");
    let journal = store::begin_turn_redo(chat_id, &operation_id)?;
    apply_journal_target(&store, &identity, &journal, "redo").await?;
    store::finish_turn_redo(chat_id, &operation_id)?;
    Ok(journal_result(&journal, "redo", conflicts, false, true))
}

pub(crate) async fn resume_chat_turn(
    chat_id: &str,
    force: bool,
) -> Result<TurnRecoveryResult, String> {
    let initial = store::recovery_journal(chat_id)?
        .filter(|journal| matches!(journal.state.as_str(), "applying_undo" | "applying_redo"))
        .ok_or_else(|| "No interrupted turn recovery is available to resume.".to_string())?;
    let action = if initial.state == "applying_undo" {
        "undo"
    } else {
        "redo"
    };
    let store = turn::shared_reconciled_store().await?;
    let (identity, _lease) = lock_project(&store, &initial.project_path).await?;
    match (action, store::recovery_journal(chat_id)?) {
        ("undo", Some(current))
            if current.state == "undone" && current.operation_id == initial.operation_id => {}
        ("redo", None) => {}
        (_, Some(current))
            if current.state == initial.state && current.operation_id == initial.operation_id =>
        {
            verify_storage_identity(&identity, &current.storage_key)?;
            let conflicts = interrupted_recovery_conflicts(&store, &identity, &current).await?;
            if !conflicts.is_empty() && !force {
                let mut result = journal_result(&current, action, conflicts, true, false);
                result.conversation_rewound = action == "redo";
                return Ok(result);
            }
            finish_journal_locked(&store, &identity, &current).await?;
        }
        _ => return Err("Turn recovery changed before it could be resumed.".to_string()),
    }
    Ok(TurnRecoveryResult {
        action,
        chat_id: initial.chat_id,
        user_message_id: initial.user_message_id,
        coverage: initial.capture.coverage,
        changed_paths: initial.changed_paths.clone(),
        skipped_paths: initial.capture.skipped_paths,
        conflicts: Vec::new(),
        confirmation_required: false,
        project_restored: initial.start_snapshot_id.is_some() && !initial.changed_paths.is_empty(),
        conversation_rewound: action == "undo",
        rewind_boundary_sequence: initial.boundary_sequence,
    })
}

async fn reconcile_pending_for_chat(
    store: &CheckpointStore,
    chat_id: Option<&str>,
) -> Result<(), String> {
    for journal in store::pending_recovery_journals()? {
        if chat_id.is_some_and(|chat_id| journal.chat_id != chat_id) {
            continue;
        }
        reconcile_journal(store, journal).await?;
    }
    Ok(())
}

pub(super) async fn reconcile_pending_for_project_locked(
    store: &CheckpointStore,
    identity: &ProjectIdentity,
) -> Result<(), String> {
    for journal in store::pending_recovery_journals()? {
        if journal.storage_key == identity.storage_key {
            reconcile_journal_locked(store, identity, &journal).await?;
        }
    }
    Ok(())
}

pub(super) async fn reconcile_pending_for_project_path(
    store: &CheckpointStore,
    project_path: &Path,
) -> Result<(), String> {
    let requested = canonical_or_original(project_path).await;
    for journal in store::pending_recovery_journals()? {
        if canonical_or_original(Path::new(&journal.project_path)).await == requested {
            reconcile_journal(store, journal).await?;
        }
    }
    Ok(())
}

async fn canonical_or_original(path: &Path) -> PathBuf {
    tokio::fs::canonicalize(path)
        .await
        .unwrap_or_else(|_| path.to_path_buf())
}

async fn reconcile_journal(
    store: &CheckpointStore,
    journal: store::TurnRecoveryJournal,
) -> Result<(), String> {
    let (identity, _lease) = lock_project(store, &journal.project_path).await?;
    reconcile_journal_locked(store, &identity, &journal).await
}

async fn reconcile_journal_locked(
    store: &CheckpointStore,
    identity: &ProjectIdentity,
    journal: &store::TurnRecoveryJournal,
) -> Result<(), String> {
    verify_storage_identity(&identity, &journal.storage_key)?;
    let conflicts = interrupted_recovery_conflicts(store, identity, journal).await?;
    if !conflicts.is_empty() {
        return Err(format!(
            "Turn recovery needs confirmation before overwriting: {}",
            conflicts.join(", ")
        ));
    }
    finish_journal_locked(store, identity, journal).await
}

async fn finish_journal_locked(
    store: &CheckpointStore,
    identity: &ProjectIdentity,
    journal: &store::TurnRecoveryJournal,
) -> Result<(), String> {
    let action = match journal.state.as_str() {
        "applying_undo" => "undo",
        "applying_redo" => "redo",
        _ => return Ok(()),
    };
    apply_journal_target(store, &identity, &journal, action).await?;
    if action == "undo" {
        store::finish_turn_undo(&journal.chat_id, &journal.operation_id)
    } else {
        store::finish_turn_redo(&journal.chat_id, &journal.operation_id)
    }
}

async fn lock_project(
    store: &CheckpointStore,
    project_path: &str,
) -> Result<(ProjectIdentity, tokio::sync::OwnedMutexGuard<()>), String> {
    let identity = store
        .git
        .identify(Path::new(project_path))
        .await
        .map_err(|error| error.to_string())?;
    let lease = store.turn_lock(&identity.root).await.lock_owned().await;
    Ok((identity, lease))
}

fn verify_storage_identity(identity: &ProjectIdentity, storage_key: &str) -> Result<(), String> {
    if identity.storage_key == storage_key {
        Ok(())
    } else {
        Err("The project path now resolves to a different checkpoint store.".to_string())
    }
}

async fn preflight_conflicts(
    store: &CheckpointStore,
    identity: &ProjectIdentity,
    expected_snapshot_id: Option<&str>,
    changed_paths: &[String],
) -> Result<Vec<String>, String> {
    let Some(expected_snapshot_id) = expected_snapshot_id else {
        return Ok(Vec::new());
    };
    if changed_paths.is_empty() {
        return Ok(Vec::new());
    }
    let current = store.capture(&identity.root).await;
    let current_snapshot_id = current.snapshot_id.as_deref().ok_or_else(|| {
        "The current project state could not be verified, so files were not restored.".to_string()
    })?;
    let affected = affected_paths(changed_paths);
    ensure_safe_skipped_paths(&current.skipped_paths, &affected)?;
    let mut conflicts = store
        .changed_paths(&identity.root, expected_snapshot_id, current_snapshot_id)
        .await?
        .into_iter()
        .filter(|path| affected.contains(path.as_str()))
        .collect::<BTreeSet<_>>();
    conflicts.extend(
        current
            .skipped_paths
            .into_iter()
            .map(|path| path.path)
            .filter(|path| {
                affected
                    .iter()
                    .any(|affected| paths_overlap(affected, path))
            }),
    );
    Ok(conflicts.into_iter().collect())
}

async fn interrupted_recovery_conflicts(
    store: &CheckpointStore,
    identity: &ProjectIdentity,
    journal: &store::TurnRecoveryJournal,
) -> Result<Vec<String>, String> {
    if journal.changed_paths.is_empty()
        && journal.start_snapshot_id.is_none()
        && journal.end_snapshot_id.is_none()
    {
        return Ok(Vec::new());
    }
    let (Some(start_snapshot_id), Some(end_snapshot_id)) = (
        journal.start_snapshot_id.as_deref(),
        journal.end_snapshot_id.as_deref(),
    ) else {
        return Err("The checkpoint does not contain both project boundaries.".to_string());
    };
    let current = store.capture(&identity.root).await;
    let current_snapshot_id = current.snapshot_id.as_deref().ok_or_else(|| {
        "The current project state could not be verified, so files were not restored.".to_string()
    })?;
    let affected = affected_paths(&journal.changed_paths);
    ensure_safe_skipped_paths(&current.skipped_paths, &affected)?;
    let changed_from_start = store
        .changed_paths(&identity.root, start_snapshot_id, current_snapshot_id)
        .await?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let changed_from_end = store
        .changed_paths(&identity.root, end_snapshot_id, current_snapshot_id)
        .await?
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut conflicts = affected
        .iter()
        .filter(|path| {
            overlaps_any(path, &changed_from_start) && overlaps_any(path, &changed_from_end)
        })
        .map(|path| (*path).to_string())
        .collect::<BTreeSet<_>>();
    conflicts.extend(
        current
            .skipped_paths
            .into_iter()
            .map(|path| path.path)
            .filter(|path| {
                affected
                    .iter()
                    .any(|affected| paths_overlap(affected, path))
            }),
    );
    Ok(conflicts.into_iter().collect())
}

fn affected_paths(changed_paths: &[String]) -> BTreeSet<&str> {
    changed_paths.iter().map(String::as_str).collect()
}

fn ensure_safe_skipped_paths(
    skipped_paths: &[SkippedPath],
    affected: &BTreeSet<&str>,
) -> Result<(), String> {
    if let Some(blocked) = skipped_paths.iter().find(|path| {
        affected
            .iter()
            .any(|affected| paths_overlap(affected, &path.path))
            && matches!(
                path.reason,
                SkippedPathReason::NestedGitRepository
                    | SkippedPathReason::UnverifiedLfsObject
                    | SkippedPathReason::UnverifiedContentFilter
            )
    }) {
        return Err(format!(
            "The affected path '{}' cannot be verified for safe restoration.",
            blocked.path
        ));
    }
    Ok(())
}

fn overlaps_any(path: &str, candidates: &BTreeSet<String>) -> bool {
    candidates
        .iter()
        .any(|candidate| paths_overlap(path, candidate))
}

fn paths_overlap(left: &str, right: &str) -> bool {
    left == right
        || left
            .strip_prefix(right)
            .is_some_and(|suffix| suffix.starts_with('/'))
        || right
            .strip_prefix(left)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

async fn apply_journal_target(
    store: &CheckpointStore,
    identity: &ProjectIdentity,
    journal: &store::TurnRecoveryJournal,
    action: &str,
) -> Result<(), String> {
    let target = match action {
        "undo" => journal.start_snapshot_id.as_deref(),
        "redo" => journal.end_snapshot_id.as_deref(),
        _ => return Err("Unknown turn recovery action.".to_string()),
    };
    match (
        target,
        journal.start_snapshot_id.as_deref(),
        journal.end_snapshot_id.as_deref(),
    ) {
        (Some(target), Some(_), Some(_)) => store
            .git
            .restore_paths(identity, target, &journal.changed_paths)
            .await
            .map_err(|error| error.to_string()),
        (None, None, None) => Ok(()),
        _ => Err("The checkpoint does not contain both project boundaries.".to_string()),
    }
}

fn journal_result(
    journal: &store::TurnRecoveryJournal,
    action: &'static str,
    conflicts: Vec<String>,
    confirmation_required: bool,
    applied: bool,
) -> TurnRecoveryResult {
    TurnRecoveryResult {
        action,
        chat_id: journal.chat_id.clone(),
        user_message_id: journal.user_message_id.clone(),
        coverage: journal.capture.coverage,
        changed_paths: journal.changed_paths.clone(),
        skipped_paths: journal.capture.skipped_paths.clone(),
        conflicts,
        confirmation_required,
        project_restored: applied
            && journal.start_snapshot_id.is_some()
            && !journal.changed_paths.is_empty(),
        conversation_rewound: confirmation_required,
        rewind_boundary_sequence: journal.boundary_sequence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::OsStr,
        fs,
        path::PathBuf,
        process::Command,
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(name: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "fennara-recovery-{name}-{nonce}-{}",
                TEST_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn git<I, S>(root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "Git failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[tokio::test]
    async fn preflight_reports_only_later_edits_to_affected_paths() {
        let project = TestDirectory::new("conflict-project");
        let storage = TestDirectory::new("conflict-storage");
        git(&project.0, ["init", "--quiet"]);
        git(&project.0, ["config", "user.name", "Fennara Tests"]);
        git(
            &project.0,
            ["config", "user.email", "fennara-tests@example.invalid"],
        );
        fs::write(project.0.join("affected.txt"), "before\n").unwrap();
        fs::write(project.0.join("unrelated.txt"), "before\n").unwrap();
        git(&project.0, ["add", "--all"]);
        git(
            &project.0,
            [
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "-m",
                "test",
            ],
        );

        let store = CheckpointStore::at(storage.0.clone());
        let _start = store.capture(&project.0).await;
        fs::write(project.0.join("affected.txt"), "agent\n").unwrap();
        let end = store.capture(&project.0).await;
        fs::write(project.0.join("affected.txt"), "manual\n").unwrap();
        fs::write(project.0.join("unrelated.txt"), "manual\n").unwrap();
        let identity = store.git.identify(&project.0).await.unwrap();

        let conflicts = preflight_conflicts(
            &store,
            &identity,
            end.snapshot_id.as_deref(),
            &["affected.txt".to_string()],
        )
        .await
        .unwrap();

        assert_eq!(conflicts, vec!["affected.txt"]);

        fs::write(
            project.0.join(".gitattributes"),
            "affected.txt filter=custom\n",
        )
        .unwrap();
        let error = preflight_conflicts(
            &store,
            &identity,
            end.snapshot_id.as_deref(),
            &["affected.txt".to_string()],
        )
        .await
        .unwrap_err();
        assert!(error.contains("affected.txt"));
    }

    #[tokio::test]
    async fn interrupted_recovery_accepts_boundary_mix_but_flags_third_state() {
        let project = TestDirectory::new("interrupted-project");
        let storage = TestDirectory::new("interrupted-storage");
        git(&project.0, ["init", "--quiet"]);
        git(&project.0, ["config", "user.name", "Fennara Tests"]);
        git(
            &project.0,
            ["config", "user.email", "fennara-tests@example.invalid"],
        );
        fs::write(project.0.join("one.txt"), "before\n").unwrap();
        fs::write(project.0.join("two.txt"), "before\n").unwrap();
        git(&project.0, ["add", "--all"]);
        git(
            &project.0,
            [
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "-m",
                "test",
            ],
        );

        let store = CheckpointStore::at(storage.0.clone());
        let start = store.capture(&project.0).await;
        fs::write(project.0.join("one.txt"), "after\n").unwrap();
        fs::write(project.0.join("two.txt"), "after\n").unwrap();
        let end = store.capture(&project.0).await;
        let identity = store.git.identify(&project.0).await.unwrap();
        let journal = store::TurnRecoveryJournal {
            chat_id: "chat".to_string(),
            checkpoint_id: "checkpoint".to_string(),
            user_message_id: "message".to_string(),
            operation_id: "recovery".to_string(),
            state: "applying_undo".to_string(),
            boundary_sequence: 1,
            project_path: project.0.to_string_lossy().into_owned(),
            storage_key: identity.storage_key.clone(),
            start_snapshot_id: start.snapshot_id,
            end_snapshot_id: end.snapshot_id.clone(),
            changed_paths: vec!["one.txt".to_string(), "two.txt".to_string()],
            capture: end,
        };

        fs::write(project.0.join("one.txt"), "before\n").unwrap();
        let conflicts = interrupted_recovery_conflicts(&store, &identity, &journal)
            .await
            .unwrap();
        assert!(conflicts.is_empty());

        fs::write(project.0.join("one.txt"), "manual\n").unwrap();
        let conflicts = interrupted_recovery_conflicts(&store, &identity, &journal)
            .await
            .unwrap();
        assert_eq!(conflicts, vec!["one.txt"]);
    }

    #[test]
    fn nested_safety_paths_overlap_their_descendants() {
        assert!(paths_overlap("vendor", "vendor/file.gd"));
        assert!(paths_overlap("vendor/file.gd", "vendor"));
        assert!(paths_overlap("vendor", "vendor"));
        assert!(!paths_overlap("vendor", "vendor-two/file.gd"));
    }
}
