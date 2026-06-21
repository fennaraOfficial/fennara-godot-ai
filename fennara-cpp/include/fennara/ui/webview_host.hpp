#pragma once

#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/variant/string.hpp>

#include <memory>

namespace fennara {

namespace webview_backend {
class NativeWebviewBackend;
}

class WebviewHost {
public:
    WebviewHost();
    ~WebviewHost();

    bool start(godot::Control *owner, const godot::String &url);
    void resize_to(godot::Control *owner);
    void set_visible(bool visible);
    void stop();
    bool is_started() const;

private:
    std::unique_ptr<webview_backend::NativeWebviewBackend> backend;
};

} // namespace fennara
