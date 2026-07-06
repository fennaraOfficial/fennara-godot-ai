#include "fennara/tool_results/validate_scene.hpp"

#include "fennara/tool_results/envelope.hpp"
#include "fennara/tool_results/markdown.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara::tool_results {

namespace {

int validate_scene_budget_tokens(int target_count) {
    if (target_count <= 1) return 10000;
    if (target_count == 2) return 14000;
    if (target_count == 3) return 18000;
    if (target_count == 4) return 22000;
    return 26000;
}

godot::String scene_label(const godot::Dictionary &scene, int index) {
    godot::String path = scene.get("scene_path", "");
    if (!path.is_empty()) {
        return path;
    }
    return "scene_paths[" + godot::String::num_int64(index) + "]";
}

godot::String scope_for_scenes(const godot::Array &scenes) {
    godot::PackedStringArray paths;
    for (int i = 0; i < scenes.size(); i++) {
        if (scenes[i].get_type() != godot::Variant::DICTIONARY) {
            paths.append("scene_paths[" + godot::String::num_int64(i) + "]");
            continue;
        }
        godot::Dictionary scene = scenes[i];
        paths.append(scene_label(scene, i));
    }
    return godot::String::num_int64(scenes.size()) +
           (scenes.size() == 1 ? " scene: " : " scenes: ") +
           godot::String(", ").join(paths);
}

godot::Dictionary target_metadata(const godot::Dictionary &scene) {
    godot::Dictionary target;
    target["scene_path"] = scene.get("scene_path", "");
    target["status"] = scene.get("status", "");
    target["checks_run"] = scene.get("checks_run", 0);
    target["total_issues"] = scene.get("total_issues", 0);
    target["errors"] = scene.get("errors", 0);
    target["warnings"] = scene.get("warnings", 0);
    target["notes"] = scene.get("notes", 0);
    target["shown_issues"] = 0;
    target["omitted_issues"] =
        static_cast<int>(scene.get("total_issues", 0)) +
        static_cast<int>(scene.get("notes", 0));
    if (scene.has("error")) {
        target["error"] = scene["error"];
    }
    if (scene.has("runtime_check")) {
        target["runtime_check"] = scene["runtime_check"];
    }
    return target;
}

bool is_unset_export_issue(const godot::Dictionary &issue) {
    return godot::String(issue.get("check", "")) == "unset_export_var";
}

godot::String count_label(int count, const godot::String &singular,
                          const godot::String &plural) {
    return godot::String::num_int64(count) + " " +
           (count == 1 ? singular : plural);
}

godot::String join_array_strings(const godot::Array &values) {
    godot::PackedStringArray parts;
    for (int i = 0; i < values.size(); i++) {
        parts.append(values[i]);
    }
    return godot::String(", ").join(parts);
}

void add_unique_unset_property(godot::Dictionary &group,
                               const godot::String &name,
                               const godot::String &type) {
    if (name.is_empty()) {
        return;
    }
    godot::String key = name + godot::String("\n") + type;
    godot::Dictionary seen = group.get("property_seen", godot::Dictionary());
    if (seen.has(key)) {
        return;
    }
    seen[key] = true;
    godot::Array properties = group.get("properties", godot::Array());
    godot::Dictionary property;
    property["name"] = name;
    property["type"] = type;
    properties.append(property);
    group["properties"] = properties;
    group["property_seen"] = seen;
}

void add_unset_issue_properties(godot::Dictionary &group,
                                const godot::Dictionary &issue) {
    godot::Variant props_var = issue.get("properties", godot::Variant());
    if (props_var.get_type() == godot::Variant::ARRAY) {
        godot::Array props = props_var;
        for (int i = 0; i < props.size(); i++) {
            if (props[i].get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            godot::Dictionary prop = props[i];
            add_unique_unset_property(group, prop.get("name", ""),
                                      prop.get("type", ""));
        }
        return;
    }

    add_unique_unset_property(group, issue.get("property", ""),
                              issue.get("type", ""));
}

godot::String format_unset_properties(const godot::Array &properties) {
    godot::PackedStringArray parts;
    for (int i = 0; i < properties.size(); i++) {
        if (properties[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary prop = properties[i];
        godot::String name = prop.get("name", "");
        godot::String type = prop.get("type", "");
        if (name.is_empty()) {
            continue;
        }
        parts.append(type.is_empty() ? name : name + " (" + type + ")");
    }
    return godot::String(", ").join(parts);
}

void add_unset_node_sample(godot::Dictionary &scene_group,
                           const godot::String &node_path) {
    if (node_path.is_empty()) {
        return;
    }
    godot::Dictionary seen = scene_group.get("sample_seen", godot::Dictionary());
    if (seen.has(node_path)) {
        return;
    }
    seen[node_path] = true;
    godot::Array samples = scene_group.get("samples", godot::Array());
    if (samples.size() < 5) {
        samples.append(node_path);
        scene_group["samples"] = samples;
    }
    scene_group["sample_seen"] = seen;
}

void add_unset_issue_scene(godot::Dictionary &group,
                           const godot::String &scene_path,
                           const godot::Dictionary &issue) {
    godot::Dictionary scenes = group.get("scenes", godot::Dictionary());
    godot::Array scene_order = group.get("scene_order", godot::Array());
    godot::Dictionary scene_group;
    if (scenes.has(scene_path)) {
        scene_group = scenes[scene_path];
    } else {
        scene_group["count"] = 0;
        scene_group["samples"] = godot::Array();
        scene_group["sample_seen"] = godot::Dictionary();
        scene_order.append(scene_path);
    }

    int count = static_cast<int>(
        scene_group.get("count", 0));
    int issue_node_count = static_cast<int>(
        issue.get("node_count", issue.get("unset_count", 1)));
    scene_group["count"] = count + issue_node_count;

    godot::Variant samples_var = issue.get("samples", godot::Variant());
    if (samples_var.get_type() == godot::Variant::ARRAY) {
        godot::Array samples = samples_var;
        for (int i = 0; i < samples.size(); i++) {
            add_unset_node_sample(scene_group, samples[i]);
        }
    } else {
        add_unset_node_sample(
            scene_group,
            issue.get("node_path", issue.get("node", "")));
    }

    scenes[scene_path] = scene_group;
    group["scenes"] = scenes;
    group["scene_order"] = scene_order;
}

godot::String format_unset_scene_samples(const godot::Dictionary &group) {
    godot::Array scene_order = group.get("scene_order", godot::Array());
    godot::Dictionary scenes = group.get("scenes", godot::Dictionary());
    godot::PackedStringArray parts;
    int scene_limit = 5;
    int shown_scenes = scene_order.size() < scene_limit
        ? scene_order.size()
        : scene_limit;
    for (int i = 0; i < shown_scenes; i++) {
        godot::String scene_path = scene_order[i];
        godot::Dictionary scene_group = scenes[scene_path];
        int count = static_cast<int>(scene_group.get("count", 0));
        godot::Array samples = scene_group.get("samples", godot::Array());
        godot::String text = scene_path + ": ";
        if (samples.is_empty()) {
            text += count_label(count, "node", "nodes");
        } else {
            text += join_array_strings(samples);
            int omitted = count - samples.size();
            if (omitted > 0) {
                text += ", " + count_label(omitted, "omitted", "omitted");
            }
        }
        parts.append(text);
    }
    int omitted_scenes = scene_order.size() - shown_scenes;
    if (omitted_scenes > 0) {
        parts.append(count_label(omitted_scenes, "scene omitted",
                                 "scenes omitted"));
    }
    return godot::String("; ").join(parts);
}

godot::String global_unset_export_section(
    const godot::Array &scenes,
    godot::Dictionary &scene_unset_counts) {
    godot::Dictionary groups;
    godot::Array group_order;

    for (int i = 0; i < scenes.size(); i++) {
        if (scenes[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary scene = scenes[i];
        godot::String scene_path = scene_label(scene, i);
        godot::Array issues = scene.get("issues", godot::Array());
        for (int issue_index = 0; issue_index < issues.size(); issue_index++) {
            if (issues[issue_index].get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            godot::Dictionary issue = issues[issue_index];
            if (!is_unset_export_issue(issue)) {
                continue;
            }

            int scene_count = static_cast<int>(
                scene_unset_counts.get(scene_path, 0));
            scene_unset_counts[scene_path] = scene_count + 1;

            godot::String script_path = issue.get("script_path", "");
            godot::String key =
                script_path.is_empty() ? godot::String(issue.get("message", ""))
                                       : script_path;
            godot::Dictionary group;
            if (groups.has(key)) {
                group = groups[key];
            } else {
                group["script_path"] = script_path;
                group["properties"] = godot::Array();
                group["property_seen"] = godot::Dictionary();
                group["node_count"] = 0;
                group["scenes"] = godot::Dictionary();
                group["scene_order"] = godot::Array();
                group_order.append(key);
            }

            add_unset_issue_properties(group, issue);
            int node_count = static_cast<int>(group.get("node_count", 0));
            group["node_count"] = node_count +
                static_cast<int>(issue.get("node_count", issue.get("unset_count", 1)));
            add_unset_issue_scene(group, scene_path, issue);
            groups[key] = group;
        }
    }

    if (group_order.is_empty()) {
        return "";
    }

    godot::PackedStringArray lines;
    lines.append("## Unset export notes");
    for (int i = 0; i < group_order.size(); i++) {
        godot::String key = group_order[i];
        godot::Dictionary group = groups[key];
        godot::String script_path = group.get("script_path", "");
        godot::Array properties = group.get("properties", godot::Array());
        int node_count = static_cast<int>(group.get("node_count", 0));
        godot::String prop_text = format_unset_properties(properties);

        godot::String bullet = "- info (unset_export_var) ";
        bullet += script_path.is_empty()
            ? godot::String("Script <unknown>")
            : godot::String("Script ") + script_path;
        bullet += ": " +
            count_label(properties.size(), "unset exported Object/Resource var",
                        "unset exported Object/Resource vars");
        if (!prop_text.is_empty()) {
            bullet += ": " + prop_text;
        }
        bullet += " on " + count_label(node_count, "node", "nodes") + ". ";
        bullet += "Ignore this note if these references are intentionally optional or assigned at runtime.";
        godot::String node_text = format_unset_scene_samples(group);
        if (!node_text.is_empty()) {
            bullet += " Nodes: " + node_text;
        }
        lines.append(bullet);
    }

    return godot::String("\n").join(lines);
}

godot::String issue_extra_text(const godot::Dictionary &issue) {
    static const char *skip_keys[] = {
        "node", "node_path", "check", "severity", "message"
    };
    static const char *unset_export_skip_keys[] = {
        "property", "type", "script_path", "unset_count", "instance_scene",
        "properties", "node_count", "samples", "instance_scenes"
    };
    bool is_unset_export = is_unset_export_issue(issue);

    godot::PackedStringArray parts;
    godot::Array keys = issue.keys();
    for (int i = 0; i < keys.size(); i++) {
        godot::String key = keys[i];
        bool skip = false;
        for (const char *skip_key : skip_keys) {
            if (key == skip_key) {
                skip = true;
                break;
            }
        }
        if (!skip && is_unset_export) {
            for (const char *skip_key : unset_export_skip_keys) {
                if (key == skip_key) {
                    skip = true;
                    break;
                }
            }
        }
        if (!skip) {
            parts.append(key + "=" + godot::String(issue[key]));
        }
    }
    if (parts.is_empty()) {
        return "";
    }
    return " (" + godot::String(", ").join(parts) + ")";
}

godot::String issue_bullet(const godot::Dictionary &issue) {
    godot::String severity = issue.get("severity", "");
    godot::String check = issue.get("check", "");
    godot::String node = issue.get("node_path", issue.get("node", ""));
    godot::String message = issue.get("message", "");

    godot::String out = "- " + severity;
    if (!check.is_empty()) {
        out += " (" + check + ")";
    }
    if (!node.is_empty()) {
        out += " " + node + ":";
    }
    out += " " + message;
    out += issue_extra_text(issue);
    return out;
}

godot::Array issues_for_severity(const godot::Dictionary &scene,
                                 const godot::String &severity,
                                 bool include_unset_exports) {
    godot::Array out;
    godot::Array issues = scene.get("issues", godot::Array());
    for (int i = 0; i < issues.size(); i++) {
        if (issues[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary issue = issues[i];
        if (!include_unset_exports && is_unset_export_issue(issue)) {
            continue;
        }
        if (godot::String(issue.get("severity", "")) == severity) {
            out.append(issue);
        }
    }
    return out;
}

godot::String severity_heading(const godot::String &severity) {
    if (severity == "error") {
        return "### Structural errors";
    }
    if (severity == "warning") {
        return "### Structural warnings";
    }
    return "### Structural notes";
}

godot::String failed_section(const godot::Dictionary &scene, int index) {
    godot::PackedStringArray lines;
    lines.append("## " + scene_label(scene, index));
    lines.append("Status: failed");
    if (scene.has("error")) {
        lines.append("Error:\n" + godot::String(scene.get("error", "")));
    }
    return godot::String("\n").join(lines);
}

godot::String read_text_file(const godot::String &path) {
    if (path.is_empty()) {
        return "";
    }
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(path, godot::FileAccess::READ);
    if (file.is_null()) {
        return "";
    }
    godot::String text = file->get_as_text();
    file->close();
    return text;
}

bool is_runtime_metadata_line(const godot::String &line) {
    return line.begins_with("# Fennara daemon") ||
           line.begins_with("Scene: ") ||
           line.begins_with("Executable: ") ||
           line.begins_with("Working directory: ") ||
           line.begins_with("Args: ") ||
           line == "## Fennara daemon process result" ||
           line.begins_with("Status: ") ||
           line.begins_with("Exit code: ") ||
           line.begins_with("Duration: ") ||
           line.begins_with("Godot Engine ") ||
           line.begins_with("Vulkan ") ||
           line.begins_with("OpenGL ");
}

bool starts_runtime_issue_block(const godot::String &line) {
    return line.begins_with("WARNING:") ||
           line.begins_with("ERROR:") ||
           line.begins_with("SCRIPT ERROR:");
}

bool is_runtime_issue_continuation(const godot::String &line) {
    return line.begins_with("   at:") ||
           line.begins_with("          at:") ||
           line.begins_with("   GDScript backtrace") ||
           line.begins_with("       [") ||
           line.begins_with("          GDScript backtrace") ||
           line.begins_with("              [");
}

godot::String bool_text(bool value) {
    return value ? "yes" : "no";
}

godot::String runtime_status_label(const godot::Dictionary &runtime) {
    godot::String status = runtime.get("status", "");
    if (status == "stopped_after_run_seconds") {
        return "stopped after 3s validation window";
    }
    if (status == "cancelled") {
        return "cancelled";
    }
    if (status == "timeout") {
        return "timed out";
    }
    if (status == "crashed") {
        return "crashed";
    }
    if (status == "failed") {
        return "failed";
    }
    if (status.is_empty() && (bool)runtime.get("killed", false)) {
        return "stopped by Fennara";
    }
    return status.is_empty() ? godot::String("unknown") : status.replace("_", " ");
}

godot::String exit_code_note(const godot::Dictionary &runtime) {
    godot::String status = runtime.get("status", "");
    if (status == "stopped_after_run_seconds" ||
        (bool)runtime.get("killed", false)) {
        return " (process was stopped by Fennara after the validation window; this is not by itself a failure)";
    }
    return "";
}

godot::String block_text(const godot::PackedStringArray &block) {
    return godot::String("\n").join(block);
}

godot::String compact_runtime_log_for_model(const godot::String &raw) {
    godot::PackedStringArray lines = raw.split("\n");
    godot::PackedStringArray out;
    godot::PackedStringArray previous_block;
    int repeat_count = 0;
    bool in_native_backtrace = false;
    int native_frames = 0;
    const int max_blocks = 120;

    auto flush_repeat = [&]() {
        if (previous_block.is_empty()) {
            return;
        }
        godot::String text = block_text(previous_block);
        if (repeat_count > 1 && starts_runtime_issue_block(previous_block[0])) {
            out.append(text + "\n[repeated " +
                       godot::String::num_int64(repeat_count) + "x]");
        } else {
            for (int i = 0; i < repeat_count; i++) {
                out.append(text);
            }
        }
        previous_block.clear();
        repeat_count = 0;
    };

    for (int i = 0; i < lines.size();) {
        godot::String line = lines[i].rstrip("\r");
        if (line.strip_edges().is_empty() || is_runtime_metadata_line(line)) {
            i++;
            continue;
        }
        if (line.begins_with("[") &&
            line.contains("no debug info in PE/COFF executable")) {
            in_native_backtrace = true;
            native_frames++;
            i++;
            continue;
        }
        if (in_native_backtrace) {
            if (line.contains("-- END OF C++ BACKTRACE --")) {
                godot::PackedStringArray block;
                block.append("[native backtrace omitted: " +
                             godot::String::num_int64(native_frames) +
                             " frames without symbols]");
                if (block_text(block) == block_text(previous_block)) {
                    repeat_count++;
                } else {
                    flush_repeat();
                    previous_block = block;
                    repeat_count = 1;
                }
                in_native_backtrace = false;
                native_frames = 0;
            }
            i++;
            continue;
        }

        godot::PackedStringArray block;
        if (line.length() > 1000) {
            line = line.substr(0, 1000) +
                   " ... [line truncated; full output in raw log]";
        }
        block.append(line);
        i++;

        if (starts_runtime_issue_block(line)) {
            while (i < lines.size()) {
                godot::String next = lines[i].rstrip("\r");
                if (starts_runtime_issue_block(next) ||
                    !is_runtime_issue_continuation(next)) {
                    break;
                }
                block.append(next);
                i++;
            }
        }

        if (block_text(block) == block_text(previous_block)) {
            repeat_count++;
            continue;
        }
        flush_repeat();
        previous_block = block;
        repeat_count = 1;
        if (out.size() >= max_blocks) {
            out.append("[runtime output truncated; full output in raw log]");
            break;
        }
    }
    flush_repeat();
    return godot::String("\n").join(out);
}

} // namespace

godot::Dictionary format_validate_scene(const godot::Dictionary &raw_result) {
    bool raw_success = raw_result.get("success", false);
    godot::Array scenes = raw_result.get("scenes", godot::Array());
    godot::Dictionary summary = raw_result.get("summary", godot::Dictionary());
    godot::Dictionary runtime_batch = raw_result.get("runtime_batch", godot::Dictionary());
    int budget_tokens = validate_scene_budget_tokens(scenes.size());

    godot::Array targets;
    godot::PackedStringArray sections;
    int raw_success_count = 0;
    int raw_failure_count = 0;
    bool previewed = false;

    godot::PackedStringArray header;
    header.append("Tool: validate_scene");
    header.append("Status: pending");
    header.append(scenes.size() > 0 ? "Scope: " + scope_for_scenes(scenes) : "Scope: unknown");
    if (!summary.is_empty()) {
        godot::String totals_line =
            "Totals: " +
            godot::String::num_int64(static_cast<int64_t>(summary.get("success_count", 0))) +
            " succeeded, " +
            godot::String::num_int64(static_cast<int64_t>(summary.get("failure_count", 0))) +
            " failed, " +
            godot::String::num_int64(static_cast<int64_t>(summary.get("total_issues", 0))) +
            " issues (" +
            godot::String::num_int64(static_cast<int64_t>(summary.get("errors", 0))) +
            " errors, " +
            godot::String::num_int64(static_cast<int64_t>(summary.get("warnings", 0))) +
            " warnings)";
        int64_t note_total =
            static_cast<int64_t>(summary.get("notes", 0));
        if (note_total > 0) {
            totals_line += ", " + godot::String::num_int64(note_total) + " notes";
        }
        header.append(totals_line);
        if (summary.has("runtime_checked_count")) {
            godot::String runtime_line = "Runtime: ";
            if ((bool)summary.get("runtime_skipped", false)) {
                runtime_line += "skipped";
            } else {
                runtime_line += godot::String(summary.get("runtime_status", "unknown")) +
                    ", ran " +
                    godot::String::num_int64(static_cast<int64_t>(summary.get("runtime_checked_count", 0))) +
                    " scenes headlessly for 3s each";
                int64_t runtime_crashes =
                    static_cast<int64_t>(summary.get("runtime_crash_count", 0));
                int64_t runtime_errors =
                    static_cast<int64_t>(summary.get("runtime_error_count", 0));
                int64_t runtime_warnings =
                    static_cast<int64_t>(summary.get("runtime_warning_count", 0));
                runtime_line += " (" +
                    godot::String::num_int64(runtime_crashes) + " crashes, " +
                    godot::String::num_int64(runtime_errors) + " errors, " +
                    godot::String::num_int64(runtime_warnings) + " warnings)";
            }
            header.append(runtime_line);
        }
    }
    if (scenes.size() == 0 && raw_result.has("error")) {
        header.append("");
        header.append("Error:\n" + godot::String(raw_result.get("error", "")));
    }

    int used_tokens = estimate_tokens(godot::String("\n").join(header));
    int remaining_tokens = budget_tokens - used_tokens;
    int per_scene_budget = scenes.size() > 0 ? remaining_tokens / scenes.size() : remaining_tokens;
    if (per_scene_budget < 1) {
        per_scene_budget = 1;
    }

    godot::Dictionary scene_unset_export_counts;
    godot::String unset_export_section =
        global_unset_export_section(scenes, scene_unset_export_counts);
    bool summarized_unset_exports = !unset_export_section.is_empty();
    if (summarized_unset_exports) {
        sections.append(unset_export_section);
    }

    for (int i = 0; i < scenes.size(); i++) {
        if (scenes[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary scene = scenes[i];
        godot::String scene_status = scene.get("status", "");
        if (scene_status == "success") {
            raw_success_count++;
        } else {
            raw_failure_count++;
        }

        int target_index = targets.size();
        targets.append(target_metadata(scene));

        if (scene_status != "success") {
            sections.append(failed_section(scene, i));
            if (targets[target_index].get_type() == godot::Variant::DICTIONARY) {
                godot::Dictionary target = targets[target_index];
                target["shown_issues"] = 0;
                target["omitted_issues"] = 0;
                targets[target_index] = target;
            }
            continue;
        }

        godot::PackedStringArray lines;
        lines.append("## " + scene_label(scene, i));
        lines.append("Status: success");
        lines.append(
            "Checks: " + godot::String::num_int64(static_cast<int64_t>(scene.get("checks_run", 0)))
        );
        lines.append(
            "Structural issues: " + godot::String::num_int64(static_cast<int64_t>(scene.get("total_issues", 0))) +
            " (" + godot::String::num_int64(static_cast<int64_t>(scene.get("errors", 0))) +
            " errors, " + godot::String::num_int64(static_cast<int64_t>(scene.get("warnings", 0))) +
            " warnings)"
        );
        int note_count = static_cast<int>(scene.get("notes", 0));
        if (note_count > 0) {
            lines.append(
                "Structural notes: " +
                godot::String::num_int64(static_cast<int64_t>(note_count))
            );
        }

        int detail_budget = per_scene_budget - estimate_tokens(godot::String("\n").join(lines));
        if (detail_budget < 1) {
            detail_budget = 1;
        }

        int shown = summarized_unset_exports
            ? static_cast<int>(scene_unset_export_counts.get(scene_label(scene, i), 0))
            : 0;
        for (int severity_index = 0; severity_index < 3; severity_index++) {
            godot::String severity = severity_index == 0 ? "error" :
                (severity_index == 1 ? "warning" : "info");
            godot::Array issues =
                issues_for_severity(scene, severity, !summarized_unset_exports);
            if (issues.is_empty()) {
                continue;
            }

            godot::PackedStringArray bullets;
            for (int issue_index = 0; issue_index < issues.size(); issue_index++) {
                godot::Dictionary issue = issues[issue_index];
                godot::String bullet = issue_bullet(issue);
                int tokens = estimate_tokens(bullet);
                if (detail_budget - tokens < 0) {
                    previewed = true;
                    continue;
                }
                bullets.append(bullet);
                detail_budget -= tokens;
                shown++;
            }
            if (!bullets.is_empty()) {
                lines.append("");
                lines.append(severity_heading(severity));
                lines.append(godot::String("\n").join(bullets));
            }
        }

        int total_issues = static_cast<int>(scene.get("total_issues", 0));
        int total_reported_items = total_issues + note_count;
        if (shown < total_reported_items) {
            previewed = true;
            lines.append("");
            lines.append("Omitted: additional validation issues or notes exceeded model-facing size limit.");
        }

        godot::Dictionary target = targets[target_index];
        target["shown_issues"] = shown;
        target["omitted_issues"] =
            total_reported_items > shown ? total_reported_items - shown : 0;
        targets[target_index] = target;

        if (scene.has("runtime_check")) {
            godot::Variant runtime_var = scene["runtime_check"];
            lines.append("");
            lines.append("### 3s headless runtime check");
            if (runtime_var.get_type() == godot::Variant::DICTIONARY) {
                godot::Dictionary runtime = runtime_var;
                lines.append("Runtime status: " + runtime_status_label(runtime));
                lines.append("Crash detected: " +
                             bool_text((bool)runtime.get("crashed", false)));
                lines.append("Runtime errors detected: " +
                             bool_text((bool)runtime.get("has_error", false)));
                lines.append("Runtime warnings detected: " +
                             bool_text((bool)runtime.get("has_warning", false)));
                lines.append("Process exit code: " +
                             godot::String::num_int64(static_cast<int>(
                                 runtime.get("exit_code", 0))) +
                             exit_code_note(runtime));
                lines.append("Observed duration: " +
                             godot::String::num(static_cast<double>(
                                 runtime.get("duration_seconds", 0.0)), 3) +
                             "s");
                godot::String raw_log = runtime.get("raw_log_path", "");
                godot::String compacted =
                    compact_runtime_log_for_model(read_text_file(raw_log));
                if (compacted.strip_edges().is_empty()) {
                    lines.append(
                        "During this brief 3s headless run, Fennara captured no runtime errors, warnings, or crash output.");
                } else {
                    lines.append("Captured output from the 3s headless run:");
                    lines.append(compacted);
                }
            } else if (godot::String(runtime_var) == "skipped") {
                lines.append("Skipped: " +
                             godot::String(scene.get("runtime_skip_reason", "")));
            } else if (godot::String(runtime_var) == "failed") {
                lines.append("Failed: " +
                             godot::String(scene.get("runtime_error", "")));
            }
        }
        sections.append(godot::String("\n").join(lines));
    }

    godot::PackedStringArray saved_lines;
    godot::String artifact_dir = summary.get("artifact_dir", "");
    godot::String artifact_abs = summary.get("artifact_absolute_dir", "");
    godot::String result_json = summary.get("result_json_path", "");
    godot::String result_json_abs = summary.get("result_json_absolute_path", "");
    godot::String raw_logs = summary.get("runtime_raw_logs_dir", "");
    godot::String raw_logs_abs = summary.get("runtime_raw_logs_absolute_dir", "");
    if (!artifact_dir.is_empty() || !result_json.is_empty() || !raw_logs.is_empty()) {
        saved_lines.append("## Saved result/logs");
        if (!artifact_dir.is_empty()) {
            saved_lines.append("- Result artifacts: " + artifact_dir);
        }
        if (!artifact_abs.is_empty()) {
            saved_lines.append("- Result artifacts absolute: " + artifact_abs);
        }
        if (!result_json.is_empty()) {
            saved_lines.append("- Full raw result JSON: " + result_json);
        }
        if (!result_json_abs.is_empty()) {
            saved_lines.append("- Full raw result JSON absolute: " + result_json_abs);
        }
        if (!raw_logs.is_empty()) {
            saved_lines.append("- Runtime raw logs: " + raw_logs);
        }
        if (!raw_logs_abs.is_empty()) {
            saved_lines.append("- Runtime raw logs absolute: " + raw_logs_abs);
        }
        sections.append(godot::String("\n").join(saved_lines));
    }

    godot::String status = summary.get("status", "");
    if (status.is_empty()) {
        status = "success";
    }
    if (scenes.size() == 0 && raw_result.has("error")) {
        status = "failed";
    } else if (status == "success") {
        if (raw_failure_count > 0 && raw_success_count == 0) {
            status = "failed";
        } else if (raw_failure_count > 0 || previewed) {
            status = "partial";
        }
    }
    header.set(1, "Status: " + status);
    sections.insert(0, godot::String("\n").join(header));

    godot::Dictionary metadata = make_base_metadata("validate_scene", "validate_scene-md-v1", status);
    metadata["targets"] = targets;
    metadata["budget_tokens"] = validate_scene_budget_tokens(scenes.size());
    metadata["previewed"] = previewed;
    if (summary.has("runtime_compacted_log_path")) {
        metadata["runtime_compacted_log_path"] = summary.get("runtime_compacted_log_path", "");
        metadata["runtime_compacted_log_absolute_path"] =
            summary.get("runtime_compacted_log_absolute_path", "");
        metadata["runtime_results_path"] = summary.get("runtime_results_path", "");
        metadata["runtime_results_absolute_path"] =
            summary.get("runtime_results_absolute_path", "");
        metadata["runtime_raw_logs_dir"] = summary.get("runtime_raw_logs_dir", "");
        metadata["runtime_raw_logs_absolute_dir"] =
            summary.get("runtime_raw_logs_absolute_dir", "");
    }
    metadata["artifact_dir"] = summary.get("artifact_dir", "");
    metadata["artifact_absolute_dir"] = summary.get("artifact_absolute_dir", "");
    metadata["result_json_path"] = summary.get("result_json_path", "");
    metadata["result_json_absolute_path"] = summary.get("result_json_absolute_path", "");
    return make_envelope(godot::String("\n\n").join(sections), metadata, raw_success);
}

} // namespace fennara::tool_results
