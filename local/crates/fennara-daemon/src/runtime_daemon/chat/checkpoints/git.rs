use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ffi::{OsStr, OsString},
    fmt, io,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Duration,
};
use tokio::{io::AsyncWriteExt, process::Command, sync::Mutex, time::timeout};

use super::{CaptureResult, CaptureUnavailableReason, SkippedPath, SkippedPathReason};

const CAPTURE_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_CAPTURE_CANDIDATES: usize = 20_000;
const MAX_UNTRACKED_FILE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_UNTRACKED_CAPTURE_BYTES: u64 = 32 * 1024 * 1024;
const SOURCE_PATH_BATCH_SIZE: usize = 64;

#[derive(Clone)]
pub(super) struct GitSnapshotStore {
    storage_root: PathBuf,
    project_locks: Arc<Mutex<HashMap<PathBuf, Arc<Mutex<()>>>>>,
}

#[derive(Clone, Debug)]
pub(super) struct ProjectIdentity {
    pub(super) root: PathBuf,
    pub(super) storage_key: String,
}

impl GitSnapshotStore {
    pub(super) fn new(storage_root: PathBuf) -> Self {
        Self {
            storage_root,
            project_locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(super) async fn capture(&self, project_root: &Path) -> CaptureResult {
        let canonical_project = match canonical_project_root(project_root).await {
            Ok(path) => path,
            Err(error) => return CaptureResult::unavailable(error.unavailable_reason()),
        };
        let project_lock = self.project_lock(&canonical_project).await;
        let capture = async {
            let _guard = project_lock.lock().await;
            let repository = self.repository(&canonical_project).await?;
            let skipped_paths = refresh_index(&repository).await?;
            let snapshot_id = write_tree(&repository).await?;
            Ok::<_, SnapshotError>(CaptureResult::available(snapshot_id, skipped_paths))
        };
        match timeout(CAPTURE_TIMEOUT, capture).await {
            Ok(Ok(result)) => result,
            Ok(Err(error)) => CaptureResult::unavailable(error.unavailable_reason()),
            Err(_) => CaptureResult::unavailable(CaptureUnavailableReason::TimedOut),
        }
    }

    pub(super) async fn identify(
        &self,
        project_root: &Path,
    ) -> Result<ProjectIdentity, SnapshotError> {
        let root = canonical_project_root(project_root).await?;
        let storage_key = project_storage_key(&root)?;
        Ok(ProjectIdentity { root, storage_key })
    }

    pub(super) async fn pin_snapshot(
        &self,
        identity: &ProjectIdentity,
        checkpoint_id: &str,
        boundary: &str,
        snapshot_id: &str,
    ) -> Result<(), SnapshotError> {
        validate_snapshot_id(snapshot_id)?;
        let reference = checkpoint_ref(checkpoint_id, boundary)?;
        let materialized = materialized_ref(checkpoint_id, boundary)?;
        let repository = self.repository(&identity.root).await?;
        materialize_snapshot(&repository, snapshot_id).await?;
        update_snapshot_ref(&repository, &materialized, snapshot_id).await?;
        update_snapshot_ref(&repository, &reference, snapshot_id).await
    }

    pub(super) async fn release_checkpoint(
        &self,
        storage_key: &str,
        checkpoint_id: &str,
    ) -> Result<(), SnapshotError> {
        validate_storage_key(storage_key)?;
        let private_git_dir = self.storage_root.join(storage_key);
        if !private_git_dir.is_dir() {
            return Ok(());
        }
        for boundary in ["start", "end"] {
            for reference in [
                checkpoint_ref(checkpoint_id, boundary)?,
                materialized_ref(checkpoint_id, boundary)?,
            ] {
                delete_snapshot_ref(&private_git_dir, &reference).await?;
            }
        }
        Ok(())
    }

    pub(super) async fn compact_storage(&self, storage_key: &str) -> Result<(), SnapshotError> {
        validate_storage_key(storage_key)?;
        let private_git_dir = self.storage_root.join(storage_key);
        if !private_git_dir.is_dir() {
            return Ok(());
        }
        let compact = async {
            let mut command = Command::new("git");
            command.arg("--git-dir").arg(&private_git_dir);
            let output = execute_git(&mut command, ["gc", "--prune=now", "--quiet"]).await?;
            ensure_success("compact checkpoint storage", output).map(|_| ())
        };
        timeout(CAPTURE_TIMEOUT, compact)
            .await
            .map_err(|_| SnapshotError::TimedOut)?
    }

    pub(super) async fn changed_paths(
        &self,
        project_root: &Path,
        from_snapshot: &str,
        to_snapshot: &str,
    ) -> Result<Vec<String>, SnapshotError> {
        validate_snapshot_id(from_snapshot)?;
        validate_snapshot_id(to_snapshot)?;
        let canonical_project = canonical_project_root(project_root).await?;
        let project_lock = self.project_lock(&canonical_project).await;
        timeout(CAPTURE_TIMEOUT, async {
            let _guard = project_lock.lock().await;
            let repository = self.repository(&canonical_project).await?;
            let output = snapshot_git(
                &repository,
                [
                    "diff",
                    "--name-only",
                    "-z",
                    from_snapshot,
                    to_snapshot,
                    "--",
                    repository.scope.as_str(),
                ],
            )
            .await?;
            let output = ensure_success("list changed paths", output)?;
            Ok(parse_nul_paths(&output.stdout)?
                .into_iter()
                .filter(|path| !repository.is_generated_path(path))
                .filter_map(|path| repository.to_project_path(&path))
                .collect())
        })
        .await
        .map_err(|_| SnapshotError::TimedOut)?
    }

    async fn project_lock(&self, canonical_project: &Path) -> Arc<Mutex<()>> {
        let mut locks = self.project_locks.lock().await;
        locks.retain(|_, lock| Arc::strong_count(lock) > 1);
        locks
            .entry(canonical_project.to_path_buf())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }

    async fn repository(&self, canonical_project: &Path) -> Result<Repository, SnapshotError> {
        let source = discover_repository(canonical_project).await?;
        let scope_path = canonical_project
            .strip_prefix(&source.worktree)
            .map_err(|_| SnapshotError::ProjectOutsideWorktree)?;
        let scope = if scope_path.as_os_str().is_empty() {
            ".".to_string()
        } else {
            path_to_git(scope_path)?
        };
        let private_git_dir = self
            .storage_root
            .join(project_storage_key(canonical_project)?);
        let repository = Repository {
            worktree: source.worktree,
            source_common_dir: source.common_dir,
            private_git_dir,
            scope,
        };
        ensure_private_repository(&repository).await?;
        Ok(repository)
    }
}

async fn materialize_snapshot(
    repository: &Repository,
    snapshot_id: &str,
) -> Result<(), SnapshotError> {
    let output = snapshot_git(
        repository,
        [
            "for-each-ref",
            "--format=%(objectname)",
            "refs/fennara/materialized/",
        ],
    )
    .await?;
    let output = ensure_success("list materialized checkpoint snapshots", output)?;
    let materialized = parse_line_paths(&output.stdout)?;
    for snapshot in &materialized {
        validate_snapshot_id(snapshot)?;
    }
    let pack_dir = repository.private_git_dir.join("objects").join("pack");
    tokio::fs::create_dir_all(&pack_dir)
        .await
        .map_err(SnapshotError::CreateStorage)?;
    let pack_prefix = pack_dir.join("fennara-checkpoint");
    let mut input = format!("{snapshot_id}\n").into_bytes();
    for snapshot in materialized {
        input.extend_from_slice(format!("^{snapshot}\n").as_bytes());
    }
    let args = vec![
        OsString::from("pack-objects"),
        OsString::from("--quiet"),
        OsString::from("--revs"),
        OsString::from("--non-empty"),
        pack_prefix.into_os_string(),
    ];
    let output = timeout(
        CAPTURE_TIMEOUT,
        snapshot_git_with_input(repository, args, &input),
    )
    .await
    .map_err(|_| SnapshotError::TimedOut)??;
    ensure_success("materialize checkpoint snapshot", output).map(|_| ())
}

async fn update_snapshot_ref(
    repository: &Repository,
    reference: &str,
    snapshot_id: &str,
) -> Result<(), SnapshotError> {
    let output = timeout(
        CAPTURE_TIMEOUT,
        snapshot_git(repository, ["update-ref", reference, snapshot_id]),
    )
    .await
    .map_err(|_| SnapshotError::TimedOut)??;
    ensure_success("pin checkpoint snapshot", output).map(|_| ())
}

async fn delete_snapshot_ref(private_git_dir: &Path, reference: &str) -> Result<(), SnapshotError> {
    let mut command = Command::new("git");
    command.arg("--git-dir").arg(private_git_dir);
    let output = timeout(
        CAPTURE_TIMEOUT,
        execute_git(&mut command, ["update-ref", "-d", reference]),
    )
    .await
    .map_err(|_| SnapshotError::TimedOut)??;
    ensure_success("release checkpoint snapshot", output).map(|_| ())
}

#[derive(Debug)]
struct Repository {
    worktree: PathBuf,
    source_common_dir: PathBuf,
    private_git_dir: PathBuf,
    scope: String,
}

impl Repository {
    fn generated_paths(&self) -> [String; 2] {
        let prefix = if self.scope == "." {
            String::new()
        } else {
            format!("{}/", self.scope)
        };
        [format!("{prefix}.godot"), format!("{prefix}.import")]
    }

    fn is_generated_path(&self, path: &str) -> bool {
        path.ends_with(".translation")
            || self
                .generated_paths()
                .iter()
                .any(|generated| path == generated || path.starts_with(&format!("{generated}/")))
    }

    fn to_project_path(&self, worktree_path: &str) -> Option<String> {
        if self.scope == "." {
            return Some(worktree_path.to_string());
        }
        worktree_path
            .strip_prefix(&self.scope)
            .and_then(|path| path.strip_prefix('/'))
            .map(ToOwned::to_owned)
    }

    fn absolute_worktree_path(&self, git_path: &str) -> PathBuf {
        git_path
            .split('/')
            .fold(self.worktree.clone(), |path, component| {
                path.join(component)
            })
    }
}

#[derive(Debug)]
struct SourceRepository {
    worktree: PathBuf,
    common_dir: PathBuf,
}

async fn canonical_project_root(project_root: &Path) -> Result<PathBuf, SnapshotError> {
    let canonical = tokio::fs::canonicalize(project_root)
        .await
        .map_err(SnapshotError::InvalidProjectRoot)?;
    let metadata = tokio::fs::metadata(&canonical)
        .await
        .map_err(SnapshotError::InvalidProjectRoot)?;
    if !metadata.is_dir() {
        return Err(SnapshotError::ProjectRootNotDirectory);
    }
    Ok(canonical)
}

async fn discover_repository(project_root: &Path) -> Result<SourceRepository, SnapshotError> {
    let worktree = rev_parse_path(project_root, "--show-toplevel").await?;
    let common_dir = rev_parse_path_with_absolute_format(project_root, "--git-common-dir").await?;
    Ok(SourceRepository {
        worktree: tokio::fs::canonicalize(worktree)
            .await
            .map_err(SnapshotError::InvalidRepositoryPath)?,
        common_dir: tokio::fs::canonicalize(common_dir)
            .await
            .map_err(SnapshotError::InvalidRepositoryPath)?,
    })
}

async fn rev_parse_path(project_root: &Path, argument: &str) -> Result<PathBuf, SnapshotError> {
    let mut command = Command::new("git");
    command.arg("-C").arg(project_root);
    let output = execute_git(&mut command, ["rev-parse", argument]).await?;
    if !output.success {
        return Err(SnapshotError::NonGitProject);
    }
    output_path(output.stdout)
}

async fn rev_parse_path_with_absolute_format(
    project_root: &Path,
    argument: &str,
) -> Result<PathBuf, SnapshotError> {
    let mut command = Command::new("git");
    command.arg("-C").arg(project_root);
    let output = execute_git(
        &mut command,
        ["rev-parse", "--path-format=absolute", argument],
    )
    .await?;
    if !output.success {
        return Err(SnapshotError::NonGitProject);
    }
    output_path(output.stdout)
}

fn output_path(mut bytes: Vec<u8>) -> Result<PathBuf, SnapshotError> {
    while matches!(bytes.last(), Some(b'\n' | b'\r')) {
        bytes.pop();
    }
    let path = String::from_utf8(bytes).map_err(|_| SnapshotError::NonUtf8GitOutput)?;
    if path.is_empty() {
        return Err(SnapshotError::InvalidRepositoryOutput);
    }
    Ok(PathBuf::from(path))
}

async fn ensure_private_repository(repository: &Repository) -> Result<(), SnapshotError> {
    tokio::fs::create_dir_all(&repository.private_git_dir)
        .await
        .map_err(SnapshotError::CreateStorage)?;
    restrict_storage_permissions(&repository.private_git_dir).await?;
    if !repository.private_git_dir.join("HEAD").is_file() {
        let output = snapshot_git(repository, ["init", "--quiet"]).await?;
        ensure_success("initialize private checkpoint repository", output)?;
    }
    let initialization_marker = repository.private_git_dir.join("fennara-initialized");
    if !initialization_marker.is_file() {
        for (key, value) in [
            ("core.autocrlf", "false"),
            ("core.longpaths", "true"),
            ("core.symlinks", "true"),
            ("core.fsmonitor", "false"),
            ("feature.manyFiles", "true"),
            ("index.version", "4"),
            ("index.threads", "true"),
            ("core.untrackedCache", "true"),
        ] {
            let output = snapshot_git(repository, ["config", key, value]).await?;
            ensure_success("configure private checkpoint repository", output)?;
        }
        configure_alternates(repository).await?;
        if !repository.private_git_dir.join("index").is_file() {
            seed_private_index(repository).await?;
        }
        refresh_private_index_stat_cache(repository).await?;
        tokio::fs::write(&initialization_marker, b"1\n")
            .await
            .map_err(SnapshotError::CreateStorage)?;
    }
    configure_alternates(repository).await?;
    configure_generated_excludes(repository).await?;
    remove_from_index(repository, &repository.generated_paths()).await?;
    Ok(())
}

#[cfg(unix)]
async fn restrict_storage_permissions(path: &Path) -> Result<(), SnapshotError> {
    use std::os::unix::fs::PermissionsExt;

    tokio::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .await
        .map_err(SnapshotError::CreateStorage)
}

#[cfg(not(unix))]
async fn restrict_storage_permissions(_path: &Path) -> Result<(), SnapshotError> {
    Ok(())
}

async fn seed_private_index(repository: &Repository) -> Result<(), SnapshotError> {
    let head = source_head(repository).await?;
    let args = if let Some(head) = head.as_deref() {
        ["read-tree", head]
    } else {
        ["read-tree", "--empty"]
    };
    let output = snapshot_git(repository, args).await?;
    ensure_success("seed private checkpoint index", output).map(|_| ())
}

async fn source_head(repository: &Repository) -> Result<Option<String>, SnapshotError> {
    let output = source_git(repository, ["rev-parse", "--verify", "HEAD"]).await?;
    if !output.success {
        return Ok(None);
    }
    let head = String::from_utf8(output.stdout)
        .map_err(|_| SnapshotError::NonUtf8GitOutput)?
        .trim()
        .to_string();
    validate_snapshot_id(&head)?;
    Ok(Some(head))
}

async fn refresh_private_index_stat_cache(repository: &Repository) -> Result<(), SnapshotError> {
    let output = snapshot_git(repository, ["update-index", "--refresh"]).await?;
    if output.success || output.code == Some(1) {
        Ok(())
    } else {
        Err(command_failed("refresh private checkpoint index", &output))
    }
}

async fn configure_alternates(repository: &Repository) -> Result<(), SnapshotError> {
    let directory = repository.private_git_dir.join("objects").join("info");
    tokio::fs::create_dir_all(&directory)
        .await
        .map_err(SnapshotError::CreateStorage)?;
    let objects = repository.source_common_dir.join("objects");
    let mut path = path_to_git(&objects)?;
    if path.contains('\n') || path.contains('\r') {
        return Err(SnapshotError::UnsupportedRepositoryPath);
    }
    path.push('\n');
    tokio::fs::write(directory.join("alternates"), path)
        .await
        .map_err(SnapshotError::CreateStorage)
}

async fn configure_generated_excludes(repository: &Repository) -> Result<(), SnapshotError> {
    let directory = repository.private_git_dir.join("info");
    tokio::fs::create_dir_all(&directory)
        .await
        .map_err(SnapshotError::CreateStorage)?;
    let contents = repository
        .generated_paths()
        .into_iter()
        .map(|path| format!("/{path}/\n"))
        .chain(std::iter::once("*.translation\n".to_string()))
        .collect::<String>();
    tokio::fs::write(directory.join("exclude"), contents)
        .await
        .map_err(SnapshotError::CreateStorage)
}

async fn refresh_index(repository: &Repository) -> Result<Vec<SkippedPath>, SnapshotError> {
    let tracked = list_snapshot_paths(
        repository,
        [
            "diff-files",
            "--name-only",
            "-z",
            "--",
            repository.scope.as_str(),
        ],
    )
    .await?;
    let private_untracked = list_snapshot_paths(
        repository,
        [
            "ls-files",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            repository.scope.as_str(),
        ],
    )
    .await?;
    let private_delta = private_delta_paths(repository).await?;
    let source_untracked = list_source_paths(
        repository,
        [
            "ls-files",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            repository.scope.as_str(),
        ],
    )
    .await?;
    let mut candidates = tracked
        .into_iter()
        .chain(private_untracked)
        .chain(private_delta)
        .collect::<BTreeSet<_>>();
    if candidates.len() > MAX_CAPTURE_CANDIDATES {
        return Err(SnapshotError::TooManyCandidates(candidates.len()));
    }
    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    let generated = candidates
        .iter()
        .filter(|path| repository.is_generated_path(path))
        .cloned()
        .collect::<BTreeSet<_>>();
    candidates.retain(|path| !generated.contains(path));
    let candidate_list = candidates.iter().cloned().collect::<Vec<_>>();
    let ignored = source_ignored_paths(repository, &candidate_list).await?;
    let source_tracked =
        source_tracked_paths(repository, &ignored.iter().cloned().collect::<Vec<_>>()).await?;
    let ignored_untracked = ignored
        .difference(&source_tracked)
        .cloned()
        .collect::<BTreeSet<_>>();
    candidates.retain(|path| !ignored_untracked.contains(path));
    let candidate_list = candidates.iter().cloned().collect::<Vec<_>>();
    let mut nested_repositories = source_gitlink_paths(repository, &candidate_list).await?;
    for path in &candidate_list {
        let absolute = repository.absolute_worktree_path(path);
        if tokio::fs::symlink_metadata(&absolute)
            .await
            .is_ok_and(|metadata| metadata.is_dir())
            && tokio::fs::symlink_metadata(absolute.join(".git"))
                .await
                .is_ok()
        {
            nested_repositories.insert(path.clone());
        }
    }
    candidates.retain(|path| !nested_repositories.contains(path));
    let candidate_list = candidates.iter().cloned().collect::<Vec<_>>();
    let filters = source_content_filters(repository, &candidate_list).await?;

    let source_untracked = source_untracked.into_iter().collect::<HashSet<_>>();
    let mut skipped = ignored_untracked
        .iter()
        .map(|path| repository.skipped(path, SkippedPathReason::IgnoredPath))
        .chain(
            nested_repositories
                .iter()
                .map(|path| repository.skipped(path, SkippedPathReason::NestedGitRepository)),
        )
        .collect::<Vec<_>>();
    let mut remove = ignored_untracked
        .into_iter()
        .chain(generated)
        .collect::<BTreeSet<_>>();
    let mut untracked_bytes = 0_u64;
    for path in &candidates {
        if let Some(filter) = filters.get(path) {
            remove.insert(path.clone());
            let reason = if filter == "lfs" {
                SkippedPathReason::UnverifiedLfsObject
            } else {
                SkippedPathReason::UnverifiedContentFilter
            };
            skipped.push(repository.skipped(path, reason));
            continue;
        }
        if !source_untracked.contains(path) {
            continue;
        }
        let Ok(metadata) =
            tokio::fs::symlink_metadata(repository.absolute_worktree_path(path)).await
        else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        if let Some(reason) = untracked_skip_reason(metadata.len(), untracked_bytes) {
            remove.insert(path.clone());
            skipped.push(repository.skipped(path, reason));
            continue;
        }
        untracked_bytes += metadata.len();
    }

    let stage = candidates
        .into_iter()
        .filter(|path| !remove.contains(path))
        .collect::<Vec<_>>();
    let remove = remove.into_iter().collect::<Vec<_>>();
    remove_from_index(repository, &remove).await?;
    add_to_index(repository, &stage).await?;
    skipped.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(skipped)
}

fn untracked_skip_reason(file_bytes: u64, captured_bytes: u64) -> Option<SkippedPathReason> {
    if file_bytes > MAX_UNTRACKED_FILE_BYTES {
        Some(SkippedPathReason::LargeUntrackedFile)
    } else if captured_bytes.saturating_add(file_bytes) > MAX_UNTRACKED_CAPTURE_BYTES {
        Some(SkippedPathReason::UntrackedByteBudgetExceeded)
    } else {
        None
    }
}

async fn private_delta_paths(repository: &Repository) -> Result<Vec<String>, SnapshotError> {
    if let Some(head) = source_head(repository).await? {
        let output = snapshot_git(
            repository,
            [
                "diff",
                "--cached",
                "--name-only",
                "-z",
                head.as_str(),
                "--",
                repository.scope.as_str(),
            ],
        )
        .await?;
        let output = ensure_success("list private checkpoint delta", output)?;
        parse_nul_paths(&output.stdout)
    } else {
        list_snapshot_paths(
            repository,
            [
                "ls-files",
                "--cached",
                "-z",
                "--",
                repository.scope.as_str(),
            ],
        )
        .await
    }
}

impl Repository {
    fn skipped(&self, worktree_path: &str, reason: SkippedPathReason) -> SkippedPath {
        SkippedPath {
            path: self
                .to_project_path(worktree_path)
                .unwrap_or_else(|| worktree_path.to_string()),
            reason,
        }
    }
}

async fn source_tracked_paths(
    repository: &Repository,
    paths: &[String],
) -> Result<HashSet<String>, SnapshotError> {
    let mut tracked = HashSet::new();
    for batch in paths.chunks(SOURCE_PATH_BATCH_SIZE) {
        let mut args = vec![
            OsString::from("ls-files"),
            OsString::from("--cached"),
            OsString::from("-z"),
            OsString::from("--"),
        ];
        args.extend(batch.iter().map(OsString::from));
        let output = source_git_literal(repository, args).await?;
        let output = ensure_success("list source tracked paths", output)?;
        tracked.extend(parse_nul_paths(&output.stdout)?);
    }
    Ok(tracked)
}

async fn source_gitlink_paths(
    repository: &Repository,
    paths: &[String],
) -> Result<HashSet<String>, SnapshotError> {
    let mut gitlinks = HashSet::new();
    for batch in paths.chunks(SOURCE_PATH_BATCH_SIZE) {
        let mut args = vec![
            OsString::from("ls-files"),
            OsString::from("--stage"),
            OsString::from("-z"),
            OsString::from("--"),
        ];
        args.extend(batch.iter().map(OsString::from));
        let output = source_git_literal(repository, args).await?;
        let output = ensure_success("list source gitlinks", output)?;
        for entry in parse_nul_paths(&output.stdout)? {
            let Some((metadata, path)) = entry.split_once('\t') else {
                return Err(SnapshotError::InvalidRepositoryOutput);
            };
            if metadata.starts_with("160000 ") {
                gitlinks.insert(path.to_string());
            }
        }
    }
    Ok(gitlinks)
}

async fn source_ignored_paths(
    repository: &Repository,
    paths: &[String],
) -> Result<HashSet<String>, SnapshotError> {
    let mut ignored = HashSet::new();
    let (ordinary, control): (Vec<_>, Vec<_>) = paths.iter().partition(|path| {
        !path
            .chars()
            .any(|character| character.is_control() || matches!(character, '"' | '\\'))
    });
    for batch in ordinary.chunks(SOURCE_PATH_BATCH_SIZE) {
        let mut args = vec![
            OsString::from("-c"),
            OsString::from("core.quotepath=false"),
            OsString::from("check-ignore"),
            OsString::from("--no-index"),
            OsString::from("--"),
        ];
        args.extend(batch.iter().map(|path| OsString::from(format!("./{path}"))));
        let output = source_git(repository, args).await?;
        if !output.success && output.code != Some(1) {
            return Err(command_failed("check source ignore rules", &output));
        }
        ignored.extend(
            parse_line_paths(&output.stdout)?
                .into_iter()
                .map(unprotect_check_path),
        );
    }
    for path in control {
        let protected = format!("./{path}");
        let output = source_git(
            repository,
            ["check-ignore", "--no-index", "--quiet", "--", &protected],
        )
        .await?;
        match output.code {
            Some(0) => {
                ignored.insert(path.clone());
            }
            Some(1) => {}
            _ => return Err(command_failed("check source ignore rules", &output)),
        }
    }
    Ok(ignored)
}

async fn source_content_filters(
    repository: &Repository,
    paths: &[String],
) -> Result<HashMap<String, String>, SnapshotError> {
    let mut filters = HashMap::new();
    for batch in paths.chunks(SOURCE_PATH_BATCH_SIZE) {
        let mut args = vec![
            OsString::from("check-attr"),
            OsString::from("-z"),
            OsString::from("filter"),
            OsString::from("--"),
        ];
        args.extend(batch.iter().map(|path| OsString::from(format!("./{path}"))));
        let output = source_git(repository, args).await?;
        let output = ensure_success("check content filter attributes", output)?;
        let fields = parse_nul_paths(&output.stdout)?;
        if fields.len() % 3 != 0 {
            return Err(SnapshotError::InvalidRepositoryOutput);
        }
        filters.extend(
            fields
                .chunks_exact(3)
                .filter(|entry| {
                    entry[1] == "filter" && !matches!(entry[2].as_str(), "unspecified" | "unset")
                })
                .map(|entry| (unprotect_check_path(entry[0].clone()), entry[2].clone())),
        );
    }
    Ok(filters)
}

async fn list_snapshot_paths<const N: usize>(
    repository: &Repository,
    args: [&str; N],
) -> Result<Vec<String>, SnapshotError> {
    let output = snapshot_git(repository, args).await?;
    let output = ensure_success("list checkpoint paths", output)?;
    parse_nul_paths(&output.stdout)
}

async fn list_source_paths<const N: usize>(
    repository: &Repository,
    args: [&str; N],
) -> Result<Vec<String>, SnapshotError> {
    let output = source_git(repository, args).await?;
    if !output.success {
        return Err(command_failed("list source paths", &output));
    }
    parse_nul_paths(&output.stdout)
}

async fn remove_from_index<I, S>(repository: &Repository, paths: I) -> Result<(), SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let paths = paths
        .into_iter()
        .map(|path| path.as_ref().to_string())
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Ok(());
    }
    for batch in paths.chunks(SOURCE_PATH_BATCH_SIZE) {
        let mut args = vec![
            OsString::from("rm"),
            OsString::from("--cached"),
            OsString::from("-r"),
            OsString::from("-f"),
            OsString::from("--ignore-unmatch"),
            OsString::from("--"),
        ];
        args.extend(batch.iter().map(OsString::from));
        let output = snapshot_git(repository, args).await?;
        ensure_success("remove paths from checkpoint index", output)?;
    }
    Ok(())
}

