use super::{
    daemon_client::{daemon_status, daemon_tool_call},
    protocol::{SERVER_NAME, SERVER_VERSION, error_response, success_response},
    schemas::{is_forwarded_tool, load_embedded_tool_schemas},
};
use serde_json::{Value, json};

const MAX_MCP_SCREENSHOT_IMAGE_BYTES: usize = 8 * 1024 * 1024;

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
    if tool_name != "screenshot_scene" {
        return Vec::new();
    }

    let Some(primary_image) = primary_screenshot_image(response) else {
        return Vec::new();
    };

    match mcp_image_block(primary_image) {
        ImageBlockResult::Block(block) => {
            let label = model_image_label(primary_image);
            vec![json!({ "type": "text", "text": label }), block]
        }
        ImageBlockResult::Omitted(reason) => vec![json!({
            "type": "text",
            "text": format!("[Screenshot image omitted from MCP image context: {reason}]")
        })],
        ImageBlockResult::None => Vec::new(),
    }
}

fn primary_screenshot_image(response: &Value) -> Option<&Value> {
    response
        .get("model_images")
        .and_then(Value::as_array)
        .and_then(|images| images.first())
        .or_else(|| {
            response.get("raw_result").filter(|raw_result| {
                raw_result
                    .get("image_base64")
                    .and_then(Value::as_str)
                    .is_some()
            })
        })
}

enum ImageBlockResult {
    Block(Value),
    Omitted(String),
    None,
}

fn mcp_image_block(image: &Value) -> ImageBlockResult {
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
    if estimated_decoded_bytes(data) > MAX_MCP_SCREENSHOT_IMAGE_BYTES {
        return ImageBlockResult::Omitted(format!(
            "image exceeded {} MB",
            MAX_MCP_SCREENSHOT_IMAGE_BYTES / 1024 / 1024
        ));
    }

    let mime_type = image
        .get("mime_type")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|mime| !mime.is_empty())
        .unwrap_or("image/png");
    if !is_supported_image_mime(mime_type) {
        return ImageBlockResult::Omitted(format!("unsupported MIME type {mime_type}"));
    }

    ImageBlockResult::Block(json!({
        "type": "image",
        "data": data,
        "mimeType": mime_type
    }))
}

fn model_image_label(image: &Value) -> String {
    image
        .get("label")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .map(|label| format!("[{label}]"))
        .unwrap_or_else(|| "[Screenshot image from screenshot_scene]".to_string())
}

fn estimated_decoded_bytes(base64: &str) -> usize {
    base64.trim().len().saturating_mul(3) / 4
}

fn is_base64_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '+' | '/' | '=')
}

fn is_supported_image_mime(mime_type: &str) -> bool {
    matches!(
        mime_type,
        "image/png" | "image/jpeg" | "image/webp" | "image/gif"
    )
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
                    "data": "YWJjMTIz+/=",
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
        assert_eq!(result["content"][2]["data"], "YWJjMTIz+/=");
        assert_eq!(result["content"][2]["mimeType"], "image/png");
        assert!(
            !result["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("YWJjMTIz")
        );
    }

    #[test]
    fn forwarded_screenshot_result_uses_legacy_raw_result_image_content() {
        let response = json!({
            "ok": true,
            "result": "Tool: screenshot_scene\nStatus: success",
            "raw_result": {
                "success": true,
                "image_base64": "bGVnYWN5",
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
        assert_eq!(result["content"][2]["data"], "bGVnYWN5");
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
                    "data": "a".repeat((MAX_MCP_SCREENSHOT_IMAGE_BYTES * 4 / 3) + 16),
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
}
