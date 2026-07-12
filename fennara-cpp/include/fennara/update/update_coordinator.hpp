#pragma once

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara {

class UpdateCoordinator : public godot::Node {
    GDCLASS(UpdateCoordinator, godot::Node)

public:
    enum class Step {
        Idle,
        Preparing,
        ReadyToClose,
        WaitingForGodot,
        Failed,
        RecoveryRequired,
    };

    UpdateCoordinator() = default;

    void _ready() override;
    void _process(double delta) override;

    void start_prepare();
    void confirm_close_and_install();
    void restore_previous_version();
    void dismiss();
    void retry();
    void cancel_waiting();
    void copy_report();
    void open_logs();

    bool should_show() const;
    bool is_preparing() const;
    bool is_ready_to_close() const;
    bool is_waiting_for_godot() const;
    bool has_failed() const;
    bool needs_recovery() const;
    godot::String get_status() const;
    godot::String get_detail() const;
    godot::String get_error_code() const;
    godot::String get_operation_id() const;
    godot::String get_target_version() const;

protected:
    static void _bind_methods();

private:
    Step step = Step::Idle;
    godot::String status_text;
    godot::String detail_text;
    godot::String error_code;
    godot::String operation_id;
    godot::String target_version;
    godot::String staging_root;
    int64_t child_pid = -1;
    double poll_timer = 0.0;
    bool dismissed = false;

    godot::String _project_path() const;
    godot::String _update_root() const;
    godot::String _receipt_path() const;
    godot::Dictionary _read_operation() const;
    godot::Dictionary _read_receipt(const godot::String &root) const;
    void _poll_operation();
    void _scan_pending_updates();
    void _write_activation_handshake(const godot::String &root,
                                     const godot::Dictionary &receipt) const;
    bool _launch_completion(const godot::String &command);
    void _request_editor_close();
    void _set_step(Step next, const godot::String &status, const godot::String &detail);
    void _fail(const godot::String &code, const godot::String &message);
};

} // namespace fennara
