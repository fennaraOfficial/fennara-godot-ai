extends SceneTree


func _initialize() -> void:
	call_deferred("_run")


func _run() -> void:
	await process_frame
	var extension := load("res://addons/fennara/fennara.gdextension")
	if extension == null:
		_fail("Fennara GDExtension did not load.")
		return

	var result: Dictionary = ClassDB.class_call_static(
		"FennaraScreenshotSceneTool",
		"test_script_viewport_reuse",
	)
	if not result.get("success", false):
		_fail("Viewport reuse hook failed: %s" % result)
		return

	var required_flags: Array[String] = [
		"same_viewport",
		"root_preserved",
		"supplied_camera_preserved",
		"helper_removed",
		"resized",
		"failed_setup_preserved",
		"session_cleared",
	]
	for flag: String in required_flags:
		if not result.get(flag, false):
			_fail("Viewport reuse check failed: %s. Result: %s" % [flag, result])
			return

	print("screenshot session viewport test passed")
	quit()


func _fail(message: String) -> void:
	push_error(message)
	quit(1)
