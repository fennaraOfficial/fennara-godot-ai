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
    let raw = result.raw_result().unwrap_or(Value::Null);
    let metadata = result.metadata().unwrap_or(Value::Null);
    let arguments = result.arguments().unwrap_or(Value::Null);
    let mut details = vec![
        format!(
            "status={}",
            tool_facing_status(result.status(), &raw, &metadata)
        ),
        format!(
            "omitted~{}tok/{}ch",
            compact_count(estimated_tokens),
            compact_count(original_chars)
        ),
    ];

    match result.tool_name().unwrap_or_default() {
        "read_file" | "get_scene_tree" | "get_node_properties" | "get_class_info" => {}
        "write_or_update_file" => {
            push_field(&mut details, "mode", &["mode"], &raw, &metadata, &arguments);
            push_field(
                &mut details,
                "created",
                &["created"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "repl",
                &["replacements_made"],
                &raw,
                &metadata,
                &arguments,
            );
            push_diagnostic_counts(&mut details, &raw, &metadata, &arguments);
        }
        "script_diagnostics" => {
            push_field(
                &mut details,
                "scan_project",
                &["scan_project"],
                &raw,
                &metadata,
                &arguments,
            );
            push_diagnostic_counts(&mut details, &raw, &metadata, &arguments);
        }
        "validate_scene" => {
            push_issue_counts(&mut details, &raw, &metadata, &arguments);
            push_field(
                &mut details,
                "runtime_log",
                &["runtime_compacted_log_path", "raw_log_path", "log_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_artifact_fields(&mut details, &raw, &metadata, &arguments);
        }
        "screenshot_scene" => {
            push_field(
                &mut details,
                "img",
                &["image_path", "screenshot_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "dir",
                &["screenshot_dir"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "images",
                &["image_count"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        "runtime_session" => {
            push_field(
                &mut details,
                "action",
                &["action"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "session",
                &["session_id"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "scene",
                &["scene_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "log",
                &["log_path", "raw_log_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "issues",
                &["runtime_issue_count"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "debugger_errors",
                &["runtime_debugger_error_count"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "launch_errors",
                &["launch_error_count"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "msbuild_log",
                &["msbuild_log_path"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        "runtime_script" => {
            push_field(
                &mut details,
                "session",
                &["session_id"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "script",
                &["script_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "log",
                &["log_path", "raw_log_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_diagnostic_counts(&mut details, &raw, &metadata, &arguments);
            push_issue_counts(&mut details, &raw, &metadata, &arguments);
            push_array_count(
                &mut details,
                "captures",
                &["captures"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        "run_scene_edit_script" => {
            push_field(
                &mut details,
                "modified",
                &["modified"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "saved",
                &["scene_saved"],
                &raw,
                &metadata,
                &arguments,
            );
            push_diagnostic_counts(&mut details, &raw, &metadata, &arguments);
            push_issue_counts(&mut details, &raw, &metadata, &arguments);
            push_field(
                &mut details,
                "logs",
                &["log_count"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        "run_asset_import_script" => {
            push_field(
                &mut details,
                "modified",
                &["modified"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "reimported",
                &["reimported"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "changes",
                &["change_count"],
                &raw,
                &metadata,
                &arguments,
            );
            push_diagnostic_counts(&mut details, &raw, &metadata, &arguments);
        }
        "project_settings" => {
            push_field(
                &mut details,
                "action",
                &["action"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(&mut details, "key", &["key"], &raw, &metadata, &arguments);
            push_field(
                &mut details,
                "prefix",
                &["prefix"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "query",
                &["query"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "count",
                &["count", "total_count"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        "scrape_editor" => {
            push_field(
                &mut details,
                "target",
                &["target"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "source",
                &["source"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "tree",
                &["tree_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "errors",
                &["error_count"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        "exec_command" => {
            push_field(&mut details, "cwd", &["cwd"], &raw, &metadata, &arguments);
            push_field(
                &mut details,
                "exit",
                &["exit_code"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "timeout",
                &["timed_out"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "log",
                &["log_path", "raw_log_path"],
                &raw,
                &metadata,
                &arguments,
            );
            push_field(
                &mut details,
                "ms",
                &["duration_ms"],
                &raw,
                &metadata,
                &arguments,
            );
        }
        _ => push_generic_fields(&mut details, &raw, &metadata, &arguments),
    }

    push_field(
        &mut details,
        "error",
        &["error", "block_reason"],
        &raw,
        &metadata,
        &arguments,
    );
    dedupe_preserve_order(details)
}

fn tool_facing_status<'a>(fallback: &'a str, raw: &'a Value, metadata: &'a Value) -> &'a str {
    metadata
        .get("status")
        .or_else(|| raw.get("status"))
        .and_then(Value::as_str)
        .filter(|status| !status.trim().is_empty())
        .unwrap_or(fallback)
}

fn push_diagnostic_counts(
    details: &mut Vec<String>,
    raw: &Value,
    metadata: &Value,
    arguments: &Value,
) {
    push_field(
        details,
        "errors",
        &["total_errors", "error_count"],
        raw,
        metadata,
        arguments,
    );
    push_field(
        details,
        "warnings",
        &["total_warnings", "warning_count"],
        raw,
        metadata,
        arguments,
    );
    push_field(
        details,
        "diagnostics",
        &["diagnostic_count"],
        raw,
        metadata,
        arguments,
    );
    push_field(
        details,
        "omitted_diag",
        &["omitted_diagnostics"],
        raw,
        metadata,
        arguments,
    );
}

fn push_issue_counts(details: &mut Vec<String>, raw: &Value, metadata: &Value, arguments: &Value) {
    push_field(
        details,
        "runtime_errors",
        &["runtime_error_count", "runtime_debugger_error_count"],
        raw,
        metadata,
        arguments,
    );
    push_field(
        details,
        "runtime_warnings",
        &["runtime_warning_count"],
        raw,
        metadata,
        arguments,
    );
}

fn push_artifact_fields(
    details: &mut Vec<String>,
    raw: &Value,
    metadata: &Value,
    arguments: &Value,
) {
    push_field(
        details,
        "result",
        &["result_path", "result_json_path"],
        raw,
        metadata,
        arguments,
    );
    push_field(
        details,
        "artifact",
        &["artifact_path", "artifact_dir"],
        raw,
        metadata,
        arguments,
    );
}

fn push_generic_fields(
    details: &mut Vec<String>,
    raw: &Value,
    metadata: &Value,
    arguments: &Value,
) {
    for (label, keys) in [
        ("cmd", &["command"][..]),
        ("cwd", &["cwd"][..]),
        ("exit", &["exit_code"][..]),
        ("timeout", &["timed_out"][..]),
        ("ms", &["duration_ms"][..]),
        (
            "path",
            &["file_path", "path", "resource_path", "script_path"][..],
        ),
        ("log", &["log_path", "raw_log_path"][..]),
        ("artifact", &["artifact_path", "artifact_dir"][..]),
        ("result", &["result_path"][..]),
        ("out", &["output_path", "screenshot_path", "image_path"][..]),
    ] {
        push_field(details, label, keys, raw, metadata, arguments);
    }
}

fn push_array_count(
    details: &mut Vec<String>,
    label: &str,
    keys: &[&str],
    raw: &Value,
    metadata: &Value,
    arguments: &Value,
) {
    let Some(value) = first_value(keys, raw, metadata, arguments) else {
        return;
    };
    let Some(values) = value.as_array() else {
        return;
    };
    if !values.is_empty() {
        details.push(format!("{label}={}", values.len()));
    }
}

fn push_field(
    details: &mut Vec<String>,
    label: &str,
    keys: &[&str],
    raw: &Value,
    metadata: &Value,
    arguments: &Value,
) {
    let Some(value) = first_value(keys, raw, metadata, arguments) else {
        return;
    };
    details.push(format!("{label}={}", short_value(value)));
}

fn first_value<'a>(
    keys: &[&str],
    raw: &'a Value,
    metadata: &'a Value,
    arguments: &'a Value,
) -> Option<&'a Value> {
    keys.iter().find_map(|key| {
        raw.get(*key)
            .or_else(|| metadata.get(*key))
            .or_else(|| arguments.get(*key))
            .filter(|value| !is_empty_value(value))
    })
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
        text.replace(['\r', '\n'], " ")
    } else {
        serde_json::to_string(value).unwrap_or_default()
    };
    if raw.chars().count() > 96 {
        raw.chars().take(93).collect::<String>() + "..."
    } else {
        raw
    }
}

fn compact_count(value: usize) -> String {
    if value >= 1_000_000 {
        format!("{}m", (value + 500_000) / 1_000_000)
    } else if value >= 1_000 {
        format!("{}k", (value + 500) / 1_000)
    } else {
        value.to_string()
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
    fn pressure_placeholder_keeps_compact_tool_specific_details() {
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
        assert!(
            placeholder
                .details
                .contains(&"status=completed".to_string())
        );
        assert!(placeholder.details.contains(&"exit=0".to_string()));
        assert!(placeholder.details.contains(&"cwd=C:/repo".to_string()));
        assert!(
            placeholder.details.contains(
                &"log=user://.fennara/tool_logs/session/results/tool_1_exec_command/result.json"
                    .to_string()
            )
        );
        assert!(
            placeholder
                .details
                .iter()
                .all(|detail| !detail.contains("40000"))
        );
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
