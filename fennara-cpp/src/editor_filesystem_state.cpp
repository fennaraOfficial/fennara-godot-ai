#include "fennara/editor_filesystem_state.hpp"

#include <godot_cpp/classes/editor_file_system.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/time.hpp>

namespace fennara {

EditorFilesystemState &EditorFilesystemState::get_singleton() {
    static EditorFilesystemState singleton;
    return singleton;
}

godot::Dictionary EditorFilesystemState::snapshot() const {
    godot::Dictionary status;
    status["schema_version"] = "editor-filesystem-status-v1";

    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::EditorFileSystem *filesystem =
        editor != nullptr ? editor->get_resource_filesystem() : nullptr;
    if (filesystem == nullptr) {
        status["available"] = false;
        status["state"] = "unavailable";
        status["initial_scan_complete"] = false;
        status["is_scanning"] = false;
        status["scan_progress"] = 0.0;
        status["asset_tools_ready"] = false;
        status["not_ready_reason"] =
            "Godot's EditorFileSystem is unavailable.";
        return status;
    }

    const bool is_scanning = filesystem->is_scanning();
    const double scan_progress = filesystem->get_scanning_progress();
    const bool initial_scan_complete = !is_scanning;
    const bool is_importing = _signal_import_active || _owned_import_active;
    const bool ready = initial_scan_complete && !is_importing;

    godot::String state = "ready";
    if (is_scanning && is_importing) {
        state = "scanning_and_importing";
    } else if (is_scanning) {
        state = "scanning";
    } else if (is_importing) {
        state = "importing";
    }

    status["available"] = true;
    status["state"] = state;
    status["initial_scan_complete"] = initial_scan_complete;
    status["is_scanning"] = is_scanning;
    status["scan_progress"] = scan_progress;
    status["active_import_count"] = _active_import_count;
    status["last_imported_count"] = _last_imported_count;
    status["asset_tools_ready"] = ready;
    status["owned_import_active"] = _owned_import_active;
    status["last_owned_import_success"] = _last_owned_import_success;
    status["last_owned_import_duration_ms"] =
        static_cast<int64_t>(_last_owned_import_duration_ms);
    if (_owned_import_active) {
        const uint64_t now = godot::Time::get_singleton()->get_ticks_msec();
        status["active_import_asset_path"] = _owned_import_asset_path;
        status["active_import_elapsed_ms"] =
            static_cast<int64_t>(now - _owned_import_started_ms);
    }

    if (is_scanning && is_importing) {
        status["not_ready_reason"] =
            "Godot is still scanning and importing project resources.";
    } else if (is_scanning) {
        status["not_ready_reason"] =
            "Godot is still scanning project resources.";
    } else if (is_importing) {
        status["not_ready_reason"] =
            "Godot is still importing project resources.";
    }
    return status;
}

void EditorFilesystemState::on_resources_reimporting(
    const godot::PackedStringArray &paths) {
    _signal_import_active = true;
    _active_import_count = paths.size();
}

void EditorFilesystemState::on_resources_reimported(
    const godot::PackedStringArray &paths) {
    _signal_import_active = false;
    _last_imported_count = paths.size();
    _active_import_count = _owned_import_active ? 1 : 0;
}

bool EditorFilesystemState::begin_owned_import(
    const godot::String &asset_path,
    godot::String &error) {
    godot::Dictionary current = snapshot();
    if (!(bool)current.get("available", false)) {
        error = current.get("not_ready_reason",
                            "Godot's EditorFileSystem is unavailable.");
        return false;
    }
    if ((bool)current.get("is_scanning", false)) {
        error = "Godot is still scanning project resources.";
        return false;
    }
    if (_owned_import_active || _signal_import_active) {
        error = "Another Godot asset import is already active.";
        return false;
    }

    _owned_import_active = true;
    _owned_import_asset_path = asset_path;
    _owned_import_started_ms = godot::Time::get_singleton()->get_ticks_msec();
    _active_import_count = 1;
    return true;
}

void EditorFilesystemState::finish_owned_import(bool success) {
    if (_owned_import_active) {
        const uint64_t now = godot::Time::get_singleton()->get_ticks_msec();
        _last_owned_import_duration_ms = now - _owned_import_started_ms;
    }
    _last_owned_import_success = success;
    _owned_import_active = false;
    _owned_import_started_ms = 0;
    _owned_import_asset_path = "";
    if (!_signal_import_active) {
        _active_import_count = 0;
    }
}

} // namespace fennara
