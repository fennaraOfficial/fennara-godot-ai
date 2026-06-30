use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicU64, Ordering},
        mpsc::{Receiver, RecvTimeoutError, SyncSender, TryRecvError, TrySendError, sync_channel},
    },
    thread,
    time::{Duration, Instant},
};

use super::{
    ids::{new_id, now_ms},
    schema::{to_store_error, trace_connection},
};

const TRACE_RETENTION_MS: i64 = 14 * 24 * 60 * 60 * 1000;
const TRACE_MAX_EVENTS: i64 = 20_000;
const DEFAULT_TRACE_LIMIT: i64 = 500;
const MAX_TRACE_LIMIT: i64 = 2_000;
const MAX_DATA_JSON_BYTES: usize = 8 * 1024;
const TRACE_QUEUE_CAPACITY: usize = 4_096;
const TRACE_BATCH_MAX: usize = 64;
const TRACE_FLUSH_INTERVAL: Duration = Duration::from_millis(50);
const TRACE_CONNECT_RETRY_INITIAL: Duration = Duration::from_millis(50);
const TRACE_CONNECT_RETRY_MAX: Duration = Duration::from_secs(2);

static TRACE_WRITER_RECOVERY_COUNT: AtomicU64 = AtomicU64::new(0);
static TRACE_WRITER_DROPPED_EVENT_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Default)]
pub(crate) struct TraceContext {
    pub(crate) chat_id: Option<String>,
    pub(crate) request_id: Option<String>,
    pub(crate) generation_id: Option<String>,
    pub(crate) assistant_message_id: Option<String>,
    pub(crate) provider_attempt_id: Option<String>,
    pub(crate) tool_call_id: Option<String>,
    pub(crate) provisional_tool_id: Option<String>,
    pub(crate) approval_id: Option<String>,
    pub(crate) bridge_request_id: Option<String>,
    pub(crate) godot_session_id: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct TraceRecorder {
    inner: Arc<TraceRecorderInner>,
    context: TraceContext,
}

#[derive(Debug)]
struct TraceRecorderInner {
    trace_id: String,
    turn_id: String,
    dropped_events: AtomicU64,
}

impl TraceRecorder {
    pub(crate) fn new(
        chat_id: impl Into<String>,
        request_id: Option<String>,
        godot_session_id: Option<String>,
    ) -> Self {
        Self {
            inner: Arc::new(TraceRecorderInner {
                trace_id: new_id("trace"),
                turn_id: new_id("turn"),
                dropped_events: AtomicU64::new(0),
            }),
            context: TraceContext {
                chat_id: Some(chat_id.into()),
                request_id,
                godot_session_id,
                ..TraceContext::default()
            },
        }
    }

    pub(crate) fn trace_id(&self) -> &str {
        &self.inner.trace_id
    }

    pub(crate) fn turn_id(&self) -> &str {
        &self.inner.turn_id
    }

    pub(crate) fn with_generation(
        &self,
        generation_id: impl Into<String>,
        assistant_message_id: impl Into<String>,
    ) -> Self {
        self.with_context(|context| {
            context.generation_id = Some(generation_id.into());
            context.assistant_message_id = Some(assistant_message_id.into());
        })
    }

    pub(crate) fn with_provider_attempt(&self, provider_attempt_id: impl Into<String>) -> Self {
        self.with_context(|context| {
            context.provider_attempt_id = Some(provider_attempt_id.into());
        })
    }

    pub(crate) fn with_tool_call(&self, tool_call_id: impl Into<String>) -> Self {
        self.with_context(|context| {
            context.tool_call_id = Some(tool_call_id.into());
        })
    }

    pub(crate) fn with_provisional_tool(&self, provisional_tool_id: impl Into<String>) -> Self {
        self.with_context(|context| {
            context.provisional_tool_id = Some(provisional_tool_id.into());
        })
    }

    pub(crate) fn with_approval(&self, approval_id: impl Into<String>) -> Self {
        self.with_context(|context| {
            context.approval_id = Some(approval_id.into());
        })
    }

