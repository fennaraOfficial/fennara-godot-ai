#pragma once

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/classes/image.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/signal.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara {

class FennaraScreenshotSceneScriptContext : public godot::RefCounted {
    GDCLASS(FennaraScreenshotSceneScriptContext, godot::RefCounted);

protected:
    static void _bind_methods();

public:
    ~FennaraScreenshotSceneScriptContext();

    void configure(godot::Node *root, const godot::String &scene_path);

    godot::Node *get_root() const;
    godot::Signal capture(
        const godot::Variant &nodes,
        const godot::Dictionary &options = godot::Dictionary());
    godot::Dictionary output(
        const godot::Ref<godot::Image> &image,
        const godot::String &description = godot::String());
    void log(const godot::String &message);
    void error(const godot::String &message);

    bool was_capture_requested() const;
    bool has_pending_capture() const;
    godot::Dictionary take_pending_capture();
    void complete_capture(const godot::Ref<godot::Image> &image);
    void fail_capture(const godot::String &message);
    void cancel(const godot::String &message);
    int get_capture_count() const;
    godot::Array get_outputs() const;
    godot::Array get_logs() const;
    godot::Array get_errors() const;

    void _emit_empty_capture_completed();

private:
    godot::Node *_root = nullptr;
    bool _root_accessible = false;
    godot::String _scene_path;
    godot::Dictionary _pending_capture;
    bool _capture_pending = false;
    bool _cancelled = false;
    int _capture_count = 0;
    godot::Array _outputs;
    godot::Array _logs;
    godot::Array _errors;
};

} // namespace fennara
