use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    env, fs, io,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::runtime_daemon::state::AppState;

use super::tools::ExecutedTool;

const DEFAULT_TIMEOUT_MS: u64 = 30_000;
const MAX_TIMEOUT_MS: u64 = 5 * 60 * 1000;
const OUTPUT_MAX_BYTES: usize = 256 * 1024;
const READ_CHUNK_SIZE: usize = 8192;
const WAIT_AFTER_KILL_MS: u64 = 2_000;
const OUTPUT_DRAIN_TIMEOUT_MS: u64 = 2_000;
const POWERSHELL_UTF8_PREFIX: &str =
    "try { [Console]::OutputEncoding=[System.Text.Encoding]::UTF8 } catch {}\n";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ShellKind {
    Zsh,
    Bash,
    Sh,
    PowerShell,
    Cmd,
}

impl ShellKind {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Zsh => "zsh",
            Self::Bash => "bash",
            Self::Sh => "sh",
            Self::PowerShell => "powershell",
            Self::Cmd => "cmd",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ShellInfo {
    pub(crate) kind: ShellKind,
    pub(crate) path: PathBuf,
}

impl ShellInfo {
    pub(crate) fn name(&self) -> &'static str {
        self.kind.name()
    }
}

#[derive(Debug, Deserialize)]
struct ExecCommandRequest {
    command: String,
    #[serde(default)]
    cwd: Option<String>,
    #[serde(default)]
    timeout_ms: Option<u64>,
    #[serde(default)]
    shell: Option<String>,
    #[serde(default)]
    justification: Option<String>,
}

#[derive(Debug)]
struct ResolvedRequest {
    command: String,
    cwd: PathBuf,
    timeout: Duration,
    shell: ShellInfo,
    argv: Vec<String>,
}

#[derive(Debug)]
struct ProcessOutput {
    status: ExecStatus,
    exit_code: Option<i32>,
    stdout: CapturedOutput,
    stderr: CapturedOutput,
    duration: Duration,
    timed_out: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExecStatus {
    Completed,
    TimedOut,
    Cancelled,
}

impl ExecStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug)]
struct CapturedOutput {
    text: String,
    truncated: bool,
}

#[derive(Clone)]
struct CancellationCheck {
    state: Option<AppState>,
    chat_id: Option<String>,
}

impl CancellationCheck {
    #[cfg(test)]
    fn none() -> Self {
        Self {
            state: None,
            chat_id: None,
        }
    }

    fn for_chat(state: &AppState, chat_id: &str) -> Self {
        Self {
            state: Some(state.clone()),
            chat_id: Some(chat_id.to_string()),
        }
    }

    fn is_cancelled(&self) -> bool {
        match (&self.state, &self.chat_id) {
            (Some(state), Some(chat_id)) => state.cancelled_chats.blocking_read().contains(chat_id),
            _ => false,
        }
    }
}

pub(crate) async fn execute(
    state: &AppState,
    chat_id: &str,
    project_root: Option<&str>,
    arguments: &Value,
) -> ExecutedTool {
    let request = match serde_json::from_value::<ExecCommandRequest>(arguments.clone()) {
        Ok(request) => request,
        Err(error) => {
            return failed_exec_tool("validation_failed", format!("Invalid arguments: {error}"));
        }
    };

    match execute_request_with_cancellation(
        project_root,
        request,
        OUTPUT_MAX_BYTES,
        CancellationCheck::for_chat(state, chat_id),
    )
    .await
    {
        Ok((resolved, output)) => completed_exec_tool(resolved, output),
        Err(error) => failed_exec_tool(error.status, error.message),
    }
}

pub(crate) fn default_shell() -> ShellInfo {
    default_shell_from_user_path(user_shell_path())
}

