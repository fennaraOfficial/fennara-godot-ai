#include "fennara/ui/setup_panel.hpp"

#include "fennara/setup/first_run_setup.hpp"

#include <godot_cpp/classes/button.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/h_box_container.hpp>
#include <godot_cpp/classes/label.hpp>
#include <godot_cpp/classes/margin_container.hpp>
#include <godot_cpp/classes/panel_container.hpp>
#include <godot_cpp/classes/progress_bar.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/v_box_container.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {
namespace {

godot::Label *make_wrapped_label(const godot::String &text) {
    godot::Label *label = memnew(godot::Label);
    label->set_text(text);
    label->set_autowrap_mode(godot::TextServer::AUTOWRAP_WORD_SMART);
    label->set_horizontal_alignment(godot::HORIZONTAL_ALIGNMENT_CENTER);
    label->set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    return label;
}

godot::String read_addon_version() {
    return godot::FileAccess::get_file_as_string("res://addons/fennara/VERSION").strip_edges();
}

} // namespace

void FirstRunSetupPanel::_bind_methods() {
    godot::ClassDB::bind_method(godot::D_METHOD("_on_setup_pressed"),
                                &FirstRunSetupPanel::_on_setup_pressed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_retry_pressed"),
                                &FirstRunSetupPanel::_on_retry_pressed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_copy_report_pressed"),
                                &FirstRunSetupPanel::_on_copy_report_pressed);
    godot::ClassDB::bind_method(godot::D_METHOD("_on_open_logs_pressed"),
                                &FirstRunSetupPanel::_on_open_logs_pressed);
    godot::ClassDB::bind_method(godot::D_METHOD("refresh"), &FirstRunSetupPanel::refresh);
}

void FirstRunSetupPanel::_ready() {
    _build_ui();
    refresh();
}

void FirstRunSetupPanel::set_setup(FirstRunSetup *value) {
    if (setup != nullptr) {
        const godot::Callable callback = callable_mp(this, &FirstRunSetupPanel::refresh);
        if (setup->is_connected("state_changed", callback)) {
            setup->disconnect("state_changed", callback);
        }
    }
    setup = value;
    if (setup != nullptr) {
        setup->connect("state_changed", callable_mp(this, &FirstRunSetupPanel::refresh));
    }
    refresh();
}

