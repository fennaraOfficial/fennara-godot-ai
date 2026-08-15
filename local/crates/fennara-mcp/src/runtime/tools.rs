use super::{
    RuntimeConfig,
    daemon_client::{daemon_bound_status, daemon_status, daemon_tool_call},
    protocol::{SERVER_NAME, SERVER_VERSION, error_response, success_response},
    schemas::{is_forwarded_tool, load_embedded_tool_schemas},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

const MAX_MCP_TOOL_IMAGE_COUNT: usize = 6;
const MAX_MCP_TOOL_IMAGE_BYTES: usize = 8 * 1024 * 1024;
const MAX_MCP_TOOL_IMAGE_TOTAL_BYTES: usize = 24 * 1024 * 1024;
const LEGACY_CONCURRENCY_WARNING: &str = "Legacy-unbound routing uses the dock MCP Target or sole-editor fallback and is not safe for concurrent multi-project work.";

pub(crate) fn tools_list_result() -> Value {
    let mut tools = vec![json!({
        "name": "fennara_status",
        "description": "Return local Fennara MCP status. This verifies the MCP server is installed and reachable, and shows this MCP process's effective project route.",
        "inputSchema": {
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }
    })];

    tools.extend(load_embedded_tool_schemas());

    json!({
        "tools": tools
    })
}

pub(crate) fn handle_tool_call(id: Value, params: Option<&Value>, config: &RuntimeConfig) -> Value {
    let tool_name = params
        .and_then(|params| params.get("name"))
        .and_then(Value::as_str);

    match tool_name {
        Some("fennara_status") => success_response(id, status_tool_result(status_payload(config))),
        Some(name) if is_forwarded_tool(name) => {
            let args = tool_arguments(params);
            if !args.is_object() {
                return success_response(
                    id,
                    forwarded_tool_result(
                        name,
                        &json!({ "ok": false, "error": "Tool arguments must be a JSON object." }),
                        true,
                    ),
                );
            }
            let result = match daemon_tool_call(name, args, config.project_path()) {
                Ok(payload) => payload,
                Err(error) => json!({
                    "ok": false,
                    "error": error
                }),
            };
            let is_error = result.get("ok").and_then(Value::as_bool) == Some(false);
            success_response(id, forwarded_tool_result(name, &result, is_error))
        }
        Some(name) => error_response(id, -32602, format!("Unknown tool: {name}")),
        None => error_response(id, -32602, "Missing tool name".to_string()),
    }
}

fn tool_arguments(params: Option<&Value>) -> Value {
    params
        .and_then(|params| params.get("arguments"))
        .cloned()
        .unwrap_or_else(|| json!({}))
}

fn status_payload(config: &RuntimeConfig) -> Value {
    let status = match config.project_path() {
        Some(project_path) => daemon_bound_status(project_path),
        None => daemon_status(),
    };
    match status {
        Ok(status) => connected_status_payload_for_config(status, config),
        Err(error) => json!({
            "ok": true,
            "server": SERVER_NAME,
            "version": SERVER_VERSION,
            "daemon_connected": false,
            "godot_plugin_connected": false,
            "routing_mode": if config.project_path().is_some() { "bound" } else { "legacy_unbound" },
            "binding_source": config.binding_source().map(|source| source.as_str()),
            "bound_project_path": config.project_path(),
            "concurrency_warning": if config.project_path().is_some() {
                Value::Null
            } else {
                Value::String(LEGACY_CONCURRENCY_WARNING.to_string())
            },
            "message": format!("Open a Godot project with Fennara enabled. The local daemon is not reachable yet: {error}")
        }),
    }
}

#[cfg(test)]
fn connected_status_payload(status: Value) -> Value {
    connected_status_payload_for_config(status, &RuntimeConfig::default())
}

fn connected_status_payload_for_config(status: Value, config: &RuntimeConfig) -> Value {
    let is_bound = config.project_path().is_some();
    let bound_editor_state = status.get("bound_editor_state").and_then(Value::as_str);
    let selected_project = if is_bound && bound_editor_state == Some("connected") {
        status
            .get("selected_project")
            .filter(|project| project.is_object())
            .cloned()
    } else {
        None
    };
    let editor_filesystem =
        if is_bound && bound_editor_state == Some("connected") && selected_project.is_some() {
            status
                .get("editor_filesystem")
                .filter(|filesystem| filesystem.is_object())
                .or_else(|| {
                    selected_project
                        .as_ref()
                        .and_then(|project| project.get("editor_filesystem"))
                        .filter(|filesystem| filesystem.is_object())
                })
                .cloned()
        } else {
            status
                .get("active_project")
                .filter(|project| project.is_object())
                .and_then(|project| project.get("editor_filesystem"))
                .filter(|filesystem| filesystem.is_object())
                .cloned()
        };
    json!({
        "ok": true,
        "server": SERVER_NAME,
        "version": SERVER_VERSION,
        "daemon_connected": true,
        "routing_mode": if is_bound { "bound" } else { "legacy_unbound" },
        "binding_source": config.binding_source().map(|source| source.as_str()),
        "bound_project_path": config.project_path(),
        "bound_editor_state": bound_editor_state,
        "routing_code": status.get("code").cloned().unwrap_or(Value::Null),
        "retryable": status.get("retryable").cloned().unwrap_or(Value::Null),
        "daemon": daemon_status_for_mcp(status, is_bound),
        "editor_filesystem": editor_filesystem,
        "selected_project": selected_project,
        "concurrency_warning": if is_bound {
            Value::Null
        } else {
            Value::String(LEGACY_CONCURRENCY_WARNING.to_string())
        }
    })
}

fn daemon_status_for_mcp(mut status: Value, is_bound: bool) -> Value {
    if let Some(active_project) = status
        .get("active_project")
        .filter(|active_project| active_project.is_object())
    {
        status["active_project"] = active_project_summary(active_project);
    }
    if let Some(legacy_active_project) = status
        .get("legacy_active_project")
        .filter(|project| project.is_object())
    {
        status["legacy_active_project"] = active_project_summary(legacy_active_project);
    }
    if is_bound && let Some(object) = status.as_object_mut() {
        object.remove("connected_projects");
    }
    status
}

fn active_project_summary(project: &Value) -> Value {
    json!({
        "project_name": string_field(project, "project_name"),
        "project_path": string_field(project, "project_path")
    })
}

fn status_tool_result(payload: Value) -> Value {
    let text = status_markdown(&payload);
    json!({
        "content": [
            {
                "type": "text",
                "text": text
            }
        ],
        "isError": false
    })
}

fn status_markdown(payload: &Value) -> String {
    let mut lines = vec![
        "Tool: fennara_status".to_string(),
        "Status: success".to_string(),
    ];

    let server = string_field(payload, "server").unwrap_or_else(|| SERVER_NAME.to_string());
    let version = string_field(payload, "version").unwrap_or_else(|| SERVER_VERSION.to_string());
    lines.push(format!(
        "MCP server: {} {}",
        markdown_escape(&server),
        markdown_escape(&version)
    ));

    let daemon_connected = payload
        .get("daemon_connected")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    lines.push(format!("Daemon: {}", connection_state(daemon_connected)));

    append_routing_lines(&mut lines, payload);

    if daemon_connected {
        let is_bound = string_field(payload, "routing_mode").as_deref() == Some("bound");
        append_daemon_status_lines(&mut lines, payload.get("daemon"), is_bound);
        append_editor_filesystem_status(&mut lines, payload.get("editor_filesystem"));
    } else {
        if let Some(plugin_connected) = payload
            .get("godot_plugin_connected")
            .and_then(Value::as_bool)
        {
            lines.push(format!(
                "Godot plugin: {}",
                connection_state(plugin_connected)
            ));
        }
        if let Some(message) = string_field(payload, "message") {
            lines.push(format!("Message: {}", markdown_escape(&message)));
        }
    }

    lines.join("\n")
}

fn append_routing_lines(lines: &mut Vec<String>, payload: &Value) {
    let routing_mode =
        string_field(payload, "routing_mode").unwrap_or_else(|| "legacy_unbound".to_string());
    lines.push(format!("Routing mode: {routing_mode}"));

    if routing_mode == "bound" {
        if let Some(source) = string_field(payload, "binding_source") {
            lines.push(format!("Binding source: {}", markdown_escape(&source)));
        }
        if let Some(project_path) = string_field(payload, "bound_project_path") {
            lines.push(format!(
                "Bound project root: {}",
                markdown_escape(&project_path)
            ));
        }
        if let Some(state) = string_field(payload, "bound_editor_state") {
            lines.push(format!("Bound editor: {state}"));
        }
        if let Some(project) = payload
            .get("selected_project")
            .filter(|project| project.is_object())
        {
            let project_name = string_field(project, "project_name")
                .unwrap_or_else(|| "connected project".to_string());
            lines.push(format!(
                "Selected editor project: {}",
                markdown_escape(&project_name)
            ));
            append_project_field(lines, project, "project_path", "Selected project path");
            append_project_field(lines, project, "session_id", "Selected editor session");
        }
        if let Some(code) = string_field(payload, "routing_code") {
            lines.push(format!("Routing code: {code}"));
        }
        if let Some(retryable) = payload.get("retryable").and_then(Value::as_bool) {
            lines.push(format!(
                "Retryable: {}",
                if retryable { "yes" } else { "no" }
            ));
        }
    } else if let Some(warning) = string_field(payload, "concurrency_warning") {
        lines.push(format!(
            "Concurrency warning: {}",
            markdown_escape(&warning)
        ));
    }
}

fn append_editor_filesystem_status(lines: &mut Vec<String>, status: Option<&Value>) {
    let Some(status) = status.filter(|status| status.is_object()) else {
        return;
    };

    let state = string_field(status, "state").unwrap_or_else(|| "unknown".to_string());
    lines.push(format!("Editor filesystem: {}", markdown_escape(&state)));

    let asset_tools_ready = status
        .get("asset_tools_ready")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    lines.push(format!(
        "Asset tools ready: {}",
        if asset_tools_ready { "yes" } else { "no" }
    ));

    if status.get("is_scanning").and_then(Value::as_bool) == Some(true)
        && let Some(progress) = status.get("scan_progress").and_then(Value::as_f64)
    {
        lines.push(format!(
            "Editor filesystem scan progress: {:.1}%",
            progress.clamp(0.0, 1.0) * 100.0
        ));
    }

    if let Some(reason) = string_field(status, "not_ready_reason") {
        lines.push(format!("Asset tools note: {}", markdown_escape(&reason)));
    }
}

fn append_daemon_status_lines(lines: &mut Vec<String>, daemon: Option<&Value>, is_bound: bool) {
    let Some(daemon) = daemon else {
        return;
    };

    if let Some(version) = string_field(daemon, "version") {
        lines.push(format!("Daemon version: {}", markdown_escape(&version)));
    }
    if let Some(plugin_connected) = daemon
        .get("godot_plugin_connected")
        .and_then(Value::as_bool)
    {
        lines.push(format!(
            "Godot plugin: {}",
            connection_state(plugin_connected)
        ));
    }

    let legacy_active_project = daemon
        .get(if is_bound {
            "legacy_active_project"
        } else {
            "active_project"
        })
        .or_else(|| is_bound.then(|| daemon.get("active_project")).flatten());
    if let Some(project) = legacy_active_project.filter(|value| value.is_object()) {
        append_active_project_summary(lines, project, is_bound);
    } else {
        lines.push(format!(
            "{}: none",
            if is_bound {
                "Dock MCP target"
            } else {
                "Active project"
            }
        ));
    }

    let active_session_id = string_field(
        daemon,
        if is_bound {
            "legacy_active_session_id"
        } else {
            "active_session_id"
        },
    )
    .or_else(|| {
        is_bound
            .then(|| string_field(daemon, "active_session_id"))
            .flatten()
    });
    if let Some(session_id) = active_session_id.as_deref() {
        lines.push(format!(
            "{}: {}",
            if is_bound {
                "Dock MCP target session"
            } else {
                "Active session"
            },
            markdown_escape(session_id)
        ));
    }
    if let Some(projects) = daemon.get("connected_projects").and_then(Value::as_array) {
        append_connected_projects(lines, projects, active_session_id.as_deref());
    }
}

fn append_active_project_summary(lines: &mut Vec<String>, project: &Value, is_bound: bool) {
    let project_name =
        string_field(project, "project_name").unwrap_or_else(|| "connected project".to_string());
    lines.push(format!(
        "{}: {}",
        if is_bound {
            "Dock MCP target"
        } else {
            "Active project"
        },
        markdown_escape(&project_name)
    ));

    if let Some(project_path) = string_field(project, "project_path") {
        lines.push(format!(
            "{}: {}",
            if is_bound {
                "Dock MCP target path"
            } else {
                "Active project path"
            },
            markdown_escape(&project_path)
        ));
    }
}

fn append_connected_projects(
    lines: &mut Vec<String>,
    projects: &[Value],
    active_session_id: Option<&str>,
) {
    lines.push(format!("Connected projects: {}", projects.len()));
    for (index, project) in projects.iter().enumerate() {
        if !project.is_object() {
            lines.push(format!("{}. unsupported project status", index + 1));
            continue;
        }

        let title = string_field(project, "project_name")
            .or_else(|| string_field(project, "project_path"))
            .unwrap_or_else(|| "connected project".to_string());
        let is_active = active_session_id
            .zip(string_field(project, "session_id").as_deref())
            .is_some_and(|(active, project_session)| active == project_session);
        let marker = if is_active { " (active)" } else { "" };
        lines.push(format!(
            "{}. {}{marker}",
            index + 1,
            markdown_escape(&title)
        ));

        append_project_field(lines, project, "project_path", "Path");
        append_project_field(lines, project, "session_id", "Session");
        append_project_field(lines, project, "godot_version", "Godot");
        append_project_field(lines, project, "plugin_version", "Plugin");
        append_project_field(lines, project, "godot_executable_path", "Godot executable");
        append_project_tools(lines, project);
        append_rendering_context(lines, project.get("rendering_context"));
    }
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(single_line)
        .filter(|value| !value.is_empty())
}

fn append_project_field(lines: &mut Vec<String>, value: &Value, key: &str, label: &str) {
    if let Some(field) = string_field(value, key) {
        lines.push(format!("   {label}: {}", markdown_escape(&field)));
    }
}

fn append_project_tools(lines: &mut Vec<String>, project: &Value) {
    let Some(tools) = project.get("tools").and_then(Value::as_array) else {
        return;
    };
    let tool_names: Vec<_> = tools
        .iter()
        .filter_map(Value::as_str)
        .map(|tool_name| markdown_escape(&single_line(tool_name)))
        .filter(|name| !name.is_empty())
        .collect();
    if !tool_names.is_empty() {
        lines.push(format!("   Tools: {}", tool_names.join(", ")));
    }
}

fn append_rendering_context(lines: &mut Vec<String>, rendering_context: Option<&Value>) {
    let Some(context) = rendering_context.filter(|value| value.is_object()) else {
        return;
    };

    append_project_field(
        lines,
        context,
        "runtime_rendering_method",
        "Rendering method",
    );
    append_project_field(
        lines,
        context,
        "runtime_rendering_driver_name",
        "Rendering driver",
    );
    append_project_field(lines, context, "video_adapter_name", "Video adapter");
    append_project_field(lines, context, "os_name", "OS");

    let warnings: Vec<_> = context
        .get("warnings")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(|warning| markdown_escape(&single_line(warning)))
        .filter(|warning| !warning.is_empty())
        .collect();
    if !warnings.is_empty() {
        lines.push(format!("   Rendering warnings: {}", warnings.join("; ")));
    }
}

fn single_line(value: &str) -> String {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn markdown_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        if matches!(
            ch,
            '\\' | '`' | '*' | '_' | '[' | ']' | '<' | '>' | '(' | ')'
        ) {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped
}

fn connection_state(connected: bool) -> &'static str {
    if connected {
        "connected"
    } else {
        "not connected"
    }
}

fn forwarded_tool_result(tool_name: &str, response: &Value, is_error: bool) -> Value {
    let mut content = vec![json!({
        "type": "text",
        "text": text_from_plugin_result(tool_name, response)
    })];
    content.extend(image_content_for_tool_result(tool_name, response));

    json!({
        "content": content,
        "isError": is_error
    })
}

fn image_content_for_tool_result(tool_name: &str, response: &Value) -> Vec<Value> {
    if !tool_supports_mcp_images(tool_name) {
        return Vec::new();
    }

    let mut content = Vec::new();
    let mut total_bytes = 0usize;
    for image in model_images_for_tool_result(tool_name, response)
        .into_iter()
        .take(MAX_MCP_TOOL_IMAGE_COUNT)
    {
        match mcp_image_block(image, &mut total_bytes) {
            ImageBlockResult::Block(block) => {
                let label = model_image_label(tool_name, image);
                content.push(json!({ "type": "text", "text": label }));
                content.push(block);
            }
            ImageBlockResult::Omitted(reason) => content.push(json!({
                "type": "text",
                "text": format!("[Image from {tool_name} omitted from MCP image context: {reason}]")
            })),
            ImageBlockResult::None => {}
        }
    }
    content
}

fn tool_supports_mcp_images(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "screenshot_scene" | "runtime_session" | "runtime_script"
    )
}

