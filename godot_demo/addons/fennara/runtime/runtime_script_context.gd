extends RefCounted

const RuntimeInputDriver := preload("res://addons/fennara/runtime/runtime_input_driver.gd")
const RuntimeNodeSnapshot := preload("res://addons/fennara/runtime/runtime_node_snapshot.gd")
const RuntimePhysicsQuery := preload("res://addons/fennara/runtime/runtime_physics_query.gd")

var _helper: Node
var _input_driver
var _node_snapshot
var _physics_query
var _session_id: String
var _script_run_id: String
var _status_path: String
var _captures_dir: String
var _captures: Array[Dictionary] = []
var _close_requested := false
var _pressed_actions: Array[String] = []


func _init(helper: Node, session_id: String, script_run_id: String, status_path: String = "") -> void:
	_helper = helper
	_input_driver = RuntimeInputDriver.new(helper)
	_node_snapshot = RuntimeNodeSnapshot.new(helper, Callable(self, "get_scene_root"))
	_physics_query = RuntimePhysicsQuery.new(helper, Callable(self, "get_scene_root"))
	_session_id = session_id
	_script_run_id = script_run_id

	var artifact_dir: String = _helper._file_artifact_dir
	if artifact_dir.strip_edges().is_empty():
		artifact_dir = ProjectSettings.globalize_path("user://.fennara/runtime_sessions/%s" % _helper._safe_file_component(session_id, "runtime"))

	_status_path = status_path
	if _status_path.strip_edges().is_empty():
		_status_path = artifact_dir.path_join("runtime_script_results").path_join("%s.json" % _helper._safe_file_component(script_run_id, "script"))
	_captures_dir = artifact_dir.path_join("captures")


func log(message: String, data: Dictionary = {}) -> void:
	var event := data.duplicate(true)
	event["message"] = message
	_print_event("FENNARA_SCRIPT_LOG", event)


func error(message: String) -> void:
	_print_event("FENNARA_SCRIPT_ERROR", {"message": message})


func close_scene() -> void:
	if _close_requested:
		return
	_close_requested = true
	_print_event("FENNARA_SCRIPT_CLOSE_REQUESTED", {})
	_write_status("completed", "", {"scene_closed": true, "session_active": false})
	_helper._finish_runtime_script_session.call_deferred(self)


func restart_scene(options: Dictionary = {}) -> Dictionary:
	_release_all_inputs()

	var tree := _helper.get_tree()
	if tree == null:
		var tree_error := "Runtime helper has no SceneTree."
		error(tree_error)
		return {"success": false, "error": tree_error}

	if tree.current_scene == null:
		var scene_error := "No current scene is available to restart."
		error(scene_error)
		return {"success": false, "error": scene_error}

	var previous_scene_path := str(tree.current_scene.scene_file_path)
	var err := tree.reload_current_scene()
	if err != OK:
		var reload_error := "Failed to reload current scene: %s" % error_string(err)
		error(reload_error)
		return {"success": false, "error": reload_error, "error_code": err, "scene_path": previous_scene_path}

	await tree.scene_changed

	var process_frames: int = maxi(0, int(options.get("process_frames", 1)))
	for _process_index in range(process_frames):
		await tree.process_frame

	var physics_frames: int = maxi(0, int(options.get("physics_frames", 1)))
	for _physics_index in range(physics_frames):
		await tree.physics_frame

	var extra_wait := maxf(0.0, float(options.get("wait", 0.0)))
	if extra_wait > 0.0:
		await wait(extra_wait)

	if _helper != null and _helper.has_method("_print_runtime_orientation"):
		_helper.call("_print_runtime_orientation", "restart_scene")

	_release_all_inputs()

	var current_scene := tree.current_scene
	return {
		"success": current_scene != null,
		"scene_path": str(current_scene.scene_file_path) if current_scene != null else previous_scene_path,
		"scene_name": str(current_scene.name) if current_scene != null else "",
		"process_frames": process_frames,
		"physics_frames": physics_frames,
		"wait": extra_wait,
	}


func has_close_requested() -> bool:
	return _close_requested


func wait(seconds: float) -> void:
	await _helper.get_tree().create_timer(maxf(0.0, seconds)).timeout


func capture(label: String, max_resolution: int = 1280) -> Dictionary:
	var result: Dictionary = await _helper._capture_runtime_script(self, label, max_resolution)
	if result.get("success", false):
		_captures.append(result)
	return result


