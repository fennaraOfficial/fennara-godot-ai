extends RefCounted

const RuntimeQueryUtils := preload("res://addons/fennara/runtime/runtime_query_utils.gd")

var _helper: Node
var _scene_root_getter: Callable
var _query_utils


func _init(helper: Node, scene_root_getter: Callable) -> void:
	_helper = helper
	_scene_root_getter = scene_root_getter
	_query_utils = RuntimeQueryUtils.new(helper, scene_root_getter)


func raycast_3d(from_value: Variant, to_value: Variant, options: Dictionary = {}) -> Dictionary:
	var from_variant: Variant = _coerce_vector3(from_value)
	if from_variant == null:
		return _failure("raycast_3d from must be a Vector3 or Dictionary/Array with x/y/z.")
	var to_variant: Variant = _coerce_vector3(to_value)
	if to_variant == null:
		return _failure("raycast_3d to must be a Vector3 or Dictionary/Array with x/y/z.")

	var from: Vector3 = from_variant
	var to: Vector3 = to_variant
	var space := _space_state_3d(options)
	if space == null:
		return _failure("No 3D physics space is available for raycast_3d.")

	var query := PhysicsRayQueryParameters3D.create(from, to)
	_apply_3d_query_options(query, options)

	return _format_3d_result(space.intersect_ray(query), from, to)


func raycast_2d(from_value: Variant, to_value: Variant, options: Dictionary = {}) -> Dictionary:
	var from_variant: Variant = _coerce_vector2(from_value)
	if from_variant == null:
		return _failure("raycast_2d from must be a Vector2 or Dictionary/Array with x/y.")
	var to_variant: Variant = _coerce_vector2(to_value)
	if to_variant == null:
		return _failure("raycast_2d to must be a Vector2 or Dictionary/Array with x/y.")

	var from: Vector2 = from_variant
	var to: Vector2 = to_variant
	var space := _space_state_2d(options)
	if space == null:
		return _failure("No 2D physics space is available for raycast_2d.")

	var query := PhysicsRayQueryParameters2D.create(from, to)
	_apply_2d_query_options(query, options)

	return _format_2d_result(space.intersect_ray(query), from, to)


func raycast_2d_scan(options: Dictionary = {}) -> Dictionary:
	var origin_variant: Variant = _coerce_vector2(options.get("origin", options.get("from", null)))
	if origin_variant == null:
		return _failure("raycast_2d_scan origin must be a Vector2 or Dictionary/Array with x/y.")
	var distance := _scan_distance(options)
	if distance <= 0.0:
		return _failure("raycast_2d_scan distance/radius must be greater than zero.")

	var ray_spec := _scan_rays_2d(options)
	if not ray_spec.get("success", false):
		return _failure(str(ray_spec.get("error", "raycast_2d_scan ray setup failed.")))
	var rays: Array = ray_spec.get("rays", [])
	if rays.is_empty():
		return _failure("raycast_2d_scan produced no rays.")

	var query_options := _ray_options_from_scan(options)
	var space := _space_state_2d(query_options)
	if space == null:
		return _failure("No 2D physics space is available for raycast_2d_scan.")

	var origin: Vector2 = origin_variant
	var hits: Array[Dictionary] = []
	var matches: Array[Dictionary] = []
	var clear: Array[Dictionary] = []
	var best: Dictionary = {}
	var hit_count := 0
	var match_count := 0
	var clear_count := 0
	var max_results := maxi(0, int(options.get("max_results", 64)))
	var include_clear := bool(options.get("include_clear", false))

	for i in range(rays.size()):
		var ray: Dictionary = rays[i]
		var direction: Vector2 = ray.get("direction", Vector2.RIGHT)
		var to := origin + direction * distance
		var query := PhysicsRayQueryParameters2D.create(origin, to)
		_apply_2d_query_options(query, query_options)
		var hit := space.intersect_ray(query)
		if hit.is_empty():
			clear_count += 1
			if include_clear and clear.size() < max_results:
				clear.append(_format_clear_ray(i, ray, distance))
			continue

		hit_count += 1
		var receipt := _format_2d_scan_hit(hit, origin, to, i, ray, options)
		if hits.size() < max_results:
			hits.append(receipt)
		if bool(receipt.get("target_match", false)):
			match_count += 1
			if matches.size() < max_results:
				matches.append(receipt)
			best = _nearest_hit(best, receipt)

	return {
		"success": true,
		"mode": str(ray_spec.get("mode", "radial")),
		"origin": origin,
		"distance": distance,
		"rays_cast": rays.size(),
		"hits_count": hit_count,
		"matches_count": match_count,
		"clear_count": clear_count,
		"target_filter": _query_utils.target_filter_active(options),
		"hits": hits,
		"matches": matches,
		"clear": clear,
		"best": best,
		"omitted_hits": maxi(0, hit_count - hits.size()),
		"omitted_matches": maxi(0, match_count - matches.size()),
		"omitted_clear": maxi(0, clear_count - clear.size()),
	}


