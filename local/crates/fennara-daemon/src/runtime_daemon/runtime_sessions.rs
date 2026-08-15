use axum::{Json, extract::State};
use fennara_project_identity::ProjectRoot;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{ffi::OsStr, path::PathBuf, sync::atomic::Ordering, time::Duration};
use tokio::{process::Command, sync::oneshot, task::JoinHandle};

use super::{
    process_helpers::{auto_continue_local_debugger, resolve_godot_executable},
    runtime_log,
    runtime_slot::{
        BUSY_RETRY_AFTER_MS, LeaseExpiry, LeaseSnapshot, SlotObservation, validate_max_run_seconds,
    },
    state::{
        AppState, RuntimeEndReason, RuntimeLogCursor, RuntimeSession, RuntimeSessionMetadata,
        RuntimeSessionReceipt,
    },
    util::{sanitize_path_component, unix_millis},
};

mod launch_command;
#[cfg(test)]
mod launch_command_tests;

#[cfg(target_os = "windows")]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
const STARTUP_READY_TIMEOUT_MS: u64 = 5_000;
const STARTUP_CAPTURE_TIMEOUT_MS: u64 = 3_000;
const STARTUP_CAPTURE_MAX_RESOLUTION: u16 = 1280;
const SUPERVISOR_INTERVAL_MS: u64 = 500;
const DEFAULT_RUNTIME_SCRIPT_TIMEOUT_MS: u64 = 30_000;
const MIN_RUNTIME_SCRIPT_TIMEOUT_MS: u64 = 500;
const MAX_RUNTIME_SCRIPT_TIMEOUT_MS: u64 = 120_000;
const NOT_OWNED_OR_FOUND_MESSAGE: &str =
    "Runtime session is not owned by this Project Root or was not found.";

#[derive(Debug)]
struct RuntimeSessionError {
    code: Option<&'static str>,
    message: String,
}

impl RuntimeSessionError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            code: None,
            message: message.into(),
        }
    }

    fn not_owned_or_found() -> Self {
        Self {
            code: Some("runtime_session_not_owned_or_found"),
            message: NOT_OWNED_OR_FOUND_MESSAGE.to_string(),
        }
    }

    fn into_response(self) -> Value {
        json!({
            "ok": false,
            "error": self.message,
            "code": self.code,
        })
    }
}

impl From<String> for RuntimeSessionError {
    fn from(message: String) -> Self {
        Self::new(message)
    }
}

