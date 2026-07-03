use super::{
    tokens::estimate_text_tokens,
    types::{PlaceholderReason, PlaceholderSpec, ReplayPlan, ToolResultRef},
};
use serde_json::Value;

pub(crate) const DEFAULT_PRUNE_PROTECT_TOKENS: usize = 40_000;
pub(crate) const DEFAULT_PRUNE_MINIMUM_SAVED_TOKENS: usize = 20_000;

pub(crate) fn apply_pressure_fallback(
    plan: &mut ReplayPlan,
    protected: &[bool],
    protect_tokens: usize,
    minimum_saved_tokens: usize,
) {
    let mut kept_old_tool_tokens = 0usize;
    let mut saved_tokens = 0usize;
    let mut candidates = Vec::new();

    for group_index in (0..plan.groups.len()).rev() {
        if protected.get(group_index).copied().unwrap_or(false) {
            continue;
        }

        for row_index in (0..plan.groups[group_index].rows.len()).rev() {
            let planned = &plan.groups[group_index].rows[row_index];
            if !planned.is_exact() {
                continue;
            }
            let Some(result) = planned.row.tool_result() else {
                continue;
            };
            if is_incomplete_status(result.status()) {
                continue;
            }

            let estimated_tokens = estimate_text_tokens(result.content_markdown());
            if kept_old_tool_tokens.saturating_add(estimated_tokens) > protect_tokens {
                saved_tokens = saved_tokens.saturating_add(estimated_tokens);
                candidates.push((group_index, row_index, estimated_tokens));
            } else {
                kept_old_tool_tokens = kept_old_tool_tokens.saturating_add(estimated_tokens);
            }
        }
    }

    if saved_tokens < minimum_saved_tokens {
        return;
    }

    for (group_index, row_index, estimated_tokens) in candidates {
        let planned = &mut plan.groups[group_index].rows[row_index];
        let result = planned.row.tool_result().expect("checked as tool result");
        planned.replace_with_placeholder(PlaceholderSpec {
            tool_name: result.tool_name().unwrap_or("unknown").to_string(),
            tool_call_id: result.tool_call_id().map(ToOwned::to_owned),
            targets: target_labels(result),
            details: placeholder_details(result, estimated_tokens),
            reason: PlaceholderReason::OldToolResultUnderPressure {
                protected_estimated_tokens: protect_tokens,
                minimum_saved_estimated_tokens: minimum_saved_tokens,
            },
        });
    }
}

fn is_incomplete_status(status: &str) -> bool {
    matches!(status, "pending" | "in_progress" | "running")
}

fn target_labels(result: ToolResultRef<'_>) -> Vec<String> {
    let target_keys = result.target_keys();
    if !target_keys.is_empty() {
        return target_keys;
    }

    let mut labels = Vec::new();
    if let Some(metadata) = result.metadata() {
        append_metadata_targets(&mut labels, &metadata);
    }
    if labels.is_empty() {
        if let Some(arguments) = result.arguments() {
            append_argument_targets(&mut labels, &arguments);
        }
    }
    dedupe_preserve_order(labels)
}

fn append_metadata_targets(labels: &mut Vec<String>, metadata: &Value) {
    let Some(targets) = metadata.get("targets").and_then(Value::as_array) else {
        return;
    };

    for target in targets {
        if let Some(label) = target_label_from_value(target) {
            labels.push(label);
        }
    }
}

fn target_label_from_value(value: &Value) -> Option<String> {
    let object = value.as_object()?;
    for &key in target_keys() {
        if let Some(label) = object.get(key).and_then(Value::as_str) {
            let label = label.trim();
            if !label.is_empty() {
                return Some(format!("{}: {}", label_for_key(key), label));
            }
        }
    }
    None
}

fn append_argument_targets(labels: &mut Vec<String>, arguments: &Value) {
    for &key in target_keys() {
        append_argument_target(labels, arguments, key);
    }
}

fn append_argument_target(labels: &mut Vec<String>, arguments: &Value, key: &str) {
    let Some(value) = arguments.get(key) else {
        return;
    };
    match value {
        Value::String(text) => {
            let text = text.trim();
            if !text.is_empty() {
                labels.push(format!("{}: {}", label_for_key(key), text));
            }
        }
        Value::Array(values) => {
            let rendered = values
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .filter(|text| !text.is_empty())
                .take(6)
                .collect::<Vec<_>>();
            if !rendered.is_empty() {
                labels.push(format!("{}: {}", label_for_key(key), rendered.join(", ")));
            }
        }
        _ => {}
    }
}