pub(crate) fn detect_shell_kind(shell_path: impl AsRef<Path>) -> Option<ShellKind> {
    let shell_path = shell_path.as_ref();
    let raw = shell_path.as_os_str().to_str()?.trim().to_ascii_lowercase();
    let file_name = raw.rsplit(['/', '\\']).next().unwrap_or(raw.as_str());
    let stem = file_name.strip_suffix(".exe").unwrap_or(file_name);
    shell_kind_from_name(stem)
}

fn shell_kind_from_name(name: &str) -> Option<ShellKind> {
    match name {
        "zsh" => Some(ShellKind::Zsh),
        "bash" => Some(ShellKind::Bash),
        "sh" => Some(ShellKind::Sh),
        "pwsh" | "powershell" => Some(ShellKind::PowerShell),
        "cmd" => Some(ShellKind::Cmd),
        _ => None,
    }
}

fn default_shell_from_user_path(user_shell_path: Option<PathBuf>) -> ShellInfo {
    if cfg!(windows) {
        get_shell(ShellKind::PowerShell, None).unwrap_or_else(ultimate_fallback_shell)
    } else {
        let user_shell = user_shell_path
            .as_ref()
            .and_then(detect_shell_kind)
            .and_then(|kind| get_shell(kind, user_shell_path.as_ref()));
        let shell = if cfg!(target_os = "macos") {
            user_shell
                .or_else(|| get_shell(ShellKind::Zsh, None))
                .or_else(|| get_shell(ShellKind::Bash, None))
                .or_else(|| get_shell(ShellKind::Sh, None))
        } else {
            user_shell
                .or_else(|| get_shell(ShellKind::Bash, None))
                .or_else(|| get_shell(ShellKind::Zsh, None))
                .or_else(|| get_shell(ShellKind::Sh, None))
        };
        shell.unwrap_or_else(ultimate_fallback_shell)
    }
}

fn user_shell_path() -> Option<PathBuf> {
    env::var_os("SHELL")
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
}

fn ultimate_fallback_shell() -> ShellInfo {
    if cfg!(windows) {
        ShellInfo {
            kind: ShellKind::Cmd,
            path: PathBuf::from("cmd.exe"),
        }
    } else {
        ShellInfo {
            kind: ShellKind::Sh,
            path: PathBuf::from("/bin/sh"),
        }
    }
}

fn get_shell(kind: ShellKind, provided_path: Option<&PathBuf>) -> Option<ShellInfo> {
    let (binary_names, fallback_paths): (&[&str], &[&str]) = match kind {
        ShellKind::Zsh => (&["zsh"], &["/bin/zsh"]),
        ShellKind::Bash => (&["bash"], &["/bin/bash", "/usr/bin/bash"]),
        ShellKind::Sh => (&["sh"], &["/bin/sh"]),
        ShellKind::PowerShell => {
            if cfg!(windows) {
                (
                    &["pwsh", "pwsh.exe", "powershell", "powershell.exe"],
                    &[
                        r"C:\Program Files\PowerShell\7\pwsh.exe",
                        r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe",
                    ],
                )
            } else {
                (&["pwsh", "powershell"], &["/usr/local/bin/pwsh"])
            }
        }
        ShellKind::Cmd => (&["cmd", "cmd.exe"], &[]),
    };

    if let Some(path) = provided_path {
        if is_path_like(path) {
            return file_exists(path).map(|path| ShellInfo { kind, path });
        }
        if let Some(found) = find_on_path(path.to_string_lossy().as_ref()) {
            return Some(ShellInfo { kind, path: found });
        }
    }

    for binary_name in binary_names {
        if let Some(path) = find_on_path(binary_name) {
            return Some(ShellInfo { kind, path });
        }
    }
    for path in fallback_paths {
        if let Some(path) = file_exists(Path::new(path)) {
            return Some(ShellInfo { kind, path });
        }
    }
    None
}

fn resolve_explicit_shell(shell: &str) -> Result<ShellInfo, String> {
    let path = PathBuf::from(shell.trim());
    let Some(kind) = detect_shell_kind(&path) else {
        return Err(format!(
            "Unsupported shell `{shell}`. Supported shells are zsh, bash, sh, pwsh, powershell, and cmd."
        ));
    };
    get_shell(kind, Some(&path)).ok_or_else(|| {
        format!(
            "Explicit shell `{shell}` is supported by name, but Fennara could not find an executable for it."
        )
    })
}

