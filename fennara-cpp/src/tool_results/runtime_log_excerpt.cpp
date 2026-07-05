#include "fennara/tool_results/runtime_log_excerpt.hpp"

#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara::tool_results {
namespace {

godot::String count_text(int count) {
    return godot::String::num_int64(count) +
           (count == 1 ? " line" : " lines");
}

godot::String shown_ranges_text(const godot::Array &ranges) {
    godot::PackedStringArray parts;
    for (int i = 0; i < ranges.size(); i++) {
        godot::Dictionary range = ranges[i];
        int first = static_cast<int>(range.get("first", 0));
        int last = static_cast<int>(range.get("last", 0));
        if (first <= 0 || last < first) {
            continue;
        }
        if (first == last) {
            parts.append(godot::String::num_int64(first));
        } else {
            parts.append(godot::String::num_int64(first) + "-" +
                         godot::String::num_int64(last));
        }
    }
    return godot::String(", ").join(parts);
}

} // namespace

void append_runtime_log_excerpt(godot::PackedStringArray &lines,
                                const godot::Dictionary &raw_result) {
    godot::Dictionary runtime_log =
        raw_result.get("runtime_log", godot::Dictionary());
    if (runtime_log.is_empty()) {
        return;
    }

    lines.append("");
    lines.append("Runtime log update:");
    if (!(bool)runtime_log.get("available", false)) {
        godot::String error = runtime_log.get("error", "");
        if (error.is_empty()) {
            lines.append("- Log file was not readable yet for this receipt.");
        } else {
            lines.append("- Log file was not readable yet for this receipt: " + error);
        }
        return;
    }

    if ((bool)runtime_log.get("log_reset", false)) {
        lines.append("- Log cursor reset because the log appeared shorter than before.");
    }

    int added = static_cast<int>(runtime_log.get("lines_added", 0));
    int first_line = static_cast<int>(runtime_log.get("first_line", 0));
    int last_line = static_cast<int>(runtime_log.get("last_line", 0));
    int omitted = static_cast<int>(runtime_log.get("omitted_line_count", 0));
    int truncated = static_cast<int>(runtime_log.get("truncated_line_count", 0));
    int shown_first_line = static_cast<int>(runtime_log.get("shown_first_line", 0));
    int shown_last_line = static_cast<int>(runtime_log.get("shown_last_line", 0));
    if (added <= 0) {
        lines.append("- No new runtime log lines since the previous runtime receipt.");
    } else {
        godot::String range;
        if (first_line > 0 && last_line >= first_line) {
            range = godot::String(" (log lines ") +
                    godot::String::num_int64(first_line) + "-" +
                    godot::String::num_int64(last_line) + ")";
        }
        lines.append("- New runtime log lines since previous receipt: " +
                     count_text(added) + range);
        godot::String shown_ranges =
            shown_ranges_text(runtime_log.get("shown_ranges", godot::Array()));
        if (!shown_ranges.is_empty()) {
            lines.append("- Showing log lines " + shown_ranges + ".");
        } else if (shown_first_line > 0 && shown_last_line >= shown_first_line) {
            lines.append("- Showing log lines " +
                         godot::String::num_int64(shown_first_line) + "-" +
                         godot::String::num_int64(shown_last_line) + ".");
        }
        lines.append("```text");
        godot::Array new_lines = runtime_log.get("lines", godot::Array());
        for (int i = 0; i < new_lines.size(); i++) {
            lines.append(godot::String(new_lines[i]));
        }
        lines.append("```");
        if (omitted > 0) {
            lines.append("- " + count_text(omitted) +
                         " omitted from this receipt; read the full log file for the complete text.");
        }
        if (truncated > 0) {
            lines.append("- " + count_text(truncated) +
                         " truncated in this receipt; read the full log file for complete lines.");
        }
    }

    int cursor_after = static_cast<int>(runtime_log.get("cursor_after_line", -1));
    if (cursor_after >= 0) {
        lines.append("Runtime log cursor is now at line " +
                     godot::String::num_int64(cursor_after) + ".");
    }
}

} // namespace fennara::tool_results
