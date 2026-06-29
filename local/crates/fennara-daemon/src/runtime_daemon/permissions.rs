use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::sync::oneshot;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ApprovalMode {
    #[default]
    Ask,
    FullAccess,
}

impl ApprovalMode {
    pub(crate) const ASK_VALUE: &'static str = "ask";
    pub(crate) const FULL_ACCESS_VALUE: &'static str = "full_access";

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Ask => Self::ASK_VALUE,
            Self::FullAccess => Self::FULL_ACCESS_VALUE,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Ask => "Ask for approval",
            Self::FullAccess => "Full access",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ToolPermissionKind {
    ReadOnly,
    MutatesProject,
    ExecutesProject,
    Denied,
}

impl ToolPermissionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::MutatesProject => "mutates_project",
            Self::ExecutesProject => "executes_project",
            Self::Denied => "denied",
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::ReadOnly => "Read-only",
            Self::MutatesProject => "Changes project files or settings",
            Self::ExecutesProject => "Runs project code or scenes",
            Self::Denied => "Denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PermissionDecision {
    Allow,
    AskUser { reason: String },
    Deny { reason: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ToolPermission {
    pub(crate) kind: ToolPermissionKind,
    pub(crate) reason: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ToolApprovalReview {
    Approved,
    Denied,
    TimedOut,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub(crate) enum ToolApprovalStatus {
    PendingApproval,
    Approved,
    Executing,
    Completed,
    Denied,
    Cancelled,
}

impl ToolApprovalStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::PendingApproval => "pending_approval",
            Self::Approved => "approved",
            Self::Executing => "executing",
            Self::Completed => "completed",
            Self::Denied => "denied",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ToolApprovalRequest {
    pub(crate) id: String,
    pub(crate) chat_id: String,
    pub(crate) session_id: String,
    pub(crate) tool_call_id: String,
    pub(crate) tool_name: String,
    pub(crate) tool_kind: ToolPermissionKind,
    pub(crate) tool_kind_label: &'static str,
    pub(crate) approval_mode: ApprovalMode,
    pub(crate) status: ToolApprovalStatus,
    pub(crate) reason: String,
    pub(crate) summary: String,
}

pub(crate) struct PendingToolApproval {
    pub(crate) request: ToolApprovalRequest,
    pub(crate) responder: oneshot::Sender<ToolApprovalReview>,
}

#[derive(Clone, Debug)]
pub(crate) struct PermissionPolicy {
    mode: ApprovalMode,
}

impl PermissionPolicy {
    pub(crate) fn new(mode: ApprovalMode) -> Self {
        Self { mode }
    }

    pub(crate) fn evaluate_tool(&self, tool_name: &str, arguments: &Value) -> ToolPermission {
        classify_tool(tool_name, arguments)
    }

    pub(crate) fn decide_tool(&self, tool_name: &str, arguments: &Value) -> PermissionDecision {
        let permission = self.evaluate_tool(tool_name, arguments);
        match permission.kind {
            ToolPermissionKind::ReadOnly => PermissionDecision::Allow,
            ToolPermissionKind::Denied => PermissionDecision::Deny {
                reason: permission.reason,
            },
            ToolPermissionKind::MutatesProject | ToolPermissionKind::ExecutesProject => {
                match self.mode {
                    ApprovalMode::Ask => PermissionDecision::AskUser {
                        reason: permission.reason,
                    },
                    ApprovalMode::FullAccess => PermissionDecision::Allow,
                }
            }
        }
    }
}

pub(crate) fn clean_approval_mode(value: &str) -> ApprovalMode {
    match value.trim().to_ascii_lowercase().as_str() {
        "full-access" | "full_access" | "full access" => ApprovalMode::FullAccess,
        _ => ApprovalMode::Ask,
    }
}

pub(crate) fn approval_mode_options() -> Vec<Value> {
    vec![
        approval_mode_option(ApprovalMode::Ask),
        approval_mode_option(ApprovalMode::FullAccess),
    ]
}

pub(crate) fn approval_request_payload(request: &ToolApprovalRequest) -> Value {
    json!({
        "id": request.id,
        "chat_id": request.chat_id,
        "session_id": request.session_id,
        "tool_call_id": request.tool_call_id,
        "tool_name": request.tool_name,
        "tool_kind": request.tool_kind.as_str(),
        "tool_kind_label": request.tool_kind_label,
        "approval_mode": request.approval_mode.as_str(),
        "approval_mode_label": request.approval_mode.label(),
        "status": request.status.as_str(),
        "reason": request.reason,
        "summary": request.summary,
    })
}

fn classify_tool(tool_name: &str, arguments: &Value) -> ToolPermission {
    match tool_name {
        "read_file"
        | "get_scene_tree"
        | "get_node_properties"
        | "get_class_info"
        | "script_diagnostics"
        | "screenshot_scene"
        | "scrape_editor" => read_only("This tool only inspects project/editor state."),
        "validate_scene" => executes(
            "This tool can run scenes headlessly for runtime validation, which executes project code.",
        ),
        "write_or_update_file" => mutates("This tool creates or updates project files."),
        "run_scene_edit_script" => {
            mutates("This tool can run editor-side Godot code and save scene/resource changes.")
        }
        "save_custom_resource" => mutates("This tool creates or updates project resources."),
        "runtime_script" => {
            executes("This tool runs code inside an active project runtime session.")
        }
        "exec_command" => executes(
            "This tool runs a host shell command from the daemon with project-root cwd restrictions.",
        ),
        "project_settings" => classify_project_settings(arguments),
        "runtime_session" => classify_runtime_session(arguments),
        _ => denied(format!("Unsupported plugin chat tool: {tool_name}")),
    }
}

fn classify_project_settings(arguments: &Value) -> ToolPermission {
    match action(arguments).as_deref() {
        Some("get" | "list" | "find_setting") => {
            read_only("This project_settings action only reads or discovers settings.")
        }
        Some("set" | "remove") => mutates("This project_settings action changes project.godot."),
        Some(other) => denied(format!("Unsupported project_settings action: {other}")),
        None => denied("project_settings requires an action.".to_string()),
    }
}

fn classify_runtime_session(arguments: &Value) -> ToolPermission {
    match action(arguments).as_deref() {
        Some("status") => read_only("runtime_session.status only inspects the managed session."),
        Some("start" | "stop") => {
            executes("This runtime_session action starts or stops project execution.")
        }
        Some(other) => denied(format!("Unsupported runtime_session action: {other}")),
        None => denied("runtime_session requires an action.".to_string()),
    }
}

fn action(arguments: &Value) -> Option<String> {
    arguments
        .get("action")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_ascii_lowercase())
}

fn read_only(reason: &str) -> ToolPermission {
    ToolPermission {
        kind: ToolPermissionKind::ReadOnly,
        reason: reason.to_string(),
    }
}

fn mutates(reason: &str) -> ToolPermission {
    ToolPermission {
        kind: ToolPermissionKind::MutatesProject,
        reason: reason.to_string(),
    }
}

fn executes(reason: &str) -> ToolPermission {
    ToolPermission {
        kind: ToolPermissionKind::ExecutesProject,
        reason: reason.to_string(),
    }
}

fn denied(reason: String) -> ToolPermission {
    ToolPermission {
        kind: ToolPermissionKind::Denied,
        reason,
    }
}

fn approval_mode_option(mode: ApprovalMode) -> Value {
    json!({
        "value": mode.as_str(),
        "label": mode.label(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn ask_mode_allows_read_only_tools() {
        let policy = PermissionPolicy::new(ApprovalMode::Ask);

        assert_eq!(
            policy.decide_tool("read_file", &json!({})),
            PermissionDecision::Allow
        );
        assert_eq!(
            policy.decide_tool("project_settings", &json!({ "action": "find_setting" })),
            PermissionDecision::Allow
        );
        assert_eq!(
            policy.decide_tool("runtime_session", &json!({ "action": "status" })),
            PermissionDecision::Allow
        );
    }

    #[test]
    fn ask_mode_gates_mutating_and_execution_tools() {
        let policy = PermissionPolicy::new(ApprovalMode::Ask);

        assert!(matches!(
            policy.decide_tool("write_or_update_file", &json!({})),
            PermissionDecision::AskUser { .. }
        ));
        assert!(matches!(
            policy.decide_tool("project_settings", &json!({ "action": "set" })),
            PermissionDecision::AskUser { .. }
        ));
        assert!(matches!(
            policy.decide_tool("runtime_script", &json!({})),
            PermissionDecision::AskUser { .. }
        ));
        assert!(matches!(
            policy.decide_tool(
                "validate_scene",
                &json!({ "scene_paths": ["res://main.tscn"] })
            ),
            PermissionDecision::AskUser { .. }
        ));
        assert!(matches!(
            policy.decide_tool("runtime_session", &json!({ "action": "start" })),
            PermissionDecision::AskUser { .. }
        ));
        assert!(matches!(
            policy.decide_tool("exec_command", &json!({ "command": "echo ok" })),
            PermissionDecision::AskUser { .. }
        ));
    }

    #[test]
    fn full_access_allows_mutating_tools_but_not_invalid_shapes() {
        let policy = PermissionPolicy::new(ApprovalMode::FullAccess);

        assert_eq!(
            policy.decide_tool("write_or_update_file", &json!({})),
            PermissionDecision::Allow
        );
        assert_eq!(
            policy.decide_tool("exec_command", &json!({ "command": "echo ok" })),
            PermissionDecision::Allow
        );
        assert!(matches!(
            policy.decide_tool("project_settings", &json!({ "action": "unknown" })),
            PermissionDecision::Deny { .. }
        ));
    }

    #[test]
    fn cleans_only_v1_approval_modes() {
        assert_eq!(clean_approval_mode("full access"), ApprovalMode::FullAccess);
        assert_eq!(clean_approval_mode("approve_for_me"), ApprovalMode::Ask);
    }
}
