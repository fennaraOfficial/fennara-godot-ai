#include "fennara/tools/screenshot_scene.hpp"

#include "fennara/helpers.hpp"
#include "fennara/logger.hpp"
#include "fennara/tools/run_scene_edit_script/internal.hpp"
#include "fennara/tools/screenshot_scene_script.hpp"
#include "fennara/warning_capture.hpp"

#include <godot_cpp/classes/sub_viewport.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

bool FennaraScreenshotSceneTool::_is_3d_scene = false;

godot::String &FennaraScreenshotSceneTool::_current_scene_path_ref() {
    static godot::String *value = new godot::String;
    return *value;
}

godot::String &FennaraScreenshotSceneTool::_capture_name_hint_ref() {
    static godot::String *value = new godot::String;
    return *value;
}

godot::String &FennaraScreenshotSceneTool::_artifact_dir_ref() {
    static godot::String *value = new godot::String;
    return *value;
}

godot::SubViewport *&FennaraScreenshotSceneTool::_camera_capture_viewport_ref() {
    static godot::SubViewport *value = nullptr;
    return value;
}

godot::Node *&FennaraScreenshotSceneTool::_camera_capture_root_ref() {
    static godot::Node *value = nullptr;
    return value;
}

bool &FennaraScreenshotSceneTool::_capture_requires_content_ref() {
    static bool value = false;
    return value;
}

godot::Dictionary &FennaraScreenshotSceneTool::_camera_search_capture_state_ref() {
    static godot::Dictionary *value = new godot::Dictionary;
    return *value;
}

godot::Node *&FennaraScreenshotSceneTool::_script_capture_root_ref() {
    static godot::Node *value = nullptr;
    return value;
}

godot::Ref<FennaraScreenshotSceneScriptContext>
    &FennaraScreenshotSceneTool::_script_context_ref() {
    static godot::Ref<FennaraScreenshotSceneScriptContext> *value =
        new godot::Ref<FennaraScreenshotSceneScriptContext>;
    return *value;
}

godot::Ref<godot::RefCounted>
    &FennaraScreenshotSceneTool::_script_runner_ref() {
    static godot::Ref<godot::RefCounted> *value =
        new godot::Ref<godot::RefCounted>;
    return *value;
}

godot::Ref<godot::RefCounted>
    &FennaraScreenshotSceneTool::_script_instance_ref() {
    static godot::Ref<godot::RefCounted> *value =
        new godot::Ref<godot::RefCounted>;
    return *value;
}

godot::Ref<FennaraWarningCapture>
    &FennaraScreenshotSceneTool::_script_warning_capture_ref() {
    static godot::Ref<FennaraWarningCapture> *value =
        new godot::Ref<FennaraWarningCapture>;
    return *value;
}

bool &FennaraScreenshotSceneTool::_preserve_script_root_after_capture_ref() {
    static bool value = false;
    return value;
}

void FennaraScreenshotSceneTool::_clear_script_capture_session(
    bool free_detached_root) {
    const bool context_owns_root = _script_context_ref().is_valid();
    godot::Node *root = _script_capture_root_ref();
    if (context_owns_root && root && root->get_parent() ==
            _camera_capture_viewport_ref()) {
        _discard_temporary_viewport(true);
    }
    godot::Ref<FennaraWarningCapture> &warning_capture =
        _script_warning_capture_ref();
    if (warning_capture.is_valid()) {
        godot::OS::get_singleton()->remove_logger(warning_capture);
        warning_capture.unref();
    }
    _script_context_ref().unref();
    _script_runner_ref().unref();
    _script_instance_ref().unref();
    if (!context_owns_root && free_detached_root && root &&
        !root->get_parent()) {
        memdelete(root);
    }
    _script_capture_root_ref() = nullptr;
    _preserve_script_root_after_capture_ref() = false;
}

void FennaraScreenshotSceneTool::_clear_camera_search_capture_state() {
    _reset_camera_search_job();
    _camera_search_capture_state_ref().clear();
}

uint64_t &FennaraScreenshotSceneTool::_active_capture_owner_ref() {
    static uint64_t value = 0;
    return value;
}

uint64_t &FennaraScreenshotSceneTool::_next_capture_owner_ref() {
    static uint64_t value = 0;
    return value;
}

void FennaraScreenshotSceneTool::_discard_temporary_viewport(
    bool preserve_script_root) {
    _reset_camera_search_job();
    godot::SubViewport *viewport = _camera_capture_viewport_ref();
    godot::Node *root = _camera_capture_root_ref();
    if (viewport) {
        if (preserve_script_root && root && root->get_parent() == viewport) {
            viewport->remove_child(root);
        }
        if (viewport->is_inside_tree()) {
            viewport->queue_free();
        } else {
            memdelete(viewport);
        }
    }
    if (!preserve_script_root && _script_capture_root_ref() == root) {
        _script_capture_root_ref() = nullptr;
    }
    _camera_capture_viewport_ref() = nullptr;
    _camera_capture_root_ref() = nullptr;
    _capture_requires_content_ref() = false;
    _preserve_script_root_after_capture_ref() = false;
    _clear_camera_search_capture_state();
}

