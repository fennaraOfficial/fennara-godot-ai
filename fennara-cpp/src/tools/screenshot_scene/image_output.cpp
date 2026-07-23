#include "fennara/tools/screenshot_scene.hpp"

#include <godot_cpp/classes/image.hpp>
#include <godot_cpp/classes/marshalls.hpp>

namespace fennara {

namespace {

constexpr int MODEL_IMAGE_OUTPUT_LIMIT = 6;

} // namespace

godot::Dictionary FennaraScreenshotSceneTool::publish_image(
    const godot::Ref<godot::Image> &image,
    const godot::String &description,
    int output_index) {
    godot::Dictionary result;
    if (image.is_null() || image->get_width() <= 0 || image->get_height() <= 0) {
        result["success"] = false;
        result["error"] = "ctx.output() expects a non-empty Image.";
        return result;
    }

    godot::PackedByteArray png_data = image->save_png_to_buffer();
    if (png_data.is_empty()) {
        result["success"] = false;
        result["error"] = "Failed to encode ctx.output() image as PNG.";
        return result;
    }

    result["output_index"] = output_index;
    result["description"] = description.strip_edges().is_empty()
        ? godot::String("Screenshot output ") +
              godot::String::num_int64(output_index + 1)
        : description.strip_edges();
    result["format"] = "png";
    result["mime_type"] = "image/png";
    result["width"] = image->get_width();
    result["height"] = image->get_height();
    result["image_role"] = "authored_output";

    godot::String hint = _make_name_hint(
        _current_scene_path_ref(),
        "output_" + godot::String::num_int64(output_index + 1), "");
    godot::String saved_path = _save_png_data(png_data, hint, result);
    if (saved_path.is_empty()) {
        result["success"] = false;
        result["error"] = result.get(
            "save_error", "Failed to save ctx.output() image.");
        return result;
    }
    result["success"] = true;
    if (output_index < MODEL_IMAGE_OUTPUT_LIMIT) {
        result["image_base64"] =
            godot::Marshalls::get_singleton()->raw_to_base64(png_data);
    } else {
        result["model_image_omitted"] = true;
    }
    return result;
}

} // namespace fennara
