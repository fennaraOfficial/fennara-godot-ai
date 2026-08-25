use super::schemas;
use serde_json::Value;

#[test]
fn runtime_session_schema_exposes_string_user_args() {
    let tools = schemas::load_embedded_tool_schemas();
    let runtime_session = tools
        .iter()
        .find(|tool| tool.get("name").and_then(Value::as_str) == Some("runtime_session"))
        .expect("runtime_session schema should be embedded");
    let user_args = &runtime_session["inputSchema"]["properties"]["user_args"];

    assert_eq!(user_args["type"], "array");
    assert_eq!(user_args["items"]["type"], "string");
}