fn placeholder_details(result: ToolResultRef<'_>, estimated_tokens: usize) -> Vec<String> {
    let original_chars = result.content_markdown().chars().count();
    let original_bytes = result.content_markdown().len();
    let mut details = vec![
        format!("Status: {}", result.status()),
        format!("Omitted estimated tokens: {estimated_tokens}"),
        format!("Omitted chars: {original_chars}"),
        format!("Omitted bytes: {original_bytes}"),
    ];

    let raw = result.raw_result().unwrap_or(Value::Null);
    let metadata = result.metadata().unwrap_or(Value::Null);
    let arguments = result.arguments().unwrap_or(Value::Null);
    for &key in generic_identity_detail_keys() {
        push_detail_field(&mut details, key, &raw, &metadata, &arguments);
    }

    dedupe_preserve_order(details)
}

fn generic_identity_detail_keys() -> &'static [&'static str] {
    &[
        "command",
        "cwd",
        "exit_code",
        "timed_out",
        "duration_ms",
        "result_path",
        "artifact_path",
        "artifact_dir",
        "raw_log_path",
        "log_path",
        "error",
        "block_reason",
        "output_path",
        "screenshot_path",
        "image_path",
        "file_path",
        "path",
    ]
}

fn push_detail_field(
    details: &mut Vec<String>,
    key: &str,
    raw: &Value,
    metadata: &Value,
    arguments: &Value,
) {
    let value = raw
        .get(key)
        .or_else(|| metadata.get(key))
        .or_else(|| arguments.get(key));
    let Some(value) = value else {
        return;
    };
    if is_empty_value(value) {
        return;
    }
    details.push(format!("{}: {}", label_for_key(key), short_value(value)));
}

fn target_keys() -> &'static [&'static str] {
    &[
        "path",
        "file_path",
        "file",
        "scene_path",
        "scene_paths",
        "node_path",
        "node_paths",
        "script_path",
        "project_path",
        "resource_path",
        "session_id",
        "class_name",
        "command",
        "cwd",
    ]
}

fn label_for_key(key: &str) -> String {
    key.replace('_', " ")
}

fn is_empty_value(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(text) => text.trim().is_empty(),
        Value::Array(values) => values.is_empty(),
        Value::Object(object) => object.is_empty(),
        _ => false,
    }
}

fn short_value(value: &Value) -> String {
    let raw = if let Some(text) = value.as_str() {
        text.to_string()
    } else {
        serde_json::to_string(value).unwrap_or_default()
    };
    if raw.chars().count() > 180 {
        raw.chars().take(177).collect::<String>() + "..."
    } else {
        raw
    }
}