async fn add_to_index(repository: &Repository, paths: &[String]) -> Result<(), SnapshotError> {
    if paths.is_empty() {
        return Ok(());
    }
    for batch in paths.chunks(SOURCE_PATH_BATCH_SIZE) {
        let mut args = vec![
            OsString::from("add"),
            OsString::from("--all"),
            OsString::from("--"),
        ];
        args.extend(batch.iter().map(OsString::from));
        let output = snapshot_git(repository, args).await?;
        ensure_success("update checkpoint index", output)?;
    }
    Ok(())
}

async fn write_tree(repository: &Repository) -> Result<String, SnapshotError> {
    let output = snapshot_git(repository, ["write-tree"]).await?;
    let output = ensure_success("write checkpoint tree", output)?;
    let snapshot_id = String::from_utf8(output.stdout)
        .map_err(|_| SnapshotError::NonUtf8GitOutput)?
        .trim()
        .to_string();
    validate_snapshot_id(&snapshot_id)?;
    Ok(snapshot_id)
}

fn validate_snapshot_id(snapshot_id: &str) -> Result<(), SnapshotError> {
    if matches!(snapshot_id.len(), 40 | 64)
        && snapshot_id.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(SnapshotError::InvalidSnapshotId)
    }
}

fn project_storage_key(project_root: &Path) -> Result<String, SnapshotError> {
    let path = path_to_git(project_root)?;
    #[cfg(target_os = "windows")]
    let path = path.to_lowercase();
    Ok(format!("{:x}", Sha256::digest(path.as_bytes())))
}

