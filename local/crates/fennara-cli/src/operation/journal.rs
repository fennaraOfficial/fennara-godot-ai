use super::artifact;
use super::error::fallback_code;
use super::redaction::sanitize_text;
use super::storage::{read_operation_state, unix_ms, validate_operation_id, write_json_atomic};
use super::{FailureClass, OperationKind, Phase, SCHEMA_VERSION};
use crate::VERSION;
use crate::app_layout::{AppLayout, arch_name, display_path, platform_name, read_current_manifest};
use serde_json::{Map, Value, json};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(super) struct OperationJournal {
    pub(super) id: String,
    kind: OperationKind,
    phase: Phase,
    started_at_unix_ms: u128,
    updated_at_unix_ms: u128,
    requested_version: String,
    components: Map<String, Value>,
    artifacts: Map<String, Value>,
    rollback_state: String,
    last_error: Option<Value>,
    failure_code_override: Option<String>,
    pub(super) state_path: PathBuf,
    pub(super) log_path: PathBuf,
    log_file: File,
    pub(super) app_dir: PathBuf,
    project_dir: Option<PathBuf>,
    home_dir: Option<PathBuf>,
    pub(super) completion_deferred: bool,
}

impl OperationJournal {
    pub(super) fn create(
        layout: AppLayout,
        kind: OperationKind,
        project_dir: Option<&Path>,
        requested_version: &str,
    ) -> Result<Self, String> {
        let now = unix_ms();
        let id = format!(
            "{}-{now}-{}",
            kind.as_str().replace('_', "-"),
            std::process::id()
        );
        let state_path = layout.operations_dir.join(format!("{id}.json"));
        let log_path = layout.operation_logs_dir.join(format!("{id}.jsonl"));
        let log_file = OpenOptions::new()
            .create_new(true)
            .append(true)
            .open(&log_path)
            .map_err(|err| format!("failed to create {}: {err}", display_path(&log_path)))?;
        let mut components = installed_components(&layout, project_dir);
        components.insert("cli".to_string(), Value::String(VERSION.to_string()));
        let mut journal = Self {
            id,
            kind,
            phase: Phase::Checking,
            started_at_unix_ms: now,
            updated_at_unix_ms: now,
            requested_version: requested_version.to_string(),
            components,
            artifacts: Map::new(),
            rollback_state: "not_started".to_string(),
            last_error: None,
            failure_code_override: None,
            state_path,
            log_path,
            log_file,
            app_dir: layout.app_dir,
            project_dir: project_dir.map(Path::to_path_buf),
            home_dir: home_dir(),
            completion_deferred: false,
        };
        journal.record(Phase::Checking, "Operation started", None)?;
        Ok(journal)
    }