func press_action(action: String, strength: float = 1.0) -> bool:
	if not InputMap.has_action(action):
		error("Input action does not exist: %s" % action)
		return false
	Input.action_press(action, clampf(strength, 0.0, 1.0))
	if not _pressed_actions.has(action):
		_pressed_actions.append(action)
	return true


func release_action(action: String) -> bool:
	if not InputMap.has_action(action):
		error("Input action does not exist: %s" % action)
		return false
	Input.action_release(action)
	_pressed_actions.erase(action)
	return true


func tap_action(action: String, duration: float = 0.1, strength: float = 1.0) -> bool:
	if not press_action(action, strength):
		return false
	await wait(duration)
	return release_action(action)


func action(action_name: String, phase_or_duration: Variant = "tap", duration: float = 0.1, strength: float = 1.0) -> bool:
	var actions: Array[String] = [action_name]
	return await _apply_action_phase(actions, phase_or_duration, duration, strength)


func action_sequence(steps: Array) -> Dictionary:
	for i in range(steps.size()):
		var step = steps[i]
		if not step is Dictionary:
			var type_error := "Action sequence step %d must be a Dictionary." % i
			error(type_error)
			return {"success": false, "step_index": i, "error": type_error}

		var result: Dictionary = await _run_action_sequence_step(step)
		if not result.get("success", false):
			result["step_index"] = i
			return result

	self.log("action sequence completed", {"steps": steps.size()})
	return {"success": true, "steps": steps.size()}


func node(node_or_path: Variant) -> Node:
	return _node_snapshot.node(node_or_path)


func exists(node_or_path: Variant) -> bool:
	return _node_snapshot.exists(node_or_path)


func snapshot(spec: Dictionary) -> Dictionary:
	return _node_snapshot.snapshot(spec)


func until(predicate: Callable, options: Dictionary = {}) -> Dictionary:
	if not predicate.is_valid():
		var predicate_error := "until predicate is not a valid Callable."
		error(predicate_error)
		return {"success": false, "timeout": false, "attempts": 0, "elapsed": 0.0, "error": predicate_error}
	return await _until_condition(predicate, options)


func until_exists(node_or_path: Variant, options: Dictionary = {}) -> Dictionary:
	var result: Dictionary = await until(func() -> bool: return exists(node_or_path), options)
	result["path"] = str(node_or_path)
	result["exists"] = exists(node_or_path)
	return result


func until_missing(node_or_path: Variant, options: Dictionary = {}) -> Dictionary:
	var result: Dictionary = await until(func() -> bool: return not exists(node_or_path), options)
	result["path"] = str(node_or_path)
	result["exists"] = exists(node_or_path)
	return result


func until_prop(node_or_path: Variant, property_name: String, expected: Variant, options: Dictionary = {}) -> Dictionary:
	var timeout := maxf(0.0, float(options.get("timeout", 1.0)))
	var interval := maxf(0.0, float(options.get("interval", 0.05)))
	var start_ms := Time.get_ticks_msec()
	var attempts := 0
	var last_value: Variant = null
	var saw_node := false
	var saw_property := false

	while true:
		attempts += 1
		var target := node(node_or_path)
		saw_node = target != null
		if target != null and _object_has_property(target, property_name):
			saw_property = true
			last_value = target.get(property_name)
			if _values_match(last_value, expected, options):
				var success_result := _until_receipt(true, false, start_ms, attempts)
				success_result.merge({
					"path": str(node_or_path),
					"property": property_name,
					"expected": expected,
					"last_value": last_value,
					"exists": true,
					"property_found": true,
				}, true)
				return success_result

		var elapsed := float(Time.get_ticks_msec() - start_ms) / 1000.0
		if elapsed >= timeout:
			var timeout_result := _until_receipt(false, true, start_ms, attempts)
			timeout_result.merge({
				"path": str(node_or_path),
				"property": property_name,
				"expected": expected,
				"last_value": last_value,
				"exists": saw_node,
				"property_found": saw_property,
			}, true)
			return timeout_result

		await _wait_until_next_poll(interval, timeout, elapsed)

	return _until_receipt(false, true, start_ms, attempts)


func raycast_3d(from: Variant, to: Variant, options: Dictionary = {}) -> Dictionary:
	var result: Dictionary = _physics_query.raycast_3d(from, to, options)
	if not result.get("success", false):
		error(str(result.get("error", "raycast_3d failed.")))
	return result


