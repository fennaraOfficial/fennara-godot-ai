use std::{
    env,
    path::{Path, PathBuf},
    time::Duration,
};

use sysinfo::System;

const LOW_MEMORY_AVAILABLE_BYTES: u64 = 1_500_000_000;
const HIGH_MEMORY_USED_RATIO: f64 = 0.90;

pub(crate) fn resolve_godot_executable(sent_path: &str) -> Option<PathBuf> {
    let trimmed = sent_path.trim();
    if !trimmed.is_empty() {
        let path = PathBuf::from(trimmed);
        if path.is_file() {
            return Some(path);
        }
    }

    for candidate in ["godot", "godot4", "godot-mono", "godot4-mono"] {
        if let Some(path) = find_on_path(candidate) {
            return Some(path);
        }
    }
    None
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(target_os = "windows")]
        {
            let candidate_exe = dir.join(format!("{name}.exe"));
            if candidate_exe.is_file() {
                return Some(candidate_exe);
            }
        }
    }
    None
}

pub(crate) fn allowed_child_env(key: &str) -> bool {
    matches!(key, "FENNARA_RT_SPEC")
}

pub(crate) async fn wait_for_memory_headroom() {
    for _ in 0..20 {
        if has_memory_headroom() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

fn has_memory_headroom() -> bool {
    let mut system = System::new();
    system.refresh_memory();
    let total = system.total_memory();
    let available = system.available_memory();
    if total == 0 {
        return true;
    }
    let used_ratio = 1.0 - (available as f64 / total as f64);
    available >= LOW_MEMORY_AVAILABLE_BYTES && used_ratio < HIGH_MEMORY_USED_RATIO
}

pub(crate) async fn append_runtime_log_footer(
    path: &Path,
    status: &str,
    exit_code: i32,
    duration_seconds: f64,
) -> Result<(), String> {
    let footer = format!(
        "\n\n## Fennara daemon process result\nStatus: {status}\nExit code: {exit_code}\nDuration: {duration_seconds:.3}s\n"
    );
    let mut file = tokio::fs::OpenOptions::new()
        .append(true)
        .open(path)
        .await
        .map_err(|err| format!("open raw log for footer failed: {err}"))?;
    tokio::io::AsyncWriteExt::write_all(&mut file, footer.as_bytes())
        .await
        .map_err(|err| format!("write raw log footer failed: {err}"))
}
