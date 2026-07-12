#include "fennara/ui/update_panel.hpp"

#include "fennara/update/update_coordinator.hpp"

#include <godot_cpp/classes/h_box_container.hpp>
#include <godot_cpp/classes/margin_container.hpp>
#include <godot_cpp/classes/panel_container.hpp>
#include <godot_cpp/classes/progress_bar.hpp>
#include <godot_cpp/classes/v_box_container.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

void UpdatePanel::_bind_methods() {
    godot::ClassDB::bind_method(godot::D_METHOD("refresh"), &UpdatePanel::refresh);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_close_pressed"),
                                &UpdatePanel::_on_close_pressed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_close_confirmed"),
                                &UpdatePanel::_on_close_confirmed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_restore_pressed"),
                                &UpdatePanel::_on_restore_pressed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_restore_confirmed"),
                                &UpdatePanel::_on_restore_confirmed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_copy_report_pressed"),
                                &UpdatePanel::_on_copy_report_pressed);
}

void UpdatePanel::_ready() {
    _build_ui();
    refresh();
}

void UpdatePanel::set_coordinator(UpdateCoordinator *value) {
    if (coordinator != nullptr) {
        const godot::Callable callback = callable_mp(this, &UpdatePanel::refresh);
        if (coordinator->is_connected("state_changed", callback)) {
            coordinator->disconnect("state_changed", callback);
        }
    }
    coordinator = value;
    if (coordinator != nullptr) {
        coordinator->connect("state_changed", callable_mp(this, &UpdatePanel::refresh));
    }
    refresh();
}

void UpdatePanel::_build_ui() {
    set_anchors_preset(godot::Control::PRESET_FULL_RECT);
    set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    set_v_size_flags(godot::Control::SIZE_EXPAND_FILL);

    godot::PanelContainer *panel = memnew(godot::PanelContainer);
    panel->set_anchors_preset(godot::Control::PRESET_FULL_RECT);
    panel->set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    panel->set_v_size_flags(godot::Control::SIZE_EXPAND_FILL);
    add_child(panel);

    godot::MarginContainer *margin = memnew(godot::MarginContainer);
    margin->add_theme_constant_override("margin_left", 28);
    margin->add_theme_constant_override("margin_right", 28);
    margin->add_theme_constant_override("margin_top", 40);
    margin->add_theme_constant_override("margin_bottom", 28);
    panel->add_child(margin);

    godot::VBoxContainer *content = memnew(godot::VBoxContainer);
    content->set_alignment(godot::BoxContainer::ALIGNMENT_CENTER);
    content->add_theme_constant_override("separation", 14);
    margin->add_child(content);

    title_label = memnew(godot::Label);
    title_label->set_text("Update Fennara");
    title_label->set_horizontal_alignment(godot::HORIZONTAL_ALIGNMENT_CENTER);
    title_label->add_theme_font_size_override("font_size", 22);
    content->add_child(title_label);

    status_label = memnew(godot::Label);
    status_label->set_autowrap_mode(godot::TextServer::AUTOWRAP_WORD_SMART);
    status_label->set_horizontal_alignment(godot::HORIZONTAL_ALIGNMENT_CENTER);
    content->add_child(status_label);

    progress = memnew(godot::ProgressBar);
    progress->set_indeterminate(true);
    progress->set_show_percentage(false);
    progress->set_custom_minimum_size(godot::Vector2(280, 8));
    progress->set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    content->add_child(progress);

    operation_label = memnew(godot::Label);
    operation_label->set_horizontal_alignment(godot::HORIZONTAL_ALIGNMENT_CENTER);
    operation_label->add_theme_color_override("font_color", godot::Color("#8fa3b8"));
    content->add_child(operation_label);

    error_label = memnew(godot::Label);
    error_label->set_horizontal_alignment(godot::HORIZONTAL_ALIGNMENT_CENTER);
    error_label->add_theme_color_override("font_color", godot::Color("#ff8585"));
    content->add_child(error_label);

    godot::HBoxContainer *primary = memnew(godot::HBoxContainer);
    primary->set_alignment(godot::BoxContainer::ALIGNMENT_CENTER);
    primary->add_theme_constant_override("separation", 8);
    content->add_child(primary);

    close_button = memnew(godot::Button);
    close_button->set_text("Install Update...");
    close_button->connect("pressed", callable_mp(this, &UpdatePanel::_on_close_pressed));
    primary->add_child(close_button);

    not_now_button = memnew(godot::Button);
    not_now_button->set_text("Not Now");
    not_now_button->connect("pressed", callable_mp(coordinator, &UpdateCoordinator::dismiss));
    primary->add_child(not_now_button);

    retry_button = memnew(godot::Button);
    retry_button->set_text("Retry");
    retry_button->connect("pressed", callable_mp(coordinator, &UpdateCoordinator::retry));
    primary->add_child(retry_button);

    restore_button = memnew(godot::Button);
    restore_button->set_text("Restore Previous Version");
    restore_button->connect("pressed", callable_mp(this, &UpdatePanel::_on_restore_pressed));
    primary->add_child(restore_button);

    cancel_button = memnew(godot::Button);
    cancel_button->set_text("Cancel");
    cancel_button->connect("pressed", callable_mp(coordinator, &UpdateCoordinator::cancel_waiting));
    primary->add_child(cancel_button);

    godot::HBoxContainer *diagnostics = memnew(godot::HBoxContainer);
    diagnostics->set_alignment(godot::BoxContainer::ALIGNMENT_CENTER);
    diagnostics->add_theme_constant_override("separation", 8);
    content->add_child(diagnostics);

    copy_button = memnew(godot::Button);
    copy_button->set_text("Copy Report");
    copy_button->connect("pressed", callable_mp(this, &UpdatePanel::_on_copy_report_pressed));
    diagnostics->add_child(copy_button);

    logs_button = memnew(godot::Button);
    logs_button->set_text("Open Logs");
    logs_button->connect("pressed", callable_mp(coordinator, &UpdateCoordinator::open_logs));
    diagnostics->add_child(logs_button);

    action_label = memnew(godot::Label);
    action_label->set_horizontal_alignment(godot::HORIZONTAL_ALIGNMENT_CENTER);
    action_label->add_theme_color_override("font_color", godot::Color("#8fc79f"));
    content->add_child(action_label);

    close_confirmation = memnew(godot::ConfirmationDialog);
    close_confirmation->set_title("Close Godot and install Fennara?");
    close_confirmation->set_text(
        "Godot will handle unsaved-work confirmation normally. The verified update will only be installed after this editor process exits.");
    close_confirmation->get_ok_button()->set_text("Close Godot and Install");
    close_confirmation->connect("confirmed", callable_mp(this, &UpdatePanel::_on_close_confirmed));
    add_child(close_confirmation);

    restore_confirmation = memnew(godot::ConfirmationDialog);
    restore_confirmation->set_title("Restore the previous Fennara version?");
    restore_confirmation->set_text(
        "Godot must close briefly so the external updater can restore the saved addon and runtime selection.");
    restore_confirmation->get_ok_button()->set_text("Close Godot and Restore");
    restore_confirmation->connect("confirmed",
                                  callable_mp(this, &UpdatePanel::_on_restore_confirmed));
    add_child(restore_confirmation);
}