    pub(crate) fn with_bridge_request(
        &self,
        bridge_request_id: impl Into<String>,
        godot_session_id: impl Into<String>,
    ) -> Self {
        self.with_context(|context| {
            context.bridge_request_id = Some(bridge_request_id.into());
            context.godot_session_id = Some(godot_session_id.into());
        })
    }

    pub(crate) fn event(&self, name: &str, data: Value) {
        self.record(name, "info", None, None, data);
    }

    pub(crate) fn event_status(&self, name: &str, status: &str, data: Value) {
        self.record(name, "info", Some(status), None, data);
    }

    pub(crate) fn warn(&self, name: &str, status: &str, data: Value) {
        self.record(name, "warn", Some(status), None, data);
    }

    pub(crate) fn error(&self, name: &str, status: &str, data: Value) {
        self.record(name, "error", Some(status), None, data);
    }

    pub(crate) fn start_span(&self, base_name: &str, data: Value) -> TraceSpan {
        self.event(&format!("{base_name}.start"), data);
        TraceSpan {
            recorder: self.clone(),
            base_name: base_name.to_string(),
            started_at: Instant::now(),
        }
    }

    fn with_context(&self, update: impl FnOnce(&mut TraceContext)) -> Self {
        let mut context = self.context.clone();
        update(&mut context);
        Self {
            inner: Arc::clone(&self.inner),
            context,
        }
    }

    fn record(
        &self,
        name: &str,
        level: &str,
        status: Option<&str>,
        duration_ms: Option<i64>,
        data: Value,
    ) {
        let dropped_event_count = self.inner.dropped_events.swap(0, Ordering::Relaxed);
        let writer_dropped_event_count =
            TRACE_WRITER_DROPPED_EVENT_COUNT.swap(0, Ordering::Relaxed);
        let writer_recovery_count = TRACE_WRITER_RECOVERY_COUNT.swap(0, Ordering::Relaxed);
        let event = self.build_event_with_counts(
            name,
            level,
            status,
            duration_ms,
            data,
            dropped_event_count,
            writer_dropped_event_count,
            writer_recovery_count,
        );
        match trace_writer().try_send(event) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) | Err(TrySendError::Disconnected(_)) => {
                self.inner
                    .dropped_events
                    .fetch_add(dropped_event_count.saturating_add(1), Ordering::Relaxed);
            }
        }
    }

    #[cfg(test)]
    fn build_event(
        &self,
        name: &str,
        level: &str,
        status: Option<&str>,
        duration_ms: Option<i64>,
        data: Value,
    ) -> QueuedTraceEvent {
        self.build_event_with_counts(name, level, status, duration_ms, data, 0, 0, 0)
    }

    fn build_event_with_counts(
        &self,
        name: &str,
        level: &str,
        status: Option<&str>,
        duration_ms: Option<i64>,
        data: Value,
        dropped_event_count: u64,
        writer_dropped_event_count: u64,
        writer_recovery_count: u64,
    ) -> QueuedTraceEvent {
        QueuedTraceEvent {
            id: new_id("trace_event"),
            trace_id: self.inner.trace_id.clone(),
            turn_id: self.inner.turn_id.clone(),
            context: self.context.clone(),
            name: name.to_string(),
            level: level.to_string(),
            status: status.map(ToOwned::to_owned),
            ts_ms: now_ms(),
            duration_ms,
            data_json: bounded_data_json(with_trace_counts(
                data,
                dropped_event_count,
                writer_dropped_event_count,
                writer_recovery_count,
            )),
        }
    }
}

pub(crate) struct TraceSpan {
    recorder: TraceRecorder,
    base_name: String,
    started_at: Instant,
}

impl TraceSpan {
    pub(crate) fn finish(self, status: &str, data: Value) {
        let duration_ms = self.started_at.elapsed().as_millis() as i64;
        self.recorder.record(
            &format!("{}.end", self.base_name),
            "info",
            Some(status),
            Some(duration_ms),
            data,
        );
    }

