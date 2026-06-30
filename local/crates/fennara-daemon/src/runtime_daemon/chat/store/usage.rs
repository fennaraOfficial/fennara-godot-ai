use rusqlite::{Connection, params};
use serde_json::Value;

use super::super::{
    ids::{new_id, now_ms},
    schema::{ModelTrace, to_store_error},
};

pub(super) fn usage_cost(usage: &Value) -> Option<f64> {
    usage
        .get("cost")
        .or_else(|| usage.get("total_cost"))
        .and_then(Value::as_f64)
}

pub(super) fn record_usage_log(
    conn: &Connection,
    chat_id: &str,
    assistant_message_id: &str,
    fallback_model: &str,
    agent_type: &str,
    usage: &Value,
    generation_id: Option<&str>,
    model_trace: Option<&ModelTrace>,
) -> Result<(), String> {
    let now = now_ms();
    let model = usage_string(usage, "model").unwrap_or_else(|| fallback_model.to_string());
    let usage_generation = usage_string(usage, "generation_id");
    let usage_generation_id = generation_id.or(usage_generation.as_deref());
    conn.execute(
        "INSERT INTO chat_usage_logs
         (id, chat_id, assistant_message_id, generation_id, model,
          provider_id, model_id, model_variant, model_ref_json,
          agent_type, prompt_tokens,
          completion_tokens, total_tokens, reasoning_tokens, cached_tokens,
          cache_write_tokens, cost, upstream_cost, provider_name, created_at_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
         ON CONFLICT(assistant_message_id) WHERE assistant_message_id IS NOT NULL DO UPDATE SET
           generation_id = COALESCE(excluded.generation_id, chat_usage_logs.generation_id),
           model = excluded.model,
           provider_id = COALESCE(excluded.provider_id, chat_usage_logs.provider_id),
           model_id = COALESCE(excluded.model_id, chat_usage_logs.model_id),
           model_variant = COALESCE(excluded.model_variant, chat_usage_logs.model_variant),
           model_ref_json = COALESCE(excluded.model_ref_json, chat_usage_logs.model_ref_json),
           agent_type = excluded.agent_type,
           prompt_tokens = excluded.prompt_tokens,
           completion_tokens = excluded.completion_tokens,
           total_tokens = excluded.total_tokens,
           reasoning_tokens = excluded.reasoning_tokens,
           cached_tokens = excluded.cached_tokens,
           cache_write_tokens = excluded.cache_write_tokens,
           cost = excluded.cost,
           upstream_cost = excluded.upstream_cost,
           provider_name = excluded.provider_name",
        params![
            new_id("usage"),
            chat_id,
            assistant_message_id,
            usage_generation_id,
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
            usage_cost(usage).unwrap_or(0.0),
            usage_f64(usage, "upstream_cost", "upstreamCost"),
            usage_string(usage, "provider_name"),
            now
        ],
    )
    .map_err(to_store_error)?;
    Ok(())
}

pub(super) fn total_cost_for_chat(conn: &Connection, chat_id: &str) -> Result<f64, String> {
    let from_usage_logs: Option<f64> = conn
        .query_row(
            "SELECT SUM(cost) FROM chat_usage_logs WHERE chat_id = ?1",
            [chat_id],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    if let Some(cost) = from_usage_logs {
        return Ok(cost);
    }
    let from_messages: Option<f64> = conn
        .query_row(
            "SELECT SUM(cost) FROM chat_messages WHERE chat_id = ?1",
            [chat_id],
            |row| row.get(0),
        )
        .map_err(to_store_error)?;
    Ok(from_messages.unwrap_or(0.0))
}

pub(super) fn latest_prompt_tokens_for_chat(
    conn: &Connection,
    chat_id: &str,
) -> Result<i64, String> {
    let mut usage_statement = conn
        .prepare(
            "SELECT prompt_tokens, total_tokens
             FROM chat_usage_logs
             WHERE chat_id = ?1
               AND agent_type != 'context_summary'
             ORDER BY created_at_ms DESC",
        )
        .map_err(to_store_error)?;
    let usage_rows = usage_statement
        .query_map([chat_id], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
        })
        .map_err(to_store_error)?;
    for row in usage_rows {
        let (prompt_tokens, total_tokens) = row.map_err(to_store_error)?;
        if prompt_tokens > 0 {
            return Ok(prompt_tokens);
        }
        if total_tokens > 0 {
            return Ok(total_tokens);
        }
    }

    let mut statement = conn
        .prepare(
            "SELECT usage_json
             FROM chat_messages
             WHERE chat_id = ?1
               AND role = 'assistant'
               AND status = 'done'
               AND usage_json IS NOT NULL
               AND usage_json != ''
             ORDER BY sequence DESC",
        )
        .map_err(to_store_error)?;
    let rows = statement
        .query_map([chat_id], |row| row.get::<_, String>(0))
        .map_err(to_store_error)?;

    for row in rows {
        let usage_json = row.map_err(to_store_error)?;
        let Ok(usage) = serde_json::from_str::<Value>(&usage_json) else {
            continue;
        };
        if let Some(tokens) = usage_prompt_tokens(&usage) {
            return Ok(tokens);
        }
    }
    Ok(0)
}

fn usage_prompt_tokens(usage: &Value) -> Option<i64> {
    usage
        .get("prompt_tokens")
        .or_else(|| usage.get("promptTokens"))
        .and_then(Value::as_i64)
        .filter(|tokens| *tokens > 0)
        .or_else(|| {
            usage
                .get("total_tokens")
                .or_else(|| usage.get("totalTokens"))
                .and_then(Value::as_i64)
                .filter(|tokens| *tokens > 0)
        })
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
