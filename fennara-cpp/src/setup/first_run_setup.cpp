#include "fennara/setup/first_run_setup.hpp"

#include "fennara/app_paths.hpp"
#include "fennara/logger.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/http_client.hpp>
#include <godot_cpp/classes/http_request.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {
namespace {

constexpr const char *kReleaseBase =
    "https://github.com/fennaraOfficial/fennara-godot-ai/releases/download";

bool valid_release_version(const godot::String &version) {
    if (version.is_empty()) {
        return false;
    }
    for (int i = 0; i < version.length(); i++) {
        const char32_t c = version[i];
        const bool alpha_numeric =
            (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
        if (!alpha_numeric && c != '.' && c != '-' && c != '+') {
            return false;
        }
    }
    const godot::String core = version.split("-", false)[0];
    const godot::PackedStringArray parts = core.split(".", false);
    return parts.size() == 3 && parts[0].is_valid_int() && parts[1].is_valid_int() &&
           parts[2].is_valid_int();
}

} // namespace

void FirstRunSetup::_bind_methods() {
    ADD_SIGNAL(godot::MethodInfo("state_changed"));
    ADD_SIGNAL(godot::MethodInfo("setup_succeeded"));

    godot::ClassDB::bind_method(godot::D_METHOD("_on_manifest_request_completed", "result",
                                                "response_code", "headers", "body"),
                                &FirstRunSetup::_on_manifest_request_completed);
    godot::ClassDB::bind_method(
        godot::D_METHOD("_on_cli_request_completed", "result", "response_code", "headers", "body"),
        &FirstRunSetup::_on_cli_request_completed);
    godot::ClassDB::bind_method(godot::D_METHOD("is_setup_required"),
                                &FirstRunSetup::is_setup_required);
    godot::ClassDB::bind_method(godot::D_METHOD("is_running"), &FirstRunSetup::is_running);
    godot::ClassDB::bind_method(godot::D_METHOD("has_failed"), &FirstRunSetup::has_failed);
    godot::ClassDB::bind_method(godot::D_METHOD("has_succeeded"), &FirstRunSetup::has_succeeded);
    godot::ClassDB::bind_method(godot::D_METHOD("get_status"), &FirstRunSetup::get_status);
    godot::ClassDB::bind_method(godot::D_METHOD("get_detail"), &FirstRunSetup::get_detail);
    godot::ClassDB::bind_method(godot::D_METHOD("get_error_code"), &FirstRunSetup::get_error_code);
    godot::ClassDB::bind_method(godot::D_METHOD("get_operation_id"),
                                &FirstRunSetup::get_operation_id);
    godot::ClassDB::bind_method(godot::D_METHOD("start", "project_path", "addon_version"),
                                &FirstRunSetup::start);
    godot::ClassDB::bind_method(godot::D_METHOD("retry"), &FirstRunSetup::retry);
}

void FirstRunSetup::_ready() {
    manifest_request = memnew(godot::HTTPRequest);
    manifest_request->set_timeout(30.0);
    add_child(manifest_request);
    manifest_request->connect("request_completed",
                              callable_mp(this, &FirstRunSetup::_on_manifest_request_completed));

    cli_request = memnew(godot::HTTPRequest);
    cli_request->set_timeout(120.0);
    add_child(cli_request);
    cli_request->connect("request_completed",
                         callable_mp(this, &FirstRunSetup::_on_cli_request_completed));
    set_process(true);
}

void FirstRunSetup::_process(double delta) {
    if (step == Step::WaitingForLock) {
        lock_poll_timer -= delta;
        if (lock_poll_timer <= 0.0) {
            lock_poll_timer = 0.5;
            if (_try_acquire_lock()) {
                _continue_start();
            }
        }
        return;
    }
    if (step != Step::Installing) {
        return;
    }
    operation_poll_timer -= delta;
    if (operation_poll_timer <= 0.0) {
        operation_poll_timer = 0.25;
        _poll_operation();
    }
}

bool FirstRunSetup::is_setup_required() const {
#ifdef FENNARA_SETUP_TEST_HOOKS
    godot::OS *os = godot::OS::get_singleton();
    if (os != nullptr && os->get_environment("FENNARA_FORCE_FIRST_RUN_SETUP") == "1") {
        return true;
    }
#endif
    return !_installed_components_match();
}

bool FirstRunSetup::is_running() const {
    return step == Step::WaitingForLock || step == Step::DownloadingManifest ||
           step == Step::DownloadingCli || step == Step::LaunchingInstaller ||
           step == Step::Installing;
}

bool FirstRunSetup::has_failed() const {
    return step == Step::Failed;
}

bool FirstRunSetup::has_succeeded() const {
    return step == Step::Succeeded;
}

godot::String FirstRunSetup::get_status() const {
    return status_text;
}

godot::String FirstRunSetup::get_detail() const {
    return detail_text;
}

godot::String FirstRunSetup::get_error_code() const {
    return error_code;
}

godot::String FirstRunSetup::get_operation_id() const {
    return operation_id;
}

void FirstRunSetup::start(const godot::String &next_project_path,
                          const godot::String &next_addon_version) {
    if (is_running()) {
        return;
    }

    project_path = next_project_path.strip_edges();
    addon_version = next_addon_version.strip_edges();
    operation_id = "";
    operation_state.clear();
    error_code = "";
    installer_pid = -1;
    installer_cli_path = "";

    if (project_path.is_empty() || !valid_release_version(addon_version)) {
        _fail("FEN-SETUP-PROJECT-INVALID",
              "The project path or addon version is missing or invalid.");
        return;
    }
    if (!_try_acquire_lock()) {
        if (!has_failed()) {
            step = Step::WaitingForLock;
            lock_poll_timer = 0.5;
            _set_status("Waiting for another Fennara setup...",
                        "Setup will continue automatically when the shared installer is available");
        }
        return;
    }
    _continue_start();
}

void FirstRunSetup::_continue_start() {
    godot::OS *os = godot::OS::get_singleton();
#ifdef FENNARA_SETUP_TEST_HOOKS
    const bool forced =
        os != nullptr && os->get_environment("FENNARA_FORCE_FIRST_RUN_SETUP") == "1";
#else
    const bool forced = false;
#endif
    if (!forced && _installed_components_match()) {
        step = Step::Succeeded;
        _release_lock();
        _set_status("Fennara is ready.",
                    "Another editor completed the matching shared installation.");
        emit_signal("setup_succeeded");
        return;
    }
    if (_test_failure("manifest")) {
        _fail("FEN-SETUP-MANIFEST-DOWNLOAD", "Simulated release manifest download failure.");
        return;
    }

#ifdef FENNARA_SETUP_TEST_HOOKS
    if (os != nullptr && os->get_environment("FENNARA_SETUP_TEST_SUCCESS") == "1") {
        operation_id = "install-test-success";
        operation_state["operation_id"] = operation_id;
        operation_state["phase"] = "succeeded";
        operation_state["updated_at_unix_ms"] = 1;
        _apply_operation_state(operation_state);
        return;
    }
    const godot::String local_cli =
        os != nullptr ? os->get_environment("FENNARA_SETUP_CLI_PATH").strip_edges()
                      : godot::String();
    if (!local_cli.is_empty()) {
        if (!godot::FileAccess::file_exists(local_cli)) {
            _fail("FEN-SETUP-CLI-LAUNCH",
                  "The CLI configured by FENNARA_SETUP_CLI_PATH does not exist.");
            return;
        }
        installer_cli_path = local_cli;
        step = Step::LaunchingInstaller;
        _set_status("Starting the local Fennara test CLI...",
                    "The installed Fennara CLI will not be replaced in this test mode");
        _launch_installer();
        return;
    }
#endif
    if (!_prepare_download_paths()) {
        return;
    }

    step = Step::DownloadingManifest;
    _set_status("Checking the matching Fennara release...", "Addon version " + addon_version);
    const godot::String manifest_name = "fennara-release-manifest-v" + addon_version + ".json";
    const godot::Error request_error = manifest_request->request(
        _release_asset_url(manifest_name),
        godot::PackedStringArray{"Accept: application/json", "User-Agent: fennara-godot-setup"},
        godot::HTTPClient::METHOD_GET);
    if (request_error != godot::OK) {
        _fail("FEN-SETUP-MANIFEST-DOWNLOAD", "Could not start the release manifest download.");
    }
}

void FirstRunSetup::retry() {
    if (!has_failed()) {
        return;
    }
    _cleanup_download();
    start(project_path, addon_version);
}

void FirstRunSetup::_set_status(const godot::String &status, const godot::String &detail) {
    status_text = status;
    detail_text = detail;
    emit_signal("state_changed");
}

void FirstRunSetup::_fail(const godot::String &code, const godot::String &message) {
    step = Step::Failed;
    error_code = code;
    _cleanup_download();
    _release_lock();
    _set_status("Fennara setup could not finish.", message);
    FLOG_ERR("First-run setup failed " + code + ": " + message);
}

godot::String FirstRunSetup::_release_asset_url(const godot::String &asset_name) const {
    return godot::String(kReleaseBase) + "/v" + addon_version + "/" + asset_name;
}

godot::String FirstRunSetup::_platform_key() const {
    godot::OS *os = godot::OS::get_singleton();
    godot::Engine *engine = godot::Engine::get_singleton();
    if (os == nullptr || engine == nullptr) {
        return "";
    }
    godot::String platform = os->get_name().to_lower();
    if (platform != "windows" && platform != "macos" && platform != "linux") {
        return "";
    }
    godot::String architecture = engine->get_architecture_name().to_lower();
    if (architecture == "aarch64") {
        architecture = "arm64";
    } else if (architecture == "x86-64" || architecture == "amd64") {
        architecture = "x86_64";
    }
    if (architecture != "arm64" && architecture != "x86_64") {
        return "";
    }
    return platform + "-" + architecture;
}

godot::String FirstRunSetup::_cli_archive_entry() const {
    godot::OS *os = godot::OS::get_singleton();
    const bool windows = os != nullptr && os->get_name() == "Windows";
    return windows ? "bin/fennara.exe" : "bin/fennara";
}

bool FirstRunSetup::_prepare_download_paths() {
    const godot::String app_dir = app_paths::app_dir();
    if (app_dir.is_empty()) {
        _fail("FEN-SETUP-APP-DATA", "Fennara could not determine its app-data directory.");
        return false;
    }
    download_dir = app_dir.path_join("cache").path_join("setup").path_join(addon_version);
    if (godot::DirAccess::make_dir_recursive_absolute(download_dir) != godot::OK) {
        _fail("FEN-SETUP-STAGE-FILESYSTEM",
              "Fennara could not create its setup staging directory.");
        return false;
    }
    cli_archive_path = download_dir.path_join("fennara-cli.zip");
    if (godot::FileAccess::file_exists(cli_archive_path)) {
        godot::DirAccess::remove_absolute(cli_archive_path);
    }
    return true;
}

} // namespace fennara
