extends RefCounted

var _helper: Node
var _pressed_keys: Dictionary = {}
var _pressed_mouse_buttons: Dictionary = {}


func _init(helper: Node) -> void:
	_helper = helper


func input_event(event_class: String, properties: Dictionary = {}) -> Dictionary:
	if event_class.strip_edges().is_empty():
		return {"success": false, "error": "Input event class name is required."}
	if not ClassDB.class_exists(event_class):
		return {"success": false, "error": "Input event class does not exist: %s" % event_class}

	var event: Variant = ClassDB.instantiate(event_class)
	if event == null:
		return {"success": false, "error": "Could not instantiate input event: %s" % event_class}
	if not event is InputEvent:
		return {"success": false, "error": "Class is not an InputEvent: %s" % event_class}

	var input_event_value := event as InputEvent
	var unknown: Array[String] = _apply_properties(input_event_value, properties)
	if not unknown.is_empty():
		return {
			"success": false,
			"error": "Unknown input event properties for %s: %s" % [event_class, ", ".join(unknown)],
			"unknown_properties": unknown,
		}

	Input.parse_input_event(input_event_value)
	_track_event(input_event_value)
	return {"success": true, "class_name": event_class}


func press_key(keycode: int, options: Dictionary = {}) -> bool:
	var data := _key_data(keycode, options)
	var event := _make_key_event(data, true)
	Input.parse_input_event(event)
	_pressed_keys[_key_signature(data)] = data
	return true


func release_key(keycode: int, options: Dictionary = {}) -> bool:
	var data := _key_data(keycode, options)
	var event := _make_key_event(data, false)
	Input.parse_input_event(event)
	_pressed_keys.erase(_key_signature(data))
	return true


func press_mouse(button: int, options: Dictionary = {}) -> bool:
	var data := _mouse_button_data(button, options)
	var event := _make_mouse_button_event(data, true)
	Input.parse_input_event(event)
	_pressed_mouse_buttons[str(button)] = data
	return true


func release_mouse(button: int, options: Dictionary = {}) -> bool:
	var data := _mouse_button_data(button, options)
	var event := _make_mouse_button_event(data, false)
	Input.parse_input_event(event)
	_pressed_mouse_buttons.erase(str(button))
	return true


func mouse_motion_to(position: Vector2, options: Dictionary = {}) -> void:
	var event := InputEventMouseMotion.new()
	event.position = position
	event.global_position = position
	event.button_mask = int(options.get("button_mask", current_mouse_button_mask()))
	Input.parse_input_event(event)


func mouse_motion_relative(delta: Vector2, options: Dictionary = {}) -> void:
	var current := _current_mouse_position()
	var next := current + delta
	var event := InputEventMouseMotion.new()
	event.position = next
	event.global_position = next
	event.relative = delta
	_set_property_if_available(event, "screen_relative", delta)
	event.button_mask = int(options.get("button_mask", current_mouse_button_mask()))

	var duration := float(options.get("step_duration", 0.0))
	if duration > 0.0:
		event.velocity = delta / duration
		_set_property_if_available(event, "screen_velocity", event.velocity)

	Input.parse_input_event(event)


func motion_deltas(delta: Vector2, steps: int, profile: String = "linear") -> Array[Vector2]:
	var count := maxi(1, steps)
	var normalized_profile := profile.strip_edges().to_lower()
	var deltas: Array[Vector2] = []

	match normalized_profile:
		"ease_in_out":
			var previous := 0.0
			for i in range(count):
				var t := float(i + 1) / float(count)
				var eased := _ease_in_out(t)
				deltas.append(delta * (eased - previous))
				previous = eased
		_:
			var step_delta := delta / float(count)
			for _i in range(count):
				deltas.append(step_delta)

	return deltas


func current_mouse_button_mask() -> int:
	var mask := 0
	for key in _pressed_mouse_buttons.keys():
		mask |= _mouse_button_mask(int(key))
	return mask


