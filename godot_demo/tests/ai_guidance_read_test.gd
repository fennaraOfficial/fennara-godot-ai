extends SceneTree


const GUIDANCE_PATHS: Array[String] = [
	"res://addons/fennara/ai/guidelines.md",
	"res://addons/fennara/ai/index.md",
	"res://addons/fennara/ai/visual-observation.md",
	"res://addons/fennara/ai/runtime-observation.md",
	"res://addons/fennara/ai/operations.md",
	"res://addons/fennara/ai/clients/cursor.md",
]


func _initialize() -> void:
	var extension: Resource = load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)

	for start_index: int in range(0, GUIDANCE_PATHS.size(), 5):
		var end_index: int = mini(start_index + 5, GUIDANCE_PATHS.size())
		var paths: Array[String] = GUIDANCE_PATHS.slice(start_index, end_index)
		var result: Dictionary = _read(paths)
		assert(result.get("success", false))
		for file: Dictionary in result.get("files", []):
			assert(file.get("status", "") == "success")
			assert(file.get("kind", "") == "text")

	var blocked_result: Dictionary = _read([
		"res://addons/fennara/VERSION",
		"res://addons/fennara/ai/../VERSION",
	])
	assert(not blocked_result.get("success", true))
	var blocked_files: Array = blocked_result.get("files", [])
	assert(blocked_files[0].get("status", "") == "blocked")
	assert(blocked_files[1].get("status", "") == "failed")
	assert("cannot contain '..'" in blocked_files[1].get("error", ""))

	print("AI guidance read test passed")
	quit()


func _read(paths: Array[String]) -> Dictionary:
	return ClassDB.class_call_static(
		"FennaraReadFileTool",
		"execute",
		{"file_paths": paths},
	)
