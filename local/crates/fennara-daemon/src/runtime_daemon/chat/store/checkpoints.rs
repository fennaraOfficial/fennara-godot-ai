use rusqlite::{Connection, TransactionBehavior, params};

use super::super::{
    checkpoints::CaptureResult,
    ids::now_ms,
    schema::{connection, to_store_error},
};

pub(crate) struct NewTurnCheckpoint<'a> {
    pub(crate) id: &'a str,
    pub(crate) chat_id: &'a str,
    pub(crate) user_message_id: &'a str,
    pub(crate) assistant_message_id: &'a str,
    pub(crate) generation_id: &'a str,
    pub(crate) project_path: &'a str,
    pub(crate) storage_key: &'a str,
    pub(crate) start_capture: &'a CaptureResult,
}

pub(crate) struct CompletedTurnCheckpoint<'a> {
    pub(crate) id: &'a str,
    pub(crate) end_capture: &'a CaptureResult,
    pub(crate) changed_paths: &'a [String],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrunableTurnCheckpoint {
    pub(crate) id: String,
    pub(crate) storage_key: String,
}

pub(crate) fn insert_turn_checkpoint(input: NewTurnCheckpoint<'_>) -> Result<(), String> {
    insert_turn_checkpoint_on_connection(&connection()?, input)
}

fn insert_turn_checkpoint_on_connection(
    conn: &Connection,
    input: NewTurnCheckpoint<'_>,
) -> Result<(), String> {
    let now = now_ms();
    let available = input.start_capture.snapshot_id.is_some();
    let status = if available {
        "capturing"
    } else {
        "unavailable"
    };
    let completed_at = (!available).then_some(now);
    let start_capture_json =
        serde_json::to_string(input.start_capture).map_err(|error| error.to_string())?;
    conn.execute(
        "INSERT INTO chat_turn_checkpoints
         (id, chat_id, user_message_id, assistant_message_id, generation_id,
          project_path, storage_key, status, start_snapshot_id,
          start_capture_json, created_at_ms, completed_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            input.id,
            input.chat_id,
            input.user_message_id,
            input.assistant_message_id,
            input.generation_id,
            input.project_path,
            input.storage_key,
            status,
            input.start_capture.snapshot_id.as_deref(),
            start_capture_json,
            now,
            completed_at,
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}

pub(crate) fn complete_turn_checkpoint(input: CompletedTurnCheckpoint<'_>) -> Result<(), String> {
    complete_turn_checkpoint_on_connection(&connection()?, input)
}

fn complete_turn_checkpoint_on_connection(
    conn: &Connection,
    input: CompletedTurnCheckpoint<'_>,
) -> Result<(), String> {
    let end_capture_json =
        serde_json::to_string(input.end_capture).map_err(|error| error.to_string())?;
    let changed_paths_json =
        serde_json::to_string(input.changed_paths).map_err(|error| error.to_string())?;
    let status = if input.end_capture.snapshot_id.is_some() {
        "complete"
    } else {
        "unavailable"
    };
    let updated = conn
        .execute(
            "UPDATE chat_turn_checkpoints
             SET status = ?2,
                 end_snapshot_id = ?3,
                 end_capture_json = ?4,
                 changed_paths_json = ?5,
                 completed_at_ms = ?6
             WHERE id = ?1 AND status = 'capturing'",
            params![
                input.id,
                status,
                input.end_capture.snapshot_id.as_deref(),
                end_capture_json,
                changed_paths_json,
                now_ms(),
            ],
        )
        .map_err(to_store_error)?;
    if updated != 1 {
        return Err("Turn checkpoint is no longer capturing.".to_string());
    }
    Ok(())
}

pub(crate) fn mark_capturing_checkpoints_interrupted() -> Result<(), String> {
    connection()?
        .execute(
            "UPDATE chat_turn_checkpoints
             SET status = 'interrupted', completed_at_ms = ?1
             WHERE status = 'capturing'",
            [now_ms()],
        )
        .map_err(to_store_error)?;
    Ok(())
}

pub(crate) fn mark_turn_checkpoint_interrupted(id: &str) -> Result<(), String> {
    connection()?
        .execute(
            "UPDATE chat_turn_checkpoints
             SET status = 'interrupted', completed_at_ms = ?2
             WHERE id = ?1 AND status = 'capturing'",
            params![id, now_ms()],
        )
        .map_err(to_store_error)?;
    Ok(())
}

pub(crate) fn claim_prunable_turn_checkpoints(
    storage_key: &str,
    keep: usize,
) -> Result<Vec<PrunableTurnCheckpoint>, String> {
    let mut conn = connection()?;
    claim_prunable_turn_checkpoints_on_connection(&mut conn, storage_key, keep)
}

fn claim_prunable_turn_checkpoints_on_connection(
    conn: &mut Connection,
    storage_key: &str,
    keep: usize,
) -> Result<Vec<PrunableTurnCheckpoint>, String> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let entries = {
        let mut statement = tx
            .prepare(
                "SELECT id, storage_key
                 FROM chat_turn_checkpoints
                 WHERE storage_key = ?1
                   AND status IN ('complete', 'unavailable', 'interrupted')
                 ORDER BY created_at_ms DESC, id DESC
                 LIMIT -1 OFFSET ?2",
            )
            .map_err(to_store_error)?;
        let rows = statement
            .query_map(params![storage_key, keep as i64], |row| {
                Ok(PrunableTurnCheckpoint {
                    id: row.get(0)?,
                    storage_key: row.get(1)?,
                })
            })
            .map_err(to_store_error)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(to_store_error)?
    };
    for entry in &entries {
        tx.execute(
            "UPDATE chat_turn_checkpoints SET status = 'pruning' WHERE id = ?1",
            [&entry.id],
        )
        .map_err(to_store_error)?;
    }
    tx.commit().map_err(to_store_error)?;
    Ok(entries)
}

pub(crate) fn pruning_turn_checkpoints() -> Result<Vec<PrunableTurnCheckpoint>, String> {
    let conn = connection()?;
    let mut statement = conn
        .prepare(
            "SELECT id, storage_key
             FROM chat_turn_checkpoints
             WHERE status = 'pruning'
             ORDER BY created_at_ms, id",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok(PrunableTurnCheckpoint {
                id: row.get(0)?,
                storage_key: row.get(1)?,
            })
        })
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(to_store_error)
}

