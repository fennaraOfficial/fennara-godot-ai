#include "fennara/tools/screenshot_scene.hpp"
#include "fennara/tools/screenshot_scene_script.hpp"

#include "fennara/tools/run_scene_edit_script/internal.hpp"
#include "fennara/warning_capture.hpp"

#include <algorithm>

#include <godot_cpp/classes/gd_script.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/packed_scene.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

namespace {

constexpr int MODEL_IMAGE_OUTPUT_LIMIT = 6;
constexpr int SCRIPT_DIAGNOSTIC_LIMIT = 200;
constexpr const char *SCRIPT_RUNNER_PATH =
    "res://addons/fennara/runtime/screenshot_script_runner.gd";

godot::String &capture_script_path() {
    static godot::String *value = new godot::String;
    return *value;
}

godot::Dictionary &capture_script_diagnostics() {
    static godot::Dictionary *value = new godot::Dictionary;
    return *value;
}

godot::Dictionary script_error(const godot::String &message,
                               const godot::String &source) {
    return run_scene_edit_script_internal::make_runtime_error(message, source);
}

godot::Dictionary session_error(const godot::String &message) {
    godot::Dictionary result;
    result["success"] = false;
    result["error"] = message;
    result["runtime_errors"] = godot::Array::make(
        script_error(message, "screenshot_session"));
    result["logs"] = godot::Array();
    return result;
}

} // namespace

bool FennaraScreenshotSceneTool::_configure_capture_script(
    const godot::Dictionary &args, godot::Dictionary &result) {
    _clear_capture_script();

    godot::String prepared_script_path =
        args.get("_fennara_screenshot_script_path", "");
    if (prepared_script_path.is_empty()) {
        return true;
    }

    capture_script_path() = prepared_script_path;
    godot::Dictionary diagnostics;
    diagnostics["diagnostic_success"] =
        args.get("diagnostic_success", false);
    diagnostics["diagnostics"] =
        args.get("script_diagnostics", godot::Array());
    diagnostics["total_errors"] = args.get("total_errors", 0);
    diagnostics["total_warnings"] = args.get("total_warnings", 0);
    if (args.has("diagnostic_error")) {
        diagnostics["diagnostic_error"] = args["diagnostic_error"];
    }
    capture_script_diagnostics() = diagnostics;
    run_scene_edit_script_internal::apply_diagnostics_to_result(
        diagnostics, result);
    result["script_path"] = capture_script_path();
    return true;
}

bool FennaraScreenshotSceneTool::_has_capture_script() {
    return !capture_script_path().is_empty();
}

void FennaraScreenshotSceneTool::_append_capture_script_receipt(
    godot::Dictionary &result) {
    if (!_has_capture_script()) {
        return;
    }
    result["script_path"] = capture_script_path();
    run_scene_edit_script_internal::apply_diagnostics_to_result(
        capture_script_diagnostics(), result);
}

void FennaraScreenshotSceneTool::_clear_capture_script() {
    capture_script_path() = godot::String();
    capture_script_diagnostics() = godot::Dictionary();
}

godot::Dictionary FennaraScreenshotSceneTool::begin_script_session(
    const godot::Callable &capture_requested,
    const godot::Callable &script_completed) {
    if (!_has_capture_script()) {
        return session_error("No screenshot script was prepared.");
    }
    if (has_script_session()) {
        return session_error("A screenshot script session is already active.");
    }

    godot::Ref<godot::PackedScene> packed =
        godot::ResourceLoader::get_singleton()->load(
            _current_scene_path_ref(), "PackedScene",
            godot::ResourceLoader::CACHE_MODE_IGNORE);
    if (packed.is_null() || !packed->can_instantiate()) {
        return session_error("Could not load scene for isolated capture: " +
                             _current_scene_path_ref());
    }
    godot::Node *root = packed->instantiate();
    if (!root) {
        return session_error("Could not instantiate scene for isolated capture: " +
                             _current_scene_path_ref());
    }
    _script_capture_root_ref() = root;

    godot::Dictionary result;
    godot::Ref<godot::GDScript> user_script =
        run_scene_edit_script_internal::load_script(capture_script_path(), result);
    if (!user_script.is_valid()) {
        _clear_script_capture_session(true);
        return result;
    }
    godot::StringName base_type = user_script->get_instance_base_type();
    godot::StringName ref_counted_type("RefCounted");
    if (base_type != ref_counted_type &&
        !godot::ClassDB::is_parent_class(base_type, ref_counted_type)) {
        _clear_script_capture_session(true);
        return session_error(
            "screenshot_scene scripts must use `@tool extends RefCounted`.");
    }

    godot::Variant user_variant = user_script->new_();
    godot::Object *user_object = user_variant;
    godot::RefCounted *user =
        godot::Object::cast_to<godot::RefCounted>(user_object);
    if (!user || !user->has_method("run")) {
        _clear_script_capture_session(true);
        return session_error(
            "Screenshot script must define func run(ctx) -> void.");
    }
    _script_instance_ref() = godot::Ref<godot::RefCounted>(user);

    godot::Ref<godot::GDScript> runner_script =
        run_scene_edit_script_internal::load_script(SCRIPT_RUNNER_PATH, result);
    if (!runner_script.is_valid()) {
        _clear_script_capture_session(true);
        return result;
    }
    godot::Variant runner_variant = runner_script->new_();
    godot::Object *runner_object = runner_variant;
    godot::RefCounted *runner =
        godot::Object::cast_to<godot::RefCounted>(runner_object);
    if (!runner || !runner->has_method("execute")) {
        _clear_script_capture_session(true);
        return session_error("Screenshot script runner could not instantiate.");
    }
    _script_runner_ref() = godot::Ref<godot::RefCounted>(runner);

    godot::Ref<FennaraScreenshotSceneScriptContext> ctx;
    ctx.instantiate();
    ctx->configure(root, _current_scene_path_ref());
    ctx->connect("capture_requested", capture_requested);
    runner->connect("completed", script_completed);
    _script_context_ref() = ctx;

    godot::Ref<FennaraWarningCapture> warning_capture;
    warning_capture.instantiate();
    warning_capture->configure_source_filter(
        {capture_script_path(), SCRIPT_RUNNER_PATH},
        SCRIPT_DIAGNOSTIC_LIMIT);
    godot::OS::get_singleton()->add_logger(warning_capture);
    _script_warning_capture_ref() = warning_capture;

    runner->call("execute", _script_instance_ref(), ctx);

    result["success"] = true;
    result["scripted"] = true;
    _append_capture_script_receipt(result);
    return result;
}

