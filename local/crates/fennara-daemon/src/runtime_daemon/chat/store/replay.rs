use rusqlite::{Connection, params};
use serde_json::{Value, json};

use super::super::{
    context,
    context_compaction::{
        self, CompactionAction, PlaceholderRenderer, PlannedReplayGroup, PlannedReplayRow,
        ReplayGroup, ReplayPlan, ReplayRow,
    },
    images::{self, ImagePlaceholder},
    schema::to_store_error,
    tools,
};

pub(super) fn replay_messages_from_conn(
    conn: &Connection,
    chat_id: &str,
) -> Result<Vec<Value>, String> {
    replay_messages_with_summary_budget_from_conn(
        conn,
        chat_id,
        Some(context_compaction::SUMMARY_REPLAY_BUDGET_MAX),
    )
}

pub(super) fn replay_messages_with_summary_budget_from_conn(
    conn: &Connection,
    chat_id: &str,
    summary_replay_budget_tokens: Option<usize>,
) -> Result<Vec<Value>, String> {
    let mut replay_groups = replay_groups_from_conn(conn, chat_id)?;
    if let Some(summary_replay_budget_tokens) = summary_replay_budget_tokens {
        let summaries = context_compaction::load_context_summaries_from_conn(conn, chat_id)?;
        replay_groups = context_compaction::apply_summary_replay(
            replay_groups,
            &summaries,
            summary_replay_budget_tokens,
        );
    }
    let replay_plan = context_compaction::plan_replay(replay_groups);
    Ok(render_replay_plan(replay_plan))
}

pub(super) fn replay_groups_from_conn(
    conn: &Connection,
    chat_id: &str,
) -> Result<Vec<ReplayGroup>, String> {
    let replay_rows = replay_rows_from_conn(conn, chat_id)?;
    Ok(context_compaction::sanitize_replay_groups(&replay_rows))
}

pub(super) fn raw_summary_groups_from_conn(
    conn: &Connection,
    chat_id: &str,
) -> Result<Vec<ReplayGroup>, String> {
    let summary_rows = summary_rows_from_conn(conn, chat_id)?;
    Ok(context_compaction::group_raw_summary_rows(&summary_rows))
}

fn replay_rows_from_conn(conn: &Connection, chat_id: &str) -> Result<Vec<ReplayRow>, String> {
    query_replay_rows(
        conn,
        chat_id,
        "SELECT
               id,
               sequence,
               role,
               status,
               content,
               tool_call_id,
               tool_name,
               tool_calls_json,
               metadata_json,
               raw_result_json,
               arguments_json,
               target_keys_json,
               tool_status
             FROM (
               SELECT
                 m.id,
                 m.sequence,
                 m.role,
                 m.status,
                 CASE
                   WHEN m.role = 'tool' THEN COALESCE(t.mcp_markdown, m.content)
                   ELSE m.content
                 END AS content,
                 m.tool_call_id,
                 m.tool_name,
                 m.tool_calls_json,
                 CASE
                   WHEN m.role = 'tool' THEN COALESCE(t.metadata_json, m.metadata_json)
                   ELSE m.metadata_json
                 END AS metadata_json,
                 t.raw_result_json,
                 t.arguments_json,
                 t.target_keys_json,
                 t.status AS tool_status
               FROM chat_messages m
               LEFT JOIN chat_tool_calls t ON t.id = m.tool_call_id
               LEFT JOIN chat_messages a ON a.id = t.assistant_message_id
               WHERE m.chat_id = ?1
                 AND (
                   m.status = 'done'
                   OR (
                     m.role = 'tool'
                     AND m.status IN ('failed', 'timed_out', 'cancelled', 'denied')
                     AND a.status = 'done'
                   )
                 )
                 AND m.role IN ('user', 'assistant', 'tool')
               ORDER BY m.sequence DESC
             )
             ORDER BY sequence ASC",
    )
}

fn summary_rows_from_conn(conn: &Connection, chat_id: &str) -> Result<Vec<ReplayRow>, String> {
    query_replay_rows(
        conn,
        chat_id,
        "SELECT
           m.id,
           m.sequence,
           m.role,
           m.status,
           CASE
             WHEN m.role = 'tool' THEN COALESCE(t.mcp_markdown, m.content)
             ELSE m.content
           END AS content,
           m.tool_call_id,
           m.tool_name,
           m.tool_calls_json,
           CASE
             WHEN m.role = 'tool' THEN COALESCE(t.metadata_json, m.metadata_json)
             ELSE m.metadata_json
           END AS metadata_json,
           t.raw_result_json,
           t.arguments_json,
           t.target_keys_json,
           t.status AS tool_status
         FROM chat_messages m
         LEFT JOIN chat_tool_calls t ON t.id = m.tool_call_id
         WHERE m.chat_id = ?1
           AND m.role IN ('user', 'assistant', 'tool')
         ORDER BY m.sequence ASC",
    )
}