pub(crate) fn pruning_turn_checkpoints_for_storage(
    storage_key: &str,
) -> Result<Vec<PrunableTurnCheckpoint>, String> {
    let conn = connection()?;
    let mut statement = conn
        .prepare(
            "SELECT id, storage_key
             FROM chat_turn_checkpoints
             WHERE status = 'pruning' AND storage_key = ?1
             ORDER BY created_at_ms, id",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map([storage_key], |row| {
            Ok(PrunableTurnCheckpoint {
                id: row.get(0)?,
                storage_key: row.get(1)?,
            })
        })
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(to_store_error)
}

pub(crate) fn delete_pruning_turn_checkpoint(id: &str) -> Result<(), String> {
    connection()?
        .execute(
            "DELETE FROM chat_turn_checkpoints WHERE id = ?1 AND status = 'pruning'",
            [id],
        )
        .map_err(to_store_error)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::{
        checkpoints::{CheckpointCoverage, SkippedPath},
        schema::create_turn_checkpoint_tables,
    };

    fn capture(snapshot_id: Option<&str>, coverage: CheckpointCoverage) -> CaptureResult {
        CaptureResult {
            snapshot_id: snapshot_id.map(ToOwned::to_owned),
            coverage,
            skipped_paths: Vec::<SkippedPath>::new(),
            unavailable_reason: None,
        }
    }

    fn test_connection() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE chats (id TEXT PRIMARY KEY);
            CREATE TABLE chat_messages (id TEXT PRIMARY KEY);
            CREATE TABLE chat_generations (id TEXT PRIMARY KEY);
            ",
        )
        .unwrap();
        create_turn_checkpoint_tables(&conn).unwrap();
        conn
    }

    fn insert_parents(
        conn: &Connection,
        chat_id: &str,
        user_message_id: &str,
        assistant_message_id: &str,
        generation_id: &str,
    ) {
        conn.execute("INSERT OR IGNORE INTO chats (id) VALUES (?1)", [chat_id])
            .unwrap();
        conn.execute(
            "INSERT INTO chat_messages (id) VALUES (?1), (?2)",
            params![user_message_id, assistant_message_id],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO chat_generations (id) VALUES (?1)",
            [generation_id],
        )
        .unwrap();
    }

    #[test]
    fn persists_and_completes_a_turn_checkpoint() {
        let conn = test_connection();
        insert_parents(&conn, "chat_1", "user_1", "assistant_1", "generation_1");
        let start = capture(Some("start"), CheckpointCoverage::Full);
        insert_turn_checkpoint_on_connection(
            &conn,
            NewTurnCheckpoint {
                id: "checkpoint_1",
                chat_id: "chat_1",
                user_message_id: "user_1",
                assistant_message_id: "assistant_1",
                generation_id: "generation_1",
                project_path: "/project",
                storage_key: "storage",
                start_capture: &start,
            },
        )
        .unwrap();
        let end = capture(Some("end"), CheckpointCoverage::Partial);
        complete_turn_checkpoint_on_connection(
            &conn,
            CompletedTurnCheckpoint {
                id: "checkpoint_1",
                end_capture: &end,
                changed_paths: &["player.gd".to_string()],
            },
        )
        .unwrap();

        let row: (String, String, String) = conn
            .query_row(
                "SELECT status, end_snapshot_id, changed_paths_json
                 FROM chat_turn_checkpoints WHERE id = 'checkpoint_1'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(
            row,
            (
                "complete".to_string(),
                "end".to_string(),
                "[\"player.gd\"]".to_string()
            )
        );
    }

    #[test]
    fn retention_marks_only_old_terminal_checkpoints_for_pruning() {
        let mut conn = test_connection();
        for index in 0..3 {
            insert_parents(
                &conn,
                "chat",
                &format!("user_{index}"),
                &format!("assistant_{index}"),
                &format!("generation_{index}"),
            );
            conn.execute(
                "INSERT INTO chat_turn_checkpoints
                 (id, chat_id, user_message_id, assistant_message_id, generation_id,
                  project_path, storage_key, status, start_capture_json, created_at_ms)
                 VALUES (?1, 'chat', ?2, ?3, ?4, '/project', 'storage', ?5, '{}', ?6)",
                params![
                    format!("checkpoint_{index}"),
                    format!("user_{index}"),
                    format!("assistant_{index}"),
                    format!("generation_{index}"),
                    if index == 2 { "capturing" } else { "complete" },
                    index,
                ],
            )
            .unwrap();
        }

        let claimed =
            claim_prunable_turn_checkpoints_on_connection(&mut conn, "storage", 1).unwrap();
        assert_eq!(
            claimed,
            vec![PrunableTurnCheckpoint {
                id: "checkpoint_0".to_string(),
                storage_key: "storage".to_string(),
            }]
        );
        let statuses: Vec<(String, String)> = conn
            .prepare("SELECT id, status FROM chat_turn_checkpoints ORDER BY id")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(
            statuses,
            vec![
                ("checkpoint_0".to_string(), "pruning".to_string()),
                ("checkpoint_1".to_string(), "complete".to_string()),
                ("checkpoint_2".to_string(), "capturing".to_string()),
            ]
        );
    }
}
