use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    sync::OnceLock,
    time::Duration,
};
use tokio::{
    process::Command,
    sync::{Mutex, MutexGuard},
    time::timeout,
};

const SETUP_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_REPORT_CHARS: usize = 16_000;

static SETUP_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Serialize)]
pub(crate) struct SetupResult {
    pub(crate) target: String,
    pub(crate) report: String,
    pub(crate) warning: Option<String>,
}

pub(crate) async fn run(target: &str) -> Result<SetupResult, String> {
    let flag = target_flag(target).ok_or_else(|| format!("Unsupported MCP app: {target}."))?;
    let _guard = try_setup_guard()?;
    let cli_path = resolve_cli_path()?;
    let mut command = Command::new(&cli_path);
    command.arg("mcp-setup").arg(flag).kill_on_drop(true);

    let output = timeout(SETUP_TIMEOUT, command.output())
        .await
        .map_err(|_| "Fennara CLI setup timed out after 60 seconds.".to_string())?
        .map_err(|error| format!("Could not start Fennara CLI: {error}"))?;

    let stdout = bounded_text(&output.stdout);
    let stderr = bounded_text(&output.stderr);
    if !output.status.success() {
        let detail = if stderr.is_empty() { stdout } else { stderr };
        return Err(if detail.is_empty() {
            format!("Fennara CLI exited with status {}.", output.status)
        } else {
            detail
        });
    }

    let warning = stdout
        .lines()
        .find(|line| line.contains(" skipped:"))
        .map(str::to_string);
    Ok(SetupResult {
        target: target.to_string(),
        report: stdout,
        warning,
    })
}

fn try_setup_guard() -> Result<MutexGuard<'static, ()>, String> {
    SETUP_LOCK
        .get_or_init(|| Mutex::new(()))
        .try_lock()
        .map_err(|_| "Another Fennara MCP setup is already running. Try again shortly.".to_string())
}

fn target_flag(target: &str) -> Option<&'static str> {
    match target {
        "claude" => Some("--claude"),
        "gemini" => Some("--gemini"),
        "cline" => Some("--cline"),
        "cursor" => Some("--cursor"),
        "vscode" => Some("--vscode"),
        "opencode" => Some("--opencode"),
        "windsurf" => Some("--windsurf"),
        "kiro" => Some("--kiro"),
        "codex" => Some("--codex"),
        _ => None,
    }
}

fn resolve_cli_path() -> Result<PathBuf, String> {
    let binary = format!("fennara{}", env::consts::EXE_SUFFIX);
    let current_exe = env::current_exe()
        .map_err(|error| format!("Could not locate the Fennara daemon: {error}"))?;

    if let Some(app_dir) = current_exe
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
    {
        let installed = app_dir.join("bin").join(&binary);
        if installed.is_file() {
            return Ok(installed);
        }
    }

    if let Some(sibling) = current_exe.parent().map(|dir| dir.join(&binary))
        && sibling.is_file()
    {
        return Ok(sibling);
    }

    if let Some(path) = find_on_path(&binary) {
        return Ok(path);
    }

    Err("The installed Fennara CLI could not be found. Run Fennara setup first.".to_string())
}

fn find_on_path(binary: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path)
        .map(|dir| dir.join(binary))
        .find(|path| path.is_file())
}

fn bounded_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .trim()
        .chars()
        .take(MAX_REPORT_CHARS)
        .collect()
}

#[cfg(test)]
mod tests;
