#include "fennara/app_paths.hpp"
#include "fennara/setup/first_run_setup.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/time.hpp>

namespace fennara {
namespace {

constexpr uint64_t kOwnerWriteGraceSeconds = 10;

godot::String lock_owner_path(const godot::String &lock_path) {
    return lock_path.path_join("owner.json");
}

} // namespace

bool installed_components_match_addon() {
    const godot::String expected_version =
        godot::FileAccess::get_file_as_string("res://addons/fennara/VERSION").strip_edges();
    if (expected_version.is_empty() ||
        !godot::FileAccess::file_exists(app_paths::cli_binary_path()) ||
        !godot::FileAccess::file_exists(app_paths::daemon_binary_path())) {
        return false;
    }
    const godot::PackedStringArray paths{app_paths::current_manifest_path()};
    const godot::Dictionary current = app_paths::read_json_first_existing(paths);
    return godot::String(current.get("version", "")) == expected_version;
}

bool FirstRunSetup::_installed_components_match() const {
    return installed_components_match_addon();
}

bool FirstRunSetup::_try_acquire_lock() {
    if (owns_bootstrap_lock) {
        return true;
    }
    const godot::String app_dir = app_paths::app_dir();
    if (app_dir.is_empty()) {
        _fail("FEN-SETUP-APP-DATA", "Fennara could not determine its app-data directory.");
        return false;
    }
    const godot::String parent = app_dir.path_join("cache").path_join("setup");
    bootstrap_lock_path = parent.path_join("bootstrap.lock");
    if (godot::DirAccess::make_dir_recursive_absolute(parent) != godot::OK) {
        _fail("FEN-SETUP-STAGE-FILESYSTEM", "Fennara could not create its setup lock directory.");
        return false;
    }

    if (godot::DirAccess::make_dir_absolute(bootstrap_lock_path) == godot::OK) {
        owns_bootstrap_lock = true;
        godot::OS *os = godot::OS::get_singleton();
        if (os == nullptr || !_write_lock_owner(os->get_process_id())) {
            _release_lock();
            _fail("FEN-SETUP-STAGE-FILESYSTEM",
                  "Fennara could not record ownership of its setup lock.");
            return false;
        }
        return true;
    }

    const godot::String owner_path = lock_owner_path(bootstrap_lock_path);
    int32_t owner_pid = -1;
    if (godot::FileAccess::file_exists(owner_path)) {
        const godot::Variant parsed =
            godot::JSON::parse_string(godot::FileAccess::get_file_as_string(owner_path));
        if (parsed.get_type() == godot::Variant::DICTIONARY) {
            owner_pid = static_cast<int32_t>((int64_t)godot::Dictionary(parsed).get("pid", -1));
        }
    }
    godot::OS *os = godot::OS::get_singleton();
    if (os != nullptr && owner_pid > 0 && os->is_process_running(owner_pid)) {
        return false;
    }

    if (owner_pid <= 0) {
        const uint64_t modified = godot::FileAccess::get_modified_time(bootstrap_lock_path);
        const uint64_t now =
            static_cast<uint64_t>(godot::Time::get_singleton()->get_unix_time_from_system());
        if (modified == 0 || now <= modified + kOwnerWriteGraceSeconds) {
            return false;
        }
    }

    godot::DirAccess::remove_absolute(owner_path);
    if (godot::DirAccess::remove_absolute(bootstrap_lock_path) != godot::OK) {
        return false;
    }
    return _try_acquire_lock();
}

bool FirstRunSetup::_write_lock_owner(int32_t pid) const {
    if (!owns_bootstrap_lock || bootstrap_lock_path.is_empty() || pid <= 0) {
        return false;
    }
    godot::Dictionary owner;
    owner["pid"] = pid;
    owner["updated_at_unix_ms"] =
        static_cast<int64_t>(godot::Time::get_singleton()->get_unix_time_from_system() * 1000.0);
    return app_paths::write_json(lock_owner_path(bootstrap_lock_path), owner);
}

void FirstRunSetup::_release_lock() {
    if (!owns_bootstrap_lock || bootstrap_lock_path.is_empty()) {
        return;
    }
    godot::DirAccess::remove_absolute(lock_owner_path(bootstrap_lock_path));
    godot::DirAccess::remove_absolute(bootstrap_lock_path);
    owns_bootstrap_lock = false;
}

void FirstRunSetup::_exit_tree() {
    godot::OS *os = godot::OS::get_singleton();
    if (installer_pid > 0 && os != nullptr && os->is_process_running(installer_pid)) {
        owns_bootstrap_lock = false;
        return;
    }
    _release_lock();
}

} // namespace fennara
