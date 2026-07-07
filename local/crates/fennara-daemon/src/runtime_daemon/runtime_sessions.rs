use axum::{Json, extract::State};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{path::PathBuf, time::Duration};
use tokio::process::Command;

use super::{
    process_helpers::{auto_continue_local_debugger, resolve_godot_executable},
    runtime_log,
    state::{AppState, RuntimeLogCursor, RuntimeSession},
    util::{sanitize_path_component, unix_millis},
};

#[cfg(target_os = "windows")]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
const STARTUP_READY_TIMEOUT_MS: u64 = 5_000;
const STARTUP_CAPTURE_TIMEOUT_MS: u64 = 3_000;
const STARTUP_CAPTURE_MAX_RESOLUTION: u16 = 1280;

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeSessionStartRequest {
    session_id: Option<String>,
    executable: String,
    working_directory: String,
    scene_path: String,
    artifact_dir: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeSessionIdRequest {
    session_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeScriptRequest {
    session_id: String,
    script_run_id: String,
    script_path: String,
    timeout_ms: Option<u64>,
}

pub(crate) async fn runtime_session_start(
    State(state): State<AppState>,
    Json(request): Json<RuntimeSessionStartRequest>,
) -> Json<Value> {
    match runtime_session_start_inner(&state, request).await {
        Ok(value) => Json(value),
        Err(error) => Json(json!({ "ok": false, "error": error })),
    }
}

pub(crate) async fn runtime_session_status(
    State(state): State<AppState>,
    Json(request): Json<RuntimeSessionIdRequest>,
) -> Json<Value> {
    match runtime_session_status_inner(&state, &request.session_id).await {
        Ok(value) => Json(value),
        Err(error) => Json(json!({ "ok": false, "error": error })),
    }
}

pub(crate) async fn runtime_session_stop(
    State(state): State<AppState>,
    Json(request): Json<RuntimeSessionIdRequest>,
) -> Json<Value> {
    match runtime_session_stop_inner(&state, &request.session_id).await {
        Ok(value) => Json(value),
        Err(error) => Json(json!({ "ok": false, "error": error })),
    }
}

pub(crate) async fn runtime_session_script(
    State(state): State<AppState>,
    Json(request): Json<RuntimeScriptRequest>,
) -> Json<Value> {
    match runtime_session_script_inner(&state, request).await {
        Ok(value) => Json(value),
        Err(error) => Json(json!({ "ok": false, "error": error })),
    }
}

async fn runtime_session_start_inner(
    state: &AppState,
    request: RuntimeSessionStartRequest,
) -> Result<Value, String> {
    {
        let mut sessions = state.runtime_sessions.lock().await;
        for (existing_id, existing) in sessions.iter_mut() {
            if existing
                .child
                .try_wait()
                .map_err(|err| format!("runtime session wait failed: {err}"))?
                .is_none()
            {
                return Err(format!(
                    "Runtime session already running: {existing_id}. Fennara currently allows one managed runtime session across all connected Godot editors. Use runtime_session.status or runtime_session.stop before starting another scene."
                ));
            }
        }
    }

    if request.scene_path.trim().is_empty() {
        return Err("scene_path is required.".to_string());
    }
    let executable = resolve_godot_executable(&request.executable).ok_or_else(|| {
        format!(
            "Could not find Godot executable. Tried sent path '{}' and PATH candidates: godot, godot4, godot-mono, godot4-mono.",
            request.executable
        )
    })?;
    let working_directory = PathBuf::from(request.working_directory.trim());
    if !working_directory.is_dir() {
        return Err(format!(
            "Working directory was not found: {}",
            working_directory.display()
        ));
    }
    let artifact_dir = PathBuf::from(request.artifact_dir.trim());
    if artifact_dir.as_os_str().is_empty() {
        return Err("artifact_dir is required.".to_string());
    }
    tokio::fs::create_dir_all(&artifact_dir)
        .await
        .map_err(|err| format!("create artifact_dir failed: {err}"))?;
    let command_dir = artifact_dir.join("commands");
    tokio::fs::create_dir_all(&command_dir)
        .await
        .map_err(|err| format!("create command_dir failed: {err}"))?;
    let captures_dir = artifact_dir.join("captures");
    tokio::fs::create_dir_all(&captures_dir)
        .await
        .map_err(|err| format!("create captures_dir failed: {err}"))?;

    let session_id = request
        .session_id
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| format!("runtime-{}", unix_millis()));
    let raw_log_path = artifact_dir.join("runtime_session.log");
    let spec_path = artifact_dir.join("runtime_session_spec.json");
    let startup_capture_status_path = artifact_dir.join("runtime_session_startup_capture.json");
    let spec = json!({
        "mode": "runtime_session",
        "session_id": session_id,
        "command_dir": command_dir.to_string_lossy(),
        "artifact_dir": artifact_dir.to_string_lossy(),
        "captures_dir": captures_dir.to_string_lossy(),
        "startup_capture_status_path": startup_capture_status_path.to_string_lossy(),
        "startup_capture_max_resolution": STARTUP_CAPTURE_MAX_RESOLUTION,
        "scene_path": request.scene_path,
    });
    tokio::fs::write(
        &spec_path,
        serde_json::to_string_pretty(&spec)
            .map_err(|err| format!("serialize runtime spec failed: {err}"))?,
    )
    .await
    .map_err(|err| format!("write runtime spec failed: {err}"))?;

    let log_file = std::fs::File::create(&raw_log_path)
        .map_err(|err| format!("create runtime session log failed: {err}"))?;
    let stderr_file = log_file
        .try_clone()
        .map_err(|err| format!("clone runtime session log failed: {err}"))?;

    let mut command = Command::new(&executable);
    command
        .arg("--windowed")
        .arg("--debug")
        .arg("--ignore-error-breaks")
        .arg("--path")
        .arg(&working_directory)
        .arg("--scene")
        .arg(&request.scene_path)
        .current_dir(&working_directory)
        .env("FENNARA_RT_SPEC", &spec_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::from(log_file))
        .stderr(std::process::Stdio::from(stderr_file));

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NEW_PROCESS_GROUP);
    command.kill_on_drop(true);

    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to start runtime session: {err}"))?;
    if let Some(stdin) = child.stdin.take() {
        auto_continue_local_debugger(stdin);
    }
    let pid = child.id().unwrap_or_default();
    let mut log_cursor = RuntimeLogCursor::default();
    let (ready_seen, orientation_seen, process_exited, startup_wait_ms) =
        runtime_log::wait_for_ready(
            &mut child,
            &raw_log_path,
            log_cursor.byte_offset,
            STARTUP_READY_TIMEOUT_MS,
        )
        .await?;
    let log_capture =
        runtime_log::capture_update(&session_id, &raw_log_path, "start", &mut log_cursor).await;
    if process_exited && !ready_seen {
        let exit_code = child
            .try_wait()
            .map_err(|err| format!("runtime session wait failed: {err}"))?
            .and_then(|status| status.code());
        let response = json!({
            "ok": false,
            "status": "exited_before_ready",
            "scope": "global",
            "scope_note": "Fennara currently allows one managed runtime session across all connected Godot editors.",
            "session_id": session_id,
            "pid": pid,
            "scene_path": request.scene_path,
            "artifact_dir": artifact_dir.to_string_lossy(),
            "captures_dir": captures_dir.to_string_lossy(),
            "command_dir": command_dir.to_string_lossy(),
            "raw_log_path": raw_log_path.to_string_lossy(),
            "spec_path": spec_path.to_string_lossy(),
            "startup_capture_status_path": startup_capture_status_path.to_string_lossy(),
            "executable": executable.to_string_lossy(),
            "startup_log_wait_ms": startup_wait_ms,
            "startup_ready_seen": ready_seen,
            "startup_orientation_seen": orientation_seen,
            "startup_process_exited": process_exited,
            "exit_code": exit_code,
            "error": "Runtime process exited before the runtime helper reported scene ready.",
            "runtime_log": log_capture.receipt,
        });
        return Ok(response);
    }
    let startup_capture =
        wait_for_json_file(&startup_capture_status_path, STARTUP_CAPTURE_TIMEOUT_MS).await;

    let session = RuntimeSession {
        session_id: session_id.clone(),
        scene_path: request.scene_path.clone(),
        working_directory,
        artifact_dir: artifact_dir.clone(),
        captures_dir: captures_dir.clone(),
        command_dir: command_dir.clone(),
        raw_log_path: raw_log_path.clone(),
        startup_capture: startup_capture.clone(),
        log_cursor,
        script_log_start_offsets: Default::default(),
        child,
        started_ms: unix_millis(),
    };
    let mut sessions = state.runtime_sessions.lock().await;
    let mut running_session_id = None;
    for (existing_id, existing) in sessions.iter_mut() {
        if existing
            .child
            .try_wait()
            .map_err(|err| format!("runtime session wait failed: {err}"))?
            .is_none()
        {
            running_session_id = Some(existing_id.clone());
            break;
        }
    }
    if let Some(existing_id) = running_session_id {
        drop(sessions);
        let mut session = session;
        let _ = session.child.kill().await;
        return Err(format!(
            "Runtime session already running: {existing_id}. Fennara currently allows one managed runtime session across all connected Godot editors. Use runtime_session.status or runtime_session.stop before starting another scene."
        ));
    }
    sessions.insert(session_id.clone(), session);
    let session = sessions
        .get_mut(&session_id)
        .ok_or_else(|| format!("Runtime session disappeared after start: {session_id}"))?;

    let mut response = json!({
        "ok": true,
        "status": "started",
        "scope": "global",
        "scope_note": "Fennara currently allows one managed runtime session across all connected Godot editors.",
        "session_id": session.session_id,
        "pid": pid,
        "scene_path": session.scene_path,
        "artifact_dir": artifact_dir.to_string_lossy(),
        "captures_dir": captures_dir.to_string_lossy(),
        "command_dir": command_dir.to_string_lossy(),
        "raw_log_path": raw_log_path.to_string_lossy(),
        "spec_path": spec_path.to_string_lossy(),
        "startup_capture_status_path": startup_capture_status_path.to_string_lossy(),
        "executable": executable.to_string_lossy(),
        "startup_log_wait_ms": startup_wait_ms,
        "startup_ready_seen": ready_seen,
        "startup_orientation_seen": orientation_seen,
        "startup_process_exited": process_exited,
        "runtime_log": log_capture.receipt,
    });
    attach_startup_capture(&mut response, startup_capture);
    Ok(response)
}

