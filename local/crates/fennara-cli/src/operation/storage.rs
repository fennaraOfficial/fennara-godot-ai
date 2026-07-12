use crate::app_layout::display_path;
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

const FILE_REPLACE_TIMEOUT: Duration = Duration::from_secs(2);
const FILE_REPLACE_RETRY_DELAY: Duration = Duration::from_millis(20);

pub(super) fn write_json_atomic(path: &Path, value: &Value) -> Result<(), String> {
    let temp = path.with_extension(format!("json.tmp-{}", std::process::id()));
    let backup = path.with_extension("json.previous");
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temp)
        .map_err(|err| format!("failed to create {}: {err}", display_path(&temp)))?;
    serde_json::to_writer_pretty(&mut file, value)
        .map_err(|err| format!("failed to serialize operation state: {err}"))?;
    file.write_all(b"\n")
        .map_err(|err| format!("failed to write {}: {err}", display_path(&temp)))?;
    file.sync_all()
        .map_err(|err| format!("failed to flush {}: {err}", display_path(&temp)))?;
    drop(file);

    if path.exists() {
        let _ = fs::remove_file(&backup);
        rename_with_retry(path, &backup).map_err(|err| {
            format!(
                "failed to back up {} as {}: {err}",
                display_path(path),
                display_path(&backup)
            )
        })?;
    }
    match rename_with_retry(&temp, path) {
        Ok(()) => {
            let _ = fs::remove_file(&backup);
            Ok(())
        }
        Err(error) => {
            if backup.exists() && !path.exists() {
                let _ = rename_with_retry(&backup, path);
            }
            Err(format!(
                "failed to activate {} as {}: {error}",
                display_path(&temp),
                display_path(path)
            ))
        }
    }
}

fn rename_with_retry(source: &Path, destination: &Path) -> std::io::Result<()> {
    let deadline = std::time::Instant::now() + FILE_REPLACE_TIMEOUT;
    loop {
        match fs::rename(source, destination) {
            Ok(()) => return Ok(()),
            Err(error)
                if error.kind() == std::io::ErrorKind::PermissionDenied
                    && std::time::Instant::now() < deadline =>
            {
                thread::sleep(FILE_REPLACE_RETRY_DELAY);
            }
            Err(error) => return Err(error),
        }
    }
}

pub(super) fn read_operation_state(path: &Path) -> Result<Value, String> {
    let backup = path.with_extension("json.previous");
    let selected = if path.is_file() {
        path
    } else if backup.is_file() {
        &backup
    } else {
        return Err(format!(
            "operation state is missing: {}",
            display_path(path)
        ));
    };
    let raw = fs::read_to_string(selected)
        .map_err(|err| format!("failed to read {}: {err}", display_path(selected)))?;
    serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", display_path(selected)))
}

pub(super) fn latest_operation_id(root: &Path) -> Result<Option<String>, String> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("failed to read {}: {error}", display_path(root))),
    };
    let mut latest: Option<(SystemTime, String)> = None;
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let Some(id) = path.file_stem().and_then(|value| value.to_str()) else {
            continue;
        };
        if validate_operation_id(id).is_err() {
            continue;
        }
        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .unwrap_or(UNIX_EPOCH);
        if latest.as_ref().is_none_or(|current| modified > current.0) {
            latest = Some((modified, id.to_string()));
        }
    }
    Ok(latest.map(|(_, id)| id))
}

pub(super) fn validate_operation_id(id: &str) -> Result<&str, String> {
    if id.is_empty()
        || id.len() > 128
        || !id
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err("operation ID contains invalid characters".to_string());
    }
    Ok(id)
}

pub(super) fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}
