#include "fennara/tools/screenshot_scene.hpp"

#include <algorithm>

#include <godot_cpp/classes/camera2d.hpp>
#include <godot_cpp/classes/canvas_item.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/node2d.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/sub_viewport.hpp>
#include <godot_cpp/variant/rect2.hpp>
#include <godot_cpp/variant/transform2d.hpp>

namespace fennara {

namespace {

void merge_rect(const godot::Rect2 &rect, godot::Rect2 &bounds,
                bool &has_bounds) {
    if (rect.size.x <= 0.0 || rect.size.y <= 0.0) {
        return;
    }
    if (has_bounds) {
        bounds = bounds.merge(rect);
    } else {
        bounds = rect;
        has_bounds = true;
    }
}

bool accumulate_2d_bounds(godot::Node *node, godot::Rect2 &bounds,
                          bool &has_bounds) {
    if (!node) return false;
    bool subtree_has_bounds = false;

    godot::CanvasItem *item = godot::Object::cast_to<godot::CanvasItem>(node);
    if (item && item->is_visible() && item->has_method("_edit_get_rect")) {
        godot::Variant local_rect_value = item->call("_edit_get_rect");
        if (local_rect_value.get_type() == godot::Variant::RECT2) {
            godot::Rect2 local_rect = local_rect_value;
            godot::Rect2 world_rect =
                item->get_global_transform().xform(local_rect);
            bool had_bounds = has_bounds;
            merge_rect(world_rect, bounds, has_bounds);
            subtree_has_bounds = has_bounds != had_bounds ||
                                 (has_bounds && world_rect.size.x > 0.0 &&
                                  world_rect.size.y > 0.0);
        }
    }

    for (int i = 0; i < node->get_child_count(); i++) {
        if (accumulate_2d_bounds(node->get_child(i), bounds, has_bounds)) {
            subtree_has_bounds = true;
        }
    }

    if (!subtree_has_bounds) {
        godot::Node2D *node_2d = godot::Object::cast_to<godot::Node2D>(node);
        if (node_2d) {
            godot::Vector2 point = node_2d->get_global_position();
            merge_rect(godot::Rect2(point - godot::Vector2(1.0, 1.0),
                                    godot::Vector2(2.0, 2.0)),
                       bounds, has_bounds);
            subtree_has_bounds = true;
        }
    }
    return subtree_has_bounds;
}

} // namespace

godot::Dictionary FennaraScreenshotSceneTool::_frame_2d_script_capture(
    godot::Node *root, const godot::Array &capture_nodes,
    const godot::Dictionary &capture_options) {
    godot::Dictionary result;
    _capture_requires_content_ref() = false;
    _clear_camera_search_capture_state();
    const godot::String scene_path = _current_scene_path_ref();

    godot::ProjectSettings *ps = godot::ProjectSettings::get_singleton();
    int width = std::max(
        int(ps->get_setting("display/window/size/viewport_width", 1920)), 64);
    int height = std::max(
        int(ps->get_setting("display/window/size/viewport_height", 1080)), 64);

    godot::SubViewport *viewport = _prepare_capture_viewport(
        root, "FennaraScripted2DScreenshotViewport",
        godot::Vector2i(width, height), false, result);
    if (!viewport) {
        return result;
    }

    godot::Camera2D *camera = nullptr;
    if (capture_options.has("camera")) {
        godot::Object *camera_object = capture_options["camera"];
        godot::Node *camera_node = godot::Object::cast_to<godot::Node>(camera_object);
        camera = godot::Object::cast_to<godot::Camera2D>(camera_node);
        if (!camera ||
            (camera_node != root && !root->is_ancestor_of(camera_node))) {
            result["success"] = false;
            result["error"] =
                "Script capture option `camera` must be a Camera2D under ctx.root for this scene.";
            _cleanup_failed_capture_setup();
            return result;
        }
    }

    godot::Rect2 bounds;
    bool has_bounds = false;
    if (!camera) {
        for (int i = 0; i < capture_nodes.size(); i++) {
            godot::Object *object = capture_nodes[i];
            godot::Node *node = godot::Object::cast_to<godot::Node>(object);
            accumulate_2d_bounds(node, bounds, has_bounds);
        }
        if (!has_bounds) {
            result["success"] = false;
            result["error"] =
                "No visible 2D bounds found for the scripted capture subjects.";
            _cleanup_failed_capture_setup();
            return result;
        }

        camera = memnew(godot::Camera2D);
        camera->set_name("FennaraScripted2DCamera");
        viewport->add_child(camera);
        camera->set_position(bounds.get_center());
        double margin = std::max(
            double(capture_options.get("context_margin", 1.1)), 0.25);
        double fit_x = double(width) / std::max(double(bounds.size.x), 1.0);
        double fit_y = double(height) / std::max(double(bounds.size.y), 1.0);
        double zoom = std::max(std::min(fit_x, fit_y) / margin, 0.01);
        camera->set_zoom(godot::Vector2(zoom, zoom));
        result["context_margin"] = margin;
        result["zoom"] = zoom;
        godot::Dictionary bounds_dict;
        bounds_dict["x"] = bounds.position.x;
        bounds_dict["y"] = bounds.position.y;
        bounds_dict["width"] = bounds.size.x;
        bounds_dict["height"] = bounds.size.y;
        result["framed_bounds"] = bounds_dict;
        result["view"] = "auto_2d";
        result["note"] =
            "2D scene: isolated capture auto-framed around ctx.capture subjects.";
    } else {
        result["view"] = "camera_2d";
        result["note"] =
            "2D scene captured from the Camera2D supplied to ctx.capture.";
    }

    camera->set_enabled(true);
    camera->make_current();
    camera->force_update_scroll();
    _capture_name_hint_ref() =
        _make_name_hint(scene_path, "selection", result["view"]);

    result["success"] = true;
    result["scene_path"] = scene_path;
    result["is_3d"] = false;
    result["capture_delay_seconds"] = 0.15;
    godot::Dictionary viewport_size;
    viewport_size["width"] = width;
    viewport_size["height"] = height;
    result["viewport_size"] = viewport_size;
    _append_capture_script_receipt(result);
    return result;
}

} // namespace fennara