func raycast_3d_scan(options: Dictionary = {}) -> Dictionary:
	var origin_variant: Variant = _coerce_vector3(options.get("origin", options.get("from", null)))
	if origin_variant == null:
		return _failure("raycast_3d_scan origin must be a Vector3 or Dictionary/Array with x/y/z.")
	var distance := _scan_distance(options)
	if distance <= 0.0:
		return _failure("raycast_3d_scan distance/radius must be greater than zero.")

	var mode := str(options.get("mode", "plane")).to_lower()
	var ray_spec := _scan_rays_3d(options, mode)
	if not ray_spec.get("success", false):
		return _failure(str(ray_spec.get("error", "raycast_3d_scan ray setup failed.")))
	var rays: Array = ray_spec.get("rays", [])
	if rays.is_empty():
		return _failure("raycast_3d_scan produced no rays.")

	var query_options := _ray_options_from_scan(options)
	var space := _space_state_3d(query_options)
	if space == null:
		return _failure("No 3D physics space is available for raycast_3d_scan.")

	var origin: Vector3 = origin_variant
	var hits: Array[Dictionary] = []
	var matches: Array[Dictionary] = []
	var clear: Array[Dictionary] = []
	var best: Dictionary = {}
	var hit_count := 0
	var match_count := 0
	var clear_count := 0
	var max_results := maxi(0, int(options.get("max_results", 64)))
	var include_clear := bool(options.get("include_clear", false))

	for i in range(rays.size()):
		var ray: Dictionary = rays[i]
		var direction: Vector3 = ray.get("direction", Vector3.FORWARD)
		var to: Vector3 = origin + direction * distance
		var query := PhysicsRayQueryParameters3D.create(origin, to)
		_apply_3d_query_options(query, query_options)
		var hit := space.intersect_ray(query)
		if hit.is_empty():
			clear_count += 1
			if include_clear and clear.size() < max_results:
				clear.append(_format_clear_ray(i, ray, distance))
			continue

		hit_count += 1
		var receipt := _format_3d_scan_hit(hit, origin, to, i, ray, options)
		if hits.size() < max_results:
			hits.append(receipt)
		if bool(receipt.get("target_match", false)):
			match_count += 1
			if matches.size() < max_results:
				matches.append(receipt)
			best = _nearest_hit(best, receipt)

	return {
		"success": true,
		"mode": mode,
		"origin": origin,
		"distance": distance,
		"rays_cast": rays.size(),
		"hits_count": hit_count,
		"matches_count": match_count,
		"clear_count": clear_count,
		"target_filter": _query_utils.target_filter_active(options),
		"hits": hits,
		"matches": matches,
		"clear": clear,
		"best": best,
		"omitted_hits": maxi(0, hit_count - hits.size()),
		"omitted_matches": maxi(0, match_count - matches.size()),
		"omitted_clear": maxi(0, clear_count - clear.size()),
	}


func _space_state_3d(options: Dictionary) -> PhysicsDirectSpaceState3D:
	var node := _space_node(options)
	if node == null:
		node = _first_node3d(_get_scene_root())
	if node is Node3D:
		return (node as Node3D).get_world_3d().direct_space_state
	return null


func _space_state_2d(options: Dictionary) -> PhysicsDirectSpaceState2D:
	var node := _space_node(options)
	if node == null:
		node = _first_node2d(_get_scene_root())
	if node is Node2D:
		return (node as Node2D).get_world_2d().direct_space_state
	return null


