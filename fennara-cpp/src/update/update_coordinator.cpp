#include "fennara/update/update_coordinator.hpp"

#include "fennara/app_paths.hpp"
#include "fennara/logger.hpp"
#include "fennara/update_notice.hpp"

#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/classes/window.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {
namespace {

constexpr double kPollSeconds = 0.25;

} // namespace

void UpdateCoordinator::_bind_methods() {
    ADD_SIGNAL(godot::MethodInfo("state_changed"));
    godot::ClassDB::bind_method(godot::D_METHOD("start_prepare"),
                                &UpdateCoordinator::start_prepare);
    godot::ClassDB::bind_method(godot::D_METHOD("confirm_close_and_install"),
                                &UpdateCoordinator::confirm_close_and_install);
    godot::ClassDB::bind_method(godot::D_METHOD("restore_previous_version"),
                                &UpdateCoordinator::restore_previous_version);
    godot::ClassDB::bind_method(godot::D_METHOD("dismiss"), &UpdateCoordinator::dismiss);
    godot::ClassDB::bind_method(godot::D_METHOD("retry"), &UpdateCoordinator::retry);
    godot::ClassDB::bind_method(godot::D_METHOD("cancel_waiting"),
                                &UpdateCoordinator::cancel_waiting);
    godot::ClassDB::bind_method(godot::D_METHOD("copy_report"),
                                &UpdateCoordinator::copy_report);
    godot::ClassDB::bind_method(godot::D_METHOD("open_logs"),
                                &UpdateCoordinator::open_logs);
    godot::ClassDB::bind_method(godot::D_METHOD("should_show"),
                                &UpdateCoordinator::should_show);
    godot::ClassDB::bind_method(godot::D_METHOD("is_preparing"),
                                &UpdateCoordinator::is_preparing);
    godot::ClassDB::bind_method(godot::D_METHOD("is_ready_to_close"),
                                &UpdateCoordinator::is_ready_to_close);
    godot::ClassDB::bind_method(godot::D_METHOD("is_waiting_for_godot"),
                                &UpdateCoordinator::is_waiting_for_godot);
    godot::ClassDB::bind_method(godot::D_METHOD("has_failed"),
                                &UpdateCoordinator::has_failed);
    godot::ClassDB::bind_method(godot::D_METHOD("needs_recovery"),
                                &UpdateCoordinator::needs_recovery);
    godot::ClassDB::bind_method(godot::D_METHOD("get_status"),
                                &UpdateCoordinator::get_status);
    godot::ClassDB::bind_method(godot::D_METHOD("get_detail"),
                                &UpdateCoordinator::get_detail);
    godot::ClassDB::bind_method(godot::D_METHOD("get_error_code"),
                                &UpdateCoordinator::get_error_code);
    godot::ClassDB::bind_method(godot::D_METHOD("get_operation_id"),
                                &UpdateCoordinator::get_operation_id);
    godot::ClassDB::bind_method(godot::D_METHOD("get_target_version"),
                                &UpdateCoordinator::get_target_version);
    godot::ClassDB::bind_method(godot::D_METHOD("get_release_track"),
                                &UpdateCoordinator::get_release_track);
    godot::ClassDB::bind_method(godot::D_METHOD("get_release_channel"),
                                &UpdateCoordinator::get_release_channel);
}

void UpdateCoordinator::_ready() {
    set_process(true);
    _scan_pending_updates();
}

void UpdateCoordinator::_process(double delta) {
    poll_timer -= delta;
    if (poll_timer <= 0.0) {
        poll_timer = kPollSeconds;
        if (step == Step::Idle) {
            _scan_pending_updates();
        } else if (step == Step::Preparing || step == Step::WaitingForGodot) {
            _poll_operation();
        }
    }
}

