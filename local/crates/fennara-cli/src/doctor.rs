use crate::VERSION;
use crate::app_layout::{
    AppLayout, arch_name, binary_name, display_path, platform_name, read_current_manifest,
    resolve_manifest_path,
};
use crate::webview_prereq;
use serde_json::Value;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::time::Duration;
use sysinfo::System;

const DAEMON_ADDR: &str = "127.0.0.1:41287";
const DAEMON_HEALTH_TIMEOUT: Duration = Duration::from_millis(500);

pub fn run(args: Vec<&str>) -> Result<(), String> {
    let repair = args.contains(&"--repair");
    for arg in args {
        if arg != "--repair" {
            return Err(format!("unknown doctor option: {arg}"));
        }
    }

    let layout = AppLayout::detect()?;
    println!("Fennara doctor");
    println!("version: {VERSION}");
    println!("platform: {} {}", platform_name(), arch_name());
    println!("app_dir: {}", display_path(&layout.app_dir));

    if repair {
        layout.ensure_base_dirs()?;
    }

    report_dir("bin", &layout.bin_dir);
    report_dir("versions", &layout.versions_dir);
    report_dir("cache", &layout.cache_dir);
    report_dir("logs", &layout.logs_dir);
    report_dir("tools", &layout.tools_dir);
    report_dir("webview", &layout.webview_dir);
    report_file("current manifest", &layout.current_manifest_path);
    report_file(
        "fennara-mcp shim",
        &layout.bin_dir.join(binary_name("fennara-mcp")),
    );
    report_file(
        "fennara-daemon shim",
        &layout.bin_dir.join(binary_name("fennara-daemon")),
    );

    let current_manifest = read_current_manifest(&layout.current_manifest_path);
    match &current_manifest {
        Ok(Some(manifest)) => report_manifest(&layout.app_dir, manifest),
        Ok(None) => println!("current version: not installed yet"),
        Err(error) => println!("current manifest: invalid ({error})"),
    }
    if let Ok(Some(manifest)) = &current_manifest {
        report_running_state(&layout.app_dir, manifest);
    }

    println!(
        "release local asset hint: fennara-release-local-{}-{}-v{VERSION}.zip",
        platform_name(),
        arch_name()
    );
    println!(
        "cli asset hint: fennara-cli-{}-{}-v{VERSION}.zip",
        platform_name(),
        arch_name()
    );
    println!("release addon asset hint: fennara-release-addon-v{VERSION}.zip");
    webview_prereq::report_for_doctor(&layout, repair)?;

    if repair {
        println!("repair: base directories ensured");
    } else {
        println!("repair: not run; use `fennara doctor --repair` to create base directories");
    }

    Ok(())
}

fn report_manifest(app_dir: &Path, manifest: &Value) {
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    println!("current version: {version}");

    for field in ["mcp_runtime", "daemon_runtime", "cli_runtime", "addon"] {
        if let Some(path) = manifest.get(field).and_then(Value::as_str) {
            let resolved = resolve_manifest_path(app_dir, path);
            report_file(field, &resolved);
        }
    }
}

fn report_running_state(app_dir: &Path, manifest: &Value) {
    let expected_version = manifest
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or("unknown");

    report_running_daemon_version(expected_version);
    report_running_runtime_processes(app_dir, manifest);
}

fn report_running_daemon_version(expected_version: &str) {
    match daemon_health() {
        Ok(health) => {
            let running_version = health
                .get("version")
                .and_then(Value::as_str)
                .unwrap_or("unknown");
            if running_version == expected_version {
                println!("running daemon: version {running_version} (ok)");
            } else {
                println!(
                    "warning: running daemon is version {running_version}, but current.json points at {expected_version}"
                );
                println!("warning: restart Godot or the Fennara daemon to use the current runtime");
            }
        }
        Err(error) if error.kind == DaemonHealthErrorKind::NotRunning => {
            println!("running daemon: not detected");
        }
        Err(error) => {
            println!("running daemon: could not check ({})", error.message);
        }
    }
}

fn report_running_runtime_processes(app_dir: &Path, manifest: &Value) {
    let specs = [
        RuntimeSpec {
            label: "MCP runtime",
            process_name: binary_name("fennara-mcp-runtime"),
            manifest_field: "mcp_runtime",
            restart_hint: "restart the MCP client so it launches the current Fennara runtime",
        },
        RuntimeSpec {
            label: "daemon runtime",
            process_name: binary_name("fennara-daemon-runtime"),
            manifest_field: "daemon_runtime",
            restart_hint: "restart Godot or the Fennara daemon so it uses the current runtime",
        },
    ];

    let mut system = System::new_all();
    system.refresh_processes();

    for spec in specs {
        let expected = manifest
            .get(spec.manifest_field)
            .and_then(Value::as_str)
            .map(|path| resolve_manifest_path(app_dir, path));
        report_runtime_process(&system, &spec, expected.as_deref());
    }
}