func _space_node(options: Dictionary) -> Node:
	for key in ["space_node", "world_node"]:
		if options.has(key):
			return _resolve_node(options.get(key))
	return null


func _format_3d_result(hit: Dictionary, from: Vector3, to: Vector3) -> Dictionary:
	var result: Dictionary = {
		"success": true,
		"hit": not hit.is_empty(),
		"from": from,
		"to": to,
		"max_distance": from.distance_to(to),
	}
	if hit.is_empty():
		return result

	var position: Vector3 = hit.get("position", from)
	result.merge({
		"position": position,
		"normal": hit.get("normal", Vector3.ZERO),
		"distance": from.distance_to(position),
		"collider_path": _object_path_text(hit.get("collider", null), false),
		"collider_absolute_path": _object_path_text(hit.get("collider", null), true),
		"collider_name": _object_name(hit.get("collider", null)),
		"collider_class": _object_class(hit.get("collider", null)),
		"collider_instance_id": _object_instance_id(hit.get("collider", null)),
		"shape": int(hit.get("shape", -1)),
	}, true)
	return result


func _format_2d_result(hit: Dictionary, from: Vector2, to: Vector2) -> Dictionary:
	var result: Dictionary = {
		"success": true,
		"hit": not hit.is_empty(),
		"from": from,
		"to": to,
		"max_distance": from.distance_to(to),
	}
	if hit.is_empty():
		return result

	var position: Vector2 = hit.get("position", from)
	result.merge({
		"position": position,
		"normal": hit.get("normal", Vector2.ZERO),
		"distance": from.distance_to(position),
		"collider_path": _object_path_text(hit.get("collider", null), false),
		"collider_absolute_path": _object_path_text(hit.get("collider", null), true),
		"collider_name": _object_name(hit.get("collider", null)),
		"collider_class": _object_class(hit.get("collider", null)),
		"collider_instance_id": _object_instance_id(hit.get("collider", null)),
		"shape": int(hit.get("shape", -1)),
	}, true)
	return result


func _scan_distance(options: Dictionary) -> float:
	return maxf(0.0, float(options.get("distance", options.get("radius", 0.0))))


func _scan_rays_2d(options: Dictionary) -> Dictionary:
	var raw_directions: Variant = options.get("directions", [])
	if raw_directions is Array and not (raw_directions as Array).is_empty():
		var explicit_rays: Array[Dictionary] = []
		for item in raw_directions:
			var direction_variant: Variant = _coerce_vector2(item)
			if direction_variant == null:
				return {"success": false, "error": "raycast_2d_scan directions must contain Vector2 or x/y values."}
			var direction: Vector2 = direction_variant
			if direction.length() <= 0.0001:
				continue
			explicit_rays.append({"direction": direction.normalized()})
		return {"success": true, "mode": "directions", "rays": explicit_rays}

	var rays := _ray_count(options, 64)
	var start := _scan_start_angle(options)
	var arc := _scan_arc_angle(options, start)
	var full_circle := absf(absf(arc) - TAU) <= 0.0001
	var divisor := float(rays if full_circle else maxi(1, rays - 1))
	var generated: Array[Dictionary] = []

	for i in range(rays):
		var t := 0.0
		if rays > 1:
			t = float(i) / divisor
		var angle := start + arc * t
		var direction := Vector2(cos(angle), sin(angle)).normalized()
		generated.append({
			"direction": direction,
			"angle": angle,
			"angle_degrees": rad_to_deg(angle),
		})

	return {"success": true, "mode": "radial", "rays": generated}


func _scan_rays_3d(options: Dictionary, mode: String) -> Dictionary:
	match mode:
		"plane", "radial", "circle", "arc":
			return _scan_rays_3d_plane(options)
		"cone":
			return _scan_rays_3d_cone(options)
		_:
			return {"success": false, "error": "raycast_3d_scan mode must be plane or cone."}


