use super::{
    daemon_client::{daemon_status, daemon_tool_call},
    protocol::{SERVER_NAME, SERVER_VERSION, error_response, success_response},
    schemas::{is_forwarded_tool, load_embedded_tool_schemas},
};
use serde_json::{Value, json};

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
            success_response(id, tool_result_with_error(result, is_error))
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
    tool_result_with_error(payload, false)
}

fn tool_result_with_error(payload: Value, is_error: bool) -> Value {
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
