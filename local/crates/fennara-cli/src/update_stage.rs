use crate::app_layout::{arch_name, display_path, platform_name};
use crate::operation;
use crate::project_addon;
use crate::project_install;
use crate::release_package::InstalledPackage;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

mod integrity;

const RECEIPT_SCHEMA_VERSION: u64 = 2;
pub(crate) const STAGED_ADDON_NAME: &str = "addon";
pub(crate) const BACKUP_ADDON_NAME: &str = "previous-addon";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateReceipt {
    pub schema_version: u64,
    pub operation_id: String,
    pub state: String,
    pub from_version: String,
    pub to_version: String,
    pub platform: String,
    pub architecture: String,
    pub addon: String,
    pub backup_addon: String,
    pub addon_sha256: String,
    pub godot_pid: Option<u32>,
    pub godot_started_at: Option<u64>,
    pub godot_executable: Option<String>,
    pub had_current_manifest: Option<bool>,
    pub launchers_snapshotted: bool,
    pub addon_replaced: bool,
    pub updater_pid: Option<u32>,
    pub updater_started_at: Option<u64>,
}

pub struct StagedUpdate {
    pub root: PathBuf,
    pub receipt_path: PathBuf,
    pub version: String,
}

pub fn staging_root(project_dir: &Path, operation_id: &str) -> Result<PathBuf, String> {
    operation::validate_id(operation_id)?;
    Ok(project_dir
        .join(".godot")
        .join("fennara-update")
        .join(operation_id))
}

pub fn prepare(
    project_dir: &Path,
    current_version: &str,
    package: &InstalledPackage,
    operation_id: &str,
    godot_process: Option<(u32, u64, &Path)>,
) -> Result<StagedUpdate, String> {
    operation::validate_id(operation_id)?;
    let source = project_addon::validate(&package.addon_dir)?;
    if source.version != package.version {
        return Err(format!(
            "staged addon version {} did not match package version {}",
            source.version, package.version
        ));
    }

    let staging_parent = project_dir.join(".godot").join("fennara-update");
    let final_root = staging_root(project_dir, operation_id)?;
    let preparing_root = staging_parent.join(format!("{operation_id}.preparing"));
    project_install::ensure_target_within_project(project_dir, &staging_parent)?;
    project_install::ensure_target_within_project(project_dir, &final_root)?;
    project_install::ensure_target_within_project(project_dir, &preparing_root)?;
    fs::create_dir_all(&staging_parent).map_err(|error| {
        format!(
            "failed to create update staging directory {}: {error}",
            display_path(&staging_parent)
        )
    })?;
    if final_root.exists() {
        return Err(format!(
            "update staging directory already exists at {}",
            display_path(&final_root)
        ));
    }
    if preparing_root.exists() {
        fs::remove_dir_all(&preparing_root).map_err(|error| {
            format!(
                "failed to clean incomplete update staging directory {}: {error}",
                display_path(&preparing_root)
            )
        })?;
    }

    let mut cleanup = PreparingCleanup::new(preparing_root.clone());
    let staged_addon = preparing_root.join(STAGED_ADDON_NAME);
    copy_dir_without_links(&package.addon_dir, &staged_addon)?;
    let staged = project_addon::validate(&staged_addon)?;
    if staged.version != package.version {
        return Err(format!(
            "copied addon version {} did not match package version {}",
            staged.version, package.version
        ));
    }

    let receipt_path = preparing_root.join("receipt.json");
    let receipt = UpdateReceipt {
        schema_version: RECEIPT_SCHEMA_VERSION,
        operation_id: operation_id.to_string(),
        state: "ready_to_close".to_string(),
        from_version: current_version.to_string(),
        to_version: package.version.clone(),
        platform: platform_name().to_string(),
        architecture: arch_name().to_string(),
        addon: STAGED_ADDON_NAME.to_string(),
        backup_addon: BACKUP_ADDON_NAME.to_string(),
        addon_sha256: integrity::hash_directory(&staged_addon)?,
        godot_pid: godot_process.map(|value| value.0),
        godot_started_at: godot_process.map(|value| value.1),
        godot_executable: godot_process.map(|value| value.2.display().to_string()),
        had_current_manifest: None,
        launchers_snapshotted: false,
        addon_replaced: false,
        updater_pid: None,
        updater_started_at: None,
    };
    write_receipt(&receipt_path, &receipt)?;
    fs::rename(&preparing_root, &final_root).map_err(|error| {
        format!(
            "failed to finalize update staging directory {}: {error}",
            display_path(&final_root)
        )
    })?;
    cleanup.disarm();

    Ok(StagedUpdate {
        receipt_path: final_root.join("receipt.json"),
        root: final_root,
        version: package.version.clone(),
    })
}

