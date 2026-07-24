use reqwest::Client;
use serde::Serialize;
use std::{
    env,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    sync::{mpsc, oneshot, watch},
    task::JoinHandle,
};

use super::{DAEMON_VERSION, util::fennara_app_dir};
use state::TelemetryState;

mod state;

const ENDPOINT: &str = "https://fennara.io/api/telemetry";
const EVENT_NAME: &str = "fennara_active_installation";
const PAYLOAD_SCHEMA_VERSION: u8 = 1;
const QUEUE_CAPACITY: usize = 4;
const CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(3);
const SHUTDOWN_TIMEOUT: Duration = Duration::from_millis(500);
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;

#[derive(Clone)]
pub(crate) struct TelemetryHandle {
    activity_tx: mpsc::Sender<Activity>,
    enabled_tx: watch::Sender<bool>,
}

pub(crate) struct TelemetryRuntime {
    handle: TelemetryHandle,
    shutdown_tx: Option<oneshot::Sender<()>>,
    worker: Option<JoinHandle<()>>,
}

#[derive(Clone, Debug)]
struct Activity {
    godot_version: String,
}

#[derive(Clone)]
struct WorkerConfig {
    endpoint: String,
    state_path: PathBuf,
    platform: &'static str,
    architecture: &'static str,
    check_interval: Duration,
}

#[derive(Serialize)]
struct ActiveInstallationEvent<'a> {
    schema_version: u8,
    event: &'static str,
    installation_id: &'a str,
    fennara_version: &'static str,
    godot_version: &'a str,
    platform: &'static str,
    architecture: &'static str,
}

impl TelemetryHandle {
    pub(crate) fn record_activity(&self, godot_version: &str) {
        let Some(godot_version) = normalize_godot_version(godot_version) else {
            return;
        };
        let _ = self.activity_tx.try_send(Activity { godot_version });
    }

    pub(crate) fn set_enabled(&self, enabled: bool) {
        let _ = self.enabled_tx.send(enabled);
    }
}

impl TelemetryRuntime {
    pub(crate) fn start(enabled: bool) -> Self {
        let Some(config) = production_config() else {
            return Self::disabled(enabled);
        };
        Self::start_with_config(enabled, config)
    }

    fn start_with_config(enabled: bool, config: WorkerConfig) -> Self {
        let (activity_tx, activity_rx) = mpsc::channel(QUEUE_CAPACITY);
        let (enabled_tx, enabled_rx) = watch::channel(enabled);
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let handle = TelemetryHandle {
            activity_tx,
            enabled_tx,
        };
        let worker = tokio::spawn(run_worker(config, activity_rx, enabled_rx, shutdown_rx));
        Self {
            handle,
            shutdown_tx: Some(shutdown_tx),
            worker: Some(worker),
        }
    }

    fn disabled(enabled: bool) -> Self {
        let (activity_tx, activity_rx) = mpsc::channel(1);
        drop(activity_rx);
        let (enabled_tx, enabled_rx) = watch::channel(enabled);
        drop(enabled_rx);
        Self {
            handle: TelemetryHandle {
                activity_tx,
                enabled_tx,
            },
            shutdown_tx: None,
            worker: None,
        }
    }

    pub(crate) fn handle(&self) -> TelemetryHandle {
        self.handle.clone()
    }

    pub(crate) async fn shutdown(mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
        let Some(mut worker) = self.worker.take() else {
            return;
        };
        if tokio::time::timeout(SHUTDOWN_TIMEOUT, &mut worker)
            .await
            .is_err()
        {
            worker.abort();
            let _ = worker.await;
        }
    }
}

fn production_config() -> Option<WorkerConfig> {
    let platform = match env::consts::OS {
        "windows" => "windows",
        "macos" => "macos",
        "linux" => "linux",
        _ => return None,
    };
    let architecture = match env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        _ => return None,
    };
    Some(WorkerConfig {
        endpoint: ENDPOINT.to_string(),
        state_path: fennara_app_dir().ok()?.join("telemetry").join("state.json"),
        platform,
        architecture,
        check_interval: CHECK_INTERVAL,
    })
}

