use crate::app_layout::{AppLayout, binary_name, display_path};
use std::fs;
use std::path::Path;

const SNAPSHOT_DIR: &str = "previous-launchers";

pub(super) fn snapshot(layout: &AppLayout, root: &Path) -> Result<(), String> {
    let snapshot = root.join(SNAPSHOT_DIR);
    if snapshot.exists() {
        return Ok(());
    }
    fs::create_dir_all(&snapshot).map_err(|error| {
        format!(
            "failed to create launcher snapshot {}: {error}",
            display_path(&snapshot)
        )
    })?;
    for launcher in launcher_names() {
        let source = layout.bin_dir.join(&launcher);
        if source.is_file() {
            copy(&source, &snapshot.join(&launcher))?;
        } else {
            fs::write(snapshot.join(format!("{launcher}.missing")), b"missing\n").map_err(
                |error| format!("failed to record missing launcher {launcher}: {error}"),
            )?;
        }
    }
    Ok(())
}

pub(super) fn restore(layout: &AppLayout, root: &Path) -> Result<(), String> {
    let snapshot = root.join(SNAPSHOT_DIR);
    if !snapshot.is_dir() {
        return Err(format!(
            "launcher snapshot is missing at {}",
            display_path(&snapshot)
        ));
    }
    for launcher in launcher_names() {
        let saved = snapshot.join(&launcher);
        let missing = snapshot.join(format!("{launcher}.missing"));
        let target = layout.bin_dir.join(&launcher);
        if saved.is_file() {
            copy(&saved, &target)?;
        } else if missing.is_file() {
            if target.exists() {
                fs::remove_file(&target).map_err(|error| {
                    format!(
                        "failed to remove newly installed launcher {}: {error}",
                        display_path(&target)
                    )
                })?;
            }
        } else {
            return Err(format!("launcher snapshot is incomplete for {launcher}"));
        }
    }
    Ok(())
}

pub(super) fn cleanup(root: &Path) -> Result<(), String> {
    let snapshot = root.join(SNAPSHOT_DIR);
    if snapshot.exists() {
        fs::remove_dir_all(&snapshot).map_err(|error| {
            format!(
                "failed to remove launcher snapshot {}: {error}",
                display_path(&snapshot)
            )
        })?;
    }
    Ok(())
}

fn launcher_names() -> [String; 2] {
    [binary_name("fennara-mcp"), binary_name("fennara-daemon")]
}

fn copy(source: &Path, target: &Path) -> Result<(), String> {
    fs::copy(source, target).map_err(|error| {
        format!(
            "failed to copy launcher {} to {}: {error}",
            display_path(source),
            display_path(target)
        )
    })?;
    fs::OpenOptions::new()
        .write(true)
        .open(target)
        .and_then(|file| file.sync_all())
        .map_err(|error| format!("failed to flush launcher {}: {error}", display_path(target)))
}

#[cfg(test)]
mod tests;
