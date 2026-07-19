#include "fennara/tools/screenshot_scene.hpp"
#include "fennara/tools/screenshot_scene_script.hpp"

#include "fennara/helpers.hpp"
#include "fennara/tools/run_scene_edit_script/internal.hpp"
#include "fennara/warning_capture.hpp"

#include <godot_cpp/classes/gd_script.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

namespace {

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

bool is_scene_node(godot::Node *root, godot::Node *node) {
    return root != nullptr && node != nullptr &&
           (root == node || root->is_ancestor_of(node));
}

} // namespace

void FennaraScreenshotSceneScriptContext::_bind_methods() {
    godot::ClassDB::bind_method(godot::D_METHOD("get_root"),
                                &FennaraScreenshotSceneScriptContext::get_root);
    godot::ClassDB::bind_method(
        godot::D_METHOD("capture", "nodes", "options"),
        &FennaraScreenshotSceneScriptContext::capture,
        DEFVAL(godot::Dictionary()));
    godot::ClassDB::bind_method(godot::D_METHOD("log", "message"),
                                &FennaraScreenshotSceneScriptContext::log);
    godot::ClassDB::bind_method(godot::D_METHOD("error", "message"),
                                &FennaraScreenshotSceneScriptContext::error);
    ADD_PROPERTY(godot::PropertyInfo(godot::Variant::OBJECT, "root",
                                     godot::PROPERTY_HINT_NODE_TYPE, "Node"),
                 "", "get_root");
}

void FennaraScreenshotSceneScriptContext::configure(godot::Node *root) {
    _root = root;
    _capture_requested = false;
    _capture_nodes.clear();
    _capture_options.clear();
    _logs.clear();
    _errors.clear();
}

godot::Node *FennaraScreenshotSceneScriptContext::get_root() const {
    return _root;
}

void FennaraScreenshotSceneScriptContext::capture(
    const godot::Variant &nodes, const godot::Dictionary &options) {
    if (_capture_requested) {
        _errors.append(script_error(
            "ctx.capture() may only be called once per screenshot script.",
            "contract"));
        return;
    }
    _capture_requested = true;
    _capture_options = options.duplicate();

    godot::Array requested;
    if (nodes.get_type() == godot::Variant::ARRAY) {
        requested = nodes;
    } else if (nodes.get_type() == godot::Variant::OBJECT) {
        requested.append(nodes);
    } else {
        _errors.append(script_error(
            "ctx.capture() expects a Node or Array[Node].", "contract"));
        return;
    }

    for (int i = 0; i < requested.size(); i++) {
        godot::Object *object = requested[i];
        godot::Node *node = godot::Object::cast_to<godot::Node>(object);
        if (!is_scene_node(_root, node)) {
            _errors.append(script_error(
                "Every ctx.capture() subject must be the detached scene root or one of its descendants.",
                "contract"));
            continue;
        }
        if (!_capture_nodes.has(node)) {
            _capture_nodes.append(node);
        }
    }

    if (_capture_nodes.is_empty()) {
        _errors.append(script_error(
            "ctx.capture() did not receive any valid scene nodes.", "contract"));
    }
}

void FennaraScreenshotSceneScriptContext::log(const godot::String &message) {
    _logs.append(message);
}

void FennaraScreenshotSceneScriptContext::error(const godot::String &message) {
    _errors.append(script_error(message, "ctx"));
}

bool FennaraScreenshotSceneScriptContext::was_capture_requested() const {
    return _capture_requested;
}

godot::Array FennaraScreenshotSceneScriptContext::get_capture_nodes() const {
    return _capture_nodes;
}

godot::Dictionary FennaraScreenshotSceneScriptContext::get_capture_options() const {
    return _capture_options;
}

godot::Array FennaraScreenshotSceneScriptContext::get_logs() const {
    return _logs;
}

godot::Array FennaraScreenshotSceneScriptContext::get_errors() const {
    return _errors;
}

