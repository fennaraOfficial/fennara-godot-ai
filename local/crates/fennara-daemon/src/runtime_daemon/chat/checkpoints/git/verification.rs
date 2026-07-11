use std::{
    collections::{BTreeSet, HashSet},
    ffi::{OsStr, OsString},
    io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use tokio::{process::Command, time::timeout};

use super::{
    CAPTURE_TIMEOUT, GitSnapshotStore, ProjectIdentity, Repository, SnapshotError,
    canonical_project_root, ensure_no_symlink_ancestors, ensure_success, execute_git,
    parse_nul_paths, path_batches, project_storage_key, snapshot_paths_at, source_content_filters,
    source_git_literal, source_gitlink_paths, untracked_skip_reason, validate_snapshot_id,
};
use crate::runtime_daemon::chat::checkpoints::{SkippedPath, SkippedPathReason};

static INDEX_COUNTER: AtomicU64 = AtomicU64::new(0);

pub(in crate::runtime_daemon::chat::checkpoints) struct PathVerification {
    pub(in crate::runtime_daemon::chat::checkpoints) changed_paths: Vec<String>,
    pub(in crate::runtime_daemon::chat::checkpoints) skipped_paths: Vec<SkippedPath>,
}

struct PathInspection {
    present_paths: BTreeSet<String>,
    skipped_paths: Vec<SkippedPath>,
}

pub(super) async fn verify_paths(
    store: &GitSnapshotStore,
    identity: &ProjectIdentity,
    expected_snapshot_id: &str,
    project_paths: &[String],
) -> Result<PathVerification, SnapshotError> {
    validate_snapshot_id(expected_snapshot_id)?;
    if project_paths.is_empty() {
        return Ok(empty_verification());
    }
    let canonical_project = canonical_project_root(&identity.root).await?;
    if canonical_project != identity.root
        || project_storage_key(&canonical_project)? != identity.storage_key
    {
        return Err(SnapshotError::ProjectIdentityChanged);
    }
    let project_lock = store.project_lock(&canonical_project).await;
    timeout(CAPTURE_TIMEOUT, async {
        let _guard = project_lock.lock().await;
        let repository = store.repository(&canonical_project).await?;
        let worktree_paths = project_paths
            .iter()
            .map(|path| repository.project_path_to_worktree(path))
            .collect::<Result<Vec<_>, _>>()?;
        ensure_no_symlink_ancestors(&repository.worktree, &worktree_paths).await?;
        let PathInspection {
            present_paths,
            skipped_paths,
        } = inspect_paths(&repository, expected_snapshot_id, &worktree_paths).await?;
        if skipped_paths.iter().any(|path| {
            matches!(
                path.reason,
                SkippedPathReason::NestedGitRepository
                    | SkippedPathReason::UnverifiedLfsObject
                    | SkippedPathReason::UnverifiedContentFilter
            )
        }) {
            return Ok(PathVerification {
                changed_paths: Vec::new(),
                skipped_paths,
            });
        }
        let verifiable_paths = project_paths
            .iter()
            .zip(&worktree_paths)
            .filter(|(project_path, worktree_path)| {
                !skipped_paths
                    .iter()
                    .any(|skipped| paths_overlap(project_path, &skipped.path))
                    && present_paths
                        .iter()
                        .any(|present| paths_overlap(worktree_path, present))
            })
            .map(|(_, worktree_path)| worktree_path.clone())
            .collect::<Vec<_>>();
        if verifiable_paths.is_empty() {
            return Ok(PathVerification {
                changed_paths: Vec::new(),
                skipped_paths,
            });
        }

        let index = TemporaryIndex::new(&repository.private_git_dir);
        let output = verification_git(
            &repository,
            &index.path,
            ["read-tree", expected_snapshot_id],
        )
        .await?;
        ensure_success("seed recovery verification index", output)?;
        for batch in path_batches(&verifiable_paths) {
            let mut args = vec![
                OsString::from("add"),
                OsString::from("--all"),
                OsString::from("--"),
            ];
            args.extend(batch.iter().map(OsString::from));
            let output = verification_git(&repository, &index.path, args).await?;
            ensure_success("verify recovery paths", output)?;
        }
        let output = verification_git(
            &repository,
            &index.path,
            [
                "diff",
                "--cached",
                "--name-only",
                "-z",
                expected_snapshot_id,
            ],
        )
        .await?;
        let output = ensure_success("list recovery path conflicts", output)?;
        let changed_paths = parse_nul_paths(&output.stdout)?
            .into_iter()
            .filter_map(|path| repository.to_project_path(&path))
            .collect();
        Ok(PathVerification {
            changed_paths,
            skipped_paths,
        })
    })
    .await
    .map_err(|_| SnapshotError::TimedOut)?
}

fn empty_verification() -> PathVerification {
    PathVerification {
        changed_paths: Vec::new(),
        skipped_paths: Vec::new(),
    }
}

struct TemporaryIndex {
    path: PathBuf,
}

impl TemporaryIndex {
    fn new(private_git_dir: &Path) -> Self {
        let nonce = INDEX_COUNTER.fetch_add(1, Ordering::Relaxed);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self {
            path: private_git_dir.join(format!(
                "fennara-verify-{}-{timestamp}-{nonce}.index",
                std::process::id()
            )),
        }
    }
}

impl Drop for TemporaryIndex {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
        let mut lock = self.path.as_os_str().to_os_string();
        lock.push(".lock");
        let _ = std::fs::remove_file(PathBuf::from(lock));
    }
}

