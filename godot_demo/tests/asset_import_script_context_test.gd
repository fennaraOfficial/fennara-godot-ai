extends SceneTree


func _initialize() -> void:
	var extension := load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)
	var context: Variant = ClassDB.instantiate(
		"FennaraRunAssetImportScriptContext",
	)
	assert(context != null)

	context.configure_for_test(
		"texture",
		{"mipmaps/generate": false},
		null,
		false,
	)
	assert(context.set_import_option("mipmaps/generate", true))
	assert(context.get_staged_changes().size() == 1)
	assert(not context.set_import_option("missing/option", true))
	assert(context.get_staged_changes().is_empty())
	assert(not context.set_import_option("mipmaps/generate", true))

	var scene_root := Node.new()
	var scripted_child := Node.new()
	var attached_script := GDScript.new()
	attached_script.source_code = "extends Node\n"
	assert(attached_script.reload() == OK)
	scripted_child.set_script(attached_script)
	scene_root.add_child(scripted_child)
	scripted_child.owner = scene_root
	var packed_scene := PackedScene.new()
	assert(packed_scene.pack(scene_root) == OK)
	scene_root.free()

	context.configure_for_test("scene", {}, packed_scene, true)
	assert(context.instantiate_imported_scene() == null)
	assert(_has_attached_script_error(context.get_edit_errors()))

	context.configure_for_test("scene", {}, packed_scene, true)
	var summary: Dictionary = context.get_subresource_summary()
	assert(summary.get("nodes", -1) == 0)
	assert(_has_attached_script_error(context.get_edit_errors()))

	var successful_reimport: Dictionary = ClassDB.class_call_static(
		"FennaraRunAssetImportScriptTool",
		"apply_reimport_result_for_test",
		{"success": true, "reimported": true},
		1,
	)
	assert(successful_reimport.get("modified", false))
	var failed_reimport: Dictionary = ClassDB.class_call_static(
		"FennaraRunAssetImportScriptTool",
		"apply_reimport_result_for_test",
		{"success": false, "reimported": true},
		1,
	)
	assert(not failed_reimport.get("modified", true))

	var missing_paths: Array[String] = []
	for index: int in range(205):
		missing_paths.append("res://missing-output-%03d.res" % index)
	var output_verification: Dictionary = ClassDB.class_call_static(
		"FennaraRunAssetImportScriptTool",
		"verify_generated_outputs_for_test",
		missing_paths,
	)
	assert(output_verification.get("generated_file_count", 0) == 205)
	assert(output_verification.get("generated_files", []).size() == 200)
	assert(output_verification.get("generated_files_omitted_count", 0) == 5)
	assert(output_verification.get("missing_output_count", 0) == 205)
	assert(output_verification.get("missing_outputs", []).size() == 200)
	assert(output_verification.get("missing_outputs_omitted_count", 0) == 5)

	context = null
	print("asset import script context test passed")
	quit()


func _has_attached_script_error(errors: Array) -> bool:
	for entry: Variant in errors:
		if entry is Dictionary and "attached script" in str(entry.get("message", "")):
			return true
	return false