func release_all() -> Dictionary:
	var released_keys := 0
	var released_mouse_buttons := 0

	for key_data in _pressed_keys.values():
		if key_data is Dictionary:
			var key_event := _make_key_event(key_data, false)
			Input.parse_input_event(key_event)
			released_keys += 1
	_pressed_keys.clear()

	for mouse_data in _pressed_mouse_buttons.values():
		if mouse_data is Dictionary:
			var mouse_event := _make_mouse_button_event(mouse_data, false)
			Input.parse_input_event(mouse_event)
			released_mouse_buttons += 1
	_pressed_mouse_buttons.clear()

	return {
		"released_keys": released_keys,
		"released_mouse_buttons": released_mouse_buttons,
	}


func _apply_properties(event: InputEvent, properties: Dictionary) -> Array[String]:
	var known := _property_names(event)
	var unknown: Array[String] = []
	for raw_name in properties.keys():
		var name := str(raw_name)
		if not known.has(name):
			unknown.append(name)
			continue
		event.set(name, properties[raw_name])
	return unknown


func _property_names(object: Object) -> Dictionary:
	var names := {}
	for property_info in object.get_property_list():
		if property_info is Dictionary:
			var name := str(property_info.get("name", ""))
			if not name.is_empty():
				names[name] = true
	return names


func _track_event(event: InputEvent) -> void:
	if event is InputEventKey:
		var key_event := event as InputEventKey
		var data := _key_data_from_event(key_event)
		var signature := _key_signature(data)
		if key_event.pressed:
			_pressed_keys[signature] = data
		else:
			_pressed_keys.erase(signature)
	elif event is InputEventMouseButton:
		var mouse_event := event as InputEventMouseButton
		var button := int(mouse_event.button_index)
		if mouse_event.pressed:
			_pressed_mouse_buttons[str(button)] = _mouse_button_data_from_event(mouse_event)
		else:
			_pressed_mouse_buttons.erase(str(button))


func _key_data(keycode: int, options: Dictionary = {}) -> Dictionary:
	var physical := bool(options.get("physical", true))
	var data := {
		"keycode": keycode,
		"physical": physical,
		"key_label": int(options.get("key_label", 0)),
		"location": int(options.get("location", 0)),
		"unicode": int(options.get("unicode", 0)),
		"echo": bool(options.get("echo", false)),
		"modifiers": _modifier_options(options),
	}
	return data


func _key_data_from_event(event: InputEventKey) -> Dictionary:
	var keycode := int(event.physical_keycode)
	var physical := true
	if keycode == 0:
		keycode = int(event.keycode)
		physical = false
	return {
		"keycode": keycode,
		"physical": physical,
		"key_label": int(event.key_label),
		"location": int(event.location),
		"unicode": int(event.unicode),
		"echo": bool(event.echo),
		"modifiers": _modifier_options_from_event(event),
	}


func _make_key_event(data: Dictionary, pressed: bool) -> InputEventKey:
	var event := InputEventKey.new()
	event.pressed = pressed
	event.echo = bool(data.get("echo", false))
	if bool(data.get("physical", true)):
		event.physical_keycode = int(data.get("keycode", 0))
	else:
		event.keycode = int(data.get("keycode", 0))

	var key_label := int(data.get("key_label", 0))
	if key_label != 0:
		event.key_label = key_label

	var location := int(data.get("location", 0))
	if location != 0:
		event.location = location

	var unicode_value := int(data.get("unicode", 0))
	if unicode_value != 0 and pressed:
		event.unicode = unicode_value

	_apply_modifiers(event, data.get("modifiers", {}))
	return event


func _modifier_options(options: Dictionary) -> Dictionary:
	var modifiers: Dictionary = {}
	var nested: Variant = options.get("modifiers", {})
	if nested is Dictionary:
		modifiers = nested.duplicate(true)

	for name in [
		"shift_pressed",
		"ctrl_pressed",
		"alt_pressed",
		"meta_pressed",
		"command_or_control_autoremap",
		"shift",
		"ctrl",
		"alt",
		"meta",
	]:
		if options.has(name):
			modifiers[name] = options[name]
	return modifiers


func _modifier_options_from_event(event: InputEventWithModifiers) -> Dictionary:
	return {
		"shift_pressed": _get_bool_property(event, "shift_pressed"),
		"ctrl_pressed": _get_bool_property(event, "ctrl_pressed"),
		"alt_pressed": _get_bool_property(event, "alt_pressed"),
		"meta_pressed": _get_bool_property(event, "meta_pressed"),
		"command_or_control_autoremap": _get_bool_property(event, "command_or_control_autoremap"),
	}


