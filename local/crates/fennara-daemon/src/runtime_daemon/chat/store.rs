use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde::Serialize;
use serde_json::{Value, json};

use super::{
    context_compaction::{self, ContextSummaryChunk, InsertContextSummary, SummaryCandidate},
    ids::{new_id, now_ms},
    schema::{connection, model_trace_from_selection, to_store_error},
    settings::{self, DEFAULT_MODEL},
};

mod checkpoints;
mod generations;
mod replay;
mod tool_calls;
mod usage;

pub(crate) use self::checkpoints::{
    CompletedTurnCheckpoint, NewTurnCheckpoint, PrunableTurnCheckpoint,
    claim_prunable_turn_checkpoints, complete_turn_checkpoint, delete_pruning_turn_checkpoint,
    insert_turn_checkpoint, mark_capturing_checkpoints_interrupted,
    mark_turn_checkpoint_interrupted, pruning_turn_checkpoints,
    pruning_turn_checkpoints_for_storage,
};
use self::usage::{
    latest_prompt_tokens_for_chat, record_usage_log, total_cost_for_chat, usage_cost,
};

const CHAT_LIST_LIMIT: i64 = 40;
const MAX_IMAGE_METADATA_MESSAGES_PER_CHAT: i64 = 12;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ChatSummary {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) project_path: Option<String>,
    pub(crate) project_name: Option<String>,
    pub(crate) model: String,
    pub(crate) reasoning_effort: String,
    pub(crate) total_cost: f64,
    pub(crate) latest_prompt_tokens: i64,
    pub(crate) message_count: i64,
    pub(crate) created_at_ms: i64,
    pub(crate) updated_at_ms: i64,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ProjectScope {
    pub(crate) project_path: Option<String>,
    pub(crate) project_name: Option<String>,
}

