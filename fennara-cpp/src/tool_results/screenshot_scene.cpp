#include "fennara/tool_results/screenshot_scene.hpp"

#include "fennara/tool_results/envelope.hpp"

#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara::tool_results {

namespace {

godot::String bool_text(bool value) {
    return value ? "true" : "false";
}

godot::String image_summary_line(const godot::Dictionary &image) {
    int64_t width = static_cast<int64_t>(image.get("width", 0));
    int64_t height = static_cast<int64_t>(image.get("height", 0));
    godot::String mime = image.get("mime_type", "");
    godot::String line = "Image:";
    if (width > 0 && height > 0) {
        line += " " + godot::String::num_int64(width) + "x" +
                godot::String::num_int64(height);
    }
    if (!mime.is_empty()) {
        line += " " + mime;
    }
    godot::String role = image.get("image_role", "");
    if (!role.is_empty()) {
        line += " (" + role + ")";
    }
    return line;
}

void append_rect(godot::PackedStringArray &lines,
                 const godot::String &label,
                 const godot::Dictionary &rect) {
    godot::String line = label;
    line += ": x1=" + godot::String::num(rect.get("x1", 0.0), 1) +
                 ", y1=" + godot::String::num(rect.get("y1", 0.0), 1) +
                 ", x2=" + godot::String::num(rect.get("x2", 0.0), 1) +
                 ", y2=" + godot::String::num(rect.get("y2", 0.0), 1);
    lines.append(line);
}

void copy_if_present(godot::Dictionary &target,
                     const godot::Dictionary &source,
                     const godot::String &key) {
    if (source.has(key)) {
        target[key] = source[key];
    }
}

godot::Dictionary image_metadata_from_result(const godot::Dictionary &result) {
    godot::Dictionary image;
    image["view"] = result.get("view", "");
    image["image_role"] = result.get("image_role", "");
    image["format"] = result.get("format", "");
    image["mime_type"] = result.get("mime_type", "");
    image["width"] = result.get("width", 0);
    image["height"] = result.get("height", 0);
    image["image_res_path"] = result.get("image_res_path", "");
    image["image_path"] = result.get("image_path", "");
    image["transport"] = result.get("transport", "");
    return image;
}

} // namespace

