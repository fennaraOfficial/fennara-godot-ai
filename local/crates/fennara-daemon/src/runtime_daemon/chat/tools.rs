use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use serde_json::{Value, json};

use crate::runtime_daemon::{godot_bridge, state::AppState};

use super::{exec_command, trace};

const READ_FILE_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/read_file.json"
));
const SCRIPT_DIAGNOSTICS_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/script_diagnostics.json"
));
const GET_CLASS_INFO_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/get_class_info.json"
));
const GET_SCENE_TREE_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/get_scene_tree.json"
));
const GET_NODE_PROPERTIES_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/get_node_properties.json"
));
const VALIDATE_SCENE_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/validate_scene.json"
));
const SCREENSHOT_SCENE_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/screenshot_scene.json"
));
const SCRAPE_EDITOR_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/scrape_editor.json"
));
const PROJECT_SETTINGS_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/project_settings.json"
));
const WRITE_OR_UPDATE_FILE_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/write_or_update_file.json"
));
const RUN_SCENE_EDIT_SCRIPT_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/run_scene_edit_script.json"
));
const RUNTIME_SESSION_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/runtime_session.json"
));
const RUNTIME_SCRIPT_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/runtime_script.json"
));
const EXEC_COMMAND_SCHEMA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/tools/exec_command.json"
));
const MAX_TOOL_MODEL_IMAGE_COUNT: usize = 1;
const MAX_TOOL_MODEL_IMAGE_BYTES: usize = 8 * 1024 * 1024;
const ALLOWED_TOOL_NAMES: &[&str] = &[
    "read_file",
    "script_diagnostics",
    "get_class_info",
    "get_scene_tree",
    "get_node_properties",
    "validate_scene",
    "screenshot_scene",
    "scrape_editor",
    "project_settings",
    "write_or_update_file",
    "run_scene_edit_script",
    "runtime_session",
    "runtime_script",
    "exec_command",
];
const TOOL_SCHEMAS: &[&str] = &[
    READ_FILE_SCHEMA,
    SCRIPT_DIAGNOSTICS_SCHEMA,
    GET_CLASS_INFO_SCHEMA,
    GET_SCENE_TREE_SCHEMA,
    GET_NODE_PROPERTIES_SCHEMA,
    VALIDATE_SCENE_SCHEMA,
    SCREENSHOT_SCENE_SCHEMA,
    SCRAPE_EDITOR_SCHEMA,
    PROJECT_SETTINGS_SCHEMA,
    WRITE_OR_UPDATE_FILE_SCHEMA,
    RUN_SCENE_EDIT_SCRIPT_SCHEMA,
    RUNTIME_SESSION_SCHEMA,
    RUNTIME_SCRIPT_SCHEMA,
    EXEC_COMMAND_SCHEMA,
];

#[derive(Clone, Debug)]
pub(crate) struct ExecutedTool {
    pub(crate) ok: bool,
    pub(crate) raw_result: Value,
    pub(crate) mcp_markdown: String,
    pub(crate) plugin_markdown: String,
    pub(crate) metadata: Value,
    pub(crate) target_keys: Vec<String>,
    pub(crate) model_followup_messages: Vec<Value>,
    pub(crate) model_images: Vec<ModelImage>,
}

#[derive(Clone, Debug)]
pub(crate) struct ModelImage {
    data: Option<String>,
    mime_type: String,
    label: String,
    size_bytes: usize,
    file_path: Option<String>,
    resource_path: Option<String>,
    access_token: String,
}

pub(crate) fn definitions() -> Vec<Value> {
    TOOL_SCHEMAS
        .iter()
        .map(|schema| openrouter_tool_from_schema(schema))
        .collect()
}

pub(crate) fn allowed_tool_names() -> &'static [&'static str] {
    ALLOWED_TOOL_NAMES
}

pub(crate) fn is_allowed_tool(name: &str) -> bool {
    ALLOWED_TOOL_NAMES.contains(&name)
}

fn is_daemon_local_tool(name: &str) -> bool {
    name == "exec_command"
}

