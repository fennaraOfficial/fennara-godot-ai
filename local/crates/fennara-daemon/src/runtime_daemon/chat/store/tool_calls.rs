use rusqlite::{Connection, TransactionBehavior, params};
use serde_json::Value;

use super::super::{ids::now_ms, schema::to_store_error};
use super::{NewMessage, StoredMessage, insert_message_in_tx};

pub(super) fn upsert_tool_call_on_connection(
    conn: &Connection,
    chat_id: &str,
    assistant_message_id: &str,
    generation_id: Option<&str>,
    tool_call_id: &str,
    provider_tool_call_id: Option<&str>,
    tool_name: &str,
    arguments: &Value,
    status: &str,
) -> Result<(), String> {
    let now = now_ms();
    let args_json = serde_json::to_string(arguments).map_err(|error| error.to_string())?;
    let updated = conn.execute(
        "INSERT INTO chat_tool_calls
         (id, provider_tool_call_id, chat_id, assistant_message_id, generation_id, tool_name, arguments_json, status, created_at_ms, updated_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)
         ON CONFLICT(id) DO UPDATE SET
           provider_tool_call_id = COALESCE(excluded.provider_tool_call_id, chat_tool_calls.provider_tool_call_id),
           generation_id = COALESCE(excluded.generation_id, chat_tool_calls.generation_id),
           tool_name = excluded.tool_name,
           arguments_json = excluded.arguments_json,
           status = excluded.status,
           updated_at_ms = excluded.updated_at_ms
         WHERE chat_tool_calls.chat_id = excluded.chat_id
           AND chat_tool_calls.assistant_message_id = excluded.assistant_message_id",
        params![
            tool_call_id,
            provider_tool_call_id,
            chat_id,
            assistant_message_id,
            generation_id,
            tool_name,
            args_json,
            status,
            now
        ],
    )
    .map_err(to_store_error)?;
    if updated == 0 {
        return Err("Tool call id belongs to another chat turn.".to_string());
    }
    Ok(())
}

pub(super) fn finish_tool_call_with_message_on_connection(
    conn: &mut Connection,
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
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let now = now_ms();
    update_tool_call_result(
        &tx,
        tool_call_id,
        status,
        raw_result,
        mcp_markdown,
        plugin_markdown,
        metadata,
        target_keys,
        now,
    )?;
    let metadata_json = serde_json::to_string(metadata).map_err(|error| error.to_string())?;
    let message = insert_message_in_tx(
        &tx,
        chat_id,
        NewMessage {
            role: "tool",
            status,
            content: plugin_markdown,
            reasoning_content: None,
            usage_json: None,
            cost: None,
            tool_call_id: Some(tool_call_id),
            tool_name: Some(tool_name),
            metadata_json: Some(&metadata_json),
        },
        now,
    )?;
    tx.commit().map_err(to_store_error)?;
    Ok(message)
}

fn update_tool_call_result(
    conn: &Connection,
    tool_call_id: &str,
    status: &str,
    raw_result: &Value,
    mcp_markdown: &str,
    plugin_markdown: &str,
    metadata: &Value,
    target_keys: &[String],
    now: i64,
) -> Result<(), String> {
    let raw_json = serde_json::to_string(raw_result).map_err(|error| error.to_string())?;
    let metadata_json = serde_json::to_string(metadata).map_err(|error| error.to_string())?;
    let target_keys_json = serde_json::to_string(target_keys).map_err(|error| error.to_string())?;
    let updated = conn
        .execute(
            "UPDATE chat_tool_calls
         SET status = ?2,
             raw_result_json = ?3,
             mcp_markdown = ?4,
             plugin_markdown = ?5,
             metadata_json = ?6,
             target_keys_json = ?7,
             updated_at_ms = ?8
         WHERE id = ?1",
            params![
                tool_call_id,
                status,
                raw_json,
                mcp_markdown,
                plugin_markdown,
                metadata_json,
                target_keys_json,
                now
            ],
        )
        .map_err(to_store_error)?;
    if updated == 0 {
        return Err("Tool call not found.".to_string());
    }
    Ok(())
}
