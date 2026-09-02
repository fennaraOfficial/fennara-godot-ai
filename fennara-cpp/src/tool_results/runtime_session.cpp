#include "fennara/tool_results/runtime_session.hpp"

#include "fennara/tool_results/envelope.hpp"
#include "fennara/tool_results/runtime_log_excerpt.hpp"

#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara::tool_results {

namespace {

godot::String normalize_path_for_model(const godot::String &path) {
    godot::String normalized = path.replace("\\", "/");
    if (normalized.begins_with("res://") || normalized.begins_with("user://")) {
        return normalized;
    }

    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        return normalized;
    }

    godot::String user_root =
        settings->globalize_path("user://").replace("\\", "/");
    if (!user_root.ends_with("/")) {
        user_root += "/";
    }
    if (normalized.begins_with(user_root)) {
        return "user://" + normalized.substr(user_root.length());
    }
    return normalized;
}

godot::String status_value(const godot::Dictionary &raw_result) {
    godot::String status = raw_result.get("status", "");
    if (!status.is_empty()) {
        return status;
    }
    if (raw_result.has("running")) {
        return (bool)raw_result.get("running", false) ? "running" : "exited";
    }
    return "unknown";
}

godot::String bool_text(bool value) {
    return value ? "yes" : "no";
}

godot::String exit_code_note(const godot::String &status) {
    if (status == "stopped") {
        return " (expected when `runtime_session.stop` terminates a managed scene; inspect runtime issues/logs to judge failure)";
    }
    if (status == "exited") {
        return " (the scene process has exited; inspect runtime issues/logs to judge failure)";
    }
    return "";
}

void append_if_present(godot::PackedStringArray &lines,
                       const godot::String &label,
                       const godot::Dictionary &raw_result,
                       const godot::String &key) {
    godot::String value = raw_result.get(key, "");
    if (!value.is_empty()) {
        godot::String visible = normalize_path_for_model(value);
        lines.append(label + godot::String(": ") + visible);
        if (visible != value) {
            lines.append(label + godot::String(" absolute: ") +
                         value.replace("\\", "/"));
        }
    }
}

void append_integer_if_present(godot::PackedStringArray &lines,
                               const godot::String &label,
                               const godot::Dictionary &raw_result,
                               const godot::String &key,
                               const godot::String &suffix = "") {
    godot::Variant value = raw_result.get(key, godot::Variant());
    if (value.get_type() == godot::Variant::INT) {
        lines.append(label + godot::String(": ") +
                     godot::String::num_int64(static_cast<int64_t>(value)) +
                     suffix);
    }
}

} // namespace

