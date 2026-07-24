use super::state::{
    SCHEMA_VERSION as STATE_SCHEMA_VERSION, generate_installation_id, is_valid_installation_id,
    load_or_create as load_or_create_state, read_valid as read_valid_state, write as write_state,
};
use super::*;
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde_json::Value;
use std::{
    fs,
    path::Path,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU16, AtomicU64, Ordering},
    },
};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
struct CaptureState {
    events: Arc<Mutex<Vec<Value>>>,
    status: Arc<AtomicU16>,
    delay_ms: Arc<AtomicU64>,
}

#[test]
fn normalizes_only_supported_godot_version_shapes() {
    assert_eq!(
        normalize_godot_version("4.6.3.stable.official"),
        Some("4.6.3".to_string())
    );
    assert_eq!(
        normalize_godot_version(" 4.5.stable "),
        Some("4.5".to_string())
    );
    assert_eq!(normalize_godot_version("4"), None);
    assert_eq!(normalize_godot_version("4.6.3.1"), None);
    assert_eq!(normalize_godot_version("custom"), None);
    assert_eq!(normalize_godot_version("4..6"), None);
}

#[test]
fn generated_installation_ids_are_valid_uuid_v4_values() {
    for _ in 0..128 {
        let installation_id = generate_installation_id().unwrap();
        assert!(is_valid_installation_id(&installation_id));
        assert_eq!(&installation_id[14..15], "4");
        assert!(matches!(&installation_id[19..20], "8" | "9" | "a" | "b"));
    }
    assert!(!is_valid_installation_id("machine-name"));
    assert!(!is_valid_installation_id(
        "550e8400-e29b-11d4-a716-446655440000"
    ));
}