func raycast_2d(from: Variant, to: Variant, options: Dictionary = {}) -> Dictionary:
	var result: Dictionary = _physics_query.raycast_2d(from, to, options)
	if not result.get("success", false):
		error(str(result.get("error", "raycast_2d failed.")))
	return result


func raycast_2d_scan(options: Dictionary = {}) -> Dictionary:
	var result: Dictionary = _physics_query.raycast_2d_scan(options)
	if not result.get("success", false):
		error(str(result.get("error", "raycast_2d_scan failed.")))
	return result


func raycast_3d_scan(options: Dictionary = {}) -> Dictionary:
	var result: Dictionary = _physics_query.raycast_3d_scan(options)
	if not result.get("success", false):
		error(str(result.get("error", "raycast_3d_scan failed.")))
	return result


func input_event(event_class: String, properties: Dictionary = {}) -> Dictionary:
	var result: Dictionary = _input_driver.input_event(event_class, properties)
	if not result.get("success", false):
		error(str(result.get("error", "Input event failed.")))
	return result


func press_key(keycode: int, options: Dictionary = {}) -> bool:
	return _input_driver.press_key(keycode, options)


func release_key(keycode: int, options: Dictionary = {}) -> bool:
	return _input_driver.release_key(keycode, options)


func tap_key(keycode: int, options: Dictionary = {}) -> bool:
	if not press_key(keycode, options):
		return false
	await wait(float(options.get("duration", 0.05)))
	return release_key(keycode, options)


func hold_key(keycode: int, duration: float, options: Dictionary = {}) -> bool:
	if not press_key(keycode, options):
		return false
	await wait(duration)
	return release_key(keycode, options)


func press_mouse(button: int, options: Dictionary = {}) -> bool:
	return _input_driver.press_mouse(button, options)


func release_mouse(button: int, options: Dictionary = {}) -> bool:
	return _input_driver.release_mouse(button, options)


func tap_mouse(button: int, options: Dictionary = {}) -> bool:
	var moved := await _move_mouse_for_options(options)
	if not moved and options.has("position"):
		return false
	if not press_mouse(button, options):
		return false
	await wait(float(options.get("duration", 0.05)))
	return release_mouse(button, options)


func hold_mouse(button: int, duration: float, options: Dictionary = {}) -> bool:
	var moved := await _move_mouse_for_options(options)
	if not moved and options.has("position"):
		return false
	if not press_mouse(button, options):
		return false
	await wait(duration)
	return release_mouse(button, options)


func set_mouse_position(position: Variant) -> bool:
	var point_value: Variant = _coerce_vector2(position)
	if point_value == null:
		error("Mouse position must be a Vector2 or Dictionary with x/y.")
		return false
	var point: Vector2 = point_value
	_input_driver.mouse_motion_to(point)
	return true


func world_to_viewport_2d(world_position: Variant, reference_node_or_path: Variant = null) -> Variant:
	var world_value: Variant = _coerce_vector2(world_position)
	if world_value == null:
		error("World 2D position must be a Vector2 or Dictionary/Array with x/y.")
		return null
	var viewport: Viewport = _viewport_for_reference(reference_node_or_path)
	if viewport == null:
		error("No viewport is available for world_to_viewport_2d.")
		return null
	var world: Vector2 = world_value
	return viewport.get_canvas_transform() * world


func set_mouse_world_2d(world_position: Variant, reference_node_or_path: Variant = null) -> bool:
	var viewport_position: Variant = world_to_viewport_2d(world_position, reference_node_or_path)
	if viewport_position == null:
		return false
	return set_mouse_position(viewport_position)


func move_mouse_to(position: Variant, options: Dictionary = {}) -> bool:
	var point_value: Variant = _coerce_vector2(position)
	if point_value == null:
		error("Mouse position must be a Vector2 or Dictionary with x/y.")
		return false
	var point: Vector2 = point_value

	var current := _helper.get_tree().root.get_mouse_position()
	var delta := point - current
	var steps := maxi(1, int(options.get("steps", 1)))
	var duration := maxf(0.0, float(options.get("duration", 0.0)))
	var wait_time := duration / float(steps)
	var deltas: Array[Vector2] = _input_driver.motion_deltas(delta, steps, str(options.get("profile", "linear")))

	for step_delta in deltas:
		current += step_delta
		_input_driver.mouse_motion_to(current, options)
		await _wait_between_input_steps(wait_time)

	return true