fn is_path_like(path: &Path) -> bool {
    let raw = path.to_string_lossy();
    path.is_absolute() || raw.contains('/') || raw.contains('\\') || raw.contains(':')
}

fn file_exists(path: &Path) -> Option<PathBuf> {
    fs::metadata(path)
        .ok()
        .filter(|metadata| metadata.is_file())
        .map(|_| path.to_path_buf())
}

fn find_on_path(binary_name: &str) -> Option<PathBuf> {
    let name_path = Path::new(binary_name);
    if is_path_like(name_path) {
        return file_exists(name_path);
    }
    let path_var = env::var_os("PATH")?;
    for dir in env::split_paths(&path_var) {
        for candidate in path_candidates(&dir, binary_name) {
            if let Some(path) = file_exists(&candidate) {
                return Some(path);
            }
        }
    }
    None
}

fn path_candidates(dir: &Path, binary_name: &str) -> Vec<PathBuf> {
    let base = dir.join(binary_name);
    if !cfg!(windows) || binary_name.to_ascii_lowercase().ends_with(".exe") {
        return vec![base];
    }
    vec![base.clone(), base.with_extension("exe")]
}

fn derive_argv(shell: &ShellInfo, command: &str) -> Vec<String> {
    let shell_path = shell.path.to_string_lossy().to_string();
    match shell.kind {
        ShellKind::Zsh | ShellKind::Bash | ShellKind::Sh => {
            vec![shell_path, "-c".to_string(), command.to_string()]
        }
        ShellKind::PowerShell => vec![
            shell_path,
            "-NoProfile".to_string(),
            "-Command".to_string(),
            prefix_powershell_command(command),
        ],
        ShellKind::Cmd => vec![shell_path, "/c".to_string(), command.to_string()],
    }
}

fn prefix_powershell_command(command: &str) -> String {
    let trimmed = command.trim_start();
    if trimmed.starts_with(POWERSHELL_UTF8_PREFIX) {
        command.to_string()
    } else {
        format!("{POWERSHELL_UTF8_PREFIX}{command}")
    }
}

#[cfg(test)]
async fn execute_request(
    project_root: Option<&str>,
    request: ExecCommandRequest,
    max_output_bytes: usize,
) -> Result<(ResolvedRequest, ProcessOutput), ExecCommandError> {
    execute_request_with_cancellation(
        project_root,
        request,
        max_output_bytes,
        CancellationCheck::none(),
    )
    .await
}

async fn execute_request_with_cancellation(
    project_root: Option<&str>,
    request: ExecCommandRequest,
    max_output_bytes: usize,
    cancellation: CancellationCheck,
) -> Result<(ResolvedRequest, ProcessOutput), ExecCommandError> {
    let resolved = resolve_request(project_root, request)?;
    let command = resolved.argv.clone();
    let cwd = resolved.cwd.clone();
    let timeout = resolved.timeout;
    let output = tokio::task::spawn_blocking(move || {
        run_process_blocking(&command, &cwd, timeout, max_output_bytes, cancellation)
    })
    .await
    .map_err(|error| ExecCommandError::new("spawn_failed", format!("exec worker failed: {error}")))?
    .map_err(|error| ExecCommandError::new("spawn_failed", error))?;
    Ok((resolved, output))
}

