#include "fennara/ui/script_context_menu.hpp"

#include "fennara/local_bridge.hpp"
#include "fennara/logger.hpp"

#include <godot_cpp/classes/code_edit.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/script.hpp>
#include <godot_cpp/classes/script_editor.hpp>
#include <godot_cpp/classes/script_editor_base.hpp>
#include <godot_cpp/classes/text_edit.hpp>
#include <godot_cpp/core/class_db.hpp>
#include <godot_cpp/core/object.hpp>

namespace fennara {

namespace {

godot::CodeEdit *find_code_edit(godot::Node *node) {
    if (node == nullptr) {
        return nullptr;
    }
    if (auto *code_edit = godot::Object::cast_to<godot::CodeEdit>(node)) {
        return code_edit;
    }
    const int32_t child_count = node->get_child_count();
    for (int32_t i = 0; i < child_count; i++) {
        if (auto *found = find_code_edit(node->get_child(i))) {
            return found;
        }
    }
    return nullptr;
}

godot::TextEdit *find_text_edit(godot::Node *node) {
    if (node == nullptr) {
        return nullptr;
    }
    if (auto *text_edit = godot::Object::cast_to<godot::TextEdit>(node)) {
        return text_edit;
    }
    const int32_t child_count = node->get_child_count();
    for (int32_t i = 0; i < child_count; i++) {
        if (auto *found = find_text_edit(node->get_child(i))) {
            return found;
        }
    }
    return nullptr;
}

} // namespace

void FennaraScriptContextMenuPlugin::_bind_methods() {
    godot::ClassDB::bind_method(
        godot::D_METHOD("_on_add_to_chat", "callback_data"),
        &FennaraScriptContextMenuPlugin::_on_add_to_chat);
}

void FennaraScriptContextMenuPlugin::set_local_bridge(FennaraLocalBridge *bridge) {
    local_bridge = bridge;
}

void FennaraScriptContextMenuPlugin::_popup_menu(const godot::PackedStringArray &paths) {
    (void)paths;

    godot::String path;
    godot::String text;
    int32_t start_line = 0;
    int32_t end_line = 0;
    if (_collect_selection(path, start_line, end_line, text)) {
        add_context_menu_item("Add to Chat",
                              callable_mp(this, &FennaraScriptContextMenuPlugin::_on_add_to_chat));
    }
}

bool FennaraScriptContextMenuPlugin::_collect_selection(godot::String &path,
                                                        int32_t &start_line,
                                                        int32_t &end_line,
                                                        godot::String &text) const {
    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    if (editor == nullptr) {
        return false;
    }

    godot::ScriptEditor *script_editor = editor->get_script_editor();
    if (script_editor == nullptr) {
        return false;
    }

    godot::Ref<godot::Script> script = script_editor->get_current_script();
    if (script.is_null()) {
        return false;
    }

    godot::String script_path = script->get_path().strip_edges();
    if (script_path.is_empty()) {
        return false;
    }

    godot::ScriptEditorBase *current_editor = script_editor->get_current_editor();
    if (current_editor == nullptr) {
        return false;
    }

    godot::Control *base_editor = current_editor->get_base_editor();
    godot::TextEdit *text_edit = find_code_edit(base_editor);
    if (text_edit == nullptr) {
        text_edit = find_text_edit(base_editor);
    }
    if (text_edit == nullptr || !text_edit->has_selection()) {
        return false;
    }

    godot::String selected_text = text_edit->get_selected_text();
    if (selected_text.strip_edges().is_empty()) {
        return false;
    }

    int32_t from_line = text_edit->get_selection_from_line();
    int32_t to_line = text_edit->get_selection_to_line();
    if (to_line < from_line) {
        int32_t tmp = from_line;
        from_line = to_line;
        to_line = tmp;
    }
    if (from_line < 0 || to_line < 0) {
        return false;
    }

    path = script_path;
    start_line = from_line + 1;
    end_line = to_line + 1;
    text = selected_text;
    return true;
}

void FennaraScriptContextMenuPlugin::_on_add_to_chat(const godot::Variant &callback_data) {
    (void)callback_data;

    godot::String path;
    godot::String text;
    int32_t start_line = 0;
    int32_t end_line = 0;
    if (!_collect_selection(path, start_line, end_line, text)) {
        FLOG_UI("Add to Chat skipped: no selected script text");
        return;
    }

    if (local_bridge == nullptr ||
        !local_bridge->send_chat_context_snippet(path, start_line, end_line, text)) {
        FLOG_NET("Add to Chat skipped: local daemon is not connected");
    }
}

} // namespace fennara
