use serde_json::Value;
use std::env;
use std::fs;
#[cfg(not(unix))]
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(not(unix))]
use std::process::Stdio;
#[cfg(not(unix))]
use std::thread;

fn main() {
    if let Err(error) = run() {
        eprintln!("fennara-mcp launcher failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let runtime_path = runtime_path("mcp_runtime")?;
    run_runtime(runtime_path)
}

#[cfg(unix)]
fn run_runtime(runtime_path: PathBuf) -> Result<(), String> {
    use std::os::unix::process::CommandExt;

    let error = Command::new(&runtime_path)
        .args(env::args_os().skip(1))
        .exec();
    Err(format!(
        "failed to exec {}: {error}",
        runtime_path.display()
    ))
}

#[cfg(not(unix))]
fn run_runtime(runtime_path: PathBuf) -> Result<(), String> {
    let mut child = Command::new(&runtime_path)
        .args(env::args_os().skip(1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|err| format!("failed to start {}: {err}", runtime_path.display()))?;

    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| "failed to open runtime stdin".to_string())?;
    let mut child_stdout = child
        .stdout
        .take()
        .ok_or_else(|| "failed to open runtime stdout".to_string())?;

    let stdin_thread = thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let _ = io::copy(&mut stdin, &mut child_stdin);
    });
    let stdout_thread = thread::spawn(move || {
        let mut stdout = io::stdout().lock();
        let _ = io::copy(&mut child_stdout, &mut stdout);
    });

    let status = child.wait().map_err(|err| err.to_string())?;
    let _ = stdin_thread.join();
    let _ = stdout_thread.join();

    std::process::exit(status.code().unwrap_or(1));
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
