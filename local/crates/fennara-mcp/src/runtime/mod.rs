mod daemon_client;
mod protocol;
mod schemas;
mod tools;

use serde_json::Value;
use std::io::{self, BufRead, Write};

pub(crate) fn run_stdio() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let Ok(line) = line else {
            continue;
        };

        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<Value>(&line) {
            Ok(request) => protocol::handle_request(request),
            Err(error) => Some(protocol::error_response(
                Value::Null,
                -32700,
                format!("Parse error: {error}"),
            )),
        };

        if let Some(response) = response {
            if writeln!(stdout, "{response}").is_err() {
                break;
            }
            let _ = stdout.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{protocol, schemas, tools};
    use serde_json::{Value, json};

    fn listed_tool_names() -> Vec<String> {
        tools::tools_list_result()["tools"]
            .as_array()
            .expect("tools/list should return a tools array")
            .iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str))
            .map(ToOwned::to_owned)
            .collect()
    }

    fn initialize_request(protocol_version: &str) -> Value {
        json!({
            "protocolVersion": protocol_version,
            "capabilities": {},
            "clientInfo": {
                "name": "test-client",
                "version": "0.0.0"
            }
        })
    }

    #[test]
    fn initialize_negotiates_2025_06_18_when_requested() {
        let params = initialize_request("2025-06-18");
        let result = protocol::initialize_result(Some(&params));

        assert_eq!(result["protocolVersion"], "2025-06-18");
    }

    #[test]
    fn initialize_negotiates_2025_03_26_when_requested() {
        let params = initialize_request("2025-03-26");
        let result = protocol::initialize_result(Some(&params));

        assert_eq!(result["protocolVersion"], "2025-03-26");
    }

    #[test]
    fn initialize_falls_back_to_latest_supported_protocol() {
        let params = initialize_request("2024-11-05");
        let result = protocol::initialize_result(Some(&params));

        assert_eq!(result["protocolVersion"], protocol::MCP_PROTOCOL_VERSION);
    }

    #[test]
    fn tools_list_includes_expected_forwarded_tools() {
        let tool_names = listed_tool_names();

        assert!(tool_names.iter().any(|name| name == "fennara_status"));

        for name in schemas::FORWARDED_TOOLS {
            assert!(
                tool_names.iter().any(|tool_name| tool_name == name),
                "expected tools/list to include {name}"
            );
        }
    }

    #[test]
    fn tools_list_does_not_include_git() {
        let tool_names = listed_tool_names();

        assert!(
            !tool_names.iter().any(|name| name == "git"),
            "tools/list should not expose git"
        );
    }

    #[test]
    fn run_scene_edit_script_description_is_flattened_for_mcp_clients() {
        let tool = schemas::tool_from_embedded_definition(include_str!(
            "../../../../schemas/tools/run_scene_edit_script.json"
        ))
        .expect("run_scene_edit_script definition should parse");

        let description = tool
            .get("description")
            .and_then(Value::as_str)
            .expect("run_scene_edit_script should expose description");

        assert!(description.contains("Run a one-off scene edit worker script"));
        assert!(description.contains("script_path"));
        assert!(description.contains("ctx.get_scene_root()"));
        assert!(tool.get("description_lines").is_none());
    }

    #[test]
    fn tools_list_uses_embedded_schemas_without_remote_lookup() {
        let tool_names = listed_tool_names();

        assert!(
            tool_names
                .iter()
                .any(|name| name == "run_scene_edit_script")
        );
        assert!(tool_names.iter().any(|name| name == "project_settings"));
    }

    #[test]
    fn tools_list_schemas_are_openai_function_compatible_at_top_level() {
        let tools = tools::tools_list_result()["tools"]
            .as_array()
            .expect("tools/list should return a tools array")
            .clone();
        let unsupported_top_level_keys = ["oneOf", "anyOf", "allOf", "enum", "not"];

        for tool in tools {
            let name = tool
                .get("name")
                .and_then(Value::as_str)
                .expect("tool should have a name");
            let schema = tool
                .get("inputSchema")
                .expect("tool should have an inputSchema");

            assert_eq!(
                schema.get("type").and_then(Value::as_str),
                Some("object"),
                "{name} inputSchema must be a top-level object"
            );

            for key in unsupported_top_level_keys {
                assert!(
                    schema.get(key).is_none(),
                    "{name} inputSchema must not use top-level {key}"
                );
            }
        }
    }
}
