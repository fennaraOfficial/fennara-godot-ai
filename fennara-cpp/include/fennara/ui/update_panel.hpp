#pragma once

#include <godot_cpp/classes/button.hpp>
#include <godot_cpp/classes/confirmation_dialog.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/label.hpp>

namespace godot {
class ProgressBar;
}

namespace fennara {

class UpdateCoordinator;

class UpdatePanel : public godot::Control {
    GDCLASS(UpdatePanel, godot::Control)

public:
    void _ready() override;
    void set_coordinator(UpdateCoordinator *value);
    void refresh();

protected:
    static void _bind_methods();

private:
    UpdateCoordinator *coordinator = nullptr;
    godot::Label *title_label = nullptr;
    godot::Label *status_label = nullptr;
    godot::Label *error_label = nullptr;
    godot::Label *operation_label = nullptr;
    godot::Label *action_label = nullptr;
    godot::ProgressBar *progress = nullptr;
    godot::Button *close_button = nullptr;
    godot::Button *not_now_button = nullptr;
    godot::Button *retry_button = nullptr;
    godot::Button *restore_button = nullptr;
    godot::Button *cancel_button = nullptr;
    godot::Button *copy_button = nullptr;
    godot::Button *logs_button = nullptr;
    godot::ConfirmationDialog *close_confirmation = nullptr;
    godot::ConfirmationDialog *restore_confirmation = nullptr;

    void _build_ui();
    void _on_close_pressed();
    void _on_close_confirmed();
    void _on_restore_pressed();
    void _on_restore_confirmed();
    void _on_copy_report_pressed();
};

} // namespace fennara