fn model_images_for_tool_result<'a>(tool_name: &str, response: &'a Value) -> Vec<&'a Value> {
    let images: Vec<_> = response
        .get("model_images")
        .and_then(Value::as_array)
        .map(|images| images.iter().collect())
        .unwrap_or_default();
    if !images.is_empty() || tool_name != "screenshot_scene" {
        return images;
    }
    response
        .get("raw_result")
        .filter(|raw_result| {
            raw_result
                .get("image_base64")
                .and_then(Value::as_str)
                .is_some()
        })
        .into_iter()
        .collect()
}

enum ImageBlockResult {
    Block(Value),
    Omitted(String),
    None,
}

fn mcp_image_block(image: &Value, total_bytes: &mut usize) -> ImageBlockResult {
    let Some(data) = image
        .get("data")
        .or_else(|| image.get("image_base64"))
        .and_then(Value::as_str)
    else {
        return ImageBlockResult::None;
    };
    if data.trim().is_empty() {
        return ImageBlockResult::None;
    }
    if !data.chars().all(is_base64_char) {
        return ImageBlockResult::Omitted("base64 payload was invalid".to_string());
    }
    let decoded_bytes = estimated_decoded_bytes(data);
    if decoded_bytes > MAX_MCP_TOOL_IMAGE_BYTES {
        return ImageBlockResult::Omitted(format!(
            "image exceeded {} MB",
            MAX_MCP_TOOL_IMAGE_BYTES / 1024 / 1024
        ));
    }
    if total_bytes.saturating_add(decoded_bytes) > MAX_MCP_TOOL_IMAGE_TOTAL_BYTES {
        return ImageBlockResult::Omitted(format!(
            "image budget exceeded {} MB",
            MAX_MCP_TOOL_IMAGE_TOTAL_BYTES / 1024 / 1024
        ));
    }

    let decoded = match STANDARD.decode(data.as_bytes()) {
        Ok(decoded) if !decoded.is_empty() => decoded,
        _ => return ImageBlockResult::Omitted("base64 payload was invalid".to_string()),
    };
    let Some(detected_mime) = detect_image_mime(&decoded) else {
        return ImageBlockResult::Omitted("unsupported image bytes".to_string());
    };
    let declared_mime = image
        .get("mime_type")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|mime| !mime.is_empty());
    let Some(mime_type) = declared_mime
        .map(normalize_supported_image_mime)
        .unwrap_or(Some(detected_mime))
    else {
        let mime_type = declared_mime.unwrap_or("unknown");
        return ImageBlockResult::Omitted(format!("unsupported MIME type {mime_type}"));
    };
    if mime_type != detected_mime {
        return ImageBlockResult::Omitted(format!(
            "MIME type {mime_type} did not match image bytes {detected_mime}"
        ));
    }

    *total_bytes += decoded_bytes;
    ImageBlockResult::Block(json!({
        "type": "image",
        "data": data,
        "mimeType": mime_type
    }))
}

