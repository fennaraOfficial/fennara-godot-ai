extends RefCounted

var _helper: Node


func _init(helper: Node) -> void:
	_helper = helper


func safe_file_component(value: String, fallback: String) -> String:
	var safe := value.strip_edges().to_lower()
	safe = safe.replace(" ", "_")
	safe = safe.replace("/", "_")
	safe = safe.replace("\\", "_")
	safe = safe.replace(":", "_")
	safe = safe.replace("@", "_")
	safe = safe.replace(".", "_")
	return fallback if safe.is_empty() else safe


func absolute_path(path: String) -> String:
	return path if path.is_absolute_path() else ProjectSettings.globalize_path(path)


func ensure_dir(path: String) -> bool:
	return DirAccess.make_dir_recursive_absolute(absolute_path(path)) == OK


func read_json_file(path: String) -> Dictionary:
	var file := FileAccess.open(path, FileAccess.READ)
	if file == null:
		return {}
	var parsed: Variant = JSON.parse_string(file.get_as_text())
	file.close()
	if parsed is Dictionary:
		return parsed
	return {}


func normalized_screenshot_times(value: Variant) -> Array[float]:
	var collected: Array[float] = []
	if value is Array:
		for entry in value:
			collected.append(maxf(0.0, float(entry)))
	collected.sort()
	var times: Array[float] = []
	for time in collected:
		if times.is_empty() or absf(times[times.size() - 1] - time) > 0.001:
			times.append(time)
	return times


func capture_runtime_script(ctx, label: String, max_resolution: int = 1280) -> Dictionary:
	var capture: Dictionary = await wait_for_viewport_image(max_resolution)
	if not capture.get("success", false):
		var capture_error := str(capture.get("error", "Runtime screenshot failed."))
		ctx.error(capture_error)
		return {"success": false, "error": capture_error}

	var captures_dir: String = ctx._captures_dir
	if not ensure_dir(captures_dir):
		var dir_message := "Could not create runtime capture directory."
		ctx.error(dir_message)
		return {"success": false, "error": dir_message}

	var file_name := "%s_%s_%d.png" % [
		safe_file_component(ctx._script_run_id, "script"),
		safe_file_component(label, "capture"),
		Time.get_ticks_msec(),
	]
	var image_res_path: String = captures_dir.path_join(file_name)
	var image: Image = capture["image"]
	var png_error_code := image.save_png(image_res_path)
	if png_error_code != OK:
		var png_error := "Failed to save runtime capture PNG."
		ctx.error(png_error)
		return {"success": false, "error": png_error}

	var result := {
		"success": true,
		"label": label,
		"image_res_path": image_res_path,
		"image_path": absolute_path(image_res_path),
		"width": capture["width"],
		"height": capture["height"],
		"original_width": capture["original_width"],
		"original_height": capture["original_height"],
	}
	ctx._print_event("FENNARA_SCRIPT_CAPTURE", result)
	return result


func capture_runtime_session_start(artifact_dir: String, session_id: String, scene_path: String, max_resolution: int = 1280) -> Dictionary:
	var capture: Dictionary = await wait_for_viewport_image(max_resolution)
	if not capture.get("success", false):
		return {
			"success": false,
			"error": str(capture.get("error", "Runtime startup screenshot failed.")),
			"label": "startup",
			"image_role": "runtime_startup",
			"session_id": session_id,
			"scene_path": scene_path,
		}

	var captures_dir := artifact_dir.path_join("captures")
	if not ensure_dir(captures_dir):
		return {
			"success": false,
			"error": "Could not create runtime capture directory.",
			"label": "startup",
			"image_role": "runtime_startup",
			"session_id": session_id,
			"scene_path": scene_path,
		}

	var file_name := "%s_startup_%d.png" % [
		safe_file_component(session_id, "runtime"),
		Time.get_ticks_msec(),
	]
	var image_res_path: String = captures_dir.path_join(file_name)
	var image: Image = capture["image"]
	var png_error_code := image.save_png(image_res_path)
	if png_error_code != OK:
		return {
			"success": false,
			"error": "Failed to save runtime startup PNG.",
			"label": "startup",
			"image_role": "runtime_startup",
			"session_id": session_id,
			"scene_path": scene_path,
		}

	return {
		"success": true,
		"label": "startup",
		"image_role": "runtime_startup",
		"session_id": session_id,
		"scene_path": scene_path,
		"image_res_path": image_res_path,
		"image_path": absolute_path(image_res_path),
		"width": capture["width"],
		"height": capture["height"],
		"original_width": capture["original_width"],
		"original_height": capture["original_height"],
	}