async fn inspect_paths(
    repository: &Repository,
    expected_snapshot_id: &str,
    paths: &[String],
) -> Result<PathInspection, SnapshotError> {
    let expected = snapshot_paths_at(repository, expected_snapshot_id, paths).await?;
    let tracked = list_source_paths(repository, ["ls-files", "--cached", "-z"], paths).await?;
    let untracked = list_source_paths(
        repository,
        ["ls-files", "--others", "--exclude-standard", "-z"],
        paths,
    )
    .await?;
    let ignored = list_source_paths(
        repository,
        [
            "ls-files",
            "--others",
            "--ignored",
            "--exclude-standard",
            "-z",
        ],
        paths,
    )
    .await?;
    let present_paths = expected
        .into_iter()
        .chain(tracked)
        .chain(untracked.iter().cloned())
        .chain(ignored.iter().cloned())
        .collect::<BTreeSet<_>>();
    let inspection_paths = paths
        .iter()
        .cloned()
        .chain(present_paths.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let gitlinks = source_gitlink_paths(repository, &inspection_paths).await?;
    let nested = nested_repository_paths(repository, &inspection_paths).await?;
    let filters = source_content_filters(repository, &inspection_paths).await?;

    let mut skipped = ignored
        .into_iter()
        .map(|path| repository.skipped(&path, SkippedPathReason::IgnoredPath))
        .chain(
            gitlinks
                .into_iter()
                .chain(nested)
                .map(|path| repository.skipped(&path, SkippedPathReason::NestedGitRepository)),
        )
        .chain(filters.into_iter().map(|(path, filter)| {
            let reason = if filter == "lfs" {
                SkippedPathReason::UnverifiedLfsObject
            } else {
                SkippedPathReason::UnverifiedContentFilter
            };
            repository.skipped(&path, reason)
        }))
        .collect::<Vec<_>>();
    let mut untracked_bytes = 0_u64;
    for path in untracked {
        let Ok(metadata) =
            tokio::fs::symlink_metadata(repository.absolute_worktree_path(&path)).await
        else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        if let Some(reason) = untracked_skip_reason(metadata.len(), untracked_bytes) {
            skipped.push(repository.skipped(&path, reason));
        } else {
            untracked_bytes += metadata.len();
        }
    }
    skipped.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| reason_rank(left.reason).cmp(&reason_rank(right.reason)))
    });
    skipped.dedup();
    Ok(PathInspection {
        present_paths,
        skipped_paths: skipped,
    })
}

async fn list_source_paths<const N: usize>(
    repository: &Repository,
    prefix: [&str; N],
    paths: &[String],
) -> Result<Vec<String>, SnapshotError> {
    let mut found = BTreeSet::new();
    for batch in path_batches(paths) {
        let mut args = prefix.iter().map(OsString::from).collect::<Vec<_>>();
        args.push(OsString::from("--"));
        args.extend(batch.iter().map(OsString::from));
        let output = source_git_literal(repository, args).await?;
        let output = ensure_success("inspect recovery paths", output)?;
        found.extend(parse_nul_paths(&output.stdout)?);
    }
    Ok(found.into_iter().collect())
}