void UpdatePanel::refresh() {
    if (coordinator == nullptr || title_label == nullptr) {
        return;
    }
    if (coordinator->is_preparing()) {
        title_label->set_text("Preparing Fennara update");
    } else if (coordinator->is_ready_to_close()) {
        title_label->set_text("Ready to install Fennara");
    } else if (coordinator->is_waiting_for_godot()) {
        title_label->set_text("Installing Fennara update");
    } else {
        title_label->set_text("Update Fennara");
    }
    status_label->set_text(coordinator->get_status() + "\n\n" + coordinator->get_detail());
    progress->set_visible(coordinator->is_preparing() || coordinator->is_waiting_for_godot());
    const godot::String operation = coordinator->get_operation_id();
    operation_label->set_visible(!operation.is_empty());
    operation_label->set_text(operation.is_empty() ? godot::String() : "Operation: " + operation);
    error_label->set_text(coordinator->get_error_code().is_empty()
                              ? godot::String()
                              : "Error: " + coordinator->get_error_code());
    close_button->set_visible(coordinator->is_ready_to_close());
    not_now_button->set_visible(coordinator->is_ready_to_close());
    retry_button->set_visible(coordinator->has_failed());
    restore_button->set_visible(coordinator->needs_recovery());
    cancel_button->set_visible(coordinator->is_waiting_for_godot());
    const bool diagnostics = coordinator->has_failed() || coordinator->needs_recovery();
    copy_button->set_visible(diagnostics);
    logs_button->set_visible(diagnostics);
    if (!diagnostics) {
        action_label->set_text("");
    }
}

void UpdatePanel::_on_close_pressed() { close_confirmation->popup_centered(); }
void UpdatePanel::_on_close_confirmed() { coordinator->confirm_close_and_install(); }
void UpdatePanel::_on_restore_pressed() { restore_confirmation->popup_centered(); }
void UpdatePanel::_on_restore_confirmed() { coordinator->restore_previous_version(); }
void UpdatePanel::_on_copy_report_pressed() {
    coordinator->copy_report();
    action_label->set_text("Sanitized update report copied.");
}

} // namespace fennara
