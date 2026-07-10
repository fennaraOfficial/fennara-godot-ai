use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::Serialize;

use super::super::{
    checkpoints::{CaptureResult, CaptureUnavailableReason, CheckpointCoverage, SkippedPath},
    ids::now_ms,
    schema::{connection, to_store_error},
};

#[derive(Clone, Debug)]
pub(crate) struct RecoverableTurnCheckpoint {
    pub(crate) id: String,
    pub(crate) chat_id: String,
    pub(crate) user_message_id: String,
    pub(crate) project_path: String,
    pub(crate) storage_key: String,
    pub(crate) start_snapshot_id: Option<String>,
    pub(crate) end_snapshot_id: Option<String>,
    pub(crate) changed_paths: Vec<String>,
    pub(crate) boundary_sequence: i64,
    pub(crate) capture: CaptureResult,
}

#[derive(Clone, Debug)]
pub(crate) struct TurnRecoveryJournal {
    pub(crate) chat_id: String,
    pub(crate) checkpoint_id: String,
    pub(crate) user_message_id: String,
    pub(crate) operation_id: String,
    pub(crate) state: String,
    pub(crate) boundary_sequence: i64,
    pub(crate) project_path: String,
    pub(crate) storage_key: String,
    pub(crate) start_snapshot_id: Option<String>,
    pub(crate) end_snapshot_id: Option<String>,
    pub(crate) changed_paths: Vec<String>,
    pub(crate) capture: CaptureResult,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TurnRecoveryStatus {
    pub(crate) eligible_user_message_id: Option<String>,
    pub(crate) can_undo: bool,
    pub(crate) can_redo: bool,
    pub(crate) coverage: Option<CheckpointCoverage>,
    pub(crate) changed_file_count: usize,
    pub(crate) skipped_paths: Vec<SkippedPath>,
    pub(crate) operation_state: Option<String>,
}

impl TurnRecoveryStatus {
    pub(super) fn unavailable() -> Self {
        Self {
            eligible_user_message_id: None,
            can_undo: false,
            can_redo: false,
            coverage: None,
            changed_file_count: 0,
            skipped_paths: Vec::new(),
            operation_state: None,
        }
    }
}

pub(crate) fn recoverable_turn_checkpoint(
    chat_id: &str,
    user_message_id: &str,
) -> Result<RecoverableTurnCheckpoint, String> {
    let conn = connection()?;
    if recovery_journal_on_connection(&conn, chat_id)?.is_some() {
        return Err("This chat already has a rewound turn.".to_string());
    }
    let checkpoint = latest_recoverable_turn_checkpoint_on_connection(&conn, chat_id)?
        .ok_or_else(|| "No completed turn checkpoint is available for this chat.".to_string())?;
    if checkpoint.user_message_id != user_message_id {
        return Err("Only the latest recoverable user turn can be undone.".to_string());
    }
    Ok(checkpoint)
}

pub(crate) fn recovery_journal(chat_id: &str) -> Result<Option<TurnRecoveryJournal>, String> {
    recovery_journal_on_connection(&connection()?, chat_id)
}

pub(super) fn turn_recovery_status_on_connection(
    conn: &Connection,
    chat_id: &str,
) -> Result<TurnRecoveryStatus, String> {
    if let Some(journal) = recovery_journal_on_connection(conn, chat_id)? {
        return Ok(TurnRecoveryStatus {
            eligible_user_message_id: Some(journal.user_message_id),
            can_undo: false,
            can_redo: journal.state == "undone",
            coverage: Some(journal.capture.coverage),
            changed_file_count: journal.changed_paths.len(),
            skipped_paths: journal.capture.skipped_paths,
            operation_state: Some(journal.state),
        });
    }
    let Some(checkpoint) = latest_recoverable_turn_checkpoint_on_connection(conn, chat_id)? else {
        return Ok(TurnRecoveryStatus::unavailable());
    };
    Ok(TurnRecoveryStatus {
        eligible_user_message_id: Some(checkpoint.user_message_id),
        can_undo: true,
        can_redo: false,
        coverage: Some(checkpoint.capture.coverage),
        changed_file_count: checkpoint.changed_paths.len(),
        skipped_paths: checkpoint.capture.skipped_paths,
        operation_state: None,
    })
}

pub(crate) fn pending_recovery_journals() -> Result<Vec<TurnRecoveryJournal>, String> {
    let conn = connection()?;
    let mut statement = conn
        .prepare(
            "SELECT chat_id, checkpoint_id, user_message_id, operation_id, state, boundary_sequence,
                    project_path, storage_key, start_snapshot_id, end_snapshot_id,
                    changed_paths_json, capture_json
             FROM chat_turn_recovery
             WHERE state IN ('applying_undo', 'applying_redo')
             ORDER BY started_at_ms, chat_id",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map([], recovery_journal_from_row)
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(to_store_error)?
        .into_iter()
        .map(parse_recovery_paths)
        .collect()
}

pub(crate) fn begin_turn_undo(
    checkpoint: &RecoverableTurnCheckpoint,
    operation_id: &str,
) -> Result<TurnRecoveryJournal, String> {
    let mut conn = connection()?;
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    if recovery_journal_on_connection(&tx, &checkpoint.chat_id)?.is_some() {
        return Err("This chat already has a rewound turn.".to_string());
    }
    let changed_paths_json =
        serde_json::to_string(&checkpoint.changed_paths).map_err(|error| error.to_string())?;
    let capture_json =
        serde_json::to_string(&checkpoint.capture).map_err(|error| error.to_string())?;
    let now = now_ms();
    let held = tx
        .execute(
            "UPDATE chat_turn_checkpoints
             SET status = 'held'
             WHERE id = ?1 AND status IN ('complete', 'unavailable', 'interrupted')",
            [&checkpoint.id],
        )
        .map_err(to_store_error)?;
    if held != 1 {
        return Err("That turn checkpoint is no longer recoverable.".to_string());
    }
    tx.execute(
        "INSERT INTO chat_turn_recovery
         (chat_id, checkpoint_id, user_message_id, operation_id, state, boundary_sequence,
          project_path, storage_key, start_snapshot_id, end_snapshot_id,
          changed_paths_json, capture_json, started_at_ms, updated_at_ms)
         VALUES (?1, ?2, ?3, ?4, 'applying_undo', ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?12)",
        params![
            checkpoint.chat_id,
            checkpoint.id,
            checkpoint.user_message_id,
            operation_id,
            checkpoint.boundary_sequence,
            checkpoint.project_path,
            checkpoint.storage_key,
            checkpoint.start_snapshot_id,
            checkpoint.end_snapshot_id,
            changed_paths_json,
            capture_json,
            now,
        ],
    )
    .map_err(to_store_error)?;
    tx.commit().map_err(to_store_error)?;
    recovery_journal(&checkpoint.chat_id)?
        .ok_or_else(|| "Recovery journal was not saved.".to_string())
}

pub(crate) fn finish_turn_undo(chat_id: &str, operation_id: &str) -> Result<(), String> {
    let updated = connection()?
        .execute(
            "UPDATE chat_turn_recovery
             SET state = 'undone', updated_at_ms = ?3
             WHERE chat_id = ?1 AND operation_id = ?2 AND state = 'applying_undo'
               AND EXISTS (
                 SELECT 1 FROM chat_turn_checkpoints
                 WHERE id = chat_turn_recovery.checkpoint_id AND status = 'held'
               )",
            params![chat_id, operation_id, now_ms()],
        )
        .map_err(to_store_error)?;
    if updated != 1 {
        return Err("Undo recovery journal changed before completion.".to_string());
    }
    Ok(())
}

pub(crate) fn begin_turn_redo(
    chat_id: &str,
    operation_id: &str,
) -> Result<TurnRecoveryJournal, String> {
    let conn = connection()?;
    let updated = conn
        .execute(
            "UPDATE chat_turn_recovery
             SET operation_id = ?2, state = 'applying_redo', updated_at_ms = ?3
             WHERE chat_id = ?1 AND state = 'undone'
               AND EXISTS (
                 SELECT 1 FROM chat_turn_checkpoints
                 WHERE id = chat_turn_recovery.checkpoint_id AND status = 'held'
               )",
            params![chat_id, operation_id, now_ms()],
        )
        .map_err(to_store_error)?;
    if updated != 1 {
        return Err("No undone turn is available to redo.".to_string());
    }
    recovery_journal_on_connection(&conn, chat_id)?
        .ok_or_else(|| "Recovery journal was not found.".to_string())
}

pub(crate) fn finish_turn_redo(chat_id: &str, operation_id: &str) -> Result<(), String> {
    let mut conn = connection()?;
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let journal = recovery_journal_on_connection(&tx, chat_id)?
        .ok_or_else(|| "Recovery journal was not found.".to_string())?;
    if journal.operation_id != operation_id || journal.state != "applying_redo" {
        return Err("Redo recovery journal changed before completion.".to_string());
    }
    tx.execute(
        "DELETE FROM chat_turn_recovery WHERE chat_id = ?1",
        [chat_id],
    )
    .map_err(to_store_error)?;
    let status = if journal.start_snapshot_id.is_some() && journal.end_snapshot_id.is_some() {
        "complete"
    } else {
        "unavailable"
    };
    let restored = tx
        .execute(
            "UPDATE chat_turn_checkpoints SET status = ?2 WHERE id = ?1 AND status = 'held'",
            params![journal.checkpoint_id, status],
        )
        .map_err(to_store_error)?;
    if restored != 1 {
        return Err("Redo checkpoint changed before completion.".to_string());
    }
    tx.commit().map_err(to_store_error)
}

pub(super) fn visible_before_sequence(
    conn: &Connection,
    chat_id: &str,
) -> Result<Option<i64>, String> {
    conn.query_row(
        "SELECT boundary_sequence
         FROM chat_turn_recovery
         WHERE chat_id = ?1 AND state IN ('undone', 'applying_redo')",
        [chat_id],
        |row| row.get(0),
    )
    .optional()
    .map_err(to_store_error)
}

pub(super) fn discard_rewound_tail_on_connection(
    conn: &Connection,
    chat_id: &str,
) -> Result<bool, String> {
    let recovery = conn
        .query_row(
            "SELECT state, boundary_sequence FROM chat_turn_recovery WHERE chat_id = ?1",
            [chat_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
        )
        .optional()
        .map_err(to_store_error)?;
    let Some((state, boundary)) = recovery else {
        return Ok(false);
    };
    if state != "undone" {
        return Err("Turn recovery is still in progress for this chat.".to_string());
    }
    conn.execute(
        "DELETE FROM chat_generations
         WHERE chat_id = ?1
           AND assistant_message_id IN (
             SELECT id FROM chat_messages WHERE chat_id = ?1 AND sequence >= ?2
           )",
        params![chat_id, boundary],
    )
    .map_err(to_store_error)?;
    conn.execute(
        "DELETE FROM chat_messages WHERE chat_id = ?1 AND sequence >= ?2",
        params![chat_id, boundary],
    )
    .map_err(to_store_error)?;
    conn.execute(
        "DELETE FROM chat_turn_recovery WHERE chat_id = ?1",
        [chat_id],
    )
    .map_err(to_store_error)?;
    Ok(true)
}

pub(super) fn abandon_turn_recovery_on_connection(
    conn: &Connection,
    chat_id: &str,
) -> Result<(), String> {
    let state = conn
        .query_row(
            "SELECT state FROM chat_turn_recovery WHERE chat_id = ?1",
            [chat_id],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(to_store_error)?;
    if state.as_deref().is_some_and(|state| state != "undone") {
        return Err("Turn recovery is still in progress for this chat.".to_string());
    }
    conn.execute(
        "UPDATE chat_turn_checkpoints
         SET status = 'pruning'
         WHERE id = (SELECT checkpoint_id FROM chat_turn_recovery WHERE chat_id = ?1)
           AND status = 'held'",
        [chat_id],
    )
    .map_err(to_store_error)?;
    conn.execute(
        "DELETE FROM chat_turn_recovery WHERE chat_id = ?1",
        [chat_id],
    )
    .map_err(to_store_error)?;
    Ok(())
}

fn latest_recoverable_turn_checkpoint_on_connection(
    conn: &Connection,
    chat_id: &str,
) -> Result<Option<RecoverableTurnCheckpoint>, String> {
    let row = conn
        .query_row(
            "SELECT c.id, c.chat_id, c.user_message_id, c.project_path, c.storage_key,
                    c.start_snapshot_id, c.end_snapshot_id, c.changed_paths_json, m.sequence,
                    c.status, c.start_capture_json, c.end_capture_json
             FROM chat_turn_checkpoints c
             JOIN chat_messages m ON m.id = c.user_message_id
             WHERE c.chat_id = ?1
               AND c.status IN ('complete', 'unavailable', 'interrupted')
               AND m.sequence = (
                 SELECT MAX(latest.sequence)
                 FROM chat_messages latest
                 WHERE latest.chat_id = c.chat_id AND latest.role = 'user'
               )
             ORDER BY m.sequence DESC
             LIMIT 1",
            [chat_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<String>>(6)?,
                    row.get::<_, String>(7)?,
                    row.get::<_, i64>(8)?,
                    row.get::<_, String>(9)?,
                    row.get::<_, String>(10)?,
                    row.get::<_, Option<String>>(11)?,
                ))
            },
        )
        .optional()
        .map_err(to_store_error)?;
    row.map(|row| {
        let changed_paths = serde_json::from_str(&row.7).map_err(|error| error.to_string())?;
        let start_capture: CaptureResult =
            serde_json::from_str(&row.10).map_err(|error| error.to_string())?;
        let mut capture = match row.11 {
            Some(end_capture_json) => {
                let end_capture =
                    serde_json::from_str(&end_capture_json).map_err(|error| error.to_string())?;
                merge_boundary_captures(start_capture, end_capture)
            }
            None => start_capture,
        };
        let (start_snapshot_id, end_snapshot_id) = match (row.9.as_str(), row.5, row.6) {
            ("complete", Some(start), Some(end)) => (Some(start), Some(end)),
            _ => {
                capture.snapshot_id = None;
                capture.coverage = CheckpointCoverage::ConversationOnly;
                capture.unavailable_reason = Some(CaptureUnavailableReason::CaptureFailed);
                (None, None)
            }
        };
        Ok(RecoverableTurnCheckpoint {
            id: row.0,
            chat_id: row.1,
            user_message_id: row.2,
            project_path: row.3,
            storage_key: row.4,
            start_snapshot_id,
            end_snapshot_id,
            changed_paths,
            boundary_sequence: row.8,
            capture,
        })
    })
    .transpose()
}

fn merge_boundary_captures(start: CaptureResult, end: CaptureResult) -> CaptureResult {
    let mut skipped_paths = start.skipped_paths;
    for skipped in end.skipped_paths {
        if !skipped_paths.contains(&skipped) {
            skipped_paths.push(skipped);
        }
    }
    let both_available = start.snapshot_id.is_some() && end.snapshot_id.is_some();
    let partial = start.coverage == CheckpointCoverage::Partial
        || end.coverage == CheckpointCoverage::Partial
        || !skipped_paths.is_empty();
    CaptureResult {
        snapshot_id: both_available.then_some(end.snapshot_id).flatten(),
        coverage: if !both_available {
            CheckpointCoverage::ConversationOnly
        } else if partial {
            CheckpointCoverage::Partial
        } else {
            CheckpointCoverage::Full
        },
        skipped_paths,
        unavailable_reason: end.unavailable_reason.or(start.unavailable_reason),
    }
}

fn recovery_journal_on_connection(
    conn: &Connection,
    chat_id: &str,
) -> Result<Option<TurnRecoveryJournal>, String> {
    conn.query_row(
        "SELECT chat_id, checkpoint_id, user_message_id, operation_id, state, boundary_sequence,
                project_path, storage_key, start_snapshot_id, end_snapshot_id,
                changed_paths_json, capture_json
         FROM chat_turn_recovery WHERE chat_id = ?1",
        [chat_id],
        recovery_journal_from_row,
    )
    .optional()
    .map_err(to_store_error)?
    .map(parse_recovery_paths)
    .transpose()
}

fn recovery_journal_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TurnRecoveryJournal> {
    Ok(TurnRecoveryJournal {
        chat_id: row.get(0)?,
        checkpoint_id: row.get(1)?,
        user_message_id: row.get(2)?,
        operation_id: row.get(3)?,
        state: row.get(4)?,
        boundary_sequence: row.get(5)?,
        project_path: row.get(6)?,
        storage_key: row.get(7)?,
        start_snapshot_id: row.get(8)?,
        end_snapshot_id: row.get(9)?,
        changed_paths: vec![row.get::<_, String>(10)?],
        capture: serde_json::from_str(&row.get::<_, String>(11)?).map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                11,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })?,
    })
}

