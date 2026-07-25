#include "fennara/ui/export_plugin.hpp"

#include "fennara/logger.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

namespace {

constexpr const char *RUNTIME_AUTOLOAD_KEY =
    "autoload/_fennara_game_capture";
constexpr const char *ADDON_EXPORT_PREFIX = "res://addons/fennara/";
constexpr const char *PROJECT_STATE_EXPORT_PREFIX = "res://.fennara/";
constexpr const char *EXTENSION_LIST_BACKUP_SUFFIX =
    ".fennara-export-backup";
constexpr const char *EXTENSION_LIST_DISCARD_SUFFIX =
    ".fennara-export-discard";
constexpr const char *EXTENSION_LIST_STAGING_SUFFIX =
    ".fennara-export-tmp";

} // namespace

void FennaraExportPlugin::_bind_methods() {
#ifdef FENNARA_SETUP_TEST_HOOKS
    godot::ClassDB::bind_method(
        godot::D_METHOD("test_should_skip_path", "path"),
        &FennaraExportPlugin::test_should_skip_path);
    godot::ClassDB::bind_method(
        godot::D_METHOD("test_filter_extension_list", "contents"),
        &FennaraExportPlugin::test_filter_extension_list);
    godot::ClassDB::bind_method(
        godot::D_METHOD("test_suppress_extension_list_entry"),
        &FennaraExportPlugin::test_suppress_extension_list_entry);
    godot::ClassDB::bind_method(
        godot::D_METHOD("test_export_begin"),
        &FennaraExportPlugin::test_export_begin);
    godot::ClassDB::bind_method(
        godot::D_METHOD("test_export_end"),
        &FennaraExportPlugin::test_export_end);
#endif
}

FennaraExportPlugin::FennaraExportPlugin() {
    _resolve_extension_list_path();
    const godot::String backup_path =
        extension_list_path + godot::String(EXTENSION_LIST_BACKUP_SUFFIX);
    if (!extension_list_path.is_empty() &&
        godot::FileAccess::file_exists(backup_path)) {
        extension_list_changed = true;
        if (_restore_extension_list_backup()) {
            FLOG_SYS(
                "Export guard recovered an interrupted extension registry update");
        } else {
            FLOG_ERR(
                "Export guard could not recover an interrupted extension registry update");
        }
    }
}

FennaraExportPlugin::~FennaraExportPlugin() {
    _restore_export_state();
}

godot::String FennaraExportPlugin::_get_name() const {
    // Godot sorts export plugins by name. This must run before the built-in
    // GDExtension plugin so skip() also prevents native libraries from being added.
    return "FennaraExportGuard";
}

bool FennaraExportPlugin::_supports_platform(
    const godot::Ref<godot::EditorExportPlatform> &platform) const {
    (void)platform;
    return true;
}

void FennaraExportPlugin::_export_begin(
    const godot::PackedStringArray &features,
    bool is_debug,
    const godot::String &path,
    uint32_t flags) {
    (void)features;
    (void)is_debug;
    (void)path;
    (void)flags;

    if (export_active || extension_list_changed) {
        FLOG_ERR(
            "Export guard found stale state; restoring it before the next export");
        _restore_export_state();
        if (extension_list_changed) {
            const godot::Ref<godot::EditorExportPlatform> platform =
                get_export_platform();
            if (platform.is_valid()) {
                platform->add_message(
                    godot::EditorExportPlatform::EXPORT_MESSAGE_ERROR,
                    "Fennara",
                    "Could not restore Godot's extension registry from the previous export.");
            }
            return;
        }
    }

    godot::ProjectSettings *settings =
        godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        FLOG_ERR("Export guard could not access ProjectSettings");
        return;
    }

    export_active = true;
    had_runtime_autoload = settings->has_setting(RUNTIME_AUTOLOAD_KEY);
    if (had_runtime_autoload) {
        saved_runtime_autoload =
            settings->get_setting(RUNTIME_AUTOLOAD_KEY);
        settings->set_setting(RUNTIME_AUTOLOAD_KEY, godot::Variant());
        FLOG_SYS("Export guard removed the Fennara runtime autoload");
    } else {
        saved_runtime_autoload = godot::Variant();
    }

}

