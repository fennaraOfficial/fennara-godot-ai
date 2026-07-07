extends Node
const RUNTIME_SPEC_ENV := "FENNARA_RT_SPEC"
const RuntimeCaptureStore := preload("res://addons/fennara/runtime/runtime_capture_store.gd")
const RuntimeCheckRunner := preload("res://addons/fennara/runtime/runtime_check_runner.gd")
const RuntimeScriptContext := preload("res://addons/fennara/runtime/runtime_script_context.gd")

var _file_session_id := ""
var _file_command_dir := ""
var _file_artifact_dir := ""
var _file_processed_commands := {}
var _runtime_session_closing := false
var _capture_store
var _check_runner


func _ensure_runtime_helpers() -> void:
	if _capture_store == null:
		_capture_store = RuntimeCaptureStore.new(self)
	if _check_runner == null:
		_check_runner = RuntimeCheckRunner.new(self, _capture_store)


func _safe_file_component(value: String, fallback: String) -> String:
	_ensure_runtime_helpers()
	return _capture_store.safe_file_component(value, fallback)

func _ready() -> void:
	if Engine.is_editor_hint():
		return
	_ensure_runtime_helpers()
	var runtime_spec := OS.get_environment(RUNTIME_SPEC_ENV)
	if not runtime_spec.strip_edges().is_empty():
		var request := _read_json_file(runtime_spec)
		if str(request.get("mode", "")) == "runtime_session":
			_run_env_runtime_session.call_deferred(request)
		else:
			_check_runner.call_deferred("run_env_runtime_check", runtime_spec)

func _run_runtime_script(data: Array) -> void:
	_ensure_runtime_helpers()
	var script_run_id := str(data[0]) if data.size() > 0 else ""
	var session_id := str(data[1]) if data.size() > 1 else ""
	var script_path := str(data[2]) if data.size() > 2 else ""
	var status_path := str(data[3]) if data.size() > 3 else ""
	get_tree().root.set_meta("_fennara_runtime_session_id", session_id)
	var ctx := RuntimeScriptContext.new(self, session_id, script_run_id, status_path)
	ctx._write_status("running", "", {"scene_closed": false, "session_active": true})
	ctx._print_event("FENNARA_SCRIPT_STARTED", {"script_path": script_path})

	var script := load(script_path)
	if script == null:
		var load_error := "Could not load runtime script: %s" % script_path
		ctx._print_event("FENNARA_SCRIPT_FAILED", {"error": load_error})
		ctx._write_status("failed", load_error)
		return

	if script is Script and not script.can_instantiate():
		var instantiate_error := "Runtime script could not instantiate, likely due to parse errors: %s" % script_path
		ctx._print_event("FENNARA_SCRIPT_FAILED", {"error": instantiate_error})
		ctx._write_status("failed", instantiate_error)
		return
	var instance = script.new()
	if instance == null or not instance.has_method("run"):
		var contract_error := "Runtime script must instantiate and define run(ctx)."
		ctx._print_event("FENNARA_SCRIPT_FAILED", {"error": contract_error})
		ctx._write_status("failed", contract_error)
		return

	await instance.call("run", ctx)
	if ctx.has_close_requested():
		ctx._print_event("FENNARA_SCRIPT_COMPLETED", {})
	else:
		ctx._release_all_inputs()
		ctx._print_event("FENNARA_SCRIPT_COMPLETED", {"scene_closed": false})
		ctx._write_status("completed", "", {"scene_closed": false, "session_active": true})

func _finish_runtime_script_session(ctx) -> void:
	_runtime_session_closing = true
	ctx._release_all_inputs()
	await get_tree().process_frame
	await get_tree().process_frame
	if get_tree().current_scene != null:
		get_tree().current_scene.queue_free()
	await get_tree().process_frame
	await get_tree().process_frame
	get_tree().quit(0)

func _run_env_runtime_session(request: Dictionary) -> void:
	_ensure_runtime_helpers()
	_runtime_session_closing = false
	_file_session_id = str(request.get("session_id", ""))
	_file_command_dir = str(request.get("command_dir", ""))
	_file_artifact_dir = str(request.get("artifact_dir", ""))
	if _file_session_id.strip_edges().is_empty() or _file_command_dir.strip_edges().is_empty():
		return

	DirAccess.make_dir_recursive_absolute(_file_command_dir)
	if not _file_artifact_dir.strip_edges().is_empty():
		DirAccess.make_dir_recursive_absolute(_file_artifact_dir)

	var scene_frame: Dictionary = await _check_runner.wait_for_env_runtime_scene_frame()
	await _check_runner.raise_runtime_window_once()
	print("FENNARA_RUNTIME_SESSION_READY: %s" % JSON.stringify({
		"session_id": _file_session_id,
		"scene_frame": scene_frame,
		"scene_path": str(request.get("scene_path", "")),
		"time_ms": Time.get_ticks_msec(),
	}))
	_print_runtime_orientation("startup")
	var startup_capture: Dictionary = {}
	if not _file_artifact_dir.strip_edges().is_empty():
		startup_capture = await _capture_store.capture_runtime_session_start(
			_file_artifact_dir,
			_file_session_id,
			str(request.get("scene_path", "")),
			int(request.get("startup_capture_max_resolution", 1280))
		)
		_capture_store.write_env_runtime_status(
			str(request.get("startup_capture_status_path", "")),
			startup_capture
		)
		print("FENNARA_RUNTIME_SESSION_STARTUP_CAPTURE: %s" % JSON.stringify(startup_capture))

	while is_inside_tree() and not _runtime_session_closing:
		_poll_runtime_session_commands()
		for _i in range(6):
			if _runtime_session_closing:
				break
			await get_tree().process_frame