uint64_t FennaraScreenshotSceneTool::try_reserve_capture() {
    if (_active_capture_owner_ref() != 0) {
        return 0;
    }
    uint64_t &next_owner = _next_capture_owner_ref();
    next_owner++;
    if (next_owner == 0) next_owner++;
    _active_capture_owner_ref() = next_owner;
    return next_owner;
}

bool FennaraScreenshotSceneTool::owns_capture(uint64_t owner) {
    return owner != 0 && _active_capture_owner_ref() == owner;
}

void FennaraScreenshotSceneTool::release_capture(uint64_t owner) {
    if (owner == 0 || _active_capture_owner_ref() != owner) {
        return;
    }
    _discard_temporary_viewport();
    _clear_script_capture_session(true);
    _clear_capture_script();
    _active_capture_owner_ref() = 0;
}

void FennaraScreenshotSceneTool::_bind_methods() {
    godot::ClassDB::bind_static_method(
        "FennaraScreenshotSceneTool", godot::D_METHOD("open_scene", "scene_path"),
        &FennaraScreenshotSceneTool::open_scene);
    godot::ClassDB::bind_static_method(
        "FennaraScreenshotSceneTool", godot::D_METHOD("navigate", "args"),
        &FennaraScreenshotSceneTool::navigate);
    godot::ClassDB::bind_static_method(
        "FennaraScreenshotSceneTool", godot::D_METHOD("execute", "args"),
        &FennaraScreenshotSceneTool::execute);
#ifdef FENNARA_SETUP_TEST_HOOKS
    godot::ClassDB::bind_static_method(
        "FennaraScreenshotSceneTool",
        godot::D_METHOD("prepare_execution_for_test", "args"),
        &FennaraScreenshotSceneTool::prepare_execution);
#endif
}

godot::Dictionary FennaraScreenshotSceneTool::prepare_execution(
    const godot::Dictionary &args) {
    godot::Dictionary result = args.duplicate();

    godot::Array keys = args.keys();
    for (int i = 0; i < keys.size(); i++) {
        godot::String key = keys[i];
        if (key == "scene_path" || key == "code" || key == "script_path" ||
            key == "_fennara_tool_artifact_dir") {
            continue;
        }
        result["success"] = false;
        result["error"] =
            "Unsupported screenshot_scene argument: " + key +
            ". Select nodes and configure framing inside ctx.capture(...).";
        return result;
    }

    godot::String scene_path = args.get("scene_path", "");
    if (scene_path.is_empty()) {
        result["success"] = false;
        result["error"] = "scene_path is required";
        return result;
    }

    godot::String code = args.get("code", "");
    godot::String script_path = args.get("script_path", "");
    if (!code.is_empty() && !script_path.is_empty()) {
        result["success"] = false;
        result["error"] = "Provide exactly one of code or script_path.";
        return result;
    }

    if (!code.is_empty() || !script_path.is_empty()) {
        godot::String prepared_script_path =
            run_scene_edit_script_internal::write_or_resolve_script_path(
                normalize_path(scene_path), code, script_path, result);
        if (prepared_script_path.is_empty()) {
            return result;
        }
        result["_fennara_screenshot_script_path"] = prepared_script_path;
    }

    result["success"] = true;
    return result;
}

godot::Dictionary FennaraScreenshotSceneTool::execute_prepared(
    const godot::Dictionary &prepared_args) {
    godot::Dictionary result;

    godot::String artifact_dir =
        godot::String(prepared_args.get(
            "_fennara_tool_artifact_dir", "")).strip_edges();
    _artifact_dir_ref() = artifact_dir.is_empty()
        ? godot::String()
        : artifact_dir.path_join("screenshot_scene");

    godot::String scene_path = prepared_args.get("scene_path", "");
    if (scene_path.is_empty()) {
        FLOG_ERR("SS: scene_path is required");
        result["success"] = false;
        result["error"] = "scene_path is required";
        return result;
    }
    if (!_configure_capture_script(prepared_args, result)) {
        return result;
    }

    result = open_scene(scene_path);
    _append_capture_script_receipt(result);
    return result;
}

godot::Dictionary FennaraScreenshotSceneTool::execute(
    const godot::Dictionary &args) {
    godot::Dictionary prepared = prepare_execution(args);
    if (!(bool)prepared.get("success", false)) {
        return prepared;
    }

    godot::String script_path =
        prepared.get("_fennara_screenshot_script_path", "");
    if (!script_path.is_empty()) {
        prepared["diagnostic_success"] = false;
        prepared["diagnostic_error"] =
            "LSP diagnostics require the asynchronous tool executor; using direct script validation.";
    }

    return execute_prepared(prepared);
}

} // namespace fennara