func _scan_rays_3d_plane(options: Dictionary) -> Dictionary:
	var rays := _ray_count(options, 96)
	var plane := str(options.get("plane", "xz")).to_lower()
	var start := _scan_start_angle(options)
	var arc := _scan_arc_angle(options, start)
	var full_circle := absf(absf(arc) - TAU) <= 0.0001
	var divisor := float(rays if full_circle else maxi(1, rays - 1))
	var generated: Array[Dictionary] = []

	for i in range(rays):
		var t := 0.0
		if rays > 1:
			t = float(i) / divisor
		var angle := start + arc * t
		var direction_variant: Variant = _plane_direction(angle, plane)
		if direction_variant == null:
			return {"success": false, "error": "raycast_3d_scan plane must be xz, xy, or yz."}
		var direction: Vector3 = direction_variant
		generated.append({
			"direction": direction.normalized(),
			"angle": angle,
			"angle_degrees": rad_to_deg(angle),
			"plane": plane,
		})

	return {"success": true, "mode": "plane", "rays": generated}


func _scan_rays_3d_cone(options: Dictionary) -> Dictionary:
	var forward := Vector3.FORWARD
	var up := Vector3.UP
	var basis_node: Node = _resolve_node(options.get("basis_node", options.get("node", null)))
	if basis_node is Node3D:
		var node3d := basis_node as Node3D
		forward = -node3d.global_transform.basis.z
		up = node3d.global_transform.basis.y

	if options.has("forward"):
		var forward_variant: Variant = _coerce_vector3(options.get("forward"))
		if forward_variant == null:
			return {"success": false, "error": "raycast_3d_scan forward must be a Vector3 or x/y/z value."}
		forward = forward_variant
	if options.has("up"):
		var up_variant: Variant = _coerce_vector3(options.get("up"))
		if up_variant == null:
			return {"success": false, "error": "raycast_3d_scan up must be a Vector3 or x/y/z value."}
		up = up_variant

	if forward.length() <= 0.0001:
		return {"success": false, "error": "raycast_3d_scan cone forward vector must not be zero."}
	forward = forward.normalized()
	if up.length() <= 0.0001:
		up = Vector3.UP
	up = up.normalized()

	var right := forward.cross(up)
	if right.length() <= 0.0001:
		right = _fallback_right(forward)
	right = right.normalized()
	var true_up := right.cross(forward).normalized()

	var has_grid := options.has("columns") or options.has("rows") or options.has("horizontal_rays") or options.has("vertical_rays")
	var total_limit := int(options.get("rays", 0))
	var columns := int(options.get("columns", options.get("horizontal_rays", 0)))
	var rows := int(options.get("rows", options.get("vertical_rays", 0)))
	if columns <= 0 or rows <= 0:
		var total := _ray_count(options, 45)
		columns = maxi(1, int(ceil(sqrt(float(total)))))
		rows = maxi(1, int(ceil(float(total) / float(columns))))
		if not has_grid:
			total_limit = total

	var horizontal := deg_to_rad(float(options.get("horizontal_degrees", options.get("horizontal_fov_degrees", 60.0))))
	var vertical := deg_to_rad(float(options.get("vertical_degrees", options.get("vertical_fov_degrees", 30.0))))
	var generated: Array[Dictionary] = []

	for row in range(rows):
		if total_limit > 0 and generated.size() >= total_limit:
			break
		var pitch_t := 0.5
		if rows > 1:
			pitch_t = float(row) / float(rows - 1)
		var pitch := (pitch_t - 0.5) * vertical
		for column in range(columns):
			if total_limit > 0 and generated.size() >= total_limit:
				break
			var yaw_t := 0.5
			if columns > 1:
				yaw_t = float(column) / float(columns - 1)
			var yaw := (yaw_t - 0.5) * horizontal
			var direction := (forward + right * tan(yaw) + true_up * tan(pitch)).normalized()
			generated.append({
				"direction": direction,
				"yaw": yaw,
				"yaw_degrees": rad_to_deg(yaw),
				"pitch": pitch,
				"pitch_degrees": rad_to_deg(pitch),
				"row": row,
				"column": column,
			})

	return {"success": true, "mode": "cone", "rays": generated}