bool FennaraScreenshotSceneTool::has_script_session() {
    return _script_context_ref().is_valid();
}

void FennaraScreenshotSceneTool::complete_script_capture(
    const godot::Ref<godot::Image> &image) {
    if (_script_context_ref().is_valid()) {
        _script_context_ref()->complete_capture(image);
    }
}

void FennaraScreenshotSceneTool::fail_script_capture(
    const godot::String &message) {
    if (_script_context_ref().is_valid()) {
        _script_context_ref()->fail_capture(message);
    }
}

void FennaraScreenshotSceneTool::cancel_script_session(
    const godot::String &message) {
    if (_script_context_ref().is_valid()) {
        _script_context_ref()->cancel(message);
    }
}

godot::Dictionary FennaraScreenshotSceneTool::finish_script_session() {
    if (!has_script_session()) {
        return session_error("Screenshot script session was unavailable.");
    }

    godot::Ref<FennaraScreenshotSceneScriptContext> ctx =
        _script_context_ref();
    godot::Array runtime_errors = ctx->get_errors();
    godot::Ref<FennaraWarningCapture> warning_capture =
        _script_warning_capture_ref();
    if (warning_capture.is_valid()) {
        godot::OS::get_singleton()->remove_logger(warning_capture);
        run_scene_edit_script_internal::append_capture_errors(
            warning_capture->get_captured(), runtime_errors);
        _script_warning_capture_ref().unref();
    }

    godot::Array outputs = ctx->get_outputs();
    if (!ctx->was_capture_requested()) {
        runtime_errors.append(script_error(
            "Screenshot script must await ctx.capture(nodes, options) at least once.",
            "contract"));
    }
    if (outputs.is_empty()) {
        runtime_errors.append(script_error(
            "Screenshot script must publish at least one image with ctx.output(image, description).",
            "contract"));
    }

    godot::Dictionary result;
    if (!outputs.is_empty()) {
        result = godot::Dictionary(outputs[0]).duplicate(true);
        godot::Array additional;
        for (int i = 1; i < outputs.size(); i++) {
            additional.append(outputs[i]);
        }
        if (!additional.is_empty()) {
            result["images"] = additional;
        }
    }
    result["success"] = runtime_errors.is_empty() && !outputs.is_empty();
    result["scripted"] = true;
    result["scene_path"] = _current_scene_path_ref();
    result["capture_count"] = ctx->get_capture_count();
    result["output_count"] = outputs.size();
    result["captured_image_count"] = outputs.size();
    const int output_count = int(outputs.size());
    result["model_image_count"] =
        std::min(output_count, MODEL_IMAGE_OUTPUT_LIMIT);
    result["omitted_image_count"] =
        std::max(0, output_count - MODEL_IMAGE_OUTPUT_LIMIT);
    result["logs"] = ctx->get_logs();
    result["runtime_errors"] = runtime_errors;
    if (!runtime_errors.is_empty()) {
        result["error"] = "Screenshot script execution failed.";
    }
    if (outputs.size() > MODEL_IMAGE_OUTPUT_LIMIT) {
        result["image_output_warning"] =
            godot::String::num_int64(outputs.size() - MODEL_IMAGE_OUTPUT_LIMIT) +
            " additional images were saved but omitted from model image context. Their paths remain in the result.";
    }
    _append_capture_script_receipt(result);
    _clear_script_capture_session(true);
    return result;
}

} // namespace fennara