    pub(super) fn resume(
        layout: AppLayout,
        id: &str,
        project_dir: Option<&Path>,
    ) -> Result<Self, String> {
        validate_operation_id(id)?;
        let state_path = layout.operations_dir.join(format!("{id}.json"));
        let log_path = layout.operation_logs_dir.join(format!("{id}.jsonl"));
        let state = read_operation_state(&state_path)
            .map_err(|err| format!("failed to resume operation {id}: {err}"))?;
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|err| format!("failed to open {}: {err}", display_path(&log_path)))?;
        let phase = phase_from_str(
            state
                .get("phase")
                .and_then(Value::as_str)
                .unwrap_or("checking"),
        );
        let mut journal = Self {
            id: id.to_string(),
            kind: operation_kind_from_str(
                state
                    .get("kind")
                    .and_then(Value::as_str)
                    .unwrap_or("update"),
            ),
            phase,
            started_at_unix_ms: state
                .get("started_at_unix_ms")
                .and_then(Value::as_u64)
                .map(u128::from)
                .unwrap_or_else(unix_ms),
            updated_at_unix_ms: unix_ms(),
            requested_version: state
                .get("requested_version")
                .and_then(Value::as_str)
                .unwrap_or("latest")
                .to_string(),
            components: state
                .get("components")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default(),
            artifacts: state
                .get("artifacts")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default(),
            rollback_state: state
                .get("rollback_state")
                .and_then(Value::as_str)
                .unwrap_or("not_started")
                .to_string(),
            last_error: state
                .get("last_error")
                .filter(|value| !value.is_null())
                .cloned(),
            failure_code_override: None,
            state_path,
            log_path,
            log_file,
            app_dir: layout.app_dir,
            project_dir: project_dir.map(Path::to_path_buf),
            home_dir: home_dir(),
            completion_deferred: false,
        };
        journal
            .components
            .insert("cli".into(), Value::String(VERSION.into()));
        journal.record(phase, "Operation resumed in a new CLI process", None)?;
        Ok(journal)
    }

    pub(super) fn record(
        &mut self,
        phase: Phase,
        message: &str,
        error_code: Option<&str>,
    ) -> Result<(), String> {
        self.phase = phase;
        self.updated_at_unix_ms = unix_ms();
        let message = self.sanitize(message);
        if let Some(code) = error_code {
            self.last_error = Some(json!({ "code": code, "message": message }));
        }
        let event = json!({
            "schema_version": SCHEMA_VERSION,
            "operation_id": self.id,
            "timestamp_unix_ms": self.updated_at_unix_ms,
            "kind": self.kind.as_str(),
            "phase": phase.as_str(),
            "message": message,
            "error_code": error_code,
            "components": self.components,
            "artifacts": self.artifacts,
            "rollback_state": self.rollback_state,
        });
        let mut entry = serde_json::to_vec(&event)
            .map_err(|err| format!("failed to serialize operation event: {err}"))?;
        entry.push(b'\n');
        self.log_file
            .write_all(&entry)
            .map_err(|err| format!("failed to append {}: {err}", display_path(&self.log_path)))?;
        self.log_file
            .sync_data()
            .map_err(|err| format!("failed to flush {}: {err}", display_path(&self.log_path)))?;
        self.write_state()
    }

    pub(super) fn set_component(&mut self, name: &str, version: &str) -> Result<(), String> {
        if !name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return Err(format!("invalid operation component name: {name}"));
        }
        self.components
            .insert(name.to_string(), Value::String(self.sanitize(version)));
        self.updated_at_unix_ms = unix_ms();
        self.write_state()
    }

    pub(super) fn set_requested_version(&mut self, version: &str) -> Result<(), String> {
        self.requested_version = self.sanitize(version);
        self.updated_at_unix_ms = unix_ms();
        self.write_state()
    }

    pub(super) fn select_asset(
        &mut self,
        name: &str,
        expected_sha256: Option<&str>,
    ) -> Result<(), String> {
        let name = self.sanitize(name);
        let expected_sha256 = expected_sha256.map(|value| self.sanitize(value));
        artifact::select(&mut self.artifacts, name, expected_sha256)?;
        self.updated_at_unix_ms = unix_ms();
        self.write_state()
    }

    pub(super) fn record_asset_hash(
        &mut self,
        name: &str,
        actual_sha256: &str,
        verified: Option<bool>,
    ) -> Result<(), String> {
        let name = self.sanitize(name);
        let actual_sha256 = self.sanitize(actual_sha256);
        artifact::record_hash(&mut self.artifacts, name, actual_sha256, verified)?;
        self.updated_at_unix_ms = unix_ms();
        self.write_state()
    }

    pub(super) fn set_failure_class(&mut self, class: FailureClass) -> Result<(), String> {
        self.failure_code_override = Some(class.code(self.kind));
        Ok(())
    }

    pub(super) fn failure_code(&self, _error: &str) -> String {
        if let Some(code) = &self.failure_code_override {
            return code.clone();
        }
        fallback_code(self.kind, self.phase)
    }

    fn write_state(&self) -> Result<(), String> {
        let state = json!({
            "schema_version": SCHEMA_VERSION,
            "operation_id": self.id,
            "kind": self.kind.as_str(),
            "phase": self.phase.as_str(),
            "started_at_unix_ms": self.started_at_unix_ms,
            "updated_at_unix_ms": self.updated_at_unix_ms,
            "requested_version": self.sanitize(&self.requested_version),
            "platform": platform_name(),
            "architecture": arch_name(),
            "components": self.components,
            "artifacts": self.artifacts,
            "rollback_state": self.rollback_state,
            "paths": {
                "project": self.project_dir.as_ref().map(|_| "<project>"),
                "app_data": "<fennara-data>",
                "event_log": format!("<fennara-data>/logs/operations/{}.jsonl", self.id),
            },
            "last_error": self.last_error,
        });
        write_json_atomic(&self.state_path, &state)
    }

    fn sanitize(&self, text: &str) -> String {
        sanitize_text(
            text,
            [
                Some((&self.app_dir, "<fennara-data>")),
                self.project_dir.as_ref().map(|path| (path, "<project>")),
                self.home_dir.as_ref().map(|path| (path, "<home>")),
            ]
            .into_iter()
            .flatten(),
        )
    }
}

fn installed_components(layout: &AppLayout, project_dir: Option<&Path>) -> Map<String, Value> {
    let mut components = Map::new();
    if let Ok(Some(manifest)) = read_current_manifest(&layout.current_manifest_path)
        && let Some(version) = manifest.get("version").and_then(Value::as_str)
    {
        components.insert("installed_runtime".into(), Value::String(version.into()));
    }
    if let Some(project) = project_dir
        && let Ok(version) =
            fs::read_to_string(project.join("addons").join("fennara").join("VERSION"))
    {
        let version = version.trim();
        if !version.is_empty() {
            components.insert("addon".into(), Value::String(version.into()));
        }
    }
    components
}

fn phase_from_str(value: &str) -> Phase {
    match value {
        "downloading" => Phase::Downloading,
        "verifying" => Phase::Verifying,
        "staging" => Phase::Staging,
        "handoff" => Phase::Handoff,
        "ready_to_close" => Phase::ReadyToClose,
        "waiting_for_godot" => Phase::WaitingForGodot,
        "applying" => Phase::Applying,
        "reopening" => Phase::Reopening,
        "validating" => Phase::Validating,
        "succeeded" => Phase::Succeeded,
        "rolled_back" => Phase::RolledBack,
        "failed" => Phase::Failed,
        "recovery_required" => Phase::RecoveryRequired,
        _ => Phase::Checking,
    }
}

fn operation_kind_from_str(value: &str) -> OperationKind {
    match value {
        "install" => OperationKind::Install,
        "self_update" => OperationKind::SelfUpdate,
        _ => OperationKind::Update,
    }
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}
