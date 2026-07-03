use serde_json::{Value, json};

use super::{
    summary::{SUMMARY_OUTPUT_MAX_TOKENS, SummaryCandidate},
    types::{ReplayGroup, ReplayRow, parse_json},
};

pub(crate) const TOOL_OUTPUT_MAX_CHARS: usize = 2_000;
const JSON_FIELD_MAX_CHARS: usize = 2_000;

pub(crate) fn build_summary_messages(candidate: &SummaryCandidate) -> Vec<Value> {
    vec![
        json!({ "role": "system", "content": SUMMARY_SYSTEM_PROMPT }),
        json!({
            "role": "user",
            "content": format!(
                "<history-to-summarize>\n{}\n</history-to-summarize>",
                render_history_to_summarize(&candidate.groups)
            )
        }),
    ]
}

pub(crate) fn summary_output_max_tokens() -> u32 {
    SUMMARY_OUTPUT_MAX_TOKENS
}

pub(crate) fn render_history_to_summarize(groups: &[ReplayGroup]) -> String {
    groups
        .iter()
        .flat_map(|group| group.rows.iter().map(render_row))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn render_row(row: &ReplayRow) -> String {
    match row.role.as_str() {
        "user" => render_user(row),
        "assistant" => render_assistant(row),
        "tool" => render_tool(row),
        _ => render_generic(row),
    }
}

fn render_user(row: &ReplayRow) -> String {
    let mut lines = vec![format!("[User message {} seq {}]", row.id, row.sequence)];
    lines.push(strip_media_from_user_content(row));
    lines.join("\n")
}

fn render_assistant(row: &ReplayRow) -> String {
    let mut lines = vec![format!(
        "[Assistant message {} seq {} status {}]",
        row.id, row.sequence, row.status
    )];
    if !row.content.trim().is_empty() {
        lines.push(row.content.clone());
    }
    if let Some(tool_calls_json) = row.tool_calls_json.as_deref() {
        lines.push(format!("Tool calls: {}", compact_json(tool_calls_json)));
    }
    lines.join("\n")
}

fn render_tool(row: &ReplayRow) -> String {
    let mut lines = vec![format!(
        "[Tool result {} seq {} status {}]",
        row.tool_name.as_deref().unwrap_or("tool"),
        row.sequence,
        row.tool_status.as_deref().unwrap_or(row.status.as_str())
    )];
    if let Some(tool_call_id) = row.tool_call_id.as_deref() {
        lines.push(format!("Tool call id: {tool_call_id}"));
    }
    if let Some(targets) = target_keys(row) {
        lines.push(format!("Target keys: {}", targets.join(", ")));
    }
    if let Some(arguments_json) = row.arguments_json.as_deref() {
        lines.push(format!("Arguments: {}", compact_json(arguments_json)));
    }
    lines.push(truncate_tool_output(&row.content));
    lines.join("\n")
}

fn render_generic(row: &ReplayRow) -> String {
    format!(
        "[{} message {} seq {} status {}]\n{}",
        row.role, row.id, row.sequence, row.status, row.content
    )
}

fn strip_media_from_user_content(row: &ReplayRow) -> String {
    let mut content = row.content.clone();
    if let Some(metadata) = parse_json(row.metadata_json.as_deref()) {
        let placeholders = image_placeholders(&metadata);
        if !placeholders.is_empty() {
            if !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(&placeholders.join("\n"));
        }
    }
    content
}

fn image_placeholders(metadata: &Value) -> Vec<String> {
    metadata
        .get("images")
        .and_then(Value::as_array)
        .map(|images| {
            images
                .iter()
                .filter_map(|image| {
                    let object = image.as_object()?;
                    let mime = object
                        .get("mime_type")
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .unwrap_or("image");
                    let label = object
                        .get("name")
                        .or_else(|| object.get("description"))
                        .and_then(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty());
                    Some(match label {
                        Some(label) => format!("[Attached {mime}: {label}]"),
                        None => format!("[Attached {mime}]"),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn target_keys(row: &ReplayRow) -> Option<Vec<String>> {
    let Value::Array(values) = parse_json(row.target_keys_json.as_deref())? else {
        return None;
    };
    let targets = values
        .iter()
        .filter_map(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if targets.is_empty() {
        None
    } else {
        Some(targets)
    }
}

fn truncate_tool_output(content: &str) -> String {
    truncate_text_with_marker(
        content,
        TOOL_OUTPUT_MAX_CHARS,
        "Tool output truncated for compaction",
    )
}

fn truncate_json_field(content: &str) -> String {
    truncate_text_with_marker(
        content,
        JSON_FIELD_MAX_CHARS,
        "JSON truncated for compaction",
    )
}

fn truncate_text_with_marker(content: &str, max_chars: usize, marker: &str) -> String {
    let char_count = content.chars().count();
    if char_count <= max_chars {
        return content.to_string();
    }
    let truncated = content.chars().take(max_chars).collect::<String>();
    let omitted = char_count.saturating_sub(max_chars);
    format!("{truncated}\n[{marker}: omitted {omitted} chars]")
}

fn compact_json(raw: &str) -> String {
    let compacted = serde_json::from_str::<Value>(raw)
        .and_then(|value| serde_json::to_string(&value))
        .unwrap_or_else(|_| raw.to_string());
    truncate_json_field(&compacted)
}

const SUMMARY_SYSTEM_PROMPT: &str = r#"You are Fennara's context checkpoint summarization assistant for coding and Godot editor sessions.

Summarize only the conversation history provided in <history-to-summarize>. This summary will be used by another LLM to continue the same task later.

The newest turns may be kept verbatim outside your summary, so focus on older context that still matters. Do not summarize retained exact tail content unless it is explicitly included in <history-to-summarize>.

Important rules:
- Do not answer the user's task.
- Do not mention compaction, summarization, token limits, or that context was shortened.
- Preserve exact file paths, Godot resource paths, scene paths, node paths, command strings, error text, tool names, tool call ids, and user decisions when known.
- Preserve user preferences and constraints, especially "do not do X" instructions.
- If tool output is truncated, summarize only visible facts and say exact details are in stored Fennara history when relevant. Do not invent missing output.
- Keep bullets terse and useful. Prefer concrete facts over vague prose.
- Respond in the same language/style as the conversation when possible.

Output exactly this Markdown structure:

## Goal
- ...

## Constraints & Preferences
- ...

## Progress
### Done
- ...

### In Progress
- ...

### Blocked
- ...

## Key Decisions
- ...

## Tool And Runtime Facts
- ...

## Files, Scenes, Nodes, And Artifacts
- ...

## Errors And Diagnostics
- ...

## Next Steps
- ...

## Open Questions
- ...
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_prompt_truncates_tool_output_and_strips_media() {
        let groups = vec![
            ReplayGroup::new(vec![ReplayRow {
                id: "msg_user".to_string(),
                sequence: 1,
                role: "user".to_string(),
                status: "done".to_string(),
                content: "look".to_string(),
                tool_call_id: None,
                tool_name: None,
                tool_calls_json: None,
                metadata_json: Some(
                    json!({ "images": [{ "mime_type": "image/png", "name": "screen.png", "base64": "x".repeat(100) }] })
                        .to_string(),
                ),
                raw_result_json: None,
                arguments_json: None,
                target_keys_json: None,
                tool_status: None,
            }]),
            ReplayGroup::new(vec![ReplayRow {
                id: "msg_tool".to_string(),
                sequence: 2,
                role: "tool".to_string(),
                status: "done".to_string(),
                content: "a".repeat(2_010),
                tool_call_id: Some("call_1".to_string()),
                tool_name: Some("read_file".to_string()),
                tool_calls_json: None,
                metadata_json: None,
                raw_result_json: Some(json!({ "path": "res://main.gd" }).to_string()),
                arguments_json: Some(json!({ "path": "res://main.gd" }).to_string()),
                target_keys_json: Some(json!(["res://main.gd"]).to_string()),
                tool_status: Some("done".to_string()),
            }]),
        ];

        let rendered = render_history_to_summarize(&groups);

        assert!(rendered.contains("[Attached image/png: screen.png]"));
        assert!(!rendered.contains(&"x".repeat(64)));
        assert!(rendered.contains("[Tool output truncated for compaction: omitted 10 chars]"));
        assert!(rendered.contains("Tool call id: call_1"));
    }

    #[test]
    fn summary_prompt_truncates_tool_call_and_argument_json() {
        let assistant_tail = "assistant-json-tail";
        let argument_tail = "argument-json-tail";
        let tool_call_arguments =
            json!({ "code": format!("{}{}", "a".repeat(2_400), assistant_tail) }).to_string();
        let arguments_json =
            json!({ "script": format!("{}{}", "b".repeat(2_400), argument_tail) }).to_string();
        let groups = vec![
            ReplayGroup::new(vec![ReplayRow {
                id: "msg_assistant".to_string(),
                sequence: 1,
                role: "assistant".to_string(),
                status: "done".to_string(),
                content: "running script".to_string(),
                tool_call_id: None,
                tool_name: None,
                tool_calls_json: Some(
                    json!([{
                        "id": "call_1",
                        "type": "function",
                        "function": {
                            "name": "run_scene_edit_script",
                            "arguments": tool_call_arguments
                        }
                    }])
                    .to_string(),
                ),
                metadata_json: None,
                raw_result_json: None,
                arguments_json: None,
                target_keys_json: None,
                tool_status: None,
            }]),
            ReplayGroup::new(vec![ReplayRow {
                id: "msg_tool".to_string(),
                sequence: 2,
                role: "tool".to_string(),
                status: "done".to_string(),
                content: "ok".to_string(),
                tool_call_id: Some("call_1".to_string()),
                tool_name: Some("run_scene_edit_script".to_string()),
                tool_calls_json: None,
                metadata_json: None,
                raw_result_json: Some(json!({ "status": "done" }).to_string()),
                arguments_json: Some(arguments_json),
                target_keys_json: Some(json!(["res://Main.tscn"]).to_string()),
                tool_status: Some("done".to_string()),
            }]),
        ];

        let rendered = render_history_to_summarize(&groups);

        assert!(rendered.contains("Tool calls:"));
        assert!(rendered.contains("Arguments:"));
        assert_eq!(
            rendered
                .matches("[JSON truncated for compaction: omitted ")
                .count(),
            2
        );
        assert!(!rendered.contains(assistant_tail));
        assert!(!rendered.contains(argument_tail));
    }
}
