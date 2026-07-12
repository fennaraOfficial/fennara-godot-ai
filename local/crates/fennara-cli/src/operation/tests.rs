use super::journal::OperationJournal;
use super::storage::{read_operation_state, unix_ms, validate_operation_id};
use super::{FailureClass, OperationKind, Phase};
use crate::app_layout::AppLayout;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;

fn test_layout(name: &str) -> AppLayout {
    let app_dir = env::temp_dir().join(format!(
        "fennara-operation-test-{name}-{}-{}",
        std::process::id(),
        unix_ms()
    ));
    test_layout_at(app_dir)
}

fn test_layout_at(app_dir: PathBuf) -> AppLayout {
    AppLayout {
        bin_dir: app_dir.join("bin"),
        versions_dir: app_dir.join("versions"),
        cache_dir: app_dir.join("cache"),
        logs_dir: app_dir.join("logs"),
        operation_logs_dir: app_dir.join("logs").join("operations"),
        operations_dir: app_dir.join("operations"),
        tools_dir: app_dir.join("tools"),
        webview_dir: app_dir.join("webview"),
        current_manifest_path: app_dir.join("current.json"),
        app_dir,
    }
}

#[test]
fn writes_durable_state_and_jsonl_events() {
    let layout = test_layout("durable");
    layout.ensure_base_dirs().unwrap();
    let mut journal =
        OperationJournal::create(layout, OperationKind::Install, None, "latest").unwrap();
    journal
        .record(Phase::Downloading, "Downloading CLI", None)
        .unwrap();
    journal.record(Phase::Succeeded, "Complete", None).unwrap();

    let state: Value =
        serde_json::from_str(&fs::read_to_string(&journal.state_path).unwrap()).unwrap();
    assert_eq!(state["phase"], "succeeded");
    assert_eq!(state["kind"], "install");
    let events = fs::read_to_string(&journal.log_path).unwrap();
    assert_eq!(events.lines().count(), 3);
    assert!(events.contains("downloading"));
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn sanitizes_paths_tokens_and_url_queries() {
    let layout = test_layout("sanitize");
    layout.ensure_base_dirs().unwrap();
    let project = layout.app_dir.join("private-project");
    let mut journal =
        OperationJournal::create(layout, OperationKind::Update, Some(&project), "latest").unwrap();
    let message = format!(
        "project={} api_key=super-secret Authorization: Bearer abc123 https://example.com/file?token=secret",
        project.display()
    );
    journal
        .record(Phase::Failed, &message, Some("FEN-UPDATE-FAILED"))
        .unwrap();
    let events = fs::read_to_string(&journal.log_path).unwrap();
    assert!(events.contains("<project>"));
    assert!(events.contains("<redacted>"));
    assert!(!events.contains("super-secret"));
    assert!(!events.contains("abc123"));
    assert!(!events.contains("token=secret"));
    assert!(!events.contains("private-project"));
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn sanitizes_spaced_quoted_and_basic_credentials() {
    let layout = test_layout("structured-secrets");
    layout.ensure_base_dirs().unwrap();
    let mut journal =
        OperationJournal::create(layout, OperationKind::Install, None, "latest").unwrap();
    journal
        .record(
            Phase::Failed,
            r#"api_key: super-secret {"access_token": "json-secret"}; Authorization: Basic dXNlcjpwYXNz"#,
            Some("FEN-INSTALL-FAILED"),
        )
        .unwrap();

    let events = fs::read_to_string(&journal.log_path).unwrap();
    assert!(!events.contains("super-secret"));
    assert!(!events.contains("json-secret"));
    assert!(!events.contains("dXNlcjpwYXNz"));
    assert!(events.matches("<redacted>").count() >= 3);
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn rejects_unsafe_operation_ids() {
    assert!(validate_operation_id("../../current").is_err());
    assert!(validate_operation_id("install-123_456").is_ok());
}

#[test]
fn reads_previous_state_after_interrupted_atomic_swap() {
    let layout = test_layout("previous-state");
    layout.ensure_base_dirs().unwrap();
    let journal = OperationJournal::create(layout, OperationKind::Update, None, "latest").unwrap();
    let backup = journal.state_path.with_extension("json.previous");
    fs::rename(&journal.state_path, &backup).unwrap();
    let state = read_operation_state(&journal.state_path).unwrap();
    assert_eq!(state["operation_id"], journal.id);
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn resume_keeps_operation_identity_and_appends_to_the_same_log() {
    let layout = test_layout("resume");
    layout.ensure_base_dirs().unwrap();
    let app_dir = layout.app_dir.clone();
    let mut first =
        OperationJournal::create(layout, OperationKind::SelfUpdate, None, "latest").unwrap();
    first
        .record(Phase::Handoff, "Starting the replacement CLI", None)
        .unwrap();
    let id = first.id.clone();
    let log_path = first.log_path.clone();
    drop(first);

    let resumed = OperationJournal::resume(test_layout_at(app_dir), &id, None).unwrap();
    assert_eq!(resumed.id, id);
    assert_eq!(resumed.log_path, log_path);
    let events = fs::read_to_string(&resumed.log_path).unwrap();
    assert_eq!(events.lines().count(), 3);
    assert!(events.contains("Operation resumed in a new CLI process"));
    fs::remove_dir_all(&resumed.app_dir).unwrap();
}

#[test]
fn path_names_do_not_change_failure_classification() {
    let layout = test_layout("classification");
    layout.ensure_base_dirs().unwrap();
    let project = layout.app_dir.join("Downloads").join("missing-project");
    let journal =
        OperationJournal::create(layout, OperationKind::Install, Some(&project), "latest").unwrap();
    let error = format!("{} is not a Godot project", project.display());
    assert_eq!(journal.failure_code(&error), "FEN-INSTALL-FAILED");
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn typed_failure_class_produces_a_stable_operation_code() {
    let layout = test_layout("typed-error");
    layout.ensure_base_dirs().unwrap();
    let mut journal =
        OperationJournal::create(layout, OperationKind::Install, None, "latest").unwrap();
    journal
        .set_failure_class(FailureClass::HashMismatch)
        .unwrap();

    assert_eq!(
        journal.failure_code("wording can change freely"),
        "FEN-INSTALL-HASH-MISMATCH"
    );
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn records_structured_asset_hashes_and_rollback_state() {
    let layout = test_layout("artifact-state");
    layout.ensure_base_dirs().unwrap();
    let mut journal =
        OperationJournal::create(layout, OperationKind::Update, None, "1.2.3").unwrap();
    journal
        .select_asset("fennara-addon-v1.2.3.zip", Some("expected-hash"))
        .unwrap();
    journal
        .record_asset_hash("fennara-addon-v1.2.3.zip", "actual-hash", Some(false))
        .unwrap();
    journal
        .record(Phase::Verifying, "Hash verification failed", None)
        .unwrap();

    let state: Value =
        serde_json::from_str(&fs::read_to_string(&journal.state_path).unwrap()).unwrap();
    let artifact = &state["artifacts"]["fennara-addon-v1.2.3.zip"];
    assert_eq!(artifact["expected_sha256"], "expected-hash");
    assert_eq!(artifact["actual_sha256"], "actual-hash");
    assert_eq!(artifact["status"], "mismatch");
    assert_eq!(state["rollback_state"], "not_started");

    let last_event: Value = serde_json::from_str(
        fs::read_to_string(&journal.log_path)
            .unwrap()
            .lines()
            .last()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        last_event["artifacts"]["fennara-addon-v1.2.3.zip"]["status"],
        "mismatch"
    );
    assert_eq!(last_event["rollback_state"], "not_started");
    fs::remove_dir_all(&journal.app_dir).unwrap();
}

#[test]
fn later_journal_write_failure_is_returned_to_the_caller() {
    let layout = test_layout("journal-write-failure");
    layout.ensure_base_dirs().unwrap();
    let mut journal =
        OperationJournal::create(layout, OperationKind::Update, None, "latest").unwrap();
    let operations_dir = journal.state_path.parent().unwrap().to_path_buf();
    fs::remove_file(&journal.state_path).unwrap();
    fs::remove_dir(&operations_dir).unwrap();
    fs::write(&operations_dir, "blocks later state writes").unwrap();

    let error = journal
        .record(Phase::Staging, "Preparing to modify files", None)
        .unwrap_err();
    assert!(error.contains("failed to create"));

    fs::remove_file(operations_dir).unwrap();
    fs::remove_dir_all(&journal.app_dir).unwrap();
}
