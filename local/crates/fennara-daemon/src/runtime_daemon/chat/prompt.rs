use serde_json::{Value, json};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::runtime_daemon::{
    permissions::{ApprovalMode, PermissionDecision, PermissionPolicy, ToolPermissionKind},
    state::GodotProjectStatus,
};

use super::{exec_command, store, tools};

const BASE_SYSTEM_PROMPT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../prompts/plugin_chat_system.md"
));

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PromptRuntimeContext {
    pub(crate) current_date: String,
    pub(crate) timezone: Option<String>,
    pub(crate) os: String,
    pub(crate) arch: String,
    pub(crate) daemon_cwd: Option<String>,
    pub(crate) project_name: Option<String>,
    pub(crate) project_path: Option<String>,
    pub(crate) godot_executable_path: Option<String>,
    pub(crate) godot_version: Option<String>,
    pub(crate) plugin_version: Option<String>,
    pub(crate) rendering_context: Option<Value>,
    pub(crate) approval_mode: ApprovalMode,
}

impl PromptRuntimeContext {
    pub(crate) fn from_turn(
        approval_mode: ApprovalMode,
        scope: &store::ProjectScope,
        active_project: Option<&GodotProjectStatus>,
    ) -> Self {
        Self {
            current_date: current_utc_date(),
            timezone: known_timezone(),
            os: env::consts::OS.to_string(),
            arch: env::consts::ARCH.to_string(),
            daemon_cwd: env::current_dir()
                .ok()
                .map(|path| path.to_string_lossy().into_owned()),
            project_name: first_nonempty([
                active_project.and_then(|project| clean_owned(project.project_name.as_deref())),
                clean_owned(scope.project_name.as_deref()),
            ]),
            project_path: first_nonempty([
                active_project.and_then(|project| clean_owned(project.project_path.as_deref())),
                clean_owned(scope.project_path.as_deref()),
            ]),
            godot_executable_path: active_project
                .and_then(|project| clean_owned(project.godot_executable_path.as_deref())),
            godot_version: active_project
                .and_then(|project| clean_owned(project.godot_version.as_deref())),
            plugin_version: active_project
                .and_then(|project| clean_owned(project.plugin_version.as_deref())),
            rendering_context: active_project.and_then(|project| project.rendering_context.clone()),
            approval_mode,
        }
    }

    fn render_environment_context(&self) -> String {
        let mut rendered = String::from("<environment_context>\n");
        push_text_element_with_attrs(
            &mut rendered,
            "  ",
            "current_date",
            &[("source", "system_utc")],
            &self.current_date,
        );
        match self.timezone.as_deref() {
            Some(timezone) => push_text_element_with_attrs(
                &mut rendered,
                "  ",
                "timezone",
                &[("known", "true")],
                timezone,
            ),
            None => push_text_element_with_attrs(
                &mut rendered,
                "  ",
                "timezone",
                &[("known", "false")],
                "unknown",
            ),
        }
        push_empty_element(
            &mut rendered,
            "  ",
            "platform",
            &[("os", &self.os), ("arch", &self.arch)],
        );
        rendered.push_str("  <workspace>\n");
        push_optional_text_element(
            &mut rendered,
            "    ",
            "project_name",
            self.project_name.as_deref(),
        );
        match self.project_path.as_deref() {
            Some(path) => push_text_element(&mut rendered, "    ", "project_root", path),
            None => {
                push_empty_element(&mut rendered, "    ", "project_root", &[("known", "false")])
            }
        }
        push_optional_text_element(
            &mut rendered,
            "    ",
            "daemon_cwd",
            self.daemon_cwd.as_deref(),
        );
        push_optional_text_element(
            &mut rendered,
            "    ",
            "godot_executable_path",
            self.godot_executable_path.as_deref(),
        );
        push_optional_text_element(
            &mut rendered,
            "    ",
            "godot_version",
            self.godot_version.as_deref(),
        );
        push_optional_text_element(
            &mut rendered,
            "    ",
            "fennara_plugin_version",
            self.plugin_version.as_deref(),
        );
        if let Some(rendering_context) = self.rendering_context.as_ref() {
            if let Ok(rendered_context) = serde_json::to_string(rendering_context) {
                push_text_element_with_attrs(
                    &mut rendered,
                    "    ",
                    "rendering_context",
                    &[("format", "json")],
                    &rendered_context,
                );
            }
        }
        push_text_element(
            &mut rendered,
            "    ",
            "filesystem_boundary",
            "Godot tools operate on the active project and allowed user:// artifacts; tool-level blocked paths and internal addon-file protections still apply.",
        );
        rendered.push_str("  </workspace>\n");
        render_capabilities(&mut rendered, self.project_path.as_deref());
        render_permissions(&mut rendered, self.approval_mode);
        rendered.push_str("</environment_context>");
        rendered
    }
}