pub(crate) struct RuntimeSupervisor {
    cancel: oneshot::Sender<()>,
    task: JoinHandle<()>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeSessionStartRequest {
    project_path: String,
    executable: String,
    working_directory: String,
    scene_path: String,
    artifact_dir: String,
    #[serde(default)]
    user_args: Vec<String>,
    #[serde(default)]
    max_run_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeSessionStatusRequest {
    project_path: String,
    #[serde(default)]
    session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeSessionIdRequest {
    project_path: String,
    session_id: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RuntimeScriptRequest {
    project_path: String,
    session_id: String,
    script_run_id: String,
    script_path: String,
    timeout_ms: Option<u64>,
}

enum RuntimeScriptOperationOutcome {
    Completed(Value),
    SessionExited(Value),
}

trait RuntimeStartAdapter: Send + Sync {
    fn before_admission(&self) -> impl std::future::Future<Output = ()> + Send;
    fn spawn(&self, command: &mut Command) -> std::io::Result<tokio::process::Child>;
}

#[derive(Clone, Copy)]
struct DirectRuntimeStartAdapter;

impl RuntimeStartAdapter for DirectRuntimeStartAdapter {
    fn before_admission(&self) -> impl std::future::Future<Output = ()> + Send {
        std::future::ready(())
    }

    fn spawn(&self, command: &mut Command) -> std::io::Result<tokio::process::Child> {
        command.spawn()
    }
}

pub(crate) fn spawn_runtime_supervisor(state: AppState) -> RuntimeSupervisor {
    let (cancel, mut cancelled) = oneshot::channel();
    let task = tokio::spawn(async move {
        let mut interval = tokio::time::interval_at(
            tokio::time::Instant::now() + Duration::from_millis(SUPERVISOR_INTERVAL_MS),
            Duration::from_millis(SUPERVISOR_INTERVAL_MS),
        );
        loop {
            tokio::select! {
                _ = &mut cancelled => break,
                _ = interval.tick() => maintain_runtime_sessions(&state).await,
            }
        }
    });
    RuntimeSupervisor { cancel, task }
}

impl RuntimeSupervisor {
    pub(crate) async fn shutdown(self) {
        let Self { cancel, task } = self;
        let _ = cancel.send(());
        if let Err(error) = task.await {
            eprintln!("Runtime Session supervisor failed while shutting down: {error}");
        }
    }
}

pub(crate) async fn runtime_session_start(
    State(state): State<AppState>,
    Json(request): Json<RuntimeSessionStartRequest>,
) -> Json<Value> {
    let task_state = state.clone();
    finish_runtime_request(tokio::spawn(async move {
        maintain_runtime_sessions(&task_state).await;
        runtime_session_start_inner(&task_state, request).await
    }))
    .await
}

pub(crate) async fn runtime_session_status(
    State(state): State<AppState>,
    Json(request): Json<RuntimeSessionStatusRequest>,
) -> Json<Value> {
    let task_state = state.clone();
    finish_runtime_request(tokio::spawn(async move {
        maintain_runtime_sessions(&task_state).await;
        runtime_session_status_inner(&task_state, request).await
    }))
    .await
}

pub(crate) async fn runtime_session_stop(
    State(state): State<AppState>,
    Json(request): Json<RuntimeSessionIdRequest>,
) -> Json<Value> {
    let task_state = state.clone();
    finish_runtime_request(tokio::spawn(async move {
        maintain_runtime_sessions(&task_state).await;
        runtime_session_stop_inner(&task_state, request).await
    }))
    .await
}

pub(crate) async fn runtime_session_script(
    State(state): State<AppState>,
    Json(request): Json<RuntimeScriptRequest>,
) -> Json<Value> {
    let task_state = state.clone();
    finish_runtime_request(tokio::spawn(async move {
        maintain_runtime_sessions(&task_state).await;
        runtime_session_script_inner(&task_state, request).await
    }))
    .await
}

async fn finish_runtime_request(
    task: tokio::task::JoinHandle<Result<Value, RuntimeSessionError>>,
) -> Json<Value> {
    match task.await {
        Ok(Ok(value)) => Json(value),
        Ok(Err(error)) => Json(error_response(error)),
        Err(error) => Json(error_response(RuntimeSessionError::new(format!(
            "Runtime Session task failed: {error}"
        )))),
    }
}

async fn runtime_session_start_inner(
    state: &AppState,
    request: RuntimeSessionStartRequest,
) -> Result<Value, RuntimeSessionError> {
    runtime_session_start_inner_with_ready_timeout(state, request, STARTUP_READY_TIMEOUT_MS).await
}

async fn runtime_session_start_inner_with_ready_timeout(
    state: &AppState,
    request: RuntimeSessionStartRequest,
    startup_ready_timeout_ms: u64,
) -> Result<Value, RuntimeSessionError> {
    runtime_session_start_inner_with_adapter(
        state,
        request,
        startup_ready_timeout_ms,
        &DirectRuntimeStartAdapter,
    )
    .await
}

async fn runtime_session_start_inner_with_adapter(
    state: &AppState,
    request: RuntimeSessionStartRequest,
    startup_ready_timeout_ms: u64,
    adapter: &impl RuntimeStartAdapter,
) -> Result<Value, RuntimeSessionError> {
    let owner = resolve_project_root(&request.project_path)?;
    let working_root = resolve_project_root(&request.working_directory)?;
    if !owner.same_project(&working_root) {
        return Err(RuntimeSessionError::new(
            "Runtime working_directory must resolve to the requesting Godot Project Root."
                .to_string(),
        ));
    }
    if request.scene_path.trim().is_empty() {
        return Err("scene_path is required.".to_string().into());
    }
    let executable = resolve_godot_executable(&request.executable).ok_or_else(|| {
        format!(
            "Could not find Godot executable. Tried sent path '{}' and PATH candidates: godot, godot4, godot-mono, godot4-mono.",
            request.executable
        )
    })?;
    let artifact_dir = PathBuf::from(request.artifact_dir.trim());
    if artifact_dir.as_os_str().is_empty() {
        return Err("artifact_dir is required.".to_string().into());
    }

    let max_run_seconds =
        validate_max_run_seconds(request.max_run_seconds).map_err(ToString::to_string)?;
    adapter.before_admission().await;
    let request_started_ms = now_ms();
    let mut claim = match state
        .runtime_slot
        .try_claim(owner.clone(), request_started_ms)
    {
        Ok(claim) => claim,
        Err(_) => return Ok(busy_response()),
    };

    tokio::fs::create_dir_all(&artifact_dir)
        .await
        .map_err(|error| format!("create artifact_dir failed: {error}"))?;
    let command_dir = artifact_dir.join("commands");
    tokio::fs::create_dir_all(&command_dir)
        .await
        .map_err(|error| format!("create command_dir failed: {error}"))?;
    let captures_dir = artifact_dir.join("captures");
    tokio::fs::create_dir_all(&captures_dir)
        .await
        .map_err(|error| format!("create captures_dir failed: {error}"))?;

    let request_sequence = state.request_counter.fetch_add(1, Ordering::Relaxed) + 1;
    let session_id = format!("runtime-{request_started_ms}-{request_sequence}");
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
            .map_err(|error| format!("serialize runtime spec failed: {error}"))?,
    )
    .await
    .map_err(|error| format!("write runtime spec failed: {error}"))?;

    let log_file = std::fs::File::create(&raw_log_path)
        .map_err(|error| format!("create runtime session log failed: {error}"))?;
    let stderr_file = log_file
        .try_clone()
        .map_err(|error| format!("clone runtime session log failed: {error}"))?;

    let working_directory = working_root.canonical_path().to_path_buf();
    let mut command = Command::new(&executable);
    command
        .args(launch_command::godot_runtime_arguments(
            &working_directory,
            &request.scene_path,
            &request.user_args,
        ))
        .current_dir(&working_directory)
        .env("FENNARA_RT_SPEC", &spec_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::from(log_file))
        .stderr(std::process::Stdio::from(stderr_file));

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NEW_PROCESS_GROUP);
    command.kill_on_drop(true);

    let mut child = adapter
        .spawn(&mut command)
        .map_err(|error| format!("failed to start runtime session: {error}"))?;
    claim.mark_process_spawned();
    if let Some(stdin) = child.stdin.take() {
        auto_continue_local_debugger(stdin);
    }
    let pid = child.id().unwrap_or_default();
    let mut log_cursor = RuntimeLogCursor::default();
    let (ready_seen, orientation_seen, process_exited, startup_wait_ms) =
        match runtime_log::wait_for_ready(
            &mut child,
            &raw_log_path,
            log_cursor.byte_offset,
            startup_ready_timeout_ms,
        )
        .await
        {
            Ok(result) => result,
            Err(error) => {
                terminate_and_reap(&mut child).await;
                claim.release_after_cleanup();
                return Err(error.into());
            }
        };
    let log_capture =
        runtime_log::capture_update(&session_id, &raw_log_path, "start", &mut log_cursor).await;
    let startup_failure_response =
        |status: &str, startup_process_exited: bool, exit_code: Option<i32>, error: &str| {
            json!({
                "ok": false,
                "status": status,
                "availability": "free",
                "slot_acquired": false,
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
                "startup_process_exited": startup_process_exited,
                "exit_code": exit_code,
                "error": error,
                "runtime_log": log_capture.receipt.clone(),
            })
        };
    if process_exited {
        let exit_code = terminate_and_reap(&mut child).await;
        claim.release_after_cleanup();
        let error = if ready_seen {
            "Runtime process exited after reporting ready but before startup completed."
        } else {
            "Runtime process exited before the runtime helper reported scene ready."
        };
        return Ok(startup_failure_response(
            "exited_before_ready",
            true,
            exit_code,
            error,
        ));
    }
    if !(ready_seen && orientation_seen) {
        let exit_code = terminate_and_reap(&mut child).await;
        claim.release_after_cleanup();
        return Ok(startup_failure_response(
            "startup_timeout",
            false,
            exit_code,
            "Runtime process did not report scene readiness and startup orientation before the startup deadline.",
        ));
    }
    let startup_capture =
        wait_for_json_file(&startup_capture_status_path, STARTUP_CAPTURE_TIMEOUT_MS).await;

    let exit_before_commit = match child.try_wait() {
        Ok(status) => status,
        Err(error) => {
            terminate_and_reap(&mut child).await;
            claim.release_after_cleanup();
            return Err(format!(
                "failed to inspect runtime process before startup commit: {error}"
            )
            .into());
        }
    };
    if let Some(status) = exit_before_commit {
        let exit_code = status.code();
        claim.release_after_cleanup();
        return Ok(startup_failure_response(
            "exited_before_ready",
            true,
            exit_code,
            "Runtime process exited before startup could be committed.",
        ));
    }

    let running_started_ms = now_ms();
    let session = RuntimeSession {
        metadata: RuntimeSessionMetadata {
            session_id: session_id.clone(),
            owner,
            scene_path: request.scene_path.clone(),
            artifact_dir: artifact_dir.clone(),
            captures_dir: captures_dir.clone(),
            raw_log_path: raw_log_path.clone(),
            startup_capture: startup_capture.clone(),
            started_ms: u128::from(running_started_ms),
            max_run_seconds,
        },
        working_directory,
        command_dir: command_dir.clone(),
        log_cursor,
        script_log_start_offsets: Default::default(),
        child,
    };

    let mut sessions = state.runtime_sessions.lock().await;
    let lease_snapshot = match claim.commit(session_id.clone(), max_run_seconds, running_started_ms)
    {
        Ok(snapshot) => snapshot,
        Err(error) => {
            drop(sessions);
            let mut session = session;
            terminate_and_reap(&mut session.child).await;
            claim.release_after_cleanup();
            return Err(format!("Runtime Slot claim could not be committed: {error}").into());
        }
    };
    sessions.insert(session_id.clone(), session);
    drop(sessions);

    let mut response = json!({
        "ok": true,
        "status": "started",
        "availability": "busy",
        "slot_acquired": true,
        "scope": "global",
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
        "runtime_log": log_capture.receipt,
        "max_run_seconds": max_run_seconds,
    });
    attach_lease_fields(&mut response, lease_snapshot);
    attach_startup_capture(&mut response, startup_capture);
    Ok(response)
}

async fn runtime_session_status_inner(
    state: &AppState,
    request: RuntimeSessionStatusRequest,
) -> Result<Value, RuntimeSessionError> {
    let owner = resolve_project_root(&request.project_path)?;
    let requested_session_id = nonempty(request.session_id.as_deref());
    let now = now_ms();
    match state
        .runtime_slot
        .renew_and_observe(&owner, requested_session_id, now)
    {
        SlotObservation::Free => {
            if let Some(session_id) = requested_session_id {
                return receipt_status(state, &owner, session_id)
                    .await
                    .ok_or_else(RuntimeSessionError::not_owned_or_found);
            }
            Ok(free_response())
        }
        SlotObservation::Busy => Ok(busy_response()),
        SlotObservation::NotOwnedOrFound => Err(RuntimeSessionError::not_owned_or_found()),
        SlotObservation::Owned(owned) => {
            let mut sessions = state.runtime_sessions.lock().await;
            let session = sessions
                .get_mut(&owned.session_id)
                .ok_or_else(RuntimeSessionError::not_owned_or_found)?;
            let log_capture = runtime_log::capture_update(
                &session.metadata.session_id,
                &session.metadata.raw_log_path,
                "status",
                &mut session.log_cursor,
            )
            .await;
            let mut response = json!({
                "ok": true,
                "status": "running",
                "availability": "busy",
                "slot_acquired": true,
                "session_id": session.metadata.session_id,
                "scene_path": session.metadata.scene_path,
                "running": true,
                "scope": "global",
                "artifact_dir": session.metadata.artifact_dir.to_string_lossy(),
                "captures_dir": session.metadata.captures_dir.to_string_lossy(),
                "command_dir": session.command_dir.to_string_lossy(),
                "raw_log_path": session.metadata.raw_log_path.to_string_lossy(),
                "working_directory": session.working_directory.to_string_lossy(),
                "started_ms": session.metadata.started_ms,
                "startup_capture": session.metadata.startup_capture.clone(),
                "runtime_log": log_capture.receipt,
                "max_run_seconds": session.metadata.max_run_seconds,
            });
            attach_lease_fields(&mut response, owned.lease);
            Ok(response)
        }
    }
}

async fn runtime_session_stop_inner(
    state: &AppState,
    request: RuntimeSessionIdRequest,
) -> Result<Value, RuntimeSessionError> {
    let owner = resolve_project_root(&request.project_path)?;
    let session_id = require_session_id(&request.session_id)?;
    let cleanup = state
        .runtime_slot
        .begin_owner_cleanup(&owner, session_id, now_ms())
        .map_err(|_| RuntimeSessionError::not_owned_or_found())?;
    let session = state
        .runtime_sessions
        .lock()
        .await
        .remove(session_id)
        .ok_or_else(RuntimeSessionError::not_owned_or_found)?;
    let terminal = finish_session(session, RuntimeEndReason::Stopped, None).await;
    let response = stopped_response(&terminal.receipt, terminal.log_receipt);
    record_receipt(state, terminal.receipt).await;
    cleanup.release_after_reap();
    Ok(response)
}

async fn runtime_session_script_inner(
    state: &AppState,
    request: RuntimeScriptRequest,
) -> Result<Value, RuntimeSessionError> {
    runtime_session_script_inner_with_preparation_delay(state, request, Duration::ZERO).await
}

async fn runtime_session_script_inner_with_preparation_delay(
    state: &AppState,
    request: RuntimeScriptRequest,
    preparation_delay: Duration,
) -> Result<Value, RuntimeSessionError> {
    let owner = resolve_project_root(&request.project_path)?;
    let session_id = require_session_id(&request.session_id)?.to_string();
    let script_run_id = request.script_run_id.clone();
    let timeout_ms = runtime_script_timeout_ms(request.timeout_ms);
    let (command_dir, artifact_dir, captures_dir, raw_log_path) = {
        let sessions = state.runtime_sessions.lock().await;
        let session = sessions
            .get(&session_id)
            .ok_or_else(RuntimeSessionError::not_owned_or_found)?;
        (
            session.command_dir.clone(),
            session.metadata.artifact_dir.clone(),
            session.metadata.captures_dir.clone(),
            session.metadata.raw_log_path.clone(),
        )
    };
    let status_dir = artifact_dir.join("runtime_script_results");
    let safe_script_run_id = sanitize_path_component(&script_run_id);
    let status_path = status_dir.join(format!("{safe_script_run_id}.json"));
    let command_path = command_dir.join(format!("{safe_script_run_id}.json"));
    let command_temp_path = command_dir.join(format!("{safe_script_run_id}.tmp"));
    let operation = state
        .runtime_slot
        .begin_owner_operation(&owner, &session_id, timeout_ms)
        .map_err(|_| RuntimeSessionError::not_owned_or_found())?;
    let operation_deadline = operation.deadline();

    let outcome = tokio::time::timeout_at(operation_deadline, async {
        if !preparation_delay.is_zero() {
            tokio::time::sleep(preparation_delay).await;
        }
        tokio::fs::create_dir_all(&command_dir)
            .await
            .map_err(|error| format!("create command_dir failed: {error}"))?;
        tokio::fs::create_dir_all(&status_dir)
            .await
            .map_err(|error| format!("create runtime_script_results dir failed: {error}"))?;
        let _ = tokio::fs::remove_file(&status_path).await;
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
            .map_err(|error| format!("serialize script command failed: {error}"))?;
        tokio::fs::write(&command_temp_path, command_text)
            .await
            .map_err(|error| format!("write script command temp file failed: {error}"))?;
        tokio::fs::rename(&command_temp_path, &command_path)
            .await
            .map_err(|error| format!("publish script command failed: {error}"))?;
        {
            let mut sessions = state.runtime_sessions.lock().await;
            let session = sessions
                .get_mut(&session_id)
                .ok_or_else(RuntimeSessionError::not_owned_or_found)?;
            session
                .script_log_start_offsets
                .insert(script_run_id.clone(), script_log_start_offset);
        }

        loop {
            if tokio::fs::try_exists(&status_path).await.unwrap_or(false) {
                let text = tokio::fs::read_to_string(&status_path)
                    .await
                    .map_err(|error| format!("read script status failed: {error}"))?;
                if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    let status = value.get("status").and_then(Value::as_str).unwrap_or("");
                    if status == "completed" || status == "failed" {
                        return Ok(RuntimeScriptOperationOutcome::Completed(json!({
                            "ok": status == "completed",
                            "status": status,
                            "availability": "busy",
                            "slot_acquired": true,
                            "scope": "global",
                            "session_id": session_id,
                            "script_run_id": script_run_id,
                            "command_path": command_path.to_string_lossy(),
                            "artifact_dir": artifact_dir.to_string_lossy(),
                            "captures_dir": captures_dir.to_string_lossy(),
                            "status_path": status_path.to_string_lossy(),
                            "raw_log_path": raw_log_path.to_string_lossy(),
                            "result": value,
                        })));
                    }
                }
            }
            if !state
                .runtime_sessions
                .lock()
                .await
                .contains_key(&session_id)
            {
                let availability = if state.runtime_slot.is_occupied_now() {
                    "busy"
                } else {
                    "free"
                };
                return Ok(RuntimeScriptOperationOutcome::SessionExited(json!({
                    "ok": false,
                    "status": "session_exited",
                    "availability": availability,
                    "slot_acquired": false,
                    "code": "runtime_session_not_owned_or_found",
                    "error": NOT_OWNED_OR_FOUND_MESSAGE,
                })));
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await;

    match outcome {
        Ok(Ok(RuntimeScriptOperationOutcome::Completed(mut response))) => {
            operation.finish(now_ms());
            attach_script_log(state, &session_id, &script_run_id, &mut response).await;
            Ok(response)
        }
        Ok(Ok(RuntimeScriptOperationOutcome::SessionExited(response))) => {
            drop(operation);
            Ok(response)
        }
        Ok(Err(error)) => {
            drop(operation);
            Err(error)
        }
        Err(_) => {
            drop(operation);
            let mut response = json!({
                "ok": false,
                "status": "timeout",
                "availability": "busy",
                "slot_acquired": true,
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
            Ok(response)
        }
    }
}

async fn maintain_runtime_sessions(state: &AppState) {
    maintain_runtime_sessions_at(state, now_ms()).await;
}

async fn maintain_runtime_sessions_at(state: &AppState, now: u64) {
    if let Some(cleanup) = state.runtime_slot.claim_expired(now) {
        let session_id = cleanup.session_id().to_string();
        let reason = match cleanup.reason() {
            Some(LeaseExpiry::Absolute) => RuntimeEndReason::AbsoluteLeaseExpired,
            Some(LeaseExpiry::Inactivity) => RuntimeEndReason::InactivityLeaseExpired,
            None => return,
        };
        if let Some(session) = state.runtime_sessions.lock().await.remove(&session_id) {
            let terminal = finish_session(session, reason, None).await;
            record_receipt(state, terminal.receipt).await;
            cleanup.release_after_reap();
        }
        return;
    }

    let ended = {
        let mut sessions = state.runtime_sessions.lock().await;
        let ended = sessions.iter_mut().find_map(|(session_id, session)| {
            session
                .child
                .try_wait()
                .ok()
                .flatten()
                .map(|status| (session_id.clone(), status.code()))
        });
        ended.and_then(|(session_id, exit_code)| {
            let cleanup = state.runtime_slot.claim_finished(&session_id)?;
            sessions
                .remove(&session_id)
                .map(|session| (session, exit_code, cleanup))
        })
    };
    if let Some((session, exit_code, cleanup)) = ended {
        let terminal = finish_session(session, RuntimeEndReason::NaturalExit, exit_code).await;
        record_receipt(state, terminal.receipt).await;
        cleanup.release_after_reap();
    }
}

struct FinishedSession {
    receipt: RuntimeSessionReceipt,
    log_receipt: Value,
}

async fn terminate_and_reap(child: &mut tokio::process::Child) -> Option<i32> {
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status.code(),
            Ok(None) => {}
            Err(error) => {
                eprintln!("Failed to inspect Runtime Session child while reaping: {error}");
            }
        }

        if let Err(error) = child.start_kill() {
            eprintln!("Failed to signal Runtime Session child while reaping: {error}");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

async fn finish_session(
    mut session: RuntimeSession,
    end_reason: RuntimeEndReason,
    known_exit_code: Option<i32>,
) -> FinishedSession {
    let exit_code = if known_exit_code.is_some() {
        known_exit_code
    } else if end_reason.should_terminate() {
        terminate_and_reap(&mut session.child).await
    } else {
        // A caller that did not already observe the exit must still retain the
        // Runtime Slot until this Adapter proves the child has been reaped.
        loop {
            match session.child.try_wait() {
                Ok(Some(status)) => break status.code(),
                Ok(None) => {}
                Err(error) => {
                    eprintln!("Failed to inspect naturally exiting Runtime Session child: {error}");
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    };
    let log_capture = runtime_log::capture_update(
        &session.metadata.session_id,
        &session.metadata.raw_log_path,
        end_reason.log_mode(),
        &mut session.log_cursor,
    )
    .await;
    FinishedSession {
        receipt: RuntimeSessionReceipt {
            metadata: session.metadata,
            ended_ms: unix_millis(),
            end_reason,
            exit_code,
        },
        log_receipt: log_capture.receipt,
    }
}

async fn record_receipt(state: &AppState, receipt: RuntimeSessionReceipt) {
    let mut receipts = state.runtime_session_receipts.lock().await;
    receipts.insert(receipt.metadata.session_id.clone(), receipt);
    while receipts.len() > 32 {
        let oldest = receipts
            .iter()
            .min_by_key(|(_, receipt)| receipt.ended_ms)
            .map(|(session_id, _)| session_id.clone());
        if let Some(session_id) = oldest {
            receipts.remove(&session_id);
        } else {
            break;
        }
    }
}

async fn receipt_status(state: &AppState, owner: &ProjectRoot, session_id: &str) -> Option<Value> {
    let receipts = state.runtime_session_receipts.lock().await;
    let receipt = receipts.get(session_id)?;
    if !owner.same_project(&receipt.metadata.owner) {
        return None;
    }
    Some(receipt_status_response(receipt))
}

fn terminal_response(receipt: &RuntimeSessionReceipt) -> Value {
    json!({
        "ok": true,
        "status": "stopped",
        "availability": "free",
        "slot_acquired": false,
        "session_id": receipt.metadata.session_id,
        "exit_code": receipt.exit_code,
        "end_reason": receipt.end_reason.wire_value(),
        "artifact_dir": receipt.metadata.artifact_dir.to_string_lossy(),
        "captures_dir": receipt.metadata.captures_dir.to_string_lossy(),
        "raw_log_path": receipt.metadata.raw_log_path.to_string_lossy(),
        "startup_capture": receipt.metadata.startup_capture,
        "max_run_seconds": receipt.metadata.max_run_seconds,
    })
}

fn stopped_response(receipt: &RuntimeSessionReceipt, runtime_log: Value) -> Value {
    let mut response = terminal_response(receipt);
    response["scope"] = json!("global");
    response["runtime_log"] = runtime_log;
    response
}

fn receipt_status_response(receipt: &RuntimeSessionReceipt) -> Value {
    let mut response = terminal_response(receipt);
    response["scene_path"] = json!(receipt.metadata.scene_path);
    response["running"] = json!(false);
    response["code"] = json!(receipt.end_reason.code());
    response["started_ms"] = json!(receipt.metadata.started_ms);
    response["ended_ms"] = json!(receipt.ended_ms);
    response
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
        &session.metadata.session_id,
        &session.metadata.raw_log_path,
        "runtime_script",
        &mut session.log_cursor,
    )
    .await;
    let finding_lines =
        if let Some(byte_offset) = session.script_log_start_offsets.remove(script_run_id) {
            runtime_log::capture_from_offset(
                &session.metadata.session_id,
                &session.metadata.raw_log_path,
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

fn resolve_project_root(value: &str) -> Result<ProjectRoot, RuntimeSessionError> {
    ProjectRoot::resolve_absolute(OsStr::new(value))
        .map_err(|error| RuntimeSessionError::new(format!("Invalid runtime Project Root: {error}")))
}

fn free_response() -> Value {
    json!({
        "ok": true,
        "status": "idle",
        "availability": "free",
        "slot_acquired": false,
    })
}

fn busy_response() -> Value {
    json!({
        "ok": true,
        "status": "busy",
        "availability": "busy",
        "slot_acquired": false,
        "retry_after_ms": BUSY_RETRY_AFTER_MS,
    })
}

fn error_response(error: RuntimeSessionError) -> Value {
    error.into_response()
}

fn require_session_id(value: &str) -> Result<&str, RuntimeSessionError> {
    nonempty(Some(value)).ok_or_else(|| RuntimeSessionError::new("session_id is required."))
}

fn runtime_script_timeout_ms(requested: Option<u64>) -> u64 {
    requested
        .unwrap_or(DEFAULT_RUNTIME_SCRIPT_TIMEOUT_MS)
        .clamp(MIN_RUNTIME_SCRIPT_TIMEOUT_MS, MAX_RUNTIME_SCRIPT_TIMEOUT_MS)
}

fn nonempty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn now_ms() -> u64 {
    unix_millis().min(u128::from(u64::MAX)) as u64
}

fn attach_lease_fields(response: &mut Value, lease: LeaseSnapshot) {
    response["absolute_deadline_ms"] = json!(lease.absolute_deadline_ms);
    response["absolute_remaining_seconds"] = json!(lease.absolute_remaining_seconds);
    response["inactivity_deadline_ms"] = json!(lease.inactivity_deadline_ms);
    response["inactivity_remaining_seconds"] = json!(lease.inactivity_remaining_seconds);
    response["heartbeat_interval_ms"] = json!(lease.heartbeat_interval_ms);
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

#[cfg(test)]
mod tests {
    use super::{
        RuntimeScriptRequest, RuntimeSessionError, RuntimeSessionIdRequest,
        RuntimeSessionStartRequest, busy_response, error_response, finish_runtime_request,
        free_response, maintain_runtime_sessions, maintain_runtime_sessions_at,
        runtime_script_timeout_ms, runtime_session_script_inner_with_preparation_delay,
        runtime_session_start_inner, runtime_session_start_inner_with_adapter,
        runtime_session_start_inner_with_ready_timeout, runtime_session_status_inner,
        runtime_session_stop_inner,
    };
    use crate::runtime_daemon::{
        control_auth::{self, CONTROL_HEADER},
        state::AppState,
    };
    use axum::{Json, Router, extract::State, http::StatusCode, middleware, routing::post};
    use serde_json::{Value, json};
    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
        },
        time::Duration,
    };
    use tokio::sync::{Barrier, oneshot};

    static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(1);

    #[cfg(unix)]
    #[derive(Clone)]
    struct ConcurrentStartAdapter {
        admission_barrier: Arc<Barrier>,
        spawn_calls: Arc<AtomicUsize>,
    }

    #[cfg(unix)]
    impl super::RuntimeStartAdapter for ConcurrentStartAdapter {
        async fn before_admission(&self) {
            self.admission_barrier.wait().await;
        }

        fn spawn(
            &self,
            command: &mut tokio::process::Command,
        ) -> std::io::Result<tokio::process::Child> {
            self.spawn_calls.fetch_add(1, Ordering::SeqCst);
            command.spawn()
        }
    }

    #[cfg(unix)]
    #[derive(Clone)]
    struct ConcurrentStartState {
        app: AppState,
        adapter: ConcurrentStartAdapter,
    }

    #[cfg(unix)]
    async fn synchronized_runtime_session_start(
        State(state): State<ConcurrentStartState>,
        Json(request): Json<RuntimeSessionStartRequest>,
    ) -> Json<Value> {
        let task_state = state.app.clone();
        let adapter = state.adapter.clone();
        finish_runtime_request(tokio::spawn(async move {
            maintain_runtime_sessions(&task_state).await;
            runtime_session_start_inner_with_adapter(
                &task_state,
                request,
                super::STARTUP_READY_TIMEOUT_MS,
                &adapter,
            )
            .await
        }))
        .await
    }

    #[cfg(unix)]
    async fn synchronized_runtime_session_stop(
        State(state): State<ConcurrentStartState>,
        Json(request): Json<RuntimeSessionIdRequest>,
    ) -> Json<Value> {
        let task_state = state.app.clone();
        finish_runtime_request(tokio::spawn(async move {
            maintain_runtime_sessions(&task_state).await;
            runtime_session_stop_inner(&task_state, request).await
        }))
        .await
    }

    #[cfg(unix)]
    async fn delayed_runtime_session_script(
        State(state): State<AppState>,
        Json(request): Json<RuntimeScriptRequest>,
    ) -> Json<Value> {
        let task_state = state.clone();
        finish_runtime_request(tokio::spawn(async move {
            maintain_runtime_sessions(&task_state).await;
            runtime_session_script_inner_with_preparation_delay(
                &task_state,
                request,
                Duration::from_millis(400),
            )
            .await
        }))
        .await
    }

    #[test]
    fn anonymous_busy_response_contains_no_owner_or_session_details() {
        let response = busy_response();
        assert_eq!(response["ok"], true);
        assert_eq!(response["status"], "busy");
        assert_eq!(response["availability"], "busy");
        assert_eq!(response["slot_acquired"], false);
        assert!(response.get("session_id").is_none());
        assert!(response.get("project_path").is_none());
        assert!(response.get("scene_path").is_none());
        assert!(response.get("artifact_dir").is_none());
    }

    #[test]
    fn free_response_uses_the_strict_capacity_contract() {
        let response = free_response();
        assert_eq!(response["status"], "idle");
        assert_eq!(response["availability"], "free");
        assert_eq!(response["slot_acquired"], false);
    }

    #[test]
    fn named_non_owner_error_uses_one_indistinguishable_code() {
        let response = error_response(RuntimeSessionError::not_owned_or_found());
        assert_eq!(response["ok"], false);
        assert_eq!(response["code"], "runtime_session_not_owned_or_found");

        let same_prose_without_the_typed_variant =
            error_response(RuntimeSessionError::new(super::NOT_OWNED_OR_FOUND_MESSAGE));
        assert!(same_prose_without_the_typed_variant["code"].is_null());
    }

    #[test]
    fn runtime_script_timeout_matches_the_public_default_and_bounds() {
        assert_eq!(runtime_script_timeout_ms(None), 30_000);
        assert_eq!(runtime_script_timeout_ms(Some(1)), 500);
        assert_eq!(runtime_script_timeout_ms(Some(120_001)), 120_000);
    }

    #[tokio::test]
    async fn dropping_the_http_waiter_does_not_cancel_runtime_work() {
        let completed = Arc::new(AtomicBool::new(false));
        let task_completed = Arc::clone(&completed);
        let (release_sender, release_receiver) = oneshot::channel::<()>();
        let runtime_task = tokio::spawn(async move {
            release_receiver.await.unwrap();
            task_completed.store(true, Ordering::SeqCst);
            Ok(json!({ "ok": true }))
        });
        let http_waiter = tokio::spawn(finish_runtime_request(runtime_task));

        tokio::task::yield_now().await;
        http_waiter.abort();
        let _ = http_waiter.await;
        release_sender.send(()).unwrap();

        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while !completed.load(Ordering::SeqCst) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn runtime_supervisor_can_be_cancelled_and_awaited_by_its_owner() {
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);
        let supervisor = super::spawn_runtime_supervisor(state);

        tokio::time::timeout(Duration::from_secs(1), supervisor.shutdown())
            .await
            .expect("Runtime Session supervisor did not shut down");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn startup_exit_is_reaped_before_the_slot_becomes_free() {
        let fixture = unix_fixture("startup-exit", "#!/bin/sh\nexit 7\n");
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);

        let response = runtime_session_start_inner(&state, fixture.start_request("artifacts"))
            .await
            .unwrap();

        assert_eq!(response["status"], "exited_before_ready");
        assert_eq!(response["exit_code"], 7);
        assert!(!state.runtime_slot.is_occupied_now());
        assert!(state.runtime_sessions.lock().await.is_empty());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn startup_timeout_reaps_live_child_before_the_slot_becomes_free() {
        let fixture = unix_fixture("startup-timeout", "#!/bin/sh\nwhile :; do sleep 1; done\n");
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);

        let response = runtime_session_start_inner_with_ready_timeout(
            &state,
            fixture.start_request("artifacts"),
            250,
        )
        .await
        .unwrap();

        assert_eq!(response["ok"], false);
        assert_eq!(response["status"], "startup_timeout");
        assert_eq!(response["availability"], "free");
        assert_eq!(response["slot_acquired"], false);
        assert_eq!(response["startup_ready_seen"], false);
        assert_eq!(response["startup_orientation_seen"], false);
        assert_eq!(response["startup_process_exited"], false);
        assert_process_gone(response["pid"].as_u64().unwrap());
        assert!(!state.runtime_slot.is_occupied_now());
        assert!(state.runtime_sessions.lock().await.is_empty());
        assert!(state.runtime_session_receipts.lock().await.is_empty());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn one_real_child_owns_the_slot_until_stop_reaps_it() {
        let fixture = unix_fixture(
            "running",
            r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while :; do sleep 1; done
"#,
        );
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);

        let started = runtime_session_start_inner(&state, fixture.start_request("artifacts-a"))
            .await
            .unwrap();
        assert_eq!(started["status"], "started");
        let session_id = started["session_id"].as_str().unwrap().to_string();

        let contender = runtime_session_start_inner(&state, fixture.start_request("artifacts-b"))
            .await
            .unwrap();
        assert_eq!(contender["status"], "busy");
        assert!(contender.get("session_id").is_none());

        let stopped = runtime_session_stop_inner(
            &state,
            RuntimeSessionIdRequest {
                project_path: fixture.root.to_string_lossy().into_owned(),
                session_id,
            },
        )
        .await
        .unwrap();
        assert_eq!(stopped["status"], "stopped");
        assert!(!state.runtime_slot.is_occupied_now());
        assert!(state.runtime_sessions.lock().await.is_empty());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn natural_exit_records_its_receipt_before_the_slot_becomes_free() {
        let fixture = unix_fixture(
            "natural-exit",
            r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while [ ! -f natural-exit.gate ]; do sleep 1; done
exit 4
"#,
        );
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);

        let started = runtime_session_start_inner(&state, fixture.start_request("artifacts"))
            .await
            .unwrap();
        let session_id = started["session_id"].as_str().unwrap().to_string();
        let pid = started["pid"].as_u64().unwrap();
        std::fs::write(fixture.root.join("natural-exit.gate"), b"go\n").unwrap();
        drive_maintenance_until_free(&state).await;

        assert_process_gone(pid);
        assert!(state.runtime_sessions.lock().await.is_empty());
        let status = runtime_session_status_inner(
            &state,
            super::RuntimeSessionStatusRequest {
                project_path: fixture.root.to_string_lossy().into_owned(),
                session_id: Some(session_id),
            },
        )
        .await
        .unwrap();
        assert_eq!(status["status"], "stopped");
        assert_eq!(status["availability"], "free");
        assert_eq!(status["end_reason"], "natural_exit");
        assert_eq!(status["exit_code"], 4);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn absolute_expiry_reaps_before_recording_its_receipt_and_freeing_the_slot() {
        let fixture = unix_fixture(
            "absolute-expiry",
            r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while :; do sleep 1; done
"#,
        );
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);

        let started = runtime_session_start_inner(
            &state,
            fixture.start_request_with_max_run_seconds("artifacts", 1),
        )
        .await
        .unwrap();
        let session_id = started["session_id"].as_str().unwrap().to_string();
        let pid = started["pid"].as_u64().unwrap();
        drive_maintenance_until_free(&state).await;

        assert_process_gone(pid);
        assert!(state.runtime_sessions.lock().await.is_empty());
        let status = runtime_session_status_inner(
            &state,
            super::RuntimeSessionStatusRequest {
                project_path: fixture.root.to_string_lossy().into_owned(),
                session_id: Some(session_id),
            },
        )
        .await
        .unwrap();
        assert_eq!(status["status"], "stopped");
        assert_eq!(status["availability"], "free");
        assert_eq!(status["end_reason"], "runtime_lease_expired");
        assert_eq!(status["code"], "runtime_lease_expired");
        assert_eq!(status["max_run_seconds"], 1);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn inactivity_expiry_reaps_before_recording_its_receipt_and_freeing_the_slot() {
        let fixture = unix_fixture(
            "inactivity-expiry",
            r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while :; do sleep 1; done
"#,
        );
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);

        let started = runtime_session_start_inner(
            &state,
            fixture.start_request_with_max_run_seconds("artifacts", 300),
        )
        .await
        .unwrap();
        let session_id = started["session_id"].as_str().unwrap().to_string();
        let pid = started["pid"].as_u64().unwrap();
        let inactivity_deadline = started["inactivity_deadline_ms"].as_u64().unwrap();
        maintain_runtime_sessions_at(&state, inactivity_deadline).await;

        assert_process_gone(pid);
        assert!(!state.runtime_slot.is_occupied_now());
        assert!(state.runtime_sessions.lock().await.is_empty());
        let status = runtime_session_status_inner(
            &state,
            super::RuntimeSessionStatusRequest {
                project_path: fixture.root.to_string_lossy().into_owned(),
                session_id: Some(session_id),
            },
        )
        .await
        .unwrap();
        assert_eq!(status["status"], "stopped");
        assert_eq!(status["availability"], "free");
        assert_eq!(status["end_reason"], "runtime_inactivity_expired");
        assert_eq!(status["code"], "runtime_lease_expired");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn authenticated_http_concurrent_starts_spawn_exactly_one_runtime_child() {
        let runtime_script = r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while :; do sleep 1; done
"#;
        let contenders = [
            unix_fixture("http-concurrent-a", runtime_script),
            unix_fixture("http-concurrent-b", runtime_script),
        ];
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);
        let spawn_calls = Arc::new(AtomicUsize::new(0));
        let route_state = ConcurrentStartState {
            app: state.clone(),
            adapter: ConcurrentStartAdapter {
                admission_barrier: Arc::new(Barrier::new(2)),
                spawn_calls: Arc::clone(&spawn_calls),
            },
        };
        let control_token: Arc<str> = Arc::from("runtime-concurrent-start-test-token");
        let app = Router::new()
            .route(
                "/runtime/session/start",
                post(synchronized_runtime_session_start),
            )
            .route(
                "/runtime/session/stop",
                post(synchronized_runtime_session_stop),
            )
            .route_layer(middleware::from_fn_with_state(
                control_token.clone(),
                control_auth::require_control_auth,
            ))
            .with_state(route_state);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let client = reqwest::Client::new();
        let start_url = format!("http://{address}/runtime/session/start");

        let [start_a, start_b] = contenders.each_ref().map(|contender| {
            client
                .post(&start_url)
                .header(CONTROL_HEADER, control_token.as_ref())
                .json(&json!({
                    "project_path": contender.root.to_string_lossy(),
                    "executable": contender.executable.to_string_lossy(),
                    "working_directory": contender.root.to_string_lossy(),
                    "scene_path": "res://test_scene.tscn",
                    "artifact_dir": contender.root.join("artifacts").to_string_lossy(),
                    "max_run_seconds": 60,
                }))
                .send()
        });
        let (response_a, response_b) = tokio::join!(start_a, start_b);
        let [response_a, response_b] = [response_a, response_b]
            .map(Result::unwrap)
            .map(reqwest::Response::error_for_status)
            .map(Result::unwrap);
        let (response_a, response_b) =
            tokio::join!(response_a.json::<Value>(), response_b.json::<Value>());
        let responses = [response_a.unwrap(), response_b.unwrap()];

        let winners = responses
            .iter()
            .enumerate()
            .filter(|(_, response)| response["status"] == "started")
            .collect::<Vec<_>>();
        let losers = responses
            .iter()
            .filter(|response| response["status"] == "busy")
            .collect::<Vec<_>>();
        assert_eq!(winners.len(), 1, "responses: {responses:?}");
        assert_eq!(losers.len(), 1, "responses: {responses:?}");
        assert_eq!(losers[0]["ok"], true);
        assert_eq!(losers[0]["availability"], "busy");
        assert_eq!(losers[0]["slot_acquired"], false);
        assert_no_runtime_details(losers[0]);
        assert_eq!(
            spawn_calls.load(Ordering::SeqCst),
            1,
            "exactly one Command::spawn call must cross the process adapter"
        );

        let (winner_index, winner) = winners[0];
        let pid = winner["pid"].as_u64().unwrap();
        let session_id = winner["session_id"].as_str().unwrap();
        assert!(pid > 0);
        assert_eq!(state.runtime_sessions.lock().await.len(), 1);
        let stopped: Value = client
            .post(format!("http://{address}/runtime/session/stop"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": contenders[winner_index].root.to_string_lossy(),
                "session_id": session_id,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(stopped["status"], "stopped");
        assert_process_gone(pid);
        assert!(state.runtime_sessions.lock().await.is_empty());
        assert!(!state.runtime_slot.is_occupied_now());

        server.abort();
        let _ = server.await;
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn authenticated_http_routes_preserve_runtime_session_ownership_and_privacy() {
        let owner = unix_fixture(
            "http-owner",
            r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while :; do sleep 1; done
"#,
        );
        let other = unix_fixture("http-other", "#!/bin/sh\nexit 0\n");
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);
        let control_token: Arc<str> = Arc::from("runtime-session-http-test-token");
        let app = Router::new()
            .route("/runtime/session/start", post(super::runtime_session_start))
            .route(
                "/runtime/session/status",
                post(super::runtime_session_status),
            )
            .route("/runtime/session/stop", post(super::runtime_session_stop))
            .route(
                "/runtime/session/script",
                post(super::runtime_session_script),
            )
            .route_layer(middleware::from_fn_with_state(
                control_token.clone(),
                control_auth::require_control_auth,
            ))
            .with_state(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let client = reqwest::Client::new();
        let base_url = format!("http://{address}/runtime/session");
        let owner_path = owner.root.to_string_lossy().into_owned();
        let other_path = other.root.to_string_lossy().into_owned();
        let artifact_path = owner
            .root
            .join("http-artifacts")
            .to_string_lossy()
            .into_owned();

        let unauthorized = client
            .post(format!("{base_url}/status"))
            .json(&json!({ "project_path": owner_path }))
            .send()
            .await
            .unwrap();
        assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

        let started: Value = client
            .post(format!("{base_url}/start"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": owner_path,
                "executable": owner.executable.to_string_lossy(),
                "working_directory": owner_path,
                "scene_path": "res://test_scene.tscn",
                "artifact_dir": artifact_path,
                "max_run_seconds": 60,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(started["ok"], true);
        assert_eq!(started["status"], "started");
        let session_id = started["session_id"].as_str().unwrap().to_string();
        let pid = started["pid"].as_u64().unwrap();

        let anonymous_other: Value = client
            .post(format!("{base_url}/status"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({ "project_path": other_path }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(anonymous_other["ok"], true);
        assert_eq!(anonymous_other["status"], "busy");
        assert_eq!(anonymous_other["availability"], "busy");
        assert_eq!(anonymous_other["slot_acquired"], false);
        assert_no_runtime_details(&anonymous_other);

        let named_other: Value = client
            .post(format!("{base_url}/status"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": other_path,
                "session_id": session_id,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(named_other["ok"], false);
        assert_eq!(named_other["code"], "runtime_session_not_owned_or_found");
        assert_no_runtime_details(&named_other);

        for (action, request) in [
            (
                "stop",
                json!({
                    "project_path": other_path,
                    "session_id": session_id,
                }),
            ),
            (
                "script",
                json!({
                    "project_path": other_path,
                    "session_id": session_id,
                    "script_run_id": "forbidden-script",
                    "script_path": "res://forbidden.gd",
                    "timeout_ms": 500,
                }),
            ),
        ] {
            let denied: Value = client
                .post(format!("{base_url}/{action}"))
                .header(CONTROL_HEADER, control_token.as_ref())
                .json(&request)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .json()
                .await
                .unwrap();
            assert_eq!(denied["ok"], false);
            assert_eq!(denied["code"], "runtime_session_not_owned_or_found");
            assert_no_runtime_details(&denied);
        }

        let owner_status: Value = client
            .post(format!("{base_url}/status"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": owner_path,
                "session_id": session_id,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(owner_status["ok"], true);
        assert_eq!(owner_status["status"], "running");
        assert_eq!(owner_status["session_id"], session_id);
        assert_eq!(owner_status["scene_path"], "res://test_scene.tscn");
        assert_eq!(owner_status["artifact_dir"], artifact_path);
        assert_eq!(owner_status["running"], true);

        let stopped: Value = client
            .post(format!("{base_url}/stop"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": owner_path,
                "session_id": session_id,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(stopped["ok"], true);
        assert_eq!(stopped["status"], "stopped");
        assert_eq!(stopped["session_id"], session_id);
        assert_process_gone(pid);
        assert!(!state.runtime_slot.is_occupied_now());
        assert!(state.runtime_sessions.lock().await.is_empty());

        server.abort();
        let _ = server.await;
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn script_preparation_consumes_the_end_to_end_operation_timeout() {
        let owner = unix_fixture(
            "http-script-preparation-timeout",
            r#"#!/bin/sh
capture_path=$(sed -n 's/^[[:space:]]*"startup_capture_status_path": "\(.*\)"[,]\{0,1\}$/\1/p' "$FENNARA_RT_SPEC")
printf '{"success":false}\n' > "$capture_path"
printf 'FENNARA_RUNTIME_SESSION_READY: {}\n'
printf 'FENNARA_RUNTIME_ORIENTATION_NOTE: startup\n'
while :; do sleep 1; done
"#,
        );
        let (shutdown_sender, _shutdown_receiver) = oneshot::channel();
        let state = AppState::new(shutdown_sender);
        let control_token: Arc<str> = Arc::from("runtime-script-preparation-timeout-test-token");
        let app = Router::new()
            .route("/runtime/session/start", post(super::runtime_session_start))
            .route("/runtime/session/stop", post(super::runtime_session_stop))
            .route(
                "/runtime/session/script",
                post(delayed_runtime_session_script),
            )
            .route_layer(middleware::from_fn_with_state(
                control_token.clone(),
                control_auth::require_control_auth,
            ))
            .with_state(state.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let client = reqwest::Client::new();
        let base_url = format!("http://{address}/runtime/session");
        let owner_path = owner.root.to_string_lossy().into_owned();
        let artifact_dir = owner.root.join("http-artifacts");

        let started: Value = client
            .post(format!("{base_url}/start"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": owner_path,
                "executable": owner.executable.to_string_lossy(),
                "working_directory": owner_path,
                "scene_path": "res://test_scene.tscn",
                "artifact_dir": artifact_dir.to_string_lossy(),
                "max_run_seconds": 300,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        let session_id = started["session_id"].as_str().unwrap().to_string();

        let operation_started = tokio::time::Instant::now();
        let script_result: Value = client
            .post(format!("{base_url}/script"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": owner_path,
                "session_id": session_id,
                "script_run_id": "preparation-timeout",
                "script_path": "res://preparation_timeout.gd",
                "timeout_ms": 500,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        let operation_elapsed = operation_started.elapsed();

        assert_eq!(script_result["ok"], false);
        assert_eq!(script_result["status"], "timeout");
        assert_eq!(script_result["availability"], "busy");
        assert_eq!(script_result["slot_acquired"], true);
        assert_eq!(
            script_result["error"],
            "Runtime script result did not arrive before timeout."
        );
        assert!(
            operation_elapsed < Duration::from_millis(800),
            "preparation was added before the result timeout: {operation_elapsed:?}"
        );
        assert!(
            artifact_dir
                .join("commands/preparation-timeout.json")
                .is_file()
        );

        let stopped: Value = client
            .post(format!("{base_url}/stop"))
            .header(CONTROL_HEADER, control_token.as_ref())
            .json(&json!({
                "project_path": owner_path,
                "session_id": session_id,
            }))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(stopped["status"], "stopped");

        server.abort();
        let _ = server.await;
        assert!(!state.runtime_slot.is_occupied_now());
        assert!(state.runtime_sessions.lock().await.is_empty());
    }

    #[cfg(unix)]
    struct UnixFixture {
        root: std::path::PathBuf,
        executable: std::path::PathBuf,
    }

    #[cfg(unix)]
    impl UnixFixture {
        fn start_request(&self, artifact_name: &str) -> RuntimeSessionStartRequest {
            self.start_request_with_max_run_seconds(artifact_name, 60)
        }

        fn start_request_with_max_run_seconds(
            &self,
            artifact_name: &str,
            max_run_seconds: u64,
        ) -> RuntimeSessionStartRequest {
            RuntimeSessionStartRequest {
                project_path: self.root.to_string_lossy().into_owned(),
                executable: self.executable.to_string_lossy().into_owned(),
                working_directory: self.root.to_string_lossy().into_owned(),
                scene_path: "res://test_scene.tscn".to_string(),
                artifact_dir: self.root.join(artifact_name).to_string_lossy().into_owned(),
                max_run_seconds: Some(max_run_seconds),
            }
        }
    }

    #[cfg(unix)]
    impl Drop for UnixFixture {
        fn drop(&mut self) {
            if let Err(error) = std::fs::remove_dir_all(&self.root)
                && error.kind() != std::io::ErrorKind::NotFound
            {
                eprintln!(
                    "Failed to remove Runtime Session test fixture {}: {error}",
                    self.root.display()
                );
            }
        }
    }

    #[cfg(unix)]
    async fn drive_maintenance_until_free(state: &AppState) {
        tokio::time::timeout(std::time::Duration::from_secs(4), async {
            loop {
                maintain_runtime_sessions(state).await;
                if !state.runtime_slot.is_occupied_now() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(25)).await;
            }
        })
        .await
        .expect("Runtime Session cleanup did not free the slot");
    }

    #[cfg(unix)]
    fn assert_no_runtime_details(response: &Value) {
        for field in [
            "session_id",
            "project_path",
            "owner",
            "scene_path",
            "artifact_dir",
            "captures_dir",
            "command_dir",
            "raw_log_path",
            "working_directory",
            "pid",
        ] {
            assert!(
                response.get(field).is_none(),
                "anonymous or non-owner response leaked {field}: {response}"
            );
        }
    }

    #[cfg(unix)]
    fn assert_process_gone(pid: u64) {
        let pid = i32::try_from(pid).expect("test child PID must fit in i32");
        // SAFETY: signal 0 performs no mutation; it only probes whether this
        // process identifier still names a live process or zombie.
        let result = unsafe { libc::kill(pid, 0) };
        assert_eq!(result, -1, "Runtime Session child {pid} still exists");
        assert_eq!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ESRCH),
            "Runtime Session child {pid} was not fully reaped"
        );
    }

    #[cfg(unix)]
    fn unix_fixture(name: &str, script: &str) -> UnixFixture {
        use std::os::unix::fs::PermissionsExt;

        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "fennara-runtime-handler-{}-{sequence}-{name}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("project.godot"), b"[application]\n").unwrap();
        let executable = root.join("fake-godot.sh");
        std::fs::write(&executable, script.as_bytes()).unwrap();
        let mut permissions = std::fs::metadata(&executable).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&executable, permissions).unwrap();
        UnixFixture { root, executable }
    }
}
