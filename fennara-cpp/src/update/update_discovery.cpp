#include "fennara/update/update_coordinator.hpp"

#include "fennara/app_paths.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>

namespace fennara {
namespace {

bool terminal_failure(const godot::String &phase) {
    return phase == "failed" || phase == "rolled_back" || phase == "recovery_required";
}

godot::String addon_version() {
    return godot::FileAccess::get_file_as_string("res://addons/fennara/VERSION").strip_edges();
}

} // namespace

godot::String UpdateCoordinator::_project_path() const {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    return settings == nullptr ? godot::String() : settings->globalize_path("res://");
}

godot::String UpdateCoordinator::_update_root() const {
    return _project_path().path_join(".godot").path_join("fennara-update");
}

godot::String UpdateCoordinator::_receipt_path() const {
    return staging_root.path_join("receipt.json");
}

godot::Dictionary UpdateCoordinator::_read_operation() const {
    if (operation_id.is_empty()) {
        return godot::Dictionary();
    }
    const godot::Variant parsed = godot::JSON::parse_string(
        godot::FileAccess::get_file_as_string(
            app_paths::operations_dir().path_join(operation_id + godot::String(".json"))));
    return parsed.get_type() == godot::Variant::DICTIONARY ? godot::Dictionary(parsed)
                                                            : godot::Dictionary();
}

godot::Dictionary UpdateCoordinator::_read_receipt(const godot::String &root) const {
    const godot::Variant parsed = godot::JSON::parse_string(
        godot::FileAccess::get_file_as_string(root.path_join("receipt.json")));
    return parsed.get_type() == godot::Variant::DICTIONARY ? godot::Dictionary(parsed)
                                                            : godot::Dictionary();
}

void UpdateCoordinator::_poll_operation() {
    const godot::Dictionary state = _read_operation();
    if (state.is_empty()) {
        return;
    }
    const godot::String phase = state.get("phase", "");
    if (phase == "ready_to_close") {
        const godot::Dictionary receipt = _read_receipt(staging_root);
        target_version = receipt.get("to_version", target_version);
        _set_step(Step::ReadyToClose, "Fennara " + target_version + " is ready.",
                  "Godot must close briefly to install the verified update");
        return;
    }
    if (phase == "recovery_required") {
        _set_step(Step::RecoveryRequired, "Fennara could not validate the update.",
                  "Restore the previous version, or open the logs for details");
        return;
    }
    if (phase == "succeeded") {
        dismissed = true;
        _set_step(Step::Idle, "Fennara is already up to date.", "No update was required");
        return;
    }
    if (terminal_failure(phase)) {
        godot::Dictionary last_error = state.get("last_error", godot::Dictionary());
        _fail(last_error.get("code", "FEN-UPDATE-FAILED"),
              last_error.get("message", "The Fennara update could not finish."));
        return;
    }
    _set_step(step, "Preparing the Fennara update...", "Operation " + operation_id);
}

void UpdateCoordinator::_scan_pending_updates() {
    const godot::String root = _update_root();
    godot::Ref<godot::DirAccess> dir = godot::DirAccess::open(root);
    if (dir.is_null()) {
        return;
    }
    dir->list_dir_begin();
    for (godot::String name = dir->get_next(); !name.is_empty(); name = dir->get_next()) {
        if (!dir->current_is_dir() || name.ends_with(".preparing")) {
            continue;
        }
        const godot::String candidate = root.path_join(name);
        const godot::Dictionary receipt = _read_receipt(candidate);
        const godot::String state = receipt.get("state", "");
        const int64_t updater_pid = receipt.get("updater_pid", -1);
        const bool updater_running =
            updater_pid > 0 && godot::OS::get_singleton()->is_process_running(updater_pid);
        if (state == "validating" && updater_running) {
            _write_activation_handshake(candidate, receipt);
        }
        if (state == "recovery_required" ||
            ((state == "applying" || state == "reopening" || state == "validating") &&
             !updater_running)) {
            operation_id = receipt.get("operation_id", name);
            target_version = receipt.get("to_version", "");
            staging_root = candidate;
            _set_step(Step::RecoveryRequired, "Fennara could not validate the update.",
                      "The previous working version is still available to restore");
            break;
        }
        if (state == "ready_to_close") {
            operation_id = receipt.get("operation_id", name);
            target_version = receipt.get("to_version", "");
            staging_root = candidate;
            _set_step(Step::ReadyToClose, "Fennara " + target_version + " is ready.",
                      "Godot must close briefly to install the verified update");
            break;
        }
    }
    dir->list_dir_end();
}

void UpdateCoordinator::_write_activation_handshake(
    const godot::String &root, const godot::Dictionary &receipt) const {
    const godot::String expected = receipt.get("to_version", "");
    if (expected.is_empty() || addon_version() != expected) {
        return;
    }
    godot::Dictionary handshake;
    handshake["operation_id"] = receipt.get("operation_id", "");
    handshake["addon_version"] = expected;
    handshake["godot_pid"] = godot::OS::get_singleton()->get_process_id();
    app_paths::write_json(root.path_join("activation-handshake.json"), handshake);
}

} // namespace fennara
