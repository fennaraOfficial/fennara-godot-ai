#include "fennara/tools/screenshot_scene.hpp"

#include "fennara/logger.hpp"

#include <algorithm>
#include <cmath>

#include <godot_cpp/classes/camera2d.hpp>
#include <godot_cpp/classes/camera3d.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/image.hpp>
#include <godot_cpp/classes/marshalls.hpp>
#include <godot_cpp/classes/sub_viewport.hpp>
#include <godot_cpp/classes/viewport_texture.hpp>

namespace fennara {

namespace {

struct ContentMetrics {
    bool meaningful = false;
    int sample_count = 0;
    int changed_samples = 0;
    float coverage = 0.0f;
    float max_span = 0.0f;
    float color_range = 0.0f;
};

ContentMetrics analyze_content(const godot::Ref<godot::Image> &image) {
    ContentMetrics metrics;
    if (image.is_null() || image->get_width() <= 0 || image->get_height() <= 0) {
        return metrics;
    }

    int step_x = std::max(1, image->get_width() / 128);
    int step_y = std::max(1, image->get_height() / 128);
    godot::Color reference = image->get_pixel(0, 0);
    godot::Color min_color(1.0, 1.0, 1.0, 1.0);
    godot::Color max_color(0.0, 0.0, 0.0, 0.0);
    int min_x = image->get_width();
    int min_y = image->get_height();
    int max_x = -1;
    int max_y = -1;
    for (int y = 0; y < image->get_height(); y += step_y) {
        for (int x = 0; x < image->get_width(); x += step_x) {
            godot::Color color = image->get_pixel(x, y);
            min_color.r = std::min(min_color.r, color.r);
            min_color.g = std::min(min_color.g, color.g);
            min_color.b = std::min(min_color.b, color.b);
            max_color.r = std::max(max_color.r, color.r);
            max_color.g = std::max(max_color.g, color.g);
            max_color.b = std::max(max_color.b, color.b);
            metrics.sample_count++;
            float difference = std::max(
                std::abs(color.r - reference.r),
                std::max(std::abs(color.g - reference.g),
                         std::abs(color.b - reference.b)));
            if (difference >= 0.02f) {
                metrics.changed_samples++;
                min_x = std::min(min_x, x);
                min_y = std::min(min_y, y);
                max_x = std::max(max_x, x);
                max_y = std::max(max_y, y);
            }
        }
    }
    metrics.color_range = std::max(
        max_color.r - min_color.r,
        std::max(max_color.g - min_color.g, max_color.b - min_color.b));
    if (metrics.sample_count > 0) {
        metrics.coverage = float(metrics.changed_samples) /
                           float(metrics.sample_count);
    }
    if (max_x >= min_x && max_y >= min_y) {
        float span_x = float(max_x - min_x + step_x) /
                       float(image->get_width());
        float span_y = float(max_y - min_y + step_y) /
                       float(image->get_height());
        metrics.max_span = std::max(span_x, span_y);
    }
    metrics.meaningful = metrics.changed_samples >= 8 &&
                         metrics.coverage >= 0.001f &&
                         metrics.max_span >= 0.40f &&
                         metrics.color_range >= 0.01f;
    return metrics;
}

} // namespace

godot::Dictionary FennaraScreenshotSceneTool::capture_image_owned(
    uint64_t owner) {
    godot::Dictionary result;

    if (owner == 0 || owner != _active_capture_owner_ref()) {
        result["success"] = false;
        result["error"] = "Screenshot capture ownership expired";
        return result;
    }

    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    if (!editor) {
        result["success"] = false;
        result["error"] = "EditorInterface not available";
        return result;
    }

    FLOG_TOOL(godot::String("SS: capture started, is_3d=") + (_is_3d_scene ? "true" : "false"));

    godot::SubViewport *viewport = _camera_capture_viewport_ref();
    bool using_isolated_viewport = viewport != nullptr;
    if (viewport) {
        FLOG_TOOL("SS: capturing isolated screenshot viewport");
    } else if (_is_3d_scene) {
        viewport = editor->get_editor_viewport_3d(0);
        if (!viewport) {
            FLOG_ERR("SS: 3D viewport null");
            result["success"] = false;
            result["error"] = "3D editor viewport not available";
            return result;
        }
    } else {
        viewport = editor->get_editor_viewport_2d();
        if (!viewport) {
            FLOG_ERR("SS: 2D viewport null");
            result["success"] = false;
            result["error"] = "2D editor viewport not available";
            return result;
        }
    }

    auto cleanup_temporary_viewport = [&](bool preserve_script_root = false) {
        if (!using_isolated_viewport) return;
        if (preserve_script_root) return;
        _discard_temporary_viewport(preserve_script_root);
    };

    godot::Ref<godot::Image> image;
    if ((bool)_camera_search_capture_state_ref().get("enabled", false)) {
        image = _capture_camera_searched_3d(viewport, result);
        if ((bool)result.get("pending", false)) {
            return result;
        }
        if (image.is_null()) {
            result["success"] = false;
            if (!result.has("error")) {
                result["error"] = "Could not render the camera-searched 3D capture";
            }
            cleanup_temporary_viewport();
            return result;
        }
    } else {
        godot::Ref<godot::ViewportTexture> tex = viewport->get_texture();
        if (!tex.is_valid()) {
            FLOG_ERR("SS: viewport texture null");
            result["success"] = false;
            result["error"] = "Could not get viewport texture";
            cleanup_temporary_viewport();
            return result;
        }
        image = tex->get_image();
        if (!image.is_valid()) {
            FLOG_ERR("SS: image from viewport null");
            result["success"] = false;
            result["error"] = "Could not get image from viewport texture";
            cleanup_temporary_viewport();
            return result;
        }
    }

    ContentMetrics content;
    if (_capture_requires_content_ref()) {
        content = analyze_content(image);
    }
    bool content_is_meaningful =
        !_capture_requires_content_ref() || content.meaningful;
    if (_capture_requires_content_ref() && !content.meaningful) {
        FLOG_TOOL(godot::String("SS: isolated 3D capture framing warning coverage=") +
                  godot::String::num(content.coverage, 4) +
                  " span=" + godot::String::num(content.max_span, 4));
    }

    FLOG_TOOL(godot::String("SS: captured size=") + godot::String::num_int64(image->get_width()) + "x" + godot::String::num_int64(image->get_height()));
    result["success"] = true;
    if (using_isolated_viewport) {
        godot::Node *root = _camera_capture_root_ref();
        godot::Node *current_camera = nullptr;
        godot::Camera2D *camera_2d = viewport->get_camera_2d();
        godot::Camera3D *camera_3d = viewport->get_camera_3d();
        if (camera_2d) {
            current_camera = camera_2d;
            result["current_camera_type"] = "Camera2D";
        } else if (camera_3d) {
            current_camera = camera_3d;
            result["current_camera_type"] = "Camera3D";
        }
        if (root && current_camera) {
            result["current_camera_path"] = godot::String(root->get_path_to(current_camera));
        } else if (!current_camera) {
            result["camera_warning"] = "No current Camera2D or Camera3D was active in the temporary SubViewport at capture time.";
        }
    }
    result["image"] = image;
    result["width"] = image->get_width();
    result["height"] = image->get_height();
    if (!result.has("image_role")) {
        result["image_role"] = _is_3d_scene ? "view" : "single";
    }
    if (_capture_requires_content_ref()) {
        result["content_validation"] =
            content_is_meaningful ? "passed" : "failed";
        result["content_coverage"] = content.coverage;
        result["content_max_span"] = content.max_span;
        if (!content_is_meaningful) {
            result["content_warning"] =
                "Captured image was returned, but automatic framing may be too small or visually sparse.";
        }
    }
    cleanup_temporary_viewport(_preserve_script_root_after_capture_ref());

    return result;
}

godot::Dictionary FennaraScreenshotSceneTool::capture_owned(uint64_t owner) {
    godot::Dictionary result = capture_image_owned(owner);
    if ((bool)result.get("pending", false) ||
        !(bool)result.get("success", false)) {
        return result;
    }

    godot::Ref<godot::Image> image = result.get("image", godot::Variant());
    result.erase("image");
    if (image.is_null()) {
        result["success"] = false;
        result["error"] = "Captured image was unavailable.";
        return result;
    }
    godot::PackedByteArray png_data = image->save_png_to_buffer();
    if (png_data.is_empty()) {
        result["success"] = false;
        result["error"] = "Failed to encode image as PNG";
        return result;
    }
    result["image_base64"] =
        godot::Marshalls::get_singleton()->raw_to_base64(png_data);
    result["format"] = "png";
    result["mime_type"] = "image/png";
    godot::String hint = _capture_name_hint_ref();
    if (hint.is_empty()) {
        hint = _is_3d_scene ? "3d_view" : "2d";
    }
    _save_png_data(png_data, hint, result);
    return result;
}

} // namespace fennara