void UpdateCoordinator::start_prepare() {
    if (step == Step::Preparing || step == Step::WaitingForGodot) {
        return;
    }
    if (step == Step::ReadyToClose || step == Step::RecoveryRequired) {
        dismissed = false;
        emit_signal("state_changed");
        return;
    }
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr || !godot::FileAccess::file_exists(app_paths::cli_binary_path())) {
        _fail("FEN-UPDATE-CLI-MISSING", "The installed Fennara CLI is unavailable.");
        return;
    }
    dismissed = false;
    error_code = "";
    target_version = update_notice::latest_version();
    release_track = update_notice::track();
    release_channel = update_notice::channel();
    release_tag = update_notice::target_release_tag();
    source_commit = update_notice::source_commit();
    if (target_version.is_empty()) {
        _fail("FEN-UPDATE-TARGET-MISSING", "No exact Fennara update target was resolved.");
        return;
    }
    const uint64_t now = static_cast<uint64_t>(
        godot::Time::get_singleton()->get_unix_time_from_system() * 1000.0);
    operation_id = "update-" + godot::String::num_uint64(now) + "-godot-" +
                   godot::String::num_int64(os->get_process_id());
    staging_root = _update_root().path_join(operation_id);

    godot::PackedStringArray args;
    args.append("update");
    args.append("--prepare");
    args.append("--version");
    args.append(target_version);
    args.append("--project");
    args.append(_project_path());
    args.append("--operation-id");
    args.append(operation_id);
    args.append("--godot-pid");
    args.append(godot::String::num_int64(os->get_process_id()));
    args.append("--godot-executable");
    args.append(os->get_executable_path());
    child_pid = os->create_process(app_paths::cli_binary_path(), args, false);
    if (child_pid <= 0) {
        _fail("FEN-UPDATE-CLI-LAUNCH", "The Fennara update process could not be started.");
        return;
    }
    _set_step(Step::Preparing, "Preparing the Fennara update...",
              "Downloading and verifying every required file while Godot stays open");
}

void UpdateCoordinator::confirm_close_and_install() {
    if (step != Step::ReadyToClose || !_launch_completion("__complete-project-update")) {
        return;
    }
    _set_step(Step::WaitingForGodot, "Waiting for Godot to close...",
              "The external updater will install the staged files after this editor exits");
    _request_editor_close();
}

void UpdateCoordinator::restore_previous_version() {
    if (step != Step::RecoveryRequired || !_launch_completion("__rollback-project-update")) {
        return;
    }
    _set_step(Step::WaitingForGodot, "Waiting for Godot to close...",
              "The external updater will restore the previous Fennara version");
    _request_editor_close();
}

void UpdateCoordinator::dismiss() {
    dismissed = true;
    emit_signal("state_changed");
}

void UpdateCoordinator::retry() {
    if (step == Step::Failed) {
        start_prepare();
    } else if (step == Step::ReadyToClose) {
        dismissed = false;
        emit_signal("state_changed");
    }
}

void UpdateCoordinator::cancel_waiting() {
    if (step != Step::WaitingForGodot || staging_root.is_empty()) {
        return;
    }
    godot::Ref<godot::FileAccess> file = godot::FileAccess::open(
        staging_root.path_join("cancel"), godot::FileAccess::WRITE);
    if (file.is_valid()) {
        file->store_string("cancel\n");
    }
}

