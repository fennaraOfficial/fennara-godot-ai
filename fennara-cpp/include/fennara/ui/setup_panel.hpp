#pragma once

#include <godot_cpp/classes/control.hpp>

namespace godot {
class Button;
class Label;
class ProgressBar;
} // namespace godot

namespace fennara {

class FirstRunSetup;

class FirstRunSetupPanel : public godot::Control {
    GDCLASS(FirstRunSetupPanel, godot::Control)

  protected:
    static void _bind_methods();

  public:
    void _ready() override;
    void set_setup(FirstRunSetup *value);
    void refresh();

  private:
    FirstRunSetup *setup = nullptr;
    godot::Label *status_label = nullptr;
    godot::Label *detail_label = nullptr;
    godot::Label *error_label = nullptr;
    godot::Label *operation_label = nullptr;
    godot::Label *action_label = nullptr;
    godot::ProgressBar *progress = nullptr;
    godot::Button *setup_button = nullptr;
    godot::Button *retry_button = nullptr;
    godot::Button *copy_report_button = nullptr;
    godot::Button *open_logs_button = nullptr;

    void _build_ui();
    void _on_setup_pressed();
    void _on_retry_pressed();
    void _on_copy_report_pressed();
    void _on_open_logs_pressed();
};

} // namespace fennara