fn report_runtime_process(system: &System, spec: &RuntimeSpec, expected: Option<&Path>) {
    let expected_for_compare = expected.and_then(canonicalize_for_compare);
    let mut found = false;
    let mut current_count = 0usize;
    let mut first_current_path = None;

    for process in system.processes().values() {
        let exe = process.exe();
        let exe_for_compare = exe.and_then(canonicalize_for_compare);
        let exe_display = exe
            .map(display_path)
            .unwrap_or_else(|| "unknown".to_string());
        let path_matches_expected = expected_for_compare
            .as_ref()
            .zip(exe_for_compare.as_ref())
            .is_some_and(|(expected, exe)| expected == exe);
        let exe_name_matches = exe
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == spec.process_name);
        let process_name_matches = process.name() == spec.process_name;

        if !process_name_matches && !exe_name_matches && !path_matches_expected {
            continue;
        }

        found = true;

        if path_matches_expected {
            current_count += 1;
            first_current_path.get_or_insert(exe_display);
        } else if let Some(expected) = expected {
            println!(
                "warning: running {} may be stale: pid {} at {}",
                spec.label,
                process.pid(),
                exe_display
            );
            println!("warning: expected {}", display_path(expected));
            println!("warning: {}", spec.restart_hint);
        } else {
            println!(
                "warning: running {} detected but {} is missing from current.json: pid {} at {}",
                spec.label,
                spec.manifest_field,
                process.pid(),
                exe_display
            );
        }
    }

    if current_count == 1 {
        println!(
            "running {}: 1 current process at {} (ok)",
            spec.label,
            first_current_path.unwrap_or_else(|| "unknown".to_string())
        );
    } else if current_count > 1 {
        println!(
            "running {}: {current_count} current processes (ok)",
            spec.label
        );
    }

    if !found {
        println!("running {}: not detected", spec.label);
    }
}

fn canonicalize_for_compare(path: &Path) -> Option<PathBuf> {
    fs::canonicalize(path).ok()
}

struct RuntimeSpec {
    label: &'static str,
    process_name: String,
    manifest_field: &'static str,
    restart_hint: &'static str,
}

#[derive(Debug)]
struct DaemonHealthError {
    kind: DaemonHealthErrorKind,
    message: String,
}

#[derive(Debug, PartialEq, Eq)]
enum DaemonHealthErrorKind {
    NotRunning,
    Other,
}

fn daemon_health() -> Result<Value, DaemonHealthError> {
    let mut stream = TcpStream::connect(DAEMON_ADDR).map_err(|error| DaemonHealthError {
        kind: if matches!(
            error.kind(),
            std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::TimedOut
        ) {
            DaemonHealthErrorKind::NotRunning
        } else {
            DaemonHealthErrorKind::Other
        },
        message: error.to_string(),
    })?;
    stream
        .set_read_timeout(Some(DAEMON_HEALTH_TIMEOUT))
        .map_err(other_daemon_health_error)?;
    stream
        .set_write_timeout(Some(DAEMON_HEALTH_TIMEOUT))
        .map_err(other_daemon_health_error)?;

    stream
        .write_all(b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .map_err(other_daemon_health_error)?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(other_daemon_health_error)?;
    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or_else(|| other_daemon_health_message("invalid daemon HTTP response"))?;
    if !headers.starts_with("HTTP/1.1 200") && !headers.starts_with("HTTP/1.0 200") {
        return Err(other_daemon_health_message(
            "daemon returned non-200 status",
        ));
    }
    serde_json::from_str(body).map_err(|error| other_daemon_health_message(error.to_string()))
}

fn other_daemon_health_error(error: std::io::Error) -> DaemonHealthError {
    other_daemon_health_message(error.to_string())
}

fn other_daemon_health_message(message: impl Into<String>) -> DaemonHealthError {
    DaemonHealthError {
        kind: DaemonHealthErrorKind::Other,
        message: message.into(),
    }
}

fn report_dir(label: &str, path: &Path) {
    println!(
        "{label}: {} ({})",
        display_path(path),
        if path.is_dir() { "ok" } else { "missing" }
    );
}

fn report_file(label: &str, path: &Path) {
    println!(
        "{label}: {} ({})",
        display_path(path),
        if path.is_file() || path.is_dir() {
            "ok"
        } else {
            "missing"
        }
    );
}
