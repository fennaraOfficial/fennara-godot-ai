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

constexpr int MAX_SCREENSHOT_CAPTURES = 6;

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
    _capture_requests.clear();
    _logs.clear();
    _errors.clear();
}

godot::Node *FennaraScreenshotSceneScriptContext::get_root() const {
    return _root;
}

void FennaraScreenshotSceneScriptContext::capture(
    const godot::Variant &nodes, const godot::Dictionary &options) {
    if (_capture_requests.size() >= MAX_SCREENSHOT_CAPTURES) {
        _errors.append(script_error(
            "A screenshot script may request at most six captures.",
            "contract"));
        return;
    }

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

    godot::Array capture_nodes;
    for (int i = 0; i < requested.size(); i++) {
        godot::Object *object = requested[i];
        godot::Node *node = godot::Object::cast_to<godot::Node>(object);
        if (!is_scene_node(_root, node)) {
            _errors.append(script_error(
                "Every ctx.capture() subject must be the detached scene root or one of its descendants.",
                "contract"));
            continue;
        }
        if (!capture_nodes.has(node)) {
            capture_nodes.append(node);
        }
    }

    if (capture_nodes.is_empty()) {
        _errors.append(script_error(
            "ctx.capture() did not receive any valid scene nodes.", "contract"));
        return;
    }

    godot::Dictionary request;
    request["nodes"] = capture_nodes;
    request["options"] = options.duplicate();
    _capture_requests.append(request);
}

void FennaraScreenshotSceneScriptContext::log(const godot::String &message) {
    _logs.append(message);
}

void FennaraScreenshotSceneScriptContext::error(const godot::String &message) {
    _errors.append(script_error(message, "ctx"));
}

bool FennaraScreenshotSceneScriptContext::was_capture_requested() const {
    return !_capture_requests.is_empty();
}

godot::Array FennaraScreenshotSceneScriptContext::get_capture_requests() const {
    return _capture_requests;
}

godot::Array FennaraScreenshotSceneScriptContext::get_logs() const {
    return _logs;
}

godot::Array FennaraScreenshotSceneScriptContext::get_errors() const {
    return _errors;
}

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

bool FennaraScreenshotSceneTool::_run_capture_script(
    godot::Node *root, godot::Dictionary &result,
    godot::Array &capture_nodes, godot::Dictionary &capture_options,
    int capture_index) {
    capture_nodes.clear();
    capture_options.clear();
    if (!_has_capture_script()) {
        if (capture_index != 0) {
            result["success"] = false;
            result["error"] = "Whole-scene capture has only one image.";
            return false;
        }
        capture_nodes.append(root);
        result["capture_index"] = 0;
        result["capture_count"] = 1;
        return true;
    }

    godot::Array &capture_requests = _script_capture_requests_ref();
    if (capture_index > 0) {
        if (root != _script_capture_root_ref() || capture_requests.is_empty()) {
            result["success"] = false;
            result["error"] =
                "Screenshot capture queue was not available for the retained scene.";
            return false;
        }
        result = _script_capture_receipt_ref().duplicate();
    } else {
        _clear_script_capture_session(false);

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
                "Screenshot script must call ctx.capture(nodes, options) at least once.",
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

        capture_requests = ctx->get_capture_requests();
        _script_capture_root_ref() = root;
        _script_capture_receipt_ref() = result.duplicate();
    }

    if (capture_index < 0 || capture_index >= capture_requests.size()) {
        result["success"] = false;
        result["error"] = "Screenshot capture index was outside the script request queue.";
        return false;
    }
    godot::Dictionary request = capture_requests[capture_index];
    capture_nodes = request.get("nodes", godot::Array());
    capture_options = request.get("options", godot::Dictionary());
    result["scripted"] = true;
    result["script_subject_count"] = capture_nodes.size();
    result["capture_index"] = capture_index;
    result["capture_count"] = capture_requests.size();
    return true;
}

} // namespace fennara