fn resolve_request(
    project_root: Option<&str>,
    request: ExecCommandRequest,
) -> Result<ResolvedRequest, ExecCommandError> {
    let command = request.command.trim();
    if command.is_empty() {
        return Err(ExecCommandError::new(
            "validation_failed",
            "command must not be empty.",
        ));
    }
    if request.justification.as_deref().is_some_and(str::is_empty) {
        return Err(ExecCommandError::new(
            "validation_failed",
            "justification must not be empty when provided.",
        ));
    }
    let cwd = resolve_cwd(project_root, request.cwd.as_deref())?;
    let timeout_ms = request.timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
    if timeout_ms > MAX_TIMEOUT_MS {
        return Err(ExecCommandError::new(
            "validation_failed",
            format!("timeout_ms must be <= {MAX_TIMEOUT_MS}."),
        ));
    }
    let shell = match request
        .shell
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(shell) => resolve_explicit_shell(shell)
            .map_err(|message| ExecCommandError::new("validation_failed", message))?,
        None => default_shell(),
    };
    let argv = derive_argv(&shell, command);
    Ok(ResolvedRequest {
        command: command.to_string(),
        cwd,
        timeout: Duration::from_millis(timeout_ms),
        shell,
        argv,
    })
}

fn resolve_cwd(project_root: Option<&str>, cwd: Option<&str>) -> Result<PathBuf, ExecCommandError> {
    let project_root = project_root
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ExecCommandError::new(
                "validation_failed",
                "exec_command requires an active Godot project root.",
            )
        })?;
    let project_root = canonical_dir(Path::new(project_root), "project root")?;
    let raw_cwd = cwd.map(str::trim).filter(|value| !value.is_empty());
    if raw_cwd.is_some_and(|value| value.starts_with("res://") || value.starts_with("user://")) {
        return Err(ExecCommandError::new(
            "validation_failed",
            "exec_command cwd must be a real filesystem path, not a Godot res:// or user:// path.",
        ));
    }
    let candidate = match raw_cwd {
        Some(value) => {
            let path = Path::new(value);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                project_root.join(path)
            }
        }
        None => project_root.clone(),
    };
    let cwd = canonical_dir(&candidate, "cwd")?;
    if !cwd.starts_with(&project_root) {
        return Err(ExecCommandError::new(
            "validation_failed",
            format!(
                "exec_command cwd `{}` is outside the active project root `{}`. Phase one only allows project-root cwd values.",
                cwd.display(),
                project_root.display()
            ),
        ));
    }
    Ok(cwd)
}

fn canonical_dir(path: &Path, label: &str) -> Result<PathBuf, ExecCommandError> {
    let canonical = fs::canonicalize(path).map_err(|error| {
        ExecCommandError::new(
            "validation_failed",
            format!("{label} `{}` is not usable: {error}", path.display()),
        )
    })?;
    if !canonical.is_dir() {
        return Err(ExecCommandError::new(
            "validation_failed",
            format!("{label} `{}` is not a directory.", canonical.display()),
        ));
    }
    Ok(canonical)
}

fn run_process_blocking(
    argv: &[String],
    cwd: &Path,
    timeout: Duration,
    max_output_bytes: usize,
    cancellation: CancellationCheck,
) -> Result<ProcessOutput, String> {
    let (program, args) = argv
        .split_first()
        .ok_or_else(|| "command argv is empty".to_string())?;
    let start = Instant::now();
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    configure_process_group(&mut command);

    let mut child = command
        .spawn()
        .map_err(|error| format!("Failed to spawn `{program}`: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "stdout pipe was unexpectedly unavailable".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "stderr pipe was unexpectedly unavailable".to_string())?;
    let stdout_rx = spawn_reader(stdout, max_output_bytes);
    let stderr_rx = spawn_reader(stderr, max_output_bytes);

    let mut timed_out = false;
    let mut cancelled = false;
    let exit_status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {}
            Err(error) => return Err(format!("Failed while waiting for process: {error}")),
        }
        if cancellation.is_cancelled() {
            cancelled = true;
            kill_process_tree(&mut child);
            break wait_after_kill(&mut child)
                .map_err(|error| format!("Failed to wait after cancellation: {error}"))?;
        }
        if start.elapsed() >= timeout {
            timed_out = true;
            kill_process_tree(&mut child);
            break wait_after_kill(&mut child)
                .map_err(|error| format!("Failed to wait after timeout: {error}"))?;
        }
        thread::sleep(Duration::from_millis(20));
    };

    let stdout = receive_reader_output(stdout_rx, "stdout")?;
    let stderr = receive_reader_output(stderr_rx, "stderr")?;
    let status = if cancelled {
        ExecStatus::Cancelled
    } else if timed_out {
        ExecStatus::TimedOut
    } else {
        ExecStatus::Completed
    };
    Ok(ProcessOutput {
        status,
        exit_code: exit_status.and_then(exit_code),
        stdout,
        stderr,
        duration: start.elapsed(),
        timed_out,
    })
}

