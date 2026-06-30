#pragma once

#include <godot_cpp/classes/editor_context_menu_plugin.hpp>
#include <godot_cpp/classes/ref.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

#include <cstdint>

namespace fennara {

class FennaraLocalBridge;

class FennaraScriptContextMenuPlugin : public godot::EditorContextMenuPlugin {
    GDCLASS(FennaraScriptContextMenuPlugin, godot::EditorContextMenuPlugin)

protected:
    static void _bind_methods();

private:
    FennaraLocalBridge *local_bridge = nullptr;

    bool _collect_selection(godot::String &path,
                            int32_t &start_line,
                            int32_t &end_line,
                            godot::String &text) const;
    void _on_add_to_chat(const godot::Variant &callback_data);

public:
    void set_local_bridge(FennaraLocalBridge *bridge);
    void _popup_menu(const godot::PackedStringArray &paths) override;
};

} // namespace fennara