godot::Dictionary format_runtime_session(const godot::Dictionary &raw_input) {
    godot::Dictionary raw_result = raw_input;
    bool anonymous_busy =
        godot::String(raw_input.get("status", "")) == "busy" &&
        !(bool)raw_input.get("slot_acquired", false);
    if (anonymous_busy) {
        // Anonymous contention is deliberately capability-only. Do not let an
        // accidental daemon field expose another project's runtime details.
        godot::Dictionary sanitized;
        sanitized["success"] = raw_input.get("success", false);
        sanitized["ok"] = raw_input.get("ok", false);
        sanitized["tool_name"] = "runtime_session";
        sanitized["format_version"] = "runtime-session-result-v1";
        sanitized["status"] = "busy";
        sanitized["availability"] = "busy";
        sanitized["slot_acquired"] = false;
        if (raw_input.has("retry_after_ms")) {
            sanitized["retry_after_ms"] = raw_input["retry_after_ms"];
        }
        raw_result = sanitized;
    }

    godot::String status = status_value(raw_result);
    godot::PackedStringArray lines;
    lines.append("Tool: runtime_session");
    lines.append("Status: " + status);
    append_if_present(lines, "End reason", raw_result, "end_reason");
    append_if_present(lines, "Code", raw_result, "code");

    godot::String availability = raw_result.get("availability", "");
    if (!availability.is_empty()) {
        lines.append("Runtime slot availability: " + availability);
    }
    if (raw_result.has("slot_acquired")) {
        lines.append(
            "Runtime slot owned by this project: " +
            godot::String((bool)raw_result.get("slot_acquired", false)
                              ? "yes"
                              : "no"));
    }
    append_integer_if_present(
        lines, "Suggested retry delay", raw_result, "retry_after_ms", " ms");

    if (raw_result.has("error")) {
        lines.append("");
        lines.append(godot::String(raw_result.get("error", "")));
    }

    append_if_present(lines, "Playing scene", raw_result, "playing_scene");
    append_if_present(lines, "Scene", raw_result, "scene_path");
    append_if_present(lines, "Session id", raw_result, "session_id");
    append_if_present(lines, "Log file", raw_result, "log_path");
    append_if_present(lines, "Captures dir", raw_result, "captures_dir");
    godot::Dictionary startup_capture =
        raw_result.get("startup_capture", godot::Dictionary());
    if (!startup_capture.is_empty() &&
        (bool)startup_capture.get("success", false)) {
        godot::String startup_image =
            normalize_path_for_model(
                godot::String(startup_capture.get("image_path", "")));
        if (!startup_image.is_empty()) {
            lines.append("Startup capture: " + startup_image);
        }
    }
    if (!godot::String(raw_result.get("log_path", "")).is_empty()) {
        lines.append(
            "Session log remains the full source of truth; this receipt includes new log lines since the previous runtime receipt when available.");
    }
    append_runtime_log_excerpt(lines, raw_result);

    if (raw_result.has("running")) {
        lines.append(
            godot::String("Session process running: ") +
            bool_text((bool)raw_result.get("running", false)));
    }
    godot::Variant exit_code = raw_result.get("exit_code", godot::Variant());
    if (exit_code.get_type() != godot::Variant::NIL) {
        lines.append("Process exit code: " + godot::String(exit_code) +
                     exit_code_note(status));
    }

    if (raw_result.has("script_running")) {
        lines.append(
            "Script running: " +
            godot::String((bool)raw_result.get("script_running", false)
                              ? "true"
                              : "false"));
    }
    if (raw_result.has("startup_log_wait_ms")) {
        lines.append(
            "Startup log wait: " +
            godot::String::num_int64(
                static_cast<int64_t>(raw_result.get("startup_log_wait_ms", 0))) +
            " ms");
        if (!(bool)raw_result.get("startup_ready_seen", false)) {
            lines.append(
                "Runtime helper did not report scene ready before the startup deadline. The daemon stopped and reaped the process; inspect `runtime_session.log` before retrying.");
        } else if (!(bool)raw_result.get("startup_orientation_seen", false)) {
            lines.append(
                "Runtime helper reported scene ready without completing startup orientation before the deadline. The daemon stopped and reaped the process; inspect `runtime_session.log` before retrying.");
        }
    }
    if (raw_result.has("max_run_seconds")) {
        double seconds = static_cast<double>(
            raw_result.get("max_run_seconds", 0.0));
        if (seconds > 0.0) {
            lines.append("Max run seconds: " + godot::String::num(seconds));
        }
    }
    if ((bool)raw_result.get("slot_acquired", false)) {
        append_integer_if_present(
            lines,
            "Absolute lease remaining",
            raw_result,
            "absolute_remaining_seconds",
            " seconds");
        append_integer_if_present(
            lines,
            "Inactivity lease remaining",
            raw_result,
            "inactivity_remaining_seconds",
            " seconds");
        append_integer_if_present(
            lines, "Absolute lease deadline", raw_result, "absolute_deadline_ms", " ms Unix time");
        append_integer_if_present(
            lines, "Inactivity lease deadline", raw_result, "inactivity_deadline_ms", " ms Unix time");
        if (raw_result.has("inactivity_deadline_ms") &&
            raw_result.get("inactivity_deadline_ms", godot::Variant()).get_type() ==
                godot::Variant::NIL) {
            lines.append(
                "Inactivity lease: suspended while this project's bounded runtime operation is active");
        }
    }

    godot::Array launch_errors =
        raw_result.get("launch_errors", godot::Array());
    godot::Array runtime_debugger_errors =
        raw_result.get("runtime_debugger_errors", godot::Array());
    godot::Array latest_runtime_issues =
        raw_result.get("latest_runtime_issues", godot::Array());
    godot::Array shown_runtime_issues =
        !runtime_debugger_errors.is_empty() ? runtime_debugger_errors : latest_runtime_issues;
    if (!shown_runtime_issues.is_empty()) {
        lines.append("");
        lines.append("Runtime debugger issues:");
        int count = shown_runtime_issues.size() < 6 ? shown_runtime_issues.size() : 6;
        for (int i = 0; i < count; i++) {
            if (shown_runtime_issues[i].get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            godot::Dictionary issue = shown_runtime_issues[i];
            godot::String message = issue.get("message", "");
            int repeated = static_cast<int>(issue.get("count", 1));
            lines.append(
                "- " + message +
                (repeated > 1
                     ? godot::String(" (repeated ") +
                           godot::String::num_int64(repeated) + " times)"
                     : godot::String()));

            godot::Array raw_lines = issue.get("raw_lines", godot::Array());
            int detail_count = raw_lines.size() < 4 ? raw_lines.size() : 4;
            for (int j = 1; j < detail_count; j++) {
                lines.append("  - " + godot::String(raw_lines[j]));
            }
        }
        if (shown_runtime_issues.size() > count) {
            lines.append(
                "- ... " +
                godot::String::num_int64(shown_runtime_issues.size() - count) +
                " more runtime issue(s) omitted");
        }
    }
    godot::Array msbuild_issues =
        raw_result.get("msbuild_issues", godot::Array());
    godot::Dictionary csharp_build =
        raw_result.get("csharp_build", godot::Dictionary());
    if (!csharp_build.is_empty() &&
        (bool)csharp_build.get("needed", false)) {
        lines.append("");
        lines.append("C# build:");
        lines.append("- Status: " + godot::String(csharp_build.get("status", "")));
        lines.append("- Command: " + godot::String(csharp_build.get("command", "dotnet build")));
        if (godot::String(csharp_build.get("output_mode", "")) ==
            "godot_runtime") {
            lines.append(
                "- Output: Godot Debug assembly for the runtime session");
            lines.append(
                "- Editor reload: an open Godot editor may detect and reload this assembly");
        }
        lines.append("- Duration: " +
                     godot::String::num((double)csharp_build.get("duration_seconds", 0.0)) +
                     "s");
        if (godot::String(csharp_build.get("status", "")) != "success") {
            godot::String output = csharp_build.get("output", "");
            if (!output.strip_edges().is_empty()) {
                lines.append("");
                lines.append(output.strip_edges());
            }
        }
    }

    godot::Dictionary preflight =
        raw_result.get("preflight", godot::Dictionary());
    if (!preflight.is_empty()) {
        godot::Dictionary preflight_summary =
            preflight.get("summary", godot::Dictionary());
        int preflight_errors =
            static_cast<int>(preflight_summary.get("errors", 0));
        int preflight_warnings =
            static_cast<int>(preflight_summary.get("warnings", 0));
        lines.append("");
        lines.append("Scene preflight:");
        lines.append("- Errors: " + godot::String::num_int64(preflight_errors) +
                     ", warnings: " + godot::String::num_int64(preflight_warnings));
        if (preflight_errors > 0) {
            godot::Array scenes = preflight.get("scenes", godot::Array());
            for (int i = 0; i < scenes.size(); i++) {
                if (scenes[i].get_type() != godot::Variant::DICTIONARY) {
                    continue;
                }
                godot::Dictionary scene = scenes[i];
                godot::Array issues = scene.get("issues", godot::Array());
                for (int j = 0; j < issues.size() && j < 8; j++) {
                    if (issues[j].get_type() != godot::Variant::DICTIONARY) {
                        continue;
                    }
                    godot::Dictionary issue = issues[j];
                    if (godot::String(issue.get("severity", "")) != "error") {
                        continue;
                    }
                    lines.append("- " + godot::String(issue.get("message", "")));
                }
            }
        }
    }

    godot::Dictionary script_preflight =
        raw_result.get("script_preflight", godot::Dictionary());
    if (!script_preflight.is_empty()) {
        bool script_preflight_succeeded =
            script_preflight.get("success", false);
        int script_errors =
            static_cast<int>(script_preflight.get("error_count", 0));
        int script_warnings =
            static_cast<int>(script_preflight.get("warning_count", 0));
        int checked_scripts =
            static_cast<int>(script_preflight.get("checked_script_count", 0));
        lines.append("");
        lines.append("GDScript preflight:");
        lines.append("- Checked scripts: " +
                     godot::String::num_int64(checked_scripts));
        lines.append("- Errors: " + godot::String::num_int64(script_errors) +
                     ", warnings: " +
                     godot::String::num_int64(script_warnings));
        if (!script_preflight_succeeded) {
            godot::String error = script_preflight.get("error", "");
            if (!error.is_empty()) {
                lines.append("- Failure: " + error);
            }
        }
        if (script_errors > 0) {
            godot::Array diagnostics =
                script_preflight.get("diagnostics", godot::Array());
            int shown = 0;
            for (int i = 0; i < diagnostics.size() && shown < 8; i++) {
                if (diagnostics[i].get_type() != godot::Variant::DICTIONARY) {
                    continue;
                }
                godot::Dictionary diagnostic = diagnostics[i];
                if (godot::String(diagnostic.get("severity", "")) != "error") {
                    continue;
                }
                lines.append("- " +
                             godot::String(diagnostic.get("message", "")));
                shown++;
            }
        }
    }

    if (!msbuild_issues.is_empty()) {
        lines.append("");
        lines.append("MSBuild issues:");
        int count = msbuild_issues.size() < 8 ? msbuild_issues.size() : 8;
        for (int i = 0; i < count; i++) {
            if (msbuild_issues[i].get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            godot::Dictionary issue = msbuild_issues[i];
            godot::String file = issue.get("file", "");
            int line = static_cast<int>(issue.get("line", 0));
            int column = static_cast<int>(issue.get("column", 0));
            godot::String code = issue.get("code", "");
            godot::String message = issue.get("message", "");
            lines.append(
                "- " + code + ": " + message + " (" + file + ":" +
                godot::String::num_int64(line) + ":" +
                godot::String::num_int64(column) + ")");
        }
        if (msbuild_issues.size() > count) {
            lines.append(
                "- ... " +
                godot::String::num_int64(msbuild_issues.size() - count) +
                " more MSBuild issue(s) omitted");
        }
    }
    append_if_present(lines, "MSBuild issues file", raw_result, "msbuild_issues_path");
    append_if_present(lines, "MSBuild log file", raw_result, "msbuild_log_path");

    if (!launch_errors.is_empty()) {
        lines.append("");
        lines.append("Launch debugger issues:");
        int count = launch_errors.size() < 5 ? launch_errors.size() : 5;
        for (int i = 0; i < count; i++) {
            if (launch_errors[i].get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            godot::Dictionary error = launch_errors[i];
            godot::String message = error.get("message", "");
            int repeated = static_cast<int>(error.get("count", 1));
            lines.append(
                "- " + message +
                (repeated > 1
                     ? godot::String(" (repeated ") +
                           godot::String::num_int64(repeated) + " times)"
                     : godot::String()));

            godot::Array raw_lines = error.get("raw_lines", godot::Array());
            int detail_count = raw_lines.size() < 4 ? raw_lines.size() : 4;
            for (int j = 1; j < detail_count; j++) {
                lines.append("  - " + godot::String(raw_lines[j]));
            }
        }
        if (launch_errors.size() > count) {
            lines.append(
                "- ... " +
                godot::String::num_int64(launch_errors.size() - count) +
                " more launch error(s) omitted");
        }
    }

    if (status == "started" || status == "managed_running") {
        lines.append("");
        lines.append(
            "Stop this session with `runtime_session` action `stop` as soon as runtime work is complete.");
    }
    if (status == "running") {
        lines.append("");
        lines.append(
            "This is a live-session status receipt. Use runtime_script probes and the session log to verify behavior; the status line alone is not proof the scene is healthy.");
    }
    if (status == "stopped" || status == "exited") {
        lines.append("");
        lines.append(
            "This is a process/session receipt. A non-zero exit code after an intentional stop is not by itself a validation failure; use debugger issues and the session log as the health signal.");
    }
    if (status == "started_with_errors") {
        lines.append("");
        lines.append(
            "The scene entered play mode but debugger issues are already present. Inspect the session log before running runtime scripts.");
    }
    if (status == "unmanaged_running") {
        lines.append("");
        lines.append(
            "Next step: ask the user to stop the running scene in Godot, then call `runtime_session` with action `start` again.");
    }
    if (status == "managed_stale") {
        lines.append("");
        lines.append(
            "Resolve the active or stale runtime state before starting another scene.");
    }
    if (status == "idle" && availability == "free") {
        lines.append("");
        lines.append(
            "The machine-wide Runtime Slot is free. A following start still performs the authoritative atomic claim after preflight.");
    }
    if (anonymous_busy) {
        lines.append("");
        lines.append(
            "Another project is using the machine-wide Runtime Slot. Retry after the suggested delay with a small random jitter; this receipt intentionally omits owner and session details.");
    }

    godot::Dictionary metadata = make_base_metadata(
        "runtime_session",
        "runtime_session-md-v1",
        status);
    metadata["summary"] = raw_result;
    metadata["scene_running"] = raw_result.get("scene_running", false);
    metadata["session_id"] = raw_result.get("session_id", "");
    metadata["scene_path"] = raw_result.get("scene_path", "");
    metadata["playing_scene"] = raw_result.get("playing_scene", "");
    metadata["log_path"] = raw_result.get("log_path", "");
    metadata["captures_dir"] = raw_result.get("captures_dir", "");
    metadata["startup_capture"] = startup_capture;
    metadata["captures"] = raw_result.get("captures", godot::Array());
    metadata["script_running"] = raw_result.get("script_running", false);
    metadata["startup_log_wait_ms"] = raw_result.get("startup_log_wait_ms", 0);
    metadata["startup_ready_seen"] = raw_result.get("startup_ready_seen", false);
    metadata["startup_orientation_seen"] =
        raw_result.get("startup_orientation_seen", false);
    metadata["runtime_issue_count"] = raw_result.get("runtime_issue_count", 0);
    metadata["latest_runtime_issues"] = latest_runtime_issues;
    metadata["latest_runtime_summary"] = raw_result.get("latest_runtime_summary", godot::Dictionary());
    metadata["runtime_debugger_errors"] = runtime_debugger_errors;
    metadata["runtime_debugger_error_count"] =
        raw_result.get("runtime_debugger_error_count", runtime_debugger_errors.size());
    metadata["runtime_debugger_summary"] =
        raw_result.get("runtime_debugger_summary", godot::Dictionary());
    metadata["launch_errors"] = launch_errors;
    metadata["launch_error_count"] = raw_result.get("launch_error_count", launch_errors.size());
    metadata["msbuild_issues"] = msbuild_issues;
    metadata["csharp_build"] = csharp_build;
    metadata["preflight"] = preflight;
    metadata["script_preflight"] = script_preflight;
    metadata["msbuild_issue_count"] = raw_result.get("msbuild_issue_count", msbuild_issues.size());
    metadata["msbuild_issues_path"] = raw_result.get("msbuild_issues_path", "");
    metadata["msbuild_log_path"] = raw_result.get("msbuild_log_path", "");
    metadata["runtime_log"] = raw_result.get("runtime_log", godot::Dictionary());
    metadata["availability"] = availability;
    metadata["end_reason"] = raw_result.get("end_reason", "");
    metadata["code"] = raw_result.get("code", "");
    metadata["slot_acquired"] = raw_result.get("slot_acquired", false);
    metadata["retry_after_ms"] = raw_result.get("retry_after_ms", 0);
    metadata["max_run_seconds"] = raw_result.get("max_run_seconds", 0);
    metadata["absolute_deadline_ms"] =
        raw_result.get("absolute_deadline_ms", godot::Variant());
    metadata["absolute_remaining_seconds"] =
        raw_result.get("absolute_remaining_seconds", godot::Variant());
    metadata["inactivity_deadline_ms"] =
        raw_result.get("inactivity_deadline_ms", godot::Variant());
    metadata["inactivity_remaining_seconds"] =
        raw_result.get("inactivity_remaining_seconds", godot::Variant());
    metadata["heartbeat_interval_ms"] =
        raw_result.get("heartbeat_interval_ms", godot::Variant());

    godot::Dictionary envelope = make_envelope(
        godot::String("\n").join(lines),
        metadata,
        raw_result.get("success", false));
    if (raw_result.has("log_path")) {
        envelope["log_path"] = raw_result["log_path"];
    }
    if (raw_result.has("captures_dir")) {
        envelope["captures_dir"] = raw_result["captures_dir"];
    }
    return envelope;
}

} // namespace fennara::tool_results
