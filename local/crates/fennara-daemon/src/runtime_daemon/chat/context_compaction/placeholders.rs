use super::types::{PlaceholderReason, PlaceholderSpec};

pub(crate) struct PlaceholderRenderer;

impl PlaceholderRenderer {
    pub(crate) fn render(spec: &PlaceholderSpec) -> String {
        let mut lines = vec![
            "[tool result omitted from model context]".to_string(),
            format!("Tool: {}", spec.tool_name),
        ];
        if let Some(tool_call_id) = spec.tool_call_id.as_deref() {
            lines.push(format!("Tool call id: {tool_call_id}"));
        }
        if !spec.targets.is_empty() {
            lines.push(format!("Target: {}", spec.targets.join(", ")));
        }
        if !spec.details.is_empty() {
            lines.push("Details:".to_string());
            lines.extend(spec.details.iter().map(|detail| format!("- {detail}")));
        }
        lines.push(format!("Reason: {}", reason_text(&spec.reason)));
        lines.push(storage_reminder(&spec.reason));
        lines.join("\n")
    }
}

fn reason_text(reason: &PlaceholderReason) -> String {
    match reason {
        PlaceholderReason::OldToolResultUnderPressure {
            protected_estimated_tokens,
            minimum_saved_estimated_tokens,
        } => format!(
            "old tool result omitted to reduce model context after preserving the newest {protected_estimated_tokens} estimated tokens of older tool output; pruning is applied only when it saves at least {minimum_saved_estimated_tokens} estimated tokens."
        ),
    }
}

fn storage_reminder(reason: &PlaceholderReason) -> String {
    match reason {
        PlaceholderReason::OldToolResultUnderPressure { .. } => {
            "The original stored tool result was not modified. Inspect the stored chat/tool result in Fennara history if the exact old output is needed.".to_string()
        }
    }
}
