use super::*;
use crate::app_layout::{arch_name, platform_name};
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn stages_valid_addon_without_touching_active_addon() {
    let root = TestRoot::new("success");
    let project = root.join("project");
    let source = root.join("package");
    write_project(&project);
    write_addon(&project.join("addons/fennara"), "1.0.0");
    write_addon(&source, "1.1.0");
    let active_before = fs::read(project.join("addons/fennara/VERSION")).unwrap();
    let package = InstalledPackage {
        version: "1.1.0".to_string(),
        addon_dir: source,
    };

    let staged = prepare(&project, "1.0.0", &package, "update-123-test", None).unwrap();

    assert_eq!(staged.version, "1.1.0");
    assert!(staged.root.join("addon/fennara.gdextension").is_file());
    assert_eq!(
        fs::read(project.join("addons/fennara/VERSION")).unwrap(),
        active_before
    );
    let receipt = read_receipt(&staged.receipt_path).unwrap();
    assert_eq!(receipt.state, "ready_to_close");
    assert_eq!(receipt.from_version, "1.0.0");
    assert_eq!(receipt.to_version, "1.1.0");
    verify_staged_addon(&staged.root, &receipt).unwrap();
}

#[test]
fn detects_staged_addon_mutation() {
    let root = TestRoot::new("mutation");
    let project = root.join("project");
    let source = root.join("package");
    write_project(&project);
    write_addon(&project.join("addons/fennara"), "1.0.0");
    write_addon(&source, "1.1.0");
    let package = InstalledPackage {
        version: "1.1.0".to_string(),
        addon_dir: source,
    };
    let staged = prepare(&project, "1.0.0", &package, "update-789-test", None).unwrap();
    let receipt = read_receipt(&staged.receipt_path).unwrap();

    fs::write(staged.root.join("addon/ai/guidelines.md"), "changed\n").unwrap();

    assert!(verify_staged_addon(&staged.root, &receipt).is_err());
}

#[test]
fn rejects_unsafe_receipt_paths_and_versions() {
    let root = TestRoot::new("receipt");
    fs::create_dir_all(&*root).unwrap();
    let receipt_path = root.join("receipt.json");
    let mut receipt = valid_receipt();
    receipt.backup_addon = "../fennara".to_string();
    write_receipt(&receipt_path, &receipt).unwrap();
    assert!(read_receipt(&receipt_path).is_err());

    receipt.backup_addon = BACKUP_ADDON_NAME.to_string();
    receipt.to_version = "../../outside".to_string();
    write_receipt(&receipt_path, &receipt).unwrap();
    assert!(read_receipt(&receipt_path).is_err());
}

#[test]
fn failed_validation_leaves_no_staging_directory() {
    let root = TestRoot::new("invalid");
    let project = root.join("project");
    let source = root.join("package");
    write_project(&project);
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("VERSION"), "1.1.0\n").unwrap();
    let package = InstalledPackage {
        version: "1.1.0".to_string(),
        addon_dir: source,
    };

    assert!(prepare(&project, "1.0.0", &package, "update-456-test", None).is_err());
    assert!(
        !project
            .join(".godot/fennara-update/update-456-test")
            .exists()
    );
    assert!(
        !project
            .join(".godot/fennara-update/update-456-test.preparing")
            .exists()
    );
}

fn valid_receipt() -> UpdateReceipt {
    UpdateReceipt {
        schema_version: RECEIPT_SCHEMA_VERSION,
        operation_id: "update-123-test".to_string(),
        state: "ready_to_close".to_string(),
        from_version: "1.0.0".to_string(),
        to_version: "1.1.0".to_string(),
        platform: platform_name().to_string(),
        architecture: arch_name().to_string(),
        addon: STAGED_ADDON_NAME.to_string(),
        backup_addon: BACKUP_ADDON_NAME.to_string(),
        addon_sha256: "a".repeat(64),
        godot_pid: None,
        godot_started_at: None,
        godot_executable: None,
        had_current_manifest: None,
        launchers_snapshotted: false,
        addon_replaced: false,
        updater_pid: None,
        updater_started_at: None,
    }
}

fn write_project(project: &Path) {
    fs::create_dir_all(project).unwrap();
    fs::write(project.join("project.godot"), "[application]\n").unwrap();
}

fn write_addon(addon: &Path, version: &str) {
    fs::create_dir_all(addon.join("bin")).unwrap();
    fs::create_dir_all(addon.join("ai")).unwrap();
    fs::write(addon.join("VERSION"), format!("{version}\n")).unwrap();
    fs::write(addon.join("ai/guidelines.md"), "guidance\n").unwrap();
    fs::write(addon.join("bin/fennara-test-library"), "library").unwrap();
    fs::write(
        addon.join("fennara.gdextension"),
        format!(
            "[libraries]\n{}.editor.{} = \"res://addons/fennara/bin/fennara-test-library\"\n",
            platform_name(),
            arch_name()
        ),
    )
    .unwrap();
}

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(std::env::temp_dir().join(format!(
            "fennara-update-stage-{name}-{}-{nonce}",
            std::process::id()
        )))
    }
}

impl Deref for TestRoot {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
