#pragma once

#include <godot_cpp/classes/editor_plugin.hpp>
#include "fennara/ui/dock.hpp"
#include <atomic>
#include <thread>

namespace fennara {

class FennaraLocalBridge;
class FennaraExportPlugin;
class FennaraScriptContextMenuPlugin;

class FennaraPlugin : public godot::EditorPlugin {
    GDCLASS(FennaraPlugin, godot::EditorPlugin)

protected:
    static void _bind_methods();

private:
    FennaraDock *dock_instance = nullptr;
    FennaraLocalBridge *local_bridge = nullptr;
    godot::Ref<FennaraExportPlugin> export_plugin;
    godot::Ref<FennaraScriptContextMenuPlugin> script_context_menu_plugin;
    bool csharp_preparation_pending = false;
    bool initial_filesystem_scan_completed = false;
    std::atomic_bool update_check_cancelled{false};
    std::thread update_check_thread;
    void _stop_update_check();
    void _configure_editor_settings();
    void _ensure_runtime_helper_autoload();
    void _start_csharp_preparation();
    void _on_editor_filesystem_changed();

public:
    FennaraPlugin();
    ~FennaraPlugin();

    void _enter_tree() override;
    void _exit_tree() override;
    void _process(double delta) override;
};

} // namespace fennara