pub(crate) async fn execute(
    state: &AppState,
    chat_id: &str,
    session_id: &str,
    tool_call_id: &str,
    project_root: Option<&str>,
    name: &str,
    arguments: &Value,
    recorder: Option<&trace::TraceRecorder>,
) -> ExecutedTool {
    if !is_allowed_tool(name) {
        return failed_tool(name, format!("Unsupported plugin chat tool: {name}"));
    }

    if is_daemon_local_tool(name) {
        return exec_command::execute(
            state,
            chat_id,
            session_id,
            tool_call_id,
            project_root,
            arguments,
        )
        .await;
    }

    let response = godot_bridge::call_tool_value_for_session_traced(
        state,
        Some(session_id),
        name,
        arguments.clone(),
        recorder,
    )
    .await;
    let ok = response.get("ok").and_then(Value::as_bool).unwrap_or(false);
    let mut raw_result = response
        .get("raw_result")
        .cloned()
        .unwrap_or_else(|| response.clone());
    let model_images = model_images_from_response(name, &response);
    if name == "screenshot_scene" {
        raw_result = strip_screenshot_image_bytes(raw_result);
    }
    let formatted = response.get("formatted_result").cloned().unwrap_or_else(
        || json!({ "content": response.get("result").cloned().unwrap_or(Value::Null) }),
    );
    let format_span = recorder.map(|recorder| {
        recorder.start_span(
            "tool.result.format",
            json!({
                "tool_name": name,
                "ok": ok,
                "raw_result_bytes": trace::value_size(&raw_result)
            }),
        )
    });
    let mut metadata = formatted
        .get("metadata")
        .cloned()
        .unwrap_or_else(|| json!({ "tool_name": name }));
    if let Some(plugin_metadata) = response
        .get("plugin_metadata")
        .filter(|value| value.is_object())
    {
        metadata["plugin_metadata"] = plugin_metadata.clone();
    }
    if !model_images.is_empty() {
        metadata["tool_images"] = tool_image_metadata(tool_call_id, &model_images);
    }
    let mcp_markdown = strip_update_notice(&markdown_from_response(&response, &formatted, name));
    let plugin_markdown = plugin_markdown_for(name, &mcp_markdown, &metadata, &raw_result, ok);
    let target_keys = target_keys_from_metadata(&metadata);
    let model_followup_messages = model_followups_for(name, &raw_result);
    if let Some(span) = format_span {
        span.finish(
            if ok { "ok" } else { "failed" },
            json!({
                "tool_name": name,
                "metadata_bytes": trace::value_size(&metadata),
                "mcp_markdown_bytes": mcp_markdown.len(),
                "plugin_markdown_bytes": plugin_markdown.len(),
                "target_key_count": target_keys.len(),
                "model_followup_count": model_followup_messages.len(),
                "model_image_count": model_images.len()
            }),
        );
    }

    ExecutedTool {
        ok,
        raw_result,
        mcp_markdown,
        plugin_markdown,
        metadata,
        target_keys,
        model_followup_messages,
        model_images,
    }
}

fn strip_update_notice(markdown: &str) -> String {
    const MARKER: &str = "\n\n---\n\nFennara is out of date.";
    markdown
        .find(MARKER)
        .map(|index| markdown[..index].trim_end().to_string())
        .unwrap_or_else(|| markdown.to_string())
}

pub(crate) fn failed_tool(name: &str, error: String) -> ExecutedTool {
    terminal_tool(name, "failed", error)
}

pub(crate) fn cancelled_tool(name: &str, error: String) -> ExecutedTool {
    terminal_tool(name, "cancelled", error)
}

pub(crate) fn timed_out_tool(name: &str, error: String) -> ExecutedTool {
    terminal_tool(name, "timed_out", error)
}

pub(crate) fn denied_tool(name: &str, error: String) -> ExecutedTool {
    terminal_tool(name, "denied", error)
}

fn terminal_tool(name: &str, status: &'static str, error: String) -> ExecutedTool {
    let markdown = format!("Tool: {name}\nStatus: {status}\nError: {error}");
    ExecutedTool {
        ok: false,
        raw_result: json!({ "success": false, "status": status, "error": error }),
        mcp_markdown: markdown.clone(),
        plugin_markdown: markdown,
        metadata: json!({
            "tool_name": name,
            "status": status,
            "format": "markdown",
        }),
        target_keys: Vec::new(),
        model_followup_messages: Vec::new(),
        model_images: Vec::new(),
    }
}

