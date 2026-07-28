extends SceneTree


func _initialize() -> void:
	var extension := load("res://addons/fennara/fennara.gdextension")
	assert(extension != null)

	var result: Dictionary = ClassDB.class_call_static(
		"FennaraGetNodePropertiesTool",
		"execute",
		{
			"targets": [
				{
					"scene_path": "res://tests/fixtures/animation_tree_external.tscn",
					"node_path": "AnimationTree",
				},
			],
		},
	)
	assert(result.get("success", false))

	var nodes: Array = result.get("nodes", [])
	assert(nodes.size() == 1)
	var properties_text := str((nodes[0] as Dictionary).get("properties_text", ""))
	assert("tree_root = <AnimationTreeGraph>" in properties_text)
	assert("idle -> walk" in properties_text)
	assert("advance_mode:2" in properties_text)
	assert("advance_condition:go" in properties_text)
	assert("switch_mode:1" in properties_text)
	assert("<AnimationNodeStateMachineTransition>" not in properties_text)

	print("get_node_properties AnimationTree test passed")
	quit()
