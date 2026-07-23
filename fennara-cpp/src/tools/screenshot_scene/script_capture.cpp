#include "fennara/tools/screenshot_scene.hpp"
#include "fennara/tools/screenshot_scene_script.hpp"

#include "fennara/tools/run_scene_edit_script/internal.hpp"

#include <godot_cpp/core/class_db.hpp>

namespace fennara {

namespace {

godot::Dictionary script_error(const godot::String &message,
                               const godot::String &source) {
    return run_scene_edit_script_internal::make_runtime_error(message, source);
}

bool is_scene_node(godot::Node *root, godot::Node *node) {
    return root != nullptr && node != nullptr &&
           (root == node || root->is_ancestor_of(node));
}

} // namespace

FennaraScreenshotSceneScriptContext::
    ~FennaraScreenshotSceneScriptContext() {
    if (_root && !_root->get_parent()) {
        memdelete(_root);
    }
    _root = nullptr;
}

void FennaraScreenshotSceneScriptContext::_bind_methods() {
    godot::ClassDB::bind_method(godot::D_METHOD("get_root"),
                                &FennaraScreenshotSceneScriptContext::get_root);
    godot::ClassDB::bind_method(
        godot::D_METHOD("capture", "nodes", "options"),
        &FennaraScreenshotSceneScriptContext::capture,
        DEFVAL(godot::Dictionary()));
    godot::ClassDB::bind_method(
        godot::D_METHOD("output", "image", "description"),
        &FennaraScreenshotSceneScriptContext::output,
        DEFVAL(godot::String()));
    godot::ClassDB::bind_method(godot::D_METHOD("log", "message"),
                                &FennaraScreenshotSceneScriptContext::log);
    godot::ClassDB::bind_method(godot::D_METHOD("error", "message"),
                                &FennaraScreenshotSceneScriptContext::error);
    godot::ClassDB::bind_method(
        godot::D_METHOD("_emit_empty_capture_completed"),
        &FennaraScreenshotSceneScriptContext::_emit_empty_capture_completed);
    ADD_PROPERTY(godot::PropertyInfo(godot::Variant::OBJECT, "root",
                                     godot::PROPERTY_HINT_NODE_TYPE, "Node"),
                 "", "get_root");
    ADD_SIGNAL(godot::MethodInfo("capture_requested"));
    ADD_SIGNAL(godot::MethodInfo(
        "capture_completed",
        godot::PropertyInfo(godot::Variant::OBJECT, "image",
                            godot::PROPERTY_HINT_RESOURCE_TYPE, "Image")));
}

void FennaraScreenshotSceneScriptContext::configure(
    godot::Node *root, const godot::String &scene_path) {
    _root = root;
    _root_accessible = true;
    _scene_path = scene_path;
    _pending_capture.clear();
    _capture_pending = false;
    _cancelled = false;
    _capture_count = 0;
    _outputs.clear();
    _logs.clear();
    _errors.clear();
}

godot::Node *FennaraScreenshotSceneScriptContext::get_root() const {
    return _root_accessible ? _root : nullptr;
}

godot::Signal FennaraScreenshotSceneScriptContext::capture(
    const godot::Variant &nodes, const godot::Dictionary &options) {
    godot::Signal completed(this, "capture_completed");
    if (_cancelled) {
        _errors.append(script_error(
            "ctx.capture() was called after the screenshot run was cancelled.",
            "contract"));
        call_deferred("_emit_empty_capture_completed");
        return completed;
    }
    if (_capture_pending) {
        _errors.append(script_error(
            "Await each ctx.capture() before requesting another capture.",
            "contract"));
        call_deferred("_emit_empty_capture_completed");
        return completed;
    }

    godot::Array requested;
    if (nodes.get_type() == godot::Variant::ARRAY) {
        requested = nodes;
    } else if (nodes.get_type() == godot::Variant::OBJECT) {
        requested.append(nodes);
    } else {
        _errors.append(script_error(
            "ctx.capture() expects a Node or Array[Node].", "contract"));
        call_deferred("_emit_empty_capture_completed");
        return completed;
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
            "ctx.capture() did not receive any valid scene nodes.",
            "contract"));
        call_deferred("_emit_empty_capture_completed");
        return completed;
    }

    _pending_capture["nodes"] = capture_nodes;
    _pending_capture["options"] = options.duplicate(true);
    _capture_pending = true;
    _capture_count++;
    emit_signal("capture_requested");
    return completed;
}

godot::Dictionary FennaraScreenshotSceneScriptContext::output(
    const godot::Ref<godot::Image> &image,
    const godot::String &description) {
    if (_cancelled) {
        godot::Dictionary result;
        result["success"] = false;
        result["error"] =
            "ctx.output() was called after the screenshot run was cancelled.";
        return result;
    }
    godot::Dictionary result = FennaraScreenshotSceneTool::publish_image(
        image, description, _outputs.size());
    if ((bool)result.get("success", false)) {
        _outputs.append(result);
    } else {
        _errors.append(script_error(
            result.get("error", "ctx.output() failed to publish the image."),
            "ctx.output"));
    }
    return result;
}

void FennaraScreenshotSceneScriptContext::log(
    const godot::String &message) {
    if (_cancelled) {
        return;
    }
    _logs.append(message);
}

void FennaraScreenshotSceneScriptContext::error(
    const godot::String &message) {
    if (_cancelled) {
        return;
    }
    _errors.append(script_error(message, "ctx"));
}

bool FennaraScreenshotSceneScriptContext::was_capture_requested() const {
    return _capture_count > 0;
}

bool FennaraScreenshotSceneScriptContext::has_pending_capture() const {
    return _capture_pending;
}

godot::Dictionary
FennaraScreenshotSceneScriptContext::take_pending_capture() {
    if (!_capture_pending) {
        return godot::Dictionary();
    }
    godot::Dictionary request = _pending_capture.duplicate(true);
    _pending_capture.clear();
    return request;
}

void FennaraScreenshotSceneScriptContext::complete_capture(
    const godot::Ref<godot::Image> &image) {
    if (!_capture_pending) {
        return;
    }
    _capture_pending = false;
    emit_signal("capture_completed", image);
}

void FennaraScreenshotSceneScriptContext::fail_capture(
    const godot::String &message) {
    if (!message.is_empty()) {
        _errors.append(script_error(message, "capture"));
    }
    complete_capture(godot::Ref<godot::Image>());
}

void FennaraScreenshotSceneScriptContext::cancel(
    const godot::String &message) {
    _cancelled = true;
    _root_accessible = false;
    const godot::String cancellation_message =
        message.is_empty()
            ? godot::String("Screenshot capture was cancelled.")
            : message;
    _errors.append(script_error(cancellation_message, "session"));
    if (_capture_pending) {
        complete_capture(godot::Ref<godot::Image>());
    }
}

int FennaraScreenshotSceneScriptContext::get_capture_count() const {
    return _capture_count;
}

godot::Array FennaraScreenshotSceneScriptContext::get_outputs() const {
    return _outputs;
}

godot::Array FennaraScreenshotSceneScriptContext::get_logs() const {
    return _logs;
}

godot::Array FennaraScreenshotSceneScriptContext::get_errors() const {
    return _errors;
}

void FennaraScreenshotSceneScriptContext::_emit_empty_capture_completed() {
    emit_signal("capture_completed", godot::Ref<godot::Image>());
}

} // namespace fennara
