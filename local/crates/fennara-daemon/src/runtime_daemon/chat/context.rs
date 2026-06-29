use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const MAX_SESSION_ID_CHARS: usize = 2048;
const MAX_PATH_CHARS: usize = 2048;
const MAX_SNIPPET_CHARS: usize = 64000;
const MAX_CONTEXT_SNIPPETS: usize = 8;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ChatContextSnippet {
    pub(crate) session_id: String,
    pub(crate) path: String,
    pub(crate) start_line: u32,
    pub(crate) end_line: u32,
    pub(crate) text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ClientContextSnippet {
    pub(crate) path: String,
    pub(crate) start_line: u32,
    pub(crate) end_line: u32,
    pub(crate) text: String,
}

impl ChatContextSnippet {
    pub(crate) fn from_godot_message(
        value: &Value,
        fallback_session_id: Option<&str>,
    ) -> Option<Self> {
        let session_id = clean_string(
            value
                .get("session_id")
                .and_then(Value::as_str)
                .or(fallback_session_id),
            MAX_SESSION_ID_CHARS,
        )?;
        let path = clean_string(value.get("path").and_then(Value::as_str), MAX_PATH_CHARS)?;
        let start_line = clean_line(value.get("start_line").and_then(Value::as_u64))?;
        let end_line = clean_line(value.get("end_line").and_then(Value::as_u64))?;
        if end_line < start_line {
            return None;
        }
        let text = clean_text(value.get("text").and_then(Value::as_str))?;

        Some(Self {
            session_id,
            path,
            start_line,
            end_line,
            text,
        })
    }

    pub(crate) fn to_client_message(&self) -> Value {
        json!({
            "type": "chat_context_snippet",
            "session_id": self.session_id,
            "path": self.path,
            "start_line": self.start_line,
            "end_line": self.end_line,
            "text": self.text
        })
    }
}

pub(crate) fn validate_client_snippets(
    snippets: Option<Vec<ClientContextSnippet>>,
) -> Result<Vec<ClientContextSnippet>, String> {
    let Some(snippets) = snippets else {
        return Ok(Vec::new());
    };
    if snippets.len() > MAX_CONTEXT_SNIPPETS {
        return Err(format!(
            "Attach up to {MAX_CONTEXT_SNIPPETS} code snippets."
        ));
    }

    snippets
        .into_iter()
        .map(validate_client_snippet)
        .collect::<Result<Vec<_>, _>>()
}

pub(crate) fn metadata_value(snippets: &[ClientContextSnippet]) -> Option<Value> {
    if snippets.is_empty() {
        None
    } else {
        Some(json!({ "context_snippets": snippets }))
    }
}

pub(crate) fn snippets_from_metadata(metadata_json: Option<&str>) -> Vec<ClientContextSnippet> {
    let Some(metadata_json) = metadata_json else {
        return Vec::new();
    };
    let Ok(metadata) = serde_json::from_str::<Value>(metadata_json) else {
        return Vec::new();
    };
    let Some(snippets) = metadata.get("context_snippets").and_then(Value::as_array) else {
        return Vec::new();
    };
    snippets
        .iter()
        .filter_map(|snippet| serde_json::from_value::<ClientContextSnippet>(snippet.clone()).ok())
        .filter_map(|snippet| validate_client_snippet(snippet).ok())
        .collect()
}

pub(crate) fn message_with_context_snippets(
    user_message: &str,
    snippets: &[ClientContextSnippet],
) -> String {
    if snippets.is_empty() {
        return user_message.to_string();
    }
    let blocks = snippets
        .iter()
        .map(context_block_markdown)
        .collect::<Vec<_>>()
        .join("\n\n");
    [
        Some("Selected project context:".to_string()),
        Some(blocks),
        user_message
            .trim()
            .is_empty()
            .then_some("Use the selected context above to answer the user's request.".to_string()),
        (!user_message.trim().is_empty()).then_some(user_message.to_string()),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join("\n\n")
}

fn validate_client_snippet(snippet: ClientContextSnippet) -> Result<ClientContextSnippet, String> {
    let path = clean_string(Some(&snippet.path), MAX_PATH_CHARS)
        .filter(|path| path.starts_with("res://"))
        .ok_or_else(|| "Context snippet path must be a full res:// project path.".to_string())?;
    if snippet.start_line == 0 || snippet.end_line < snippet.start_line {
        return Err("Context snippet line range is invalid.".to_string());
    }
    let text = clean_text(Some(&snippet.text))
        .ok_or_else(|| "Context snippet text is empty.".to_string())?;
    Ok(ClientContextSnippet {
        path,
        start_line: snippet.start_line,
        end_line: snippet.end_line,
        text,
    })
}

fn context_block_markdown(snippet: &ClientContextSnippet) -> String {
    let fence = markdown_fence_for(&snippet.text);
    let language = language_for_path(&snippet.path);
    let range = if snippet.end_line > snippet.start_line {
        format!("{}-{}", snippet.start_line, snippet.end_line)
    } else {
        snippet.start_line.to_string()
    };
    format!(
        "{fence}{language}\n# {}:{}\n{}\n{fence}",
        snippet.path,
        range,
        snippet.text.trim_end()
    )
}

fn markdown_fence_for(text: &str) -> String {
    let mut fence = "```".to_string();
    while text.contains(&fence) {
        fence.push('`');
    }
    fence
}

fn language_for_path(path: &str) -> &'static str {
    let clean = path.to_ascii_lowercase();
    if clean.ends_with(".gd") {
        "gdscript"
    } else if clean.ends_with(".cs") {
        "csharp"
    } else if clean.ends_with(".gdshader") {
        "glsl"
    } else {
        "text"
    }
}

fn clean_string(value: Option<&str>, max_chars: usize) -> Option<String> {
    let clean = value?.trim();
    if clean.is_empty() || clean.chars().count() > max_chars {
        return None;
    }
    Some(clean.to_string())
}

fn clean_line(value: Option<u64>) -> Option<u32> {
    let line = value?;
    if line == 0 || line > u32::MAX as u64 {
        return None;
    }
    Some(line as u32)
}

fn clean_text(value: Option<&str>) -> Option<String> {
    let mut text = value?.replace("\r\n", "\n").replace('\r', "\n");
    if text.trim().is_empty() {
        return None;
    }
    if text.chars().count() > MAX_SNIPPET_CHARS {
        text = text.chars().take(MAX_SNIPPET_CHARS).collect::<String>();
        text.push_str("\n... [truncated by Fennara]\n");
    }
    Some(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_godot_snippet() {
        let snippet = ChatContextSnippet::from_godot_message(
            &json!({
                "path": "res://scripts/player.gd",
                "start_line": 12,
                "end_line": 14,
                "text": "func move():\n    pass"
            }),
            Some("session-1"),
        )
        .unwrap();

        assert_eq!(snippet.session_id, "session-1");
        assert_eq!(snippet.path, "res://scripts/player.gd");
        assert_eq!(snippet.start_line, 12);
        assert_eq!(snippet.end_line, 14);
    }

    #[test]
    fn rejects_missing_or_reversed_range() {
        assert!(
            ChatContextSnippet::from_godot_message(
                &json!({
                    "path": "res://x.gd",
                    "start_line": 4,
                    "end_line": 3,
                    "text": "x"
                }),
                Some("session-1"),
            )
            .is_none()
        );
        assert!(
            ChatContextSnippet::from_godot_message(
                &json!({
                    "path": "res://x.gd",
                    "start_line": 1,
                    "end_line": 1,
                    "text": "x"
                }),
                None,
            )
            .is_none()
        );
    }

    #[test]
    fn model_message_includes_hidden_context_blocks() {
        let snippets = validate_client_snippets(Some(vec![ClientContextSnippet {
            path: "res://scripts/player.gd".to_string(),
            start_line: 4,
            end_line: 6,
            text: "func move():\n    pass".to_string(),
        }]))
        .unwrap();

        let message = message_with_context_snippets("What is wrong here?", &snippets);

        assert!(message.contains("Selected project context"));
        assert!(message.contains("# res://scripts/player.gd:4-6"));
        assert!(message.contains("What is wrong here?"));
    }

    #[test]
    fn context_metadata_round_trips() {
        let snippets = validate_client_snippets(Some(vec![ClientContextSnippet {
            path: "res://scripts/player.gd".to_string(),
            start_line: 2,
            end_line: 2,
            text: "var speed = 10".to_string(),
        }]))
        .unwrap();

        let metadata = metadata_value(&snippets).unwrap();
        let restored = snippets_from_metadata(Some(&metadata.to_string()));

        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].path, "res://scripts/player.gd");
        assert_eq!(restored[0].start_line, 2);
    }
}
