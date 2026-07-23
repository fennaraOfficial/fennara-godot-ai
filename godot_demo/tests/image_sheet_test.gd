extends SceneTree

const ImageSheet := preload("res://addons/fennara/runtime/image_sheet.gd")
const ScreenshotScriptContext := preload(
	"res://addons/fennara/runtime/screenshot_script_context.gd"
)


class FakeScreenshotContext:
	extends RefCounted

	var root := Node.new()
	var errors: Array[String] = []

	func error(message: String) -> void:
		errors.append(message)


func _initialize() -> void:
	var wide_red := _solid_image(4, 2, Color.RED)
	var tall_blue := _solid_image(2, 4, Color.BLUE)
	var original_red_size := wide_red.get_size()
	var composed: Dictionary = ImageSheet.compose(
		[wide_red, tall_blue],
		{
			"columns": 2,
			"cell_size": Vector2i(8, 8),
			"gap": 2,
		},
	)
	assert(composed.get("success", false))
	var sheets: Array = composed.get("sheets", [])
	assert(sheets.size() == 1)
	var sheet := sheets[0] as Image
	assert(sheet.get_size() == Vector2i(18, 8))
	assert(wide_red.get_size() == original_red_size)
	var background := sheet.get_pixel(0, 0)
	assert(absf(background.r - 0.02) < 0.01)
	assert(absf(background.g - 0.02) < 0.01)
	assert(absf(background.b - 0.025) < 0.01)
	assert(sheet.get_pixel(4, 3).is_equal_approx(Color.RED))
	assert(sheet.get_pixel(13, 4).is_equal_approx(Color.BLUE))

	var pages_result: Dictionary = ImageSheet.compose(
		[
			_solid_image(2, 2, Color.RED),
			_solid_image(2, 2, Color.GREEN),
			_solid_image(2, 2, Color.BLUE),
			_solid_image(2, 2, Color.YELLOW),
			_solid_image(2, 2, Color.WHITE),
		],
		{
			"columns": 2,
			"cell_size": Vector2i(4, 4),
			"cells_per_sheet": 3,
			"gap": 1,
		},
	)
	assert(pages_result.get("success", false))
	var pages: Array = pages_result.get("sheets", [])
	assert(pages.size() == 2)
	assert((pages[0] as Image).get_size() == Vector2i(9, 9))
	assert((pages[1] as Image).get_size() == Vector2i(9, 4))

	var label_result: Dictionary = ImageSheet.compose(
		[_solid_image(64, 32, Color.BLACK)],
		{
			"columns": 1,
			"cell_size": Vector2i(64, 32),
			"labels": ["f00"],
		},
	)
	assert(label_result.get("success", false))
	assert(_contains_white_pixel((label_result.sheets[0] as Image)))

	var fake := FakeScreenshotContext.new()
	var ctx := ScreenshotScriptContext.new(fake)
	var facade_sheets: Array[Image] = ctx.sheet(
		[_solid_image(4, 4, Color.WHITE)],
		{"columns": 1, "cell_size": Vector2i(4, 4)},
	)
	assert(facade_sheets.size() == 1)
	assert(ctx.root == fake.root)
	assert(ctx.sheet([], {}).is_empty())
	assert(fake.errors.size() == 1)
	assert("at least one Image" in fake.errors[0])
	fake.root.free()

	print("image sheet test passed")
	quit()


func _solid_image(width: int, height: int, color: Color) -> Image:
	var image := Image.create(width, height, false, Image.FORMAT_RGBA8)
	image.fill(color)
	return image


func _contains_white_pixel(image: Image) -> bool:
	for y: int in range(image.get_height()):
		for x: int in range(image.get_width()):
			var color := image.get_pixel(x, y)
			if color.r > 0.9 and color.g > 0.9 and color.b > 0.9:
				return true
	return false