fn query_replay_rows(
    conn: &Connection,
    chat_id: &str,
    sql: &str,
) -> Result<Vec<ReplayRow>, String> {
    let mut statement = conn.prepare(sql).map_err(to_store_error)?;
    let mut rows = statement.query(params![chat_id]).map_err(to_store_error)?;
    let mut replay_rows = Vec::new();

    while let Some(row) = rows.next().map_err(to_store_error)? {
        replay_rows.push(ReplayRow {
            id: row.get(0).map_err(to_store_error)?,
            sequence: row.get(1).map_err(to_store_error)?,
            role: row.get(2).map_err(to_store_error)?,
            status: row.get(3).map_err(to_store_error)?,
            content: row.get(4).map_err(to_store_error)?,
            tool_call_id: row.get(5).map_err(to_store_error)?,
            tool_name: row.get(6).map_err(to_store_error)?,
            tool_calls_json: row.get(7).map_err(to_store_error)?,
            metadata_json: row.get(8).map_err(to_store_error)?,
            raw_result_json: row.get(9).map_err(to_store_error)?,
            arguments_json: row.get(10).map_err(to_store_error)?,
            target_keys_json: row.get(11).map_err(to_store_error)?,
            tool_status: row.get(12).map_err(to_store_error)?,
        });
    }
    Ok(replay_rows)
}

fn render_replay_plan(plan: ReplayPlan) -> Vec<Value> {
    plan.groups
        .iter()
        .map(replay_messages_for_group)
        .flat_map(|group| group.into_iter())
        .collect()
}

fn replay_messages_for_group(group: &PlannedReplayGroup) -> Vec<Value> {
    group
        .rows
        .iter()
        .flat_map(replay_messages_for_row)
        .collect()
}

fn replay_messages_for_row(planned: &PlannedReplayRow) -> Vec<Value> {
    let row = &planned.row;
    let context_snippets = context::snippets_from_metadata(row.metadata_json.as_deref());
    let replay_content = match planned.action {
        CompactionAction::KeepExact => row.content.clone(),
        CompactionAction::ReplaceWithPlaceholder => planned
            .placeholder
            .as_ref()
            .map(PlaceholderRenderer::render)
            .unwrap_or_else(|| row.content.clone()),
    };
    let model_content = if row.role == "user" {
        context::message_with_context_snippets(&replay_content, &context_snippets)
    } else {
        replay_content
    };
    let image_placeholders = image_placeholders_from_metadata(row.metadata_json.as_deref());
    let message_content = if row.role == "user" && !image_placeholders.is_empty() {
        images::user_content_with_image_placeholders(&model_content, &image_placeholders)
    } else {
        json!(model_content)
    };
    let mut message = json!({ "role": row.role.as_str(), "content": message_content });
    if let Some(tool_call_id) = row.tool_call_id.as_deref() {
        message["tool_call_id"] = json!(tool_call_id);
    }
    if let Some(tool_name) = row.tool_name.as_deref() {
        message["name"] = json!(tool_name);
    }
    if let Some(tool_calls_json) = row.tool_calls_json.as_deref() {
        if let Ok(tool_calls) = serde_json::from_str::<Value>(tool_calls_json) {
            message["tool_calls"] = tool_calls;
        }
    }

    let mut messages = vec![message];
    if planned.action == CompactionAction::KeepExact {
        messages.extend(replay_tool_followups(
            &row.role,
            row.tool_name.as_deref(),
            row.raw_result_json.as_deref(),
        ));
    }
    messages
}

fn replay_tool_followups(
    role: &str,
    tool_name: Option<&str>,
    raw_result_json: Option<&str>,
) -> Vec<Value> {
    if role != "tool" {
        return Vec::new();
    }
    let Some(tool_name) = tool_name else {
        return Vec::new();
    };
    let Some(raw_result_json) = raw_result_json else {
        return Vec::new();
    };
    let Ok(raw_result) = serde_json::from_str::<Value>(raw_result_json) else {
        return Vec::new();
    };
    tools::model_followups_for(tool_name, &raw_result)
}

fn image_placeholders_from_metadata(metadata_json: Option<&str>) -> Vec<ImagePlaceholder> {
    let Some(metadata_json) = metadata_json else {
        return Vec::new();
    };
    let Ok(metadata) = serde_json::from_str::<Value>(metadata_json) else {
        return Vec::new();
    };
    let Some(images) = metadata.get("images") else {
        return Vec::new();
    };
    images
        .as_array()
        .map(|images| {
            images
                .iter()
                .filter_map(image_placeholder_from_value)
                .collect()
        })
        .unwrap_or_default()
}

fn image_placeholder_from_value(value: &Value) -> Option<ImagePlaceholder> {
    let object = value.as_object()?;
    let mime_type = object
        .get("mime_type")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("image")
        .to_string();
    let name = object
        .get("name")
        .or_else(|| object.get("description"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.chars().take(120).collect::<String>());
    Some(ImagePlaceholder { mime_type, name })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_image_metadata_replays_as_placeholders() {
        let metadata = json!({
            "images": [
                {
                    "base64": "a".repeat(320_000),
                    "mime_type": "image/png",
                    "name": "old.png",
                    "size_bytes": 240000
                }
            ]
        })
        .to_string();

        let placeholders = image_placeholders_from_metadata(Some(&metadata));

        assert_eq!(
            placeholders,
            vec![ImagePlaceholder {
                mime_type: "image/png".to_string(),
                name: Some("old.png".to_string())
            }]
        );
        let content = images::user_content_with_image_placeholders("see attached", &placeholders);
        assert!(
            content
                .to_string()
                .contains("[Attached image/png: old.png]")
        );
        assert!(!content.to_string().contains(&"a".repeat(256)));
    }
}