impl ProjectScope {
    pub(crate) fn key(&self) -> String {
        self.project_path
            .as_deref()
            .map(normalize_project_path)
            .filter(|path| !path.is_empty())
            .unwrap_or_else(|| "global".to_string())
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct StoredMessage {
    pub(crate) id: String,
    pub(crate) chat_id: String,
    pub(crate) role: String,
    pub(crate) status: String,
    pub(crate) content: String,
    pub(crate) reasoning_content: Option<String>,
    pub(crate) tool_call_id: Option<String>,
    pub(crate) tool_name: Option<String>,
    pub(crate) tool_calls_json: Option<String>,
    pub(crate) metadata_json: Option<String>,
    pub(crate) usage_json: Option<String>,
    pub(crate) cost: Option<f64>,
    pub(crate) sequence: i64,
    pub(crate) created_at_ms: i64,
    pub(crate) updated_at_ms: i64,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ContextCompactionMarker {
    pub(crate) id: String,
    pub(crate) chat_id: String,
    pub(crate) created_at_ms: i64,
    pub(crate) covered_start_sequence: i64,
    pub(crate) covered_end_sequence: i64,
    pub(crate) source_message_count: i64,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct OpenedChat {
    pub(crate) chat: ChatSummary,
    pub(crate) messages: Vec<StoredMessage>,
    pub(crate) context_compactions: Vec<ContextCompactionMarker>,
}

#[derive(Clone, Debug)]
pub(crate) struct StartedGeneration {
    pub(crate) id: String,
}

#[derive(Clone, Debug)]
pub(crate) struct ToolImageFile {
    pub(crate) file_path: String,
    pub(crate) mime_type: String,
}

pub(crate) fn list_chats(scope: &ProjectScope) -> Result<Vec<ChatSummary>, String> {
    let conn = connection()?;
    let mut statement = conn
        .prepare(
            "SELECT id, title, project_path, project_name, model, reasoning_effort, total_cost, latest_prompt_tokens, message_count, created_at_ms, updated_at_ms
             FROM chats
             WHERE archived_at_ms IS NULL AND project_path IS ?1
             ORDER BY updated_at_ms DESC
             LIMIT ?2",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map(
            params![scope.project_path.as_deref(), CHAT_LIST_LIMIT],
            chat_from_row,
        )
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(to_store_error)
}

pub(crate) fn open_chat(scope: &ProjectScope, chat_id: &str) -> Result<OpenedChat, String> {
    let conn = connection()?;
    let chat = get_chat_for_scope(&conn, scope, chat_id)?
        .ok_or_else(|| "Chat not found for this project.".to_string())?;
    let messages = messages_for_chat(&conn, chat_id)?;
    let context_compactions = context_compactions_for_chat(&conn, chat_id)?;
    set_active_chat_id(scope, chat_id)?;
    Ok(OpenedChat {
        chat,
        messages,
        context_compactions,
    })
}

pub(crate) fn chat_summary(chat_id: &str) -> Result<ChatSummary, String> {
    let conn = connection()?;
    get_chat(&conn, chat_id)?.ok_or_else(|| "Chat not found.".to_string())
}

pub(crate) fn open_active_or_create(
    scope: &ProjectScope,
    model: &str,
    reasoning_effort: &str,
) -> Result<OpenedChat, String> {
    if let Some(chat_id) = active_chat_id(scope)? {
        if let Ok(opened) = open_chat(scope, &chat_id) {
            return Ok(opened);
        }
    }
    create_chat(scope, model, reasoning_effort)
}

pub(crate) fn create_chat(
    scope: &ProjectScope,
    model: &str,
    reasoning_effort: &str,
) -> Result<OpenedChat, String> {
    let conn = connection()?;
    let now = now_ms();
    let chat_id = new_id("chat");
    let clean_model = settings::clean_model(model).unwrap_or_else(|| DEFAULT_MODEL.to_string());
    let clean_effort = settings::clean_reasoning_effort(reasoning_effort);
    let model_trace = model_trace_from_selection(&clean_model);
    conn.execute(
        "INSERT INTO chats
         (id, title, project_path, project_name, model,
          provider_id, model_id, model_variant, model_ref_json,
          reasoning_effort, total_cost, latest_prompt_tokens, message_count, created_at_ms, updated_at_ms)
         VALUES (?1, 'New chat', ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0, 0, 0, ?10, ?10)",
        params![
            chat_id,
            scope.project_path.as_deref(),
            scope.project_name.as_deref(),
            clean_model,
            model_trace.as_ref().map(|trace| trace.provider_id.as_str()),
            model_trace.as_ref().map(|trace| trace.model_id.as_str()),
            model_trace
                .as_ref()
                .and_then(|trace| trace.model_variant.as_deref()),
            model_trace
                .as_ref()
                .map(|trace| trace.model_ref_json.as_str()),
            clean_effort,
            now
        ],
    )
    .map_err(to_store_error)?;
    set_active_chat_id(scope, &chat_id)?;
    open_chat(scope, &chat_id)
}

pub(crate) fn archive_chat(scope: &ProjectScope, chat_id: &str) -> Result<(), String> {
    let conn = connection()?;
    if get_chat_for_scope(&conn, scope, chat_id)?.is_none() {
        return Err("Chat not found for this project.".to_string());
    }
    let now = now_ms();
    conn.execute(
        "UPDATE chats SET archived_at_ms = ?2, updated_at_ms = ?2 WHERE id = ?1",
        params![chat_id, now],
    )
    .map_err(to_store_error)?;
    if active_chat_id(scope)?.as_deref() == Some(chat_id) {
        set_active_chat_id(scope, "")?;
    }
    Ok(())
}

pub(crate) fn revert_last_turn(scope: &ProjectScope, chat_id: &str) -> Result<OpenedChat, String> {
    let mut conn = connection()?;
    if get_chat_for_scope(&conn, scope, chat_id)?.is_none() {
        return Err("Chat not found for this project.".to_string());
    }
    let start_sequence: Option<i64> = conn
        .query_row(
            "SELECT sequence FROM chat_messages
             WHERE chat_id = ?1 AND role = 'user'
             ORDER BY sequence DESC
             LIMIT 1",
            [chat_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    let Some(start_sequence) = start_sequence else {
        return open_chat(scope, chat_id);
    };

    let tx = conn.transaction().map_err(to_store_error)?;
    tx.execute(
        "DELETE FROM chat_usage_logs
         WHERE chat_id = ?1
           AND assistant_message_id IN (
             SELECT id FROM chat_messages WHERE chat_id = ?1 AND sequence >= ?2
           )",
        params![chat_id, start_sequence],
    )
    .map_err(to_store_error)?;
    tx.execute(
        "DELETE FROM chat_tool_calls
         WHERE chat_id = ?1
           AND assistant_message_id IN (
             SELECT id FROM chat_messages WHERE chat_id = ?1 AND sequence >= ?2
           )",
        params![chat_id, start_sequence],
    )
    .map_err(to_store_error)?;
    tx.execute(
        "DELETE FROM chat_messages WHERE chat_id = ?1 AND sequence >= ?2",
        params![chat_id, start_sequence],
    )
    .map_err(to_store_error)?;
    refresh_chat_rollups(&tx, chat_id)?;
    tx.commit().map_err(to_store_error)?;
    open_chat(scope, chat_id)
}

pub(crate) fn last_user_message_content(chat_id: &str) -> Result<Option<String>, String> {
    let conn = connection()?;
    conn.query_row(
        "SELECT content FROM chat_messages
         WHERE chat_id = ?1 AND role = 'user'
         ORDER BY sequence DESC
         LIMIT 1",
        [chat_id],
        |row| row.get(0),
    )
    .optional()
    .map_err(to_store_error)
}

pub(crate) fn ensure_chat_in_scope(scope: &ProjectScope, chat_id: &str) -> Result<(), String> {
    let conn = connection()?;
    get_chat_for_scope(&conn, scope, chat_id)?
        .map(|_| ())
        .ok_or_else(|| "Chat not found for this project.".to_string())
}

pub(crate) fn set_chat_model(
    chat_id: &str,
    model: &str,
    reasoning_effort: &str,
) -> Result<(), String> {
    let conn = connection()?;
    if get_chat(&conn, chat_id)?.is_none() {
        return Err("Chat not found.".to_string());
    }
    let clean_model = settings::clean_model(model).unwrap_or_else(|| DEFAULT_MODEL.to_string());
    let clean_effort = settings::clean_reasoning_effort(reasoning_effort);
    let model_trace = model_trace_from_selection(&clean_model);
    let now = now_ms();
    conn.execute(
        "UPDATE chats
         SET model = ?2,
             provider_id = ?3,
             model_id = ?4,
             model_variant = ?5,
             model_ref_json = ?6,
             reasoning_effort = ?7,
             updated_at_ms = ?8
         WHERE id = ?1",
        params![
            chat_id,
            clean_model,
            model_trace.as_ref().map(|trace| trace.provider_id.as_str()),
            model_trace.as_ref().map(|trace| trace.model_id.as_str()),
            model_trace
                .as_ref()
                .and_then(|trace| trace.model_variant.as_deref()),
            model_trace
                .as_ref()
                .map(|trace| trace.model_ref_json.as_str()),
            clean_effort,
            now
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}

pub(crate) fn insert_user_message(
    chat_id: &str,
    content: &str,
    metadata: Option<&Value>,
) -> Result<StoredMessage, String> {
    let mut conn = connection()?;
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let now = now_ms();
    let metadata_json = metadata
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| error.to_string())?;
    let message = insert_message_in_tx(
        &tx,
        chat_id,
        NewMessage {
            role: "user",
            status: "done",
            content,
            reasoning_content: None,
            usage_json: None,
            cost: None,
            tool_call_id: None,
            tool_name: None,
            metadata_json: metadata_json.as_deref(),
        },
        now,
    )?;
    if metadata_json.is_some() {
        prune_old_image_metadata(&tx, chat_id, now)?;
    }
    tx.commit().map_err(to_store_error)?;
    Ok(message)
}

fn prune_old_image_metadata(conn: &Connection, chat_id: &str, now: i64) -> Result<(), String> {
    conn.execute(
        "UPDATE chat_messages
         SET metadata_json = NULL,
             updated_at_ms = ?2
         WHERE id IN (
           SELECT id
           FROM chat_messages
           WHERE chat_id = ?1
             AND role = 'user'
             AND metadata_json LIKE '%\"images\"%'
           ORDER BY sequence DESC
           LIMIT -1 OFFSET ?3
         )",
        params![chat_id, now, MAX_IMAGE_METADATA_MESSAGES_PER_CHAT],
    )
    .map_err(to_store_error)?;
    Ok(())
}

pub(crate) fn insert_assistant_placeholder_with_generation(
    chat_id: &str,
    model: &str,
    reasoning_effort: &str,
) -> Result<(StoredMessage, StartedGeneration), String> {
    let mut conn = connection()?;
    generations::insert_assistant_placeholder_with_generation_on_connection(
        &mut conn,
        chat_id,
        model,
        reasoning_effort,
    )
}

pub(crate) fn finish_generation(
    generation_id: &str,
    status: &str,
    error: Option<&Value>,
) -> Result<(), String> {
    let conn = connection()?;
    generations::finish_generation_on_connection(&conn, generation_id, status, error)
}

pub(crate) fn finish_assistant_message(
    message_id: &str,
    content: &str,
    reasoning_content: Option<&str>,
    usage: Option<&Value>,
    fallback_model: &str,
    generation_id: Option<&str>,
) -> Result<StoredMessage, String> {
    let mut conn = connection()?;
    finish_assistant_message_on_connection(
        &mut conn,
        message_id,
        None,
        content,
        reasoning_content,
        usage,
        fallback_model,
        generation_id,
    )
}

pub(crate) fn finish_assistant_message_with_tool_calls(
    message_id: &str,
    tool_calls: &Value,
    content: &str,
    reasoning_content: Option<&str>,
    usage: Option<&Value>,
    fallback_model: &str,
    generation_id: Option<&str>,
) -> Result<StoredMessage, String> {
    let mut conn = connection()?;
    finish_assistant_message_on_connection(
        &mut conn,
        message_id,
        Some(tool_calls),
        content,
        reasoning_content,
        usage,
        fallback_model,
        generation_id,
    )
}

fn finish_assistant_message_on_connection(
    conn: &mut Connection,
    message_id: &str,
    tool_calls: Option<&Value>,
    content: &str,
    reasoning_content: Option<&str>,
    usage: Option<&Value>,
    fallback_model: &str,
    generation_id: Option<&str>,
) -> Result<StoredMessage, String> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let now = now_ms();
    let tool_calls_json = tool_calls
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| error.to_string())?;
    let usage_json = usage
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| error.to_string())?;
    let cost = usage.and_then(usage_cost);
    let model_trace = model_trace_from_selection(fallback_model);
    if let Some(tool_calls_json) = tool_calls_json.as_deref() {
        tx.execute(
            "UPDATE chat_messages SET tool_calls_json = ?2, updated_at_ms = ?3 WHERE id = ?1",
            params![message_id, tool_calls_json, now],
        )
        .map_err(to_store_error)?;
    }
    tx.execute(
        "UPDATE chat_messages
         SET status = 'done',
             content = ?2,
             reasoning_content = ?3,
             usage_json = ?4,
             cost = ?5,
             generation_id = COALESCE(?6, generation_id),
             provider_id = COALESCE(?7, provider_id),
             model_id = COALESCE(?8, model_id),
             model_variant = COALESCE(?9, model_variant),
             model_ref_json = COALESCE(?10, model_ref_json),
             updated_at_ms = ?11
         WHERE id = ?1",
        params![
            message_id,
            content,
            reasoning_content,
            usage_json,
            cost,
            generation_id,
            model_trace.as_ref().map(|trace| trace.provider_id.as_str()),
            model_trace.as_ref().map(|trace| trace.model_id.as_str()),
            model_trace
                .as_ref()
                .and_then(|trace| trace.model_variant.as_deref()),
            model_trace
                .as_ref()
                .map(|trace| trace.model_ref_json.as_str()),
            now
        ],
    )
    .map_err(to_store_error)?;
    let message = get_message(&tx, message_id)?.ok_or_else(|| "Message not found.".to_string())?;
    if let Some(usage) = usage {
        record_usage_log(
            &tx,
            &message.chat_id,
            &message.id,
            fallback_model,
            "chat",
            usage,
            generation_id,
            model_trace.as_ref(),
        )?;
    }
    refresh_chat_rollups(&tx, &message.chat_id)?;
    tx.commit().map_err(to_store_error)?;
    Ok(message)
}

pub(crate) fn fail_assistant_message(
    message_id: &str,
    content: &str,
) -> Result<StoredMessage, String> {
    let conn = connection()?;
    let now = now_ms();
    conn.execute(
        "UPDATE chat_messages
         SET status = 'failed',
             content = ?2,
             updated_at_ms = ?3
         WHERE id = ?1",
        params![message_id, content, now],
    )
    .map_err(to_store_error)?;
    let message =
        get_message(&conn, message_id)?.ok_or_else(|| "Message not found.".to_string())?;
    refresh_chat_rollups(&conn, &message.chat_id)?;
    Ok(message)
}

pub(crate) fn cancel_turn(
    chat_id: &str,
    assistant_message_id: &str,
    assistant_content: &str,
) -> Result<StoredMessage, String> {
    let mut conn = connection()?;
    cancel_turn_on_connection(&mut conn, chat_id, assistant_message_id, assistant_content)
}

fn cancel_turn_on_connection(
    conn: &mut Connection,
    chat_id: &str,
    assistant_message_id: &str,
    assistant_content: &str,
) -> Result<StoredMessage, String> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let assistant = get_message(&tx, assistant_message_id)?
        .ok_or_else(|| "Assistant message not found.".to_string())?;
    if assistant.chat_id != chat_id {
        return Err("Assistant message not found for this chat.".to_string());
    }
    let next_user_sequence: Option<i64> = tx
        .query_row(
            "SELECT sequence FROM chat_messages
             WHERE chat_id = ?1 AND role = 'user' AND sequence > ?2
             ORDER BY sequence ASC
             LIMIT 1",
            params![chat_id, assistant.sequence],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    let now = now_ms();
    tx.execute(
        "UPDATE chat_messages
         SET status = 'cancelled', updated_at_ms = ?4
         WHERE chat_id = ?1
           AND sequence >= ?2
           AND (?3 IS NULL OR sequence < ?3)
           AND role IN ('assistant', 'tool')",
        params![chat_id, assistant.sequence, next_user_sequence, now],
    )
    .map_err(to_store_error)?;
    tx.execute(
        "UPDATE chat_messages
         SET status = 'cancelled',
             content = ?2,
             updated_at_ms = ?3
         WHERE id = ?1",
        params![assistant_message_id, assistant_content, now],
    )
    .map_err(to_store_error)?;
    tx.execute(
        "UPDATE chat_tool_calls
         SET status = 'cancelled', updated_at_ms = ?4
         WHERE chat_id = ?1
           AND assistant_message_id IN (
             SELECT id
             FROM chat_messages
             WHERE chat_id = ?1
               AND sequence >= ?2
               AND (?3 IS NULL OR sequence < ?3)
               AND role = 'assistant'
           )",
        params![chat_id, assistant.sequence, next_user_sequence, now],
    )
    .map_err(to_store_error)?;
    let message = get_message(&tx, assistant_message_id)?
        .ok_or_else(|| "Assistant message not found.".to_string())?;
    refresh_chat_rollups(&tx, chat_id)?;
    tx.commit().map_err(to_store_error)?;
    Ok(message)
}

pub(crate) fn upsert_tool_call(
    chat_id: &str,
    assistant_message_id: &str,
    generation_id: Option<&str>,
    tool_call_id: &str,
    provider_tool_call_id: Option<&str>,
    tool_name: &str,
    arguments: &Value,
    status: &str,
) -> Result<(), String> {
    let conn = connection()?;
    tool_calls::upsert_tool_call_on_connection(
        &conn,
        chat_id,
        assistant_message_id,
        generation_id,
        tool_call_id,
        provider_tool_call_id,
        tool_name,
        arguments,
        status,
    )
}

pub(crate) fn finish_tool_call_with_message(
    chat_id: &str,
    tool_call_id: &str,
    tool_name: &str,
    status: &str,
    raw_result: &Value,
    mcp_markdown: &str,
    plugin_markdown: &str,
    metadata: &Value,
    target_keys: &[String],
) -> Result<StoredMessage, String> {
    let mut conn = connection()?;
    tool_calls::finish_tool_call_with_message_on_connection(
        &mut conn,
        chat_id,
        tool_call_id,
        tool_name,
        status,
        raw_result,
        mcp_markdown,
        plugin_markdown,
        metadata,
        target_keys,
    )
}

pub(crate) fn tool_image_file(
    tool_call_id: &str,
    image_index: usize,
    access_token: &str,
) -> Result<Option<ToolImageFile>, String> {
    if !is_safe_tool_image_token(access_token) {
        return Ok(None);
    }
    let conn = connection()?;
    let metadata_json: Option<String> = conn
        .query_row(
            "SELECT metadata_json FROM chat_tool_calls WHERE id = ?1",
            params![tool_call_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?
        .flatten();
    let Some(metadata_json) = metadata_json else {
        return Ok(None);
    };
    let metadata: Value =
        serde_json::from_str(&metadata_json).map_err(|error| error.to_string())?;
    let Some(image) = metadata
        .get("tool_images")
        .and_then(Value::as_array)
        .and_then(|images| images.get(image_index))
    else {
        return Ok(None);
    };
    let Some(file_path) = image
        .get("file_path")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return Ok(None);
    };
    let Some(stored_token) = image
        .get("token")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| constant_time_eq(value.as_bytes(), access_token.as_bytes()))
    else {
        return Ok(None);
    };
    if !is_safe_tool_image_token(stored_token) {
        return Ok(None);
    }
    let Some(mime_type) = image
        .get("mime_type")
        .and_then(Value::as_str)
        .map(str::trim)
        .and_then(normalize_tool_image_mime)
    else {
        return Ok(None);
    };
    Ok(Some(ToolImageFile {
        file_path: file_path.to_string(),
        mime_type: mime_type.to_string(),
    }))
}

fn is_safe_tool_image_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut diff = 0u8;
    for (left, right) in left.iter().zip(right) {
        diff |= left ^ right;
    }
    diff == 0
}

fn normalize_tool_image_mime(mime: &str) -> Option<&'static str> {
    match mime.trim().to_ascii_lowercase().as_str() {
        "image/png" => Some("image/png"),
        "image/jpeg" | "image/jpg" => Some("image/jpeg"),
        "image/webp" => Some("image/webp"),
        "image/gif" => Some("image/gif"),
        _ => None,
    }
}

pub(crate) fn replay_messages(chat_id: &str) -> Result<Vec<Value>, String> {
    let conn = connection()?;
    replay::replay_messages_from_conn(&conn, chat_id)
}

pub(crate) fn replay_messages_with_summary_budget(
    chat_id: &str,
    summary_replay_budget_tokens: usize,
) -> Result<Vec<Value>, String> {
    let conn = connection()?;
    replay::replay_messages_with_summary_budget_from_conn(
        &conn,
        chat_id,
        Some(summary_replay_budget_tokens),
    )
}

pub(crate) fn replay_messages_with_summary_and_exact_tail_budget(
    chat_id: &str,
    summary_replay_budget_tokens: usize,
    exact_tail_budget_tokens: usize,
) -> Result<Vec<Value>, String> {
    let conn = connection()?;
    replay::replay_messages_with_summary_and_exact_tail_budget_from_conn(
        &conn,
        chat_id,
        summary_replay_budget_tokens,
        exact_tail_budget_tokens,
    )
}

pub(crate) fn replay_messages_with_summary_and_exact_tail_budget_before_sequence(
    chat_id: &str,
    summary_replay_budget_tokens: usize,
    exact_tail_budget_tokens: usize,
    before_sequence: i64,
) -> Result<Vec<Value>, String> {
    let conn = connection()?;
    replay::replay_messages_with_summary_and_exact_tail_budget_before_sequence_from_conn(
        &conn,
        chat_id,
        summary_replay_budget_tokens,
        exact_tail_budget_tokens,
        before_sequence,
    )
}

pub(crate) fn context_summary_candidate(
    chat_id: &str,
    tail_budget_tokens: usize,
) -> Result<Option<SummaryCandidate>, String> {
    let conn = connection()?;
    let groups = replay::raw_summary_groups_from_conn(&conn, chat_id)?;
    let summaries = context_compaction::load_context_summaries_from_conn(&conn, chat_id)?;
    Ok(context_compaction::select_next_summary_candidate(
        &groups,
        &summaries,
        tail_budget_tokens,
    ))
}

pub(crate) fn context_summary_candidate_before_sequence(
    chat_id: &str,
    tail_budget_tokens: usize,
    before_sequence: i64,
) -> Result<Option<SummaryCandidate>, String> {
    let conn = connection()?;
    let groups =
        replay::raw_summary_groups_before_sequence_from_conn(&conn, chat_id, before_sequence)?;
    let mut summaries = context_compaction::load_context_summaries_from_conn(&conn, chat_id)?;
    summaries.retain(|summary| summary.covered_end_sequence < before_sequence);
    Ok(context_compaction::select_next_summary_candidate(
        &groups,
        &summaries,
        tail_budget_tokens,
    ))
}

pub(crate) fn insert_context_summary(
    chat_id: &str,
    generation_id: &str,
    summary_markdown: &str,
    candidate: &SummaryCandidate,
    model: &str,
    reasoning_effort: &str,
    usage: Option<&Value>,
    metadata: &Value,
) -> Result<ContextSummaryChunk, String> {
    let mut conn = connection()?;
    context_compaction::insert_context_summary_on_connection(
        &mut conn,
        InsertContextSummary {
            chat_id,
            generation_id,
            summary_markdown,
            candidate,
            model,
            reasoning_effort,
            usage,
            metadata,
        },
    )
}

pub(crate) fn set_active_chat_id(scope: &ProjectScope, chat_id: &str) -> Result<(), String> {
    let conn = connection()?;
    let key = active_chat_key(scope);
    let value = if chat_id.is_empty() {
        Value::Null
    } else {
        json!(chat_id)
    };
    let now = now_ms();
    conn.execute(
        "INSERT INTO chat_settings (key, value_json, updated_at_ms)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value_json = excluded.value_json, updated_at_ms = excluded.updated_at_ms",
        params![key, value.to_string(), now],
    )
    .map_err(to_store_error)?;
    Ok(())
}

pub(crate) fn active_chat_id(scope: &ProjectScope) -> Result<Option<String>, String> {
    let conn = connection()?;
    let key = active_chat_key(scope);
    let raw: Option<String> = conn
        .query_row(
            "SELECT value_json FROM chat_settings WHERE key = ?1",
            [key],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    let Some(raw) = raw else {
        return Ok(None);
    };
    Ok(serde_json::from_str::<Option<String>>(&raw)
        .ok()
        .flatten()
        .filter(|id| !id.trim().is_empty()))
}

pub(super) struct NewMessage<'a> {
    pub(super) role: &'a str,
    pub(super) status: &'a str,
    pub(super) content: &'a str,
    pub(super) reasoning_content: Option<&'a str>,
    pub(super) usage_json: Option<&'a str>,
    pub(super) cost: Option<f64>,
    pub(super) tool_call_id: Option<&'a str>,
    pub(super) tool_name: Option<&'a str>,
    pub(super) metadata_json: Option<&'a str>,
}

pub(super) fn insert_message_in_tx(
    conn: &Connection,
    chat_id: &str,
    message: NewMessage<'_>,
    now: i64,
) -> Result<StoredMessage, String> {
    if get_chat(conn, chat_id)?.is_none() {
        return Err("Chat not found.".to_string());
    }
    let message_id = new_id("msg");
    let sequence = next_sequence(conn, chat_id)?;
    conn.execute(
        "INSERT INTO chat_messages
         (id, chat_id, role, status, content, reasoning_content,
          tool_call_id, tool_name, metadata_json, usage_json, cost,
          sequence, created_at_ms, updated_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?13)",
        params![
            message_id,
            chat_id,
            message.role,
            message.status,
            message.content,
            message.reasoning_content,
            message.tool_call_id,
            message.tool_name,
            message.metadata_json,
            message.usage_json,
            message.cost,
            sequence,
            now
        ],
    )
    .map_err(to_store_error)?;
    refresh_chat_rollups(conn, chat_id)?;
    get_message(conn, &message_id)?.ok_or_else(|| "Message not found.".to_string())
}

fn next_sequence(conn: &Connection, chat_id: &str) -> Result<i64, String> {
    let max_sequence: Option<i64> = conn
        .query_row(
            "SELECT MAX(sequence) FROM chat_messages WHERE chat_id = ?1",
            [chat_id],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    Ok(max_sequence.unwrap_or(0) + 1)
}

fn get_chat(conn: &Connection, chat_id: &str) -> Result<Option<ChatSummary>, String> {
    conn.query_row(
        "SELECT id, title, project_path, project_name, model, reasoning_effort, total_cost, latest_prompt_tokens, message_count, created_at_ms, updated_at_ms
         FROM chats
         WHERE id = ?1 AND archived_at_ms IS NULL",
        [chat_id],
        chat_from_row,
    )
    .optional()
    .map_err(to_store_error)
}

fn get_chat_for_scope(
    conn: &Connection,
    scope: &ProjectScope,
    chat_id: &str,
) -> Result<Option<ChatSummary>, String> {
    conn.query_row(
        "SELECT id, title, project_path, project_name, model, reasoning_effort, total_cost, latest_prompt_tokens, message_count, created_at_ms, updated_at_ms
         FROM chats
         WHERE id = ?1 AND archived_at_ms IS NULL AND project_path IS ?2",
        params![chat_id, scope.project_path.as_deref()],
        chat_from_row,
    )
    .optional()
    .map_err(to_store_error)
}

pub(super) fn get_message(
    conn: &Connection,
    message_id: &str,
) -> Result<Option<StoredMessage>, String> {
    conn.query_row(
        "SELECT id, chat_id, role, status, content, reasoning_content, tool_call_id, tool_name, tool_calls_json, metadata_json, usage_json, cost, sequence, created_at_ms, updated_at_ms
         FROM chat_messages
         WHERE id = ?1",
        [message_id],
        message_from_row,
    )
    .optional()
    .map_err(to_store_error)
}

fn messages_for_chat(conn: &Connection, chat_id: &str) -> Result<Vec<StoredMessage>, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, chat_id, role, status, content, reasoning_content, tool_call_id, tool_name, tool_calls_json, metadata_json, usage_json, cost, sequence, created_at_ms, updated_at_ms
             FROM chat_messages
             WHERE chat_id = ?1
             ORDER BY sequence ASC",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map([chat_id], message_from_row)
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(to_store_error)
}

fn context_compactions_for_chat(
    conn: &Connection,
    chat_id: &str,
) -> Result<Vec<ContextCompactionMarker>, String> {
    let mut markers: Vec<_> = context_compaction::load_context_summaries_from_conn(conn, chat_id)?
        .into_iter()
        .map(context_compaction_marker)
        .collect();
    markers.sort_by_key(|marker| (marker.created_at_ms, marker.covered_end_sequence));
    Ok(markers)
}

fn context_compaction_marker(summary: ContextSummaryChunk) -> ContextCompactionMarker {
    ContextCompactionMarker {
        id: summary.id,
        chat_id: summary.chat_id,
        created_at_ms: summary.created_at_ms,
        covered_start_sequence: summary.covered_start_sequence,
        covered_end_sequence: summary.covered_end_sequence,
        source_message_count: summary.source_message_count,
    }
}

fn refresh_chat_rollups(conn: &Connection, chat_id: &str) -> Result<(), String> {
    let now = now_ms();
    let message_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM chat_messages WHERE chat_id = ?1",
            [chat_id],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    let total_cost = total_cost_for_chat(conn, chat_id)?;
    let latest_prompt_tokens = latest_prompt_tokens_for_chat(conn, chat_id)?;
    let first_user: Option<String> = conn
        .query_row(
            "SELECT content FROM chat_messages WHERE chat_id = ?1 AND role = 'user' ORDER BY sequence ASC LIMIT 1",
            [chat_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    let title = first_user
        .as_deref()
        .map(chat_title)
        .unwrap_or_else(|| "New chat".to_string());
    conn.execute(
        "UPDATE chats
         SET title = ?2, message_count = ?3, total_cost = ?4, latest_prompt_tokens = ?5, updated_at_ms = ?6
         WHERE id = ?1",
        params![
            chat_id,
            title,
            message_count,
            total_cost,
            latest_prompt_tokens,
            now
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}

fn chat_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ChatSummary> {
    Ok(ChatSummary {
        id: row.get(0)?,
        title: row.get(1)?,
        project_path: row.get(2)?,
        project_name: row.get(3)?,
        model: row.get(4)?,
        reasoning_effort: row.get(5)?,
        total_cost: row.get(6)?,
        latest_prompt_tokens: row.get(7)?,
        message_count: row.get(8)?,
        created_at_ms: row.get(9)?,
        updated_at_ms: row.get(10)?,
    })
}

fn message_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredMessage> {
    Ok(StoredMessage {
        id: row.get(0)?,
        chat_id: row.get(1)?,
        role: row.get(2)?,
        status: row.get(3)?,
        content: row.get(4)?,
        reasoning_content: row.get(5)?,
        tool_call_id: row.get(6)?,
        tool_name: row.get(7)?,
        tool_calls_json: row.get(8)?,
        metadata_json: row.get(9)?,
        usage_json: row.get(10)?,
        cost: row.get(11)?,
        sequence: row.get(12)?,
        created_at_ms: row.get(13)?,
        updated_at_ms: row.get(14)?,
    })
}

fn chat_title(content: &str) -> String {
    let title = content
        .split_whitespace()
        .take(8)
        .collect::<Vec<_>>()
        .join(" ");
    if title.chars().count() > 60 {
        title.chars().take(57).collect::<String>() + "..."
    } else if title.is_empty() {
        "New chat".to_string()
    } else {
        title
    }
}

fn active_chat_key(scope: &ProjectScope) -> String {
    format!("active_chat_id:{}", scope.key())
}

fn normalize_project_path(path: &str) -> String {
    path.trim().replace('\\', "/").to_lowercase()
}

#[cfg(test)]
mod tests;