fn parse_recovery_paths(mut journal: TurnRecoveryJournal) -> Result<TurnRecoveryJournal, String> {
    let json = journal.changed_paths.pop().unwrap_or_default();
    journal.changed_paths = serde_json::from_str(&json).map_err(|error| error.to_string())?;
    Ok(journal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::{
        checkpoints::SkippedPathReason,
        schema::{create_turn_checkpoint_tables, create_turn_recovery_table},
    };

    fn test_connection() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE chats (id TEXT PRIMARY KEY);
            CREATE TABLE chat_messages (
              id TEXT PRIMARY KEY,
              chat_id TEXT,
              role TEXT,
              sequence INTEGER
            );
            CREATE TABLE chat_generations (
              id TEXT PRIMARY KEY,
              chat_id TEXT,
              assistant_message_id TEXT
            );
            ",
        )
        .unwrap();
        create_turn_checkpoint_tables(&conn).unwrap();
        create_turn_recovery_table(&conn).unwrap();
        conn.execute_batch(
            "
            INSERT INTO chats (id) VALUES ('chat');
            INSERT INTO chat_messages (id, chat_id, role, sequence)
              VALUES ('user', 'chat', 'user', 1),
                     ('assistant', 'chat', 'assistant', 2);
            INSERT INTO chat_generations (id, chat_id, assistant_message_id)
              VALUES ('generation', 'chat', 'assistant');
            ",
        )
        .unwrap();
        conn
    }

    fn insert_checkpoint(conn: &Connection, status: &str) {
        let (start, end, capture) = if status == "complete" {
            (
                Some("start"),
                Some("end"),
                r#"{"snapshot_id":"end","coverage":"full","skipped_paths":[],"unavailable_reason":null}"#,
            )
        } else {
            (
                Some("start"),
                None,
                r#"{"snapshot_id":"start","coverage":"full","skipped_paths":[],"unavailable_reason":null}"#,
            )
        };
        conn.execute(
            "INSERT INTO chat_turn_checkpoints
             (id, chat_id, user_message_id, assistant_message_id, generation_id,
              project_path, storage_key, status, start_snapshot_id, end_snapshot_id,
              start_capture_json, end_capture_json, changed_paths_json, created_at_ms)
             VALUES ('checkpoint', 'chat', 'user', 'assistant', 'generation',
                     '/project', 'storage', ?1, ?2, ?3, ?4, ?4, '[\"file.gd\"]', 1)",
            params![status, start, end, capture],
        )
        .unwrap();
    }

    fn insert_undone_journal(conn: &Connection) {
        conn.execute(
            "INSERT INTO chat_turn_recovery
             (chat_id, checkpoint_id, user_message_id, operation_id, state,
              boundary_sequence, project_path, storage_key, start_snapshot_id,
              end_snapshot_id, changed_paths_json, capture_json,
              started_at_ms, updated_at_ms)
             VALUES ('chat', 'checkpoint', 'user', 'operation', 'undone', 1,
                     '/project', 'storage', 'start', 'end', '[\"file.gd\"]',
                     '{\"snapshot_id\":\"end\",\"coverage\":\"full\",\"skipped_paths\":[],\"unavailable_reason\":null}',
                     1, 1)",
            [],
        )
        .unwrap();
    }

    #[test]
    fn capabilities_follow_the_persisted_recovery_state() {
        let conn = test_connection();
        insert_checkpoint(&conn, "complete");

        let available = turn_recovery_status_on_connection(&conn, "chat").unwrap();
        assert!(available.can_undo);
        assert!(!available.can_redo);
        assert_eq!(available.eligible_user_message_id.as_deref(), Some("user"));

        conn.execute(
            "UPDATE chat_turn_checkpoints SET status = 'held' WHERE id = 'checkpoint'",
            [],
        )
        .unwrap();
        insert_undone_journal(&conn);
        let undone = turn_recovery_status_on_connection(&conn, "chat").unwrap();
        assert!(!undone.can_undo);
        assert!(undone.can_redo);
    }

    #[test]
    fn an_incomplete_latest_user_turn_does_not_expose_an_older_checkpoint() {
        let conn = test_connection();
        insert_checkpoint(&conn, "complete");
        conn.execute(
            "INSERT INTO chat_messages (id, chat_id, role, sequence)
             VALUES ('new-user', 'chat', 'user', 3)",
            [],
        )
        .unwrap();

        let status = turn_recovery_status_on_connection(&conn, "chat").unwrap();
        assert!(!status.can_undo);
        assert_eq!(status.eligible_user_message_id, None);
        assert!(
            latest_recoverable_turn_checkpoint_on_connection(&conn, "chat")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn start_only_coverage_gap_is_preserved_when_the_file_is_deleted() {
        let conn = test_connection();
        insert_checkpoint(&conn, "complete");
        conn.execute(
            "UPDATE chat_turn_checkpoints
             SET start_capture_json = ?1,
                 end_capture_json = ?2,
                 changed_paths_json = '[]'
             WHERE id = 'checkpoint'",
            params![
                r#"{"snapshot_id":"start","coverage":"partial","skipped_paths":[{"path":"deleted.bin","reason":"large_untracked_file"}],"unavailable_reason":null}"#,
                r#"{"snapshot_id":"end","coverage":"full","skipped_paths":[],"unavailable_reason":null}"#,
            ],
        )
        .unwrap();

        let checkpoint = latest_recoverable_turn_checkpoint_on_connection(&conn, "chat")
            .unwrap()
            .unwrap();

        assert_eq!(checkpoint.capture.coverage, CheckpointCoverage::Partial);
        assert_eq!(
            checkpoint.capture.skipped_paths,
            vec![SkippedPath {
                path: "deleted.bin".to_string(),
                reason: SkippedPathReason::LargeUntrackedFile,
            }]
        );
    }

    #[test]
    fn replacement_tail_deletion_leaves_a_pruning_tombstone() {
        let conn = test_connection();
        insert_checkpoint(&conn, "complete");
        conn.execute(
            "UPDATE chat_turn_checkpoints SET status = 'held' WHERE id = 'checkpoint'",
            [],
        )
        .unwrap();
        insert_undone_journal(&conn);

        assert_eq!(visible_before_sequence(&conn, "chat").unwrap(), Some(1));
        assert!(discard_rewound_tail_on_connection(&conn, "chat").unwrap());

        let remaining: i64 = conn
            .query_row("SELECT COUNT(*) FROM chat_messages", [], |row| row.get(0))
            .unwrap();
        let remaining_generations: i64 = conn
            .query_row("SELECT COUNT(*) FROM chat_generations", [], |row| {
                row.get(0)
            })
            .unwrap();
        let status: String = conn
            .query_row(
                "SELECT status FROM chat_turn_checkpoints WHERE id = 'checkpoint'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(remaining, 0);
        assert_eq!(remaining_generations, 0);
        assert_eq!(status, "pruning");
        assert!(
            recovery_journal_on_connection(&conn, "chat")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn interrupted_checkpoint_downgrades_to_conversation_only() {
        let conn = test_connection();
        insert_checkpoint(&conn, "interrupted");

        let checkpoint = latest_recoverable_turn_checkpoint_on_connection(&conn, "chat")
            .unwrap()
            .unwrap();

        assert_eq!(checkpoint.start_snapshot_id, None);
        assert_eq!(checkpoint.end_snapshot_id, None);
        assert_eq!(
            checkpoint.capture.coverage,
            CheckpointCoverage::ConversationOnly
        );
    }

    #[test]
    fn abandoning_recovery_marks_held_checkpoint_for_pruning() {
        let conn = test_connection();
        insert_checkpoint(&conn, "complete");
        conn.execute(
            "UPDATE chat_turn_checkpoints SET status = 'held' WHERE id = 'checkpoint'",
            [],
        )
        .unwrap();
        insert_undone_journal(&conn);

        abandon_turn_recovery_on_connection(&conn, "chat").unwrap();

        let status: String = conn
            .query_row(
                "SELECT status FROM chat_turn_checkpoints WHERE id = 'checkpoint'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(status, "pruning");
        assert!(
            recovery_journal_on_connection(&conn, "chat")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn applying_recovery_cannot_be_abandoned() {
        let conn = test_connection();
        insert_checkpoint(&conn, "complete");
        conn.execute(
            "UPDATE chat_turn_checkpoints SET status = 'held' WHERE id = 'checkpoint'",
            [],
        )
        .unwrap();
        insert_undone_journal(&conn);
        conn.execute(
            "UPDATE chat_turn_recovery SET state = 'applying_redo' WHERE chat_id = 'chat'",
            [],
        )
        .unwrap();

        let error = abandon_turn_recovery_on_connection(&conn, "chat").unwrap_err();

        assert!(error.contains("still in progress"));
        assert!(
            recovery_journal_on_connection(&conn, "chat")
                .unwrap()
                .is_some()
        );
        let status: String = conn
            .query_row(
                "SELECT status FROM chat_turn_checkpoints WHERE id = 'checkpoint'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(status, "held");
    }
}
