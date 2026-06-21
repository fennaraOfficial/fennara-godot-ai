#pragma once

#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/variant/string.hpp>

#include <memory>

namespace fennara {
namespace webview_backend {

void output_log(const godot::String &message);
void output_error(const godot::String &message);

class NativeWebviewBackend {
public:
    virtual ~NativeWebviewBackend() = default;

    virtual bool start(godot::Control *owner, const godot::String &url) = 0;
    virtual void resize_to(godot::Control *owner) = 0;
    virtual void set_visible(bool visible) = 0;
    virtual void stop() = 0;
    virtual bool is_started() const = 0;
};

std::unique_ptr<NativeWebviewBackend> create_backend();

} // namespace webview_backend
} // namespace fennara