func _scan_start_angle(options: Dictionary) -> float:
	if options.has("start_angle"):
		return float(options.get("start_angle"))
	if options.has("from_angle"):
		return float(options.get("from_angle"))
	if options.has("start_degrees"):
		return deg_to_rad(float(options.get("start_degrees")))
	if options.has("from_degrees"):
		return deg_to_rad(float(options.get("from_degrees")))
	return 0.0


func _scan_arc_angle(options: Dictionary, start: float) -> float:
	if options.has("end_angle"):
		return float(options.get("end_angle")) - start
	if options.has("to_angle"):
		return float(options.get("to_angle")) - start
	if options.has("end_degrees"):
		return deg_to_rad(float(options.get("end_degrees"))) - start
	if options.has("to_degrees"):
		return deg_to_rad(float(options.get("to_degrees"))) - start
	if options.has("arc_angle"):
		return float(options.get("arc_angle"))
	if options.has("arc_degrees"):
		return deg_to_rad(float(options.get("arc_degrees")))
	return TAU


func _ray_count(options: Dictionary, fallback: int) -> int:
	return clampi(int(options.get("rays", fallback)), 1, 720)


func _plane_direction(angle: float, plane: String) -> Variant:
	match plane:
		"xz":
			return Vector3(cos(angle), 0.0, sin(angle))
		"xy":
			return Vector3(cos(angle), sin(angle), 0.0)
		"yz":
			return Vector3(0.0, cos(angle), sin(angle))
		_:
			return null


func _fallback_right(forward: Vector3) -> Vector3:
	var axis := Vector3.UP
	if absf(forward.dot(axis)) > 0.95:
		axis = Vector3.RIGHT
	return forward.cross(axis).normalized()


func _ray_options_from_scan(options: Dictionary) -> Dictionary:
	var query_options := options.duplicate(true)
	for key in [
		"origin", "from", "distance", "radius", "rays", "directions", "max_results", "include_clear",
		"start_angle", "from_angle", "end_angle", "to_angle", "arc_angle",
		"start_degrees", "from_degrees", "end_degrees", "to_degrees", "arc_degrees",
		"target", "target_path", "target_instance_id", "target_group", "target_name", "target_class",
		"mode", "plane", "basis_node", "node", "forward", "up", "columns", "rows",
		"horizontal_rays", "vertical_rays", "horizontal_degrees", "vertical_degrees",
		"horizontal_fov_degrees", "vertical_fov_degrees",
	]:
		query_options.erase(key)
	return query_options


func _apply_3d_query_options(query: PhysicsRayQueryParameters3D, options: Dictionary) -> void:
	query.collide_with_areas = bool(options.get("collide_with_areas", true))
	query.collide_with_bodies = bool(options.get("collide_with_bodies", true))
	if options.has("collision_mask"):
		query.collision_mask = int(options.get("collision_mask", query.collision_mask))
	if options.has("hit_from_inside"):
		query.hit_from_inside = bool(options.get("hit_from_inside", query.hit_from_inside))
	query.exclude = _exclude_rids(options.get("exclude", []))


func _apply_2d_query_options(query: PhysicsRayQueryParameters2D, options: Dictionary) -> void:
	query.collide_with_areas = bool(options.get("collide_with_areas", true))
	query.collide_with_bodies = bool(options.get("collide_with_bodies", true))
	if options.has("collision_mask"):
		query.collision_mask = int(options.get("collision_mask", query.collision_mask))
	if options.has("hit_from_inside"):
		query.hit_from_inside = bool(options.get("hit_from_inside", query.hit_from_inside))
	query.exclude = _exclude_rids(options.get("exclude", []))


func _format_2d_scan_hit(hit: Dictionary, origin: Vector2, to: Vector2, index: int, ray: Dictionary, options: Dictionary) -> Dictionary:
	var position: Vector2 = hit.get("position", origin)
	var receipt: Dictionary = {
		"index": index,
		"target_match": _query_utils.matches_target(hit.get("collider", null), options),
		"direction": ray.get("direction", Vector2.RIGHT),
		"position": position,
		"normal": hit.get("normal", Vector2.ZERO),
		"distance": origin.distance_to(position),
		"max_distance": origin.distance_to(to),
		"collider_path": _object_path_text(hit.get("collider", null), false),
		"collider_absolute_path": _object_path_text(hit.get("collider", null), true),
		"collider_name": _object_name(hit.get("collider", null)),
		"collider_class": _object_class(hit.get("collider", null)),
		"collider_instance_id": _object_instance_id(hit.get("collider", null)),
		"shape": int(hit.get("shape", -1)),
	}
	_merge_ray_metadata(receipt, ray)
	return receipt


