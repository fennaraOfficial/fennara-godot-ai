mod artifact;
mod error;
mod journal;
mod redaction;
mod storage;

#[cfg(test)]
mod tests;

use self::journal::OperationJournal;
use self::storage::{latest_operation_id, read_operation_state, validate_operation_id};
use crate::app_layout::{AppLayout, display_path};
use serde_json::Value;
use std::env;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub use self::error::FailureClass;

pub const OPERATION_ID_ENV: &str = "FENNARA_OPERATION_ID";
const SCHEMA_VERSION: u64 = 1;

static CURRENT: OnceLock<Mutex<Option<OperationJournal>>> = OnceLock::new();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperationKind {
    Install,
    Update,
    SelfUpdate,
}

impl OperationKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Install => "install",
            Self::Update => "update",
            Self::SelfUpdate => "self_update",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Phase {
    Checking,
    Downloading,
    Verifying,
    Staging,
    Handoff,
    ReadyToClose,
    WaitingForGodot,
    Applying,
    Reopening,
    Validating,
    Succeeded,
    RolledBack,
    Failed,
    RecoveryRequired,
}

impl Phase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Downloading => "downloading",
            Self::Verifying => "verifying",
            Self::Staging => "staging",
            Self::Handoff => "handoff",
            Self::ReadyToClose => "ready_to_close",
            Self::WaitingForGodot => "waiting_for_godot",
            Self::Applying => "applying",
            Self::Reopening => "reopening",
            Self::Validating => "validating",
            Self::Succeeded => "succeeded",
            Self::RolledBack => "rolled_back",
            Self::Failed => "failed",
            Self::RecoveryRequired => "recovery_required",
        }
    }
}

pub fn begin(kind: OperationKind, args: &[String]) -> Result<(), String> {
    let mut slot = current_slot()
        .lock()
        .map_err(|_| "operation journal lock is poisoned".to_string())?;
    if slot.is_some() {
        return Ok(());
    }

    let layout = AppLayout::detect()?;
    layout.ensure_base_dirs()?;
    let project_dir = project_dir_for_operation(kind, args);
    let requested_version = option_value(args, "--version").unwrap_or_else(|| "latest".into());
    let requested_operation_id = option_value(args, "--operation-id");
    let resume_id = env::var(OPERATION_ID_ENV)
        .ok()
        .filter(|value| !value.is_empty());
    let journal = if let Some(id) = resume_id {
        OperationJournal::resume(layout, &id, project_dir.as_deref())?
    } else if let Some(id) = requested_operation_id {
        OperationJournal::create_with_id(
            layout,
            kind,
            project_dir.as_deref(),
            &requested_version,
            &id,
        )?
    } else {
        OperationJournal::create(layout, kind, project_dir.as_deref(), &requested_version)?
    };
    println!("operation: {}", journal.id);
    println!("operation log: {}", display_path(&journal.log_path));
    *slot = Some(journal);
    Ok(())
}

pub fn phase(phase: Phase, message: &str) -> Result<(), String> {
    with_current(|journal| journal.record(phase, message, None))
}

pub fn set_component(name: &str, version: &str) -> Result<(), String> {
    with_current(|journal| journal.set_component(name, version))
}

pub fn set_requested_version(version: &str) -> Result<(), String> {
    with_current(|journal| journal.set_requested_version(version))
}

pub fn select_asset(name: &str, expected_sha256: Option<&str>) -> Result<(), String> {
    with_current(|journal| journal.select_asset(name, expected_sha256))
}

pub fn record_asset_hash(
    name: &str,
    actual_sha256: &str,
    verified: Option<bool>,
) -> Result<(), String> {
    with_current(|journal| journal.record_asset_hash(name, actual_sha256, verified))
}

pub fn failure(class: FailureClass, message: impl Into<String>) -> String {
    let message = message.into();
    match with_current(|journal| journal.set_failure_class(class)) {
        Ok(()) => message,
        Err(error) => format!("{message}; operation journal unavailable: {error}"),
    }
}

pub fn begin_handoff(message: &str) -> Result<(), String> {
    with_current(|journal| {
        journal.record(Phase::Handoff, message, None)?;
        journal.completion_deferred = true;
        Ok(())
    })
}

pub fn cancel_handoff() -> Result<(), String> {
    with_current(|journal| {
        journal.completion_deferred = false;
        Ok(())
    })
}

pub fn current_id() -> Option<String> {
    let slot = current_slot().lock().ok()?;
    slot.as_ref().map(|journal| journal.id.clone())
}

pub fn finish_success() -> Result<(), String> {
    with_current(|journal| {
        if journal.completion_deferred {
            Ok(())
        } else {
            journal.record(Phase::Succeeded, "Operation completed successfully", None)
        }
    })
}

pub fn finish_failure(error: &str) -> Result<Option<(String, String, PathBuf)>, String> {
    let mut slot = current_slot()
        .lock()
        .map_err(|_| "operation journal lock is poisoned".to_string())?;
    let Some(journal) = slot.as_mut() else {
        return Ok(None);
    };
    let code = journal.failure_code(error);
    if !journal.completion_deferred {
        journal.record(Phase::Failed, error, Some(&code))?;
    }
    Ok(Some((code, journal.id.clone(), journal.log_path.clone())))
}

pub fn operation_environment(command: &mut std::process::Command) {
    if let Some(id) = current_id() {
        command.env(OPERATION_ID_ENV, id);
    }
}

pub fn diagnostics(operation_id: Option<&str>) -> Result<(Value, PathBuf, PathBuf), String> {
    let layout = AppLayout::detect()?;
    let id = match operation_id {
        Some(id) => validate_operation_id(id)?.to_string(),
        None => latest_operation_id(&layout.operations_dir)?.ok_or_else(|| {
            format!(
                "no Fennara operation records were found in {}",
                display_path(&layout.operations_dir)
            )
        })?,
    };
    let state_path = layout.operations_dir.join(format!("{id}.json"));
    let log_path = layout.operation_logs_dir.join(format!("{id}.jsonl"));
    let state = read_operation_state(&state_path)?;
    Ok((state, state_path, log_path))
}

fn current_slot() -> &'static Mutex<Option<OperationJournal>> {
    CURRENT.get_or_init(|| Mutex::new(None))
}

fn with_current(
    callback: impl FnOnce(&mut OperationJournal) -> Result<(), String>,
) -> Result<(), String> {
    let mut slot = current_slot()
        .lock()
        .map_err(|_| "operation journal lock is poisoned".to_string())?;
    let Some(journal) = slot.as_mut() else {
        return Ok(());
    };
    callback(journal)
}

fn option_value(args: &[String], option: &str) -> Option<String> {
    for (index, arg) in args.iter().enumerate() {
        if arg == option {
            return args.get(index + 1).cloned();
        }
        if let Some(value) = arg.strip_prefix(&format!("{option}=")) {
            return Some(value.to_string());
        }
    }
    None
}

fn project_dir_for_operation(kind: OperationKind, args: &[String]) -> Option<PathBuf> {
    if kind == OperationKind::SelfUpdate {
        return None;
    }
    let path = option_value(args, "--project")
        .map(PathBuf::from)
        .or_else(|| env::current_dir().ok())?;
    if path.is_absolute() {
        Some(path)
    } else {
        env::current_dir().ok().map(|cwd| cwd.join(path))
    }
}
