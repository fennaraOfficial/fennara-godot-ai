#pragma once

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara {

class FennaraScreenshotSceneScriptContext : public godot::RefCounted {
    GDCLASS(FennaraScreenshotSceneScriptContext, godot::RefCounted);

protected:
    static void _bind_methods();

public:
    void configure(godot::Node *root);

    godot::Node *get_root() const;
    void capture(const godot::Variant &nodes,
                 const godot::Dictionary &options = godot::Dictionary());
    void log(const godot::String &message);
    void error(const godot::String &message);

    bool was_capture_requested() const;
    godot::Array get_capture_nodes() const;
    godot::Dictionary get_capture_options() const;
    godot::Array get_logs() const;
    godot::Array get_errors() const;

private:
    godot::Node *_root = nullptr;
    bool _capture_requested = false;
    godot::Array _capture_nodes;
    godot::Dictionary _capture_options;
    godot::Array _logs;
    godot::Array _errors;
};

} // namespace fennara
