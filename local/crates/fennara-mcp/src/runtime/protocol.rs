use super::tools;
use serde::Serialize;
use serde_json::{Value, json};

const JSONRPC_VERSION: &str = "2.0";
pub(crate) const MCP_PROTOCOL_VERSION: &str = "2025-06-18";
const SUPPORTED_MCP_PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26"];
pub(crate) const SERVER_NAME: &str = "fennara-mcp";
pub(crate) const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: &'static str,
    id: Value,
    result: Value,
}

#[derive(Serialize)]
struct JsonRpcErrorResponse {
    jsonrpc: &'static str,
    id: Value,
    error: JsonRpcError,
}

#[derive(Serialize)]
struct JsonRpcError {
    code: i64,
    message: String,
}

pub(crate) fn handle_request(request: Value) -> Option<Value> {
    let id = request.get("id").cloned();
    let method = request.get("method").and_then(Value::as_str);

    match method {
        Some("initialize") => {
            id.map(|id| success_response(id, initialize_result(request.get("params"))))
        }
        Some("notifications/initialized") => None,
        Some("tools/list") => id.map(|id| success_response(id, tools::tools_list_result())),
        Some("tools/call") => id.map(|id| tools::handle_tool_call(id, request.get("params"))),
        Some(other) => id.map(|id| error_response(id, -32601, format!("Unknown method: {other}"))),
        None => id.map(|id| error_response(id, -32600, "Missing method".to_string())),
    }
}

pub(crate) fn initialize_result(params: Option<&Value>) -> Value {
    json!({
        "protocolVersion": negotiated_protocol_version(params),
        "capabilities": {
            "tools": {}
        },
        "serverInfo": {
            "name": SERVER_NAME,
            "version": SERVER_VERSION
        }
    })
}

fn negotiated_protocol_version(params: Option<&Value>) -> &'static str {
    let requested_version = params
        .and_then(|params| params.get("protocolVersion"))
        .and_then(Value::as_str);

    match requested_version {
        Some(version) if SUPPORTED_MCP_PROTOCOL_VERSIONS.contains(&version) => {
            SUPPORTED_MCP_PROTOCOL_VERSIONS
                .iter()
                .copied()
                .find(|supported| *supported == version)
                .unwrap_or(MCP_PROTOCOL_VERSION)
        }
        _ => MCP_PROTOCOL_VERSION,
    }
}

pub(crate) fn success_response(id: Value, result: Value) -> Value {
    serde_json::to_value(JsonRpcResponse {
        jsonrpc: JSONRPC_VERSION,
        id,
        result,
    })
    .expect("JSON-RPC success response should serialize")
}

pub(crate) fn error_response(id: Value, code: i64, message: String) -> Value {
    serde_json::to_value(JsonRpcErrorResponse {
        jsonrpc: JSONRPC_VERSION,
        id,
        error: JsonRpcError { code, message },
    })
    .expect("JSON-RPC error response should serialize")
}
