use super::types::PlaceholderSpec;

pub(crate) struct PlaceholderRenderer;

const MAX_TARGETS: usize = 2;
const MAX_DETAILS: usize = 8;

impl PlaceholderRenderer {
    pub(crate) fn render(spec: &PlaceholderSpec) -> String {
        let mut parts = vec![format!(
            "old tool result omitted: {}",
            compact(&spec.tool_name)
        )];
        if !spec.targets.is_empty() {
            parts.push(format!(
                "target={}",
                join_compact(&spec.targets, MAX_TARGETS)
            ));
        }
        for detail in spec.details.iter().take(MAX_DETAILS) {
            parts.push(compact(detail));
        }
        if spec.details.len() > MAX_DETAILS {
            parts.push(format!("+{} facts", spec.details.len() - MAX_DETAILS));
        }
        parts.push("exact in Fennara history".to_string());
        format!("[{}]", parts.join("; "))
    }
}

fn join_compact(values: &[String], limit: usize) -> String {
    let mut selected = values
        .iter()
        .take(limit)
        .map(|value| compact(value))
        .collect::<Vec<_>>();
    if values.len() > limit {
        selected.push(format!("+{}", values.len() - limit));
    }
    selected.join(", ")
}

fn compact(value: &str) -> String {
    let value = value.trim();
    if value.chars().count() > 96 {
        value.chars().take(93).collect::<String>() + "..."
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::context_compaction::types::{
        PlaceholderReason, PlaceholderSpec,
    };

    #[test]
    fn renders_single_line_compact_placeholder() {
        let rendered = PlaceholderRenderer::render(&PlaceholderSpec {
            tool_name: "exec_command".to_string(),
            tool_call_id: Some("call_exec".to_string()),
            targets: vec!["command: cargo test".to_string()],
            details: vec![
                "status=done".to_string(),
                "omitted~12k tok/50k ch".to_string(),
                "exit=7".to_string(),
            ],
            reason: PlaceholderReason::OldToolResultUnderPressure {
                protected_estimated_tokens: 40_000,
                minimum_saved_estimated_tokens: 20_000,
            },
        });

        assert_eq!(
            rendered,
            "[old tool result omitted: exec_command; target=command: cargo test; status=done; omitted~12k tok/50k ch; exit=7; exact in Fennara history]"
        );
        assert!(!rendered.contains("call_exec"));
        assert!(!rendered.contains("40000"));
    }
}