func _poll_runtime_session_commands() -> void:
	var dir := DirAccess.open(_file_command_dir)
	if dir == null:
		return
	dir.list_dir_begin()
	while true:
		var file_name := dir.get_next()
		if file_name.is_empty():
			break
		if dir.current_is_dir() or not file_name.ends_with(".json"):
			continue
		if _file_processed_commands.has(file_name):
			continue
		var command_path := _file_command_dir.path_join(file_name)
		var command := _read_json_file(command_path)
		if command.is_empty():
			continue
		_file_processed_commands[file_name] = true
		if str(command.get("action", "")) == "run_runtime_script":
			_run_runtime_script.call_deferred([
				str(command.get("script_run_id", "")),
				str(command.get("session_id", _file_session_id)),
				str(command.get("script_path", "")),
				str(command.get("status_path", "")),
			])
	dir.list_dir_end()

func _capture_runtime_script(ctx, label: String, max_resolution: int = 1280) -> Dictionary:
	_ensure_runtime_helpers()
	return await _capture_store.capture_runtime_script(ctx, label, max_resolution)

func _print_runtime_orientation(reason: String = "startup") -> void:
	var tree := get_tree()
	var root := tree.current_scene
	var viewport := root.get_viewport() if root != null and root.is_inside_tree() else tree.root
	var viewport_size := viewport.get_visible_rect().size if viewport != null else Vector2.ZERO
	var root_name := str(root.name) if root != null else "<none>"
	var root_class := root.get_class() if root != null else "<none>"
	var root_path := str(root.get_path()) if root != null else ""
	var scene_path := str(root.scene_file_path) if root != null else ""

	print("FENNARA_RUNTIME_ORIENTATION: reason=%s scene=%s root=%s(%s) root_path=%s viewport=%dx%d paused=%s physics_fps=%d" % [
		_compact_value(reason),
		_compact_value(scene_path),
		_compact_value(root_name),
		_compact_value(root_class),
		_compact_value(root_path),
		int(viewport_size.x),
		int(viewport_size.y),
		str(tree.paused).to_lower(),
		Engine.physics_ticks_per_second,
	])
	print("FENNARA_RUNTIME_ORIENTATION_TOP_NODES: %s" % _orientation_top_nodes(root))
	print("FENNARA_RUNTIME_ORIENTATION_GROUPS: %s" % _orientation_groups(root))
	print("FENNARA_RUNTIME_ORIENTATION_INPUT_PROJECT: %s" % _orientation_input_actions(false))
	print("FENNARA_RUNTIME_ORIENTATION_INPUT_UI: %s" % _orientation_input_actions(true))
	print("FENNARA_RUNTIME_ORIENTATION_AUTOLOADS: %s" % _orientation_autoloads())
	print("FENNARA_RUNTIME_ORIENTATION_NOTE: startup/restart live inventory only; spawned/despawned nodes may differ later")

func _orientation_top_nodes(root: Node) -> String:
	if root == null:
		return "<no current_scene>"
	var items: Array[String] = []
	var children := root.get_children()
	var limit: int = mini(children.size(), 40)
	for i in range(limit):
		var child: Node = children[i] as Node
		if child is Node:
			items.append(_orientation_node_item(root, child))
	if children.size() > limit:
		items.append("omitted=%d" % (children.size() - limit))
	return _join_or_none(items)

func _orientation_node_item(root: Node, node: Node) -> String:
	var parts: Array[String] = []
	parts.append("%s(%s" % [_compact_value(str(root.get_path_to(node))), _compact_value(node.get_class())])
	var script_path: String = _script_path(node)
	if not script_path.is_empty():
		parts.append("script=%s" % _compact_value(script_path))
	var groups: Array[String] = _node_groups(node)
	if not groups.is_empty():
		parts.append("groups=%s" % ",".join(groups))
	parts.append("children=%d" % node.get_child_count())
	return " ".join(parts) + ")"