async fn runtime_session_status_inner(state: &AppState, session_id: &str) -> Result<Value, String> {
    let mut sessions = state.runtime_sessions.lock().await;
    let session = sessions
        .get_mut(session_id)
        .ok_or_else(|| format!("Runtime session not found: {session_id}"))?;
    let exit_status = session
        .child
        .try_wait()
        .map_err(|err| format!("runtime session wait failed: {err}"))?;
    let log_capture = runtime_log::capture_update(
        &session.session_id,
        &session.raw_log_path,
        "status",
        &mut session.log_cursor,
    )
    .await;
    Ok(json!({
        "ok": true,
        "session_id": session.session_id,
        "scene_path": session.scene_path,
        "running": exit_status.is_none(),
        "scope": "global",
        "scope_note": "Fennara currently allows one managed runtime session across all connected Godot editors.",
        "exit_code": exit_status.and_then(|status| status.code()),
        "artifact_dir": session.artifact_dir.to_string_lossy(),
        "captures_dir": session.captures_dir.to_string_lossy(),
        "command_dir": session.command_dir.to_string_lossy(),
        "raw_log_path": session.raw_log_path.to_string_lossy(),
        "working_directory": session.working_directory.to_string_lossy(),
        "started_ms": session.started_ms,
        "startup_capture": session.startup_capture.clone(),
        "runtime_log": log_capture.receipt,
    }))
}