fn wait_after_kill(child: &mut std::process::Child) -> io::Result<Option<ExitStatus>> {
    let started = Instant::now();
    let grace = Duration::from_millis(WAIT_AFTER_KILL_MS);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(Some(status));
        }
        if started.elapsed() >= grace {
            return Ok(None);
        }
        thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
fn configure_process_group(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    unsafe {
        command.pre_exec(|| {
            if libc::setpgid(0, 0) == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        });
    }
}

#[cfg(not(unix))]
fn configure_process_group(_command: &mut Command) {}

fn kill_process_tree(child: &mut std::process::Child) {
    kill_process_tree_by_pid(child.id());
    let _ = child.kill();
}

#[cfg(windows)]
fn kill_process_tree_by_pid(pid: u32) {
    let _ = Command::new("taskkill")
        .args(["/pid", &pid.to_string(), "/f", "/t"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

#[cfg(unix)]
fn kill_process_tree_by_pid(pid: u32) {
    unsafe {
        let _ = libc::kill(-(pid as i32), libc::SIGKILL);
    }
}

#[cfg(not(any(unix, windows)))]
fn kill_process_tree_by_pid(_pid: u32) {}

fn exit_code(status: ExitStatus) -> Option<i32> {
    status.code()
}

fn spawn_reader(
    reader: impl Read + Send + 'static,
    max_bytes: usize,
) -> mpsc::Receiver<io::Result<CapturedOutput>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(read_capped(reader, max_bytes));
    });
    rx
}

fn receive_reader_output(
    rx: mpsc::Receiver<io::Result<CapturedOutput>>,
    stream_name: &str,
) -> Result<CapturedOutput, String> {
    match rx.recv_timeout(Duration::from_millis(OUTPUT_DRAIN_TIMEOUT_MS)) {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(error)) => Err(format!("Failed to read {stream_name}: {error}")),
        Err(mpsc::RecvTimeoutError::Timeout) => Ok(CapturedOutput {
            text: format!(
                "\n...[{stream_name} drain timed out after process termination; output may be incomplete]..."
            ),
            truncated: true,
        }),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(format!(
            "{stream_name} reader exited before returning captured output"
        )),
    }
}

fn read_capped(mut reader: impl Read, max_bytes: usize) -> io::Result<CapturedOutput> {
    let mut buffer = Vec::with_capacity(max_bytes.min(READ_CHUNK_SIZE));
    let mut scratch = [0u8; READ_CHUNK_SIZE];
    let mut truncated = false;
    loop {
        let read = reader.read(&mut scratch)?;
        if read == 0 {
            break;
        }
        let remaining = max_output_remaining(max_bytes, buffer.len());
        if remaining < read {
            truncated = true;
        }
        let take = remaining.min(read);
        if take > 0 {
            buffer.extend_from_slice(&scratch[..take]);
        }
    }
    let mut text = String::from_utf8_lossy(&buffer).into_owned();
    if truncated {
        text.push_str(&format!(
            "\n...[truncated after {max_bytes} bytes retained by Fennara]..."
        ));
    }
    Ok(CapturedOutput { text, truncated })
}

fn max_output_remaining(max_bytes: usize, current_len: usize) -> usize {
    max_bytes.saturating_sub(current_len)
}

#[derive(Debug)]
struct ExecCommandError {
    status: &'static str,
    message: String,
}