func move_mouse_relative(delta: Variant, options: Dictionary = {}) -> bool:
	var relative_value: Variant = _coerce_vector2(delta)
	if relative_value == null:
		error("Mouse relative movement must be a Vector2 or Dictionary with x/y.")
		return false
	var relative: Vector2 = relative_value

	var steps := maxi(1, int(options.get("steps", 1)))
	var duration := maxf(0.0, float(options.get("duration", 0.0)))
	var wait_time := duration / float(steps)
	var step_options := options.duplicate(true)
	step_options["step_duration"] = wait_time
	var deltas: Array[Vector2] = _input_driver.motion_deltas(relative, steps, str(options.get("profile", "linear")))

	for step_delta in deltas:
		_input_driver.mouse_motion_relative(step_delta, step_options)
		await _wait_between_input_steps(wait_time)

	return true


func click_at(position: Variant, options: Dictionary = {}) -> Dictionary:
	var point_value: Variant = _coerce_vector2(position)
	if point_value == null:
		var position_error := "Click position must be a Vector2 or Dictionary with x/y."
		error(position_error)
		return {"success": false, "error": position_error}
	var point: Vector2 = point_value

	var button := int(options.get("button", MOUSE_BUTTON_LEFT))
	var duration := float(options.get("duration", 0.05))
	set_mouse_position(point)

	var button_options := options.duplicate(true)
	button_options["position"] = point
	_input_driver.press_mouse(button, button_options)

	await wait(duration)

	_input_driver.release_mouse(button, button_options)

	return {"success": true, "button": button, "position": {"x": point.x, "y": point.y}}


func click_button(node_or_path: Variant, options: Dictionary = {}) -> Dictionary:
	var node := _resolve_node(node_or_path)
	if node == null:
		var missing_error := "Button node was not found: %s" % str(node_or_path)
		error(missing_error)
		return {"success": false, "error": missing_error}
	if not node is BaseButton:
		var type_error := "Node is not a BaseButton: %s" % _node_path_text(node)
		error(type_error)
		return {"success": false, "error": type_error}

	var button := node as BaseButton
	var mode := str(options.get("mode", "mouse")).to_lower()
	if mode == "signal":
		button.pressed.emit()
		return {"success": true, "mode": "signal", "path": _node_path_text(button), "text": button.text}
	if mode != "mouse":
		var mode_error := "Unsupported click_button mode: %s" % mode
		error(mode_error)
		return {"success": false, "error": mode_error}

	var center := button.get_global_rect().get_center()
	var result := await click_at(center, options)
	result["mode"] = "mouse"
	result["path"] = _node_path_text(button)
	result["text"] = button.text
	return result


func find_button_by_text(text: String, options: Dictionary = {}) -> String:
	var root := get_scene_root()
	var case_sensitive := bool(options.get("case_sensitive", false))
	var exact := bool(options.get("exact", true))
	var visible_only := bool(options.get("visible_only", true))
	var found := _find_button_by_text_recursive(root, text, case_sensitive, exact, visible_only)
	if found == null:
		self.log("button text not found", {"text": text})
		return ""
	var path := _node_path_text(found)
	self.log("found button by text", {"text": text, "path": path, "button_text": found.text})
	return path


func button_path_by_text(text: String, options: Dictionary = {}) -> String:
	return find_button_by_text(text, options)


func release_all_actions() -> void:
	_release_pressed_actions()


func release_all_inputs() -> void:
	_release_all_inputs()


func get_scene_root() -> Node:
	if _helper == null or not is_instance_valid(_helper):
		return null
	var tree := _helper.get_tree()
	if tree == null:
		return null
	if tree.current_scene != null and is_instance_valid(tree.current_scene):
		return tree.current_scene
	if tree.root != null and is_instance_valid(tree.root):
		return tree.root
	return null


func _run_action_sequence_step(step: Dictionary) -> Dictionary:
	if step.has("wait"):
		await wait(float(step.get("wait", 0.0)))
		return {"success": true}
	if step.has("capture"):
		var capture_result := await capture(str(step.get("capture", "")), int(step.get("max_resolution", 1280)))
		return {"success": bool(capture_result.get("success", false)), "capture": capture_result}
	if step.has("click_at"):
		return await click_at(step.get("click_at"), step.get("options", {}))
	if step.has("click_button"):
		return await click_button(step.get("click_button"), step.get("options", {}))
	if step.has("mouse_position"):
		var moved := set_mouse_position(step.get("mouse_position"))
		return {"success": moved}

	var actions := _actions_from_step(step)
	if actions.is_empty():
		var action_error := "Action sequence step must contain action/actions, wait, capture, click_at, click_button, or mouse_position."
		error(action_error)
		return {"success": false, "error": action_error}

	var phase: Variant = step.get("phase", "tap")
	if step.has("duration") and str(phase).to_lower() == "tap":
		phase = "tap"
	var duration := float(step.get("duration", 0.1))
	var strength := float(step.get("strength", 1.0))
	var ok := await _apply_action_phase(actions, phase, duration, strength)
	return {"success": ok}