bool FennaraScreenshotSceneTool::_prepare_capture_script(
    const godot::Dictionary &args, godot::Dictionary &result) {
    _clear_capture_script();

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
        return false;
    }

    godot::String code = args.get("code", "");
    godot::String script_path = args.get("script_path", "");
    if (code.is_empty() && script_path.is_empty()) {
        return true;
    }
    if (!code.is_empty() && !script_path.is_empty()) {
        result["success"] = false;
        result["error"] = "Provide exactly one of code or script_path.";
        return false;
    }

    godot::String scene_path = normalize_path(args.get("scene_path", ""));
    capture_script_path() =
        run_scene_edit_script_internal::write_or_resolve_script_path(
            scene_path, code, script_path, result);
    if (capture_script_path().is_empty()) {
        return false;
    }

    capture_script_diagnostics() =
        run_scene_edit_script_internal::collect_script_diagnostics(
            capture_script_path());
    run_scene_edit_script_internal::apply_diagnostics_to_result(
        capture_script_diagnostics(), result);
    result["script_path"] = capture_script_path();
    if ((bool)capture_script_diagnostics().get("diagnostic_success", false) &&
        (int)capture_script_diagnostics().get("total_errors", 0) > 0) {
        result["success"] = false;
        result["error"] =
            "Screenshot script diagnostics reported errors. Patch script_path and rerun.";
        return false;
    }
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

bool FennaraScreenshotSceneTool::_run_capture_script(
    godot::Node *root, godot::Dictionary &result,
    godot::Array &capture_nodes, godot::Dictionary &capture_options) {
    capture_nodes.clear();
    capture_options.clear();
    if (!_has_capture_script()) {
        capture_nodes.append(root);
        return true;
    }

    godot::Ref<godot::GDScript> script =
        run_scene_edit_script_internal::load_script(capture_script_path(), result);
    if (!script.is_valid()) {
        return false;
    }

    godot::StringName base_type = script->get_instance_base_type();
    godot::StringName ref_counted_type("RefCounted");
    if (base_type != ref_counted_type &&
        !godot::ClassDB::is_parent_class(base_type, ref_counted_type)) {
        result["success"] = false;
        result["error"] =
            "screenshot_scene scripts must use `@tool extends RefCounted`.";
        result["runtime_errors"] = godot::Array::make(script_error(
            "Expected `@tool extends RefCounted`.", "contract"));
        return false;
    }

    godot::Variant runner_variant = script->new_();
    godot::Object *runner = runner_variant;
    if (runner == nullptr || !runner->has_method("run")) {
        result["success"] = false;
        result["error"] = "Screenshot script must define func run(ctx) -> void.";
        result["runtime_errors"] = godot::Array::make(script_error(
            "Missing required run(ctx) entrypoint.", "contract"));
        return false;
    }

    godot::Ref<FennaraScreenshotSceneScriptContext> ctx;
    ctx.instantiate();
    ctx->configure(root);

    godot::Ref<FennaraWarningCapture> warning_capture;
    warning_capture.instantiate();
    godot::OS::get_singleton()->add_logger(warning_capture);
    runner->call("run", ctx.ptr());
    godot::OS::get_singleton()->remove_logger(warning_capture);

    godot::Array runtime_errors = ctx->get_errors();
    run_scene_edit_script_internal::append_capture_errors(
        warning_capture->get_captured(), runtime_errors);
    if (!ctx->was_capture_requested()) {
        runtime_errors.append(script_error(
            "Screenshot script must call ctx.capture(nodes, options) exactly once.",
            "contract"));
    }

    result["logs"] = ctx->get_logs();
    result["runtime_errors"] = runtime_errors;
    _append_capture_script_receipt(result);
    if (!runtime_errors.is_empty()) {
        result["success"] = false;
        result["error"] = "Screenshot script execution failed.";
        return false;
    }

    capture_nodes = ctx->get_capture_nodes();
    capture_options = ctx->get_capture_options();
    result["scripted"] = true;
    result["script_subject_count"] = capture_nodes.size();
    return true;
}

} // namespace fennara