    pub(crate) fn fail(self, data: Value) {
        let duration_ms = self.started_at.elapsed().as_millis() as i64;
        self.recorder.record(
            &format!("{}.end", self.base_name),
            "error",
            Some("failed"),
            Some(duration_ms),
            data,
        );
    }
}

#[derive(Debug)]
struct QueuedTraceEvent {
    id: String,
    trace_id: String,
    turn_id: String,
    context: TraceContext,
    name: String,
    level: String,
    status: Option<String>,
    ts_ms: i64,
    duration_ms: Option<i64>,
    data_json: String,
}

#[derive(Debug)]
struct PersistedTraceEvent {
    id: String,
    seq: i64,
    trace_id: String,
    turn_id: String,
    context: TraceContext,
    name: String,
    level: String,
    status: Option<String>,
    ts_ms: i64,
    duration_ms: Option<i64>,
    data_json: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TraceEventRecord {
    pub(crate) id: String,
    pub(crate) seq: i64,
    pub(crate) trace_id: String,
    pub(crate) turn_id: String,
    pub(crate) chat_id: Option<String>,
    pub(crate) request_id: Option<String>,
    pub(crate) generation_id: Option<String>,
    pub(crate) assistant_message_id: Option<String>,
    pub(crate) provider_attempt_id: Option<String>,
    pub(crate) tool_call_id: Option<String>,
    pub(crate) provisional_tool_id: Option<String>,
    pub(crate) approval_id: Option<String>,
    pub(crate) bridge_request_id: Option<String>,
    pub(crate) godot_session_id: Option<String>,
    pub(crate) name: String,
    pub(crate) level: String,
    pub(crate) status: Option<String>,
    pub(crate) ts_ms: i64,
    pub(crate) duration_ms: Option<i64>,
    pub(crate) data: Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TraceQuery {
    pub(crate) chat_id: Option<String>,
    pub(crate) trace_id: Option<String>,
    pub(crate) turn_id: Option<String>,
    pub(crate) generation_id: Option<String>,
    pub(crate) limit: Option<i64>,
}

impl TraceQuery {
    pub(crate) fn has_filter(&self) -> bool {
        self.chat_id
            .as_deref()
            .or(self.trace_id.as_deref())
            .or(self.turn_id.as_deref())
            .or(self.generation_id.as_deref())
            .is_some_and(|value| !value.trim().is_empty())
    }

    fn clean_limit(&self) -> i64 {
        self.limit
            .unwrap_or(DEFAULT_TRACE_LIMIT)
            .clamp(1, MAX_TRACE_LIMIT)
    }
}

pub(crate) fn list_events(query: &TraceQuery) -> Result<Vec<TraceEventRecord>, String> {
    let conn = trace_connection()?;
    list_events_conn(&conn, query)
}

static TRACE_WRITER: OnceLock<SyncSender<QueuedTraceEvent>> = OnceLock::new();

#[derive(Debug, Default)]
struct TraceWriterState {
    next_seq_by_trace: HashMap<String, i64>,
    inserted_since_prune: i64,
    pruned_once: bool,
}

fn trace_writer() -> &'static SyncSender<QueuedTraceEvent> {
    TRACE_WRITER.get_or_init(|| {
        let (sender, receiver) = sync_channel(TRACE_QUEUE_CAPACITY);
        let _ = thread::Builder::new()
            .name("fennara-trace-writer".to_string())
            .spawn(move || trace_writer_loop(receiver));
        sender
    })
}

fn trace_writer_loop(receiver: Receiver<QueuedTraceEvent>) {
    let mut conn = open_trace_connection_with_retry();
    let mut state = TraceWriterState::default();
    let mut batch = Vec::with_capacity(TRACE_BATCH_MAX);
    loop {
        match receiver.recv_timeout(TRACE_FLUSH_INTERVAL) {
            Ok(event) => batch.push(event),
            Err(RecvTimeoutError::Timeout) => continue,
            Err(RecvTimeoutError::Disconnected) => break,
        }

        let mut disconnected = false;
        while batch.len() < TRACE_BATCH_MAX {
            match receiver.try_recv() {
                Ok(event) => batch.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    disconnected = true;
                    break;
                }
            }
        }

        if write_event_batch_conn(&mut conn, &mut state, &batch, now_ms()).is_err() {
            TRACE_WRITER_DROPPED_EVENT_COUNT.fetch_add(batch.len() as u64, Ordering::Relaxed);
            conn = open_trace_connection_with_retry();
        }
        batch.clear();
        if disconnected {
            break;
        }
    }
}