pub(crate) struct PromptBuilder<'a> {
    base_prompt: &'a str,
    runtime_context: &'a PromptRuntimeContext,
}

impl<'a> PromptBuilder<'a> {
    pub(crate) fn new(runtime_context: &'a PromptRuntimeContext) -> Self {
        Self {
            base_prompt: BASE_SYSTEM_PROMPT,
            runtime_context,
        }
    }

    #[cfg(test)]
    fn with_base_prompt(base_prompt: &'a str, runtime_context: &'a PromptRuntimeContext) -> Self {
        Self {
            base_prompt,
            runtime_context,
        }
    }

    pub(crate) fn build(&self) -> String {
        format!(
            "{}\n\n{}",
            self.base_prompt.trim_end(),
            self.runtime_context.render_environment_context()
        )
    }
}

fn render_capabilities(rendered: &mut String, project_root: Option<&str>) {
    rendered.push_str("  <capabilities>\n");
    push_text_element(
        rendered,
        "    ",
        "available_fennara_tools",
        &tools::allowed_tool_names().join(", "),
    );
    let shell = exec_command::default_shell();
    let shell_path = shell.path.to_string_lossy().into_owned();
    let default_cwd = project_root.unwrap_or("unknown");
    push_empty_element(
        rendered,
        "    ",
        "shell",
        &[
            ("enabled", "true"),
            ("tool", "exec_command"),
            ("kind", shell.name()),
            ("path", &shell_path),
            ("default_cwd", default_cwd),
            ("cwd_policy", "project_cwd_restricted"),
        ],
    );
    push_empty_element(
        rendered,
        "    ",
        "web_search",
        &[
            ("enabled", "false"),
            (
                "reason",
                "web search is not implemented in this Fennara chat surface",
            ),
        ],
    );
    push_text_element_with_attrs(
        rendered,
        "    ",
        "network",
        &[("state", "unknown"), ("general_internet_tool", "false")],
        "No web-search tool is exposed. exec_command may run ordinary local shell commands, but Fennara does not provide OS sandboxing or network gating for shell processes.",
    );
    push_text_element(
        rendered,
        "    ",
        "path_rules",
        "Use res:// for Godot tools. Use real filesystem paths for exec_command. Omitted exec_command cwd defaults to the active project root; relative cwd values resolve under that root; outside-project cwd values are rejected in phase one.",
    );
    rendered.push_str("  </capabilities>\n");
}

fn render_permissions(rendered: &mut String, approval_mode: ApprovalMode) {
    let summary = PermissionPromptSummary::from_policy(approval_mode);
    rendered.push_str("  <permissions>\n");
    push_empty_element(
        rendered,
        "    ",
        "approval",
        &[
            ("mode", approval_mode.as_str()),
            ("label", approval_mode.label()),
        ],
    );
    push_text_element_with_attrs(
        rendered,
        "    ",
        "read_only_tools",
        &[("decision", "allow")],
        &summary.read_only_allowed.join(", "),
    );
    let mutating_decision = if summary.ask_user.is_empty() {
        "allow"
    } else {
        "ask_user"
    };
    let mutating_tools = if summary.ask_user.is_empty() {
        &summary.auto_allowed_mutating_or_execution
    } else {
        &summary.ask_user
    };
    push_text_element_with_attrs(
        rendered,
        "    ",
        "mutating_or_execution_tools",
        &[("decision", mutating_decision)],
        &mutating_tools.join(", "),
    );
    push_text_element(
        rendered,
        "    ",
        "hard_denies",
        "Unsupported tools/actions and tool-level blocked paths/internal addon-file protections are denied even in full_access.",
    );
    rendered.push_str("  </permissions>\n");
}

#[derive(Debug, Default)]
struct PermissionPromptSummary {
    read_only_allowed: Vec<String>,
    ask_user: Vec<String>,
    auto_allowed_mutating_or_execution: Vec<String>,
}

impl PermissionPromptSummary {
    fn from_policy(approval_mode: ApprovalMode) -> Self {
        let policy = PermissionPolicy::new(approval_mode);
        let mut summary = Self::default();
        for case in prompt_tool_cases() {
            let permission = policy.evaluate_tool(case.tool_name, &case.arguments);
            match policy.decide_tool(case.tool_name, &case.arguments) {
                PermissionDecision::Allow => {
                    if permission.kind == ToolPermissionKind::ReadOnly {
                        summary.read_only_allowed.push(case.display.to_string());
                    } else {
                        summary
                            .auto_allowed_mutating_or_execution
                            .push(case.display.to_string());
                    }
                }
                PermissionDecision::AskUser { .. } => {
                    summary.ask_user.push(case.display.to_string());
                }
                PermissionDecision::Deny { .. } => {}
            }
        }
        summary
    }
}