func _actions_from_step(step: Dictionary) -> Array[String]:
	var actions: Array[String] = []
	if step.has("action"):
		actions.append(str(step.get("action", "")))
	elif step.has("actions"):
		var raw_actions = step.get("actions", [])
		if raw_actions is Array:
			for action_name in raw_actions:
				actions.append(str(action_name))
	actions = actions.filter(func(action_name: String) -> bool: return not action_name.strip_edges().is_empty())
	return actions


func _apply_action_phase(actions: Array[String], phase_or_duration: Variant, duration: float = 0.1, strength: float = 1.0) -> bool:
	if phase_or_duration is int or phase_or_duration is float:
		return await _hold_actions(actions, float(phase_or_duration), strength)

	var phase := str(phase_or_duration).to_lower()
	match phase:
		"press":
			for action_name in actions:
				if not press_action(action_name, strength):
					return false
			return true
		"release":
			for action_name in actions:
				if not release_action(action_name):
					return false
			return true
		"tap":
			return await _hold_actions(actions, duration, strength)
		_:
			error("Unsupported action phase: %s" % phase)
			return false


func _until_condition(predicate: Callable, options: Dictionary) -> Dictionary:
	var timeout := maxf(0.0, float(options.get("timeout", 1.0)))
	var interval := maxf(0.0, float(options.get("interval", 0.05)))
	var start_ms := Time.get_ticks_msec()
	var attempts := 0

	while true:
		attempts += 1
		if bool(predicate.call()):
			return _until_receipt(true, false, start_ms, attempts)

		var elapsed := float(Time.get_ticks_msec() - start_ms) / 1000.0
		if elapsed >= timeout:
			return _until_receipt(false, true, start_ms, attempts)

		await _wait_until_next_poll(interval, timeout, elapsed)

	return _until_receipt(false, true, start_ms, attempts)


func _wait_until_next_poll(interval: float, timeout: float, elapsed: float) -> void:
	var remaining := maxf(0.0, timeout - elapsed)
	var wait_time := minf(interval, remaining)
	if wait_time > 0.0:
		await wait(wait_time)
	else:
		await _helper.get_tree().process_frame


func _until_receipt(success: bool, timed_out: bool, start_ms: int, attempts: int) -> Dictionary:
	return {
		"success": success,
		"timeout": timed_out,
		"attempts": attempts,
		"elapsed": float(Time.get_ticks_msec() - start_ms) / 1000.0,
	}


func _object_has_property(target: Object, property_name: String) -> bool:
	for property_info in target.get_property_list():
		if property_info is Dictionary and str(property_info.get("name", "")) == property_name:
			return true
	return false


func _values_match(value: Variant, expected: Variant, options: Dictionary = {}) -> bool:
	var op := str(options.get("op", options.get("operator", "eq"))).to_lower()
	match op:
		"eq", "equals", "==":
			return value == expected
		"ne", "not_equals", "!=":
			return value != expected
		"gt", ">":
			return _numeric_compare(value, expected, func(left: float, right: float) -> bool: return left > right)
		"gte", ">=":
			return _numeric_compare(value, expected, func(left: float, right: float) -> bool: return left >= right)
		"lt", "<":
			return _numeric_compare(value, expected, func(left: float, right: float) -> bool: return left < right)
		"lte", "<=":
			return _numeric_compare(value, expected, func(left: float, right: float) -> bool: return left <= right)
		"contains":
			if value is String:
				return str(value).contains(str(expected))
			if value is Array:
				return (value as Array).has(expected)
			if value is Dictionary:
				return (value as Dictionary).has(expected)
			return false
		_:
			return value == expected


func _numeric_compare(value: Variant, expected: Variant, compare: Callable) -> bool:
	if not (value is int or value is float) or not (expected is int or expected is float):
		return false
	return bool(compare.call(float(value), float(expected)))


