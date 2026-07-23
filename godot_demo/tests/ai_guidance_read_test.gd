extends SceneTree


const GUIDANCE_PATHS: Array[String] = [
	"res://addons/fennara/ai/guidelines.md",
	"res://addons/fennara/ai/index.md",
	"res://addons/fennara/ai/visual-observation.md",
	"res://addons/fennara/ai/runtime-observation.md",
	"res://addons/fennara/ai/operations.md",
	"res://addons/fennara/ai/clients/cursor.md",
]
const ESCAPE_LINK_PATH := "res://addons/fennara/ai/test_escape_link"
const ESCAPE_TARGET_PATH := "res://tests/fixtures/outside_guidance"


func _initialize() -> void:
	var extension: Resource = load("res://addons/fennara/fennara.gdextension")
	if extension == null:
		_fail("Fennara GDExtension did not load.")
		return

	for start_index: int in range(0, GUIDANCE_PATHS.size(), 5):
		var end_index: int = mini(start_index + 5, GUIDANCE_PATHS.size())
		var paths: Array[String] = GUIDANCE_PATHS.slice(start_index, end_index)
		var result: Dictionary = _read(paths)
		if not result.get("success", false):
			_fail("Guidance read failed: %s" % result)
			return
		for file: Dictionary in result.get("files", []):
			if file.get("status", "") != "success":
				_fail("Guidance file was not readable: %s" % file)
				return
			if file.get("kind", "") != "text":
				_fail("Guidance file was not returned as text: %s" % file)
				return

	var blocked_result: Dictionary = _read([
		"res://addons/fennara/VERSION",
		"res://addons/fennara/ai/../VERSION",
	])
	if blocked_result.get("success", true):
		_fail("Protected addon reads unexpectedly succeeded: %s" % blocked_result)
		return
	var blocked_files: Array = blocked_result.get("files", [])
	if blocked_files.size() != 2:
		_fail("Protected addon read returned an unexpected shape: %s" % blocked_result)
		return
	if blocked_files[0].get("status", "") != "blocked":
		_fail("Protected addon file was not blocked: %s" % blocked_files[0])
		return
	if blocked_files[1].get("status", "") != "failed":
		_fail("Traversal read was not rejected: %s" % blocked_files[1])
		return
	if "cannot contain '..'" not in blocked_files[1].get("error", ""):
		_fail("Traversal rejection returned the wrong error: %s" % blocked_files[1])
		return

	if not _create_escape_link():
		return
	var escaped_result: Dictionary = _read([
		ESCAPE_LINK_PATH.path_join("secret.md"),
	])
	_remove_escape_link()
	if escaped_result.get("success", true):
		_fail("Guidance link escaped the canonical guidance directory.")
		return
	var escaped_files: Array = escaped_result.get("files", [])
	if escaped_files.size() != 1 or escaped_files[0].get("status", "") != "blocked":
		_fail("Escaping guidance link was not blocked: %s" % escaped_result)
		return

	print("AI guidance read test passed")
	quit()


func _read(paths: Array[String]) -> Dictionary:
	return ClassDB.class_call_static(
		"FennaraReadFileTool",
		"execute",
		{"file_paths": paths},
	)


func _create_escape_link() -> bool:
	_remove_escape_link()
	var link_path: String = ProjectSettings.globalize_path(ESCAPE_LINK_PATH)
	var target_path: String = ProjectSettings.globalize_path(ESCAPE_TARGET_PATH)
	var output: Array = []
	var exit_code: int
	if OS.get_name() == "Windows":
		var command: String = 'mklink /J "%s" "%s"' % [
			link_path.replace("/", "\\"),
			target_path.replace("/", "\\"),
		]
		exit_code = OS.execute(
			"cmd.exe",
			PackedStringArray(["/d", "/s", "/c", command]),
			output,
			true,
		)
	else:
		exit_code = OS.execute(
			"ln",
			PackedStringArray(["-s", target_path, link_path]),
			output,
			true,
		)
	if exit_code == 0:
		return true
	_fail("Could not create guidance escape link: %s" % output)
	return false


func _remove_escape_link() -> void:
	var link_path: String = ProjectSettings.globalize_path(ESCAPE_LINK_PATH)
	if DirAccess.dir_exists_absolute(link_path) or FileAccess.file_exists(link_path):
		DirAccess.remove_absolute(link_path)


func _fail(message: String) -> void:
	_remove_escape_link()
	push_error(message)
	quit(1)