fn model_image_label(tool_name: &str, image: &Value) -> String {
    image
        .get("label")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(|label| format!("[{label}]"))
        .unwrap_or_else(|| {
            if tool_name == "screenshot_scene" {
                "[Screenshot image from screenshot_scene]".to_string()
            } else {
                format!("[Image from {tool_name}]")
            }
        })
}

fn estimated_decoded_bytes(base64: &str) -> usize {
    base64.trim().len().saturating_mul(3) / 4
}

fn is_base64_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '+' | '/' | '=')
}

fn normalize_supported_image_mime(mime_type: &str) -> Option<&'static str> {
    match mime_type.trim().to_ascii_lowercase().as_str() {
        "image/png" => Some("image/png"),
        "image/jpeg" | "image/jpg" => Some("image/jpeg"),
        "image/webp" => Some("image/webp"),
        "image/gif" => Some("image/gif"),
        _ => None,
    }
}

fn detect_image_mime(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Some("image/png");
    }
    if bytes.starts_with(b"\xff\xd8\xff") {
        return Some("image/jpeg");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    None
}

fn text_from_plugin_result(tool_name: &str, response: &Value) -> String {
    if let Some(result) = response.get("result") {
        if let Some(text) = result.as_str() {
            return text.to_string();
        }
        if !result.is_null() {
            return result.to_string();
        }
    }

    if let Some(error) = response.get("error").and_then(Value::as_str) {
        let mut lines = vec![
            format!("Tool: {tool_name}"),
            "Status: failed".to_string(),
            format!("Error: {error}"),
        ];
        if let Some(code) = response
            .get("code")
            .and_then(Value::as_str)
            .map(single_line)
            .filter(|code| !code.is_empty())
        {
            lines.push(format!("Code: {code}"));
        }
        if let Some(retryable) = response.get("retryable").and_then(Value::as_bool) {
            lines.push(format!(
                "Retryable: {}",
                if retryable { "yes" } else { "no" }
            ));
        }
        return lines.join("\n");
    }

    format!("Tool: {tool_name}\nStatus: failed\nError: Tool returned an unsupported result shape.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        ffi::OsString,
        fs,
        sync::atomic::{AtomicU64, Ordering},
    };

    const PNG_1X1: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/p9sAAAAASUVORK5CYII=";
    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    fn bound_config() -> (RuntimeConfig, String) {
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "fennara-mcp-status-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("create status fixture");
        fs::write(root.join("project.godot"), b"[application]\n").expect("write project.godot");
        let config = RuntimeConfig::from_args_and_env(
            [
                OsString::from("--project-path"),
                root.clone().into_os_string(),
            ],
            None,
            &root,
        )
        .expect("bind status fixture");
        let canonical = config
            .project_path()
            .expect("bound project root")
            .to_string();
        fs::remove_dir_all(root).expect("remove status fixture");
        (config, canonical)
    }

    #[test]
    fn bound_status_uses_daemon_authoritative_editor_selection() {
        let (config, project_root) = bound_config();
        let payload = connected_status_payload_for_config(
            json!({
                "ok": true,
                "version": "0.4.2",
                "godot_plugin_connected": true,
                "routing_mode": "bound",
                "bound_editor_state": "connected",
                "bound_project_path": project_root.clone(),
                "selected_project": {
                    "project_name": "Agent A",
                    "project_path": project_root.clone(),
                    "session_id": "agent-a#10",
                    "editor_filesystem": {
                        "state": "ready",
                        "asset_tools_ready": true
                    }
                },
                "legacy_active_project": {
                    "project_name": "Agent B",
                    "project_path": "/worktrees/agent-b"
                },
                "legacy_active_session_id": "agent-b#11"
            }),
            &config,
        );

        let text = status_tool_result(payload)["content"][0]["text"]
            .as_str()
            .expect("status text")
            .to_string();
        assert!(text.contains("Routing mode: bound"));
        assert!(text.contains("Binding source: cli"));
        assert!(text.contains(&format!("Bound project root: {project_root}")));
        assert!(text.contains("Bound editor: connected"));
        assert!(text.contains("Selected editor project: Agent A"));
        assert!(text.contains("Editor filesystem: ready"));
        assert!(text.contains("Dock MCP target: Agent B"));
    }

    #[test]
    fn failed_bound_status_never_falls_back_to_dock_filesystem_readiness() {
        let (config, project_root) = bound_config();
        let payload = connected_status_payload_for_config(
            json!({
                "ok": true,
                "version": "0.4.2",
                "godot_plugin_connected": true,
                "routing_mode": "bound",
                "bound_editor_state": "not_connected",
                "bound_project_path": project_root,
                "code": "bound_project_not_connected",
                "retryable": true,
                "selected_project": null,
                "editor_filesystem": null,
                "legacy_active_project": {
                    "project_name": "Unrelated Dock Project",
                    "project_path": "/worktrees/unrelated",
                    "editor_filesystem": {
                        "state": "ready",
                        "asset_tools_ready": true
                    }
                }
            }),
            &config,
        );

        let text = status_tool_result(payload)["content"][0]["text"]
            .as_str()
            .expect("status text")
            .to_string();
        assert!(text.contains("Bound editor: not_connected"));
        assert!(text.contains("Routing code: bound_project_not_connected"));
        assert!(text.contains("Retryable: yes"));
        assert!(text.contains("Dock MCP target: Unrelated Dock Project"));
        assert!(!text.contains("Editor filesystem:"));
        assert!(!text.contains("Asset tools ready:"));
    }

    #[test]
    fn status_tool_result_uses_plain_text_without_duplicate_structured_content() {
        let active_project = json!({
            "project_name": "Top_Down Template 2d",
            "project_path": "C:\\godot\\SimpleTopDownShooter_Template2D\\",
            "session_id": "C:\\godot\\SimpleTopDownShooter_Template2D\\#26740",
            "godot_version": "4.6.3-stable (official)",
            "plugin_version": "0.3.5",
            "godot_executable_path": "C:/Users/Tushar/Downloads/GODOT/Godot.exe",
            "tools": ["read_file", "screenshot_scene"],
            "rendering_context": {
                "schema_version": "rendering-context-v1",
                "runtime_rendering_method": "forward_plus",
                "runtime_rendering_driver_name": "vulkan",
                "video_adapter_name": "NVIDIA GPU",
                "os_name": "Windows"
            }
        });
        let second_project = json!({
            "project_name": "Puzzle_Project [Test]",
            "project_path": "D:\\Games\\Puzzle_Project\\",
            "session_id": "D:/Games/Puzzle/#99",
            "godot_version": "4.5-stable",
            "plugin_version": "0.3.7",
            "rendering_context": {
                "runtime_rendering_method": "mobile"
            }
        });
        let payload = connected_status_payload(json!({
            "ok": true,
            "version": "0.3.7",
            "godot_plugin_connected": true,
            "active_session_id": "C:\\godot\\SimpleTopDownShooter_Template2D\\#26740",
            "active_project": active_project,
            "connected_projects": [active_project, second_project]
        }));

        let result = status_tool_result(payload);

        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Tool: fennara_status"));
        assert!(text.contains(&format!(
            "MCP server: fennara-mcp {}",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(text.contains("Daemon: connected"));
        assert!(text.contains("Active project: Top\\_Down Template 2d"));
        assert!(
            text.contains(
                "Active project path: C:\\\\godot\\\\SimpleTopDownShooter\\_Template2D\\\\"
            )
        );
        assert!(text.contains(
            "Active session: C:\\\\godot\\\\SimpleTopDownShooter\\_Template2D\\\\#26740"
        ));
        assert!(text.contains("Connected projects: 2"));
        assert!(text.contains("1. Top\\_Down Template 2d (active)"));
        assert!(text.contains("2. Puzzle\\_Project \\[Test\\]"));
        assert!(text.contains("Path: D:\\\\Games\\\\Puzzle\\_Project\\\\"));
        assert!(text.contains("Godot: 4.5-stable"));
        assert!(text.contains("Tools: read\\_file, screenshot\\_scene"));
        assert!(!text.contains("rendering_context"));
        assert!(!text.contains("connected_projects"));
        assert!(result.get("structuredContent").is_none());
    }

    #[test]
    fn status_tool_result_formats_disconnected_state() {
        let payload = json!({
            "ok": true,
            "server": "fennara-mcp",
            "version": "0.3.7",
            "daemon_connected": false,
            "godot_plugin_connected": false,
            "routing_mode": "legacy_unbound",
            "concurrency_warning": LEGACY_CONCURRENCY_WARNING,
            "message": "Open a Godot project with Fennara enabled."
        });

        let result = status_tool_result(payload);

        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Daemon: not connected"));
        assert!(text.contains("Godot plugin: not connected"));
        assert!(text.contains("Routing mode: legacy_unbound"));
        assert!(text.contains("Concurrency warning:"));
        assert!(text.contains("Message: Open a Godot project with Fennara enabled."));
    }

    #[test]
    fn status_tool_result_includes_live_editor_filesystem_readiness() {
        let payload = connected_status_payload(json!({
            "ok": true,
            "version": "0.3.12",
            "godot_plugin_connected": true,
            "active_session_id": "D:/project/#42",
            "active_project": {
                "project_name": "Import Test",
                "project_path": "D:/project/",
                "editor_filesystem": {
                    "schema_version": "editor-filesystem-status-v1",
                    "available": true,
                    "state": "scanning_and_importing",
                    "initial_scan_complete": false,
                    "is_scanning": true,
                    "scan_progress": 0.625,
                    "asset_tools_ready": false,
                    "not_ready_reason": "Godot is still scanning and importing project resources."
                }
            },
            "connected_projects": []
        }));

        let result = status_tool_result(payload);
        let text = result["content"][0]["text"].as_str().unwrap();

        assert!(text.contains(r"Editor filesystem: scanning\_and\_importing"));
        assert!(text.contains("Asset tools ready: no"));
        assert!(text.contains("Editor filesystem scan progress: 62.5%"));
        assert!(result.get("structuredContent").is_none());
    }

    #[test]
    fn status_tool_result_handles_connected_daemon_without_active_project() {
        let payload = connected_status_payload(json!({
            "ok": true,
            "version": "0.3.7",
            "godot_plugin_connected": false,
            "active_session_id": null,
            "active_project": null,
            "connected_projects": []
        }));

        let result = status_tool_result(payload);

        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Daemon: connected"));
        assert!(text.contains("Godot plugin: not connected"));
        assert!(text.contains("Active project: none"));
        assert!(text.contains("Connected projects: 0"));
        assert!(result.get("structuredContent").is_none());
    }

    #[test]
    fn forwarded_tool_result_sends_only_plugin_result() {
        let response = json!({
            "ok": true,
            "result": "Tool: validate_scene\nStatus: success",
            "formatted_result": {
                "content": "wrong layer",
                "metadata": {
                    "tool_name": "validate_scene"
                }
            },
            "raw_result": {
                "scenes": [
                    { "scene_path": "res://huge.tscn", "issues": [{ "message": "raw detail" }] }
                ]
            },
            "request_id": "local-tool-1",
            "type": "tool_result"
        });

        let result = forwarded_tool_result("validate_scene", &response, false);

        assert_eq!(
            result["content"][0]["text"],
            "Tool: validate_scene\nStatus: success"
        );
        assert!(result.get("structuredContent").is_none());
        assert!(!result.to_string().contains("wrong layer"));
        assert!(!result.to_string().contains("raw detail"));
        assert!(!result.to_string().contains("raw_result"));
    }

    #[test]
    fn forwarded_tool_result_reports_bridge_error_when_plugin_result_is_missing() {
        let response = json!({
            "ok": false,
            "error": "Godot plugin disconnected before returning a tool result."
        });

        let result = forwarded_tool_result("project_settings", &response, true);

        assert_eq!(
            result["content"][0]["text"],
            "Tool: project_settings\nStatus: failed\nError: Godot plugin disconnected before returning a tool result."
        );
        assert_eq!(result["isError"], true);
    }

    #[test]
    fn forwarded_routing_error_preserves_stable_code_and_retryability() {
        let response = json!({
            "ok": false,
            "error": "No connected Godot editor matches this MCP Project Binding.",
            "code": "bound_project_not_connected",
            "retryable": true
        });

        let result = forwarded_tool_result("project_settings", &response, true);
        let text = result["content"][0]["text"].as_str().unwrap();

        assert!(text.contains("Code: bound_project_not_connected"));
        assert!(text.contains("Retryable: yes"));
        assert_eq!(result["isError"], true);
    }

    #[test]
    fn forwarded_screenshot_result_attaches_mcp_image_content() {
        let response = json!({
            "ok": true,
            "result": "Tool: screenshot_scene\nStatus: success\nImage: 10x10 image/png",
            "raw_result": {
                "success": true,
                "width": 10,
                "height": 10,
                "image_role": "single"
            },
            "model_images": [
                {
                    "data": PNG_1X1,
                    "mime_type": "image/png",
                    "label": "Screenshot from screenshot_scene (single)",
                    "width": 10,
                    "height": 10
                }
            ]
        });

        let result = forwarded_tool_result("screenshot_scene", &response, false);

        assert_eq!(result["content"][0]["type"], "text");
        assert_eq!(
            result["content"][0]["text"],
            "Tool: screenshot_scene\nStatus: success\nImage: 10x10 image/png"
        );
        assert_eq!(result["content"][1]["type"], "text");
        assert_eq!(
            result["content"][1]["text"],
            "[Screenshot from screenshot_scene (single)]"
        );
        assert_eq!(result["content"][2]["type"], "image");
        assert_eq!(result["content"][2]["data"], PNG_1X1);
        assert_eq!(result["content"][2]["mimeType"], "image/png");
        assert!(
            !result["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains(PNG_1X1)
        );
    }

    #[test]
    fn forwarded_screenshot_result_attaches_multiple_mcp_images_in_order() {
        let response = json!({
            "ok": true,
            "result": "Tool: screenshot_scene\nStatus: success\nImages: 2",
            "model_images": [
                {
                    "data": PNG_1X1,
                    "mime_type": "image/png",
                    "label": "Screenshot capture 1 of 2 (camera_search)"
                },
                {
                    "data": PNG_1X1,
                    "mime_type": "image/png",
                    "label": "Screenshot capture 2 of 2 (view=top)"
                }
            ]
        });

        let result = forwarded_tool_result("screenshot_scene", &response, false);

        assert_eq!(result["content"].as_array().unwrap().len(), 5);
        assert_eq!(
            result["content"][1]["text"],
            "[Screenshot capture 1 of 2 (camera_search)]"
        );
        assert_eq!(result["content"][2]["type"], "image");
        assert_eq!(
            result["content"][3]["text"],
            "[Screenshot capture 2 of 2 (view=top)]"
        );
        assert_eq!(result["content"][4]["type"], "image");
    }

    #[test]
    fn forwarded_screenshot_result_uses_legacy_raw_result_image_content() {
        let response = json!({
            "ok": true,
            "result": "Tool: screenshot_scene\nStatus: success",
            "raw_result": {
                "success": true,
                "image_base64": PNG_1X1,
                "mime_type": "image/png"
            }
        });

        let result = forwarded_tool_result("screenshot_scene", &response, false);

        assert_eq!(result["content"][1]["type"], "text");
        assert_eq!(
            result["content"][1]["text"],
            "[Screenshot image from screenshot_scene]"
        );
        assert_eq!(result["content"][2]["type"], "image");
        assert_eq!(result["content"][2]["data"], PNG_1X1);
        assert!(!result.to_string().contains("raw_result"));
    }

    #[test]
    fn forwarded_screenshot_result_omits_too_large_image() {
        let response = json!({
            "ok": true,
            "result": "Tool: screenshot_scene\nStatus: success",
            "raw_result": {
                "success": true
            },
            "model_images": [
                {
                    "data": "a".repeat((MAX_MCP_TOOL_IMAGE_BYTES * 4 / 3) + 16),
                    "mime_type": "image/png",
                    "label": "Screenshot from screenshot_scene"
                }
            ]
        });

        let result = forwarded_tool_result("screenshot_scene", &response, false);

        assert_eq!(result["content"].as_array().unwrap().len(), 2);
        assert_eq!(result["content"][1]["type"], "text");
        assert!(
            result["content"][1]["text"]
                .as_str()
                .unwrap()
                .contains("image exceeded")
        );
        assert!(!result.to_string().contains("\"type\":\"image\""));
    }

    #[test]
    fn forwarded_runtime_script_result_attaches_multiple_mcp_images() {
        let response = json!({
            "ok": true,
            "result": "Tool: runtime_script\nStatus: completed\nCaptures: 2",
            "model_images": [
                {
                    "data": PNG_1X1,
                    "mime_type": "image/png",
                    "label": "Runtime script capture 1: before"
                },
                {
                    "data": PNG_1X1,
                    "mime_type": "image/png",
                    "label": "Runtime script capture 2: after"
                }
            ]
        });

        let result = forwarded_tool_result("runtime_script", &response, false);

        assert_eq!(
            result["content"][1]["text"],
            "[Runtime script capture 1: before]"
        );
        assert_eq!(result["content"][2]["type"], "image");
        assert_eq!(result["content"][2]["data"], PNG_1X1);
        assert_eq!(
            result["content"][3]["text"],
            "[Runtime script capture 2: after]"
        );
        assert_eq!(result["content"][4]["type"], "image");
        assert_eq!(result["content"][4]["data"], PNG_1X1);
    }

    #[test]
    fn forwarded_image_result_omits_mime_mismatch() {
        let response = json!({
            "ok": true,
            "result": "Tool: runtime_script\nStatus: completed",
            "model_images": [
                {
                    "data": PNG_1X1,
                    "mime_type": "image/jpeg",
                    "label": "Wrong mime"
                }
            ]
        });

        let result = forwarded_tool_result("runtime_script", &response, false);

        assert_eq!(result["content"][1]["type"], "text");
        assert!(
            result["content"][1]["text"]
                .as_str()
                .unwrap()
                .contains("did not match image bytes")
        );
        assert!(!result.to_string().contains("\"type\":\"image\""));
    }
}
