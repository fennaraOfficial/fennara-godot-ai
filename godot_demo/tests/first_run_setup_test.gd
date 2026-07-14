extends SceneTree


func _on_test_watchdog_timeout() -> void:
	push_error("Native first-run setup test did not finish within 10 seconds.")
	quit(1)


func _remove_tree(path: String) -> void:
	var directory := DirAccess.open(path)
	if directory != null:
		directory.list_dir_begin()
		var entry := directory.get_next()
		while not entry.is_empty():
			var entry_path := path.path_join(entry)
			if directory.current_is_dir():
				_remove_tree(entry_path)
			else:
				DirAccess.remove_absolute(entry_path)
			entry = directory.get_next()
		directory.list_dir_end()
	DirAccess.remove_absolute(path)


func _initialize() -> void:
	var watchdog := create_timer(10.0)
	watchdog.timeout.connect(_on_test_watchdog_timeout)
	var original_local_app_data := OS.get_environment("LOCALAPPDATA")
	var original_user_profile := OS.get_environment("USERPROFILE")
	var test_local_app_data := ProjectSettings.globalize_path(
		"res://.godot/first-run-setup-test-appdata",
	)
	OS.set_environment("LOCALAPPDATA", test_local_app_data)
	OS.set_environment("USERPROFILE", test_local_app_data.path_join("profile"))
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
	assert(
		setup.get_error_code() == "FEN-SETUP-MANIFEST-DOWNLOAD",
		"Expected manifest download failure, got %s" % setup.get_error_code(),
	)
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
	OS.set_environment("USERPROFILE", original_user_profile)
	setup.queue_free()
	_remove_tree(test_local_app_data)
	print("first-run setup state test passed")
	quit(0)
