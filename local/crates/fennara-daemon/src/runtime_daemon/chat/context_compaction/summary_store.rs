use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params};
use serde_json::Value;

use super::super::{
    ids::{new_id, now_ms},
    providers::custom::CustomProviderConfig,
    schema::{model_trace_from_selection, to_store_error},
};
use super::{summary::SummaryCandidate, types::ContextSummaryChunk};

#[derive(Clone, Debug)]
pub(crate) struct InsertContextSummary<'a> {
    pub(crate) chat_id: &'a str,
    pub(crate) generation_id: &'a str,
    pub(crate) summary_markdown: &'a str,
    pub(crate) candidate: &'a SummaryCandidate,
    pub(crate) model: &'a str,
    pub(crate) reasoning_effort: &'a str,
    pub(crate) usage: Option<&'a Value>,
    pub(crate) metadata: &'a Value,
    pub(crate) custom_providers: &'a [CustomProviderConfig],
}

pub(crate) fn load_context_summaries_from_conn(
    conn: &Connection,
    chat_id: &str,
) -> Result<Vec<ContextSummaryChunk>, String> {
    let mut statement = conn
        .prepare(
            "SELECT id, chat_id, generation_id, summary_markdown,
                    covered_start_message_id, covered_start_sequence,
                    covered_end_message_id, covered_end_sequence,
                    tail_start_message_id, tail_start_sequence,
                    source_message_count, model, provider_id, model_id,
                    model_variant, model_ref_json, metadata_json, created_at_ms
             FROM chat_context_summaries
             WHERE chat_id = ?1
             ORDER BY covered_start_sequence ASC, created_at_ms ASC",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map([chat_id], context_summary_from_row)
        .map_err(to_store_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(to_store_error)
}

pub(crate) fn insert_context_summary_on_connection(
    conn: &mut Connection,
    input: InsertContextSummary<'_>,
) -> Result<ContextSummaryChunk, String> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(to_store_error)?;
    validate_summary_insert(&tx, input.chat_id, input.candidate)?;

    let summary_id = new_id("ctxsum");
    let now = now_ms();
    let model_trace = model_trace_from_selection(input.model, input.custom_providers);
    let metadata_json = serde_json::to_string(input.metadata).map_err(|error| error.to_string())?;
    tx.execute(
        "INSERT INTO chat_context_summaries
         (id, chat_id, generation_id, summary_markdown,
          covered_start_message_id, covered_start_sequence,
          covered_end_message_id, covered_end_sequence,
          tail_start_message_id, tail_start_sequence,
          source_message_count, model, reasoning_effort,
          provider_id, model_id, model_variant, model_ref_json,
          metadata_json, created_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            summary_id,
            input.chat_id,
            input.generation_id,
            input.summary_markdown,
            input.candidate.covered_start_message_id.as_str(),
            input.candidate.covered_start_sequence,
            input.candidate.covered_end_message_id.as_str(),
            input.candidate.covered_end_sequence,
            input.candidate.tail_start_message_id.as_deref(),
            input.candidate.tail_start_sequence,
            input.candidate.source_message_count,
            input.model,
            input.reasoning_effort,
            model_trace.as_ref().map(|trace| trace.provider_id.as_str()),
            model_trace.as_ref().map(|trace| trace.model_id.as_str()),
            model_trace
                .as_ref()
                .and_then(|trace| trace.model_variant.as_deref()),
            model_trace
                .as_ref()
                .map(|trace| trace.model_ref_json.as_str()),
            metadata_json,
            now
        ],
    )
    .map_err(to_store_error)?;

    let usage = usage_with_fallbacks(input.usage, input.model, input.metadata);
    insert_usage_log(
        &tx,
        input.chat_id,
        input.generation_id,
        input.model,
        "context_summary",
        &usage,
        model_trace.as_ref(),
        now,
    )?;
    tx.commit().map_err(to_store_error)?;

    let summary = conn
        .query_row(
            "SELECT id, chat_id, generation_id, summary_markdown,
                    covered_start_message_id, covered_start_sequence,
                    covered_end_message_id, covered_end_sequence,
                    tail_start_message_id, tail_start_sequence,
                    source_message_count, model, provider_id, model_id,
                    model_variant, model_ref_json, metadata_json, created_at_ms
             FROM chat_context_summaries
             WHERE id = ?1",
            [summary_id],
            context_summary_from_row,
        )
        .map_err(to_store_error)?;
    Ok(summary)
}

fn validate_summary_insert(
    conn: &Connection,
    chat_id: &str,
    candidate: &SummaryCandidate,
) -> Result<(), String> {
    if candidate.covered_start_sequence > candidate.covered_end_sequence {
        return Err("Context summary coverage is empty.".to_string());
    }
    let first_sequence: Option<i64> = conn
        .query_row(
            "SELECT MIN(sequence) FROM chat_messages WHERE chat_id = ?1",
            [chat_id],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    if first_sequence.is_none() {
        return Err("Cannot summarize an empty chat.".to_string());
    }

    let max_existing_end: Option<i64> = conn
        .query_row(
            "SELECT MAX(covered_end_sequence)
             FROM chat_context_summaries
             WHERE chat_id = ?1",
            [chat_id],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    let supersedes_existing_summary = Some(candidate.covered_start_sequence) == first_sequence
        && max_existing_end
            .map(|end| candidate.covered_end_sequence > end)
            .unwrap_or(false);

    let overlap_count: i64 = conn
        .query_row(
            "SELECT COUNT(*)
             FROM chat_context_summaries
             WHERE chat_id = ?1
               AND covered_start_sequence <= ?3
               AND covered_end_sequence >= ?2",
            params![
                chat_id,
                candidate.covered_start_sequence,
                candidate.covered_end_sequence
            ],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    if overlap_count > 0 && !supersedes_existing_summary {
        return Err("Context summary coverage overlaps an existing summary.".to_string());
    }

    let latest_end_before: Option<i64> = conn
        .query_row(
            "SELECT covered_end_sequence
             FROM chat_context_summaries
             WHERE chat_id = ?1
               AND covered_end_sequence < ?2
             ORDER BY covered_end_sequence DESC
             LIMIT 1",
            params![chat_id, candidate.covered_start_sequence],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    match latest_end_before {
        Some(end) if candidate.covered_start_sequence != end.saturating_add(1) => {
            return Err("Context summary coverage must be contiguous.".to_string());
        }
        None if Some(candidate.covered_start_sequence) != first_sequence => {
            return Err("First context summary must begin at the first chat message.".to_string());
        }
        _ => {}
    }

    let source_count: i64 = conn
        .query_row(
            "SELECT COUNT(*)
             FROM chat_messages
             WHERE chat_id = ?1
               AND sequence BETWEEN ?2 AND ?3",
            params![
                chat_id,
                candidate.covered_start_sequence,
                candidate.covered_end_sequence
            ],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    if source_count != candidate.source_message_count {
        return Err("Context summary source message count changed before insert.".to_string());
    }

    let start_id: Option<String> = conn
        .query_row(
            "SELECT id FROM chat_messages WHERE chat_id = ?1 AND sequence = ?2",
            params![chat_id, candidate.covered_start_sequence],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    if start_id.as_deref() != Some(candidate.covered_start_message_id.as_str()) {
        return Err("Context summary start message changed before insert.".to_string());
    }
    let end_id: Option<String> = conn
        .query_row(
            "SELECT id FROM chat_messages WHERE chat_id = ?1 AND sequence = ?2",
            params![chat_id, candidate.covered_end_sequence],
            |row| row.get(0),
        )
        .optional()
        .map_err(to_store_error)?;
    if end_id.as_deref() != Some(candidate.covered_end_message_id.as_str()) {
        return Err("Context summary end message changed before insert.".to_string());
    }
    Ok(())
}

fn insert_usage_log(
    conn: &Connection,
    chat_id: &str,
    generation_id: &str,
    fallback_model: &str,
    agent_type: &str,
    usage: &Value,
    model_trace: Option<&super::super::schema::ModelTrace>,
    now: i64,
) -> Result<(), String> {
    let model = usage_string(usage, "model").unwrap_or_else(|| fallback_model.to_string());
    conn.execute(
        "INSERT INTO chat_usage_logs
         (id, chat_id, assistant_message_id, generation_id, model,
          provider_id, model_id, model_variant, model_ref_json,
          agent_type, prompt_tokens,
          completion_tokens, total_tokens, reasoning_tokens, cached_tokens,
          cache_write_tokens, cost, upstream_cost, provider_name, created_at_ms)
         VALUES (?1, ?2, NULL, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            new_id("usage"),
            chat_id,
            generation_id,
            model,
            model_trace.map(|trace| trace.provider_id.as_str()),
            model_trace.map(|trace| trace.model_id.as_str()),
            model_trace.and_then(|trace| trace.model_variant.as_deref()),
            model_trace.map(|trace| trace.model_ref_json.as_str()),
            agent_type,
            usage_i64_any(usage, &["prompt_tokens", "promptTokens", "input_tokens", "inputTokens"]),
            usage_i64_any(usage, &["completion_tokens", "completionTokens", "output_tokens", "outputTokens"]),
            usage_i64_any(usage, &["total_tokens", "totalTokens"]),
            usage_i64_any(usage, &["reasoning_tokens", "reasoningTokens"]),
            usage_i64_any(usage, &[
                "cached_tokens",
                "cachedTokens",
                "cache_read_tokens",
                "cacheReadTokens",
                "cache_read_input_tokens",
                "cacheReadInputTokens",
                "cachedInputTokens"
            ]),
            usage_i64_any(usage, &[
                "cache_write_tokens",
                "cacheWriteTokens",
                "cache_creation_input_tokens",
                "cacheWriteInputTokens"
            ]),
            usage_f64(usage, "cost", "total_cost").unwrap_or(0.0),
            usage_f64(usage, "upstream_cost", "upstreamCost"),
            usage_string(usage, "provider_name"),
            now
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}

fn usage_with_fallbacks(usage: Option<&Value>, model: &str, metadata: &Value) -> Value {
    let mut value = usage.cloned().unwrap_or_else(|| serde_json::json!({}));
    if let Some(object) = value.as_object_mut() {
        object
            .entry("model".to_string())
            .or_insert_with(|| Value::String(model.to_string()));
        for key in ["prompt_tokens", "completion_tokens", "total_tokens"] {
            if !object.contains_key(key) {
                if let Some(number) = metadata.get(key).and_then(Value::as_i64) {
                    object.insert(key.to_string(), Value::Number(number.into()));
                }
            }
        }
    }
    value
}

fn usage_i64_any(usage: &Value, keys: &[&str]) -> i64 {
    keys.iter()
        .find_map(|key| usage.get(*key).and_then(Value::as_i64))
        .unwrap_or(0)
}

fn usage_f64(usage: &Value, snake_key: &str, camel_key: &str) -> Option<f64> {
    usage
        .get(snake_key)
        .or_else(|| usage.get(camel_key))
        .and_then(Value::as_f64)
}

fn usage_string(usage: &Value, key: &str) -> Option<String> {
    usage
        .get(key)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn context_summary_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ContextSummaryChunk> {
    Ok(ContextSummaryChunk {
        id: row.get(0)?,
        chat_id: row.get(1)?,
        generation_id: row.get(2)?,
        summary_markdown: row.get(3)?,
        covered_start_message_id: row.get(4)?,
        covered_start_sequence: row.get(5)?,
        covered_end_message_id: row.get(6)?,
        covered_end_sequence: row.get(7)?,
        tail_start_message_id: row.get(8)?,
        tail_start_sequence: row.get(9)?,
        source_message_count: row.get(10)?,
        model: row.get(11)?,
        provider_id: row.get(12)?,
        model_id: row.get(13)?,
        model_variant: row.get(14)?,
        model_ref_json: row.get(15)?,
        metadata_json: row.get(16)?,
        created_at_ms: row.get(17)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn insert_validates_contiguous_coverage_and_records_context_summary_usage() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE chats (
              id TEXT PRIMARY KEY,
              model TEXT NOT NULL,
              reasoning_effort TEXT NOT NULL,
              created_at_ms INTEGER NOT NULL,
              updated_at_ms INTEGER NOT NULL
            );
            CREATE TABLE chat_messages (
              id TEXT PRIMARY KEY,
              chat_id TEXT NOT NULL,
              role TEXT NOT NULL,
              status TEXT NOT NULL,
              content TEXT NOT NULL,
              sequence INTEGER NOT NULL,
              created_at_ms INTEGER NOT NULL,
              updated_at_ms INTEGER NOT NULL
            );
            CREATE TABLE chat_context_summaries (
              id TEXT PRIMARY KEY,
              chat_id TEXT NOT NULL,
              generation_id TEXT,
              summary_markdown TEXT NOT NULL,
              covered_start_message_id TEXT,
              covered_start_sequence INTEGER NOT NULL,
              covered_end_message_id TEXT NOT NULL,
              covered_end_sequence INTEGER NOT NULL,
              tail_start_message_id TEXT,
              tail_start_sequence INTEGER,
              source_message_count INTEGER NOT NULL DEFAULT 0,
              model TEXT,
              reasoning_effort TEXT,
              provider_id TEXT,
              model_id TEXT,
              model_variant TEXT,
              model_ref_json TEXT,
              metadata_json TEXT,
              created_at_ms INTEGER NOT NULL
            );
            CREATE TABLE chat_usage_logs (
              id TEXT PRIMARY KEY,
              chat_id TEXT NOT NULL,
              assistant_message_id TEXT,
              generation_id TEXT,
              model TEXT NOT NULL,
              provider_id TEXT,
              model_id TEXT,
              model_variant TEXT,
              model_ref_json TEXT,
              agent_type TEXT NOT NULL DEFAULT 'chat',
              prompt_tokens INTEGER NOT NULL DEFAULT 0,
              completion_tokens INTEGER NOT NULL DEFAULT 0,
              total_tokens INTEGER NOT NULL DEFAULT 0,
              reasoning_tokens INTEGER NOT NULL DEFAULT 0,
              cached_tokens INTEGER NOT NULL DEFAULT 0,
              cache_write_tokens INTEGER NOT NULL DEFAULT 0,
              cost REAL NOT NULL DEFAULT 0,
              upstream_cost REAL,
              provider_name TEXT,
              created_at_ms INTEGER NOT NULL
            );
            INSERT INTO chats (id, model, reasoning_effort, created_at_ms, updated_at_ms)
              VALUES ('chat_1', 'openrouter/z-ai/glm-5.2', 'medium', 1, 1);
            INSERT INTO chat_messages
              (id, chat_id, role, status, content, sequence, created_at_ms, updated_at_ms)
              VALUES
              ('msg_1', 'chat_1', 'user', 'done', 'one', 1, 1, 1),
              ('msg_2', 'chat_1', 'assistant', 'done', 'two', 2, 1, 1);
            ",
        )
        .unwrap();
        let candidate = SummaryCandidate {
            groups: Vec::new(),
            previous_summary_markdown: None,
            covered_start_message_id: "msg_1".to_string(),
            covered_start_sequence: 1,
            covered_end_message_id: "msg_1".to_string(),
            covered_end_sequence: 1,
            tail_start_message_id: Some("msg_2".to_string()),
            tail_start_sequence: Some(2),
            source_message_count: 1,
        };

        insert_context_summary_on_connection(
            &mut conn,
            InsertContextSummary {
                chat_id: "chat_1",
                generation_id: "ctxgen_1",
                summary_markdown: "summary",
                candidate: &candidate,
                model: "openrouter/z-ai/glm-5.2",
                reasoning_effort: "medium",
                usage: Some(
                    &json!({ "prompt_tokens": 10, "completion_tokens": 2, "total_tokens": 12 }),
                ),
                metadata: &json!({ "trigger_estimated_tokens": 100 }),
                custom_providers: &[],
            },
        )
        .unwrap();

        let usage: (String, i64) = conn
            .query_row(
                "SELECT agent_type, prompt_tokens FROM chat_usage_logs",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(usage, ("context_summary".to_string(), 10));

        let cumulative_candidate = SummaryCandidate {
            groups: Vec::new(),
            previous_summary_markdown: Some("summary".to_string()),
            covered_start_message_id: "msg_1".to_string(),
            covered_start_sequence: 1,
            covered_end_message_id: "msg_2".to_string(),
            covered_end_sequence: 2,
            tail_start_message_id: None,
            tail_start_sequence: None,
            source_message_count: 2,
        };

        let cumulative = insert_context_summary_on_connection(
            &mut conn,
            InsertContextSummary {
                chat_id: "chat_1",
                generation_id: "ctxgen_2",
                summary_markdown: "updated summary",
                candidate: &cumulative_candidate,
                model: "openrouter/z-ai/glm-5.2",
                reasoning_effort: "medium",
                usage: None,
                metadata: &json!({}),
                custom_providers: &[],
            },
        )
        .unwrap();

        assert_eq!(cumulative.covered_start_sequence, 1);
        assert_eq!(cumulative.covered_end_sequence, 2);
    }
}
