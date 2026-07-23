extends SceneTree


func _initialize() -> void:
	var extension := load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)

	var no_script: Dictionary = _prepare({
		"scene_path": "res://main.tscn",
	})
	assert(no_script.get("success", false))
	assert(not no_script.has("_fennara_screenshot_script_path"))

	var scripted: Dictionary = _prepare({
		"scene_path": "res://main.tscn",
		"script_path": "res://tests/screenshot_scene_contract_test.gd",
	})
	assert(scripted.get("success", false))
	assert(
		scripted.get("_fennara_screenshot_script_path", "") ==
		"res://tests/screenshot_scene_contract_test.gd"
	)

	var conflicting: Dictionary = _prepare({
		"scene_path": "res://main.tscn",
		"code": "@tool extends RefCounted\nfunc run(ctx) -> void:\n\tpass\n",
		"script_path": "res://tests/screenshot_scene_contract_test.gd",
	})
	assert(not conflicting.get("success", true))
	assert(
		conflicting.get("error", "") ==
		"Provide exactly one of code or script_path."
	)

	var legacy_argument: Dictionary = _prepare({
		"scene_path": "res://main.tscn",
		"view": "top",
	})
	assert(not legacy_argument.get("success", true))
	assert(
		str(legacy_argument.get("error", "")).begins_with(
			"Unsupported screenshot_scene argument: view",
		)
	)

	var mixed_open: Dictionary = ClassDB.class_call_static(
		"FennaraScreenshotSceneTool",
		"open_scene",
		"res://tests/fixtures/mixed_2d_3d_no_camera.tscn",
	)
	assert(mixed_open.get("success", false))
	var mixed_capture: Dictionary = ClassDB.class_call_static(
		"FennaraScreenshotSceneTool",
		"navigate",
		{},
		0,
	)
	assert(not mixed_capture.get("success", true))
	assert(
		str(mixed_capture.get("error", "")).begins_with(
			"Whole-scene automatic capture is ambiguous",
		)
	)

	print("screenshot scene contract test passed")
	quit()


func _prepare(args: Dictionary) -> Dictionary:
	return ClassDB.class_call_static(
		"FennaraScreenshotSceneTool",
		"prepare_execution_for_test",
		args,
	)
