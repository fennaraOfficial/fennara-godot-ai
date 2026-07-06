use axum::Json;
use futures_util::stream::{FuturesUnordered, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::{process::Command, sync::Semaphore};

use super::{
    process_helpers::{
        allowed_child_env, append_runtime_log_footer, resolve_godot_executable,
        wait_for_memory_headroom,
    },
    util::sanitize_path_component,
};

#[cfg(target_os = "windows")]
const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;
#[derive(Debug, Deserialize)]
pub(crate) struct RunGodotSceneRequest {
    executable: String,
    working_directory: String,
    args: Vec<String>,
    env: Option<HashMap<String, String>>,
    run_seconds: f64,
    raw_log_path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct RunSceneBatchRequest {
    executable: String,
    working_directory: String,
    scene_paths: Vec<String>,
    run_seconds: f64,
    worker_count: usize,
    artifact_dir: String,
}

pub(crate) async fn run_godot_scene(Json(request): Json<RunGodotSceneRequest>) -> Json<Value> {
    match run_godot_scene_inner(request).await {
        Ok(value) => Json(value),
        Err(error) => Json(json!({
            "ok": false,
            "error": error
        })),
    }
}

pub(crate) async fn run_godot_scenes_batch(
    Json(request): Json<RunSceneBatchRequest>,
) -> Json<Value> {
    match run_godot_scenes_batch_inner(request).await {
        Ok(value) => Json(value),
        Err(error) => Json(json!({
            "ok": false,
            "error": error
        })),
    }
}

async fn run_godot_scene_inner(request: RunGodotSceneRequest) -> Result<Value, String> {
    let executable = PathBuf::from(request.executable.trim());
    if !executable.is_file() {
        return Err(format!(
            "Godot executable was not found: {}",
            executable.display()
        ));
    }

    let working_directory = PathBuf::from(request.working_directory.trim());
    if !working_directory.is_dir() {
        return Err(format!(
            "Working directory was not found: {}",
            working_directory.display()
        ));
    }

    let raw_log_path = PathBuf::from(request.raw_log_path.trim());
    if raw_log_path.as_os_str().is_empty() {
        return Err("raw_log_path is required.".to_string());
    }
    if let Some(parent) = raw_log_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|err| format!("create raw log directory failed: {err}"))?;
    }

    let timeout = Duration::from_millis((request.run_seconds.max(0.1) * 1000.0) as u64);
    let started = std::time::Instant::now();
    let mut log_file = std::fs::File::create(&raw_log_path)
        .map_err(|err| format!("create raw log failed: {err}"))?;
    writeln!(log_file, "# Fennara daemon runtime process log")
        .map_err(|err| format!("write raw log header failed: {err}"))?;
    writeln!(log_file, "Executable: {}", request.executable)
        .map_err(|err| format!("write raw log header failed: {err}"))?;
    writeln!(log_file, "Working directory: {}", request.working_directory)
        .map_err(|err| format!("write raw log header failed: {err}"))?;
    writeln!(log_file, "Args: {}", request.args.join(" "))
        .map_err(|err| format!("write raw log header failed: {err}"))?;
    writeln!(log_file).map_err(|err| format!("write raw log header failed: {err}"))?;
    let stderr_file = log_file
        .try_clone()
        .map_err(|err| format!("clone raw log handle failed: {err}"))?;

    let mut command = Command::new(&executable);
    command
        .args(&request.args)
        .current_dir(&working_directory)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(log_file))
        .stderr(std::process::Stdio::from(stderr_file));

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);

    if let Some(env) = &request.env {
        for (key, value) in env {
            if allowed_child_env(key) {
                command.env(key, value);
            }
        }
    }

    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to start Godot process: {err}"))?;
    let pid = child.id().unwrap_or(0);
    let mut killed = false;
    let exit_status;

    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|err| format!("process wait failed: {err}"))?
        {
            exit_status = Some(status);
            break;
        }

        if tokio::time::Instant::now() >= deadline {
            killed = true;
            let _ = child.kill().await;
            exit_status = child.wait().await.ok();
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let duration_seconds = started.elapsed().as_secs_f64();
    let exit_code = exit_status
        .and_then(|status| status.code())
        .unwrap_or(if killed { -1 } else { 0 });
    let status = if killed {
        "stopped_after_run_seconds"
    } else {
        "completed"
    };

    append_runtime_log_footer(&raw_log_path, status, exit_code, duration_seconds).await?;
    let output = tokio::fs::read_to_string(&raw_log_path)
        .await
        .unwrap_or_default();
    let output_bytes = output.len();

    Ok(json!({
        "ok": true,
        "status": status,
        "exit_code": exit_code,
        "duration_seconds": duration_seconds,
        "output": output,
        "raw_log_path": raw_log_path.to_string_lossy(),
        "pid": pid,
        "killed": killed,
        "stdout_bytes": output_bytes,
        "stderr_bytes": 0,
    }))
}

