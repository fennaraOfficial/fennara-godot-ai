use super::*;
use crate::app_layout::{arch_name, platform_name};
use crate::update_stage::{BACKUP_ADDON_NAME, STAGED_ADDON_NAME};
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn finds_interrupted_operation_without_loading_addon() {
    let root = TestRoot::new("find");
    let project = root.join("project");
    let operation = update_stage::staging_root(&project, "update-123-recovery").unwrap();
    fs::create_dir_all(&operation).unwrap();
    let receipt = receipt("update-123-recovery", "recovery_required");
    update_stage::write_receipt(&operation.join("receipt.json"), &receipt).unwrap();

    let (found_root, found_receipt) = find_operation(&project, None).unwrap();

    assert_eq!(found_root, operation);
    assert_eq!(found_receipt.operation_id, "update-123-recovery");
}

#[test]
fn ignores_updates_that_are_only_ready_to_close() {
    let root = TestRoot::new("ready");
    let project = root.join("project");
    let operation = update_stage::staging_root(&project, "update-456-ready").unwrap();
    fs::create_dir_all(&operation).unwrap();
    let receipt = receipt("update-456-ready", "ready_to_close");
    update_stage::write_receipt(&operation.join("receipt.json"), &receipt).unwrap();

    assert!(find_operation(&project, None).is_err());
}

#[test]
fn rejects_explicit_update_that_is_only_ready_to_close() {
    let root = TestRoot::new("explicit-ready");
    let project = root.join("project");
    fs::create_dir_all(&project).unwrap();
    let operation = update_stage::staging_root(&project, "update-456-ready").unwrap();
    fs::create_dir_all(&operation).unwrap();
    let receipt = receipt("update-456-ready", "ready_to_close");
    update_stage::write_receipt(&operation.join("receipt.json"), &receipt).unwrap();

    assert!(find_operation(&project, Some("update-456-ready")).is_err());
}

fn receipt(operation_id: &str, state: &str) -> UpdateReceipt {
    UpdateReceipt {
        schema_version: 2,
        operation_id: operation_id.to_string(),
        state: state.to_string(),
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

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(std::env::temp_dir().join(format!(
            "fennara-update-recovery-{name}-{}-{nonce}",
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
