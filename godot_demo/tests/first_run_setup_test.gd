extends SceneTree


func _initialize() -> void:
	var original_local_app_data := OS.get_environment("LOCALAPPDATA")
	var test_local_app_data := ProjectSettings.globalize_path(
		"res://.godot/first-run-setup-test-appdata",
	)
	OS.set_environment("LOCALAPPDATA", test_local_app_data)
	var extension := load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)
	OS.set_environment("FENNARA_FORCE_FIRST_RUN_SETUP", "1")

	var lock_path := OS.get_environment("LOCALAPPDATA").path_join(
		"Fennara/cache/setup/bootstrap.lock",
	)
	assert(DirAccess.make_dir_recursive_absolute(lock_path) == OK)
	var owner := FileAccess.open(lock_path.path_join("owner.json"), FileAccess.WRITE)
	assert(owner != null)
	owner.store_string(JSON.stringify({"pid": OS.get_process_id()}))
	owner.close()
	var waiting_setup: Variant = ClassDB.instantiate("FirstRunSetup")
	root.add_child(waiting_setup)
	waiting_setup.start(ProjectSettings.globalize_path("res://"), "0.3.8")
	assert(waiting_setup.is_running())
	assert(waiting_setup.get_status() == "Waiting for another Fennara setup...")
	waiting_setup.free()
	assert(DirAccess.remove_absolute(lock_path.path_join("owner.json")) == OK)
	assert(DirAccess.remove_absolute(lock_path) == OK)

	var setup: Variant = ClassDB.instantiate("FirstRunSetup")
	assert(setup != null)
	root.add_child(setup)

	OS.set_environment("FENNARA_SETUP_TEST_FAILURE", "manifest")
	assert(setup.is_setup_required())

	setup.start(ProjectSettings.globalize_path("res://"), "../../invalid")
	assert(setup.has_failed())
	assert(setup.get_error_code() == "FEN-SETUP-PROJECT-INVALID")

	setup.start(ProjectSettings.globalize_path("res://"), "0.3.8")
	assert(setup.has_failed())
	assert(setup.get_error_code() == "FEN-SETUP-MANIFEST-DOWNLOAD")
	assert(setup.get_status() == "Fennara setup could not finish.")
	assert(setup.get_operation_id().is_empty())

	OS.set_environment("FENNARA_SETUP_TEST_FAILURE", "launch")
	OS.set_environment(
		"FENNARA_SETUP_CLI_PATH",
		ProjectSettings.globalize_path("res://project.godot"),
	)
	setup.retry()
	assert(setup.has_failed())
	assert(setup.get_error_code() == "FEN-SETUP-CLI-LAUNCH")

	OS.set_environment("FENNARA_SETUP_CLI_PATH", "")
	OS.set_environment("FENNARA_SETUP_TEST_FAILURE", "")
	OS.set_environment("FENNARA_SETUP_TEST_SUCCESS", "1")
	var success_signal_seen := [false]
	setup.setup_succeeded.connect(func() -> void: success_signal_seen[0] = true)
	setup.retry()
	assert(setup.has_succeeded())
	assert(setup.get_operation_id() == "install-test-success")
	assert(success_signal_seen[0])

	OS.set_environment("FENNARA_SETUP_TEST_SUCCESS", "")
	OS.set_environment("FENNARA_FORCE_FIRST_RUN_SETUP", "")
	OS.set_environment("LOCALAPPDATA", original_local_app_data)
	setup.queue_free()
	DirAccess.remove_absolute(test_local_app_data.path_join("Fennara/cache/setup"))
	DirAccess.remove_absolute(test_local_app_data.path_join("Fennara/cache"))
	DirAccess.remove_absolute(test_local_app_data.path_join("Fennara"))
	DirAccess.remove_absolute(test_local_app_data)
	print("first-run setup state test passed")
	quit(0)