struct PromptToolCase {
    display: &'static str,
    tool_name: &'static str,
    arguments: Value,
}

fn prompt_tool_cases() -> Vec<PromptToolCase> {
    let mut cases = vec![
        simple_case("read_file"),
        simple_case("get_scene_tree"),
        simple_case("get_node_properties"),
        simple_case("get_class_info"),
        simple_case("script_diagnostics"),
        simple_case("validate_scene"),
        simple_case("screenshot_scene"),
        simple_case("scrape_editor"),
        simple_case("write_or_update_file"),
        simple_case("run_scene_edit_script"),
        simple_case("runtime_script"),
        simple_case("exec_command"),
    ];
    cases.extend([
        action_case("project_settings.get", "project_settings", "get"),
        action_case("project_settings.list", "project_settings", "list"),
        action_case(
            "project_settings.find_setting",
            "project_settings",
            "find_setting",
        ),
        action_case("project_settings.set", "project_settings", "set"),
        action_case("project_settings.remove", "project_settings", "remove"),
        action_case("runtime_session.status", "runtime_session", "status"),
        action_case("runtime_session.start", "runtime_session", "start"),
        action_case("runtime_session.stop", "runtime_session", "stop"),
    ]);
    cases
}

fn simple_case(name: &'static str) -> PromptToolCase {
    PromptToolCase {
        display: name,
        tool_name: name,
        arguments: json!({}),
    }
}

fn action_case(
    display: &'static str,
    tool_name: &'static str,
    action: &'static str,
) -> PromptToolCase {
    PromptToolCase {
        display,
        tool_name,
        arguments: json!({ "action": action }),
    }
}

fn current_utc_date() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    date_from_unix_days((seconds / 86_400) as i64)
}

fn date_from_unix_days(days: i64) -> String {
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = year + if month <= 2 { 1 } else { 0 };
    (year as i32, month as u32, day as u32)
}

fn known_timezone() -> Option<String> {
    env::var("TZ").ok().and_then(|value| {
        let value = value.trim();
        (!value.is_empty()).then(|| value.to_string())
    })
}

fn first_nonempty(values: impl IntoIterator<Item = Option<String>>) -> Option<String> {
    values.into_iter().flatten().find(|value| !value.is_empty())
}

fn clean_owned(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn push_optional_text_element(
    rendered: &mut String,
    indent: &str,
    name: &str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        push_text_element(rendered, indent, name, value);
    }
}

fn push_text_element(rendered: &mut String, indent: &str, name: &str, value: &str) {
    push_text_element_with_attrs(rendered, indent, name, &[], value);
}

fn push_text_element_with_attrs(
    rendered: &mut String,
    indent: &str,
    name: &str,
    attrs: &[(&str, &str)],
    value: &str,
) {
    rendered.push_str(indent);
    rendered.push('<');
    rendered.push_str(name);
    push_attrs(rendered, attrs);
    rendered.push('>');
    push_xml_escaped_text(rendered, value);
    rendered.push_str("</");
    rendered.push_str(name);
    rendered.push_str(">\n");
}

fn push_empty_element(rendered: &mut String, indent: &str, name: &str, attrs: &[(&str, &str)]) {
    rendered.push_str(indent);
    rendered.push('<');
    rendered.push_str(name);
    push_attrs(rendered, attrs);
    rendered.push_str(" />\n");
}

fn push_attrs(rendered: &mut String, attrs: &[(&str, &str)]) {
    for (name, value) in attrs {
        rendered.push(' ');
        rendered.push_str(name);
        rendered.push_str("=\"");
        push_xml_escaped_text(rendered, value);
        rendered.push('"');
    }
}