func _hold_actions(actions: Array[String], duration: float, strength: float = 1.0) -> bool:
	for action_name in actions:
		if not press_action(action_name, strength):
			return false
	await wait(duration)
	var ok := true
	for action_name in actions:
		ok = release_action(action_name) and ok
	return ok


func _move_mouse_for_options(options: Dictionary) -> bool:
	if not options.has("position"):
		return true
	var move_options: Dictionary = {}
	var raw_move: Variant = options.get("move", {})
	if raw_move is Dictionary:
		move_options = raw_move
	return await move_mouse_to(options.get("position"), move_options)


func _wait_between_input_steps(wait_time: float) -> void:
	if wait_time > 0.0:
		await wait(wait_time)
	else:
		await _helper.get_tree().process_frame


func _coerce_vector2(value: Variant) -> Variant:
	if value is Vector2:
		return value
	if value is Dictionary and value.has("x") and value.has("y"):
		return Vector2(float(value.get("x")), float(value.get("y")))
	if value is Array and value.size() >= 2:
		return Vector2(float(value[0]), float(value[1]))
	return null


func _resolve_node(node_or_path: Variant) -> Node:
	if node_or_path is Object and not is_instance_valid(node_or_path):
		return null
	if node_or_path is Node:
		return node_or_path
	var root := get_scene_root()
	var path := str(node_or_path)
	if path.strip_edges().is_empty():
		return null
	var tree := _helper.get_tree() if _helper != null and is_instance_valid(_helper) else null
	if path.begins_with("/root/"):
		return tree.root.get_node_or_null(NodePath(path.trim_prefix("/root/"))) if tree != null and tree.root != null else null
	if path.begins_with("/"):
		return tree.root.get_node_or_null(NodePath(path.trim_prefix("/"))) if tree != null and tree.root != null else null
	if root != null:
		return root.get_node_or_null(NodePath(path))
	return null


func _viewport_for_reference(reference_node_or_path: Variant = null) -> Viewport:
	if reference_node_or_path != null:
		var reference := _resolve_node(reference_node_or_path)
		if reference != null and reference.is_inside_tree():
			return reference.get_viewport()
	var root := get_scene_root()
	if root != null and root.is_inside_tree():
		return root.get_viewport()
	if _helper != null and is_instance_valid(_helper) and _helper.is_inside_tree():
		return _helper.get_viewport()
	if _helper != null and is_instance_valid(_helper):
		var tree := _helper.get_tree()
		if tree != null:
			return tree.root
	return null


func _node_path_text(node: Node) -> String:
	if node == null or not is_instance_valid(node):
		return ""
	var root := get_scene_root()
	if node == root:
		return "."
	if root != null and root.is_ancestor_of(node):
		return str(root.get_path_to(node))
	return str(node.get_path())


func _find_button_by_text_recursive(node: Node, text: String, case_sensitive: bool, exact: bool, visible_only: bool) -> BaseButton:
	if node == null or not is_instance_valid(node):
		return null
	if node is BaseButton:
		var button := node as BaseButton
		if (not visible_only or button.is_visible_in_tree()) and _text_matches(button.text, text, case_sensitive, exact):
			return button
	for child in node.get_children():
		var found := _find_button_by_text_recursive(child, text, case_sensitive, exact, visible_only)
		if found != null:
			return found
	return null


func _text_matches(value: String, query: String, case_sensitive: bool, exact: bool) -> bool:
	var left := value if case_sensitive else value.to_lower()
	var right := query if case_sensitive else query.to_lower()
	return left == right if exact else left.contains(right)


func _print_event(prefix: String, data: Dictionary = {}) -> void:
	var event := data.duplicate(true)
	event["session_id"] = _session_id
	event["script_run_id"] = _script_run_id
	event["time_ms"] = Time.get_ticks_msec()
	print("%s: %s" % [prefix, JSON.stringify(event)])


func _write_status(status: String, error: String = "", extra: Dictionary = {}) -> void:
	_helper._write_runtime_script_status(_status_path, _session_id, _script_run_id, status, _captures, error, extra)


func _release_pressed_actions() -> int:
	var released := 0
	for action in _pressed_actions.duplicate():
		if InputMap.has_action(action):
			Input.action_release(action)
			released += 1
	_pressed_actions.clear()
	return released


func _release_all_inputs() -> Dictionary:
	var released_actions := _release_pressed_actions()
	var input_result: Dictionary = _input_driver.release_all()
	input_result["released_actions"] = released_actions
	return input_result