func _format_3d_scan_hit(hit: Dictionary, origin: Vector3, to: Vector3, index: int, ray: Dictionary, options: Dictionary) -> Dictionary:
	var position: Vector3 = hit.get("position", origin)
	var receipt: Dictionary = {
		"index": index,
		"target_match": _query_utils.matches_target(hit.get("collider", null), options),
		"direction": ray.get("direction", Vector3.FORWARD),
		"position": position,
		"normal": hit.get("normal", Vector3.ZERO),
		"distance": origin.distance_to(position),
		"max_distance": origin.distance_to(to),
		"collider_path": _object_path_text(hit.get("collider", null), false),
		"collider_absolute_path": _object_path_text(hit.get("collider", null), true),
		"collider_name": _object_name(hit.get("collider", null)),
		"collider_class": _object_class(hit.get("collider", null)),
		"collider_instance_id": _object_instance_id(hit.get("collider", null)),
		"shape": int(hit.get("shape", -1)),
	}
	_merge_ray_metadata(receipt, ray)
	return receipt


func _format_clear_ray(index: int, ray: Dictionary, max_distance: float) -> Dictionary:
	var receipt: Dictionary = {
		"index": index,
		"direction": ray.get("direction", Vector3.FORWARD),
		"max_distance": max_distance,
	}
	_merge_ray_metadata(receipt, ray)
	return receipt


func _merge_ray_metadata(receipt: Dictionary, ray: Dictionary) -> void:
	for key in ray.keys():
		if str(key) == "direction":
			continue
		receipt[key] = ray[key]


func _nearest_hit(best: Dictionary, candidate: Dictionary) -> Dictionary:
	if best.is_empty():
		return candidate
	if float(candidate.get("distance", INF)) < float(best.get("distance", INF)):
		return candidate
	return best


func _exclude_rids(value: Variant) -> Array[RID]:
	var rids: Array[RID] = []
	if value == null:
		return rids

	var values: Array = value if value is Array else [value]
	for item in values:
		var rid := _rid_from_value(item)
		if rid.is_valid():
			rids.append(rid)
	return rids


func _rid_from_value(value: Variant) -> RID:
	if value is RID:
		return value
	if value is Node:
		return _rid_from_node(value)
	if value is NodePath or value is String:
		var node := _resolve_node(value)
		return _rid_from_node(node)
	return RID()


func _rid_from_node(node: Variant) -> RID:
	if node == null or not node is Object or not is_instance_valid(node):
		return RID()
	var object := node as Object
	if not object.has_method("get_rid"):
		return RID()
	var rid_value: Variant = object.call("get_rid")
	if rid_value is RID:
		return rid_value
	return RID()


func _coerce_vector3(value: Variant) -> Variant:
	return _query_utils.coerce_vector3(value)


func _coerce_vector2(value: Variant) -> Variant:
	return _query_utils.coerce_vector2(value)


func _resolve_node(node_or_path: Variant) -> Node:
	return _query_utils.resolve_node(node_or_path)


func _get_scene_root() -> Node:
	return _query_utils.get_scene_root()


func _first_node3d(node: Node) -> Node3D:
	return _query_utils.first_node3d(node)


func _first_node2d(node: Node) -> Node2D:
	return _query_utils.first_node2d(node)


func _object_path_text(value: Variant, absolute: bool) -> String:
	return _query_utils.object_path_text(value, absolute)


func _node_path_text(node: Node) -> String:
	return _query_utils.node_path_text(node)


func _object_name(value: Variant) -> String:
	return _query_utils.object_name(value)


func _object_class(value: Variant) -> String:
	return _query_utils.object_class(value)


func _object_instance_id(value: Variant) -> int:
	return _query_utils.object_instance_id(value)


func _failure(message: String) -> Dictionary:
	return {
		"success": false,
		"hit": false,
		"error": message,
	}
