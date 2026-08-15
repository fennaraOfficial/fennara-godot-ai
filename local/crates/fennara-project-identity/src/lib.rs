//! Canonical filesystem identity for a Godot project root.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

/// A validated, canonical Godot project directory and its live filesystem identity.
///
/// The canonical path is the durable locator. Native file identifiers are private,
/// best-effort comparison aids and are never suitable for persistence.
#[derive(Clone, Debug)]
pub struct ProjectRoot {
    canonical_path: PathBuf,
    protocol_path: String,
    native_identity: Option<NativeIdentity>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum NativeIdentity {
    #[cfg(unix)]
    Unix { device: u64, inode: u64 },
    #[cfg(windows)]
    Windows {
        volume_serial: u64,
        file_id: [u8; 16],
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectRootError {
    #[error("the project path is empty")]
    EmptyPath,
    #[error("the project path must be absolute")]
    RelativePath,
    #[error("the startup directory must be absolute")]
    RelativeStartupDirectory,
    #[error("the project path form is unsupported on this platform")]
    UnsupportedPathKind,
    #[error("the project path cannot be represented by Fennara's Unicode protocol")]
    UnsupportedUnicode,
    #[error("the project root could not be resolved: {0}")]
    ResolveFailed(#[source] std::io::Error),
    #[error("the project root is not a directory")]
    NotDirectory,
    #[error("the project root does not contain a regular project.godot file")]
    MissingProjectFile,
}

impl ProjectRoot {
    /// Resolves an absolute path or an ordinary relative path against `startup_cwd`.
    pub fn resolve_from(input: &OsStr, startup_cwd: &Path) -> Result<Self, ProjectRootError> {
        if input.is_empty() {
            return Err(ProjectRootError::EmptyPath);
        }

        let path = Path::new(input);
        validate_platform_path(path)?;
        if path.is_absolute() {
            return resolve_candidate(path);
        }
        if !is_ordinary_relative(path) {
            return Err(ProjectRootError::UnsupportedPathKind);
        }

        validate_platform_path(startup_cwd)?;
        if !startup_cwd.is_absolute() {
            return Err(ProjectRootError::RelativeStartupDirectory);
        }

        resolve_candidate(&startup_cwd.join(path))
    }

    /// Resolves a path that must already be absolute.
    pub fn resolve_absolute(input: &OsStr) -> Result<Self, ProjectRootError> {
        if input.is_empty() {
            return Err(ProjectRootError::EmptyPath);
        }

        let path = Path::new(input);
        validate_platform_path(path)?;
        if !path.is_absolute() {
            if !is_ordinary_relative(path) {
                return Err(ProjectRootError::UnsupportedPathKind);
            }
            return Err(ProjectRootError::RelativePath);
        }

        resolve_candidate(path)
    }

    /// Finds and resolves the nearest Godot project containing `startup_cwd`.
    pub fn discover_from(startup_cwd: &Path) -> Result<Option<Self>, ProjectRootError> {
        validate_platform_path(startup_cwd)?;
        if !startup_cwd.is_absolute() {
            return Err(ProjectRootError::RelativeStartupDirectory);
        }

        let canonical_start =
            fs::canonicalize(startup_cwd).map_err(ProjectRootError::ResolveFailed)?;
        let metadata = fs::metadata(&canonical_start).map_err(ProjectRootError::ResolveFailed)?;
        if !metadata.is_dir() {
            return Err(ProjectRootError::NotDirectory);
        }

        for ancestor in canonical_start.ancestors() {
            match fs::metadata(ancestor.join("project.godot")) {
                Ok(metadata) if metadata.is_file() => {
                    return Self::resolve_absolute(ancestor.as_os_str()).map(Some);
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(ProjectRootError::ResolveFailed(error)),
            }
        }

        Ok(None)
    }

    /// Returns the native canonical filesystem path without rewriting it.
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    /// Returns the losslessly encoded canonical path used at Fennara's protocol seam.
    pub fn as_protocol_str(&self) -> &str {
        &self.protocol_path
    }

    /// Reports whether two resolutions currently identify the same live project directory.
    pub fn same_project(&self, other: &Self) -> bool {
        match (&self.native_identity, &other.native_identity) {
            (Some(left), Some(right)) => left == right,
            _ => self.canonical_path == other.canonical_path,
        }
    }
}

fn resolve_candidate(path: &Path) -> Result<ProjectRoot, ProjectRootError> {
    let canonical_path = fs::canonicalize(path).map_err(ProjectRootError::ResolveFailed)?;
    let metadata = fs::metadata(&canonical_path).map_err(ProjectRootError::ResolveFailed)?;
    if !metadata.is_dir() {
        return Err(ProjectRootError::NotDirectory);
    }

    let marker = canonical_path.join("project.godot");
    match fs::metadata(marker) {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => return Err(ProjectRootError::MissingProjectFile),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(ProjectRootError::MissingProjectFile);
        }
        Err(error) => return Err(ProjectRootError::ResolveFailed(error)),
    }

    let protocol_path = canonical_path
        .to_str()
        .ok_or(ProjectRootError::UnsupportedUnicode)?
        .to_owned();
    let native_identity = native_identity(&canonical_path);

    Ok(ProjectRoot {
        canonical_path,
        protocol_path,
        native_identity,
    })
}

#[cfg(not(windows))]
fn validate_platform_path(_path: &Path) -> Result<(), ProjectRootError> {
    Ok(())
}

#[cfg(not(windows))]
fn is_ordinary_relative(path: &Path) -> bool {
    !path.is_absolute()
}

#[cfg(windows)]
fn validate_platform_path(path: &Path) -> Result<(), ProjectRootError> {
    use std::path::{Component, Prefix};

    let Some(Component::Prefix(prefix)) = path.components().next() else {
        return Ok(());
    };

    match prefix.kind() {
        Prefix::Disk(_)
        | Prefix::UNC(_, _)
        | Prefix::VerbatimDisk(_)
        | Prefix::VerbatimUNC(_, _) => Ok(()),
        Prefix::DeviceNS(_) | Prefix::Verbatim(_) => Err(ProjectRootError::UnsupportedPathKind),
    }
}

#[cfg(windows)]
fn is_ordinary_relative(path: &Path) -> bool {
    use std::path::Component;

    !path.has_root() && !matches!(path.components().next(), Some(Component::Prefix(_)))
}

#[cfg(unix)]
fn native_identity(path: &Path) -> Option<NativeIdentity> {
    use std::os::unix::fs::MetadataExt;

    fs::metadata(path)
        .ok()
        .map(|metadata| NativeIdentity::Unix {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
}

#[cfg(windows)]
fn native_identity(path: &Path) -> Option<NativeIdentity> {
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_FLAG_BACKUP_SEMANTICS, FILE_ID_INFO, FILE_READ_ATTRIBUTES,
        FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, FileIdInfo,
        GetFileInformationByHandleEx, OPEN_EXISTING,
    };

    let wide_path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    let handle = unsafe {
        CreateFileW(
            wide_path.as_ptr(),
            FILE_READ_ATTRIBUTES,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS,
            ptr::null_mut(),
        )
    };
    if handle == INVALID_HANDLE_VALUE {
        return None;
    }

    let mut info = FILE_ID_INFO::default();
    let succeeded = unsafe {
        GetFileInformationByHandleEx(
            handle,
            FileIdInfo,
            ptr::from_mut(&mut info).cast(),
            std::mem::size_of::<FILE_ID_INFO>() as u32,
        )
    } != 0;
    unsafe {
        CloseHandle(handle);
    }

    succeeded.then_some(NativeIdentity::Windows {
        volume_serial: info.VolumeSerialNumber,
        file_id: info.FileId.Identifier,
    })
}

#[cfg(not(any(unix, windows)))]
fn native_identity(_path: &Path) -> Option<NativeIdentity> {
    None
}

#[cfg(test)]
mod tests {
    use super::{ProjectRoot, ProjectRootError};
    use std::ffi::OsStr;
    #[cfg(unix)]
    use std::ffi::OsString;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    struct Fixture {
        path: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            for _ in 0..100 {
                let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                let path = std::env::temp_dir().join(format!(
                    "fennara-project-identity-{}-{timestamp}-{sequence}",
                    std::process::id()
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self { path },
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => panic!("create fixture directory: {error}"),
                }
            }

            panic!("could not allocate a unique fixture directory")
        }

        fn godot_project(&self, relative: impl AsRef<Path>) -> PathBuf {
            let root = self.path.join(relative);
            fs::create_dir_all(&root).expect("create project root");
            fs::write(root.join("project.godot"), b"[application]\n")
                .expect("write project marker");
            root
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.path).expect("remove fixture directory");
        }
    }

    #[test]
    fn resolves_an_absolute_godot_project() {
        let fixture = Fixture::new();
        let root = fixture.godot_project("game");

        let resolved = ProjectRoot::resolve_absolute(root.as_os_str()).expect("resolve project");

        assert_eq!(resolved.canonical_path(), fs::canonicalize(root).unwrap());
        assert_eq!(
            Some(resolved.as_protocol_str()),
            resolved.canonical_path().to_str()
        );
    }

    #[test]
    fn resolves_an_ordinary_relative_path_against_the_captured_startup_directory() {
        let fixture = Fixture::new();
        let root = fixture.godot_project("workspace/game");
        let startup = fixture.path.join("workspace/nested");
        fs::create_dir_all(&startup).unwrap();

        let resolved = ProjectRoot::resolve_from(OsStr::new("../game/./"), &startup)
            .expect("resolve relative project");

        assert_eq!(resolved.canonical_path(), fs::canonicalize(root).unwrap());
    }

    #[test]
    fn discovers_the_nearest_project_ancestor() {
        let fixture = Fixture::new();
        let outer = fixture.godot_project("outer");
        let inner = fixture.godot_project("outer/packages/inner");
        let startup = inner.join("src/deep");
        fs::create_dir_all(&startup).unwrap();

        let resolved = ProjectRoot::discover_from(&startup)
            .expect("discover project")
            .expect("project is present");

        assert_eq!(resolved.canonical_path(), fs::canonicalize(inner).unwrap());
        assert_ne!(resolved.canonical_path(), fs::canonicalize(outer).unwrap());
    }

    #[test]
    fn discovery_returns_none_when_no_ancestor_is_a_godot_project() {
        let fixture = Fixture::new();
        let startup = fixture.path.join("workspace/nested");
        fs::create_dir_all(&startup).unwrap();

        assert!(ProjectRoot::discover_from(&startup).unwrap().is_none());
    }

    #[test]
    fn explicit_binding_errors_are_typed() {
        let fixture = Fixture::new();
        let ordinary_dir = fixture.path.join("ordinary");
        fs::create_dir(&ordinary_dir).unwrap();
        let regular_file = fixture.path.join("file");
        fs::write(&regular_file, b"not a directory").unwrap();

        assert!(matches!(
            ProjectRoot::resolve_from(OsStr::new(""), &fixture.path),
            Err(ProjectRootError::EmptyPath)
        ));
        assert!(matches!(
            ProjectRoot::resolve_absolute(OsStr::new("relative")),
            Err(ProjectRootError::RelativePath)
        ));
        assert!(matches!(
            ProjectRoot::resolve_absolute(regular_file.as_os_str()),
            Err(ProjectRootError::NotDirectory)
        ));
        assert!(matches!(
            ProjectRoot::resolve_absolute(ordinary_dir.as_os_str()),
            Err(ProjectRootError::MissingProjectFile)
        ));
        assert!(matches!(
            ProjectRoot::resolve_absolute(fixture.path.join("missing").as_os_str()),
            Err(ProjectRootError::ResolveFailed(_))
        ));
    }

    #[test]
    fn whitespace_is_path_data_not_an_empty_value() {
        let fixture = Fixture::new();
        let root = fixture.godot_project("   ");

        let resolved = ProjectRoot::resolve_from(OsStr::new("   "), &fixture.path).unwrap();

        assert_eq!(resolved.canonical_path(), fs::canonicalize(root).unwrap());
    }

    #[cfg(unix)]
    #[test]
    fn a_symlink_and_its_target_are_the_same_live_project() {
        use std::os::unix::fs::symlink;

        let fixture = Fixture::new();
        let root = fixture.godot_project("game");
        let alias = fixture.path.join("alias");
        symlink(&root, &alias).unwrap();

        let target = ProjectRoot::resolve_absolute(root.as_os_str()).unwrap();
        let through_alias = ProjectRoot::resolve_absolute(alias.as_os_str()).unwrap();

        assert!(target.same_project(&through_alias));
        assert_eq!(target.canonical_path(), through_alias.canonical_path());
    }

    #[cfg(unix)]
    #[test]
    fn case_distinct_directories_are_different_projects_when_the_filesystem_preserves_case() {
        let fixture = Fixture::new();
        let upper = fixture.godot_project("Game");
        let lower = fixture.godot_project("game");

        if fs::canonicalize(&upper).unwrap() == fs::canonicalize(&lower).unwrap() {
            eprintln!("skipping case assertion on a case-insensitive fixture");
            return;
        }

        let upper = ProjectRoot::resolve_absolute(upper.as_os_str()).unwrap();
        let lower = ProjectRoot::resolve_absolute(lower.as_os_str()).unwrap();

        assert!(!upper.same_project(&lower));
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_roots_are_rejected_at_the_protocol_seam() {
        use std::os::unix::ffi::OsStringExt;

        let fixture = Fixture::new();
        let root = fixture
            .path
            .join(OsString::from_vec(b"non-unicode-\xff".to_vec()));
        fs::create_dir(&root).unwrap();
        fs::write(root.join("project.godot"), b"[application]\n").unwrap();

        assert!(matches!(
            ProjectRoot::resolve_absolute(root.as_os_str()),
            Err(ProjectRootError::UnsupportedUnicode)
        ));
    }

    #[cfg(windows)]
    #[test]
    fn dos_and_canonical_verbatim_paths_are_the_same_project() {
        let fixture = Fixture::new();
        let root = fixture.godot_project("game");
        let verbatim_root = fs::canonicalize(&root).unwrap();

        let through_dos_path = ProjectRoot::resolve_absolute(root.as_os_str()).unwrap();
        let through_verbatim_path =
            ProjectRoot::resolve_absolute(verbatim_root.as_os_str()).unwrap();

        assert!(through_dos_path.same_project(&through_verbatim_path));
    }

    #[cfg(windows)]
    #[test]
    fn a_windows_directory_symlink_and_its_target_are_the_same_project() {
        use std::os::windows::fs::symlink_dir;

        let fixture = Fixture::new();
        let root = fixture.godot_project("game");
        let alias = fixture.path.join("alias");
        if let Err(error) = symlink_dir(&root, &alias) {
            if error.kind() == std::io::ErrorKind::PermissionDenied {
                eprintln!("skipping symlink assertion without Windows symlink permission");
                return;
            }
            panic!("create directory symlink: {error}");
        }

        let target = ProjectRoot::resolve_absolute(root.as_os_str()).unwrap();
        let through_alias = ProjectRoot::resolve_absolute(alias.as_os_str()).unwrap();

        assert!(target.same_project(&through_alias));
    }

    #[cfg(windows)]
    #[test]
    fn case_variants_match_on_a_case_insensitive_windows_fixture() {
        let fixture = Fixture::new();
        let root = fixture.godot_project("game");
        let case_variant = fixture.path.join("GAME");
        if !case_variant.is_dir() {
            eprintln!("skipping case assertion on a case-sensitive Windows fixture");
            return;
        }

        let original = ProjectRoot::resolve_absolute(root.as_os_str()).unwrap();
        let variant = ProjectRoot::resolve_absolute(case_variant.as_os_str()).unwrap();

        assert!(original.same_project(&variant));
    }
}
