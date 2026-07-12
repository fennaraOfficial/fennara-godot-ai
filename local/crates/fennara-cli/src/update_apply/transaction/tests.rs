use super::*;
use crate::app_layout::{arch_name, platform_name};
use crate::update_stage::{BACKUP_ADDON_NAME, STAGED_ADDON_NAME};
use std::ops::Deref;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn replace_and_restore_addon_round_trip() {
    let root = TestRoot::new("round-trip");
    let project = root.join("project");
    let transaction = root.join("transaction");
    write_addon(&project.join("addons/fennara"), "1.0.0");
    write_addon(&transaction.join(STAGED_ADDON_NAME), "1.1.0");

    replace_addon(&project, &transaction).unwrap();
    assert!(active_has_version(&project.join("addons/fennara"), "1.1.0"));

    restore_addon(
        &project.join("addons/fennara"),
        &transaction.join(BACKUP_ADDON_NAME),
        "1.0.0",
    )
    .unwrap();
    assert!(active_has_version(&project.join("addons/fennara"), "1.0.0"));
}

#[test]
fn restore_resumes_when_active_addon_is_missing() {
    let root = TestRoot::new("missing-active");
    let active = root.join("project/addons/fennara");
    let backup = root.join("transaction").join(BACKUP_ADDON_NAME);
    write_addon(&backup, "1.0.0");

    restore_addon(&active, &backup, "1.0.0").unwrap();

    assert!(active_has_version(&active, "1.0.0"));
    assert!(!backup.exists());
}

#[test]
fn repeated_restore_accepts_already_restored_version() {
    let root = TestRoot::new("repeated");
    let active = root.join("project/addons/fennara");
    let backup = root.join("transaction").join(BACKUP_ADDON_NAME);
    write_addon(&active, "1.0.0");

    restore_addon(&active, &backup, "1.0.0").unwrap();
}

#[test]
fn restore_rejects_new_addon_without_backup() {
    let root = TestRoot::new("missing-backup");
    let active = root.join("project/addons/fennara");
    let backup = root.join("transaction").join(BACKUP_ADDON_NAME);
    write_addon(&active, "1.1.0");

    assert!(restore_addon(&active, &backup, "1.0.0").is_err());
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
            "fennara-update-transaction-{name}-{}-{nonce}",
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
