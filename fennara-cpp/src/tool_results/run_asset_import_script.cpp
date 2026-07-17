#include "fennara/tool_results/run_asset_import_script.hpp"

#include "fennara/tool_results/envelope.hpp"

#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara::tool_results {

namespace {

godot::String display_value(const godot::Variant &value) {
    godot::String text = value.stringify();
    if (text.length() > 240) {
        text = text.substr(0, 240) + "... [truncated]";
    }
    return text.replace("\r", " ").replace("\n", " ");
}

} // namespace

godot::Dictionary format_run_asset_import_script(
    const godot::Dictionary &raw_result) {
    const bool success = raw_result.get("success", false);
    godot::PackedStringArray lines;
    lines.append("Tool: run_asset_import_script");
    lines.append("Status: " + godot::String(success ? "success" : "failed"));
    lines.append("Asset: " + godot::String(raw_result.get("asset_path", "")));
    lines.append("Mode: " + godot::String(raw_result.get("mode", "inspect")));
    const godot::String script_path = raw_result.get("script_path", "");
    if (!script_path.is_empty()) {
        lines.append("Script: " + script_path);
    }
    const godot::String importer = raw_result.get("importer", "");
    if (!importer.is_empty()) {
        lines.append("Importer: " + importer);
    }
    lines.append("Modified: " + godot::String(
        (bool)raw_result.get("modified", false) ? "yes" : "no"));
    lines.append("Reimported: " + godot::String(
        (bool)raw_result.get("reimported", false) ? "yes" : "no"));
    if (raw_result.has("duration_ms")) {
        const double seconds =
            static_cast<double>(raw_result.get("duration_ms", 0)) / 1000.0;
        lines.append("Duration: " + godot::String::num(seconds, 3) + " seconds");
    }
    if (raw_result.has("error")) {
        lines.append("Error: " + godot::String(raw_result.get("error", "")));
    }
    if (raw_result.has("note")) {
        lines.append("Note: " + godot::String(raw_result.get("note", "")));
    }
    const bool recovery_attempted =
        raw_result.get("recovery_attempted", false);
    const godot::Dictionary recovery =
        raw_result.get("recovery", godot::Dictionary());
    if (recovery_attempted) {
        const bool recovery_success = recovery.get("success", false);
        lines.append("Recovery: " + godot::String(
            recovery_success ? "succeeded" : "failed"));
        const godot::String recovery_error = raw_result.get(
            "recovery_error", recovery.get("error", ""));
        if (!recovery_error.is_empty()) {
            lines.append("Recovery error: " + recovery_error);
        }
    }

    const godot::String diagnostic_mode =
        raw_result.get("diagnostic_mode", "");
    const bool diagnostic_success =
        raw_result.get("diagnostic_success", false);
    const int64_t total_errors = raw_result.get("total_errors", 0);
    const int64_t total_warnings = raw_result.get("total_warnings", 0);
    if (!diagnostic_mode.is_empty()) {
        lines.append("Diagnostics: " + diagnostic_mode + ", " +
                     godot::String::num_int64(total_errors) + " errors, " +
                     godot::String::num_int64(total_warnings) + " warnings");
    }

    godot::Array diagnostics =
        raw_result.get("script_diagnostics", godot::Array());
    if (!diagnostics.is_empty()) {
        lines.append("");
        lines.append("## Script diagnostics");
        const int shown = diagnostics.size() < 50 ? diagnostics.size() : 50;
        for (int i = 0; i < shown; i++) {
            if (diagnostics[i].get_type() != godot::Variant::DICTIONARY) {
                lines.append("- " + display_value(diagnostics[i]));
                continue;
            }
            godot::Dictionary diagnostic = diagnostics[i];
            lines.append(
                "- line " + godot::String::num_int64(
                    static_cast<int64_t>(diagnostic.get("line", 0))) +
                ":" + godot::String::num_int64(
                    static_cast<int64_t>(diagnostic.get("column", 0))) +
                " " + godot::String(diagnostic.get("severity", "")) +
                ": " + godot::String(diagnostic.get("message", "")));
        }
        if (diagnostics.size() > shown) {
            lines.append("- ... " +
                         godot::String::num_int64(diagnostics.size() - shown) +
                         " additional diagnostics omitted");
        }
    }

    godot::Array changes = raw_result.get("changes", godot::Array());
    if (!changes.is_empty()) {
        lines.append("");
        lines.append("## Import option changes");
        const int shown = changes.size() < 50 ? changes.size() : 50;
        for (int i = 0; i < shown; i++) {
            godot::Dictionary change = changes[i];
            lines.append("- " + godot::String(change.get("name", "")) +
                         ": " + display_value(change.get("before", godot::Variant())) +
                         " -> " + display_value(change.get("after", godot::Variant())));
        }
        if (changes.size() > shown) {
            lines.append("- ... " +
                         godot::String::num_int64(changes.size() - shown) +
                         " additional changes omitted");
        }
    }

    godot::Array logs = raw_result.get("logs", godot::Array());
    if (!logs.is_empty()) {
        lines.append("");
        lines.append("## Script logs");
        const int shown = logs.size() < 100 ? logs.size() : 100;
        for (int i = 0; i < shown; i++) {
            lines.append("- " + display_value(logs[i]));
        }
        if (logs.size() > shown) {
            lines.append("- ... " + godot::String::num_int64(logs.size() - shown) +
                         " additional logs omitted");
        }
    }

    godot::Array errors = raw_result.get("runtime_errors", godot::Array());
    if (!errors.is_empty()) {
        lines.append("");
        lines.append("## Runtime errors");
        const int shown = errors.size() < 50 ? errors.size() : 50;
        for (int i = 0; i < shown; i++) {
            godot::Dictionary error = errors[i];
            lines.append("- " + godot::String(error.get("message", "Unknown error")));
        }
    }

    godot::Dictionary metadata;
    metadata["asset_path"] = raw_result.get("asset_path", "");
    metadata["script_path"] = raw_result.get("script_path", "");
    metadata["mode"] = raw_result.get("mode", "inspect");
    metadata["importer"] = raw_result.get("importer", "");
    metadata["modified"] = raw_result.get("modified", false);
    metadata["reimported"] = raw_result.get("reimported", false);
    metadata["recovery_attempted"] = recovery_attempted;
    metadata["recovery_success"] = recovery.get("success", false);
    metadata["recovery_error"] = raw_result.get(
        "recovery_error", recovery.get("error", ""));
    metadata["duration_ms"] = raw_result.get("duration_ms", 0);
    metadata["change_count"] = changes.size();
    metadata["log_count"] = logs.size();
    metadata["runtime_error_count"] = errors.size();
    metadata["diagnostic_success"] = diagnostic_success;
    metadata["diagnostic_mode"] = diagnostic_mode;
    metadata["diagnostic_fallback"] =
        raw_result.get("diagnostic_fallback", "");
    metadata["total_errors"] = total_errors;
    metadata["total_warnings"] = total_warnings;
    metadata["diagnostic_count"] = diagnostics.size();
    metadata["format_version"] = "run-asset-import-script-md-v1";
    return make_envelope(godot::String("\n").join(lines), metadata, success);
}

} // namespace fennara::tool_results
