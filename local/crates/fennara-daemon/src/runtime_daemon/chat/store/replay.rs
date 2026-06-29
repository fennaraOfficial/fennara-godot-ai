use rusqlite::{Connection, params};
use serde_json::{Value, json};
use std::collections::HashSet;

use super::super::{
    context,
    images::{self, ImagePlaceholder},
    schema::to_store_error,
    tools,
};

const REPLAY_MESSAGE_LIMIT: i64 = 40;
const REPLAY_SCAN_LIMIT: i64 = REPLAY_MESSAGE_LIMIT * 5;

#[derive(Clone, Debug)]
struct ReplayRow {
    role: String,
    content: String,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    tool_calls_json: Option<String>,
    metadata_json: Option<String>,
    raw_result_json: Option<String>,
}

pub(super) fn replay_messages_from_conn(
    conn: &Connection,
    chat_id: &str,
) -> Result<Vec<Value>, String> {
    let mut statement = conn
        .prepare(
            "SELECT
               role,
               content,
               tool_call_id,
               tool_name,
               tool_calls_json,
               metadata_json,
               raw_result_json
             FROM (
               SELECT
                 m.sequence,
                 m.role,
                 CASE
                   WHEN m.role = 'tool' THEN COALESCE(t.mcp_markdown, m.content)
                   ELSE m.content
                 END AS content,
                 m.tool_call_id,
                 m.tool_name,
                 m.tool_calls_json,
                 m.metadata_json,
                 t.raw_result_json
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
               LIMIT ?2
             )
             ORDER BY sequence ASC",
        )
        .map_err(to_store_error)?;
    let mut rows = statement
        .query(params![chat_id, REPLAY_SCAN_LIMIT])
        .map_err(to_store_error)?;
    let mut replay_rows = Vec::new();

    while let Some(row) = rows.next().map_err(to_store_error)? {
        replay_rows.push(ReplayRow {
            role: row.get(0).map_err(to_store_error)?,
            content: row.get(1).map_err(to_store_error)?,
            tool_call_id: row.get(2).map_err(to_store_error)?,
            tool_name: row.get(3).map_err(to_store_error)?,
            tool_calls_json: row.get(4).map_err(to_store_error)?,
            metadata_json: row.get(5).map_err(to_store_error)?,
            raw_result_json: row.get(6).map_err(to_store_error)?,
        });
    }

    Ok(compact_replay_groups(sanitized_replay_groups(&replay_rows)))
}

fn sanitized_replay_groups(rows: &[ReplayRow]) -> Vec<Vec<ReplayRow>> {
    let mut groups = Vec::new();
    let mut index = 0;
    while index < rows.len() {
        let row = &rows[index];
        if row.role == "tool" {
            index += 1;
            continue;
        }

        if row.role == "assistant" {
            if let Some(required_tool_ids) = required_tool_call_ids(row.tool_calls_json.as_deref())
            {
                if required_tool_ids.is_empty() {
                    index += 1;
                    continue;
                }

                let mut group = vec![row.clone()];
                let mut seen_tool_ids = HashSet::new();
                let mut next = index + 1;
                while next < rows.len() && rows[next].role == "tool" {
                    if let Some(tool_call_id) = rows[next].tool_call_id.as_deref() {
                        if required_tool_ids.contains(tool_call_id) {
                            seen_tool_ids.insert(tool_call_id.to_string());
                            group.push(rows[next].clone());
                        }
                    }
                    next += 1;
                }

                if seen_tool_ids == required_tool_ids {
                    groups.push(group);
                }
                index = next;
                continue;
            }
        }

        groups.push(vec![row.clone()]);
        index += 1;
    }

    groups
}

fn compact_replay_groups(groups: Vec<Vec<ReplayRow>>) -> Vec<Value> {
    let limit = REPLAY_MESSAGE_LIMIT as usize;
    let mut selected = Vec::new();
    let mut message_count = 0usize;

    for group in groups.into_iter().rev() {
        let messages = replay_messages_for_group(&group);
        if messages.is_empty() {
            continue;
        }
        if messages.len() > limit {
            if selected.is_empty() {
                continue;
            }
            break;
        }
        if message_count + messages.len() > limit {
            break;
        }
        message_count += messages.len();
        selected.push(messages);
    }

    selected
        .into_iter()
        .rev()
        .flat_map(|group| group.into_iter())
        .collect()
}

fn replay_messages_for_group(rows: &[ReplayRow]) -> Vec<Value> {
    rows.iter().flat_map(replay_messages_for_row).collect()
}

fn replay_messages_for_row(row: &ReplayRow) -> Vec<Value> {
    let context_snippets = context::snippets_from_metadata(row.metadata_json.as_deref());
    let model_content = if row.role == "user" {
        context::message_with_context_snippets(&row.content, &context_snippets)
    } else {
        row.content.clone()
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
    messages.extend(replay_tool_followups(
        &row.role,
        row.tool_name.as_deref(),
        row.raw_result_json.as_deref(),
    ));
    messages
}

fn required_tool_call_ids(tool_calls_json: Option<&str>) -> Option<HashSet<String>> {
    let tool_calls_json = tool_calls_json?;
    let Ok(tool_calls) = serde_json::from_str::<Value>(tool_calls_json) else {
        return None;
    };
    let Some(tool_calls) = tool_calls.as_array() else {
        return None;
    };
    if tool_calls.is_empty() {
        return None;
    }

    let mut ids = HashSet::new();
    for tool_call in tool_calls {
        let Some(id) = tool_call
            .get("id")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|id| !id.is_empty())
        else {
            return Some(HashSet::new());
        };
        ids.insert(id.to_string());
    }

    Some(ids)
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
