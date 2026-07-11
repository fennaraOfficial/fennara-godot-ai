#pragma once

#include <godot_cpp/classes/editor_plugin.hpp>
#include <godot_cpp/variant/packed_byte_array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include "fennara/ui/dock.hpp"

namespace godot {
class HTTPRequest;
}

namespace fennara {

class FennaraLocalBridge;
class FennaraScriptContextMenuPlugin;

class FennaraPlugin : public godot::EditorPlugin {
    GDCLASS(FennaraPlugin, godot::EditorPlugin)

protected:
    static void _bind_methods();

private:
    FennaraDock *dock_instance = nullptr;
    FennaraLocalBridge *local_bridge = nullptr;
    godot::HTTPRequest *update_request = nullptr;
    godot::Ref<FennaraScriptContextMenuPlugin> script_context_menu_plugin;
    void _configure_editor_settings();
    void _ensure_export_presets_exclude_fennara();
    bool _is_export_preset_section(const godot::String &section) const;
    godot::PackedStringArray _split_export_filter(const godot::String &raw) const;
    void _ensure_runtime_helper_autoload();
    void _inspect_csharp_support();
    void _warm_csharp_lsp();
    void _start_update_check();
    void _on_update_check_completed(int64_t result,
                                    int64_t response_code,
                                    const godot::PackedStringArray &headers,
                                    const godot::PackedByteArray &body);

public:
    FennaraPlugin();
    ~FennaraPlugin() = default;

    void _enter_tree() override;
    void _exit_tree() override;
    void _process(double delta) override;
};

} // namespace fennara
