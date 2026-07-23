#include "fennara/tools/screenshot_scene.hpp"
#include "fennara/tools/screenshot_scene_script.hpp"

#include "fennara/logger.hpp"

#include <godot_cpp/classes/camera2d.hpp>
#include <godot_cpp/classes/camera3d.hpp>
#include <godot_cpp/classes/canvas_item.hpp>
#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/classes/packed_scene.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/classes/sub_viewport.hpp>

namespace fennara {

namespace {

struct CaptureDimensions {
    bool has_2d = false;
    bool has_3d = false;
};

void collect_capture_dimensions(godot::Node *node,
                                CaptureDimensions &dimensions) {
    if (!node) {
        return;
    }
    if (godot::Object::cast_to<godot::Node3D>(node)) {
        dimensions.has_3d = true;
    }
    if (godot::Object::cast_to<godot::CanvasItem>(node)) {
        dimensions.has_2d = true;
    }
    for (int i = 0; i < node->get_child_count(); i++) {
        collect_capture_dimensions(node->get_child(i), dimensions);
    }
}

bool resolve_scripted_dimension(
    godot::Node *root, const godot::Array &capture_nodes,
    const godot::Dictionary &capture_options, bool &is_3d,
    godot::Dictionary &result) {
    if (capture_options.has("camera")) {
        godot::Object *camera_object = capture_options["camera"];
        godot::Node *camera_node =
            godot::Object::cast_to<godot::Node>(camera_object);
        if (!camera_node ||
            (camera_node != root && !root->is_ancestor_of(camera_node))) {
            result["success"] = false;
            result["error"] =
                "Script capture option `camera` must be a Camera2D or Camera3D under ctx.root.";
            return false;
        }
        if (godot::Object::cast_to<godot::Camera3D>(camera_node)) {
            is_3d = true;
            return true;
        }
        if (godot::Object::cast_to<godot::Camera2D>(camera_node)) {
            is_3d = false;
            return true;
        }
        result["success"] = false;
        result["error"] =
            "Script capture option `camera` must be a Camera2D or Camera3D under ctx.root.";
        return false;
    }

    CaptureDimensions dimensions;
    for (int i = 0; i < capture_nodes.size(); i++) {
        godot::Object *object = capture_nodes[i];
        collect_capture_dimensions(
            godot::Object::cast_to<godot::Node>(object), dimensions);
    }
    if (dimensions.has_2d && dimensions.has_3d) {
        result["success"] = false;
        result["error"] =
            "ctx.capture() subjects span both 2D and 3D. Select one dimension or provide a Camera2D or Camera3D.";
        return false;
    }
    if (!dimensions.has_2d && !dimensions.has_3d) {
        result["success"] = false;
        result["error"] =
            "ctx.capture() subjects do not contain capturable 2D or 3D nodes.";
        return false;
    }
    is_3d = dimensions.has_3d;
    return true;
}

} // namespace

godot::Dictionary FennaraScreenshotSceneTool::navigate(
    const godot::Dictionary &args, int capture_index) {
    godot::Dictionary result;
    godot::String scene_path = _current_scene_path_ref();
    if (_has_capture_script()) {
        result["success"] = false;
        result["error"] =
            "Scripted screenshots require the asynchronous screenshot executor.";
        return result;
    }
    if (capture_index != 0) {
        result["success"] = false;
        result["error"] = "Whole-scene capture has only one image.";
        return result;
    }

    godot::Ref<godot::PackedScene> packed =
        godot::ResourceLoader::get_singleton()->load(
            scene_path, "PackedScene",
            godot::ResourceLoader::CACHE_MODE_IGNORE);
    if (packed.is_null() || !packed->can_instantiate()) {
        result["success"] = false;
        result["error"] =
            "Could not load scene for isolated capture: " + scene_path;
        return result;
    }
    godot::Node *root = packed->instantiate();
    if (!root) {
        result["success"] = false;
        result["error"] =
            "Could not instantiate scene for isolated capture: " + scene_path;
        return result;
    }

    godot::Array capture_nodes;
    capture_nodes.append(root);
    godot::Dictionary capture_options;
    if (_is_3d_scene) {
        FLOG_TOOL("SS: preparing isolated 3D capture");
        result = _frame_3d_editor_camera(
            root, capture_nodes, capture_options, false);
    } else {
        FLOG_TOOL("SS: preparing isolated 2D capture");
        result = _frame_2d_script_capture(root, capture_nodes, capture_options);
    }
    result["capture_index"] = 0;
    result["capture_count"] = 1;
    if (!root->get_parent()) {
        memdelete(root);
    }
    return result;
}

godot::Dictionary
FennaraScreenshotSceneTool::navigate_pending_script_capture() {
    godot::Dictionary result;
    godot::Ref<FennaraScreenshotSceneScriptContext> ctx =
        _script_context_ref();
    godot::Node *root = _script_capture_root_ref();
    if (ctx.is_null() || !root) {
        result["success"] = false;
        result["error"] =
            "Retained screenshot scene was unavailable for capture.";
        return result;
    }
    godot::Node *parent = root->get_parent();
    if (parent) {
        if (parent != _camera_capture_viewport_ref()) {
            result["success"] = false;
            result["error"] =
                "Retained screenshot scene had an unexpected parent.";
            return result;
        }
        _discard_temporary_viewport(true);
    }

    godot::Dictionary request = ctx->take_pending_capture();
    if (request.is_empty()) {
        result["success"] = false;
        result["error"] = "No pending ctx.capture() request was available.";
        return result;
    }
    godot::Array capture_nodes = request.get("nodes", godot::Array());
    godot::Dictionary capture_options =
        request.get("options", godot::Dictionary());
    bool capture_is_3d = _is_3d_scene;
    if (!resolve_scripted_dimension(root, capture_nodes, capture_options,
                                    capture_is_3d, result)) {
        return result;
    }

    if (capture_is_3d) {
        const bool use_default_camera_search =
            !capture_options.has("view") && !capture_options.has("camera");
        result = _frame_3d_editor_camera(
            root, capture_nodes, capture_options, use_default_camera_search);
    } else {
        result = _frame_2d_script_capture(root, capture_nodes, capture_options);
    }
    result["scripted"] = true;
    result["script_subject_count"] = capture_nodes.size();
    result["capture_index"] = ctx->get_capture_count() - 1;
    _preserve_script_root_after_capture_ref() =
        (bool)result.get("success", false);
    return result;
}

} // namespace fennara