fn dedupe_preserve_order(values: Vec<String>) -> Vec<String> {
    let mut deduped = Vec::new();
    for value in values {
        if !deduped.contains(&value) {
            deduped.push(value);
        }
    }
    deduped
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::context_compaction::{
        tail, types::ReplayGroup, types::ReplayPlan, types::ReplayRow,
    };
    use serde_json::json;

    fn tool_group(call_id: &str, content: &str) -> ReplayGroup {
        ReplayGroup::new(vec![ReplayRow {
            id: format!("msg_{call_id}"),
            sequence: 1,
            role: "tool".to_string(),
            status: "done".to_string(),
            content: content.to_string(),
            tool_call_id: Some(call_id.to_string()),
            tool_name: Some("read_file".to_string()),
            tool_calls_json: None,
            metadata_json: None,
            raw_result_json: None,
            arguments_json: Some(json!({ "path": "res://old.gd" }).to_string()),
            target_keys_json: None,
            tool_status: Some("done".to_string()),
        }])
    }

    fn tool_group_with_status(call_id: &str, status: &str) -> ReplayGroup {
        ReplayGroup::new(vec![ReplayRow {
            id: format!("msg_{call_id}"),
            sequence: 1,
            role: "tool".to_string(),
            status: status.to_string(),
            content: "x".repeat(80),
            tool_call_id: Some(call_id.to_string()),
            tool_name: Some("exec_command".to_string()),
            tool_calls_json: None,
            metadata_json: None,
            raw_result_json: Some(json!({ "status": status, "command": "cargo test" }).to_string()),
            arguments_json: Some(json!({ "command": "cargo test" }).to_string()),
            target_keys_json: None,
            tool_status: Some(status.to_string()),
        }])
    }

    #[test]
    fn compacts_oldest_tool_results_after_protected_token_window() {
        let groups = vec![
            tool_group("old", &"a".repeat(80)),
            tool_group("new", &"b".repeat(80)),
        ];
        let protected = tail::protected_groups(&groups, 0);
        let mut plan = ReplayPlan::from_groups(groups);

        apply_pressure_fallback(&mut plan, &protected, 20, 10);

        assert!(plan.groups[0].rows[0].placeholder.is_some());
        assert!(plan.groups[1].rows[0].placeholder.is_none());
    }

    #[test]
    fn does_not_compact_when_saved_tokens_are_below_minimum() {
        let groups = vec![
            tool_group("old", &"a".repeat(20)),
            tool_group("new", &"b".repeat(80)),
        ];
        let protected = tail::protected_groups(&groups, 0);
        let mut plan = ReplayPlan::from_groups(groups);

        apply_pressure_fallback(&mut plan, &protected, 20, 10);

        assert!(plan.groups[0].rows[0].placeholder.is_none());
        assert!(plan.groups[1].rows[0].placeholder.is_none());
    }

    #[test]
    fn pressure_placeholder_keeps_normal_identity_details() {
        let groups = vec![ReplayGroup::new(vec![ReplayRow {
            id: "msg_exec".to_string(),
            sequence: 1,
            role: "tool".to_string(),
            status: "done".to_string(),
            content: "x".repeat(80),
            tool_call_id: Some("call_exec".to_string()),
            tool_name: Some("exec_command".to_string()),
            tool_calls_json: None,
            metadata_json: Some(json!({ "targets": [{ "command": "cargo test", "cwd": "C:/repo" }] }).to_string()),
            raw_result_json: Some(
                json!({
                    "status": "completed",
                    "command": "cargo test",
                    "cwd": "C:/repo",
                    "exit_code": 0,
                    "raw_log_path": "user://.fennara/tool_logs/session/results/tool_1_exec_command/result.json"
                })
                .to_string(),
            ),
            arguments_json: Some(json!({ "command": "cargo test" }).to_string()),
            target_keys_json: None,
            tool_status: Some("done".to_string()),
        }])];
        let protected = tail::protected_groups(&groups, 0);
        let mut plan = ReplayPlan::from_groups(groups);

        apply_pressure_fallback(&mut plan, &protected, 0, 1);

        let placeholder = plan.groups[0].rows[0].placeholder.as_ref().unwrap();
        assert_eq!(
            placeholder.reason,
            PlaceholderReason::OldToolResultUnderPressure {
                protected_estimated_tokens: 0,
                minimum_saved_estimated_tokens: 1,
            }
        );
        assert!(
            placeholder
                .targets
                .contains(&"command: cargo test".to_string())
        );
        assert!(placeholder.details.contains(&"Status: done".to_string()));
        assert!(placeholder.details.contains(&"exit code: 0".to_string()));
        assert!(placeholder.details.contains(
            &"raw log path: user://.fennara/tool_logs/session/results/tool_1_exec_command/result.json"
                .to_string()
        ));
    }

    #[test]
    fn pending_tool_results_are_not_compacted() {
        let groups = vec![ReplayGroup::new(vec![ReplayRow {
            id: "msg_running".to_string(),
            sequence: 1,
            role: "tool".to_string(),
            status: "running".to_string(),
            content: "x".repeat(80),
            tool_call_id: Some("call_running".to_string()),
            tool_name: Some("read_file".to_string()),
            tool_calls_json: None,
            metadata_json: None,
            raw_result_json: None,
            arguments_json: Some(json!({ "path": "res://current.gd" }).to_string()),
            target_keys_json: None,
            tool_status: Some("running".to_string()),
        }])];
        let protected = tail::protected_groups(&groups, 0);
        let mut plan = ReplayPlan::from_groups(groups);

        apply_pressure_fallback(&mut plan, &protected, 0, 1);

        assert!(plan.groups[0].rows[0].placeholder.is_none());
    }

    #[test]
    fn terminal_tool_statuses_are_pressure_candidates() {
        for status in ["done", "failed", "timed_out", "cancelled", "denied"] {
            let groups = vec![tool_group_with_status(status, status)];
            let protected = tail::protected_groups(&groups, 0);
            let mut plan = ReplayPlan::from_groups(groups);

            apply_pressure_fallback(&mut plan, &protected, 0, 1);

            assert!(
                plan.groups[0].rows[0].placeholder.is_some(),
                "{status} should be eligible"
            );
        }
    }
}