async fn nested_repository_paths(
    repository: &Repository,
    paths: &[String],
) -> Result<HashSet<String>, SnapshotError> {
    let mut nested = HashSet::new();
    for path in paths {
        let mut candidate = repository.worktree.clone();
        for component in path.split('/') {
            candidate.push(component);
            match tokio::fs::symlink_metadata(candidate.join(".git")).await {
                Ok(_) => {
                    nested.insert(path.clone());
                    break;
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => return Err(SnapshotError::InspectRestorePath(error)),
            }
        }
    }
    Ok(nested)
}

fn reason_rank(reason: SkippedPathReason) -> u8 {
    match reason {
        SkippedPathReason::IgnoredPath => 0,
        SkippedPathReason::LargeUntrackedFile => 1,
        SkippedPathReason::NestedGitRepository => 2,
        SkippedPathReason::UntrackedByteBudgetExceeded => 3,
        SkippedPathReason::UnverifiedLfsObject => 4,
        SkippedPathReason::UnverifiedContentFilter => 5,
    }
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

async fn verification_git<I, S>(
    repository: &Repository,
    index_path: &Path,
    args: I,
) -> Result<super::GitOutput, SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command
        .arg("--literal-pathspecs")
        .arg("--git-dir")
        .arg(&repository.private_git_dir)
        .arg("--work-tree")
        .arg(&repository.worktree)
        .current_dir(&repository.worktree)
        .env("GIT_INDEX_FILE", index_path);
    execute_git(&mut command, args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::checkpoints::git::tests::TestRepository;
    use std::fs;

    #[tokio::test]
    async fn verifies_only_affected_paths_with_an_isolated_index() {
        let repository = TestRepository::new("verify-affected");
        fs::write(repository.root.path.join("changed.txt"), "before\n").unwrap();
        fs::write(repository.root.path.join("deleted.bin"), [0_u8, 1, 255]).unwrap();
        fs::create_dir(repository.root.path.join("folder")).unwrap();
        fs::write(
            repository.root.path.join("folder").join("inside.bin"),
            [1_u8, 0, 255],
        )
        .unwrap();
        fs::write(repository.root.path.join("unrelated.txt"), "before\n").unwrap();
        repository.commit_all();

        let store = repository.store();
        let expected = store.capture(&repository.root.path).await;
        let identity = store.identify(&repository.root.path).await.unwrap();
        let source_index = repository.root.path.join(".git").join("index");
        let source_index_before = fs::read(&source_index).unwrap();
        let private_index = repository
            .storage
            .path
            .join(&identity.storage_key)
            .join("index");
        let private_index_before = fs::read(&private_index).unwrap();

        fs::write(repository.root.path.join("changed.txt"), "after\n").unwrap();
        fs::remove_file(repository.root.path.join("deleted.bin")).unwrap();
        fs::write(repository.root.path.join("created.bin"), [255_u8, 7, 0]).unwrap();
        fs::remove_dir_all(repository.root.path.join("folder")).unwrap();
        fs::write(repository.root.path.join("folder"), [9_u8, 8, 7]).unwrap();
        fs::write(repository.root.path.join("unrelated.txt"), "manual\n").unwrap();

        let verification = store
            .verify_paths(
                &identity,
                expected.snapshot_id.as_deref().unwrap(),
                &[
                    "changed.txt".to_string(),
                    "created.bin".to_string(),
                    "deleted.bin".to_string(),
                    "folder".to_string(),
                    "folder/inside.bin".to_string(),
                ],
            )
            .await
            .unwrap();

        assert_eq!(
            verification.changed_paths,
            [
                "changed.txt",
                "created.bin",
                "deleted.bin",
                "folder",
                "folder/inside.bin",
            ]
        );
        assert!(verification.skipped_paths.is_empty());
        assert_eq!(fs::read(&source_index).unwrap(), source_index_before);
        assert_eq!(fs::read(&private_index).unwrap(), private_index_before);
        assert!(
            fs::read_dir(repository.storage.path.join(&identity.storage_key))
                .unwrap()
                .all(|entry| !entry
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with("fennara-verify-"))
        );

        fs::remove_file(repository.root.path.join("created.bin")).unwrap();
        let absent = store
            .verify_paths(
                &identity,
                expected.snapshot_id.as_deref().unwrap(),
                &["created.bin".to_string()],
            )
            .await
            .unwrap();
        assert!(absent.changed_paths.is_empty());
        assert!(absent.skipped_paths.is_empty());
    }

    #[tokio::test]
    async fn reports_unsafe_untracked_paths_and_still_verifies_safe_paths() {
        let repository = TestRepository::new("verify-skipped");
        fs::write(repository.root.path.join("tracked.txt"), "baseline\n").unwrap();
        repository.commit_all();
        let store = repository.store();
        let expected = store.capture(&repository.root.path).await;
        let identity = store.identify(&repository.root.path).await.unwrap();

        fs::write(repository.root.path.join(".gitignore"), "ignored.bin\n").unwrap();
        fs::write(repository.root.path.join("ignored.bin"), [1_u8, 2, 3]).unwrap();
        fs::write(repository.root.path.join("safe.txt"), "manual\n").unwrap();
        fs::write(
            repository.root.path.join("large.bin"),
            vec![7_u8; super::super::MAX_UNTRACKED_FILE_BYTES as usize + 1],
        )
        .unwrap();
        let verification = store
            .verify_paths(
                &identity,
                expected.snapshot_id.as_deref().unwrap(),
                &[
                    "ignored.bin".to_string(),
                    "large.bin".to_string(),
                    "safe.txt".to_string(),
                ],
            )
            .await
            .unwrap();

        assert_eq!(verification.changed_paths, ["safe.txt"]);
        assert_eq!(
            verification.skipped_paths,
            [
                SkippedPath {
                    path: "ignored.bin".to_string(),
                    reason: SkippedPathReason::IgnoredPath,
                },
                SkippedPath {
                    path: "large.bin".to_string(),
                    reason: SkippedPathReason::LargeUntrackedFile,
                },
            ]
        );
    }
}