pub fn read_receipt(path: &Path) -> Result<UpdateReceipt, String> {
    let fallback = path.with_extension("json.previous");
    for candidate in [path, fallback.as_path()] {
        if !candidate.is_file() {
            continue;
        }
        let raw = fs::read(candidate)
            .map_err(|error| format!("failed to read {}: {error}", display_path(candidate)))?;
        let receipt: UpdateReceipt = serde_json::from_slice(&raw)
            .map_err(|error| format!("failed to parse {}: {error}", display_path(candidate)))?;
        if receipt.schema_version != RECEIPT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported update receipt schema {} in {}",
                receipt.schema_version,
                display_path(candidate)
            ));
        }
        validate_receipt(&receipt)?;
        return Ok(receipt);
    }
    Err(format!(
        "update receipt was not found at {}",
        display_path(path)
    ))
}

pub(crate) fn verify_staged_addon(root: &Path, receipt: &UpdateReceipt) -> Result<(), String> {
    let staged = root.join(STAGED_ADDON_NAME);
    let actual = integrity::hash_directory(&staged)?;
    if actual.eq_ignore_ascii_case(&receipt.addon_sha256) {
        Ok(())
    } else {
        Err(format!(
            "staged addon integrity check failed: expected {}, got {actual}",
            receipt.addon_sha256
        ))
    }
}

fn validate_receipt(receipt: &UpdateReceipt) -> Result<(), String> {
    operation::validate_id(&receipt.operation_id)?;
    if receipt.addon != STAGED_ADDON_NAME || receipt.backup_addon != BACKUP_ADDON_NAME {
        return Err("update receipt contains unsafe addon paths".to_string());
    }
    if receipt.platform != platform_name() || receipt.architecture != arch_name() {
        return Err("update receipt targets a different platform or architecture".to_string());
    }
    validate_version_component(&receipt.from_version)?;
    validate_version_component(&receipt.to_version)?;
    if receipt.addon_sha256.len() != 64
        || !receipt
            .addon_sha256
            .chars()
            .all(|ch| ch.is_ascii_hexdigit())
    {
        return Err("update receipt contains an invalid addon digest".to_string());
    }
    Ok(())
}

fn validate_version_component(version: &str) -> Result<(), String> {
    if version.is_empty()
        || version.len() > 128
        || version == "."
        || version == ".."
        || !version
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '+' | '_'))
    {
        return Err(format!("unsafe update version in receipt: {version}"));
    }
    Ok(())
}

pub fn write_receipt(path: &Path, receipt: &UpdateReceipt) -> Result<(), String> {
    let mut bytes = serde_json::to_vec_pretty(&receipt)
        .map_err(|error| format!("failed to serialize update staging receipt: {error}"))?;
    bytes.push(b'\n');
    let next = path.with_extension("json.next");
    let previous = path.with_extension("json.previous");
    let mut file = File::create(&next)
        .map_err(|error| format!("failed to create {}: {error}", display_path(&next)))?;
    file.write_all(&bytes)
        .map_err(|error| format!("failed to write {}: {error}", display_path(&next)))?;
    file.sync_all()
        .map_err(|error| format!("failed to flush {}: {error}", display_path(&next)))?;
    if previous.exists() {
        fs::remove_file(&previous)
            .map_err(|error| format!("failed to remove {}: {error}", display_path(&previous)))?;
    }
    if path.exists() {
        fs::rename(path, &previous).map_err(|error| {
            format!(
                "failed to preserve update receipt {}: {error}",
                display_path(path)
            )
        })?;
    }
    if let Err(error) = fs::rename(&next, path) {
        if previous.exists() {
            let _ = fs::rename(&previous, path);
        }
        return Err(format!(
            "failed to activate update receipt {}: {error}",
            display_path(path)
        ));
    }
    if previous.exists() {
        fs::remove_file(previous)
            .map_err(|error| format!("failed to remove previous update receipt: {error}"))?;
    }
    Ok(())
}

fn copy_dir_without_links(source: &Path, target: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(source)
        .map_err(|error| format!("failed to inspect {}: {error}", display_path(source)))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "refusing to stage non-directory or linked addon path {}",
            display_path(source)
        ));
    }
    fs::create_dir_all(target)
        .map_err(|error| format!("failed to create {}: {error}", display_path(target)))?;
    for entry in fs::read_dir(source)
        .map_err(|error| format!("failed to read {}: {error}", display_path(source)))?
    {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read an entry in {}: {error}",
                display_path(source)
            )
        })?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path).map_err(|error| {
            format!("failed to inspect {}: {error}", display_path(&source_path))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "refusing to stage linked addon entry {}",
                display_path(&source_path)
            ));
        }
        if metadata.is_dir() {
            copy_dir_without_links(&source_path, &target_path)?;
        } else if metadata.is_file() {
            fs::copy(&source_path, &target_path).map_err(|error| {
                format!(
                    "failed to copy {} to {}: {error}",
                    display_path(&source_path),
                    display_path(&target_path)
                )
            })?;
        } else {
            return Err(format!(
                "refusing to stage unsupported addon entry {}",
                display_path(&source_path)
            ));
        }
    }
    Ok(())
}

struct PreparingCleanup {
    path: PathBuf,
    armed: bool,
}

impl PreparingCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for PreparingCleanup {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests;
