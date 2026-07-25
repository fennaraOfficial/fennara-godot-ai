extends SceneTree


const RUNTIME_AUTOLOAD_KEY := "autoload/_fennara_game_capture"
const TEST_AUTOLOAD_VALUE := "*res://addons/fennara/runtime/game_capture_helper.gd"
const EXTENSION_LIST_BACKUP_SUFFIX := ".fennara-export-backup"


func _initialize() -> void:
	var extension := load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)
	var project_data_directory := (
		"res://.godot"
		if ProjectSettings.get_setting(
			"application/config/use_hidden_project_data_directory",
			true,
		)
		else "res://godot"
	)
	var extension_list_path := project_data_directory.path_join("extension_list.cfg")
	var had_original_extension_list := FileAccess.file_exists(extension_list_path)
	var actual_extension_list := FileAccess.get_file_as_string(extension_list_path)
	var original_extension_list := actual_extension_list
	if not original_extension_list.ends_with("\n") and not original_extension_list.is_empty():
		original_extension_list += "\n"
	if "res://addons/fennara/" not in original_extension_list:
		original_extension_list += "res://addons/fennara/fennara.gdextension\n"
	_write_text(extension_list_path, original_extension_list)

	var export_plugin: Variant = ClassDB.instantiate("FennaraExportPlugin")
	assert(export_plugin != null)

	assert(export_plugin.test_should_skip_path(
		"res://addons/fennara/runtime/game_capture_helper.gd",
	))
	assert(export_plugin.test_should_skip_path(
		"RES:\\\\ADDONS\\FENNARA\\VERSION",
	))
	assert(export_plugin.test_should_skip_path("res://.fennara/session.json"))
	assert(not export_plugin.test_should_skip_path(
		"res://addons/another_plugin/plugin.gd",
	))
	assert(export_plugin.test_filter_extension_list(
		"res://addons/another/addon.gdextension\n"
		+ "res://addons/fennara/fennara.gdextension\n",
	) == "res://addons/another/addon.gdextension\n")

	var had_original := ProjectSettings.has_setting(RUNTIME_AUTOLOAD_KEY)
	var original_value: Variant
	if had_original:
		original_value = ProjectSettings.get_setting(RUNTIME_AUTOLOAD_KEY)

	ProjectSettings.set_setting(RUNTIME_AUTOLOAD_KEY, TEST_AUTOLOAD_VALUE)
	export_plugin.test_export_begin()
	assert(not ProjectSettings.has_setting(RUNTIME_AUTOLOAD_KEY))
	export_plugin.test_suppress_extension_list_entry()
	assert(FileAccess.file_exists(
		extension_list_path + EXTENSION_LIST_BACKUP_SUFFIX,
	))
	assert(
		"res://addons/fennara/" not in FileAccess.get_file_as_string(
			extension_list_path,
		),
	)
	export_plugin.test_export_end()
	assert(ProjectSettings.get_setting(
		RUNTIME_AUTOLOAD_KEY,
	) == TEST_AUTOLOAD_VALUE)
	assert(FileAccess.get_file_as_string(
		extension_list_path,
	) == original_extension_list)
	assert(not FileAccess.file_exists(
		extension_list_path + EXTENSION_LIST_BACKUP_SUFFIX,
	))

	ProjectSettings.set_setting(RUNTIME_AUTOLOAD_KEY, null)
	export_plugin.test_export_begin()
	assert(not ProjectSettings.has_setting(RUNTIME_AUTOLOAD_KEY))
	export_plugin.test_export_end()
	assert(not ProjectSettings.has_setting(RUNTIME_AUTOLOAD_KEY))

	if had_original:
		ProjectSettings.set_setting(RUNTIME_AUTOLOAD_KEY, original_value)
	else:
		ProjectSettings.set_setting(RUNTIME_AUTOLOAD_KEY, null)
	export_plugin = null

	_write_text(
		extension_list_path + EXTENSION_LIST_BACKUP_SUFFIX,
		original_extension_list,
	)
	_write_text(extension_list_path, "")
	var recovery_plugin: Variant = ClassDB.instantiate("FennaraExportPlugin")
	assert(recovery_plugin != null)
	assert(FileAccess.get_file_as_string(
		extension_list_path,
	) == original_extension_list)
	assert(not FileAccess.file_exists(
		extension_list_path + EXTENSION_LIST_BACKUP_SUFFIX,
	))
	recovery_plugin = null
	if had_original_extension_list:
		_write_text(extension_list_path, actual_extension_list)
	else:
		DirAccess.remove_absolute(extension_list_path)
	print("export plugin test passed")
	quit()


func _write_text(path: String, contents: String) -> void:
	var file := FileAccess.open(path, FileAccess.WRITE)
	assert(file != null)
	file.store_string(contents)
