#include "fennara/tools/screenshot_scene.hpp"

#include "fennara/logger.hpp"
#include "fennara/tools/screenshot_scene_script.hpp"

#include <godot_cpp/classes/camera3d.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/sub_viewport.hpp>
#include <godot_cpp/classes/window.hpp>

namespace fennara {

namespace {

void clear_capture_helpers(godot::SubViewport *viewport, godot::Node *root) {
    if (!viewport) {
        return;
    }
    for (int index = viewport->get_child_count() - 1; index >= 0; index--) {
        godot::Node *child = viewport->get_child(index);
        if (!child || child == root) {
            continue;
        }
        viewport->remove_child(child);
        memdelete(child);
    }
}

} // namespace

godot::SubViewport *FennaraScreenshotSceneTool::_prepare_capture_viewport(
    godot::Node *root, const godot::String &name,
    const godot::Vector2i &size, bool use_own_world_3d,
    godot::Dictionary &result) {
    if (!root) {
        result["success"] = false;
        result["error"] = "Screenshot scene root was unavailable.";
        return nullptr;
    }

    godot::SubViewport *viewport = _camera_capture_viewport_ref();
    const bool reusable_script_viewport =
        has_script_session() && root == _script_capture_root_ref() &&
        viewport && _camera_capture_root_ref() == root &&
        root->get_parent() == viewport;
    if (reusable_script_viewport) {
        _clear_camera_search_capture_state();
        clear_capture_helpers(viewport, root);
        viewport->set_size(size);
        viewport->set_update_mode(godot::SubViewport::UPDATE_ALWAYS);
        viewport->set_clear_mode(godot::SubViewport::CLEAR_MODE_ALWAYS);
        viewport->set_transparent_background(false);
        FLOG_TOOL("SS: reusing scripted screenshot viewport");
        return viewport;
    }

    if (viewport) {
        const bool preserve_script_root =
            has_script_session() && root == _script_capture_root_ref() &&
            root->get_parent() == viewport;
        _discard_temporary_viewport(preserve_script_root);
    }
    if (root->get_parent()) {
        result["success"] = false;
        result["error"] =
            "Screenshot scene root already had an unexpected parent.";
        return nullptr;
    }

    godot::EditorInterface *editor =
        godot::EditorInterface::get_singleton();
    godot::Node *base = editor
        ? godot::Object::cast_to<godot::Node>(editor->get_base_control())
        : nullptr;
    if (!base) {
        result["success"] = false;
        result["error"] = "Editor base control not available";
        return nullptr;
    }

    viewport = memnew(godot::SubViewport);
    viewport->set_name(name);
    viewport->set_size(size);
    viewport->set_update_mode(godot::SubViewport::UPDATE_ALWAYS);
    viewport->set_clear_mode(godot::SubViewport::CLEAR_MODE_ALWAYS);
    viewport->set_transparent_background(false);
    viewport->set_use_own_world_3d(
        use_own_world_3d || has_script_session());
    base->add_child(viewport);
    viewport->add_child(root);

    _camera_capture_viewport_ref() = viewport;
    _camera_capture_root_ref() = root;
    FLOG_TOOL(has_script_session()
        ? "SS: created scripted screenshot viewport"
        : "SS: created one-shot screenshot viewport");
    return viewport;
}

void FennaraScreenshotSceneTool::_cleanup_failed_capture_setup() {
    if (!has_script_session()) {
        _discard_temporary_viewport();
    }
}

#ifdef FENNARA_SETUP_TEST_HOOKS
godot::Dictionary
FennaraScreenshotSceneTool::test_script_viewport_reuse() {
    godot::Dictionary result;
    _discard_temporary_viewport();
    _clear_script_capture_session(true);

    godot::Node3D *root = memnew(godot::Node3D);
    root->set_name("ViewportReuseTestRoot");
    godot::Camera3D *supplied_camera = memnew(godot::Camera3D);
    supplied_camera->set_name("SuppliedCamera");
    root->add_child(supplied_camera);

    godot::Ref<FennaraScreenshotSceneScriptContext> context;
    context.instantiate();
    context->configure(root, "res://tests/fixtures/viewport_reuse.tscn");
    _script_capture_root_ref() = root;
    _script_context_ref() = context;

    godot::SceneTree *tree = godot::Object::cast_to<godot::SceneTree>(
        godot::Engine::get_singleton()->get_main_loop());
    godot::Window *tree_root = tree ? tree->get_root() : nullptr;
    if (!tree_root) {
        _clear_script_capture_session(true);
        result["success"] = false;
        result["error"] = "SceneTree root was unavailable.";
        return result;
    }

    godot::SubViewport *first = memnew(godot::SubViewport);
    first->set_name("ViewportReuseTest");
    first->set_size(godot::Vector2i(320, 180));
    first->set_use_own_world_3d(true);
    tree_root->add_child(first);
    first->add_child(root);
    _camera_capture_viewport_ref() = first;
    _camera_capture_root_ref() = root;
    const uint64_t first_id = first->get_instance_id();

    godot::Camera3D *helper_camera = memnew(godot::Camera3D);
    helper_camera->set_name("TemporaryHelperCamera");
    first->add_child(helper_camera);

    godot::Dictionary setup_result;
    godot::SubViewport *second = _prepare_capture_viewport(
        root, "ViewportReuseTest", godot::Vector2i(640, 360), true,
        setup_result);
    const bool same_viewport =
        second && second->get_instance_id() == first_id;
    const bool root_preserved = second && root->get_parent() == second;
    const bool supplied_camera_preserved =
        supplied_camera->get_parent() == root;
    const bool helper_removed =
        second && !second->has_node("TemporaryHelperCamera");
    const bool resized =
        second && second->get_size() == godot::Vector2i(640, 360);

    _cleanup_failed_capture_setup();
    const bool failed_setup_preserved =
        _camera_capture_viewport_ref() == second &&
        _camera_capture_root_ref() == root &&
        root->get_parent() == second;

    _clear_script_capture_session(true);
    const bool session_cleared =
        !has_script_session() &&
        _camera_capture_viewport_ref() == nullptr &&
        _camera_capture_root_ref() == nullptr &&
        _script_capture_root_ref() == nullptr;

    result["success"] =
        same_viewport && root_preserved && supplied_camera_preserved &&
        helper_removed && resized && failed_setup_preserved &&
        session_cleared;
    result["same_viewport"] = same_viewport;
    result["root_preserved"] = root_preserved;
    result["supplied_camera_preserved"] = supplied_camera_preserved;
    result["helper_removed"] = helper_removed;
    result["resized"] = resized;
    result["failed_setup_preserved"] = failed_setup_preserved;
    result["session_cleared"] = session_cleared;
    return result;
}
#endif

} // namespace fennara
