use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if let Err(error) = run() {
        eprintln!("fennara-daemon launcher failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let runtime_path = runtime_path("daemon_runtime")?;
    Command::new(&runtime_path)
        .args(env::args_os().skip(1))
        .spawn()
        .map_err(|err| format!("failed to start {}: {err}", runtime_path.display()))?;

    Ok(())
}

fn runtime_path(field: &str) -> Result<PathBuf, String> {
    let app_dir = app_dir()?;
    let manifest_path = app_dir.join("current.json");
    let raw = fs::read_to_string(&manifest_path)
        .map_err(|err| format!("failed to read {}: {err}", manifest_path.display()))?;
    let manifest: Value = serde_json::from_str(&raw)
        .map_err(|err| format!("failed to parse {}: {err}", manifest_path.display()))?;
    let value = manifest
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{field} is missing from {}", manifest_path.display()))?;
    let path = PathBuf::from(value);

    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(app_dir.join(path))
    }
}

fn app_dir() -> Result<PathBuf, String> {
    let current_exe = env::current_exe().map_err(|err| err.to_string())?;
    current_exe
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "failed to resolve Fennara app directory".to_string())
}