godot::Dictionary format_screenshot_scene(const godot::Dictionary &raw_result) {
    bool raw_success = raw_result.get("success", false);
    bool has_image = raw_result.has("image_base64");
    godot::String status = raw_success ? "success" : "failed";
    if (raw_success && !has_image) {
        status = "partial";
    }
    if (raw_success && has_image &&
        godot::String(raw_result.get("content_validation", "passed")) !=
            "passed") {
        status = "partial";
    }

    godot::PackedStringArray lines;
    lines.append("Tool: screenshot_scene");
    lines.append("Status: " + status);
    lines.append("Scene: " + godot::String(raw_result.get("scene_path", "")));
    lines.append("3D scene: " + bool_text(raw_result.get("is_3d", false)));
    if (raw_result.has("view")) {
        lines.append("View: " + godot::String(raw_result.get("view", "")));
    }
    if (raw_result.get("scripted", false) || raw_result.has("script_path")) {
        lines.append("Script: " +
                     godot::String(raw_result.get("script_path", "")));
        lines.append("Script subjects: " + godot::String::num_int64(
            static_cast<int64_t>(raw_result.get("script_subject_count", 0))));
    }
    if (raw_result.has("current_camera_path")) {
        lines.append("Current camera: " + godot::String(raw_result.get("current_camera_path", "")));
    }
    if (has_image) {
        lines.append(image_summary_line(raw_result));
    }
    godot::Array additional_images = raw_result.get("images", godot::Array());
    for (int i = 0; i < additional_images.size(); i++) {
        if (additional_images[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary image = additional_images[i];
        lines.append(
            "Capture " + godot::String::num_int64(i + 2) + ": " +
            image_summary_line(image));
    }
    if (raw_result.has("camera_search")) {
        godot::Dictionary search = raw_result.get(
            "camera_search", godot::Dictionary());
        if (!search.is_empty()) {
            lines.append(
                "Camera search: chose candidate " +
                godot::String::num_int64(int64_t(search.get("chosen_index", -1))) +
                " of " +
                godot::String::num_int64(int64_t(search.get("candidate_count", 0))) +
                "; " +
                godot::String::num_int64(int64_t(search.get("visible_count", 0))) +
                " of " +
                godot::String::num_int64(int64_t(search.get("selected_count", 0))) +
                " selected nodes visible.");
        }
        godot::Array visibility = raw_result.get(
            "selected_node_visibility", godot::Array());
        for (int i = 0; i < visibility.size() && i < 8; i++) {
            if (visibility[i].get_type() != godot::Variant::DICTIONARY) continue;
            godot::Dictionary item = visibility[i];
            godot::String state = item.get("visible", false)
                ? godot::String("visible") : godot::String("not visible");
            lines.append(
                "Selection: " + godot::String(item.get("path", "")) +
                " (" + state + ")");
        }
        if (raw_result.has("camera_search_warning")) {
            lines.append("Camera search warning: " + godot::String(
                raw_result.get("camera_search_warning", "")));
        }
    }
    if (raw_result.has("content_validation")) {
        lines.append("Content validation: " +
                     godot::String(raw_result.get("content_validation", "")));
    }
    if (raw_result.has("content_coverage") && raw_result.has("content_max_span")) {
        lines.append("Content framing: coverage " +
                     godot::String::num(
                         double(raw_result.get("content_coverage", 0.0)) * 100.0,
                         2) +
                     "%, maximum span " +
                     godot::String::num(
                         double(raw_result.get("content_max_span", 0.0)) * 100.0,
                         2) + "%");
    }
    if (raw_result.has("content_warning")) {
        lines.append("Content warning: " +
                     godot::String(raw_result.get("content_warning", "")));
    }
    if (raw_result.has("image_res_path")) {
        lines.append("Saved resource: " + godot::String(raw_result.get("image_res_path", "")));
    }
    if (raw_result.has("image_path")) {
        lines.append("Saved file: " + godot::String(raw_result.get("image_path", "")));
    }
    if (raw_result.has("screenshot_dir")) {
        lines.append("Screenshot dir: " + godot::String(raw_result.get("screenshot_dir", "")));
    }
    if (raw_result.has("screenshot_absolute_dir")) {
        lines.append("Screenshot absolute dir: " +
                     godot::String(raw_result.get("screenshot_absolute_dir", "")));
    }
    if (raw_result.has("zoom_percent")) {
        lines.append("Zoom: " + godot::String::num_int64(
            static_cast<int64_t>(raw_result.get("zoom_percent", 0))) + "%");
    }
    if (raw_result.has("visible_rect") &&
        raw_result["visible_rect"].get_type() == godot::Variant::DICTIONARY) {
        append_rect(lines, "Visible rect", raw_result["visible_rect"]);
    }
    if (raw_result.has("error")) {
        lines.append("Error: " + godot::String(raw_result.get("error", "")));
    }
    if (raw_result.has("camera_warning")) {
        lines.append("Camera warning: " + godot::String(raw_result.get("camera_warning", "")));
    }

    godot::Array script_diagnostics =
        raw_result.get("script_diagnostics", godot::Array());
    godot::Array runtime_errors =
        raw_result.get("runtime_errors", godot::Array());
    godot::Array logs = raw_result.get("logs", godot::Array());
    const int script_output_limit = 50;
    for (int i = 0; i < script_diagnostics.size() && i < script_output_limit; i++) {
        godot::String message;
        if (script_diagnostics[i].get_type() == godot::Variant::DICTIONARY) {
            godot::Dictionary diagnostic = script_diagnostics[i];
            message = diagnostic.get("message", "");
        } else {
            message = script_diagnostics[i];
        }
        lines.append("Script diagnostic: " + message);
    }
    for (int i = 0; i < runtime_errors.size() && i < script_output_limit; i++) {
        godot::String source = "runtime";
        godot::String message;
        if (runtime_errors[i].get_type() == godot::Variant::DICTIONARY) {
            godot::Dictionary runtime_error = runtime_errors[i];
            source = runtime_error.get("source", source);
            message = runtime_error.get("message", "");
        } else {
            message = runtime_errors[i];
        }
        lines.append("Script error [" + source + "]: " + message);
    }
    for (int i = 0; i < logs.size() && i < script_output_limit; i++) {
        lines.append("Script log: " + godot::String(logs[i]));
    }
    if (script_diagnostics.size() > script_output_limit ||
        runtime_errors.size() > script_output_limit ||
        logs.size() > script_output_limit) {
        lines.append("Additional screenshot script output was omitted.");
    }

    godot::Array image_metadata;
    if (has_image) {
        image_metadata.append(image_metadata_from_result(raw_result));
    }
    for (int i = 0; i < additional_images.size(); i++) {
        if (additional_images[i].get_type() == godot::Variant::DICTIONARY) {
            image_metadata.append(image_metadata_from_result(additional_images[i]));
        }
    }

    godot::Dictionary metadata = make_base_metadata(
        "screenshot_scene", "screenshot_scene-md-v1", status);
    metadata["scene_path"] = raw_result.get("scene_path", "");
    metadata["current_camera_path"] = raw_result.get("current_camera_path", "");
    metadata["current_camera_type"] = raw_result.get("current_camera_type", "");
    metadata["view"] = raw_result.get("view", "");
    metadata["scripted"] = raw_result.get("scripted", false);
    metadata["script_path"] = raw_result.get("script_path", "");
    metadata["script_subject_count"] =
        raw_result.get("script_subject_count", 0);
    metadata["capture_count"] = raw_result.get("capture_count", 1);
    metadata["script_diagnostic_count"] = script_diagnostics.size();
    metadata["runtime_error_count"] = runtime_errors.size();
    metadata["script_log_count"] = logs.size();
    metadata["is_3d"] = raw_result.get("is_3d", false);
    metadata["image_count"] = image_metadata.size();
    metadata["images"] = image_metadata;
    metadata["has_primary_image"] = has_image;
    metadata["content_validation"] =
        raw_result.get("content_validation", "not_run");
    metadata["content_coverage"] = raw_result.get("content_coverage", 0.0);
    metadata["content_max_span"] = raw_result.get("content_max_span", 0.0);
    metadata["content_warning"] = raw_result.get("content_warning", "");
    metadata["previewed"] = false;
    metadata["selected_node_visibility"] = raw_result.get(
        "selected_node_visibility", godot::Array());
    metadata["camera_search"] = raw_result.get(
        "camera_search", godot::Dictionary());
    metadata["camera_search_warning"] = raw_result.get(
        "camera_search_warning", "");

    godot::Dictionary envelope = make_envelope(
        godot::String("\n").join(lines), metadata, raw_success);

    copy_if_present(envelope, raw_result, "image_base64");
    copy_if_present(envelope, raw_result, "format");
    copy_if_present(envelope, raw_result, "mime_type");
    copy_if_present(envelope, raw_result, "width");
    copy_if_present(envelope, raw_result, "height");
    copy_if_present(envelope, raw_result, "image_role");
    copy_if_present(envelope, raw_result, "capture_index");
    copy_if_present(envelope, raw_result, "capture_count");
    copy_if_present(envelope, raw_result, "captured_image_count");
    copy_if_present(envelope, raw_result, "images");
    copy_if_present(envelope, raw_result, "current_camera_path");
    copy_if_present(envelope, raw_result, "current_camera_type");
    copy_if_present(envelope, raw_result, "image_res_path");
    copy_if_present(envelope, raw_result, "image_path");
    copy_if_present(envelope, raw_result, "screenshot_dir");
    copy_if_present(envelope, raw_result, "screenshot_absolute_dir");
    copy_if_present(envelope, raw_result, "transport");
    copy_if_present(envelope, raw_result, "script_path");
    copy_if_present(envelope, raw_result, "scripted");
    copy_if_present(envelope, raw_result, "script_subject_count");
    copy_if_present(envelope, raw_result, "script_diagnostics");
    copy_if_present(envelope, raw_result, "runtime_errors");
    copy_if_present(envelope, raw_result, "logs");
    copy_if_present(envelope, raw_result, "selected_node_visibility");
    copy_if_present(envelope, raw_result, "camera_search");
    copy_if_present(envelope, raw_result, "camera_search_warning");
    return envelope;
}

} // namespace fennara::tool_results
