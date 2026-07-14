#pragma once

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/packed_byte_array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>

#include "fennara/release/identity.hpp"

namespace godot {
class HTTPRequest;
}

namespace fennara {

bool installed_components_match_addon();

class FirstRunSetup : public godot::Node {
    GDCLASS(FirstRunSetup, godot::Node)

  protected:
    static void _bind_methods();

  public:
    FirstRunSetup() = default;

    void _ready() override;
    void _process(double delta) override;
    void _exit_tree() override;

    bool is_setup_required() const;
    bool is_running() const;
    bool has_failed() const;
    bool has_succeeded() const;
    godot::String get_status() const;
    godot::String get_detail() const;
    godot::String get_error_code() const;
    godot::String get_operation_id() const;

    void start(const godot::String &project_path, const godot::String &addon_version);
    void retry();
    void open_logs() const;
    void copy_report() const;

  private:
    enum class Step {
        Idle,
        WaitingForLock,
        DownloadingManifest,
        DownloadingCli,
        LaunchingInstaller,
        Installing,
        Succeeded,
        Failed,
    };

    godot::HTTPRequest *manifest_request = nullptr;
    godot::HTTPRequest *cli_request = nullptr;
    Step step = Step::Idle;
    godot::String project_path;
    godot::String addon_version;
    release_identity::Identity addon_identity;
    godot::String cli_asset_name;
    godot::String expected_cli_sha256;
    godot::String download_dir;
    godot::String cli_archive_path;
    godot::String installer_cli_path;
    godot::String operation_id;
    godot::Dictionary operation_state;
    godot::String status_text = "Fennara needs to finish setup.";
    godot::String detail_text;
    godot::String error_code;
    int32_t installer_pid = -1;
    uint64_t installer_started_at_ms = 0;
    double operation_poll_timer = 0.0;
    double lock_poll_timer = 0.0;
    uint64_t last_operation_updated_at_ms = 0;
    uint64_t process_exit_observed_at_ms = 0;
    godot::String bootstrap_lock_path;
    bool owns_bootstrap_lock = false;

    void _set_status(const godot::String &status, const godot::String &detail = "");
    void _fail(const godot::String &code, const godot::String &message);
    godot::String _release_asset_url(const godot::String &asset_name) const;
    godot::String _platform_key() const;
    godot::String _cli_archive_entry() const;
    bool _prepare_download_paths();
    void _continue_start();
    bool _installed_components_match() const;
    bool _try_acquire_lock();
    bool _write_lock_owner(int32_t pid) const;
    void _release_lock();
    bool _install_verified_cli();
    bool _launch_installer();
    void _poll_operation();
    godot::Dictionary _find_install_operation() const;
    void _apply_operation_state(const godot::Dictionary &state);
    void _cleanup_download() const;
    godot::String _diagnostic_report() const;
    bool _test_failure(const godot::String &name) const;

    void _on_manifest_request_completed(int64_t result, int64_t response_code,
                                        godot::PackedStringArray headers,
                                        godot::PackedByteArray body);
    void _on_cli_request_completed(int64_t result, int64_t response_code,
                                   godot::PackedStringArray headers, godot::PackedByteArray body);
};

} // namespace fennara
