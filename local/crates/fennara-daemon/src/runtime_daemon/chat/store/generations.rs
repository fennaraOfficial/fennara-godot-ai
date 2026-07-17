use rusqlite::{Connection, params};
use serde_json::Value;

use super::super::{
    ids::{new_id, now_ms},
    providers::custom::CustomProviderConfig,
    schema::{ModelTrace, model_trace_from_selection, to_store_error},
};
use super::{NewMessage, StartedGeneration, StoredMessage, get_message, insert_message_in_tx};

pub(super) fn insert_assistant_placeholder_with_generation_on_connection(
    conn: &mut Connection,
    chat_id: &str,
    model: &str,
    reasoning_effort: &str,
    custom_providers: &[CustomProviderConfig],
) -> Result<(StoredMessage, StartedGeneration), String> {
    let tx = conn
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    let now = now_ms();
    let placeholder = insert_message_in_tx(
        &tx,
        chat_id,
        NewMessage {
            role: "assistant",
            status: "in_progress",
            content: "",
            reasoning_content: None,
            usage_json: None,
            cost: None,
            tool_call_id: None,
            tool_name: None,
            metadata_json: None,
        },
        now,
    )?;
    let generation = start_generation_on_connection(
        &tx,
        chat_id,
        &placeholder.id,
        model,
        reasoning_effort,
        custom_providers,
    )?;
    let placeholder = get_message(&tx, &placeholder.id)?
        .ok_or_else(|| "Assistant message not found.".to_string())?;
    tx.commit().map_err(to_store_error)?;
    Ok((placeholder, generation))
}

fn start_generation_on_connection(
    conn: &Connection,
    chat_id: &str,
    assistant_message_id: &str,
    model: &str,
    reasoning_effort: &str,
    custom_providers: &[CustomProviderConfig],
) -> Result<StartedGeneration, String> {
    if get_message(conn, assistant_message_id)?.is_none() {
        return Err("Assistant message not found.".to_string());
    }
    let now = now_ms();
    let generation_id = new_id("gen");
    let model_trace = model_trace_from_selection(model, custom_providers);
    conn.execute(
        "INSERT INTO chat_generations
         (id, chat_id, assistant_message_id, provider_id, model_id, model_variant,
          model_ref_json, reasoning_effort, status, started_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'running', ?9)",
        params![
            generation_id,
            chat_id,
            assistant_message_id,
            model_trace.as_ref().map(|trace| trace.provider_id.as_str()),
            model_trace.as_ref().map(|trace| trace.model_id.as_str()),
            model_trace
                .as_ref()
                .and_then(|trace| trace.model_variant.as_deref()),
            model_trace
                .as_ref()
                .map(|trace| trace.model_ref_json.as_str()),
            reasoning_effort,
            now
        ],
    )
    .map_err(to_store_error)?;
    set_message_generation_trace(
        conn,
        assistant_message_id,
        Some(&generation_id),
        model_trace.as_ref(),
    )?;
    Ok(StartedGeneration { id: generation_id })
}

pub(super) fn finish_generation_on_connection(
    conn: &Connection,
    generation_id: &str,
    status: &str,
    error: Option<&Value>,
) -> Result<(), String> {
    let now = now_ms();
    let error_json = error
        .map(serde_json::to_string)
        .transpose()
        .map_err(|error| error.to_string())?;
    conn.execute(
        "UPDATE chat_generations
         SET status = ?2,
             error_json = ?3,
             finished_at_ms = ?4
         WHERE id = ?1",
        params![generation_id, status, error_json, now],
    )
    .map_err(to_store_error)?;
    Ok(())
}

fn set_message_generation_trace(
    conn: &Connection,
    message_id: &str,
    generation_id: Option<&str>,
    model_trace: Option<&ModelTrace>,
) -> Result<(), String> {
    let now = now_ms();
    conn.execute(
        "UPDATE chat_messages
         SET generation_id = COALESCE(?2, generation_id),
             provider_id = COALESCE(?3, provider_id),
             model_id = COALESCE(?4, model_id),
             model_variant = COALESCE(?5, model_variant),
             model_ref_json = COALESCE(?6, model_ref_json),
             updated_at_ms = ?7
         WHERE id = ?1",
        params![
            message_id,
            generation_id,
            model_trace.map(|trace| trace.provider_id.as_str()),
            model_trace.map(|trace| trace.model_id.as_str()),
            model_trace.and_then(|trace| trace.model_variant.as_deref()),
            model_trace.map(|trace| trace.model_ref_json.as_str()),
            now
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}
