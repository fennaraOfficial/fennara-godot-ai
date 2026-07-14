#pragma once

#include <godot_cpp/classes/button.hpp>
#include <godot_cpp/classes/check_box.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/input_event.hpp>
#include <godot_cpp/classes/label.hpp>
#include <godot_cpp/classes/ref.hpp>
#include <memory>

namespace fennara {

class FennaraLocalBridge;
class FirstRunSetup;
class FirstRunSetupPanel;
class WebviewHost;
class UpdateCoordinator;
class UpdatePanel;

class FennaraDock : public godot::Control {
    GDCLASS(FennaraDock, godot::Control)

protected:
    static void _bind_methods();
    void _notification(int what);

private:
    FennaraLocalBridge *local_bridge = nullptr;
    godot::Control *webview_region = nullptr;
    godot::Control *internal_webview_surface = nullptr;
    godot::Label *fallback_label = nullptr;
    godot::Label *staging_badge = nullptr;
    godot::Control *browser_fallback_panel = nullptr;
    FirstRunSetup *first_run_setup = nullptr;
    FirstRunSetupPanel *setup_panel = nullptr;
    UpdateCoordinator *update_coordinator = nullptr;
    UpdatePanel *update_panel = nullptr;
    godot::Label *browser_fallback_message = nullptr;
    godot::Label *browser_restart_label = nullptr;
    godot::Button *open_browser_button = nullptr;
    godot::CheckBox *use_embedded_checkbox = nullptr;
    godot::String browser_chat_url;
    std::unique_ptr<WebviewHost> webview_host;
    double refresh_timer = 0.0;
    double startup_delay = 0.0;
    bool attempted_webview = false;
    bool webview_keyboard_focused = false;
    int stable_geometry_frames = 0;
    godot::Vector2 last_region_position;
    godot::Vector2 last_region_size;

    void _build_ui();
    void _try_start_webview();
    void _sync_webview_bounds();
    void _set_webview_keyboard_focus(bool focused);
    void _release_webview_keyboard_focus();
    bool _event_is_inside_webview_region(const godot::Ref<godot::InputEvent> &event);
    void _refresh_status();
    void _refresh_staging_badge();
    void _on_mcp_target_state_changed(bool active);
    void _on_setup_succeeded();
    void _on_update_requested();
    void _on_open_browser_pressed();
    void _on_use_embedded_toggled(bool pressed);
    godot::String _chat_url() const;
    godot::String _browser_chat_url() const;
    bool _chat_surface_prefers_browser() const;
    bool _save_chat_surface(const godot::String &surface) const;
    void _show_browser_fallback(const godot::String &message);
    bool _show_setup_if_needed();
    bool _show_update_if_needed();
    bool _webview_region_is_stable();
    void _output_log(const godot::String &message) const;

public:
    FennaraDock();
    ~FennaraDock();

    static void release_active_webview_keyboard_focus();

    void set_local_bridge(FennaraLocalBridge *bridge);
    void _ready() override;
    void _input(const godot::Ref<godot::InputEvent> &event) override;
    void _process(double delta) override;
    void _gui_input(const godot::Ref<godot::InputEvent> &event) override;
};

} // namespace fennara