fn push_xml_escaped_text(rendered: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '&' => rendered.push_str("&amp;"),
            '<' => rendered.push_str("&lt;"),
            '>' => rendered.push_str("&gt;"),
            '"' => rendered.push_str("&quot;"),
            '\'' => rendered.push_str("&apos;"),
            _ => rendered.push(ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_context(approval_mode: ApprovalMode) -> PromptRuntimeContext {
        PromptRuntimeContext {
            current_date: "2026-06-28".to_string(),
            timezone: Some("Asia/Calcutta".to_string()),
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            daemon_cwd: Some("C:\\Fennara".to_string()),
            project_name: Some("Prompt & Policy".to_string()),
            project_path: Some("C:\\Projects\\Prompt <Policy>".to_string()),
            godot_executable_path: Some("C:\\Godot\\Godot_v4.4-stable_win64.exe".to_string()),
            godot_version: Some("4.4.stable".to_string()),
            plugin_version: Some("0.3.1".to_string()),
            rendering_context: Some(json!({
                "schema_version": "rendering-context-v1",
                "runtime_rendering_method": "gl_compatibility",
                "runtime_rendering_driver_name": "opengl3",
                "has_rendering_device": false,
                "warnings": ["Compatibility/OpenGL renderer is active or configured."]
            })),
            approval_mode,
        }
    }

    #[test]
    fn prompt_builder_appends_runtime_context_to_base_prompt() {
        let context = test_context(ApprovalMode::Ask);
        let prompt = PromptBuilder::with_base_prompt("Base behavior", &context).build();

        assert!(prompt.starts_with("Base behavior\n\n<environment_context>"));
        assert!(prompt.contains("<current_date source=\"system_utc\">2026-06-28</current_date>"));
        assert!(prompt.contains("<timezone known=\"true\">Asia/Calcutta</timezone>"));
        assert!(prompt.contains("<platform os=\"windows\" arch=\"x86_64\" />"));
        assert!(prompt.contains("<rendering_context format=\"json\">"));
        assert!(
            prompt.contains("&quot;runtime_rendering_method&quot;:&quot;gl_compatibility&quot;")
        );
    }

    #[test]
    fn prompt_escapes_project_context_values() {
        let context = test_context(ApprovalMode::Ask);
        let prompt = PromptBuilder::with_base_prompt("Base", &context).build();

        assert!(prompt.contains("<project_name>Prompt &amp; Policy</project_name>"));
        assert!(
            prompt.contains("<project_root>C:\\Projects\\Prompt &lt;Policy&gt;</project_root>")
        );
        assert!(prompt.contains(
            "<godot_executable_path>C:\\Godot\\Godot_v4.4-stable_win64.exe</godot_executable_path>"
        ));
    }

    #[test]
    fn ask_mode_marks_mutating_tools_as_approval_gated() {
        let context = test_context(ApprovalMode::Ask);
        let prompt = PromptBuilder::with_base_prompt("Base", &context).build();

        assert!(prompt.contains("<approval mode=\"ask\" label=\"Ask for approval\" />"));
        assert!(prompt.contains("<mutating_or_execution_tools decision=\"ask_user\">"));
        assert!(prompt.contains("validate_scene"));
        assert!(prompt.contains("write_or_update_file"));
        assert!(prompt.contains("project_settings.set"));
        assert!(prompt.contains("runtime_session.start"));
        assert!(prompt.contains("runtime_script"));
        assert!(prompt.contains("<shell enabled=\"true\" tool=\"exec_command\""));
        assert!(prompt.contains("cwd_policy=\"project_cwd_restricted\""));
        assert!(prompt.contains("<web_search enabled=\"false\""));
        assert!(!prompt.contains("file_ops"));
    }

    #[test]
    fn full_access_marks_mutating_tools_as_auto_allowed() {
        let context = test_context(ApprovalMode::FullAccess);
        let prompt = PromptBuilder::with_base_prompt("Base", &context).build();

        assert!(prompt.contains("<approval mode=\"full_access\" label=\"Full access\" />"));
        assert!(prompt.contains("<mutating_or_execution_tools decision=\"allow\">"));
        assert!(prompt.contains("validate_scene"));
        assert!(prompt.contains("write_or_update_file"));
        assert!(prompt.contains("runtime_session.stop"));
        assert!(!prompt.contains("<mutating_or_execution_tools decision=\"ask_user\">"));
    }

    #[test]
    fn tool_list_comes_from_chat_tool_registry() {
        let context = test_context(ApprovalMode::Ask);
        let prompt = PromptBuilder::with_base_prompt("Base", &context).build();

        for name in tools::allowed_tool_names() {
            assert!(prompt.contains(name), "prompt missing tool {name}");
        }
        assert!(tools::allowed_tool_names().contains(&"exec_command"));
        assert!(!tools::allowed_tool_names().contains(&"file_ops"));
    }

    #[test]
    fn real_generated_prompt_does_not_mention_removed_file_ops() {
        let context = test_context(ApprovalMode::Ask);
        let prompt = PromptBuilder::new(&context).build();

        assert!(prompt.contains("<available_fennara_tools>read_file"));
        assert!(!prompt.contains("file_ops"));
    }

    #[test]
    fn unix_date_conversion_is_stable() {
        assert_eq!(date_from_unix_days(0), "1970-01-01");
        assert_eq!(date_from_unix_days(20_632), "2026-06-28");
    }
}