fn openrouter_tool_from_schema(schema: &str) -> Value {
    let schema = serde_json::from_str::<Value>(schema).unwrap_or_else(|_| json!({}));
    let description = schema
        .get("description")
        .cloned()
        .or_else(|| {
            schema
                .get("description_lines")
                .and_then(Value::as_array)
                .map(|lines| {
                    Value::String(
                        lines
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join("\n"),
                    )
                })
        })
        .unwrap_or(Value::String(String::new()));
    json!({
        "type": "function",
        "function": {
            "name": schema.get("name").cloned().unwrap_or(Value::String("unknown".to_string())),
            "description": description,
            "parameters": schema.get("parameters").cloned().unwrap_or_else(|| json!({
                "type": "object",
                "additionalProperties": false
            }))
        }
    })
}

fn markdown_from_response(response: &Value, formatted: &Value, name: &str) -> String {
    if let Some(content) = formatted.get("content").and_then(Value::as_str) {
        return content.to_string();
    }
    if let Some(result) = response.get("result").and_then(Value::as_str) {
        return result.to_string();
    }
    if let Some(error) = response.get("error").and_then(Value::as_str) {
        return format!("Tool: {name}\nStatus: failed\nError: {error}");
    }
    format!("Tool: {name}\nStatus: failed\nError: Tool returned an unsupported result shape.")
}

fn plugin_markdown_for(
    name: &str,
    mcp_markdown: &str,
    metadata: &Value,
    raw_result: &Value,
    ok: bool,
) -> String {
    let status = if ok { "completed" } else { "failed" };
    let targets = target_keys_from_metadata(metadata);
    let mut markdown = if targets.is_empty() {
        format!("{mcp_markdown}\n\nPlugin chat: {name} {status}.")
    } else {
        format!(
            "{mcp_markdown}\n\nPlugin chat: {name} {status} for {}.",
            targets.join(", ")
        )
    };
    if name == "read_file" && ok {
        let image_markdown = inline_image_markdown(raw_result, name);
        if !image_markdown.is_empty() {
            markdown.push_str("\n\n");
            markdown.push_str(&image_markdown);
        }
    }
    markdown
}

fn target_keys_from_metadata(metadata: &Value) -> Vec<String> {
    let target_keys: Vec<String> = metadata
        .get("targets")
        .and_then(Value::as_array)
        .map(|targets| {
            targets
                .iter()
                .filter_map(target_key)
                .filter(|path| !path.is_empty())
                .collect()
        })
        .unwrap_or_default();
    if target_keys.is_empty() {
        target_key(metadata).into_iter().collect()
    } else {
        target_keys
    }
}

fn target_key(target: &Value) -> Option<String> {
    if let Some(path) = target.get("file_path").and_then(Value::as_str) {
        return Some(normalize_res_path(path));
    }
    if let Some(class_name) = target.get("class_name").and_then(Value::as_str) {
        return Some(class_name.to_string());
    }
    if let Some(editor_target) = target.get("target").and_then(Value::as_str) {
        return Some(editor_target.to_string());
    }
    for key in [
        "resource_path",
        "script_path",
        "log_path",
        "session_id",
        "key",
        "prefix",
        "query",
    ] {
        if let Some(value) = target.get(key).and_then(Value::as_str) {
            if !value.is_empty() {
                return Some(if value.starts_with("res://") {
                    normalize_res_path(value)
                } else {
                    value.to_string()
                });
            }
        }
    }
    let scene_path = target
        .get("scene_path")
        .and_then(Value::as_str)
        .map(normalize_res_path);
    if let Some(scene_path) = scene_path {
        let node_path = target
            .get("resolved_path")
            .or_else(|| target.get("node_path"))
            .and_then(Value::as_str)
            .unwrap_or_default();
        if node_path.is_empty() {
            Some(scene_path)
        } else {
            Some(format!("{scene_path}#{node_path}"))
        }
    } else {
        None
    }
}

pub(crate) fn model_followups_for(name: &str, raw_result: &Value) -> Vec<Value> {
    if name == "read_file" {
        return read_file_model_images(raw_result);
    }
    Vec::new()
}

