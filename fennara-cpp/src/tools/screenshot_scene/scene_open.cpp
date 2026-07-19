#include "fennara/tools/screenshot_scene.hpp"

#include "fennara/helpers.hpp"
#include "fennara/logger.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/camera3d.hpp>
#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/classes/packed_scene.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/variant/string_name.hpp>

namespace fennara {

namespace {

struct SceneViewHints {
    int node3d_count = 0;
    int control_count = 0;
    bool has_camera3d = false;
};

void collect_scene_view_hints(godot::Node *node, SceneViewHints &hints) {
    if (!node) {
        return;
    }

    if (godot::Object::cast_to<godot::Node3D>(node)) {
        hints.node3d_count++;
    }
    if (godot::Object::cast_to<godot::Control>(node)) {
        hints.control_count++;
    }
    if (godot::Object::cast_to<godot::Camera3D>(node)) {
        hints.has_camera3d = true;
    }

    for (int i = 0; i < node->get_child_count(); i++) {
        collect_scene_view_hints(node->get_child(i), hints);
    }
}

bool is_3d_scene(godot::Node *root, const SceneViewHints &hints) {
    if (!root) {
        return false;
    }
    if (godot::Object::cast_to<godot::Node3D>(root)) {
        return true;
    }
    if (hints.has_camera3d) {
        return true;
    }
    return hints.node3d_count > 0 && hints.control_count == 0;
}

} // namespace

godot::Dictionary FennaraScreenshotSceneTool::open_scene(const godot::String &scene_path) {
    godot::Dictionary result;

    godot::String path = normalize_path(scene_path);
    _current_scene_path_ref() = path;
    _capture_requires_content_ref() = false;
    _capture_name_hint_ref() = _make_name_hint(path, "", _is_3d_scene ? "3d" : "2d");
    FLOG_TOOL(godot::String("SS: opening scene=") + path);

    if (!godot::FileAccess::file_exists(path)) {
        FLOG_ERR(godot::String("SS: scene not found: ") + path);
        result["success"] = false;
        result["error"] = "Scene file not found: " + path;
        return result;
    }

    godot::Ref<godot::PackedScene> packed =
        godot::ResourceLoader::get_singleton()->load(
            path, "PackedScene", godot::ResourceLoader::CACHE_MODE_IGNORE);
    if (packed.is_null() || !packed->can_instantiate()) {
        result["success"] = false;
        result["error"] = "Could not load scene: " + path;
        return result;
    }

    godot::Node *root = packed->instantiate();
    if (!root) {
        result["success"] = false;
        result["error"] = "Could not instantiate scene: " + path;
        return result;
    }
    SceneViewHints view_hints;
    collect_scene_view_hints(root, view_hints);
    _is_3d_scene = is_3d_scene(root, view_hints);
    memdelete(root);

    FLOG_TOOL(
        godot::String("SS: detected isolated ") +
        (_is_3d_scene ? "3D" : "2D") + " capture node3d=" +
        godot::String::num_int64(view_hints.node3d_count) +
        " control=" + godot::String::num_int64(view_hints.control_count));
    _capture_name_hint_ref() = _make_name_hint(path, "", _is_3d_scene ? "3d" : "2d");

    result["success"] = true;
    result["scene_path"] = path;
    result["is_3d"] = _is_3d_scene;
    result["view_mode"] = _is_3d_scene ? "3D" : "2D";
    result["node3d_count"] = view_hints.node3d_count;
    result["control_count"] = view_hints.control_count;
    result["has_camera3d"] = view_hints.has_camera3d;
    result["capture_mode"] = "isolated_subviewport";
    return result;
}

} // namespace fennara
