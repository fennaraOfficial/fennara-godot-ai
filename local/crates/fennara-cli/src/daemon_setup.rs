use crate::app_layout::{AppLayout, binary_name, display_path};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const DAEMON_ADDR: &str = "127.0.0.1:41287";
const HEALTH_TIMEOUT: Duration = Duration::from_millis(500);
const START_TIMEOUT: Duration = Duration::from_secs(10);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

#[derive(Debug, PartialEq, Eq)]
pub enum HealthErrorKind {
    NotRunning,
    Other,
}

#[derive(Debug)]
pub struct HealthError {
    pub kind: HealthErrorKind,
    pub message: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReadyState {
    AlreadyRunning,
    Started,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Compatibility {
    NotRunning,
    Compatible,
}

pub fn ensure_running(layout: &AppLayout, expected_version: &str) -> Result<ReadyState, String> {
    if check_compatibility(expected_version)? == Compatibility::Compatible {
        return Ok(ReadyState::AlreadyRunning);
    }

    let launcher = layout.bin_dir.join(binary_name("fennara-daemon"));
    if !launcher.is_file() {
        return Err(format!(
            "the installed daemon launcher is missing at {}",
            display_path(&launcher)
        ));
    }
    let log_path = layout.logs_dir.join("daemon-startup.log");
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .map_err(|error| {
            format!(
                "failed to open daemon startup log {}: {error}",
                display_path(&log_path)
            )
        })?;
    let error_log = log_file.try_clone().map_err(|error| {
        format!(
            "failed to prepare daemon startup log {}: {error}",
            display_path(&log_path)
        )
    })?;
    let status = Command::new(&launcher)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(error_log))
        .status()
        .map_err(|error| {
            format!(
                "failed to start the Fennara daemon through {}: {error}",
                display_path(&launcher)
            )
        })?;
    if !status.success() {
        return Err(format!(
            "the Fennara daemon launcher {} exited with {status}",
            display_path(&launcher)
        ));
    }

    let deadline = Instant::now() + START_TIMEOUT;
    loop {
        match health() {
            Ok(value) => {
                ensure_version(&value, expected_version)?;
                return Ok(ReadyState::Started);
            }
            Err(error) if error.kind == HealthErrorKind::NotRunning => {}
            Err(error) if Instant::now() < deadline => {
                let _ = error;
            }
            Err(error) => {
                return Err(format!(
                    "the Fennara daemon did not become healthy: {}",
                    error.message
                ));
            }
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "the Fennara daemon did not become healthy within {} seconds",
                START_TIMEOUT.as_secs()
            ));
        }
        thread::sleep(POLL_INTERVAL);
    }
}

pub fn check_compatibility(expected_version: &str) -> Result<Compatibility, String> {
    match health() {
        Ok(value) => {
            ensure_version(&value, expected_version)?;
            Ok(Compatibility::Compatible)
        }
        Err(error) if error.kind == HealthErrorKind::NotRunning => Ok(Compatibility::NotRunning),
        Err(error) => Err(format!(
            "could not inspect the Fennara daemon: {}",
            error.message
        )),
    }
}

pub fn shutdown_if_running(layout: &AppLayout) -> Result<(), String> {
    if matches!(
        health(),
        Err(HealthError {
            kind: HealthErrorKind::NotRunning,
            ..
        })
    ) {
        return Ok(());
    }
    let token_path = layout.app_dir.join("daemon-control-token");
    let token = std::fs::read_to_string(&token_path)
        .map_err(|error| format!("failed to read {}: {error}", display_path(&token_path)))?
        .trim()
        .to_string();
    if token.is_empty() || token.contains(['\r', '\n']) {
        return Err("the local daemon control token is invalid".to_string());
    }
    let mut stream = TcpStream::connect(DAEMON_ADDR)
        .map_err(|error| format!("failed to connect to the running daemon: {error}"))?;
    stream
        .set_read_timeout(Some(HEALTH_TIMEOUT))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(HEALTH_TIMEOUT))
        .map_err(|error| error.to_string())?;
    let request = format!(
        "POST /shutdown HTTP/1.1\r\nHost: 127.0.0.1\r\nX-Fennara-Control-Token: {token}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("failed to request daemon shutdown: {error}"))?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|error| format!("failed to read daemon shutdown response: {error}"))?;
    if !response.starts_with("HTTP/1.1 200") && !response.starts_with("HTTP/1.0 200") {
        if response.starts_with("HTTP/1.1 409") || response.starts_with("HTTP/1.0 409") {
            return Err(
                "another Godot project is still connected to Fennara; close every other Fennara-enabled editor before switching the active version"
                    .to_string(),
            );
        }
        return Err("daemon rejected the authenticated shutdown request".to_string());
    }
    let deadline = Instant::now() + START_TIMEOUT;
    while Instant::now() < deadline {
        if matches!(
            health(),
            Err(HealthError {
                kind: HealthErrorKind::NotRunning,
                ..
            })
        ) {
            return Ok(());
        }
        thread::sleep(POLL_INTERVAL);
    }
    Err("the previous Fennara daemon did not stop before update activation".to_string())
}