fn open_trace_connection_with_retry() -> Connection {
    let mut delay = TRACE_CONNECT_RETRY_INITIAL;
    let mut failures = 0u64;
    loop {
        match trace_connection() {
            Ok(conn) => {
                if failures > 0 {
                    TRACE_WRITER_RECOVERY_COUNT.fetch_add(failures, Ordering::Relaxed);
                }
                return conn;
            }
            Err(_) => {
                failures = failures.saturating_add(1);
                thread::sleep(delay);
                delay = delay.saturating_mul(2).min(TRACE_CONNECT_RETRY_MAX);
            }
        }
    }
}

fn write_event_batch_conn(
    conn: &mut Connection,
    state: &mut TraceWriterState,
    events: &[QueuedTraceEvent],
    now: i64,
) -> Result<(), String> {
    if events.is_empty() {
        return Ok(());
    }
    let tx = conn.transaction().map_err(to_store_error)?;
    let mut next_seq_by_trace = state.next_seq_by_trace.clone();
    for event in events {
        let next_seq = next_seq_for_trace(&tx, &mut next_seq_by_trace, event.trace_id.as_str())?;
        let persisted = PersistedTraceEvent {
            id: event.id.clone(),
            seq: next_seq,
            trace_id: event.trace_id.clone(),
            turn_id: event.turn_id.clone(),
            context: event.context.clone(),
            name: event.name.clone(),
            level: event.level.clone(),
            status: event.status.clone(),
            ts_ms: event.ts_ms,
            duration_ms: event.duration_ms,
            data_json: event.data_json.clone(),
        };
        insert_event_conn(&tx, &persisted)?;
    }
    tx.commit().map_err(to_store_error)?;
    state.next_seq_by_trace = next_seq_by_trace;
    state.inserted_since_prune += events.len() as i64;
    if !state.pruned_once || state.inserted_since_prune >= 100 {
        let _ = prune_events_conn(conn, now);
        state.pruned_once = true;
        state.inserted_since_prune = 0;
    }
    Ok(())
}

fn next_seq_for_trace(
    conn: &Connection,
    next_seq_by_trace: &mut HashMap<String, i64>,
    trace_id: &str,
) -> Result<i64, String> {
    if !next_seq_by_trace.contains_key(trace_id) {
        let max_seq = conn
            .query_row(
                "SELECT COALESCE(MAX(seq), 0) FROM chat_trace_events WHERE trace_id = ?1",
                [trace_id],
                |row| row.get::<_, i64>(0),
            )
            .map_err(to_store_error)?;
        next_seq_by_trace.insert(trace_id.to_string(), max_seq);
    }
    let seq = next_seq_by_trace
        .get_mut(trace_id)
        .expect("trace seq initialized");
    *seq += 1;
    Ok(*seq)
}