pub(crate) fn model_messages_for_tool_result(
    tool_name: &str,
    result: &ExecutedTool,
    allow_images: bool,
) -> Vec<Value> {
    let mut messages = if allow_images {
        result.model_followup_messages.clone()
    } else {
        text_only_followups(tool_name, &result.model_followup_messages)
    };
    if !result.model_images.iter().any(ModelImage::has_model_data) {
        return messages;
    }
    if allow_images {
        messages.extend(screenshot_model_image_messages(
            tool_name,
            &result.model_images,
        ));
    } else {
        messages.push(json!({
            "role": "user",
            "content": format!(
                "[Image output from {tool_name} omitted because the selected model does not support image input. The textual tool result and saved file path remain available.]"
            )
        }));
    }
    messages
}

pub(crate) fn ui_images_for_tool_result(tool_call_id: &str, result: &ExecutedTool) -> Vec<Value> {
    result.tool_image_metadata(tool_call_id)
}

fn read_file_model_images(raw_result: &Value) -> Vec<Value> {
    raw_result
        .get("files")
        .and_then(Value::as_array)
        .map(|files| {
            files
                .iter()
                .filter_map(|file| {
                    let image = file.get("image")?;
                    let image_part = read_file_image_content_part(image)?;
                    let path = file.get("path").and_then(Value::as_str).unwrap_or("image");
                    Some(json!({
                        "role": "user",
                        "content": [
                            { "type": "text", "text": format!("[Image read from {path}]") },
                            image_part
                        ]
                    }))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn screenshot_model_image_messages(tool_name: &str, images: &[ModelImage]) -> Vec<Value> {
    images
        .iter()
        .filter_map(|image| {
            let image_part = model_image_content_part(image)?;
            Some(json!({
                "role": "user",
                "content": [
                    { "type": "text", "text": format!("[{}]", image.label_for_tool(tool_name)) },
                    image_part
                ]
            }))
        })
        .collect()
}

fn model_image_content_part(image: &ModelImage) -> Option<Value> {
    let data = image.data.as_deref()?;
    Some(json!({
        "type": "image_url",
        "image_url": {
            "url": format!("data:{};base64,{data}", image.mime_type)
        }
    }))
}

fn text_only_followups(tool_name: &str, messages: &[Value]) -> Vec<Value> {
    if !messages.iter().any(value_includes_image) {
        return messages.to_vec();
    }
    vec![json!({
        "role": "user",
        "content": format!(
            "[Image output from {tool_name} omitted because the selected model does not support image input. The textual tool result remains available.]"
        )
    })]
}

fn value_includes_image(value: &Value) -> bool {
    match value {
        Value::Object(object) => {
            object
                .get("type")
                .and_then(Value::as_str)
                .is_some_and(|kind| kind == "image_url")
                || object.values().any(value_includes_image)
        }
        Value::Array(items) => items.iter().any(value_includes_image),
        _ => false,
    }
}

fn model_images_from_response(tool_name: &str, response: &Value) -> Vec<ModelImage> {
    if tool_name != "screenshot_scene" {
        return Vec::new();
    }
    response
        .get("model_images")
        .and_then(Value::as_array)
        .map(|images| {
            images
                .iter()
                .take(MAX_TOOL_MODEL_IMAGE_COUNT)
                .filter_map(validate_model_image)
                .collect()
        })
        .unwrap_or_default()
}

fn validate_model_image(image: &Value) -> Option<ModelImage> {
    let raw_data = image
        .get("data")
        .or_else(|| image.get("base64"))
        .and_then(Value::as_str)?
        .trim();
    if raw_data.is_empty() {
        return None;
    }
    let (data_url_mime, data) = split_data_url(raw_data)?;
    if data.is_empty() || !data.chars().all(is_base64_char) {
        return None;
    }
    let approx_bytes = data.len().saturating_mul(3) / 4;
    let declared_mime = image
        .get("mime_type")
        .or_else(|| image.get("mimeType"))
        .and_then(Value::as_str)
        .and_then(normalize_image_mime);
    let data_mime = data_url_mime.as_deref().and_then(normalize_image_mime);
    if declared_mime.is_some() && data_mime.is_some() && declared_mime != data_mime {
        return None;
    }
    let mime_type = declared_mime.or(data_mime).unwrap_or("image/png");
    let model_data = if approx_bytes > MAX_TOOL_MODEL_IMAGE_BYTES + 2 {
        None
    } else {
        let decoded = STANDARD.decode(data.as_bytes()).ok()?;
        if decoded.is_empty() {
            return None;
        }
        if detect_image_mime(&decoded)? != mime_type {
            return None;
        }
        Some((data.to_string(), decoded.len()))
    };
    let size_bytes = model_data
        .as_ref()
        .map(|(_, size_bytes)| *size_bytes)
        .unwrap_or(approx_bytes);
    Some(ModelImage {
        data: model_data.map(|(data, _)| data),
        mime_type: mime_type.to_string(),
        label: image_label(image),
        size_bytes,
        file_path: clean_optional_string(
            image
                .get("image_path")
                .or_else(|| image.get("file_path"))
                .or_else(|| image.get("path")),
        ),
        resource_path: clean_optional_string(
            image
                .get("image_res_path")
                .or_else(|| image.get("resource_path"))
                .or_else(|| image.get("res_path")),
        ),
        access_token: new_tool_media_token()?,
    })
}

fn new_tool_media_token() -> Option<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).ok()?;
    Some(URL_SAFE_NO_PAD.encode(bytes))
}

fn split_data_url(raw: &str) -> Option<(Option<String>, &str)> {
    if !raw.starts_with("data:") {
        return Some((None, raw));
    }
    let (prefix, data) = raw.split_once(',')?;
    if !prefix.to_ascii_lowercase().contains(";base64") {
        return None;
    }
    let mime = prefix
        .strip_prefix("data:")
        .and_then(|value| value.split(';').next())
        .map(str::to_string);
    Some((mime, data.trim()))
}

fn normalize_image_mime(mime: &str) -> Option<&'static str> {
    match mime.trim().to_ascii_lowercase().as_str() {
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

fn image_label(image: &Value) -> String {
    image
        .get("label")
        .or_else(|| image.get("description"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.chars().take(160).collect())
        .unwrap_or_else(|| "Screenshot from screenshot_scene".to_string())
}

impl ModelImage {
    fn has_model_data(&self) -> bool {
        self.data.as_deref().is_some_and(|data| !data.is_empty())
    }

    fn label_for_tool(&self, tool_name: &str) -> String {
        if self.label.trim().is_empty() {
            return format!("Screenshot from {tool_name}");
        }
        self.label.clone()
    }
}

impl ExecutedTool {
    fn tool_image_metadata(&self, tool_call_id: &str) -> Vec<Value> {
        self.model_images
            .iter()
            .enumerate()
            .filter_map(|(index, image)| tool_image_metadata_entry(tool_call_id, index, image))
            .collect()
    }
}

fn tool_image_metadata(tool_call_id: &str, images: &[ModelImage]) -> Value {
    Value::Array(
        images
            .iter()
            .enumerate()
            .filter_map(|(index, image)| tool_image_metadata_entry(tool_call_id, index, image))
            .collect(),
    )
}

fn tool_image_metadata_entry(
    tool_call_id: &str,
    index: usize,
    image: &ModelImage,
) -> Option<Value> {
    let file_path = image.file_path.as_deref()?.trim();
    if file_path.is_empty() {
        return None;
    }
    let mut entry = json!({
        "mime_type": image.mime_type,
        "name": image.label,
        "size": image.size_bytes,
        "file_path": file_path,
        "token": image.access_token,
        "index": index
    });
    if !tool_call_id.trim().is_empty() {
        entry["tool_call_id"] = json!(tool_call_id);
        entry["url"] = json!(format!(
            "/chat/tool-media/{tool_call_id}/{index}?token={}",
            image.access_token
        ));
    }
    if let Some(resource_path) = image.resource_path.as_deref() {
        entry["resource_path"] = json!(resource_path);
    }
    Some(entry)
}

fn clean_optional_string(value: Option<&Value>) -> Option<String> {
    value
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.chars().take(1024).collect())
}

fn strip_screenshot_image_bytes(mut value: Value) -> Value {
    strip_image_base64_fields(&mut value);
    value
}

fn strip_image_base64_fields(value: &mut Value) {
    match value {
        Value::Object(object) => {
            object.remove("image_base64");
            for child in object.values_mut() {
                strip_image_base64_fields(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                strip_image_base64_fields(child);
            }
        }
        _ => {}
    }
}

fn read_file_image_content_part(image: &Value) -> Option<Value> {
    let image_base64 = image.get("base64").and_then(Value::as_str)?;
    if image_base64.is_empty() {
        return None;
    }
    let mime_type = image
        .get("mime_type")
        .and_then(Value::as_str)
        .filter(|mime| !mime.is_empty())
        .unwrap_or("image/png");
    Some(json!({
        "type": "image_url",
        "image_url": {
            "url": format!("data:{mime_type};base64,{image_base64}")
        }
    }))
}

fn inline_image_markdown(raw_result: &Value, tool_name: &str) -> String {
    if tool_name == "read_file" {
        return read_file_image_markdown(raw_result);
    }
    String::new()
}

fn read_file_image_markdown(raw_result: &Value) -> String {
    raw_result
        .get("files")
        .and_then(Value::as_array)
        .map(|files| {
            files
                .iter()
                .filter_map(|file| {
                    let image = file.get("image")?;
                    let label = file.get("path").and_then(Value::as_str).unwrap_or("Image");
                    read_file_image_markdown_part(image, label)
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        })
        .unwrap_or_default()
}

fn read_file_image_markdown_part(image: &Value, label: &str) -> Option<String> {
    let image_base64 = image.get("base64").and_then(Value::as_str)?;
    if image_base64.is_empty() {
        return None;
    }
    let mime_type = image
        .get("mime_type")
        .and_then(Value::as_str)
        .filter(|mime| !mime.is_empty())
        .unwrap_or("image/png");
    Some(format!(
        "![{label}](data:{mime_type};base64,{image_base64})"
    ))
}

fn normalize_res_path(path: &str) -> String {
    if path.starts_with("res://") || path.is_empty() {
        path.to_string()
    } else {
        format!("res://{}", path.trim_start_matches('/'))
    }
}

fn is_base64_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '+' | '/' | '=')
}

#[cfg(test)]
mod tests {
    use super::*;

    const PNG_1X1: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/p9sAAAAASUVORK5CYII=";

    #[test]
    fn exec_command_is_daemon_local_and_allowed() {
        assert!(is_allowed_tool("exec_command"));
        assert!(is_daemon_local_tool("exec_command"));
        assert!(!is_daemon_local_tool("read_file"));
    }

    #[test]
    fn screenshot_model_images_create_transient_model_context_and_ui_metadata() {
        let images = model_images_from_response(
            "screenshot_scene",
            &json!({
                "model_images": [{
                    "data": PNG_1X1,
                    "mime_type": "image/png",
                    "label": "Scene screenshot",
                    "image_path": "C:/tmp/fennara-shot.png",
                    "image_res_path": "user://.fennara/fennara-shot.png"
                }]
            }),
        );
        assert_eq!(images.len(), 1);

        let result = ExecutedTool {
            ok: true,
            raw_result: json!({}),
            mcp_markdown: "Tool: screenshot_scene\nStatus: success".to_string(),
            plugin_markdown: "Tool: screenshot_scene\nStatus: success".to_string(),
            metadata: json!({}),
            target_keys: Vec::new(),
            model_followup_messages: Vec::new(),
            model_images: images,
        };

        let model_messages = model_messages_for_tool_result("screenshot_scene", &result, true);
        assert_eq!(model_messages.len(), 1);
        assert_eq!(model_messages[0]["content"][1]["type"], "image_url");
        assert!(
            model_messages[0]["content"][1]["image_url"]["url"]
                .as_str()
                .unwrap()
                .contains(PNG_1X1)
        );

        let ui_images = ui_images_for_tool_result("call_1", &result);
        assert_eq!(ui_images.len(), 1);
        let url = ui_images[0]["url"].as_str().unwrap();
        assert!(url.starts_with("/chat/tool-media/call_1/0?token="));
        let token = ui_images[0]["token"].as_str().unwrap();
        assert!(token.len() >= 32);
        assert!(url.ends_with(token));
        assert_eq!(ui_images[0]["file_path"], "C:/tmp/fennara-shot.png");
        assert!(
            !ui_images
                .iter()
                .any(|image| serde_json::to_string(image).unwrap().contains(PNG_1X1))
        );
    }

    #[test]
    fn screenshot_model_images_degrade_to_text_for_non_vision_models() {
        let image = validate_model_image(&json!({
            "data": PNG_1X1,
            "mime_type": "image/png",
            "image_path": "C:/tmp/fennara-shot.png"
        }))
        .unwrap();
        let result = ExecutedTool {
            ok: true,
            raw_result: json!({}),
            mcp_markdown: String::new(),
            plugin_markdown: String::new(),
            metadata: json!({}),
            target_keys: Vec::new(),
            model_followup_messages: Vec::new(),
            model_images: vec![image],
        };

        let model_messages = model_messages_for_tool_result("screenshot_scene", &result, false);
        let serialized = serde_json::to_string(&model_messages).unwrap();
        assert!(serialized.contains("does not support image input"));
        assert!(!serialized.contains("image_url"));
        assert!(!serialized.contains(PNG_1X1));
    }

    #[test]
    fn oversized_screenshot_keeps_ui_metadata_without_model_image_data() {
        let oversized_base64 = "A".repeat(((MAX_TOOL_MODEL_IMAGE_BYTES + 16) * 4 / 3) + 8);
        let images = model_images_from_response(
            "screenshot_scene",
            &json!({
                "model_images": [{
                    "data": oversized_base64,
                    "mime_type": "image/png",
                    "label": "Large screenshot",
                    "image_path": "C:/tmp/large-fennara-shot.png",
                    "image_res_path": "user://.fennara/large-fennara-shot.png"
                }]
            }),
        );
        assert_eq!(images.len(), 1);
        assert!(!images[0].has_model_data());

        let result = ExecutedTool {
            ok: true,
            raw_result: json!({}),
            mcp_markdown: "Tool: screenshot_scene\nStatus: success".to_string(),
            plugin_markdown: "Tool: screenshot_scene\nStatus: success".to_string(),
            metadata: json!({}),
            target_keys: Vec::new(),
            model_followup_messages: Vec::new(),
            model_images: images,
        };

        assert!(model_messages_for_tool_result("screenshot_scene", &result, true).is_empty());
        let ui_images = ui_images_for_tool_result("call_large", &result);
        assert_eq!(ui_images.len(), 1);
        assert_eq!(ui_images[0]["file_path"], "C:/tmp/large-fennara-shot.png");
        assert!(
            ui_images[0]["url"]
                .as_str()
                .unwrap()
                .starts_with("/chat/tool-media/call_large/0?token=")
        );
        assert!(!serde_json::to_string(&ui_images).unwrap().contains("AAAA"));
    }

    #[test]
    fn screenshot_raw_result_strips_bytes_and_is_not_replayed_as_image() {
        let stripped = strip_screenshot_image_bytes(json!({
            "image_base64": PNG_1X1,
            "mime_type": "image/png",
            "image_path": "C:/tmp/fennara-shot.png",
            "images": [{
                "view": "front",
                "image_base64": PNG_1X1,
                "image_path": "C:/tmp/front.png"
            }]
        }));
        let serialized = serde_json::to_string(&stripped).unwrap();
        assert!(!serialized.contains("image_base64"));
        assert!(!serialized.contains(PNG_1X1));
        assert_eq!(stripped["image_path"], "C:/tmp/fennara-shot.png");
        assert_eq!(
            model_followups_for(
                "screenshot_scene",
                &json!({ "image_base64": PNG_1X1, "mime_type": "image/png" }),
            ),
            Vec::<Value>::new()
        );
    }

    #[test]
    fn invalid_screenshot_model_images_are_ignored() {
        assert!(model_images_from_response("screenshot_scene", &json!({})).is_empty());
        assert!(
            model_images_from_response(
                "screenshot_scene",
                &json!({
                    "model_images": [{
                        "data": PNG_1X1,
                        "mime_type": "image/jpeg",
                        "image_path": "C:/tmp/not-a-jpeg.jpg"
                    }]
                }),
            )
            .is_empty()
        );
        assert!(
            model_images_from_response(
                "screenshot_scene",
                &json!({
                    "model_images": [{
                        "data": "not base64",
                        "mime_type": "image/png",
                        "image_path": "C:/tmp/bad.png"
                    }]
                }),
            )
            .is_empty()
        );
    }
}
