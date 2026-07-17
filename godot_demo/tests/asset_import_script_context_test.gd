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

	var scripted_node := Node.new()
	var attached_script := GDScript.new()
	attached_script.source_code = "extends Node\n"
	assert(attached_script.reload() == OK)
	scripted_node.set_script(attached_script)
	var packed_scene := PackedScene.new()
	assert(packed_scene.pack(scripted_node) == OK)
	scripted_node.free()

	context.configure_for_test("scene", {}, packed_scene, true)
	assert(context.instantiate_imported_scene() == null)
	assert(_has_attached_script_error(context.get_edit_errors()))

	context.configure_for_test("scene", {}, packed_scene, true)
	var summary: Dictionary = context.get_subresource_summary()
	assert(summary.get("nodes", -1) == 0)
	assert(_has_attached_script_error(context.get_edit_errors()))

	context.cleanup()
	context = null
	quit()


func _has_attached_script_error(errors: Array) -> bool:
	for entry: Variant in errors:
		if entry is Dictionary and "attached script" in str(entry.get("message", "")):
			return true
	return false
