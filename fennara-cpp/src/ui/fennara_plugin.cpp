#include "fennara/ui/fennara_plugin.hpp"
#include "fennara/app_paths.hpp"
#include "fennara/csharp/build.hpp"
#include "fennara/local_bridge.hpp"
#include "fennara/logger.hpp"
#include "fennara/update_notice.hpp"
#include "fennara/ui/script_context_menu.hpp"

#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/config_file.hpp>
#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/editor_file_system.hpp>
#include <godot_cpp/classes/editor_settings.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

void FennaraPlugin::_bind_methods() {
    godot::ClassDB::bind_method(
        godot::D_METHOD("_start_csharp_preparation"),
        &FennaraPlugin::_start_csharp_preparation);
    godot::ClassDB::bind_method(
        godot::D_METHOD("_on_editor_filesystem_changed"),
        &FennaraPlugin::_on_editor_filesystem_changed);
}

FennaraPlugin::FennaraPlugin() {
}

FennaraPlugin::~FennaraPlugin() {
    _stop_update_check();
}

void FennaraPlugin::_stop_update_check() {
    update_check_cancelled.store(true, std::memory_order_release);
    if (update_check_thread.joinable()) {
        update_check_thread.join();
    }
}

void FennaraPlugin::_enter_tree() {
    Logger::init();
    csharp_build::begin_build_lifecycle();
    FLOG_SYS(godot::String("Plugin started, Godot ") + godot::String(godot::Engine::get_singleton()->get_version_info()["string"]));

    csharp_preparation_pending = true;
    initial_filesystem_scan_completed = false;
    csharp_build::reserve_background_preparation();
    godot::EditorFileSystem *filesystem =
        godot::EditorInterface::get_singleton()->get_resource_filesystem();
    if (filesystem != nullptr) {
        filesystem->connect(
            "filesystem_changed",
            callable_mp(this, &FennaraPlugin::_on_editor_filesystem_changed));
        initial_filesystem_scan_completed =
            !filesystem->is_scanning() &&
            filesystem->get_scanning_progress() >= 0.999f;
    } else {
        csharp_preparation_pending = false;
        csharp_build::cancel_reserved_background_preparation();
        FLOG_SYS(
            "C# background preparation skipped: EditorFileSystem unavailable");
    }

    dock_instance = memnew(FennaraDock);
    dock_instance->set_name("Fennara");
    add_control_to_dock(godot::EditorPlugin::DOCK_SLOT_RIGHT_UL, dock_instance);

    local_bridge = memnew(FennaraLocalBridge);
    local_bridge->set_name("FennaraLocalBridge");
    add_child(local_bridge);
    dock_instance->set_local_bridge(local_bridge);

    script_context_menu_plugin.instantiate();
    script_context_menu_plugin->set_local_bridge(local_bridge);
    add_context_menu_plugin(godot::EditorContextMenuPlugin::CONTEXT_SLOT_SCRIPT_EDITOR_CODE,
                            script_context_menu_plugin);
    if (godot::FileAccess::file_exists(app_paths::daemon_binary_path())) {
        update_check_cancelled.store(false, std::memory_order_release);
        update_check_thread =
            std::thread([this]() { update_notice::check_once(&update_check_cancelled); });
    } else {
        FLOG_SYS("Update check deferred until first-run setup is complete");
    }

    _ensure_runtime_helper_autoload();

    set_process(true);

    _configure_editor_settings();
    _ensure_export_presets_exclude_fennara();
    local_bridge->request_get_class_info_warmup();
}

void FennaraPlugin::_start_csharp_preparation() {
    godot::OS *os = godot::OS::get_singleton();
    godot::DisplayServer *display = godot::DisplayServer::get_singleton();
    bool is_headless =
        (os != nullptr && os->has_feature("headless")) ||
        (display != nullptr && display->get_name().to_lower() == "headless");
    if (is_headless) {
        csharp_build::cancel_reserved_background_preparation();
        FLOG_SYS("C# background preparation skipped: headless editor");
        return;
    }

    csharp_build::start_background_preparation_async();
}

void FennaraPlugin::_on_editor_filesystem_changed() {
    godot::EditorFileSystem *filesystem =
        godot::EditorInterface::get_singleton()->get_resource_filesystem();
    if (filesystem != nullptr && !filesystem->is_scanning()) {
        initial_filesystem_scan_completed = true;
    }
    csharp_build::note_csharp_source_changed();
}

void FennaraPlugin::_ensure_runtime_helper_autoload() {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        FLOG_ERR("Runtime helper autoload skipped: ProjectSettings unavailable");
        return;
    }

    const godot::String name = "_fennara_game_capture";
    const godot::String path = "res://addons/fennara/runtime/game_capture_helper.gd";
    const godot::String key = "autoload/" + name;
    const godot::String value = "*" + path;
    if ((godot::String)settings->get_setting(key, "") == value) {
        FLOG_SYS("Runtime helper autoload already registered");
        return;
    }

    settings->set_setting(key, value);
    settings->set_initial_value(key, "");
    settings->set_as_basic(key, true);
    godot::Error save_err = settings->save();
    if (save_err == godot::OK) {
        FLOG_SYS("Runtime helper autoload registered: " + path);
    } else {
        FLOG_ERR("Runtime helper autoload save failed code=" +
                 godot::String::num_int64(static_cast<int64_t>(save_err)));
    }
}

void FennaraPlugin::_configure_editor_settings() {
    godot::Ref<godot::EditorSettings> settings =
        godot::EditorInterface::get_singleton()->get_editor_settings();
    if (settings.is_null()) {
        Logger::record_incident(
            "plugin_startup",
            "editor_settings_unavailable",
            "Editor settings were unavailable during plugin startup.",
            godot::Dictionary(),
            "warning"
        );
        return;
    }

    const godot::String auto_reload_key =
        "text_editor/behavior/files/auto_reload_scripts_on_external_change";

    if (!(bool)settings->get_setting(auto_reload_key)) {
        settings->set_setting(auto_reload_key, true);
        FLOG_SYS("Auto-reload scripts on external change: enabled");
    }
}