void FennaraExportPlugin::_export_file(
    const godot::String &path,
    const godot::String &type,
    const godot::PackedStringArray &features) {
    (void)type;
    (void)features;
    if (_should_skip_path(path)) {
        if (path.get_extension().to_lower() == "gdextension") {
            _suppress_extension_list_entry();
        }
        skip();
    }
}

void FennaraExportPlugin::_export_end() {
    _restore_export_state();
}

bool FennaraExportPlugin::_should_skip_path(const godot::String &path) {
    const godot::String normalized = path.replace("\\", "/").to_lower();
    return normalized.begins_with(ADDON_EXPORT_PREFIX) ||
           normalized.begins_with(PROJECT_STATE_EXPORT_PREFIX);
}

godot::String FennaraExportPlugin::_filter_extension_list(
    const godot::String &contents) {
    godot::PackedStringArray lines = contents.split("\n", true);
    godot::PackedStringArray kept_lines;
    for (int64_t index = 0; index < lines.size(); index++) {
        const godot::String normalized =
            lines[index].strip_edges().replace("\\", "/").to_lower();
        if (!normalized.begins_with(ADDON_EXPORT_PREFIX)) {
            kept_lines.append(lines[index]);
        }
    }
    return godot::String("\n").join(kept_lines);
}

bool FennaraExportPlugin::_replace_extension_list(
    const godot::PackedByteArray &contents) {
    const godot::String staging_path =
        extension_list_path + godot::String(EXTENSION_LIST_STAGING_SUFFIX);
    const godot::String backup_path =
        extension_list_path + godot::String(EXTENSION_LIST_BACKUP_SUFFIX);
    if (godot::FileAccess::file_exists(backup_path)) {
        return false;
    }

    godot::Ref<godot::FileAccess> staging_file = godot::FileAccess::open(
        staging_path,
        godot::FileAccess::WRITE);
    if (staging_file.is_null() ||
        !staging_file->store_buffer(contents)) {
        godot::DirAccess::remove_absolute(staging_path);
        return false;
    }
    staging_file.unref();

    if (godot::DirAccess::rename_absolute(
            extension_list_path,
            backup_path) != godot::OK) {
        godot::DirAccess::remove_absolute(staging_path);
        return false;
    }

    extension_list_changed = true;
    if (godot::DirAccess::rename_absolute(
            staging_path,
            extension_list_path) != godot::OK) {
        if (godot::DirAccess::rename_absolute(
                backup_path,
                extension_list_path) == godot::OK) {
            extension_list_changed = false;
        }
        godot::DirAccess::remove_absolute(staging_path);
        return false;
    }
    return true;
}

void FennaraExportPlugin::_resolve_extension_list_path() {
    godot::ProjectSettings *settings =
        godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        extension_list_path = godot::String();
        return;
    }

    const bool use_hidden_project_data =
        settings->get_setting(
            "application/config/use_hidden_project_data_directory",
            true);
    extension_list_path =
        godot::String(use_hidden_project_data ? "res://.godot" : "res://godot")
            .path_join("extension_list.cfg");
}

bool FennaraExportPlugin::_restore_extension_list_backup() {
    const godot::String backup_path =
        extension_list_path + godot::String(EXTENSION_LIST_BACKUP_SUFFIX);
    if (!godot::FileAccess::file_exists(backup_path)) {
        return false;
    }

    const godot::String discard_path =
        extension_list_path + godot::String(EXTENSION_LIST_DISCARD_SUFFIX);
    godot::DirAccess::remove_absolute(discard_path);
    const bool had_live_registry =
        godot::FileAccess::file_exists(extension_list_path);
    if (had_live_registry &&
        godot::DirAccess::rename_absolute(
            extension_list_path,
            discard_path) != godot::OK) {
        return false;
    }

    if (godot::DirAccess::rename_absolute(
            backup_path,
            extension_list_path) != godot::OK) {
        if (had_live_registry) {
            godot::DirAccess::rename_absolute(
                discard_path,
                extension_list_path);
        }
        return false;
    }

    godot::DirAccess::remove_absolute(discard_path);
    extension_list_changed = false;
    return true;
}