func _apply_modifiers(event: InputEventWithModifiers, modifiers_value: Variant) -> void:
	if not modifiers_value is Dictionary:
		return
	var modifiers := modifiers_value as Dictionary
	event.set("shift_pressed", bool(modifiers.get("shift_pressed", modifiers.get("shift", false))))
	event.set("ctrl_pressed", bool(modifiers.get("ctrl_pressed", modifiers.get("ctrl", false))))
	event.set("alt_pressed", bool(modifiers.get("alt_pressed", modifiers.get("alt", false))))
	event.set("meta_pressed", bool(modifiers.get("meta_pressed", modifiers.get("meta", false))))
	_set_property_if_available(event, "command_or_control_autoremap", bool(modifiers.get("command_or_control_autoremap", false)))


func _key_signature(data: Dictionary) -> String:
	var modifiers: Dictionary = data.get("modifiers", {})
	return "%d:%s:%d:%d:%d:%s:%s:%s:%s:%s" % [
		int(data.get("keycode", 0)),
		str(bool(data.get("physical", true))),
		int(data.get("key_label", 0)),
		int(data.get("location", 0)),
		int(data.get("unicode", 0)),
		str(bool(modifiers.get("shift_pressed", modifiers.get("shift", false)))),
		str(bool(modifiers.get("ctrl_pressed", modifiers.get("ctrl", false)))),
		str(bool(modifiers.get("alt_pressed", modifiers.get("alt", false)))),
		str(bool(modifiers.get("meta_pressed", modifiers.get("meta", false)))),
		str(bool(modifiers.get("command_or_control_autoremap", false))),
	]


func _mouse_button_data(button: int, options: Dictionary = {}) -> Dictionary:
	var position := _current_mouse_position()
	if options.has("position"):
		var maybe_position := _coerce_vector2(options.get("position"))
		if maybe_position != null:
			position = maybe_position as Vector2

	return {
		"button": button,
		"position": position,
		"factor": float(options.get("factor", 1.0)),
		"double_click": bool(options.get("double_click", false)),
	}


func _mouse_button_data_from_event(event: InputEventMouseButton) -> Dictionary:
	return {
		"button": int(event.button_index),
		"position": event.position,
		"factor": float(event.factor),
		"double_click": bool(event.double_click),
	}


func _make_mouse_button_event(data: Dictionary, pressed: bool) -> InputEventMouseButton:
	var event := InputEventMouseButton.new()
	var button := int(data.get("button", MOUSE_BUTTON_LEFT))
	var position: Vector2 = data.get("position", _current_mouse_position())
	event.button_index = button
	event.position = position
	event.global_position = position
	event.factor = float(data.get("factor", 1.0))
	event.double_click = bool(data.get("double_click", false))
	event.pressed = pressed
	event.button_mask = _mouse_mask_after(button, pressed)
	return event


func _mouse_mask_after(button: int, pressed: bool) -> int:
	var mask := current_mouse_button_mask()
	var button_mask := _mouse_button_mask(button)
	if pressed:
		return mask | button_mask
	return mask & ~button_mask


func _mouse_button_mask(button: int) -> int:
	if button <= 0:
		return 0
	return 1 << (button - 1)


func _current_mouse_position() -> Vector2:
	if _helper != null and _helper.is_inside_tree():
		return _helper.get_tree().root.get_mouse_position()
	return Vector2.ZERO


func _coerce_vector2(value: Variant) -> Variant:
	if value is Vector2:
		return value
	if value is Dictionary and value.has("x") and value.has("y"):
		return Vector2(float(value.get("x")), float(value.get("y")))
	if value is Array and value.size() >= 2:
		return Vector2(float(value[0]), float(value[1]))
	return null


func _ease_in_out(t: float) -> float:
	var clamped := clampf(t, 0.0, 1.0)
	return clamped * clamped * (3.0 - 2.0 * clamped)


func _set_property_if_available(object: Object, property_name: String, value: Variant) -> void:
	if _property_names(object).has(property_name):
		object.set(property_name, value)


func _get_bool_property(object: Object, property_name: String) -> bool:
	if not _property_names(object).has(property_name):
		return false
	return bool(object.get(property_name))