fn validate_storage_key(storage_key: &str) -> Result<(), SnapshotError> {
    if storage_key.len() == 64 && storage_key.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(SnapshotError::InvalidCheckpointReference)
    }
}

fn checkpoint_ref(checkpoint_id: &str, boundary: &str) -> Result<String, SnapshotError> {
    let valid_id = !checkpoint_id.is_empty()
        && checkpoint_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'));
    if !valid_id || !matches!(boundary, "start" | "end") {
        return Err(SnapshotError::InvalidCheckpointReference);
    }
    Ok(format!(
        "refs/fennara/checkpoints/{checkpoint_id}/{boundary}"
    ))
}

fn materialized_ref(checkpoint_id: &str, boundary: &str) -> Result<String, SnapshotError> {
    checkpoint_ref(checkpoint_id, boundary).map(|reference| {
        reference.replacen("refs/fennara/checkpoints/", "refs/fennara/materialized/", 1)
    })
}

fn path_to_git(path: &Path) -> Result<String, SnapshotError> {
    let path = path
        .to_str()
        .ok_or(SnapshotError::UnsupportedRepositoryPath)?
        .to_string();
    #[cfg(target_os = "windows")]
    let path = path.replace('\\', "/");
    Ok(path)
}

fn parse_nul_paths(bytes: &[u8]) -> Result<Vec<String>, SnapshotError> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8(path.to_vec()).map_err(|_| SnapshotError::NonUtf8GitOutput))
        .collect()
}