async fn runtime_session_stop_inner(state: &AppState, session_id: &str) -> Result<Value, String> {
    let mut session = state
        .runtime_sessions
        .lock()
        .await
        .remove(session_id)
        .ok_or_else(|| format!("Runtime session not found: {session_id}"))?;
    let mut exit_code = None;
    if let Some(status) = session
        .child
        .try_wait()
        .map_err(|err| format!("runtime session wait failed: {err}"))?
    {
        exit_code = status.code();
    } else {
        let _ = session.child.kill().await;
        if let Ok(status) = session.child.wait().await {
            exit_code = status.code();
        }
    }
    let log_capture = runtime_log::capture_update(
        &session.session_id,
        &session.raw_log_path,
        "stop",
        &mut session.log_cursor,
    )
    .await;
    Ok(json!({
        "ok": true,
        "status": "stopped",
        "scope": "global",
        "session_id": session_id,
        "exit_code": exit_code,
        "artifact_dir": session.artifact_dir.to_string_lossy(),
        "captures_dir": session.captures_dir.to_string_lossy(),
        "raw_log_path": session.raw_log_path.to_string_lossy(),
        "startup_capture": session.startup_capture,
        "runtime_log": log_capture.receipt,
    }))
}

async fn runtime_session_script_inner(
    state: &AppState,
    request: RuntimeScriptRequest,
) -> Result<Value, String> {
    let session_id = request.session_id.clone();
    let script_run_id = request.script_run_id.clone();
    let (command_dir, artifact_dir, captures_dir, raw_log_path) = {
        let mut sessions = state.runtime_sessions.lock().await;
        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| format!("Runtime session not found: {session_id}"))?;
        let exit_status = session
            .child
            .try_wait()
            .map_err(|err| format!("runtime session wait failed: {err}"))?;
        if let Some(status) = exit_status {
            let mut session = sessions
                .remove(&session_id)
                .ok_or_else(|| format!("Runtime session not found: {session_id}"))?;
            drop(sessions);
            let log_capture = runtime_log::capture_update(
                &session.session_id,
                &session.raw_log_path,
                "runtime_script",
                &mut session.log_cursor,
            )
            .await;
            return Ok(json!({
                "ok": false,
                "status": "session_exited",
                "scope": "global",
                "session_id": session_id,
                "script_run_id": script_run_id,
                "artifact_dir": session.artifact_dir.to_string_lossy(),
                "captures_dir": session.captures_dir.to_string_lossy(),
                "raw_log_path": session.raw_log_path.to_string_lossy(),
                "exit_code": status.code(),
                "error": "Runtime session process exited before the script command could be sent.",
                "runtime_log": log_capture.receipt,
                "runtime_findings": runtime_log::findings_for_script(&log_capture.lines, &script_run_id),
            }));
        }
        let session = sessions
            .get(&session_id)
            .ok_or_else(|| format!("Runtime session not found: {session_id}"))?;
        (
            session.command_dir.clone(),
            session.artifact_dir.clone(),
            session.captures_dir.clone(),
            session.raw_log_path.clone(),
        )
    };
    tokio::fs::create_dir_all(&command_dir)
        .await
        .map_err(|err| format!("create command_dir failed: {err}"))?;
    let status_dir = artifact_dir.join("runtime_script_results");
    tokio::fs::create_dir_all(&status_dir)
        .await
        .map_err(|err| format!("create runtime_script_results dir failed: {err}"))?;
    let safe_script_run_id = sanitize_path_component(&script_run_id);
    let status_path = status_dir.join(format!("{safe_script_run_id}.json"));
    let _ = tokio::fs::remove_file(&status_path).await;
    let command_path = command_dir.join(format!("{safe_script_run_id}.json"));
    let command_temp_path = command_dir.join(format!("{safe_script_run_id}.tmp"));
    let _ = tokio::fs::remove_file(&command_temp_path).await;
    let _ = tokio::fs::remove_file(&command_path).await;
    let script_log_start_offset = tokio::fs::metadata(&raw_log_path)
        .await
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    let command = json!({
        "action": "run_runtime_script",
        "session_id": session_id,
        "script_run_id": script_run_id,
        "script_path": request.script_path,
        "status_path": status_path.to_string_lossy(),
    });
    let command_text = serde_json::to_string_pretty(&command)
        .map_err(|err| format!("serialize script command failed: {err}"))?;
    tokio::fs::write(&command_temp_path, command_text)
        .await
        .map_err(|err| format!("write script command temp file failed: {err}"))?;
    tokio::fs::rename(&command_temp_path, &command_path)
        .await
        .map_err(|err| format!("publish script command failed: {err}"))?;
    {
        let mut sessions = state.runtime_sessions.lock().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session
                .script_log_start_offsets
                .insert(script_run_id.clone(), script_log_start_offset);
        }
    }

    let deadline = tokio::time::Instant::now()
        + Duration::from_millis(request.timeout_ms.unwrap_or(10_000).clamp(500, 120_000));
    loop {
        if tokio::time::Instant::now() >= deadline {
            let mut response = json!({
                "ok": false,
                "status": "timeout",
                "scope": "global",
                "session_id": session_id,
                "script_run_id": script_run_id,
                "command_path": command_path.to_string_lossy(),
                "artifact_dir": artifact_dir.to_string_lossy(),
                "captures_dir": captures_dir.to_string_lossy(),
                "status_path": status_path.to_string_lossy(),
                "raw_log_path": raw_log_path.to_string_lossy(),
                "error": "Runtime script result did not arrive before timeout.",
            });
            attach_script_log(state, &session_id, &script_run_id, &mut response).await;
            return Ok(response);
        }
        if status_path.is_file() {
            let text = tokio::fs::read_to_string(&status_path)
                .await
                .map_err(|err| format!("read script status failed: {err}"))?;
            if let Ok(value) = serde_json::from_str::<Value>(&text) {
                let status = value.get("status").and_then(Value::as_str).unwrap_or("");
                if status == "completed" || status == "failed" {
                    let mut response = json!({
                        "ok": status == "completed",
                        "status": status,
                        "scope": "global",
                        "session_id": session_id,
                        "script_run_id": script_run_id,
                        "command_path": command_path.to_string_lossy(),
                        "artifact_dir": artifact_dir.to_string_lossy(),
                        "captures_dir": captures_dir.to_string_lossy(),
                        "status_path": status_path.to_string_lossy(),
                        "raw_log_path": raw_log_path.to_string_lossy(),
                        "result": value,
                    });
                    attach_script_log(state, &session_id, &script_run_id, &mut response).await;
                    return Ok(response);
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn attach_script_log(
    state: &AppState,
    session_id: &str,
    script_run_id: &str,
    response: &mut Value,
) {
    let mut sessions = state.runtime_sessions.lock().await;
    let Some(session) = sessions.get_mut(session_id) else {
        return;
    };
    let log_capture = runtime_log::capture_update(
        &session.session_id,
        &session.raw_log_path,
        "runtime_script",
        &mut session.log_cursor,
    )
    .await;
    let finding_lines =
        if let Some(byte_offset) = session.script_log_start_offsets.remove(script_run_id) {
            runtime_log::capture_from_offset(
                &session.session_id,
                &session.raw_log_path,
                "runtime_script_findings",
                byte_offset,
            )
            .await
            .lines
        } else {
            log_capture.lines.clone()
        };
    response["runtime_findings"] = runtime_log::findings_for_script(&finding_lines, script_run_id);
    response["runtime_log"] = log_capture.receipt;
}

async fn wait_for_json_file(path: &PathBuf, timeout_ms: u64) -> Option<Value> {
    let deadline = tokio::time::Instant::now() + Duration::from_millis(timeout_ms);
    loop {
        if let Ok(text) = tokio::fs::read_to_string(path).await {
            if let Ok(value) = serde_json::from_str::<Value>(&text) {
                return Some(value);
            }
        }
        if tokio::time::Instant::now() >= deadline {
            return None;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

fn attach_startup_capture(response: &mut Value, startup_capture: Option<Value>) {
    let Some(capture) = startup_capture else {
        return;
    };
    response["startup_capture"] = capture.clone();
    if capture.get("success").and_then(Value::as_bool) == Some(true) {
        response["captures"] = json!([capture]);
    }
}
