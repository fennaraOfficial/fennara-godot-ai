@tool
extends RefCounted

const ImageSheet := preload("res://addons/fennara/runtime/image_sheet.gd")

var _inner
var root: Node:
	get:
		return _inner.root


func _init(inner) -> void:
	_inner = inner


func capture(
	nodes: Variant,
	options: Dictionary = {},
) -> Signal:
	return _inner.capture(nodes, options)


func output(
	image: Image,
	description: String = "",
) -> Dictionary:
	return _inner.output(image, description)


func sheet(
	images: Array,
	options: Dictionary = {},
) -> Array[Image]:
	var result: Dictionary = ImageSheet.compose(images, options)
	if not result.get("success", false):
		_inner.error(str(result.get("error", "ctx.sheet() failed.")))
		return []
	var sheets: Array[Image] = []
	for image: Variant in result.get("sheets", []):
		sheets.append(image as Image)
	return sheets


func log(message: String) -> void:
	_inner.log(message)


func error(message: String) -> void:
	_inner.error(message)