#[derive(Debug, Serialize)]
struct SceneBatchItem {
    scene_path: String,
    status: String,
    exit_code: i32,
    duration_seconds: f64,
    killed: bool,
    crashed: bool,
    has_error: bool,
    has_warning: bool,
    raw_log_path: String,
    compacted_log_path: String,
}

async fn run_godot_scenes_batch_inner(request: RunSceneBatchRequest) -> Result<Value, String> {
    if request.scene_paths.is_empty() {
        return Err("scene_paths must contain at least one scene.".to_string());
    }
    if request.scene_paths.len() > 10 {
        return Err("scene_paths supports at most 10 scenes per batch.".to_string());
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
    let raw_dir = artifact_dir.join("logs");
    tokio::fs::create_dir_all(&raw_dir)
        .await
        .map_err(|err| format!("create raw log directory failed: {err}"))?;

    let run_seconds = request.run_seconds.clamp(0.5, 30.0);
    let worker_count = request.worker_count.clamp(1, 10);
    let started = std::time::Instant::now();
    let semaphore = Arc::new(Semaphore::new(worker_count));
    let mut tasks = FuturesUnordered::new();

    for (index, scene_path) in request.scene_paths.iter().cloned().enumerate() {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| "worker semaphore closed".to_string())?;
        wait_for_memory_headroom().await;

        let executable = executable.clone();
        let working_directory = working_directory.clone();
        let raw_dir = raw_dir.clone();
        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            run_batch_scene_process(
                index,
                &scene_path,
                &executable,
                &working_directory,
                run_seconds,
                &raw_dir,
            )
            .await
        }));
    }

    let mut results: Vec<SceneBatchItem> = Vec::new();
    while let Some(joined) = tasks.next().await {
        match joined {
            Ok(Ok(item)) => results.push(item),
            Ok(Err(error)) => {
                let raw_log_path = artifact_dir.join("runner_error.log");
                let _ = tokio::fs::write(&raw_log_path, &error).await;
                results.push(SceneBatchItem {
                    scene_path: "<runner>".to_string(),
                    status: "runner_failed".to_string(),
                    exit_code: -1,
                    duration_seconds: 0.0,
                    killed: false,
                    crashed: false,
                    has_error: true,
                    has_warning: false,
                    raw_log_path: raw_log_path.to_string_lossy().to_string(),
                    compacted_log_path: String::new(),
                });
            }
            Err(error) => {
                results.push(SceneBatchItem {
                    scene_path: "<runner>".to_string(),
                    status: "runner_join_failed".to_string(),
                    exit_code: -1,
                    duration_seconds: 0.0,
                    killed: false,
                    crashed: false,
                    has_error: true,
                    has_warning: false,
                    raw_log_path: String::new(),
                    compacted_log_path: String::new(),
                });
                eprintln!("scene batch worker failed: {error}");
            }
        }
    }
    results.sort_by(|a, b| a.scene_path.cmp(&b.scene_path));

    let results_path = artifact_dir.join("results.json");
    tokio::fs::write(
        &results_path,
        serde_json::to_string_pretty(&results)
            .map_err(|err| format!("serialize batch results failed: {err}"))?,
    )
    .await
    .map_err(|err| format!("write batch results failed: {err}"))?;

    let crash_count = results.iter().filter(|item| item.crashed).count();
    let error_count = results.iter().filter(|item| item.has_error).count();
    let warning_count = results.iter().filter(|item| item.has_warning).count();

    Ok(json!({
        "ok": true,
        "status": if error_count == 0 { "success" } else { "completed_with_findings" },
        "worker_count": worker_count,
        "run_seconds": run_seconds,
        "duration_seconds": started.elapsed().as_secs_f64(),
        "scene_count": results.len(),
        "crash_count": crash_count,
        "error_count": error_count,
        "warning_count": warning_count,
        "results": results,
        "artifact_dir": artifact_dir.to_string_lossy(),
        "raw_logs_dir": raw_dir.to_string_lossy(),
        "compacted_log_path": "",
        "results_path": results_path.to_string_lossy(),
        "compacted_markdown": "",
        "executable": executable.to_string_lossy(),
    }))
}

