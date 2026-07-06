extends RefCounted

var _helper: Node
var _scene_root_getter: Callable


func _init(helper: Node, scene_root_getter: Callable) -> void:
	_helper = helper
	_scene_root_getter = scene_root_getter


func coerce_vector3(value: Variant) -> Variant:
	if value is Vector3:
		return value
	if value is Dictionary and value.has("x") and value.has("y") and value.has("z"):
		return Vector3(float(value.get("x")), float(value.get("y")), float(value.get("z")))
	if value is Array and value.size() >= 3:
		return Vector3(float(value[0]), float(value[1]), float(value[2]))
	return null


func coerce_vector2(value: Variant) -> Variant:
	if value is Vector2:
		return value
	if value is Dictionary and value.has("x") and value.has("y"):
		return Vector2(float(value.get("x")), float(value.get("y")))
	if value is Array and value.size() >= 2:
		return Vector2(float(value[0]), float(value[1]))
	return null


func resolve_node(node_or_path: Variant) -> Node:
	if node_or_path is Object and not is_instance_valid(node_or_path):
		return null
	if node_or_path is Node:
		return node_or_path

	var path := str(node_or_path)
	if path.strip_edges().is_empty():
		return null

	var root := get_scene_root()
	if path.begins_with("/root/"):
		return _helper.get_tree().root.get_node_or_null(NodePath(path.trim_prefix("/root/")))
	if path.begins_with("/"):
		return _helper.get_tree().root.get_node_or_null(NodePath(path.trim_prefix("/")))
	if root != null:
		return root.get_node_or_null(NodePath(path))
	return null


func get_scene_root() -> Node:
	if _scene_root_getter.is_valid():
		var root_value: Variant = _scene_root_getter.call()
		if root_value is Object and not is_instance_valid(root_value):
			return null
		if root_value is Node:
			return root_value
	if _helper != null and is_instance_valid(_helper) and _helper.is_inside_tree():
		var tree := _helper.get_tree()
		if tree.current_scene != null:
			return tree.current_scene
		return tree.root
	return null


func first_node3d(node: Node) -> Node3D:
	if node == null or not is_instance_valid(node):
		return null
	if node is Node3D:
		return node as Node3D
	for child in node.get_children():
		var found := first_node3d(child)
		if found != null:
			return found
	return null


func first_node2d(node: Node) -> Node2D:
	if node == null or not is_instance_valid(node):
		return null
	if node is Node2D:
		return node as Node2D
	for child in node.get_children():
		var found := first_node2d(child)
		if found != null:
			return found
	return null


func object_path_text(value: Variant, absolute: bool) -> String:
	if not value is Node or not is_instance_valid(value):
		return ""
	var node := value as Node
	if absolute:
		return str(node.get_path())
	return node_path_text(node)


func node_path_text(node: Node) -> String:
	if node == null or not is_instance_valid(node):
		return ""
	var root := get_scene_root()
	if node == root:
		return "."
	if root != null and root.is_ancestor_of(node):
		return str(root.get_path_to(node))
	return str(node.get_path())


func object_name(value: Variant) -> String:
	if value is Node and is_instance_valid(value):
		return (value as Node).name
	return ""


func object_class(value: Variant) -> String:
	if value is Object and is_instance_valid(value):
		return (value as Object).get_class()
	return ""


func object_instance_id(value: Variant) -> int:
	if value is Object and is_instance_valid(value):
		return int((value as Object).get_instance_id())
	return 0


func target_filter_active(options: Dictionary) -> bool:
	for key in ["target", "target_path", "target_instance_id", "target_group", "target_name", "target_class"]:
		if not options.has(key):
			continue
		var value: Variant = options.get(key)
		if value == null:
			continue
		if value is int and int(value) == 0:
			continue
		if str(value).strip_edges().is_empty():
			continue
		return true
	return false


func matches_target(value: Variant, options: Dictionary) -> bool:
	if not target_filter_active(options):
		return true
	if value == null or not value is Object or not is_instance_valid(value):
		return false

	var object := value as Object
	if options.has("target_instance_id") and int(options.get("target_instance_id", 0)) == int(object.get_instance_id()):
		return true

	if options.has("target"):
		var target_value: Variant = options.get("target")
		if target_value is Object and is_instance_valid(target_value) and target_value == object:
			return true

	if value is Node:
		var node := value as Node
		var target_node := _target_node(options)
		if target_node != null and (node == target_node or target_node.is_ancestor_of(node)):
			return true
		if options.has("target_path") and _node_or_ancestor_matches_path(node, str(options.get("target_path", ""))):
			return true
		if options.has("target_group") and _node_or_ancestor_in_group(node, str(options.get("target_group", ""))):
			return true
		if options.has("target_name") and _node_or_ancestor_has_name(node, str(options.get("target_name", ""))):
			return true

	if options.has("target_class") and str(options.get("target_class", "")) == object.get_class():
		return true

	return false


func _target_node(options: Dictionary) -> Node:
	for key in ["target", "target_path"]:
		if options.has(key):
			var node := resolve_node(options.get(key))
			if node != null:
				return node
	return null


func _node_or_ancestor_matches_path(node: Node, path: String) -> bool:
	path = path.strip_edges()
	if path.is_empty():
		return false
	return _walk_node_or_ancestor(node, func(current: Node) -> bool:
		return node_path_text(current) == path or str(current.get_path()) == path
	)


func _node_or_ancestor_in_group(node: Node, group: String) -> bool:
	group = group.strip_edges()
	if group.is_empty():
		return false
	return _walk_node_or_ancestor(node, func(current: Node) -> bool:
		return current.is_in_group(group)
	)


func _node_or_ancestor_has_name(node: Node, target_name: String) -> bool:
	target_name = target_name.strip_edges()
	if target_name.is_empty():
		return false
	return _walk_node_or_ancestor(node, func(current: Node) -> bool:
		return current.name == target_name
	)


func _walk_node_or_ancestor(node: Node, predicate: Callable) -> bool:
	if node == null or not is_instance_valid(node):
		return false
	var root := get_scene_root()
	var current: Node = node
	while current != null and is_instance_valid(current):
		if bool(predicate.call(current)):
			return true
		if current == root:
			break
		current = current.get_parent()
	return false
