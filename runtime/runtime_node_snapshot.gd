extends RefCounted

var _helper: Node
var _scene_root_getter: Callable


func _init(helper: Node, scene_root_getter: Callable) -> void:
	_helper = helper
	_scene_root_getter = scene_root_getter


func node(node_or_path: Variant) -> Node:
	return _resolve_node(node_or_path)


func exists(node_or_path: Variant) -> bool:
	var resolved := _resolve_node(node_or_path)
	return resolved != null and is_instance_valid(resolved) and resolved.is_inside_tree()


func snapshot(spec: Dictionary) -> Dictionary:
	var result: Dictionary = {}
	for raw_key in spec.keys():
		var key := str(raw_key)
		var raw_entry: Variant = spec[raw_key]
		var entry_spec := _normalize_entry_spec(raw_entry, key)
		if not bool(entry_spec.get("success", true)):
			result[key] = entry_spec
			continue

		var path := str(entry_spec.get("path", key))
		var resolved := _resolve_node(path)
		result[key] = _snapshot_node(resolved, path, entry_spec)
	return result


func _normalize_entry_spec(raw_entry: Variant, fallback_path: String) -> Dictionary:
	if raw_entry is Dictionary:
		var entry := (raw_entry as Dictionary).duplicate(true)
		if not entry.has("path"):
			entry["path"] = fallback_path
		return entry
	if raw_entry is String or raw_entry is NodePath:
		return {"path": str(raw_entry)}
	return {
		"success": false,
		"exists": false,
		"path": fallback_path,
		"error": "snapshot entry must be a Dictionary, String, or NodePath",
	}


func _snapshot_node(resolved: Node, requested_path: String, options: Dictionary) -> Dictionary:
	var entry: Dictionary = {
		"exists": false,
		"requested_path": requested_path,
	}
	if resolved == null or not is_instance_valid(resolved) or not resolved.is_inside_tree():
		return entry

	entry.merge({
		"exists": true,
		"path": _node_path_text(resolved),
		"absolute_path": str(resolved.get_path()),
		"name": resolved.name,
		"class": resolved.get_class(),
		"instance_id": int(resolved.get_instance_id()),
	}, true)

	var props := _read_properties(resolved, options.get("props", []))
	if not props.get("values", {}).is_empty() or not props.get("missing", []).is_empty():
		entry["props"] = props.get("values", {})
		if not props.get("missing", []).is_empty():
			entry["missing_props"] = props.get("missing", [])

	if bool(options.get("children", false)):
		entry["children"] = _snapshot_children(resolved, options)

	return entry


func _snapshot_children(parent: Node, options: Dictionary) -> Array[Dictionary]:
	var children: Array[Dictionary] = []
	var max_children := maxi(0, int(options.get("max_children", 64)))
	var child_props := _string_array(options.get("child_props", []))
	var count := 0
	for child in parent.get_children():
		if count >= max_children:
			break
		var child_options: Dictionary = {"props": child_props}
		children.append(_snapshot_node(child, _node_path_text(child), child_options))
		count += 1
	if parent.get_child_count() > max_children:
		children.append({
			"exists": false,
			"truncated": true,
			"remaining_children": parent.get_child_count() - max_children,
		})
	return children


func _read_properties(target: Object, props_value: Variant) -> Dictionary:
	var values: Dictionary = {}
	var missing: Array[String] = []
	var requested := _string_array(props_value)
	if requested.is_empty():
		return {"values": values, "missing": missing}

	var known := _property_names(target)
	for property_name in requested:
		if known.has(property_name):
			values[property_name] = target.get(property_name)
		else:
			missing.append(property_name)
	return {"values": values, "missing": missing}


func _property_names(target: Object) -> Dictionary:
	var names: Dictionary = {}
	for property_info in target.get_property_list():
		if property_info is Dictionary:
			var name := str(property_info.get("name", ""))
			if not name.is_empty():
				names[name] = true
	return names


func _string_array(value: Variant) -> Array[String]:
	var result: Array[String] = []
	if value is String or value is StringName:
		result.append(str(value))
	elif value is Array:
		for item in value:
			result.append(str(item))
	return result


func _resolve_node(node_or_path: Variant) -> Node:
	if node_or_path is Object and not is_instance_valid(node_or_path):
		return null
	if node_or_path is Node:
		return node_or_path
	var path := str(node_or_path)
	if path.strip_edges().is_empty():
		return null
	var root := _get_scene_root()
	var tree := _get_scene_tree()
	if path.begins_with("/root/"):
		return tree.root.get_node_or_null(NodePath(path.trim_prefix("/root/"))) if tree != null and tree.root != null else null
	if path.begins_with("/"):
		return tree.root.get_node_or_null(NodePath(path.trim_prefix("/"))) if tree != null and tree.root != null else null
	if root != null:
		return root.get_node_or_null(NodePath(path))
	return null


func _get_scene_root() -> Node:
	if _scene_root_getter.is_valid():
		var root_value: Variant = _scene_root_getter.call()
		if root_value is Object and not is_instance_valid(root_value):
			return null
		if root_value is Node:
			return root_value
	var tree := _get_scene_tree()
	if tree != null:
		if tree.current_scene != null and is_instance_valid(tree.current_scene):
			return tree.current_scene
		if tree.root != null and is_instance_valid(tree.root):
			return tree.root
	return null


func _get_scene_tree() -> SceneTree:
	if _helper != null and is_instance_valid(_helper) and _helper.is_inside_tree():
		return _helper.get_tree()
	return null


func _node_path_text(target: Node) -> String:
	if target == null or not is_instance_valid(target):
		return ""
	var root := _get_scene_root()
	if target == root:
		return "."
	if root != null and root.is_ancestor_of(target):
		return str(root.get_path_to(target))
	return str(target.get_path())