func _orientation_groups(scene_root: Node) -> String:
	var group_names: Array[String] = []
	var discovered: Dictionary = {}
	for node in get_tree().root.find_children("*", "", true, false):
		if node is Node:
			for group in (node as Node).get_groups():
				var group_name := str(group)
				if group_name.begins_with("_"):
					continue
				discovered[group_name] = true
	for group_name in discovered.keys():
		group_names.append(str(group_name))
	group_names.sort()

	var items: Array[String] = []
	for group_name in group_names:
		var nodes: Array[Node] = get_tree().get_nodes_in_group(group_name)
		var samples: Array[String] = []
		var sample_count: int = mini(nodes.size(), 3)
		for i in range(sample_count):
			if nodes[i] is Node:
				samples.append(_orientation_node_path(scene_root, nodes[i]))
		var item := "%s=%d" % [_compact_value(group_name), nodes.size()]
		if not samples.is_empty():
			item += " samples=%s" % ",".join(samples)
		items.append(item)
	return _join_or_none(items)

func _orientation_input_actions(ui_actions: bool) -> String:
	var action_names: Array[String] = []
	for action in InputMap.get_actions():
		var action_name := str(action)
		if action_name.begins_with("ui_") == ui_actions:
			action_names.append(action_name)
	action_names.sort()

	var items: Array[String] = []
	var limit: int = mini(action_names.size(), 120)
	for i in range(limit):
		var action_name := action_names[i]
		items.append("%s=%s" % [_compact_value(action_name), _input_action_summary(action_name)])
	if action_names.size() > limit:
		items.append("omitted=%d" % (action_names.size() - limit))
	return _join_or_none(items)

func _orientation_autoloads() -> String:
	var items: Array[String] = []
	var names: Array[String] = []
	for property_info in ProjectSettings.get_property_list():
		if property_info is Dictionary:
			var property_name := str(property_info.get("name", ""))
			if property_name.begins_with("autoload/"):
				names.append(property_name.trim_prefix("autoload/"))
	names.sort()
	var limit: int = mini(names.size(), 40)
	for i in range(limit):
		var name := names[i]
		var value := str(ProjectSettings.get_setting("autoload/%s" % name, ""))
		items.append("%s=%s" % [_compact_value(name), _compact_value(value)])
	if names.size() > limit:
		items.append("omitted=%d" % (names.size() - limit))
	return _join_or_none(items)

func _input_action_summary(action_name: String) -> String:
	var event_parts: Array[String] = []
	for event in InputMap.action_get_events(action_name):
		event_parts.append(_input_event_summary(event))
	return "None" if event_parts.is_empty() else "/".join(event_parts)

func _input_event_summary(event: InputEvent) -> String:
	if event is InputEventKey:
		var key_event := event as InputEventKey
		var code := key_event.physical_keycode if key_event.physical_keycode != 0 else key_event.keycode
		var key_name := OS.get_keycode_string(code)
		return "Key(%s)" % _compact_value(key_name if not key_name.is_empty() else str(code))
	if event is InputEventMouseButton:
		return "MouseButton(%s)" % _mouse_button_name((event as InputEventMouseButton).button_index)
	if event is InputEventJoypadButton:
		return "JoyButton(%d)" % (event as InputEventJoypadButton).button_index
	if event is InputEventJoypadMotion:
		return "JoyAxis(%d)" % (event as InputEventJoypadMotion).axis
	if event is InputEventScreenTouch:
		return "Touch"
	if event is InputEventScreenDrag:
		return "Drag"
	if event is InputEventMouseMotion:
		return "MouseMotion"
	return event.get_class().trim_prefix("InputEvent")

func _mouse_button_name(button: int) -> String:
	match button:
		MOUSE_BUTTON_LEFT:
			return "Left"
		MOUSE_BUTTON_RIGHT:
			return "Right"
		MOUSE_BUTTON_MIDDLE:
			return "Middle"
		MOUSE_BUTTON_WHEEL_UP:
			return "WheelUp"
		MOUSE_BUTTON_WHEEL_DOWN:
			return "WheelDown"
		_:
			return str(button)

func _script_path(node: Node) -> String:
	var script: Script = node.get_script() as Script
	if script != null:
		return str(script.resource_path)
	return ""

func _node_groups(node: Node) -> Array[String]:
	var result: Array[String] = []
	for group in node.get_groups():
		var group_name := str(group)
		if not group_name.begins_with("_"):
			result.append(_compact_value(group_name))
	result.sort()
	return result

func _orientation_node_path(scene_root: Node, node: Node) -> String:
	if scene_root != null and (node == scene_root or scene_root.is_ancestor_of(node)):
		return _compact_value("." if node == scene_root else str(scene_root.get_path_to(node)))
	return _compact_value(str(node.get_path()))

func _compact_value(value: String) -> String:
	var compact := value.strip_edges().replace(" ", "_")
	if compact.is_empty():
		return "-"
	return compact

func _join_or_none(items: Array[String]) -> String:
	return "none" if items.is_empty() else "; ".join(items)

func _read_json_file(path: String) -> Dictionary:
	_ensure_runtime_helpers()
	return _capture_store.read_json_file(path)

func _write_runtime_script_status(status_path: String, session_id: String, script_run_id: String, status: String, captures: Array[Dictionary], error: String = "", extra: Dictionary = {}) -> void:
	_ensure_runtime_helpers()
	_capture_store.write_runtime_script_status(status_path, session_id, script_run_id, status, captures, error, extra)
