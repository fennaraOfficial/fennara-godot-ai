use serde_json::{Value, json};

const TOOL_GUIDANCE: &str = "Fennara MCP usage rule: for Godot work, use Fennara MCP tools instead of relying only on raw files or memory. Tools require the Godot project to be open in the Fennara plugin. Use `fennara_status` when connection or MCP target routing is uncertain. Inspect first, edit second, validate third. Do not guess Godot APIs or node paths: use `get_class_info` for native Godot API details, `get_scene_tree` for scene node paths, and `get_node_properties` for existing node/resource configuration. Use `runtime_session` and `runtime_script` for live runtime inspection/control, and treat the returned `runtime_session.log` as the source of truth for scene startup output, runtime errors, script logs, and captures. Use `scrape_editor` with `target: \"debugger\"` only when the user says they manually ran a scene in Godot/the editor and got a runtime error, or explicitly asks what the editor debugger currently shows; do not use it for scenes you started with `runtime_session`. Use `run_scene_edit_script` for procedural scene/resource edits; it automatically runs GDScript diagnostics and scene validation after edits. When creating Godot nodes, always assign explicit meaningful names and avoid auto names like `@Label@21109` unless there is a very strong reason. Use `write_or_update_file` for project file edits; it automatically runs script diagnostics for `.gd` and `.cs` files. `script_diagnostics` supports targeted `.gd` and `.cs` checks and project scans over `.gd` and `.cs` files. Use `screenshot_scene` when visual correctness matters. Fennara MCP intentionally focuses on Godot-aware tools; use the MCP client's own file/search tools for ordinary file reading.";

const EMBEDDED_TOOL_DEFINITIONS: &[&str] = &[
    include_str!("../../../../schemas/tools/write_or_update_file.json"),
    include_str!("../../../../schemas/tools/run_scene_edit_script.json"),
    include_str!("../../../../schemas/tools/get_scene_tree.json"),
    include_str!("../../../../schemas/tools/save_custom_resource.json"),
    include_str!("../../../../schemas/tools/script_diagnostics.json"),
    include_str!("../../../../schemas/tools/screenshot_scene.json"),
    include_str!("../../../../schemas/tools/get_node_properties.json"),
    include_str!("../../../../schemas/tools/get_class_info.json"),
    include_str!("../../../../schemas/tools/validate_scene.json"),
    include_str!("../../../../schemas/tools/project_settings.json"),
    include_str!("../../../../schemas/tools/runtime_session.json"),
    include_str!("../../../../schemas/tools/runtime_script.json"),
    include_str!("../../../../schemas/tools/scrape_editor.json"),
];

pub(crate) const FORWARDED_TOOLS: &[&str] = &[
    "write_or_update_file",
    "run_scene_edit_script",
    "get_scene_tree",
    "save_custom_resource",
    "script_diagnostics",
    "screenshot_scene",
    "get_node_properties",
    "get_class_info",
    "validate_scene",
    "project_settings",
    "runtime_session",
    "runtime_script",
    "scrape_editor",
];

pub(crate) fn load_embedded_tool_schemas() -> Vec<Value> {
    let mut tools = Vec::new();

    for definition in EMBEDDED_TOOL_DEFINITIONS {
        match tool_from_embedded_definition(definition) {
            Ok(tool) => tools.push(tool),
            Err(error) => tools.push(json!({
                "name": "invalid_embedded_tool_definition",
                "description": format!("Failed to load embedded tool definition: {error}"),
                "inputSchema": {
                    "type": "object",
                    "properties": {},
                    "additionalProperties": false
                }
            })),
        }
    }

    tools
}

pub(crate) fn tool_from_embedded_definition(definition: &str) -> Result<Value, String> {
    let mut tool: Value = serde_json::from_str(definition).map_err(|error| error.to_string())?;
    let object = tool
        .as_object_mut()
        .ok_or_else(|| "tool definition is not a JSON object".to_string())?;
    if !object.contains_key("description") {
        if let Some(description_lines) = object.remove("description_lines") {
            let lines = description_lines
                .as_array()
                .ok_or_else(|| "description_lines must be an array of strings".to_string())?;
            let joined = lines
                .iter()
                .map(|line| {
                    line.as_str()
                        .ok_or_else(|| "description_lines must contain only strings".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?
                .join("\n");
            object.insert("description".to_string(), Value::String(joined));
        }
    }
    if let Some(description) = object.get("description").and_then(Value::as_str) {
        object.insert(
            "description".to_string(),
            Value::String(format!("{description}\n\n{TOOL_GUIDANCE}")),
        );
    }
    let parameters = object
        .remove("parameters")
        .unwrap_or_else(|| json!({ "type": "object", "properties": {} }));
    object.insert("inputSchema".to_string(), parameters);
    Ok(tool)
}

pub(crate) fn is_forwarded_tool(name: &str) -> bool {
    FORWARDED_TOOLS.contains(&name)
}