pub fn ensure_switch_available(
    layout: &AppLayout,
    allowed_project: Option<&Path>,
) -> Result<(), String> {
    if matches!(
        health(),
        Err(HealthError {
            kind: HealthErrorKind::NotRunning,
            ..
        })
    ) {
        return Ok(());
    }
    let token_path = layout.app_dir.join("daemon-control-token");
    let token = std::fs::read_to_string(&token_path)
        .map_err(|error| format!("failed to read {}: {error}", display_path(&token_path)))?
        .trim()
        .to_string();
    if token.is_empty() || token.contains(['\r', '\n']) {
        return Err("the local daemon control token is invalid".to_string());
    }
    let mut stream = TcpStream::connect(DAEMON_ADDR)
        .map_err(|error| format!("failed to connect to the running daemon: {error}"))?;
    stream
        .set_read_timeout(Some(HEALTH_TIMEOUT))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(HEALTH_TIMEOUT))
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET /status HTTP/1.1\r\nHost: 127.0.0.1\r\nX-Fennara-Control-Token: {token}\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("failed to request daemon status: {error}"))?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|error| format!("failed to read daemon status: {error}"))?;
    let status = parse_success_response(&response)?;
    let conflicts = conflicting_project_count(&status, allowed_project)?;
    if conflicts == 0 {
        Ok(())
    } else {
        Err(format!(
            "{conflicts} other Fennara-enabled Godot project{} still connected; close every other editor before switching the active version",
            if conflicts == 1 { " is" } else { "s are" }
        ))
    }
}

pub fn health() -> Result<Value, HealthError> {
    let mut stream = TcpStream::connect(DAEMON_ADDR).map_err(|error| HealthError {
        kind: if matches!(
            error.kind(),
            std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::TimedOut
        ) {
            HealthErrorKind::NotRunning
        } else {
            HealthErrorKind::Other
        },
        message: error.to_string(),
    })?;
    stream
        .set_read_timeout(Some(HEALTH_TIMEOUT))
        .map_err(other_health_error)?;
    stream
        .set_write_timeout(Some(HEALTH_TIMEOUT))
        .map_err(other_health_error)?;
    stream
        .write_all(b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .map_err(other_health_error)?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(other_health_error)?;
    parse_health_response(&response)
}

fn parse_health_response(response: &str) -> Result<Value, HealthError> {
    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| other_health_message("invalid daemon HTTP response"))?;
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        return Err(other_health_message("daemon returned non-200 status"));
    }
    serde_json::from_str(body).map_err(|error| other_health_message(error.to_string()))
}

fn parse_success_response(response: &str) -> Result<Value, String> {
    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid daemon HTTP response".to_string())?;
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        return Err("daemon returned non-200 status".to_string());
    }
    serde_json::from_str(body).map_err(|error| format!("invalid daemon status JSON: {error}"))
}

fn conflicting_project_count(
    status: &Value,
    allowed_project: Option<&Path>,
) -> Result<usize, String> {
    let allowed = allowed_project.map(canonical_or_original);
    let projects = status
        .get("connected_projects")
        .and_then(Value::as_array)
        .ok_or_else(|| "daemon status is missing connected_projects".to_string())?;
    Ok(projects
        .iter()
        .filter(|project| {
            let Some(path) = project.get("project_path").and_then(Value::as_str) else {
                return true;
            };
            allowed
                .as_ref()
                .is_none_or(|allowed| canonical_or_original(Path::new(path)) != *allowed)
        })
        .count())
}

fn canonical_or_original(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn ensure_version(health: &Value, expected_version: &str) -> Result<(), String> {
    let running_version = health
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| "the Fennara daemon health response is missing its version".to_string())?;
    if running_version == expected_version {
        Ok(())
    } else {
        Err(format!(
            "the running Fennara daemon is version {running_version}, but the project addon requires {expected_version}; close Godot and stop the older daemon before retrying"
        ))
    }
}

fn other_health_error(error: std::io::Error) -> HealthError {
    other_health_message(error.to_string())
}

fn other_health_message(message: impl Into<String>) -> HealthError {
    HealthError {
        kind: HealthErrorKind::Other,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_matching_daemon_version() {
        ensure_version(&json!({ "version": "1.2.3" }), "1.2.3").unwrap();
    }

    #[test]
    fn rejects_mismatched_daemon_version() {
        let error = ensure_version(&json!({ "version": "1.2.2" }), "1.2.3").unwrap_err();
        assert!(error.contains("version 1.2.2"));
        assert!(error.contains("requires 1.2.3"));
    }

    #[test]
    fn parses_daemon_health_response() {
        let health = parse_health_response(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"ok\":true,\"version\":\"1.2.3\"}",
        )
        .unwrap();
        assert_eq!(health["version"], "1.2.3");
    }

    #[test]
    fn rejects_non_success_health_response() {
        let error = parse_health_response("HTTP/1.1 503 Unavailable\r\n\r\n{}").unwrap_err();
        assert!(error.message.contains("non-200"));
    }

    #[test]
    fn connected_project_preflight_allows_only_the_selected_project() {
        let current = std::env::current_dir().unwrap();
        let status = json!({
            "connected_projects": [
                { "project_path": current },
                { "project_path": current.join("other") }
            ]
        });
        assert_eq!(
            conflicting_project_count(&status, Some(&current)).unwrap(),
            1
        );
        assert_eq!(conflicting_project_count(&status, None).unwrap(), 2);
    }
}