fn parse_line_paths(bytes: &[u8]) -> Result<Vec<String>, SnapshotError> {
    bytes
        .split(|byte| *byte == b'\n')
        .filter(|path| !path.is_empty())
        .map(|path| {
            let path = path.strip_suffix(b"\r").unwrap_or(path);
            String::from_utf8(path.to_vec()).map_err(|_| SnapshotError::NonUtf8GitOutput)
        })
        .collect()
}

fn unprotect_check_path(path: String) -> String {
    path.strip_prefix("./").unwrap_or(&path).to_string()
}

async fn snapshot_git<I, S>(repository: &Repository, args: I) -> Result<GitOutput, SnapshotError>
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
        .current_dir(&repository.worktree);
    execute_git(&mut command, args).await
}

async fn snapshot_git_with_input<I, S>(
    repository: &Repository,
    args: I,
    input: &[u8],
) -> Result<GitOutput, SnapshotError>
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
        .current_dir(&repository.worktree);
    execute_git_with_input(&mut command, args, input).await
}

async fn source_git<I, S>(repository: &Repository, args: I) -> Result<GitOutput, SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(&repository.worktree)
        .env("GIT_OPTIONAL_LOCKS", "0");
    execute_git(&mut command, args).await
}

async fn source_git_literal<I, S>(
    repository: &Repository,
    args: I,
) -> Result<GitOutput, SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command
        .arg("--literal-pathspecs")
        .arg("-C")
        .arg(&repository.worktree)
        .env("GIT_OPTIONAL_LOCKS", "0");
    execute_git(&mut command, args).await
}