impl ExecCommandError {
    fn new(status: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

fn completed_exec_tool(resolved: ResolvedRequest, output: ProcessOutput) -> ExecutedTool {
    let status = output.status.as_str();
    let duration_ms = output.duration.as_millis() as u64;
    let raw_result = json!({
        "success": status == "completed",
        "status": status,
        "command": resolved.command,
        "cwd": resolved.cwd.to_string_lossy(),
        "shell": {
            "kind": resolved.shell.name(),
            "path": resolved.shell.path.to_string_lossy(),
            "argv": resolved.argv,
        },
        "exit_code": output.exit_code,
        "stdout": output.stdout.text,
        "stderr": output.stderr.text,
        "duration_ms": duration_ms,
        "timed_out": output.timed_out,
        "truncated": {
            "stdout": output.stdout.truncated,
            "stderr": output.stderr.truncated
        },
        "phase_one_limits": {
            "pty": false,
            "background_session": false,
            "write_stdin": false,
            "custom_env": false,
            "os_sandbox": false,
            "cwd_policy": "project_cwd_restricted"
        }
    });
    let markdown = markdown_for_exec_result(&raw_result);
    ExecutedTool {
        ok: status == "completed",
        raw_result,
        mcp_markdown: markdown.clone(),
        plugin_markdown: markdown,
        metadata: json!({
            "tool_name": "exec_command",
            "status": status,
            "format": "markdown",
            "targets": [{
                "command": resolved.command,
                "cwd": resolved.cwd.to_string_lossy(),
                "shell": resolved.shell.name()
            }]
        }),
        target_keys: vec![resolved.cwd.to_string_lossy().into_owned()],
        model_followup_messages: Vec::new(),
    }
}

fn failed_exec_tool(status: &'static str, error: String) -> ExecutedTool {
    let markdown = format!("Tool: exec_command\nStatus: {status}\nError: {error}");
    ExecutedTool {
        ok: false,
        raw_result: json!({
            "success": false,
            "status": status,
            "error": error,
        }),
        mcp_markdown: markdown.clone(),
        plugin_markdown: markdown,
        metadata: json!({
            "tool_name": "exec_command",
            "status": status,
            "format": "markdown",
        }),
        target_keys: Vec::new(),
        model_followup_messages: Vec::new(),
    }
}

fn markdown_for_exec_result(result: &Value) -> String {
    let status = result
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("completed");
    let command = result.get("command").and_then(Value::as_str).unwrap_or("");
    let cwd = result.get("cwd").and_then(Value::as_str).unwrap_or("");
    let shell = result
        .get("shell")
        .and_then(|shell| shell.get("kind"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let shell_path = result
        .get("shell")
        .and_then(|shell| shell.get("path"))
        .and_then(Value::as_str)
        .unwrap_or("");
    let exit_code = result
        .get("exit_code")
        .map(|value| value.to_string())
        .unwrap_or_else(|| "null".to_string());
    let duration_ms = result
        .get("duration_ms")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let timed_out = result
        .get("timed_out")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let stdout = result.get("stdout").and_then(Value::as_str).unwrap_or("");
    let stderr = result.get("stderr").and_then(Value::as_str).unwrap_or("");
    format!(
        "Tool: exec_command\nStatus: {status}\nCommand: {command}\nCwd: {cwd}\nShell: {shell} ({shell_path})\nExit code: {exit_code}\nDuration: {duration_ms} ms\nTimed out: {timed_out}\n\nStdout:\n```text\n{stdout}\n```\n\nStderr:\n```text\n{stderr}\n```"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn detects_supported_shells_by_name_and_path() {
        assert_eq!(detect_shell_kind("zsh"), Some(ShellKind::Zsh));
        assert_eq!(detect_shell_kind("bash"), Some(ShellKind::Bash));
        assert_eq!(detect_shell_kind("sh"), Some(ShellKind::Sh));
        assert_eq!(detect_shell_kind("pwsh"), Some(ShellKind::PowerShell));
        assert_eq!(
            detect_shell_kind("powershell.exe"),
            Some(ShellKind::PowerShell)
        );
        assert_eq!(
            detect_shell_kind(r"C:\Program Files\PowerShell\7\pwsh.exe"),
            Some(ShellKind::PowerShell)
        );
        assert_eq!(
            detect_shell_kind(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe"),
            Some(ShellKind::PowerShell)
        );
        assert_eq!(detect_shell_kind("cmd.exe"), Some(ShellKind::Cmd));
        assert_eq!(
            detect_shell_kind("C:/Windows/System32/cmd.exe"),
            Some(ShellKind::Cmd)
        );
        assert_eq!(detect_shell_kind("/bin/bash"), Some(ShellKind::Bash));
        assert_eq!(detect_shell_kind("/usr/bin/bash"), Some(ShellKind::Bash));
        assert_eq!(detect_shell_kind("/bin/sh"), Some(ShellKind::Sh));
        assert_eq!(
            detect_shell_kind("/usr/local/bin/pwsh"),
            Some(ShellKind::PowerShell)
        );
        assert_eq!(detect_shell_kind("fish"), None);
    }

    #[test]
    fn builds_shell_argv() {
        let bash = ShellInfo {
            kind: ShellKind::Bash,
            path: PathBuf::from("/bin/bash"),
        };
        assert_eq!(
            derive_argv(&bash, "echo ok"),
            ["/bin/bash", "-c", "echo ok"]
        );

        let zsh = ShellInfo {
            kind: ShellKind::Zsh,
            path: PathBuf::from("/bin/zsh"),
        };
        assert_eq!(derive_argv(&zsh, "echo ok"), ["/bin/zsh", "-c", "echo ok"]);

        let sh = ShellInfo {
            kind: ShellKind::Sh,
            path: PathBuf::from("/bin/sh"),
        };
        assert_eq!(derive_argv(&sh, "echo ok"), ["/bin/sh", "-c", "echo ok"]);

        let powershell = ShellInfo {
            kind: ShellKind::PowerShell,
            path: PathBuf::from("powershell.exe"),
        };
        assert_eq!(
            derive_argv(&powershell, "Write-Output ok"),
            [
                "powershell.exe",
                "-NoProfile",
                "-Command",
                &format!("{POWERSHELL_UTF8_PREFIX}Write-Output ok")
            ]
        );

        let cmd = ShellInfo {
            kind: ShellKind::Cmd,
            path: PathBuf::from("cmd.exe"),
        };
        assert_eq!(derive_argv(&cmd, "echo ok"), ["cmd.exe", "/c", "echo ok"]);
    }

    #[test]
    fn does_not_duplicate_powershell_utf8_prefix() {
        let command = format!("{POWERSHELL_UTF8_PREFIX}Write-Output ok");
        assert_eq!(prefix_powershell_command(&command), command);
    }

    #[test]
    fn rejects_unknown_explicit_shell() {
        let err = resolve_explicit_shell("fish").unwrap_err();
        assert!(err.contains("Unsupported shell"));
    }

    #[test]
    fn resolves_cwd_under_project_root() {
        let root = test_dir("cwd");
        fs::create_dir_all(root.join("sub")).unwrap();

        assert_eq!(
            resolve_cwd(Some(root.to_str().unwrap()), None).unwrap(),
            canonical(&root)
        );
        assert_eq!(
            resolve_cwd(Some(root.to_str().unwrap()), Some("sub")).unwrap(),
            canonical(&root.join("sub"))
        );
        assert_eq!(
            resolve_cwd(
                Some(root.to_str().unwrap()),
                Some(root.join("sub").to_str().unwrap())
            )
            .unwrap(),
            canonical(&root.join("sub"))
        );
    }

    #[test]
    fn rejects_outside_and_missing_cwd() {
        let root = test_dir("cwd-reject-root");
        let outside = test_dir("cwd-reject-outside");

        let outside_err = resolve_cwd(
            Some(root.to_str().unwrap()),
            Some(outside.to_str().unwrap()),
        )
        .unwrap_err();
        assert_eq!(outside_err.status, "validation_failed");
        assert!(outside_err.message.contains("outside"));

        let missing_err = resolve_cwd(Some(root.to_str().unwrap()), Some("missing")).unwrap_err();
        assert_eq!(missing_err.status, "validation_failed");
        assert!(missing_err.message.contains("not usable"));
    }

    #[tokio::test]
    async fn executes_echo_command() {
        let root = test_dir("echo");
        let request = ExecCommandRequest {
            command: "echo ok".to_string(),
            cwd: None,
            timeout_ms: Some(10_000),
            shell: None,
            justification: None,
        };

        let (_resolved, output) =
            execute_request(Some(root.to_str().unwrap()), request, OUTPUT_MAX_BYTES)
                .await
                .unwrap();

        assert_eq!(output.status, ExecStatus::Completed);
        assert!(output.stdout.text.contains("ok"));
    }

    #[tokio::test]
    async fn nonzero_exit_is_completed() {
        let root = test_dir("nonzero");
        let request = ExecCommandRequest {
            command: "exit 7".to_string(),
            cwd: None,
            timeout_ms: Some(10_000),
            shell: None,
            justification: None,
        };

        let (_resolved, output) =
            execute_request(Some(root.to_str().unwrap()), request, OUTPUT_MAX_BYTES)
                .await
                .unwrap();

        assert_eq!(output.status, ExecStatus::Completed);
        assert_eq!(output.exit_code, Some(7));
    }

    #[tokio::test]
    async fn timeout_returns_timed_out() {
        let root = test_dir("timeout");
        let command = if cfg!(windows) {
            "ping 127.0.0.1 -n 6 > nul"
        } else {
            "sleep 5"
        };
        let request = ExecCommandRequest {
            command: command.to_string(),
            cwd: None,
            timeout_ms: Some(100),
            shell: None,
            justification: None,
        };

        let (_resolved, output) =
            execute_request(Some(root.to_str().unwrap()), request, OUTPUT_MAX_BYTES)
                .await
                .unwrap();

        assert_eq!(output.status, ExecStatus::TimedOut);
        assert!(output.timed_out);
    }

    #[tokio::test]
    async fn cancellation_returns_cancelled() {
        let root = test_dir("cancelled");
        let chat_id = "chat-cancelled";
        let (shutdown_tx, _shutdown_rx) = tokio::sync::oneshot::channel();
        let state = AppState::new(shutdown_tx);
        let cancellation = CancellationCheck::for_chat(&state, chat_id);
        let cancelled_chats = state.cancelled_chats.clone();
        let chat_id_owned = chat_id.to_string();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(100)).await;
            cancelled_chats.write().await.insert(chat_id_owned);
        });
        let command = if cfg!(windows) {
            "ping 127.0.0.1 -n 6 > nul"
        } else {
            "sleep 5"
        };
        let request = ExecCommandRequest {
            command: command.to_string(),
            cwd: None,
            timeout_ms: Some(10_000),
            shell: None,
            justification: None,
        };

        let (_resolved, output) = execute_request_with_cancellation(
            Some(root.to_str().unwrap()),
            request,
            OUTPUT_MAX_BYTES,
            cancellation,
        )
        .await
        .unwrap();

        assert_eq!(output.status, ExecStatus::Cancelled);
        assert!(!output.timed_out);
    }

    #[test]
    fn truncates_large_output() {
        let data = vec![b'x'; 20];
        let output = read_capped(&data[..], 8).unwrap();

        assert!(output.truncated);
        assert!(output.text.starts_with("xxxxxxxx"));
        assert!(output.text.contains("truncated"));
    }

    fn test_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("fennara-exec-command-{name}-{unique}"));
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn canonical(path: &Path) -> PathBuf {
        fs::canonicalize(path).unwrap()
    }
}
