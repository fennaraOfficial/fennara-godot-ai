use super::launchers;
use super::options::ApplyOptions;
use super::process::{reopen_godot, wait_for_handshake};
use super::{activate_runtime_and_guidance, recovery_required, set_state};
use crate::app_layout::{AppLayout, display_path};
use crate::daemon_setup;
use crate::operation::{self, FailureClass, Phase};
use crate::project_addon;
use crate::project_install;
use crate::release_package;
use crate::update_stage::{self, UpdateReceipt};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub(super) fn apply_after_exit(
    options: &ApplyOptions,
    root: &Path,
    receipt_path: &Path,
    receipt: &mut UpdateReceipt,
) -> Result<(), String> {
    operation::phase(
        Phase::Applying,
        "Replacing the project addon with the verified update",
    )?;
    set_state(receipt_path, receipt, "applying")?;
    let layout = AppLayout::detect()?;
    daemon_setup::shutdown_if_running(&layout)
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    update_stage::verify_staged_addon(root, receipt)
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    launchers::snapshot(&layout, root)
        .map_err(|error| operation::failure(FailureClass::StageFilesystem, error))?;
    receipt.launchers_snapshotted = true;
    if let Err(error) = update_stage::write_receipt(receipt_path, receipt) {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    if let Err(error) = persist_previous_manifest(&layout, root, receipt) {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    if let Err(error) = release_package::activate_staged_launchers(&receipt.to_version) {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    if let Err(error) = replace_addon(&options.project_dir, root) {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    receipt.addon_replaced = true;
    if let Err(error) = update_stage::write_receipt(receipt_path, receipt) {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }

    if let Err(error) = activate_runtime_and_guidance(&options.project_dir, &receipt.to_version) {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    if let Err(error) = operation::phase(Phase::Reopening, "Reopening Godot with the updated addon")
    {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    if let Err(error) = set_state(receipt_path, receipt, "reopening") {
        return rollback_before_reopen(options, root, receipt_path, receipt, error);
    }
    let reopened_pid = match reopen_godot(&options.godot_executable, &options.project_dir) {
        Ok(pid) => pid,
        Err(error) => return rollback_before_reopen(options, root, receipt_path, receipt, error),
    };
    operation::phase(
        Phase::Validating,
        "Waiting for the updated GDExtension activation handshake",
    )?;
    set_state(receipt_path, receipt, "validating")?;
    if let Err(error) = wait_for_handshake(
        root,
        &receipt.operation_id,
        &receipt.to_version,
        reopened_pid,
    ) {
        return recovery_required(receipt_path, receipt, error);
    }
    if let Err(error) = daemon_setup::ensure_running(&layout, &receipt.to_version) {
        return recovery_required(receipt_path, receipt, error);
    }
    set_state(receipt_path, receipt, "succeeded")?;
    operation::phase(
        Phase::Succeeded,
        "The updated addon and runtime were validated",
    )?;
    if let Err(error) = remove_validated_backup(root, &receipt.to_version) {
        eprintln!("warning: validated update cleanup is pending: {error}");
    }
    Ok(())
}

fn replace_addon(project_dir: &Path, root: &Path) -> Result<(), String> {
    let active = project_install::project_addon_dir(project_dir);
    let staged = root.join(update_stage::STAGED_ADDON_NAME);
    let backup = root.join(update_stage::BACKUP_ADDON_NAME);
    project_addon::validate(&staged)
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    if backup.exists() {
        return Err(operation::failure(
            FailureClass::RollbackFailed,
            format!("update backup already exists at {}", display_path(&backup)),
        ));
    }
    fs::rename(&active, &backup).map_err(|error| {
        operation::failure(
            FailureClass::StageFilesystem,
            format!(
                "failed to move the current addon {} to its update backup: {error}",
                display_path(&active)
            ),
        )
    })?;
    if let Err(error) = fs::rename(&staged, &active) {
        let restore = fs::rename(&backup, &active);
        return Err(if let Err(restore_error) = restore {
            operation::failure(
                FailureClass::RollbackFailed,
                format!(
                    "failed to activate staged addon: {error}; failed to restore previous addon: {restore_error}"
                ),
            )
        } else {
            operation::failure(
                FailureClass::StageFilesystem,
                format!("failed to activate the staged addon: {error}"),
            )
        });
    }
    Ok(())
}

fn rollback_before_reopen(
    options: &ApplyOptions,
    root: &Path,
    receipt_path: &Path,
    receipt: &mut UpdateReceipt,
    original_error: String,
) -> Result<(), String> {
    match restore_previous(&options.project_dir, root) {
        Ok(()) => {
            let state_error = set_state(receipt_path, receipt, "rolled_back").err();
            let phase_error =
                operation::phase(Phase::RolledBack, "The failed update was rolled back").err();
            let reopen_error = reopen_godot(&options.godot_executable, &options.project_dir).err();
            let mut detail =
                format!("update failed and the previous version was restored: {original_error}");
            for persistence_error in [state_error, phase_error].into_iter().flatten() {
                detail.push_str(&format!(
                    "; rollback status persistence failed: {persistence_error}"
                ));
            }
            if let Some(reopen_error) = reopen_error {
                detail.push_str(&format!(
                    "; Godot did not reopen after rollback: {reopen_error}"
                ));
            }
            let error = operation::failure(FailureClass::ValidationFailed, detail);
            let _ = operation::defer_completion();
            Err(error)
        }
        Err(rollback_error) => {
            let _ = set_state(receipt_path, receipt, "recovery_required");
            let _ = operation::phase(
                Phase::RecoveryRequired,
                "The update and automatic rollback failed; manual recovery is required",
            );
            let error = operation::failure(
                FailureClass::RollbackFailed,
                format!("update failed: {original_error}; rollback failed: {rollback_error}"),
            );
            let _ = operation::defer_completion();
            Err(error)
        }
    }
}

pub(super) fn restore_previous(project_dir: &Path, root: &Path) -> Result<(), String> {
    let receipt = update_stage::read_receipt(&root.join("receipt.json"))?;
    let layout = AppLayout::detect()?;
    let active = project_install::project_addon_dir(project_dir);
    let backup = root.join(update_stage::BACKUP_ADDON_NAME);
    let mut errors = match restore_addon(&active, &backup, &receipt.from_version) {
        Ok(()) => Vec::new(),
        Err(error) => vec![error],
    };
    if receipt.had_current_manifest.is_some()
        && let Err(error) = restore_previous_manifest(root)
    {
        errors.push(error);
    }
    if receipt.launchers_snapshotted
        && let Err(error) = launchers::restore(&layout, root)
    {
        errors.push(error);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

fn restore_addon(active: &Path, backup: &Path, from_version: &str) -> Result<(), String> {
    if backup.is_dir() {
        if active.exists() {
            fs::remove_dir_all(active).map_err(|error| {
                format!(
                    "failed to remove failed addon {}: {error}",
                    display_path(active)
                )
            })?;
        }
        if let Some(parent) = active.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create addon parent {}: {error}",
                    display_path(parent)
                )
            })?;
        }
        fs::rename(backup, active).map_err(|error| {
            format!(
                "failed to restore addon backup {}: {error}",
                display_path(backup)
            )
        })?;
    } else if !active_has_version(active, from_version) {
        return Err(format!(
            "update backup is missing at {} and the active addon is not version {}",
            display_path(backup),
            from_version
        ));
    }
    Ok(())
}

fn active_has_version(active: &Path, expected: &str) -> bool {
    project_addon::validate(active)
        .map(|addon| addon.version == expected)
        .unwrap_or(false)
}

fn persist_previous_manifest(
    layout: &AppLayout,
    root: &Path,
    receipt: &mut UpdateReceipt,
) -> Result<(), String> {
    let snapshot = root.join("previous-current.json");
    let missing = root.join("previous-current.missing");
    if layout.current_manifest_path.is_file() {
        let bytes = fs::read(&layout.current_manifest_path)
            .map_err(|error| format!("failed to read current runtime manifest: {error}"))?;
        write_synced(&snapshot, &bytes)?;
        receipt.had_current_manifest = Some(true);
    } else {
        write_synced(&missing, b"missing\n")?;
        receipt.had_current_manifest = Some(false);
    }
    update_stage::write_receipt(&root.join("receipt.json"), receipt)
}

fn restore_previous_manifest(root: &Path) -> Result<(), String> {
    let snapshot = root.join("previous-current.json");
    let missing = root.join("previous-current.missing");
    if snapshot.is_file() {
        let bytes = fs::read(&snapshot)
            .map_err(|error| format!("failed to read previous runtime manifest: {error}"))?;
        release_package::restore_manifest(Some(&bytes))
    } else if missing.is_file() {
        release_package::restore_manifest(None)
    } else {
        Err("previous runtime manifest snapshot is missing".to_string())
    }
}

fn remove_validated_backup(root: &Path, version: &str) -> Result<(), String> {
    let backup = root.join(update_stage::BACKUP_ADDON_NAME);
    if backup.exists() {
        fs::remove_dir_all(&backup).map_err(|error| {
            operation::failure(
                FailureClass::StageFilesystem,
                format!(
                    "failed to remove validated update backup {}: {error}",
                    display_path(&backup)
                ),
            )
        })?;
    }
    for path in [
        root.join("previous-current.json"),
        root.join("previous-current.missing"),
    ] {
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|error| format!("failed to remove {}: {error}", display_path(&path)))?;
        }
    }
    launchers::cleanup(root)?;
    release_package::remove_staged_launchers(version)?;
    Ok(())
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = File::create(path)
        .map_err(|error| format!("failed to create {}: {error}", display_path(path)))?;
    file.write_all(bytes)
        .map_err(|error| format!("failed to write {}: {error}", display_path(path)))?;
    file.sync_all()
        .map_err(|error| format!("failed to flush {}: {error}", display_path(path)))
}

#[cfg(test)]
mod tests;
