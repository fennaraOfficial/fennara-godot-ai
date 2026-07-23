extends SceneTree

const RuntimeCaptureStore := preload(
	"res://addons/fennara/runtime/runtime_capture_store.gd"
)
const RuntimeScriptContext := preload(
	"res://addons/fennara/runtime/runtime_script_context.gd"
)


class FakeRuntimeHelper:
	extends Node

	var _file_artifact_dir := ProjectSettings.globalize_path(
		"user://runtime-image-context-test",
	)
	var output_calls := 0

	func _safe_file_component(value: String, fallback: String) -> String:
		return fallback if value.strip_edges().is_empty() else value

	func _frame_runtime_script(_max_resolution: int) -> Dictionary:
		var image := Image.create(16, 9, false, Image.FORMAT_RGBA8)
		image.fill(Color.CORNFLOWER_BLUE)
		return {"success": true, "image": image}

	func _output_runtime_script(
		_ctx,
		image: Image,
		description: String,
	) -> Dictionary:
		output_calls += 1
		return {
			"success": true,
			"image": image,
			"description": description,
		}


class FakeOutputContext:
	extends RefCounted

	var _captures_dir := ProjectSettings.globalize_path(
		"user://runtime-image-output-test",
	)
	var _script_run_id := "script-test"
	var _captures: Array[Dictionary] = []
	var events: Array[Dictionary] = []
	var errors: Array[String] = []

	func _print_event(_prefix: String, result: Dictionary) -> void:
		events.append(result)

	func error(message: String) -> void:
		errors.append(message)


func _initialize() -> void:
	var helper := FakeRuntimeHelper.new()
	var ctx := RuntimeScriptContext.new(
		helper,
		"runtime-test",
		"script-test",
	)
	var frame: Image = await ctx.frame(64)
	assert(frame != null)
	assert(frame.get_size() == Vector2i(16, 9))

	var sheets: Array[Image] = ctx.sheet(
		[frame],
		{
			"columns": 1,
			"cell_size": Vector2i(32, 18),
			"labels": ["R00"],
		},
	)
	assert(sheets.size() == 1)
	assert(sheets[0].get_size() == Vector2i(32, 18))
	var output: Dictionary = ctx.output(sheets[0], "runtime sheet")
	assert(output.get("success", false))
	assert(helper.output_calls == 1)

	var capture_store := RuntimeCaptureStore.new(helper)
	var output_ctx := FakeOutputContext.new()
	var saved: Dictionary = capture_store.output_runtime_script(
		output_ctx,
		sheets[0],
		"saved runtime sheet",
	)
	assert(saved.get("success", false))
	assert(saved.get("image_role", "") == "runtime_script_output")
	assert(saved.get("description", "") == "saved runtime sheet")
	assert(FileAccess.file_exists(saved.get("image_path", "")))
	assert(output_ctx.events.size() == 1)
	assert(output_ctx.errors.is_empty())

	helper.free()
	print("runtime image context test passed")
	quit()
