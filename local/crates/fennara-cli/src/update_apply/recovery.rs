use super::process::{identity_is_running, reopen_godot};
use super::{set_state, transaction};
use crate::app_layout::display_path;
use crate::operation::{self, Phase};
use crate::update_stage::{self, UpdateReceipt};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub(super) fn run(args: Vec<&str>) -> Result<(), String> {
    let options = RecoveryOptions::parse(args)?;
    let (root, mut receipt) =
        find_operation(&options.project_dir, options.operation_id.as_deref())?;
    if identity_is_running(receipt.updater_pid, receipt.updater_started_at) {
        return Err(
            "the update process is still running; wait for it to finish before recovery"
                .to_string(),
        );
    }
    if identity_is_running(receipt.godot_pid, receipt.godot_started_at) {
        return Err("close the Godot editor for this project before recovery".to_string());
    }

    operation::phase(Phase::Applying, "Restoring an interrupted Fennara update")?;
    transaction::restore_previous(&options.project_dir, &root)?;
    set_state(&root.join("receipt.json"), &mut receipt, "rolled_back")?;
    operation::phase(Phase::RolledBack, "The interrupted update was restored")?;
    if let Some(executable) = receipt.godot_executable.as_deref().map(Path::new)
        && executable.is_file()
    {
        reopen_godot(executable, &options.project_dir)?;
    }
    println!(
        "restored Fennara {} for {}",
        receipt.from_version,
        display_path(&options.project_dir)
    );
    Ok(())
}

fn find_operation(
    project_dir: &Path,
    operation_id: Option<&str>,
) -> Result<(PathBuf, UpdateReceipt), String> {
    if let Some(operation_id) = operation_id {
        let root = update_stage::staging_root(project_dir, operation_id)?;
        let receipt = update_stage::read_receipt(&root.join("receipt.json"))?;
        return Ok((root, receipt));
    }

    let parent = project_dir.join(".godot/fennara-update");
    let mut candidates = Vec::new();
    for entry in fs::read_dir(&parent).map_err(|error| {
        format!(
            "failed to read update recovery directory {}: {error}",
            display_path(&parent)
        )
    })? {
        let entry =
            entry.map_err(|error| format!("failed to read update recovery entry: {error}"))?;
        if !entry.path().is_dir() || entry.file_name().to_string_lossy().ends_with(".preparing") {
            continue;
        }
        let Ok(receipt) = update_stage::read_receipt(&entry.path().join("receipt.json")) else {
            continue;
        };
        if !matches!(
            receipt.state.as_str(),
            "applying" | "reopening" | "validating" | "recovery_required"
        ) {
            continue;
        }
        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        candidates.push((modified, entry.path(), receipt));
    }
    candidates
        .into_iter()
        .max_by_key(|candidate| candidate.0)
        .map(|(_, root, receipt)| (root, receipt))
        .ok_or_else(|| "no interrupted Fennara update is available to recover".to_string())
}

struct RecoveryOptions {
    project_dir: PathBuf,
    operation_id: Option<String>,
}

impl RecoveryOptions {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut project_dir = None;
        let mut operation_id = None;
        let mut index = 0;
        while index < args.len() {
            match args[index] {
                "--project" => {
                    index += 1;
                    project_dir = Some(PathBuf::from(value(&args, index, "--project")?));
                }
                "--operation" => {
                    index += 1;
                    let value = value(&args, index, "--operation")?;
                    operation::validate_id(value)?;
                    operation_id = Some(value.to_string());
                }
                other => return Err(format!("unknown recovery option: {other}")),
            }
            index += 1;
        }
        Ok(Self {
            project_dir: project_dir.ok_or_else(|| "--project is required".to_string())?,
            operation_id,
        })
    }
}

fn value<'a>(args: &'a [&str], index: usize, option: &str) -> Result<&'a str, String> {
    args.get(index)
        .copied()
        .ok_or_else(|| format!("{option} requires a value"))
}

#[cfg(test)]
mod tests;