void UpdateCoordinator::copy_report() {
    godot::DisplayServer *display = godot::DisplayServer::get_singleton();
    if (display == nullptr) {
        return;
    }
    const godot::Dictionary state = _read_operation();
    const godot::Dictionary components = state.get("components", godot::Dictionary());
    const godot::String phase = state.get("phase", "unknown");
    const godot::String addon = components.get("addon", update_notice::current_version());
    const godot::String cli = components.get("cli", "unknown");
    const godot::String runtime = components.get("installed_runtime", "unknown");
    godot::String report = "Fennara update report\nOperation: " + operation_id +
                           "\nTrack: " + (release_track.is_empty() ? "unknown" : release_track) +
                           "\nChannel: " + (release_channel.is_empty() ? "none" : release_channel) +
                           "\nInstalled addon: " + addon + "\nActive CLI: " + cli +
                           "\nActive runtime: " + runtime + "\nResolved target: " +
                           (target_version.is_empty() ? "unknown" : target_version) +
                           "\nRelease tag: " + (release_tag.is_empty() ? "none" : release_tag) +
                           "\nSource commit: " + (source_commit.is_empty() ? "unknown" : source_commit) +
                           "\nLast stage: " + phase + "\nStatus: " + status_text +
                           "\nDetail: " + detail_text +
                           "\nCode: " + (error_code.is_empty() ? "none" : error_code) + "\n";
    display->clipboard_set(report);
}

void UpdateCoordinator::open_logs() {
    godot::OS *os = godot::OS::get_singleton();
    if (os != nullptr && !operation_id.is_empty()) {
        os->shell_show_in_file_manager(
            app_paths::operation_logs_dir().path_join(operation_id + godot::String(".jsonl")), true);
    }
}

bool UpdateCoordinator::should_show() const {
    return !dismissed && step != Step::Idle;
}
bool UpdateCoordinator::is_preparing() const { return step == Step::Preparing; }
bool UpdateCoordinator::is_ready_to_close() const { return step == Step::ReadyToClose; }
bool UpdateCoordinator::is_waiting_for_godot() const { return step == Step::WaitingForGodot; }
bool UpdateCoordinator::has_failed() const { return step == Step::Failed; }
bool UpdateCoordinator::needs_recovery() const { return step == Step::RecoveryRequired; }
godot::String UpdateCoordinator::get_status() const { return status_text; }
godot::String UpdateCoordinator::get_detail() const { return detail_text; }
godot::String UpdateCoordinator::get_error_code() const { return error_code; }
godot::String UpdateCoordinator::get_operation_id() const { return operation_id; }
godot::String UpdateCoordinator::get_target_version() const { return target_version; }
godot::String UpdateCoordinator::get_release_track() const { return release_track; }
godot::String UpdateCoordinator::get_release_channel() const { return release_channel; }

bool UpdateCoordinator::_launch_completion(const godot::String &command) {
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr) {
        _fail("FEN-UPDATE-HANDOFF-FAILED", "Godot could not launch the external updater.");
        return false;
    }
    godot::PackedStringArray args;
    args.append(command);
    args.append("--project");
    args.append(_project_path());
    args.append("--resume-operation");
    args.append(operation_id);
    args.append("--wait-for-pid");
    args.append(godot::String::num_int64(os->get_process_id()));
    args.append("--godot-executable");
    args.append(os->get_executable_path());
    child_pid = os->create_process(app_paths::cli_binary_path(), args, false);
    if (child_pid <= 0) {
        _fail("FEN-UPDATE-HANDOFF-FAILED", "The external Fennara updater could not start.");
        return false;
    }
    return true;
}

void UpdateCoordinator::_request_editor_close() {
    godot::SceneTree *tree = get_tree();
    if (tree == nullptr || tree->get_root() == nullptr) {
        _fail("FEN-UPDATE-EDITOR-CLOSE", "Godot could not request normal editor shutdown.");
        return;
    }
    tree->get_root()->call_deferred("propagate_notification",
                                    godot::Node::NOTIFICATION_WM_CLOSE_REQUEST);
}

void UpdateCoordinator::_set_step(Step next, const godot::String &status,
                                  const godot::String &detail) {
    step = next;
    status_text = status;
    detail_text = detail;
    emit_signal("state_changed");
}

void UpdateCoordinator::_fail(const godot::String &code, const godot::String &message) {
    error_code = code;
    _set_step(Step::Failed, "Fennara update could not finish.", message);
    FLOG_ERR("Native update failed " + code + ": " + message);
}

} // namespace fennara