async fn execute_git<I, S>(command: &mut Command, args: I) -> Result<GitOutput, SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    execute_git_optional_input(command, args, None).await
}

async fn execute_git_with_input<I, S>(
    command: &mut Command,
    args: I,
    input: &[u8],
) -> Result<GitOutput, SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    execute_git_optional_input(command, args, Some(input)).await
}

async fn execute_git_optional_input<I, S>(
    command: &mut Command,
    args: I,
    input: Option<&[u8]>,
) -> Result<GitOutput, SnapshotError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    command
        .args(args)
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().map_err(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            SnapshotError::GitUnavailable
        } else {
            SnapshotError::SpawnGit(error)
        }
    })?;
    if let Some(input) = input {
        let mut stdin = child
            .stdin
            .take()
            .ok_or(SnapshotError::InvalidRepositoryOutput)?;
        stdin
            .write_all(input)
            .await
            .map_err(SnapshotError::WriteGitStdin)?;
    }
    let output = child
        .wait_with_output()
        .await
        .map_err(SnapshotError::WaitForGit)?;
    Ok(GitOutput {
        success: output.status.success(),
        code: output.status.code(),
        stdout: output.stdout,
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
    })
}

struct GitOutput {
    success: bool,
    code: Option<i32>,
    stdout: Vec<u8>,
    stderr: String,
}