fn insert_event_conn(conn: &Connection, event: &PersistedTraceEvent) -> Result<(), String> {
    conn.execute(
        "INSERT INTO chat_trace_events
         (id, seq, trace_id, turn_id, chat_id, request_id, generation_id,
          assistant_message_id, provider_attempt_id, tool_call_id,
          provisional_tool_id, approval_id, bridge_request_id, godot_session_id,
          name, level, status, ts_ms, duration_ms, data_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                 ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
        params![
            event.id.as_str(),
            event.seq,
            event.trace_id.as_str(),
            event.turn_id.as_str(),
            event.context.chat_id.as_deref(),
            event.context.request_id.as_deref(),
            event.context.generation_id.as_deref(),
            event.context.assistant_message_id.as_deref(),
            event.context.provider_attempt_id.as_deref(),
            event.context.tool_call_id.as_deref(),
            event.context.provisional_tool_id.as_deref(),
            event.context.approval_id.as_deref(),
            event.context.bridge_request_id.as_deref(),
            event.context.godot_session_id.as_deref(),
            event.name.as_str(),
            event.level.as_str(),
            event.status.as_deref(),
            event.ts_ms,
            event.duration_ms,
            event.data_json.as_str(),
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}

fn list_events_conn(
    conn: &Connection,
    query: &TraceQuery,
) -> Result<Vec<TraceEventRecord>, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, seq, trace_id, turn_id, chat_id, request_id, generation_id,
                    assistant_message_id, provider_attempt_id, tool_call_id,
                    provisional_tool_id, approval_id, bridge_request_id,
                    godot_session_id, name, level, status, ts_ms, duration_ms,
                    data_json
             FROM (
               SELECT id, seq, trace_id, turn_id, chat_id, request_id, generation_id,
                      assistant_message_id, provider_attempt_id, tool_call_id,
                      provisional_tool_id, approval_id, bridge_request_id,
                      godot_session_id, name, level, status, ts_ms, duration_ms,
                      data_json
               FROM chat_trace_events
               WHERE (?1 IS NULL OR chat_id = ?1)
                 AND (?2 IS NULL OR trace_id = ?2)
                 AND (?3 IS NULL OR turn_id = ?3)
                 AND (?4 IS NULL OR generation_id = ?4)
               ORDER BY ts_ms DESC, trace_id DESC, seq DESC, id DESC
               LIMIT ?5
             )
             ORDER BY ts_ms ASC, trace_id ASC, seq ASC, id ASC",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map(
            params![
                query.chat_id.as_deref(),
                query.trace_id.as_deref(),
                query.turn_id.as_deref(),
                query.generation_id.as_deref(),
                query.clean_limit(),
            ],
            trace_event_from_row,
        )
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(to_store_error)
}

fn prune_events_conn(conn: &Connection, now: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM chat_trace_events WHERE ts_ms < ?1",
        [now.saturating_sub(TRACE_RETENTION_MS)],
    )
    .map_err(to_store_error)?;
    conn.execute(
        "DELETE FROM chat_trace_events
         WHERE id IN (
           SELECT id FROM chat_trace_events
           ORDER BY ts_ms DESC, seq DESC
           LIMIT -1 OFFSET ?1
         )",
        [TRACE_MAX_EVENTS],
    )
    .map_err(to_store_error)?;
    Ok(())
}

fn trace_event_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TraceEventRecord> {
    let data_json: String = row.get(19)?;
    let data = serde_json::from_str::<Value>(&data_json).unwrap_or_else(|_| json!({}));
    Ok(TraceEventRecord {
        id: row.get(0)?,
        seq: row.get(1)?,
        trace_id: row.get(2)?,
        turn_id: row.get(3)?,
        chat_id: row.get(4)?,
        request_id: row.get(5)?,
        generation_id: row.get(6)?,
        assistant_message_id: row.get(7)?,
        provider_attempt_id: row.get(8)?,
        tool_call_id: row.get(9)?,
        provisional_tool_id: row.get(10)?,
        approval_id: row.get(11)?,
        bridge_request_id: row.get(12)?,
        godot_session_id: row.get(13)?,
        name: row.get(14)?,
        level: row.get(15)?,
        status: row.get(16)?,
        ts_ms: row.get(17)?,
        duration_ms: row.get(18)?,
        data,
    })
}

fn bounded_data_json(data: Value) -> String {
    let trace_counts = [
        "trace_dropped_event_count",
        "trace_writer_dropped_event_count",
        "trace_writer_recovery_count",
    ]
    .into_iter()
    .filter_map(|key| {
        data.get(key)
            .and_then(Value::as_u64)
            .map(|count| (key, count))
    })
    .collect::<Vec<_>>();
    match serde_json::to_string(&data) {
        Ok(raw) if raw.len() <= MAX_DATA_JSON_BYTES => raw,
        Ok(raw) => {
            let mut summary = json!({
                "truncated": true,
                "original_json_bytes": raw.len()
            });
            if let Some(object) = summary.as_object_mut() {
                for (key, count) in trace_counts {
                    object.insert(key.to_string(), json!(count));
                }
            }
            summary.to_string()
        }
        Err(error) => json!({
            "serialization_error": error.to_string()
        })
        .to_string(),
    }
}

