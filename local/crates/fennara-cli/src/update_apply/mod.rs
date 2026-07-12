mod launchers;
mod options;
mod process;
mod recovery;
mod transaction;

use self::options::ApplyOptions;
use self::process::{
    current_process_started_at, observe_process, reopen_godot, wait_for_process_exit,
};
use crate::operation::{self, FailureClass, Phase};
use crate::project_guidance;
use crate::release_package;
use crate::update_stage::{self, UpdateReceipt};
use std::path::Path;
use std::time::Duration;

pub const COMPLETE_COMMAND: &str = "__complete-project-update";
pub const ROLLBACK_COMMAND: &str = "__rollback-project-update";
const GODOT_EXIT_TIMEOUT: Duration = Duration::from_secs(10 * 60);

pub fn recover(args: Vec<&str>) -> Result<(), String> {
    recovery::run(args)
}

pub fn complete(args: Vec<&str>) -> Result<(), String> {
    let options = ApplyOptions::parse(args)?;
    let root = update_stage::staging_root(&options.project_dir, &options.operation_id)?;
    let receipt_path = root.join("receipt.json");
    let mut receipt = update_stage::read_receipt(&receipt_path)?;
    validate_receipt_identity(&receipt, &options.operation_id)?;
    let process = observe_process(options.wait_for_pid, &options.godot_executable)?;
    receipt.godot_pid = Some(process.pid);
    receipt.godot_started_at = Some(process.started_at);
    receipt.godot_executable = Some(options.godot_executable.display().to_string());
    receipt.updater_pid = Some(std::process::id());
    receipt.updater_started_at = current_process_started_at();
    set_state(&receipt_path, &mut receipt, "waiting_for_godot")?;
    operation::phase(
        Phase::WaitingForGodot,
        "Waiting for the confirmed Godot editor process to close",
    )?;
    if !wait_for_process_exit(&process, &root.join("cancel"), GODOT_EXIT_TIMEOUT) {
        set_state(&receipt_path, &mut receipt, "ready_to_close")?;
        operation::phase(
            Phase::ReadyToClose,
            "Godot stayed open, so the staged update remains ready for another attempt",
        )?;
        operation::defer_completion()?;
        return Ok(());
    }
    transaction::apply_after_exit(&options, &root, &receipt_path, &mut receipt)
}

pub fn rollback(args: Vec<&str>) -> Result<(), String> {
    let options = ApplyOptions::parse(args)?;
    let root = update_stage::staging_root(&options.project_dir, &options.operation_id)?;
    let receipt_path = root.join("receipt.json");
    let mut receipt = update_stage::read_receipt(&receipt_path)?;
    validate_receipt_identity(&receipt, &options.operation_id)?;
    let process = observe_process(options.wait_for_pid, &options.godot_executable)?;
    receipt.updater_pid = Some(std::process::id());
    receipt.updater_started_at = current_process_started_at();
    set_state(&receipt_path, &mut receipt, "waiting_for_godot")?;
    operation::phase(
        Phase::WaitingForGodot,
        "Waiting for Godot to close before restoring the previous version",
    )?;
    if !wait_for_process_exit(&process, &root.join("cancel"), GODOT_EXIT_TIMEOUT) {
        set_state(&receipt_path, &mut receipt, "recovery_required")?;
        operation::phase(
            Phase::RecoveryRequired,
            "Godot stayed open, so recovery still requires editor shutdown",
        )?;
        operation::defer_completion()?;
        return Ok(());
    }

    operation::phase(
        Phase::Applying,
        "Restoring the previous Fennara addon and runtime",
    )?;
    transaction::restore_previous(&options.project_dir, &root)?;
    let state_result = set_state(&receipt_path, &mut receipt, "rolled_back");
    let phase_result = operation::phase(
        Phase::RolledBack,
        "The previous Fennara version was restored",
    );
    reopen_godot(&options.godot_executable, &options.project_dir)
        .map_err(|error| operation::failure(FailureClass::HandoffFailed, error))?;
    state_result?;
    phase_result?;
    operation::defer_completion()?;
    Ok(())
}

pub(super) fn set_state(
    path: &Path,
    receipt: &mut UpdateReceipt,
    state: &str,
) -> Result<(), String> {
    receipt.state = state.to_string();
    update_stage::write_receipt(path, receipt)
}

fn validate_receipt_identity(receipt: &UpdateReceipt, operation_id: &str) -> Result<(), String> {
    if receipt.operation_id == operation_id {
        Ok(())
    } else {
        Err(format!(
            "update receipt belongs to operation {}, not {operation_id}",
            receipt.operation_id
        ))
    }
}

pub(super) fn recovery_required(
    receipt_path: &Path,
    receipt: &mut UpdateReceipt,
    error: String,
) -> Result<(), String> {
    set_state(receipt_path, receipt, "recovery_required")?;
    operation::phase(
        Phase::RecoveryRequired,
        "The updated editor is open but validation failed; user-confirmed recovery is required",
    )?;
    operation::defer_completion()?;
    eprintln!("update validation failed: {error}");
    Ok(())
}

pub(super) fn activate_runtime_and_guidance(
    project_dir: &Path,
    version: &str,
) -> Result<(), String> {
    release_package::activate_package(version)?;
    project_guidance::write(project_dir)
}
