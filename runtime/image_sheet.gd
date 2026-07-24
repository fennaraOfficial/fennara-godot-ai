extends RefCounted

const ImageLabel := preload("res://addons/fennara/runtime/image_label.gd")
const DEFAULT_CELL_SIZE := Vector2i(384, 216)


static func compose(images: Array, options: Dictionary = {}) -> Dictionary:
	if images.is_empty():
		return _error("ctx.sheet() expects at least one Image.")

	for value: Variant in images:
		if not value is Image or (value as Image).is_empty():
			return _error("Every ctx.sheet() entry must be a non-empty Image.")

	var columns := int(options.get("columns", ceili(sqrt(float(images.size())))))
	if columns <= 0:
		return _error("ctx.sheet() columns must be greater than zero.")

	var cell_size_value: Variant = options.get("cell_size", DEFAULT_CELL_SIZE)
	if not cell_size_value is Vector2i:
		return _error("ctx.sheet() cell_size must be a Vector2i.")
	var cell_size := cell_size_value as Vector2i
	if cell_size.x <= 0 or cell_size.y <= 0:
		return _error("ctx.sheet() cell_size components must be greater than zero.")

	var cells_per_sheet := int(options.get("cells_per_sheet", images.size()))
	if cells_per_sheet <= 0:
		return _error("ctx.sheet() cells_per_sheet must be greater than zero.")

	var gap := int(options.get("gap", 0))
	if gap < 0:
		return _error("ctx.sheet() gap cannot be negative.")

	var fit := str(options.get("fit", "contain")).to_lower()
	if fit not in ["contain", "cover", "stretch"]:
		return _error("ctx.sheet() fit must be contain, cover, or stretch.")

	var labels_value: Variant = options.get("labels", [])
	if not labels_value is Array:
		return _error("ctx.sheet() labels must be an Array.")
	var labels := labels_value as Array

	var background_value: Variant = options.get(
		"background",
		Color(0.02, 0.02, 0.025, 1.0),
	)
	if not background_value is Color:
		return _error("ctx.sheet() background must be a Color.")
	var background := background_value as Color

	var label_scale := float(options.get("label_scale", 1.0))
	if label_scale <= 0.0:
		return _error("ctx.sheet() label_scale must be greater than zero.")

	var sheets: Array[Image] = []
	for start: int in range(0, images.size(), cells_per_sheet):
		var count := mini(cells_per_sheet, images.size() - start)
		var rows := ceili(float(count) / float(columns))
		var sheet_size := Vector2i(
			cell_size.x * columns + gap * maxi(0, columns - 1),
			cell_size.y * rows + gap * maxi(0, rows - 1),
		)
		var sheet := Image.create(
			sheet_size.x,
			sheet_size.y,
			false,
			Image.FORMAT_RGBA8,
		)
		sheet.fill(background)

		for local_index: int in range(count):
			var source_index := start + local_index
			var cell_origin := Vector2i(
				(local_index % columns) * (cell_size.x + gap),
				floori(float(local_index) / float(columns)) * (cell_size.y + gap),
			)
			_blit_fitted(
				sheet,
				images[source_index] as Image,
				cell_origin,
				cell_size,
				fit,
			)
			if source_index < labels.size():
				var label := str(labels[source_index]).strip_edges().to_upper()
				if not label.is_empty():
					ImageLabel.draw(
						sheet,
						label,
						cell_origin,
						cell_size,
						label_scale,
					)
		sheets.append(sheet)

	return {"success": true, "sheets": sheets}


static func _blit_fitted(
	sheet: Image,
	source: Image,
	cell_origin: Vector2i,
	cell_size: Vector2i,
	fit: String,
) -> void:
	var image := source.duplicate() as Image
	if image.get_format() != Image.FORMAT_RGBA8:
		image.convert(Image.FORMAT_RGBA8)

	if fit == "stretch":
		image.resize(cell_size.x, cell_size.y, Image.INTERPOLATE_LANCZOS)
		sheet.blit_rect(image, Rect2i(Vector2i.ZERO, cell_size), cell_origin)
		return

	var scale_x := float(cell_size.x) / float(image.get_width())
	var scale_y := float(cell_size.y) / float(image.get_height())
	var scale := minf(scale_x, scale_y) if fit == "contain" else maxf(scale_x, scale_y)
	var resized := Vector2i(
		maxi(1, roundi(float(image.get_width()) * scale)),
		maxi(1, roundi(float(image.get_height()) * scale)),
	)
	image.resize(resized.x, resized.y, Image.INTERPOLATE_LANCZOS)

	if fit == "contain":
		var destination := cell_origin + (cell_size - resized) / 2
		sheet.blit_rect(image, Rect2i(Vector2i.ZERO, resized), destination)
		return

	var crop_origin := Vector2i(
		maxi(0, (resized.x - cell_size.x) / 2),
		maxi(0, (resized.y - cell_size.y) / 2),
	)
	sheet.blit_rect(image, Rect2i(crop_origin, cell_size), cell_origin)


static func _error(message: String) -> Dictionary:
	return {"success": false, "error": message, "sheets": []}
