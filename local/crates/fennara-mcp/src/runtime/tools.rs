use super::{
    daemon_client::{daemon_status, daemon_tool_call},
    protocol::{SERVER_NAME, SERVER_VERSION, error_response, success_response},
    schemas::{is_forwarded_tool, load_embedded_tool_schemas},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};

const MAX_MCP_TOOL_IMAGE_COUNT: usize = 6;
const MAX_MCP_TOOL_IMAGE_BYTES: usize = 8 * 1024 * 1024;
const MAX_MCP_TOOL_IMAGE_TOTAL_BYTES: usize = 24 * 1024 * 1024;

pub(crate) fn tools_list_result() -> Value {
    let mut tools = vec![json!({
        "name": "fennara_status",
        "description": "Return local Fennara MCP status. This verifies the MCP server is installed and reachable.",
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

pub(crate) fn handle_tool_call(id: Value, params: Option<&Value>) -> Value {
    let tool_name = params
        .and_then(|params| params.get("name"))
        .and_then(Value::as_str);

    match tool_name {
        Some("fennara_status") => success_response(id, tool_result(status_payload())),
        Some(name) if is_forwarded_tool(name) => {
            let args = params
                .and_then(|params| params.get("arguments"))
                .cloned()
                .unwrap_or_else(|| json!({}));
            let result = match daemon_tool_call(name, args) {
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

fn status_payload() -> Value {
    match daemon_status() {
        Ok(status) => json!({
            "ok": true,
            "server": SERVER_NAME,
            "version": SERVER_VERSION,
            "daemon_connected": true,
            "daemon": status
        }),
        Err(error) => json!({
            "ok": true,
            "server": SERVER_NAME,
            "version": SERVER_VERSION,
            "daemon_connected": false,
            "godot_plugin_connected": false,
            "message": format!("Open a Godot project with Fennara enabled. The local daemon is not reachable yet: {error}")
        }),
    }
}

fn tool_result(payload: Value) -> Value {
    json_tool_result_with_error(payload, false)
}

fn json_tool_result_with_error(payload: Value, is_error: bool) -> Value {
    json!({
        "content": [
            {
                "type": "text",
                "text": payload.to_string()
            }
        ],
        "structuredContent": payload,
        "isError": is_error
    })
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
        return format!("Tool: {tool_name}\nStatus: failed\nError: {error}");
    }

    format!("Tool: {tool_name}\nStatus: failed\nError: Tool returned an unsupported result shape.")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PNG_1X1: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/p9sAAAAASUVORK5CYII=";

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
