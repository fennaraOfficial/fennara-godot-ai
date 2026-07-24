use serde::{Deserialize, Serialize};
use std::{
    fmt::Write as _,
    fs,
    io::Write,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

pub(super) const SCHEMA_VERSION: u8 = 1;
static WRITE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct TelemetryState {
    pub(super) schema_version: u8,
    pub(super) installation_id: String,
    pub(super) last_sent_utc_day: Option<u64>,
}

pub(super) fn load_or_create(path: &Path) -> Result<TelemetryState, String> {
    if let Some(state) = read_valid(path)? {
        return Ok(state);
    }
    let state = TelemetryState {
        schema_version: SCHEMA_VERSION,
        installation_id: generate_installation_id()?,
        last_sent_utc_day: None,
    };
    write(path, &state)?;
    Ok(state)
}

pub(super) fn read_valid(path: &Path) -> Result<Option<TelemetryState>, String> {
    let previous = path.with_extension("json.previous");
    let selected = if path.is_file() {
        path
    } else if previous.is_file() {
        previous.as_path()
    } else {
        return Ok(None);
    };
    let raw = fs::read_to_string(selected)
        .map_err(|error| format!("failed to read {}: {error}", selected.display()))?;
    let Ok(state) = serde_json::from_str::<TelemetryState>(&raw) else {
        return Ok(None);
    };
    Ok(
        (state.schema_version == SCHEMA_VERSION
            && is_valid_installation_id(&state.installation_id))
        .then_some(state),
    )
}

pub(super) fn write(path: &Path, state: &TelemetryState) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| "telemetry state path has no parent".to_string())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    let raw = serde_json::to_vec_pretty(state).map_err(|error| error.to_string())?;
    let sequence = WRITE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = path.with_extension(format!("json.tmp-{}-{sequence}", std::process::id()));
    let result = write_secure_temp(&temp, &raw).and_then(|_| replace_file(&temp, path));
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

pub(super) fn remove(path: &Path) {
    let _ = fs::remove_file(path);
    let _ = fs::remove_file(path.with_extension("json.previous"));
    if let Some(parent) = path.parent() {
        let _ = fs::remove_dir(parent);
    }
}

pub(super) fn generate_installation_id() -> Result<String, String> {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes)
        .map_err(|error| format!("failed to generate telemetry installation id: {error}"))?;
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let mut result = String::with_capacity(36);
    for (index, byte) in bytes.iter().enumerate() {
        if matches!(index, 4 | 6 | 8 | 10) {
            result.push('-');
        }
        write!(result, "{byte:02x}").expect("writing to a String cannot fail");
    }
    Ok(result)
}

pub(super) fn is_valid_installation_id(value: &str) -> bool {
    value.len() == 36
        && value.bytes().enumerate().all(|(index, byte)| match index {
            8 | 13 | 18 | 23 => byte == b'-',
            14 => byte == b'4',
            19 => matches!(byte, b'8' | b'9' | b'a' | b'b' | b'A' | b'B'),
            _ => byte.is_ascii_hexdigit(),
        })
}

fn write_secure_temp(path: &Path, contents: &[u8]) -> Result<(), String> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
    file.write_all(contents)
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_all())
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

#[cfg(not(windows))]
fn replace_file(temp: &Path, path: &Path) -> Result<(), String> {
    fs::rename(temp, path).map_err(|error| {
        format!(
            "failed to replace {} with {}: {error}",
            path.display(),
            temp.display()
        )
    })
}

#[cfg(windows)]
fn replace_file(temp: &Path, path: &Path) -> Result<(), String> {
    let backup = path.with_extension("json.previous");
    let had_current = path.is_file();
    if had_current {
        if backup.is_file() {
            fs::remove_file(&backup)
                .map_err(|error| format!("failed to remove {}: {error}", backup.display()))?;
        }
        fs::rename(path, &backup).map_err(|error| {
            format!(
                "failed to back up {} as {}: {error}",
                path.display(),
                backup.display()
            )
        })?;
    }
    match fs::rename(temp, path) {
        Ok(()) => {
            let _ = fs::remove_file(backup);
            Ok(())
        }
        Err(error) => {
            if had_current && backup.is_file() && !path.exists() {
                let _ = fs::rename(&backup, path);
            }
            Err(format!("failed to replace {}: {error}", path.display()))
        }
    }
}