func capture_env_runtime_screenshot(
	screenshot_dir: String,
	check_id: String,
	index: int,
	time_seconds: float,
	max_resolution: int
) -> Dictionary:
	var capture: Dictionary = await wait_for_viewport_image(max_resolution)
	if not capture.get("success", false):
		return {
			"success": false,
			"error": str(capture.get("error", "Runtime screenshot failed.")),
			"time_seconds": time_seconds,
		}
	var file_name := "%s_%02d_%.2fs.png" % [
		safe_file_component(check_id, "runtime"),
		index,
		time_seconds,
	]
	var image_path := screenshot_dir.path_join(file_name)
	var image: Image = capture["image"]
	var png_error := image.save_png(image_path)
	if png_error != OK:
		return {"success": false, "error": "Failed to save runtime screenshot PNG.", "time_seconds": time_seconds}

	return {
		"success": true,
		"time_seconds": time_seconds,
		"image_path": image_path,
		"width": capture["width"],
		"height": capture["height"],
		"original_width": capture["original_width"],
		"original_height": capture["original_height"],
	}


func wait_for_viewport_image(max_resolution: int, max_frames: int = 5) -> Dictionary:
	var capture: Dictionary = {}
	for _i in range(maxi(1, max_frames)):
		await _helper.get_tree().process_frame
		capture = viewport_image(max_resolution)
		if capture.get("success", false):
			return capture
	return capture


func viewport_image(max_resolution: int) -> Dictionary:
	var texture := _helper.get_tree().root.get_texture()
	if texture == null:
		return {"success": false, "error": "Runtime viewport texture was unavailable."}

	var image := texture.get_image()
	if image == null or image.is_empty():
		return {"success": false, "error": "Runtime viewport image was empty."}

	var original_w := image.get_width()
	var original_h := image.get_height()
	if max_resolution > 0:
		var longest := maxi(original_w, original_h)
		if longest > max_resolution:
			var scale := float(max_resolution) / float(longest)
			image.resize(maxi(1, int(original_w * scale)), maxi(1, int(original_h * scale)))

	return {
		"success": true,
		"image": image,
		"width": image.get_width(),
		"height": image.get_height(),
		"original_width": original_w,
		"original_height": original_h,
	}


func write_env_runtime_status(path: String, payload: Dictionary) -> void:
	if path.strip_edges().is_empty():
		return
	var base_dir := path.get_base_dir()
	if base_dir.is_absolute_path():
		DirAccess.make_dir_recursive_absolute(base_dir)
	else:
		DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path(base_dir))
	var file := FileAccess.open(path, FileAccess.WRITE)
	if file == null:
		return
	file.store_string(JSON.stringify(payload, "\t"))
	file.close()


func write_runtime_check_status(path: String, check_id: String, status: String, captures: Array[Dictionary], errors: Array[String], extra: Dictionary = {}) -> void:
	var payload := extra.duplicate(true)
	payload["success"] = errors.is_empty() and status != "helper_started" and status != "scene_frame_ready" and status != "failed"
	payload["status"] = status
	payload["check_id"] = check_id
	payload["captures"] = captures
	if not errors.is_empty():
		payload["errors"] = errors
	write_env_runtime_status(path, payload)


func write_runtime_script_status(status_path: String, session_id: String, script_run_id: String, status: String, captures: Array[Dictionary], error: String = "", extra: Dictionary = {}) -> void:
	var base_dir := status_path.get_base_dir()
	if base_dir.is_absolute_path():
		DirAccess.make_dir_recursive_absolute(base_dir)
	else:
		DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path(base_dir))
	var file := FileAccess.open(status_path, FileAccess.WRITE)
	if file == null:
		return
	var payload := {
		"session_id": session_id,
		"script_run_id": script_run_id,
		"status": status,
		"captures": captures,
		"updated_at_ms": Time.get_ticks_msec(),
	}
	for key in extra.keys():
		payload[key] = extra[key]
	if not error.is_empty():
		payload["error"] = error
	file.store_string(JSON.stringify(payload))
	file.close()