async fn run_worker(
    config: WorkerConfig,
    mut activity_rx: mpsc::Receiver<Activity>,
    mut enabled_rx: watch::Receiver<bool>,
    mut shutdown_rx: oneshot::Receiver<()>,
) {
    let client = match Client::builder()
        .user_agent(format!("Fennara/{DAEMON_VERSION}"))
        .connect_timeout(Duration::from_secs(1))
        .timeout(REQUEST_TIMEOUT)
        .build()
    {
        Ok(client) => client,
        Err(_) => return,
    };
    let mut latest_activity: Option<Activity> = None;
    let mut state: Option<TelemetryState> = None;
    let mut interval = tokio::time::interval_at(
        tokio::time::Instant::now() + config.check_interval,
        config.check_interval,
    );
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    if !*enabled_rx.borrow() {
        remove_state(config.state_path.clone()).await;
    }

    loop {
        tokio::select! {
            _ = &mut shutdown_rx => break,
            changed = enabled_rx.changed() => {
                if changed.is_err() {
                    break;
                }
                let enabled = *enabled_rx.borrow_and_update();
                if enabled {
                    if let Some(activity) = latest_activity.as_ref() {
                        send_if_due(&client, &config, activity, &mut state, utc_day()).await;
                    }
                } else {
                    state = None;
                    remove_state(config.state_path.clone()).await;
                }
            }
            activity = activity_rx.recv() => {
                let Some(activity) = activity else {
                    break;
                };
                latest_activity = Some(activity);
                if *enabled_rx.borrow()
                    && let Some(activity) = latest_activity.as_ref()
                {
                    send_if_due(&client, &config, activity, &mut state, utc_day()).await;
                }
            }
            _ = interval.tick() => {
                if *enabled_rx.borrow()
                    && let Some(activity) = latest_activity.as_ref()
                {
                    send_if_due(&client, &config, activity, &mut state, utc_day()).await;
                }
            }
        }
    }
}

async fn send_if_due(
    client: &Client,
    config: &WorkerConfig,
    activity: &Activity,
    state: &mut Option<TelemetryState>,
    current_utc_day: u64,
) {
    if state.is_none() {
        let state_path = config.state_path.clone();
        *state = tokio::task::spawn_blocking(move || state::load_or_create(&state_path))
            .await
            .ok()
            .and_then(Result::ok);
    }
    let Some(current_state) = state.as_mut() else {
        return;
    };
    if current_state.last_sent_utc_day == Some(current_utc_day) {
        return;
    }

    let event = ActiveInstallationEvent {
        schema_version: PAYLOAD_SCHEMA_VERSION,
        event: EVENT_NAME,
        installation_id: &current_state.installation_id,
        fennara_version: DAEMON_VERSION,
        godot_version: &activity.godot_version,
        platform: config.platform,
        architecture: config.architecture,
    };
    let accepted = client
        .post(&config.endpoint)
        .json(&event)
        .send()
        .await
        .is_ok_and(|response| response.status().is_success());
    if accepted {
        current_state.last_sent_utc_day = Some(current_utc_day);
        let state_path = config.state_path.clone();
        let state_to_write = current_state.clone();
        let _ =
            tokio::task::spawn_blocking(move || state::write(&state_path, &state_to_write)).await;
    }
}

async fn remove_state(path: PathBuf) {
    let _ = tokio::task::spawn_blocking(move || state::remove(&path)).await;
}

fn normalize_godot_version(value: &str) -> Option<String> {
    let prefix: String = value
        .trim()
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '.')
        .collect();
    let parts: Vec<&str> = prefix.trim_end_matches('.').split('.').collect();
    if !(2..=3).contains(&parts.len())
        || parts
            .iter()
            .any(|part| part.is_empty() || part.len() > 3 || part.parse::<u16>().is_err())
    {
        return None;
    }
    Some(parts.join("."))
}

fn utc_day() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / SECONDS_PER_DAY
}

#[cfg(test)]
#[path = "telemetry/tests.rs"]
mod tests;