void FirstRunSetupPanel::_build_ui() {
    set_anchors_preset(godot::Control::PRESET_FULL_RECT);
    set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    set_v_size_flags(godot::Control::SIZE_EXPAND_FILL);

    godot::PanelContainer *panel = memnew(godot::PanelContainer);
    panel->set_anchors_preset(godot::Control::PRESET_FULL_RECT);
    panel->set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    panel->set_v_size_flags(godot::Control::SIZE_EXPAND_FILL);
    add_child(panel);

    godot::MarginContainer *margin = memnew(godot::MarginContainer);
    margin->add_theme_constant_override("margin_left", 32);
    margin->add_theme_constant_override("margin_top", 36);
    margin->add_theme_constant_override("margin_right", 32);
    margin->add_theme_constant_override("margin_bottom", 36);
    panel->add_child(margin);

    godot::VBoxContainer *content = memnew(godot::VBoxContainer);
    content->set_alignment(godot::BoxContainer::ALIGNMENT_CENTER);
    content->add_theme_constant_override("separation", 12);
    content->set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    content->set_v_size_flags(godot::Control::SIZE_EXPAND_FILL);
    margin->add_child(content);

    godot::Label *title = make_wrapped_label("Finish setting up Fennara");
    title->add_theme_font_size_override("font_size", 22);
    content->add_child(title);

    status_label = make_wrapped_label("Fennara needs to finish setup.");
    status_label->add_theme_font_size_override("font_size", 16);
    content->add_child(status_label);

    detail_label =
        make_wrapped_label("The matching CLI, daemon, MCP server, and runtime will be installed in "
                           "Fennara app data. Your project addon will not be replaced.");
    detail_label->add_theme_color_override("font_color", godot::Color("#a8b0ba"));
    content->add_child(detail_label);

    progress = memnew(godot::ProgressBar);
    progress->set_indeterminate(true);
    progress->set_show_percentage(false);
    progress->set_custom_minimum_size(godot::Vector2(280, 8));
    progress->set_h_size_flags(godot::Control::SIZE_EXPAND_FILL);
    progress->set_visible(false);
    content->add_child(progress);

    error_label = make_wrapped_label("");
    error_label->add_theme_color_override("font_color", godot::Color("#ff8b8b"));
    error_label->set_visible(false);
    content->add_child(error_label);

    operation_label = make_wrapped_label("");
    operation_label->add_theme_color_override("font_color", godot::Color("#8fa3b8"));
    operation_label->set_visible(false);
    content->add_child(operation_label);

    godot::HBoxContainer *primary_buttons = memnew(godot::HBoxContainer);
    primary_buttons->set_alignment(godot::BoxContainer::ALIGNMENT_CENTER);
    primary_buttons->add_theme_constant_override("separation", 8);
    content->add_child(primary_buttons);

    setup_button = memnew(godot::Button);
    setup_button->set_text("Set Up Fennara");
    setup_button->connect("pressed", callable_mp(this, &FirstRunSetupPanel::_on_setup_pressed));
    primary_buttons->add_child(setup_button);

    retry_button = memnew(godot::Button);
    retry_button->set_text("Retry");
    retry_button->connect("pressed", callable_mp(this, &FirstRunSetupPanel::_on_retry_pressed));
    retry_button->set_visible(false);
    primary_buttons->add_child(retry_button);

    godot::HBoxContainer *support_buttons = memnew(godot::HBoxContainer);
    support_buttons->set_alignment(godot::BoxContainer::ALIGNMENT_CENTER);
    support_buttons->add_theme_constant_override("separation", 8);
    support_buttons->set_visible(false);
    content->add_child(support_buttons);

    copy_report_button = memnew(godot::Button);
    copy_report_button->set_text("Copy Report");
    copy_report_button->connect("pressed",
                                callable_mp(this, &FirstRunSetupPanel::_on_copy_report_pressed));
    support_buttons->add_child(copy_report_button);

    open_logs_button = memnew(godot::Button);
    open_logs_button->set_text("Open Logs");
    open_logs_button->connect("pressed",
                              callable_mp(this, &FirstRunSetupPanel::_on_open_logs_pressed));
    support_buttons->add_child(open_logs_button);

    action_label = make_wrapped_label("");
    action_label->add_theme_color_override("font_color", godot::Color("#8fc79f"));
    content->add_child(action_label);
}

void FirstRunSetupPanel::refresh() {
    if (setup == nullptr || status_label == nullptr) {
        return;
    }

    const bool running = setup->is_running();
    const bool failed = setup->has_failed();
    status_label->set_text(setup->get_status());
    if (!setup->get_detail().is_empty()) {
        detail_label->set_text(setup->get_detail());
    }
    progress->set_visible(running);
    setup_button->set_visible(!running && !failed && !setup->has_succeeded());
    retry_button->set_visible(failed);

    error_label->set_visible(failed);
    error_label->set_text(setup->get_error_code().is_empty() ? godot::String()
                                                             : "Error: " + setup->get_error_code());

    const godot::String operation = setup->get_operation_id();
    operation_label->set_visible(!operation.is_empty());
    operation_label->set_text(operation.is_empty() ? godot::String() : "Operation: " + operation);

    godot::Control *support =
        copy_report_button != nullptr
            ? godot::Object::cast_to<godot::Control>(copy_report_button->get_parent())
            : nullptr;
    if (support != nullptr) {
        support->set_visible(failed);
    }
}

void FirstRunSetupPanel::_on_setup_pressed() {
    if (setup == nullptr) {
        return;
    }
    action_label->set_text("");
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    const godot::String path =
        settings != nullptr ? settings->globalize_path("res://") : godot::String();
    setup->start(path, read_addon_version());
}

void FirstRunSetupPanel::_on_retry_pressed() {
    if (setup != nullptr) {
        action_label->set_text("");
        setup->retry();
    }
}

void FirstRunSetupPanel::_on_copy_report_pressed() {
    if (setup != nullptr) {
        setup->copy_report();
        action_label->set_text("Sanitized setup report copied.");
    }
}

void FirstRunSetupPanel::_on_open_logs_pressed() {
    if (setup != nullptr) {
        setup->open_logs();
    }
}

} // namespace fennara