fn with_trace_counts(
    data: Value,
    dropped_event_count: u64,
    writer_dropped_event_count: u64,
    writer_recovery_count: u64,
) -> Value {
    let data = with_optional_count(data, "trace_dropped_event_count", dropped_event_count);
    let data = with_optional_count(
        data,
        "trace_writer_dropped_event_count",
        writer_dropped_event_count,
    );
    with_optional_count(data, "trace_writer_recovery_count", writer_recovery_count)
}

fn with_optional_count(mut data: Value, key: &str, count: u64) -> Value {
    if count == 0 {
        return data;
    }
    match data.as_object_mut() {
        Some(object) => {
            object.insert(key.to_string(), Value::Number(count.into()));
            data
        }
        None => {
            let mut object = serde_json::Map::new();
            object.insert("value".to_string(), data);
            object.insert(key.to_string(), json!(count));
            Value::Object(object)
        }
    }
}

pub(crate) fn value_size(value: &Value) -> usize {
    serde_json::to_string(value)
        .map(|raw| raw.len())
        .unwrap_or_default()
}

pub(crate) fn finish_reason_label(reason: &super::providers::FinishReason) -> String {
    match reason {
        super::providers::FinishReason::Stop => "stop".to_string(),
        super::providers::FinishReason::Length => "length".to_string(),
        super::providers::FinishReason::ToolCalls => "tool_calls".to_string(),
        super::providers::FinishReason::ContentFilter => "content_filter".to_string(),
        super::providers::FinishReason::Cancelled => "cancelled".to_string(),
        super::providers::FinishReason::Unknown(value) => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::schema::create_trace_tables;

    #[test]
    fn insert_and_list_trace_events_in_sequence_order() {
        let mut conn = Connection::open_in_memory().unwrap();
        create_trace_tables(&conn).unwrap();
        let mut state = TraceWriterState::default();
        let recorder = TraceRecorder::new(
            "chat_1",
            Some("request_1".to_string()),
            Some("session_1".to_string()),
        )
        .with_generation("gen_1", "msg_1");
        let first = recorder.build_event("turn.start", "info", None, None, json!({}));
        let second = recorder.build_event(
            "generation.done",
            "info",
            Some("done"),
            Some(12),
            json!({ "ok": true }),
        );

        write_event_batch_conn(&mut conn, &mut state, &[first, second], now_ms()).unwrap();

        let events = list_events_conn(
            &conn,
            &TraceQuery {
                chat_id: Some("chat_1".to_string()),
                trace_id: None,
                turn_id: None,
                generation_id: None,
                limit: None,
            },
        )
        .unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].name, "turn.start");
        assert_eq!(events[1].name, "generation.done");
        assert_eq!(events[0].seq, 1);
        assert_eq!(events[1].seq, 2);
        assert_eq!(events[1].duration_ms, Some(12));
        assert_eq!(events[1].generation_id.as_deref(), Some("gen_1"));
    }

    #[test]
    fn writer_assigns_seq_without_gaps_for_inserted_events() {
        let mut conn = Connection::open_in_memory().unwrap();
        create_trace_tables(&conn).unwrap();
        let mut state = TraceWriterState::default();
        let recorder = TraceRecorder::new("chat_1", None, None);
        let first = recorder.build_event("first", "info", None, None, json!({}));
        let dropped = recorder.build_event("dropped", "info", None, None, json!({}));
        let third = recorder.build_event("third", "info", None, None, json!({}));
        drop(dropped);

        write_event_batch_conn(&mut conn, &mut state, &[first, third], now_ms()).unwrap();

        let events = list_events_conn(
            &conn,
            &TraceQuery {
                chat_id: Some("chat_1".to_string()),
                trace_id: None,
                turn_id: None,
                generation_id: None,
                limit: None,
            },
        )
        .unwrap();

        assert_eq!(
            events.iter().map(|event| event.seq).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            events
                .iter()
                .map(|event| event.name.as_str())
                .collect::<Vec<_>>(),
            vec!["first", "third"]
        );
    }

    #[test]
    fn list_events_returns_latest_limited_events_in_chronological_order() {
        let mut conn = Connection::open_in_memory().unwrap();
        create_trace_tables(&conn).unwrap();
        let mut state = TraceWriterState::default();
        let recorder = TraceRecorder::new("chat_1", None, None);
        let mut first = recorder.build_event("first", "info", None, None, json!({}));
        let mut second = recorder.build_event("second", "info", None, None, json!({}));
        let mut third = recorder.build_event("third", "info", None, None, json!({}));
        let now = now_ms();
        first.ts_ms = now;
        second.ts_ms = now + 1;
        third.ts_ms = now + 2;

        write_event_batch_conn(&mut conn, &mut state, &[first, second, third], now).unwrap();

        let events = list_events_conn(
            &conn,
            &TraceQuery {
                chat_id: Some("chat_1".to_string()),
                trace_id: None,
                turn_id: None,
                generation_id: None,
                limit: Some(2),
            },
        )
        .unwrap();

        assert_eq!(
            events
                .iter()
                .map(|event| event.name.as_str())
                .collect::<Vec<_>>(),
            vec!["second", "third"]
        );
        assert_eq!(
            events.iter().map(|event| event.ts_ms).collect::<Vec<_>>(),
            vec![now + 1, now + 2]
        );
    }

    #[test]
    fn best_effort_recorder_ignores_insert_failures() {
        let mut conn = Connection::open_in_memory().unwrap();
        let mut state = TraceWriterState::default();
        let recorder = TraceRecorder::new("chat_1", None, None);
        let event = recorder.build_event("turn.start", "info", None, None, json!({}));

        assert!(write_event_batch_conn(&mut conn, &mut state, &[event], now_ms()).is_err());
        recorder.event("turn.start", json!({ "still": "does not panic" }));
    }

    #[test]
    fn oversized_data_is_replaced_with_size_summary() {
        let raw = "x".repeat(MAX_DATA_JSON_BYTES + 1);
        let data = bounded_data_json(json!({ "raw": raw }));
        let value = serde_json::from_str::<Value>(&data).unwrap();

        assert_eq!(value.get("truncated").and_then(Value::as_bool), Some(true));
        assert!(
            value
                .get("original_json_bytes")
                .and_then(Value::as_u64)
                .is_some()
        );
        assert!(value.get("raw").is_none());
    }

    #[test]
    fn dropped_event_count_is_added_to_object_payloads() {
        let value = with_trace_counts(json!({ "ok": true }), 3, 0, 0);

        assert_eq!(
            value
                .get("trace_dropped_event_count")
                .and_then(Value::as_u64),
            Some(3)
        );
    }

    #[test]
    fn writer_recovery_counts_are_added_to_payloads() {
        let value = with_trace_counts(json!({ "ok": true }), 0, 4, 2);

        assert_eq!(
            value
                .get("trace_writer_dropped_event_count")
                .and_then(Value::as_u64),
            Some(4)
        );
        assert_eq!(
            value
                .get("trace_writer_recovery_count")
                .and_then(Value::as_u64),
            Some(2)
        );
    }

    #[test]
    fn trace_counts_survive_oversized_data_summary() {
        let raw = "x".repeat(MAX_DATA_JSON_BYTES + 1);
        let data = bounded_data_json(with_trace_counts(json!({ "raw": raw }), 1, 2, 3));
        let value = serde_json::from_str::<Value>(&data).unwrap();

        assert_eq!(
            value
                .get("trace_dropped_event_count")
                .and_then(Value::as_u64),
            Some(1)
        );
        assert_eq!(
            value
                .get("trace_writer_dropped_event_count")
                .and_then(Value::as_u64),
            Some(2)
        );
        assert_eq!(
            value
                .get("trace_writer_recovery_count")
                .and_then(Value::as_u64),
            Some(3)
        );
    }
}
