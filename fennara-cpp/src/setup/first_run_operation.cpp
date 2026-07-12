#include "fennara/app_paths.hpp"
#include "fennara/setup/first_run_setup.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/time.hpp>

namespace fennara {
namespace {

godot::String operation_phase_label(const godot::String &phase) {
    if (phase == "checking")
        return "Checking the existing installation...";
    if (phase == "downloading")
        return "Downloading matching Fennara components...";
    if (phase == "verifying")
        return "Verifying downloaded components...";
    if (phase == "staging")
        return "Installing matching components...";
    if (phase == "validating")
        return "Starting and checking Fennara...";
    if (phase == "applying")
        return "Applying the installation...";
    return "Finishing Fennara setup...";
}

bool process_is_running(int32_t pid) {
    godot::OS *os = godot::OS::get_singleton();
    return os != nullptr && pid > 0 && os->is_process_running(pid);
}

} // namespace

void FirstRunSetup::open_logs() const {
    godot::String target = app_paths::operation_logs_dir();
    if (!operation_id.is_empty()) {
        target = target.path_join(operation_id + godot::String(".jsonl"));
    }
    godot::OS *os = godot::OS::get_singleton();
    if (os != nullptr && !target.is_empty()) {
        os->shell_show_in_file_manager(target, true);
    }
}

void FirstRunSetup::copy_report() const {
    godot::DisplayServer *display = godot::DisplayServer::get_singleton();
    if (display != nullptr) {
        display->clipboard_set(_diagnostic_report());
    }
}

bool FirstRunSetup::_launch_installer() {
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr) {
        _fail("FEN-SETUP-CLI-LAUNCH", "Godot could not provide process-launch services.");
        return false;
    }
    if (_test_failure("launch")) {
        _fail("FEN-SETUP-CLI-LAUNCH", "Simulated CLI launch failure.");
        return false;
    }

    installer_started_at_ms =
        static_cast<uint64_t>(godot::Time::get_singleton()->get_unix_time_from_system() * 1000.0);
    operation_id = "install-" + godot::String::num_uint64(installer_started_at_ms) + "-godot-" +
                   godot::String::num_int64(os->get_process_id());

    godot::PackedStringArray args;
    args.append("install");
    args.append("--project");
    args.append(project_path);
    args.append("--version");
    args.append(addon_version);
    args.append("--operation-id");
    args.append(operation_id);
    const godot::String executable =
        installer_cli_path.is_empty() ? app_paths::cli_binary_path() : installer_cli_path;
    installer_pid = os->create_process(executable, args, false);
    if (installer_pid <= 0) {
        if (installer_cli_path.is_empty()) {
            const godot::String target = app_paths::cli_binary_path();
            const godot::String backup = target + godot::String(".previous");
            if (godot::FileAccess::file_exists(backup)) {
                godot::DirAccess::remove_absolute(target);
                godot::DirAccess::rename_absolute(backup, target);
            }
        }
        _fail("FEN-SETUP-CLI-LAUNCH", "The verified Fennara CLI could not be started.");
        return false;
    }
    if (!_write_lock_owner(installer_pid)) {
        os->kill(installer_pid);
        _fail("FEN-SETUP-STAGE-FILESYSTEM",
              "Fennara could not transfer its setup lock to the installer process.");
        return false;
    }

    step = Step::Installing;
    operation_poll_timer = 0.0;
    last_operation_updated_at_ms = 0;
    process_exit_observed_at_ms = 0;
    _set_status("Installing Fennara...", "Waiting for installation progress");
    return true;
}

