#include "fennara/warning_capture.hpp"

#include <godot_cpp/core/class_db.hpp>

namespace fennara {

void FennaraWarningCapture::_bind_methods() {
}

bool FennaraWarningCapture::_matches_source_filter(
    const godot::String &file,
    const godot::Array &backtrace_frames) const {
    if (_source_paths.empty()) {
        return true;
    }
    for (const godot::String &source_path : _source_paths) {
        if (file == source_path) {
            return true;
        }
        for (int i = 0; i < backtrace_frames.size(); i++) {
            godot::Dictionary frame = backtrace_frames[i];
            if (godot::String(frame.get("file", "")) == source_path) {
                return true;
            }
        }
    }
    return false;
}

void FennaraWarningCapture::_log_error(const godot::String &p_function, const godot::String &p_file,
                                int32_t p_line, const godot::String &p_code,
                                const godot::String &p_rationale, bool p_editor_notify,
                                int32_t p_error_type,
                                const godot::TypedArray<godot::Ref<godot::ScriptBacktrace>> &p_script_backtraces) {
    godot::Dictionary entry;

    switch (p_error_type) {
        case ERROR_TYPE_WARNING:
            entry["type"] = "warning";
            break;
        case ERROR_TYPE_ERROR:
            entry["type"] = "error";
            break;
        case ERROR_TYPE_SCRIPT:
            entry["type"] = "script_error";
            break;
        case ERROR_TYPE_SHADER:
            entry["type"] = "shader_error";
            break;
        default:
            entry["type"] = "unknown";
            break;
    }

    godot::String message = p_code;
    if (!p_rationale.is_empty()) {
        message += " - " + p_rationale;
    }
    entry["message"] = message;
    entry["file"] = p_file;
    entry["line"] = p_line;
    entry["function"] = p_function;

    godot::Array backtrace_frames;
    for (int i = 0; i < p_script_backtraces.size(); i++) {
        godot::Ref<godot::ScriptBacktrace> backtrace = p_script_backtraces[i];
        if (!backtrace.is_valid() || backtrace->is_empty()) {
            continue;
        }
        int frame_count = backtrace->get_frame_count();
        for (int frame_idx = 0; frame_idx < frame_count; frame_idx++) {
            godot::Dictionary frame;
            frame["language"] = backtrace->get_language_name();
            frame["file"] = backtrace->get_frame_file(frame_idx);
            frame["line"] = backtrace->get_frame_line(frame_idx);
            frame["function"] = backtrace->get_frame_function(frame_idx);
            backtrace_frames.append(frame);
        }
    }
    if (!backtrace_frames.is_empty()) {
        entry["script_backtrace"] = backtrace_frames;
    }

    std::lock_guard<std::mutex> lock(_captured_mutex);
    if (!_matches_source_filter(p_file, backtrace_frames)) {
        return;
    }
    if (_max_entries > 0 && _captured.size() >= _max_entries) {
        _dropped_entries++;
        return;
    }
    _captured.append(entry);
}

void FennaraWarningCapture::_log_message(const godot::String &p_message, bool p_error) {
    if (!p_error) {
        return;
    }

    godot::Dictionary entry;
    entry["type"] = "error";
    entry["message"] = p_message;
    entry["file"] = "";
    entry["line"] = 0;
    entry["function"] = "";
    std::lock_guard<std::mutex> lock(_captured_mutex);
    if (!_source_paths.empty()) {
        return;
    }
    if (_max_entries > 0 && _captured.size() >= _max_entries) {
        _dropped_entries++;
        return;
    }
    _captured.append(entry);
}

godot::Array FennaraWarningCapture::get_captured() const {
    std::lock_guard<std::mutex> lock(_captured_mutex);
    godot::Array captured = _captured.duplicate(true);
    if (_dropped_entries > 0) {
        godot::Dictionary omitted;
        omitted["type"] = "warning";
        omitted["message"] =
            godot::String::num_int64(_dropped_entries) +
            " additional captured diagnostics were omitted.";
        omitted["file"] = "";
        omitted["line"] = 0;
        omitted["function"] = "";
        captured.append(omitted);
    }
    return captured;
}

void FennaraWarningCapture::clear() {
    std::lock_guard<std::mutex> lock(_captured_mutex);
    _captured.clear();
    _dropped_entries = 0;
}

void FennaraWarningCapture::configure_source_filter(
    const std::vector<godot::String> &source_paths,
    int max_entries) {
    std::lock_guard<std::mutex> lock(_captured_mutex);
    _source_paths = source_paths;
    _max_entries = max_entries;
    _dropped_entries = 0;
}

} // namespace fennara