void FennaraExportPlugin::_suppress_extension_list_entry() {
    if (extension_list_changed) {
        return;
    }

    godot::ProjectSettings *settings =
        godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        FLOG_ERR("Export guard could not locate Godot's extension registry");
        return;
    }

    _resolve_extension_list_path();
    if (!godot::FileAccess::file_exists(extension_list_path)) {
        extension_list_path = godot::String();
        return;
    }

    saved_extension_list =
        godot::FileAccess::get_file_as_bytes(extension_list_path);
    const godot::String original =
        saved_extension_list.get_string_from_utf8();
    const godot::String filtered = _filter_extension_list(original);
    if (filtered == original) {
        extension_list_path = godot::String();
        saved_extension_list = godot::PackedByteArray();
        return;
    }

    if (!_replace_extension_list(filtered.to_utf8_buffer())) {
        FLOG_ERR("Export guard could not filter Godot's extension registry");
        if (!extension_list_changed) {
            extension_list_path = godot::String();
            saved_extension_list = godot::PackedByteArray();
        }
        const godot::Ref<godot::EditorExportPlatform> platform =
            get_export_platform();
        if (platform.is_valid()) {
            platform->add_message(
                godot::EditorExportPlatform::EXPORT_MESSAGE_ERROR,
                "Fennara",
                "Could not remove Fennara from Godot's extension registry.");
        }
        return;
    }

    FLOG_SYS("Export guard removed Fennara from Godot's extension registry");
}

void FennaraExportPlugin::_restore_export_state() {
    godot::ProjectSettings *settings =
        godot::ProjectSettings::get_singleton();
    if (export_active && settings != nullptr && had_runtime_autoload) {
        settings->set_setting(RUNTIME_AUTOLOAD_KEY, saved_runtime_autoload);
        FLOG_SYS("Export guard restored the Fennara runtime autoload");
    }

    export_active = false;
    had_runtime_autoload = false;
    saved_runtime_autoload = godot::Variant();

    if (!extension_list_changed) {
        return;
    }

    bool registry_restored = _restore_extension_list_backup();
    if (!registry_restored && !saved_extension_list.is_empty() &&
        _replace_extension_list(saved_extension_list)) {
        const godot::String backup_path =
            extension_list_path +
            godot::String(EXTENSION_LIST_BACKUP_SUFFIX);
        godot::DirAccess::remove_absolute(backup_path);
        extension_list_changed = false;
        registry_restored = true;
    }
    if (!registry_restored) {
        FLOG_ERR("Export guard could not restore Godot's extension registry");
        return;
    }

    extension_list_path = godot::String();
    saved_extension_list = godot::PackedByteArray();
    FLOG_SYS("Export guard restored Godot's extension registry");
}

#ifdef FENNARA_SETUP_TEST_HOOKS
bool FennaraExportPlugin::test_should_skip_path(
    const godot::String &path) const {
    return _should_skip_path(path);
}

godot::String FennaraExportPlugin::test_filter_extension_list(
    const godot::String &contents) const {
    return _filter_extension_list(contents);
}

void FennaraExportPlugin::test_suppress_extension_list_entry() {
    _suppress_extension_list_entry();
}

void FennaraExportPlugin::test_export_begin() {
    _export_begin(godot::PackedStringArray(), false, godot::String(), 0);
}

void FennaraExportPlugin::test_export_end() {
    _export_end();
}
#endif

} // namespace fennara
