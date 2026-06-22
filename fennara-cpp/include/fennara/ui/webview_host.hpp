#pragma once

#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/input_event.hpp>
#include <godot_cpp/classes/ref.hpp>
#include <godot_cpp/variant/string.hpp>

#include <cstdint>
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
    bool uses_internal_surface() const;
    godot::Control *create_internal_control();
    void resize_to(godot::Control *owner);
    void set_visible(bool visible);
    void process(double delta);
    bool handle_input(const godot::Ref<godot::InputEvent> &event);
    void set_focused(bool focused);
    void notify_mouse_leave();
    void stop();
    bool is_started() const;

private:
    godot::Control *current_internal_control() const;

    std::unique_ptr<webview_backend::NativeWebviewBackend> backend;
    godot::Control *internal_control = nullptr;
    uint64_t internal_control_id = 0;
};

} // namespace fennara