async fn run_batch_scene_process(
    index: usize,
    scene_path: &str,
    executable: &Path,
    working_directory: &Path,
    run_seconds: f64,
    raw_dir: &Path,
) -> Result<SceneBatchItem, String> {
    let safe = sanitize_path_component(scene_path);
    let raw_log_path = raw_dir.join(format!("{index:02}_{safe}.log"));
    let timeout = Duration::from_millis((run_seconds.max(0.5) * 1000.0) as u64);
    let started = std::time::Instant::now();

    let mut log_file = std::fs::File::create(&raw_log_path)
        .map_err(|err| format!("create raw log failed for {scene_path}: {err}"))?;
    writeln!(log_file, "# Fennara daemon batch runtime log")
        .map_err(|err| format!("write raw log header failed for {scene_path}: {err}"))?;
    writeln!(log_file, "Scene: {scene_path}")
        .map_err(|err| format!("write raw log header failed for {scene_path}: {err}"))?;
    writeln!(log_file, "Executable: {}", executable.display())
        .map_err(|err| format!("write raw log header failed for {scene_path}: {err}"))?;
    writeln!(
        log_file,
        "Working directory: {}",
        working_directory.display()
    )
    .map_err(|err| format!("write raw log header failed for {scene_path}: {err}"))?;
    writeln!(
        log_file,
        "Args: --headless --debug --ignore-error-breaks --path {} {scene_path}",
        working_directory.display()
    )
    .map_err(|err| format!("write raw log header failed for {scene_path}: {err}"))?;
    writeln!(log_file)
        .map_err(|err| format!("write raw log header failed for {scene_path}: {err}"))?;
    let stderr_file = log_file
        .try_clone()
        .map_err(|err| format!("clone raw log handle failed for {scene_path}: {err}"))?;

    let mut command = Command::new(executable);
    command
        .arg("--headless")
        .arg("--debug")
        .arg("--ignore-error-breaks")
        .arg("--path")
        .arg(working_directory)
        .arg(scene_path)
        .current_dir(working_directory)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::from(log_file))
        .stderr(std::process::Stdio::from(stderr_file));

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);

    let mut child = command
        .spawn()
        .map_err(|err| format!("failed to start Godot for {scene_path}: {err}"))?;
    let mut killed = false;
    let deadline = tokio::time::Instant::now() + timeout;
    let exit_status;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|err| format!("process wait failed for {scene_path}: {err}"))?
        {
            exit_status = Some(status);
            break;
        }
        if tokio::time::Instant::now() >= deadline {
            killed = true;
            let _ = child.kill().await;
            exit_status = child.wait().await.ok();
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    let duration_seconds = started.elapsed().as_secs_f64();
    let exit_code = exit_status
        .and_then(|status| status.code())
        .unwrap_or(if killed { -1 } else { 0 });
    let status = if killed {
        "stopped_after_run_seconds"
    } else {
        "completed"
    };
    append_runtime_log_footer(&raw_log_path, status, exit_code, duration_seconds).await?;
    let text = tokio::fs::read_to_string(&raw_log_path)
        .await
        .unwrap_or_default();

    let crashed = text.contains("CrashHandlerException")
        || text.contains("Program crashed with signal")
        || exit_code == 3221225477_i64 as i32;
    let has_error = crashed
        || text.contains("\nERROR:")
        || text.starts_with("ERROR:")
        || text.contains("\nSCRIPT ERROR:")
        || text.starts_with("SCRIPT ERROR:");
    let has_warning = text.contains("\nWARNING:") || text.starts_with("WARNING:");

    Ok(SceneBatchItem {
        scene_path: scene_path.to_string(),
        status: status.to_string(),
        exit_code,
        duration_seconds,
        killed,
        crashed,
        has_error,
        has_warning,
        raw_log_path: raw_log_path.to_string_lossy().to_string(),
        compacted_log_path: String::new(),
    })
}