fn ensure_success(operation: &'static str, output: GitOutput) -> Result<GitOutput, SnapshotError> {
    if output.success {
        Ok(output)
    } else {
        Err(command_failed(operation, &output))
    }
}

fn command_failed(operation: &'static str, output: &GitOutput) -> SnapshotError {
    SnapshotError::GitCommand {
        operation,
        message: if output.stderr.is_empty() {
            "Git command failed".to_string()
        } else {
            output.stderr.clone()
        },
    }
}

#[derive(Debug)]
pub(super) enum SnapshotError {
    GitUnavailable,
    NonGitProject,
    TimedOut,
    InvalidProjectRoot(io::Error),
    ProjectRootNotDirectory,
    InvalidRepositoryPath(io::Error),
    ProjectOutsideWorktree,
    UnsupportedRepositoryPath,
    InvalidRepositoryOutput,
    NonUtf8GitOutput,
    InvalidSnapshotId,
    InvalidCheckpointReference,
    TooManyCandidates(usize),
    CreateStorage(io::Error),
    SpawnGit(io::Error),
    WaitForGit(io::Error),
    WriteGitStdin(io::Error),
    GitCommand {
        operation: &'static str,
        message: String,
    },
}

impl SnapshotError {
    fn unavailable_reason(&self) -> CaptureUnavailableReason {
        match self {
            Self::GitUnavailable => CaptureUnavailableReason::GitUnavailable,
            Self::NonGitProject => CaptureUnavailableReason::NonGitProject,
            Self::TimedOut => CaptureUnavailableReason::TimedOut,
            _ => CaptureUnavailableReason::CaptureFailed,
        }
    }
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GitUnavailable => write!(formatter, "Git is unavailable"),
            Self::NonGitProject => write!(formatter, "project is not inside a Git worktree"),
            Self::TimedOut => write!(formatter, "checkpoint operation timed out"),
            Self::InvalidProjectRoot(error) => write!(formatter, "invalid project root: {error}"),
            Self::ProjectRootNotDirectory => write!(formatter, "project root is not a directory"),
            Self::InvalidRepositoryPath(error) => {
                write!(formatter, "invalid repository path: {error}")
            }
            Self::ProjectOutsideWorktree => {
                write!(formatter, "project is outside the Git worktree")
            }
            Self::UnsupportedRepositoryPath => write!(formatter, "repository path is unsupported"),
            Self::InvalidRepositoryOutput => {
                write!(formatter, "Git returned invalid repository data")
            }
            Self::NonUtf8GitOutput => write!(formatter, "Git returned a non-UTF-8 path"),
            Self::InvalidSnapshotId => write!(formatter, "snapshot ID is invalid"),
            Self::InvalidCheckpointReference => {
                write!(formatter, "checkpoint reference is invalid")
            }
            Self::TooManyCandidates(count) => {
                write!(formatter, "checkpoint candidate limit exceeded: {count}")
            }
            Self::CreateStorage(error) => {
                write!(formatter, "failed to create checkpoint storage: {error}")
            }
            Self::SpawnGit(error) => write!(formatter, "failed to start Git: {error}"),
            Self::WaitForGit(error) => write!(formatter, "failed while waiting for Git: {error}"),
            Self::WriteGitStdin(error) => {
                write!(formatter, "failed to send checkpoint input to Git: {error}")
            }
            Self::GitCommand { operation, message } => write!(formatter, "{operation}: {message}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::checkpoints::{CheckpointCoverage, SkippedPathReason};
    use std::{
        fs,
        process::Command as StdCommand,
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new(name: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "fennara-checkpoint-{name}-{nonce}-{}",
                TEST_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    struct TestRepository {
        root: TestDirectory,
        storage: TestDirectory,
    }

    impl TestRepository {
        fn new(name: &str) -> Self {
            let root = TestDirectory::new(name);
            let storage = TestDirectory::new(&format!("{name}-storage"));
            git(&root.path, ["init", "--quiet"]);
            git(&root.path, ["config", "user.name", "Fennara Tests"]);
            git(
                &root.path,
                ["config", "user.email", "fennara-tests@example.invalid"],
            );
            Self { root, storage }
        }

        fn store(&self) -> GitSnapshotStore {
            GitSnapshotStore::new(self.storage.path.clone())
        }

        fn commit_all(&self) {
            git(&self.root.path, ["add", "--all"]);
            git(
                &self.root.path,
                [
                    "-c",
                    "commit.gpgsign=false",
                    "commit",
                    "--quiet",
                    "-m",
                    "test",
                ],
            );
        }
    }

    fn git<I, S>(root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = StdCommand::new("git")
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
    async fn prunes_inactive_project_locks_without_dropping_active_locks() {
        let storage = TestDirectory::new("project-locks");
        let store = GitSnapshotStore::new(storage.path.clone());
        let inactive_path = PathBuf::from("inactive");
        let active_path = PathBuf::from("active");
        let next_path = PathBuf::from("next");

        let inactive = store.project_lock(&inactive_path).await;
        let active = store.project_lock(&active_path).await;
        drop(inactive);
        let next = store.project_lock(&next_path).await;

        let locks = store.project_locks.lock().await;
        assert!(!locks.contains_key(&inactive_path));
        assert!(locks.contains_key(&active_path));
        assert!(locks.contains_key(&next_path));
        drop(locks);
        drop(active);
        drop(next);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn normalizes_windows_paths_for_git() {
        assert_eq!(
            path_to_git(Path::new(r"folder\script.gd")).unwrap(),
            "folder/script.gd"
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn preserves_unix_backslashes_for_git() {
        assert_eq!(
            path_to_git(Path::new(r"folder\script.gd")).unwrap(),
            r"folder\script.gd"
        );
    }

    #[tokio::test]
    async fn captures_binary_safe_changes_and_lists_project_paths() {
        let repository = TestRepository::new("binary");
        fs::write(repository.root.path.join("keep.txt"), "before\n").unwrap();
        fs::write(repository.root.path.join("asset.bin"), [0_u8, 1, 2, 255]).unwrap();
        fs::write(repository.root.path.join("delete.txt"), "delete me\n").unwrap();
        repository.commit_all();

        let store = repository.store();
        let canonical_root = canonical_project_root(&repository.root.path).await.unwrap();
        let private = store.repository(&canonical_root).await.unwrap();
        let clean_paths = list_snapshot_paths(
            &private,
            [
                "diff-files",
                "--name-only",
                "-z",
                "--",
                private.scope.as_str(),
            ],
        )
        .await
        .unwrap();
        assert!(clean_paths.is_empty());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&private.private_git_dir)
                .unwrap()
                .permissions()
                .mode();
            assert_eq!(mode & 0o077, 0);
        }
        let source_index = repository.root.path.join(".git").join("index");
        let source_index_before = fs::read(&source_index).unwrap();
        let before = store.capture(&repository.root.path).await;
        assert_eq!(before.coverage, CheckpointCoverage::Full);

        fs::write(repository.root.path.join("keep.txt"), "after\n").unwrap();
        fs::write(repository.root.path.join("asset.bin"), [255_u8, 0, 17, 9]).unwrap();
        fs::remove_file(repository.root.path.join("delete.txt")).unwrap();
        fs::write(repository.root.path.join("created.txt"), "new\n").unwrap();

        let after = store.capture(&repository.root.path).await;
        assert_eq!(after.coverage, CheckpointCoverage::Full);
        assert_eq!(fs::read(source_index).unwrap(), source_index_before);
        let changed = store
            .changed_paths(
                &repository.root.path,
                before.snapshot_id.as_deref().unwrap(),
                after.snapshot_id.as_deref().unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            changed,
            vec!["asset.bin", "created.txt", "delete.txt", "keep.txt"]
        );

        let identity = store.identify(&repository.root.path).await.unwrap();
        let snapshot_id = after.snapshot_id.as_deref().unwrap();
        store
            .pin_snapshot(&identity, "checkpoint_1", "end", snapshot_id)
            .await
            .unwrap();
        let reference = checkpoint_ref("checkpoint_1", "end").unwrap();
        let output = snapshot_git(
            &private,
            ["show-ref", "--verify", "--hash", reference.as_str()],
        )
        .await
        .unwrap();
        assert_eq!(
            String::from_utf8(
                ensure_success("read checkpoint ref", output)
                    .unwrap()
                    .stdout
            )
            .unwrap()
            .trim(),
            snapshot_id
        );
        store
            .release_checkpoint(&identity.storage_key, "checkpoint_1")
            .await
            .unwrap();
        let output = snapshot_git(
            &private,
            ["show-ref", "--verify", "--hash", reference.as_str()],
        )
        .await
        .unwrap();
        assert!(!output.success);

        let object = format!("{}:asset.bin", after.snapshot_id.unwrap());
        let output = snapshot_git(&private, ["show", object.as_str()])
            .await
            .unwrap();
        assert_eq!(
            ensure_success("read binary", output).unwrap().stdout,
            [255, 0, 17, 9]
        );
        store.compact_storage(&identity.storage_key).await.unwrap();
    }

    #[tokio::test]
    async fn pinned_snapshot_survives_source_object_pruning() {
        let repository = TestRepository::new("durable-pin");
        git(
            &repository.root.path,
            ["symbolic-ref", "HEAD", "refs/heads/fennara-test-main"],
        );
        fs::write(
            repository.root.path.join("persistent.txt"),
            "private checkpoint content\n",
        )
        .unwrap();
        repository.commit_all();

        let store = repository.store();
        let capture = store.capture(&repository.root.path).await;
        let snapshot_id = capture.snapshot_id.as_deref().unwrap();
        let identity = store.identify(&repository.root.path).await.unwrap();
        store
            .pin_snapshot(&identity, "checkpoint_durable", "start", snapshot_id)
            .await
            .unwrap();
        store
            .pin_snapshot(&identity, "checkpoint_durable", "end", snapshot_id)
            .await
            .unwrap();

        git(&repository.root.path, ["read-tree", "--empty"]);
        git(
            &repository.root.path,
            ["update-ref", "-d", "refs/heads/fennara-test-main"],
        );
        git(
            &repository.root.path,
            ["reflog", "expire", "--expire=now", "--all"],
        );
        git(&repository.root.path, ["gc", "--prune=now", "--quiet"]);

        let object = format!("{snapshot_id}:persistent.txt");
        let source = StdCommand::new("git")
            .arg("-C")
            .arg(&repository.root.path)
            .args(["cat-file", "-e", object.as_str()])
            .output()
            .unwrap();
        assert!(!source.status.success());

        let private = store.repository(&identity.root).await.unwrap();
        let output = snapshot_git(&private, ["show", object.as_str()])
            .await
            .unwrap();
        assert_eq!(
            ensure_success("read durable checkpoint", output)
                .unwrap()
                .stdout,
            b"private checkpoint content\n"
        );
    }

    #[tokio::test]
    async fn excludes_generated_cache_and_reports_large_untracked_files() {
        let repository = TestRepository::new("limits");
        fs::write(
            repository.root.path.join("project.godot"),
            "[application]\n",
        )
        .unwrap();
        fs::write(
            repository.root.path.join(".gitattributes"),
            "*.lfs filter=lfs diff=lfs merge=lfs -text\n*.filtered filter=custom\n",
        )
        .unwrap();
        repository.commit_all();
        fs::create_dir_all(repository.root.path.join(".godot")).unwrap();
        fs::write(
            repository.root.path.join(".godot").join("cache.bin"),
            [1_u8; 32],
        )
        .unwrap();
        fs::write(
            repository.root.path.join("large.bin"),
            vec![7_u8; (MAX_UNTRACKED_FILE_BYTES + 1) as usize],
        )
        .unwrap();
        fs::write(repository.root.path.join("asset.lfs"), [8_u8; 32]).unwrap();
        fs::write(repository.root.path.join("asset.filtered"), [10_u8; 32]).unwrap();
        fs::write(repository.root.path.join("script.gd.uid"), "uid://test\n").unwrap();
        fs::write(
            repository.root.path.join("generated.translation"),
            [9_u8; 32],
        )
        .unwrap();

        let store = repository.store();
        let capture = store.capture(&repository.root.path).await;
        assert_eq!(capture.coverage, CheckpointCoverage::Partial);
        assert_eq!(capture.skipped_paths.len(), 3);
        assert_eq!(capture.skipped_paths[0].path, "asset.filtered");
        assert_eq!(
            capture.skipped_paths[0].reason,
            SkippedPathReason::UnverifiedContentFilter
        );
        assert_eq!(capture.skipped_paths[1].path, "asset.lfs");
        assert_eq!(
            capture.skipped_paths[1].reason,
            SkippedPathReason::UnverifiedLfsObject
        );
        assert_eq!(capture.skipped_paths[2].path, "large.bin");
        assert_eq!(
            capture.skipped_paths[2].reason,
            SkippedPathReason::LargeUntrackedFile
        );

        let canonical_root = canonical_project_root(&repository.root.path).await.unwrap();
        let private = store.repository(&canonical_root).await.unwrap();
        for path in [
            ".godot/cache.bin",
            "asset.filtered",
            "asset.lfs",
            "generated.translation",
            "large.bin",
        ] {
            let object = format!("{}:{path}", capture.snapshot_id.as_deref().unwrap());
            let output = snapshot_git(&private, ["cat-file", "-e", object.as_str()])
                .await
                .unwrap();
            assert!(
                !output.success,
                "{path} should not be in the checkpoint tree"
            );
        }
        let uid = format!("{}:script.gd.uid", capture.snapshot_id.as_deref().unwrap());
        let output = snapshot_git(&private, ["cat-file", "-e", uid.as_str()])
            .await
            .unwrap();
        assert!(
            output.success,
            ".uid sidecars must remain checkpoint-eligible"
        );
    }

    #[test]
    fn reports_untracked_files_over_the_aggregate_byte_budget() {
        let skipped = SkippedPath {
            path: "over-budget.bin".to_string(),
            reason: untracked_skip_reason(MAX_UNTRACKED_FILE_BYTES, MAX_UNTRACKED_CAPTURE_BYTES)
                .unwrap(),
        };
        let capture = CaptureResult::available("snapshot".to_string(), vec![skipped.clone()]);
        assert_eq!(capture.coverage, CheckpointCoverage::Partial);
        assert_eq!(capture.skipped_paths, vec![skipped]);
        assert_eq!(
            capture.skipped_paths[0].reason,
            SkippedPathReason::UntrackedByteBudgetExceeded
        );
    }

    #[tokio::test]
    async fn preserves_tracked_ignored_files_and_reports_newly_ignored_paths() {
        let repository = TestRepository::new("ignored");
        fs::write(repository.root.path.join(".gitignore"), "*.ignored\n").unwrap();
        fs::write(
            repository.root.path.join("tracked.ignored"),
            "tracked before\n",
        )
        .unwrap();
        git(&repository.root.path, ["add", "--force", "tracked.ignored"]);
        repository.commit_all();

        let store = repository.store();
        let baseline = store.capture(&repository.root.path).await;
        assert_eq!(baseline.coverage, CheckpointCoverage::Full);
        fs::write(repository.root.path.join("later.txt"), "before ignore\n").unwrap();
        let with_untracked = store.capture(&repository.root.path).await;
        assert_eq!(with_untracked.coverage, CheckpointCoverage::Full);

        fs::write(
            repository.root.path.join(".gitignore"),
            "*.ignored\nlater.txt\n",
        )
        .unwrap();
        fs::write(
            repository.root.path.join("tracked.ignored"),
            "tracked after\n",
        )
        .unwrap();
        let capture = store.capture(&repository.root.path).await;
        assert_eq!(capture.coverage, CheckpointCoverage::Partial);
        assert_eq!(
            capture.skipped_paths,
            vec![SkippedPath {
                path: "later.txt".to_string(),
                reason: SkippedPathReason::IgnoredPath,
            }]
        );

        let canonical_root = canonical_project_root(&repository.root.path).await.unwrap();
        let private = store.repository(&canonical_root).await.unwrap();
        let object = format!(
            "{}:tracked.ignored",
            capture.snapshot_id.as_deref().unwrap()
        );
        let output = snapshot_git(&private, ["show", object.as_str()])
            .await
            .unwrap();
        assert_eq!(
            ensure_success("read tracked ignored file", output)
                .unwrap()
                .stdout,
            b"tracked after\n"
        );
    }

    #[tokio::test]
    async fn reports_dirty_nested_git_repositories_as_partial() {
        let repository = TestRepository::new("nested-git");
        let nested = repository.root.path.join("addons").join("vendor");
        fs::create_dir_all(&nested).unwrap();
        git(&nested, ["init", "--quiet"]);
        git(&nested, ["config", "user.name", "Fennara Tests"]);
        git(
            &nested,
            ["config", "user.email", "fennara-tests@example.invalid"],
        );
        fs::write(nested.join("addon.gd"), "extends Node\n").unwrap();
        git(&nested, ["add", "--all"]);
        git(
            &nested,
            [
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "-m",
                "nested",
            ],
        );
        repository.commit_all();

        let store = repository.store();
        let baseline = store.capture(&repository.root.path).await;
        assert_eq!(baseline.coverage, CheckpointCoverage::Full);
        let canonical_root = canonical_project_root(&repository.root.path).await.unwrap();
        let private = store.repository(&canonical_root).await.unwrap();
        remove_from_index(&private, &["addons/vendor"])
            .await
            .unwrap();
        fs::remove_dir_all(nested.join(".git")).unwrap();

        let capture = store.capture(&repository.root.path).await;
        assert_eq!(capture.coverage, CheckpointCoverage::Partial);
        assert_eq!(
            capture.skipped_paths,
            vec![SkippedPath {
                path: "addons/vendor".to_string(),
                reason: SkippedPathReason::NestedGitRepository,
            }]
        );
    }

    #[tokio::test]
    async fn returns_conversation_only_for_non_git_projects() {
        let project = TestDirectory::new("non-git");
        let storage = TestDirectory::new("non-git-storage");
        fs::write(project.path.join("project.godot"), "[application]\n").unwrap();

        let capture = GitSnapshotStore::new(storage.path.clone())
            .capture(&project.path)
            .await;
        assert_eq!(capture.coverage, CheckpointCoverage::ConversationOnly);
        assert_eq!(
            capture.unavailable_reason,
            Some(CaptureUnavailableReason::NonGitProject)
        );
        assert!(capture.snapshot_id.is_none());
    }
}
