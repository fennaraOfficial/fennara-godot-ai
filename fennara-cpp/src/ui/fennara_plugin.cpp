#include "fennara/ui/fennara_plugin.hpp"
#include "fennara/lsp/csharp_lsp.hpp"
#include "fennara/lsp/csharp_support.hpp"
#include "fennara/local_bridge.hpp"
#include "fennara/logger.hpp"
#include "fennara/update_notice.hpp"
#include "fennara/ui/script_context_menu.hpp"

#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/config_file.hpp>
#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/editor_settings.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/http_request.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

void FennaraPlugin::_bind_methods() {
    godot::ClassDB::bind_method(
        godot::D_METHOD("_warm_csharp_lsp"),
        &FennaraPlugin::_warm_csharp_lsp);
    godot::ClassDB::bind_method(
        godot::D_METHOD("_on_update_check_completed", "result", "response_code", "headers", "body"),
        &FennaraPlugin::_on_update_check_completed);
}

FennaraPlugin::FennaraPlugin() {
}

void FennaraPlugin::_enter_tree() {
    Logger::init();
    FLOG_SYS(godot::String("Plugin started, Godot ") + godot::String(godot::Engine::get_singleton()->get_version_info()["string"]));

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
    _start_update_check();
    _ensure_runtime_helper_autoload();

    set_process(true);

    _configure_editor_settings();
    _ensure_export_presets_exclude_fennara();
    _inspect_csharp_support();
    call_deferred("_warm_csharp_lsp");
    local_bridge->request_get_class_info_warmup();
}

void FennaraPlugin::_start_update_check() {
    if (!update_notice::begin_check()) {
        return;
    }
    update_request = memnew(godot::HTTPRequest);
    if (update_request == nullptr) {
        update_notice::complete_check(false, 0, godot::PackedByteArray(),
                                      "Could not create the update request.");
        return;
    }
    update_request->set_name("FennaraUpdateCheck");
    update_request->set_timeout(5.0);
    update_request->set_use_threads(true);
    add_child(update_request);
    update_request->connect(
        "request_completed",
        callable_mp(this, &FennaraPlugin::_on_update_check_completed),
        godot::Object::CONNECT_ONE_SHOT);
    godot::PackedStringArray headers;
    headers.append("Accept: application/vnd.github+json");
    headers.append("User-Agent: fennara-godot-ai");
    godot::Error error = update_request->request(
        "https://api.github.com/repos/fennaraOfficial/fennara-godot-ai/releases/latest",
        headers);
    if (error != godot::OK) {
        update_notice::complete_check(false, 0, godot::PackedByteArray(),
                                      "Failed to start the GitHub update request.");
        update_request->queue_free();
        update_request = nullptr;
    }
}

void FennaraPlugin::_on_update_check_completed(
    int64_t result,
    int64_t response_code,
    const godot::PackedStringArray &headers,
    const godot::PackedByteArray &body) {
    (void)headers;
    const bool success =
        result == godot::HTTPRequest::RESULT_SUCCESS && response_code == 200;
    update_notice::complete_check(
        success,
        static_cast<int>(response_code),
        body,
        success ? godot::String() : godot::String("GitHub update request failed."));
    if (update_request != nullptr) {
        update_request->queue_free();
        update_request = nullptr;
    }
}

void FennaraPlugin::_inspect_csharp_support() {
    godot::Dictionary status = csharp_support::inspect_project();
    FLOG_SYS(godot::String("C# support: ") +
             godot::String(status.get("state", "")) + " - " +
             godot::String(status.get("message", "")));
}

void FennaraPlugin::_warm_csharp_lsp() {
    godot::OS *os = godot::OS::get_singleton();
    godot::DisplayServer *display = godot::DisplayServer::get_singleton();
    bool is_headless =
        (os != nullptr && os->has_feature("headless")) ||
        (display != nullptr && display->get_name().to_lower() == "headless");
    if (is_headless) {
        FLOG_SYS("C# LSP background warmup skipped: headless editor");
        return;
    }

    godot::Dictionary status = csharp_support::inspect_project();
    if (godot::String(status.get("state", "")) != "ready") {
        FLOG_SYS(godot::String("C# LSP background warmup skipped: ") +
                 godot::String(status.get("message", "")));
        return;
    }

    godot::Dictionary selected_project =
        status.get("selected_project", godot::Dictionary());
    csharp_lsp::warmup_async(
        status.get("lsp_path", ""),
        selected_project.get("absolute_path", ""),
        status.get("project_root", ""),
        "fennara-csharp-warmup");
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
    if (update_request != nullptr) {
        update_request->cancel_request();
        update_request->queue_free();
        update_request = nullptr;
    }
    csharp_lsp::shutdown_warm_server();
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
}

} // namespace fennara
