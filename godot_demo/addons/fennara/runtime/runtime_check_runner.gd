extends RefCounted

var _helper: Node
var _capture_store


func _init(helper: Node, capture_store) -> void:
	_helper = helper
	_capture_store = capture_store


func run_env_runtime_check(spec_path: String) -> void:
	var request: Dictionary = _capture_store.read_json_file(spec_path)
	if request.is_empty():
		return

	var status_path := str(request.get("status_path", ""))
	var check_id := str(request.get("check_id", ""))
	_capture_store.write_runtime_check_status(status_path, check_id, "helper_started", [], [], {
		"spec_path": spec_path,
		"current_scene": _helper.get_tree().current_scene.scene_file_path if _helper.get_tree().current_scene != null else "",
		"timestamp_ms": Time.get_ticks_msec(),
	})

	var screenshot_dir := str(request.get("screenshot_dir", ""))
	var dir_error := DirAccess.make_dir_recursive_absolute(screenshot_dir)
	if dir_error != OK:
		_capture_store.write_runtime_check_status(status_path, check_id, "failed", [], [], {
			"error": "Could not create screenshot directory: %s" % screenshot_dir,
		})
		return

	var times: Array[float] = _capture_store.normalized_screenshot_times(request.get("screenshot_times", []))
	var run_seconds := maxf(0.0, float(request.get("run_seconds", 0.0)))
	var max_resolution := int(request.get("max_resolution", 1280))
	var captures: Array[Dictionary] = []
	var errors: Array[String] = []
	var last_time := 0.0
	var scene_frame := await wait_for_env_runtime_scene_frame()
	if not scene_frame.get("success", false):
		_capture_store.write_runtime_check_status(status_path, check_id, "failed", captures, errors, {
			"error": str(scene_frame.get("error", "Scene did not render a frame.")),
		})
		return

	await raise_runtime_window_once()
	_capture_store.write_runtime_check_status(status_path, check_id, "scene_frame_ready", captures, errors, {
		"scene_path": scene_frame.get("scene_path", ""),
		"scene_frame_ready_ms": scene_frame.get("scene_frame_ready_ms", 0),
	})

	for i in range(times.size()):
		var target_time := float(times[i])
		var wait_time := maxf(0.0, target_time - last_time)
		if wait_time > 0.0:
			await _helper.get_tree().create_timer(wait_time, true, false, true).timeout
		last_time = target_time

		var capture: Dictionary = await _capture_store.capture_env_runtime_screenshot(
			screenshot_dir,
			check_id if not check_id.is_empty() else "runtime",
			i + 1,
			target_time,
			max_resolution
		)
		if capture.get("success", false):
			captures.append(capture)
		else:
			var capture_error := str(capture.get("error", "Runtime screenshot failed."))
			errors.append(capture_error)
			if capture_error.contains("viewport"):
				break
		_capture_store.write_runtime_check_status(status_path, check_id, "running", captures, errors)

	if run_seconds > last_time:
		await _helper.get_tree().create_timer(run_seconds - last_time, true, false, true).timeout

	_capture_store.write_runtime_check_status(status_path, check_id, "completed" if errors.is_empty() else "completed_with_errors", captures, errors)
	_helper.get_tree().quit(0)


func raise_runtime_window_once() -> void:
	if DisplayServer.get_name() == "headless":
		return
	DisplayServer.window_set_flag(DisplayServer.WINDOW_FLAG_ALWAYS_ON_TOP, true)
	DisplayServer.window_move_to_foreground()
	DisplayServer.window_request_attention()
	await _helper.get_tree().process_frame
	await _helper.get_tree().process_frame
	DisplayServer.window_set_flag(DisplayServer.WINDOW_FLAG_ALWAYS_ON_TOP, false)


func wait_for_env_runtime_scene_frame() -> Dictionary:
	var started_ms := Time.get_ticks_msec()
	var tree := _helper.get_tree()
	for _i in range(600):
		await tree.process_frame
		var current_scene := tree.current_scene
		if current_scene == null:
			continue
		await tree.process_frame
		await tree.process_frame
		var viewport := tree.root
		var texture := viewport.get_texture()
		if texture == null:
			continue
		var image := texture.get_image()
		if image == null or image.is_empty():
			continue
		return {
			"success": true,
			"scene_path": current_scene.scene_file_path,
			"scene_frame_ready_ms": Time.get_ticks_msec() - started_ms,
		}
	return {"success": false, "error": "Scene did not produce a readable viewport frame before the runtime helper wait limit."}
