use super::*;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default)]
struct TestDependencies {
    check_calls: usize,
    fail_second_check: bool,
    manifest_path: Option<PathBuf>,
    ensure_error: Option<String>,
}

impl InstallDependencies for TestDependencies {
    type Activation = Option<Vec<u8>>;

    fn check_daemon(&mut self, version: &str) -> Result<(), String> {
        assert_eq!(version, "1.2.3");
        self.check_calls += 1;
        if self.fail_second_check && self.check_calls == 2 {
            Err("running daemon is version 1.2.2".to_string())
        } else {
            Ok(())
        }
    }

    fn install_package(&mut self, version: &str) -> Result<String, String> {
        Ok(version.to_string())
    }

    fn activate_package(&mut self, _version: &str) -> Result<Self::Activation, String> {
        let Some(path) = &self.manifest_path else {
            return Ok(None);
        };
        let previous = fs::read(path).ok();
        fs::write(path, "{\"version\":\"1.2.3\"}\n").unwrap();
        Ok(previous)
    }

    fn ensure_daemon(&mut self, _version: &str) -> Result<ReadyState, String> {
        match self.ensure_error.take() {
            Some(error) => Err(error),
            None => Ok(ReadyState::AlreadyRunning),
        }
    }

    fn restore_activation(&mut self, previous: Self::Activation) -> Result<(), String> {
        if let (Some(path), Some(previous)) = (&self.manifest_path, previous) {
            fs::write(path, previous).unwrap();
        }
        Ok(())
    }

    fn check_webview(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn explicit_version_must_match_existing_addon() {
    let addon = ExistingAddon {
        version: "1.2.3".to_string(),
    };
    let error = install_with(
        Path::new("project"),
        addon,
        Some("1.2.4"),
        &mut TestDependencies::default(),
    )
    .unwrap_err();
    assert!(error.contains("existing project addon is version 1.2.3"));
    assert!(error.contains("--version requested 1.2.4"));
}

#[test]
fn repeated_adoption_leaves_existing_addon_unchanged() {
    let root = test_root("idempotent");
    let project = root.join("project");
    let addon_dir = project.join("addons").join("fennara");
    fs::create_dir_all(addon_dir.join("ai")).unwrap();
    fs::write(addon_dir.join("VERSION"), "1.2.3\n").unwrap();
    fs::write(addon_dir.join("fennara.gdextension"), "store manifest\n").unwrap();
    fs::write(addon_dir.join("ai/guidelines.md"), "store guidance\n").unwrap();
    let initial_addon = snapshot(&addon_dir);

    let mut first_guidance = None;
    let mut dependencies = TestDependencies::default();
    for _ in 0..2 {
        install_with(
            &project,
            ExistingAddon {
                version: "1.2.3".to_string(),
            },
            None,
            &mut dependencies,
        )
        .unwrap();
        let guidance = fs::read(project.join("AGENTS.md")).unwrap();
        if let Some(first) = &first_guidance {
            assert_eq!(&guidance, first);
        } else {
            first_guidance = Some(guidance);
        }
    }

    assert_eq!(snapshot(&addon_dir), initial_addon);
    assert!(project.join("AGENTS.md").is_file());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn daemon_mismatch_after_staging_keeps_previous_manifest() {
    let root = test_root("daemon-mismatch");
    let project = root.join("project");
    let manifest = root.join("current.json");
    fs::create_dir_all(&project).unwrap();
    fs::write(&manifest, "{\"version\":\"1.2.2\"}\n").unwrap();
    let previous = fs::read(&manifest).unwrap();
    let error = install_with(
        &project,
        ExistingAddon {
            version: "1.2.3".to_string(),
        },
        None,
        &mut TestDependencies {
            fail_second_check: true,
            manifest_path: Some(manifest.clone()),
            ..Default::default()
        },
    )
    .unwrap_err();

    assert!(error.contains("running daemon is version 1.2.2"));
    assert_eq!(fs::read(&manifest).unwrap(), previous);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn daemon_start_failure_restores_previous_manifest() {
    let root = test_root("daemon-start-failure");
    let project = root.join("project");
    let manifest = root.join("current.json");
    fs::create_dir_all(&project).unwrap();
    fs::write(&manifest, "{\"version\":\"1.2.2\"}\n").unwrap();
    let previous = fs::read(&manifest).unwrap();

    let error = install_with(
        &project,
        ExistingAddon {
            version: "1.2.3".to_string(),
        },
        None,
        &mut TestDependencies {
            manifest_path: Some(manifest.clone()),
            ensure_error: Some("daemon failed to start".to_string()),
            ..Default::default()
        },
    )
    .unwrap_err();

    assert!(error.contains("daemon failed to start"));
    assert_eq!(fs::read(&manifest).unwrap(), previous);
    fs::remove_dir_all(root).unwrap();
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    collect_files(root, root, &mut files);
    files
}

fn collect_files(root: &Path, current: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
    for entry in fs::read_dir(current).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_files(root, &path, files);
        } else {
            files.insert(
                path.strip_prefix(root).unwrap().to_path_buf(),
                fs::read(path).unwrap(),
            );
        }
    }
}

fn test_root(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "fennara-addon-adoption-{name}-{}-{nonce}",
        std::process::id()
    ))
}