void FirstRunSetup::_poll_operation() {
    const bool running = process_is_running(installer_pid);
    const uint64_t now_ms =
        static_cast<uint64_t>(godot::Time::get_singleton()->get_unix_time_from_system() * 1000.0);
    const godot::Dictionary state = _find_install_operation();
    if (!state.is_empty()) {
        operation_state = state;
        const uint64_t updated_at = (uint64_t)(int64_t)state.get("updated_at_unix_ms", 0);
        if (updated_at != last_operation_updated_at_ms) {
            last_operation_updated_at_ms = updated_at;
            process_exit_observed_at_ms = 0;
        }
        _apply_operation_state(state);
        if (step != Step::Installing) {
            return;
        }
        if (running) {
            process_exit_observed_at_ms = 0;
        } else if (process_exit_observed_at_ms == 0) {
            process_exit_observed_at_ms = now_ms;
        } else if (now_ms > process_exit_observed_at_ms + 2000) {
            _fail("FEN-SETUP-CLI-EXIT",
                  "The Fennara CLI exited before completing its installation operation.");
        }
        return;
    }

    if (!running && now_ms > installer_started_at_ms + 3000) {
        _fail("FEN-SETUP-CLI-EXIT",
              "The Fennara CLI exited before it could create an installation record.");
    }
}

godot::Dictionary FirstRunSetup::_find_install_operation() const {
    const godot::String dir = app_paths::operations_dir();
    if (dir.is_empty() || operation_id.is_empty()) {
        return godot::Dictionary();
    }
    const godot::Variant parsed = godot::JSON::parse_string(godot::FileAccess::get_file_as_string(
        dir.path_join(operation_id + godot::String(".json"))));
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        return godot::Dictionary();
    }
    const godot::Dictionary state = parsed;
    return godot::String(state.get("operation_id", "")) == operation_id ? state
                                                                        : godot::Dictionary();
}

void FirstRunSetup::_apply_operation_state(const godot::Dictionary &state) {
    const godot::String phase = state.get("phase", "");
    const bool terminal = phase == "succeeded" || phase == "failed" || phase == "rolled_back" ||
                          phase == "recovery_required";
    if (terminal && process_is_running(installer_pid)) {
        _set_status("Finishing Fennara setup...",
                    "Waiting for the installer process to exit cleanly");
        return;
    }
    if (phase == "succeeded") {
        step = Step::Succeeded;
        error_code = "";
        _cleanup_download();
        if (installer_cli_path.is_empty()) {
            const godot::String backup = app_paths::cli_binary_path() + godot::String(".previous");
            godot::DirAccess::remove_absolute(backup);
        }
        _release_lock();
        _set_status("Fennara is ready.", "The matching components are installed and verified.");
        emit_signal("setup_succeeded");
        return;
    }
    if (phase == "failed" || phase == "rolled_back" || phase == "recovery_required") {
        const godot::Variant value = state.get("last_error", godot::Dictionary());
        godot::Dictionary last_error;
        if (value.get_type() == godot::Variant::DICTIONARY) {
            last_error = value;
        }
        _fail(last_error.get("code", "FEN-INSTALL-FAILED"),
              last_error.get("message", "The Fennara CLI reported that setup failed."));
        return;
    }
    _set_status(operation_phase_label(phase), "Operation " + operation_id);
}

void FirstRunSetup::_cleanup_download() const {
    if (!cli_archive_path.is_empty() && godot::FileAccess::file_exists(cli_archive_path)) {
        godot::DirAccess::remove_absolute(cli_archive_path);
    }
}

godot::String FirstRunSetup::_diagnostic_report() const {
    godot::String report = "Fennara setup report\n";
    report += "Addon version: " + addon_version + "\n";
    report += "Operation: " + (operation_id.is_empty() ? "not created" : operation_id) + "\n";
    report += "Code: " + (error_code.is_empty() ? "none" : error_code) + "\n";
    report += "Status: " + status_text + "\n";
    report += "Detail: " + detail_text + "\n";
    if (!operation_state.is_empty()) {
        report += "\nSanitized operation state:\n";
        report += godot::JSON::stringify(operation_state, "  ");
        report += "\n";
    }
    return report;
}

bool FirstRunSetup::_test_failure(const godot::String &name) const {
#ifdef FENNARA_SETUP_TEST_HOOKS
    godot::OS *os = godot::OS::get_singleton();
    return os != nullptr &&
           os->get_environment("FENNARA_SETUP_TEST_FAILURE").strip_edges().to_lower() == name;
#else
    (void)name;
    return false;
#endif
}

} // namespace fennara