#[test]
fn telemetry_state_is_persisted_reused_and_repaired() {
    let dir = test_dir("state");
    let path = dir.join("state.json");

    let first = load_or_create_state(&path).unwrap();
    let second = load_or_create_state(&path).unwrap();
    assert_eq!(first.installation_id, second.installation_id);
    assert!(is_valid_installation_id(&first.installation_id));

    fs::write(&path, r#"{"schema_version":1,"installation_id":"bad"}"#).unwrap();
    let repaired = load_or_create_state(&path).unwrap();
    assert!(is_valid_installation_id(&repaired.installation_id));
    assert_ne!(repaired.installation_id, first.installation_id);

    cleanup(&dir);
}

#[tokio::test]
async fn accepted_event_contains_only_the_approved_contract_and_sends_once_per_day() {
    let dir = test_dir("accepted");
    let (endpoint, capture, server) = capture_server(StatusCode::ACCEPTED).await;
    let config = test_config(endpoint, dir.join("state.json"));
    let client = Client::new();
    let activity = Activity {
        godot_version: "4.6.3".to_string(),
    };
    let mut state = None;

    send_if_due(&client, &config, &activity, &mut state, 42).await;
    send_if_due(&client, &config, &activity, &mut state, 42).await;

    let events = capture.events.lock().unwrap();
    assert_eq!(events.len(), 1);
    let event = events[0].as_object().unwrap();
    assert_eq!(event.len(), 7);
    assert_eq!(event["schema_version"], 1);
    assert_eq!(event["event"], EVENT_NAME);
    assert_eq!(event["fennara_version"], DAEMON_VERSION);
    assert_eq!(event["godot_version"], "4.6.3");
    assert_eq!(event["platform"], "windows");
    assert_eq!(event["architecture"], "x86_64");
    assert!(is_valid_installation_id(
        event["installation_id"].as_str().unwrap()
    ));
    drop(events);

    let persisted = read_valid_state(&config.state_path).unwrap().unwrap();
    assert_eq!(persisted.last_sent_utc_day, Some(42));

    server.abort();
    cleanup(&dir);
}

#[tokio::test]
async fn failed_delivery_is_not_marked_sent_and_retries() {
    let dir = test_dir("retry");
    let (endpoint, capture, server) = capture_server(StatusCode::BAD_GATEWAY).await;
    let config = test_config(endpoint, dir.join("state.json"));
    let client = Client::new();
    let activity = Activity {
        godot_version: "4.6".to_string(),
    };
    let mut state = None;

    send_if_due(&client, &config, &activity, &mut state, 77).await;
    assert_eq!(
        state.as_ref().and_then(|state| state.last_sent_utc_day),
        None
    );

    capture
        .status
        .store(StatusCode::ACCEPTED.as_u16(), Ordering::Relaxed);
    send_if_due(&client, &config, &activity, &mut state, 77).await;

    assert_eq!(capture.events.lock().unwrap().len(), 2);
    assert_eq!(
        state.as_ref().and_then(|state| state.last_sent_utc_day),
        Some(77)
    );

    server.abort();
    cleanup(&dir);
}

#[tokio::test]
async fn disabled_runtime_deletes_identity_and_does_not_send_until_enabled() {
    let dir = test_dir("toggle");
    let state_path = dir.join("state.json");
    let existing = TelemetryState {
        schema_version: STATE_SCHEMA_VERSION,
        installation_id: generate_installation_id().unwrap(),
        last_sent_utc_day: None,
    };
    write_state(&state_path, &existing).unwrap();
    let (endpoint, capture, server) = capture_server(StatusCode::ACCEPTED).await;
    let config = test_config(endpoint, state_path.clone());
    let runtime = TelemetryRuntime::start_with_config(false, config);
    let handle = runtime.handle();

    wait_until(|| !state_path.exists()).await;
    handle.record_activity("4.6.3.stable");
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert!(capture.events.lock().unwrap().is_empty());

    handle.set_enabled(true);
    wait_until(|| !capture.events.lock().unwrap().is_empty()).await;
    assert!(state_path.is_file());

    handle.set_enabled(false);
    wait_until(|| !state_path.exists()).await;

    runtime.shutdown().await;
    server.abort();
    cleanup(&dir);
}

#[test]
fn a_full_activity_queue_drops_new_signals_without_waiting() {
    let (activity_tx, mut activity_rx) = mpsc::channel(1);
    let (enabled_tx, _enabled_rx) = watch::channel(true);
    let handle = TelemetryHandle {
        activity_tx,
        enabled_tx,
    };

    handle.record_activity("4.6.3.stable");
    handle.record_activity("4.5.2.stable");

    assert_eq!(
        activity_rx.try_recv().unwrap().godot_version,
        "4.6.3".to_string()
    );
    assert!(activity_rx.try_recv().is_err());
}

#[tokio::test]
async fn shutdown_cancels_a_slow_request_within_its_bound() {
    let dir = test_dir("shutdown");
    let (endpoint, capture, server) = capture_server(StatusCode::ACCEPTED).await;
    capture.delay_ms.store(5_000, Ordering::Relaxed);
    let runtime =
        TelemetryRuntime::start_with_config(true, test_config(endpoint, dir.join("state.json")));
    runtime.handle().record_activity("4.6.3");
    wait_until(|| !capture.events.lock().unwrap().is_empty()).await;

    let started = std::time::Instant::now();
    runtime.shutdown().await;

    assert!(started.elapsed() < Duration::from_secs(2));
    server.abort();
    cleanup(&dir);
}

async fn capture_server(initial_status: StatusCode) -> (String, CaptureState, JoinHandle<()>) {
    async fn capture(State(state): State<CaptureState>, Json(event): Json<Value>) -> StatusCode {
        state.events.lock().unwrap().push(event);
        let delay_ms = state.delay_ms.load(Ordering::Relaxed);
        if delay_ms > 0 {
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        }
        StatusCode::from_u16(state.status.load(Ordering::Relaxed))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    let state = CaptureState {
        events: Arc::new(Mutex::new(Vec::new())),
        status: Arc::new(AtomicU16::new(initial_status.as_u16())),
        delay_ms: Arc::new(AtomicU64::new(0)),
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = Router::new()
        .route("/api/telemetry", post(capture))
        .with_state(state.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    (format!("http://{address}/api/telemetry"), state, server)
}

fn test_config(endpoint: String, state_path: PathBuf) -> WorkerConfig {
    WorkerConfig {
        endpoint,
        state_path,
        platform: "windows",
        architecture: "x86_64",
        check_interval: Duration::from_millis(10),
    }
}

fn test_dir(name: &str) -> PathBuf {
    let sequence = TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .join("..")
        .join("temp")
        .join("telemetry-tests");
    let dir = root.join(format!("{}-{name}-{sequence}", std::process::id()));
    cleanup(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn cleanup(path: &Path) {
    if path.starts_with(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("..")
            .join("..")
            .join("temp"),
    ) {
        let _ = fs::remove_dir_all(path);
    }
}

async fn wait_until(predicate: impl Fn() -> bool) {
    for _ in 0..100 {
        if predicate() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("condition did not become true before timeout");
}
