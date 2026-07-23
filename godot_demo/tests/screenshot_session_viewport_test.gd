extends SceneTree


func _initialize() -> void:
	call_deferred("_run")


func _run() -> void:
	await process_frame
	var extension := load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)

	var result: Dictionary = ClassDB.class_call_static(
		"FennaraScreenshotSceneTool",
		"test_script_viewport_reuse",
	)
	if not result.get("success", false):
		push_error(str(result))
		quit(1)
		return
	assert(result.get("same_viewport", false))
	assert(result.get("root_preserved", false))
	assert(result.get("supplied_camera_preserved", false))
	assert(result.get("helper_removed", false))
	assert(result.get("resized", false))
	assert(result.get("failed_setup_preserved", false))
	assert(result.get("session_cleared", false))

	print("screenshot session viewport test passed")
	quit()
