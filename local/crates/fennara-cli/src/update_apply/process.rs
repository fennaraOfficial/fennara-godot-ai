use crate::app_layout::display_path;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

const ACTIVATION_TIMEOUT: Duration = Duration::from_secs(90);
const POLL_INTERVAL: Duration = Duration::from_millis(250);

pub(super) struct ProcessIdentity {
    pub pid: u32,
    pub started_at: u64,
}

pub(super) fn observe_process(pid: u32, executable: &Path) -> Result<ProcessIdentity, String> {
    let mut system = System::new();
    system.refresh_processes();
    let process = system
        .process(Pid::from_u32(pid))
        .ok_or_else(|| format!("Godot process {pid} is not running"))?;
    if let Some(actual) = process.exe()
        && canonical_or_original(actual) != canonical_or_original(executable)
    {
        return Err(format!(
            "process {pid} does not match the selected Godot executable"
        ));
    }
    Ok(ProcessIdentity {
        pid,
        started_at: process.start_time(),
    })
}

pub(super) fn wait_for_process_exit(
    process: &ProcessIdentity,
    cancel: &Path,
    timeout: Duration,
) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if cancel.exists() {
            let _ = fs::remove_file(cancel);
            return false;
        }
        let mut system = System::new();
        system.refresh_processes();
        match system.process(Pid::from_u32(process.pid)) {
            Some(current) if current.start_time() == process.started_at => {
                thread::sleep(POLL_INTERVAL)
            }
            _ => return true,
        }
    }
    false
}

pub(super) fn reopen_godot(executable: &Path, project_dir: &Path) -> Result<u32, String> {
    let child = Command::new(executable)
        .arg("--editor")
        .arg("--path")
        .arg(project_dir)
        .spawn()
        .map_err(|error| {
            format!(
                "failed to reopen Godot through {}: {error}",
                display_path(executable)
            )
        })?;
    Ok(child.id())
}

pub(super) fn wait_for_handshake(
    root: &Path,
    operation_id: &str,
    expected_version: &str,
    reopened_pid: u32,
) -> Result<(), String> {
    let path = root.join("activation-handshake.json");
    let deadline = Instant::now() + ACTIVATION_TIMEOUT;
    while Instant::now() < deadline {
        if let Ok(raw) = fs::read(&path)
            && let Ok(value) = serde_json::from_slice::<Value>(&raw)
            && value.get("operation_id").and_then(Value::as_str) == Some(operation_id)
            && value.get("addon_version").and_then(Value::as_str) == Some(expected_version)
        {
            return Ok(());
        }
        if !pid_exists(reopened_pid) {
            return Err("Godot exited before the updated addon reported activation".to_string());
        }
        thread::sleep(POLL_INTERVAL);
    }
    Err(format!(
        "updated addon did not report activation within {} seconds",
        ACTIVATION_TIMEOUT.as_secs()
    ))
}

fn pid_exists(pid: u32) -> bool {
    let mut system = System::new();
    system.refresh_processes();
    system.process(Pid::from_u32(pid)).is_some()
}

pub(super) fn identity_is_running(pid: Option<u32>, started_at: Option<u64>) -> bool {
    let (Some(pid), Some(started_at)) = (pid, started_at) else {
        return false;
    };
    let mut system = System::new();
    system.refresh_processes();
    system
        .process(Pid::from_u32(pid))
        .is_some_and(|process| process.start_time() == started_at)
}

pub(super) fn current_process_started_at() -> Option<u64> {
    let mut system = System::new();
    system.refresh_processes();
    system
        .process(Pid::from_u32(std::process::id()))
        .map(|process| process.start_time())
}

fn canonical_or_original(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}