bool FennaraPlugin::_is_export_preset_section(const godot::String &section) const {
    if (!section.begins_with("preset.") || section.contains(".options")) {
        return false;
    }
    godot::String suffix = section.substr(7);
    return !suffix.is_empty() && suffix.is_valid_int();
}

godot::PackedStringArray FennaraPlugin::_split_export_filter(
    const godot::String &raw) const {
    godot::PackedStringArray filters;
    godot::PackedStringArray parts = raw.split(",", false);
    for (int i = 0; i < parts.size(); i++) {
        godot::String trimmed = parts[i].strip_edges();
        if (!trimmed.is_empty() && !filters.has(trimmed)) {
            filters.append(trimmed);
        }
    }
    return filters;
}

void FennaraPlugin::_ensure_export_presets_exclude_fennara() {
    const godot::String presets_path = "res://export_presets.cfg";
    const godot::String backup_path = "res://export_presets.cfg.fennara.bak";
    if (!godot::FileAccess::file_exists(presets_path)) {
        FLOG_SYS("Export preset guard skipped: export_presets.cfg not found");
        return;
    }

    godot::Ref<godot::ConfigFile> config;
    config.instantiate();
    godot::Error load_err = config->load(presets_path);
    if (load_err != godot::OK) {
        FLOG_ERR("Export preset guard load failed code=" +
                 godot::String::num_int64(static_cast<int64_t>(load_err)));
        return;
    }

    static const char *obsolete_filters[] = {
        "addons/fennara/*",
    };
    static const char *required_filters[] = {
        "addons/fennara/ai/*",
        "addons/fennara/bin/*",
        "addons/fennara/dist/*",
        "addons/fennara/*.gdextension",
        "addons/fennara/*.gdextension.uid",
        "addons/fennara/*.md",
        "addons/fennara/VERSION",
        ".fennara/*",
    };

    bool changed = false;
    int64_t touched_presets = 0;
    godot::PackedStringArray sections = config->get_sections();
    for (int i = 0; i < sections.size(); i++) {
        godot::String section = sections[i];
        if (!_is_export_preset_section(section)) {
            continue;
        }

        godot::PackedStringArray filters =
            _split_export_filter(config->get_value(section, "exclude_filter", ""));
        bool section_changed = false;
        for (const char *filter : obsolete_filters) {
            godot::String pattern(filter);
            int existing_index = filters.find(pattern);
            if (existing_index >= 0) {
                filters.remove_at(existing_index);
                section_changed = true;
            }
        }
        for (const char *filter : required_filters) {
            godot::String pattern(filter);
            if (!filters.has(pattern)) {
                filters.append(pattern);
                section_changed = true;
            }
        }

        if (section_changed) {
            config->set_value(section, "exclude_filter", godot::String(",").join(filters));
            changed = true;
            touched_presets++;
        }
    }

    if (!changed) {
        FLOG_SYS("Export preset guard already configured");
        return;
    }

    if (!godot::FileAccess::file_exists(backup_path)) {
        godot::Error copy_err =
            godot::DirAccess::copy_absolute(presets_path, backup_path);
        if (copy_err == godot::OK) {
            FLOG_SYS("Export preset guard backup created: " + backup_path);
        } else {
            FLOG_ERR("Export preset guard backup failed code=" +
                     godot::String::num_int64(static_cast<int64_t>(copy_err)));
        }
    }

    godot::Error save_err = config->save(presets_path);
    if (save_err == godot::OK) {
        FLOG_SYS("Export preset guard added Fennara excludes to " +
                 godot::String::num_int64(touched_presets) + " preset(s)");
    } else {
        FLOG_ERR("Export preset guard save failed code=" +
                 godot::String::num_int64(static_cast<int64_t>(save_err)));
    }
}

void FennaraPlugin::_exit_tree() {
    set_process(false);
    _stop_update_check();
    csharp_preparation_pending = false;
    initial_filesystem_scan_completed = false;
    csharp_build::request_build_shutdown();
    godot::EditorFileSystem *filesystem =
        godot::EditorInterface::get_singleton()->get_resource_filesystem();
    if (filesystem != nullptr) {
        godot::Callable callback =
            callable_mp(this, &FennaraPlugin::_on_editor_filesystem_changed);
        if (filesystem->is_connected("filesystem_changed", callback)) {
            filesystem->disconnect("filesystem_changed", callback);
        }
    }
    csharp_build::shutdown_background_preparation();
    if (script_context_menu_plugin.is_valid()) {
        remove_context_menu_plugin(script_context_menu_plugin);
        script_context_menu_plugin->set_local_bridge(nullptr);
        script_context_menu_plugin.unref();
    }
    if (local_bridge) {
        local_bridge->queue_free();
        local_bridge = nullptr;
    }

    if (dock_instance) {
        remove_control_from_docks(dock_instance);
        dock_instance->queue_free();
        dock_instance = nullptr;
    }
    FLOG_SYS("Plugin stopped");
}

void FennaraPlugin::_process(double delta) {
    (void)delta;
    if (!csharp_preparation_pending) {
        return;
    }
    if (!initial_filesystem_scan_completed) {
        return;
    }
    godot::EditorFileSystem *filesystem =
        godot::EditorInterface::get_singleton()->get_resource_filesystem();
    if (filesystem != nullptr && filesystem->is_scanning()) {
        return;
    }
    csharp_preparation_pending = false;
    _start_csharp_preparation();
}

} // namespace fennara
