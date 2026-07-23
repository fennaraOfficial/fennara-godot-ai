#include "fennara/executor.hpp"

#include "fennara/tools/screenshot_scene.hpp"

#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/scene_tree_timer.hpp>
#include <godot_cpp/classes/rendering_server.hpp>
#include <godot_cpp/classes/image.hpp>

namespace fennara {

namespace {

constexpr double SCREENSHOT_SCRIPT_TIMEOUT_SECONDS = 300.0;

void copy_if_present(godot::Dictionary &target,
                     const godot::Dictionary &source,
                     const char *key) {
    if (source.has(key)) {
        target[key] = source[key];
    }
}

godot::Dictionary screenshot_image_record(const godot::Dictionary &result) {
    godot::Dictionary image;
    const char *keys[] = {
        "success", "capture_index", "capture_count", "scene_path", "is_3d",
        "view", "projection", "camera_position", "camera_distance",
        "current_camera_path", "current_camera_type", "script_subject_count",
        "image_base64", "format", "mime_type", "width", "height",
        "image_role", "image_res_path", "image_path", "transport",
        "render_presence_validation", "render_presence_coverage",
        "render_presence_max_span", "render_presence_warning",
        "selected_node_visibility", "camera_search",
        "camera_search_warning", "output_index", "description",
        "model_image_omitted"};
    for (const char *key : keys) {
        copy_if_present(image, result, key);
    }
    return image;
}

} // namespace

void FennaraExecutor::_start_next_screenshot_scene() {
    if (_batch_cancelled || _screenshot_running ||
        _pending_screenshot_scenes.empty()) {
        return;
    }

    PendingScreenshotScene pending = _pending_screenshot_scenes.front();
    _pending_screenshot_scenes.erase(_pending_screenshot_scenes.begin());
    uint64_t batch_generation = _async_batch_generation;
    _screenshot_running = true;

    uint64_t capture_owner = FennaraScreenshotSceneTool::try_reserve_capture();
    if (capture_owner == 0) {
        godot::Dictionary busy_result;
        busy_result["success"] = false;
        busy_result["error"] =
            "Another screenshot_scene capture is already in progress";
        _screenshot_running = false;
        _on_async_tool_complete(busy_result, pending.tool_index,
                                "screenshot_scene", pending.args,
                                batch_generation);
        _start_next_screenshot_scene();
        return;
    }
    _screenshot_capture_owner = capture_owner;

    godot::Dictionary open_result =
        FennaraScreenshotSceneTool::execute_prepared(pending.args);
    if (!(bool)open_result.get("success", false)) {
        FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
        _screenshot_capture_owner = 0;
        _screenshot_running = false;
        _on_async_tool_complete(open_result, pending.tool_index, "screenshot_scene", pending.args, batch_generation);
        _start_next_screenshot_scene();
        return;
    }

    _screenshot_tool_index = pending.tool_index;
    _screenshot_args = pending.args;
    _screenshot_nav_result = godot::Dictionary();
    _screenshot_primary_result = godot::Dictionary();
    _screenshot_additional_images = godot::Array();
    _screenshot_capture_index = 0;
    _screenshot_capture_count = 1;
    _screenshot_script_active = false;

    godot::SceneTree *tree = get_tree();
    if (tree) {
        godot::Ref<godot::SceneTreeTimer> timer = tree->create_timer(0.3);
        timer->connect("timeout", callable_mp(this, &FennaraExecutor::_on_screenshot_scene_opened).bind(batch_generation));
    } else {
        _on_screenshot_scene_opened(batch_generation);
    }
}

void FennaraExecutor::_on_screenshot_scene_opened(uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation) {
        return;
    }

    godot::String script_path = _screenshot_args.get(
        "_fennara_screenshot_script_path", "");
    if (!script_path.is_empty() && !_screenshot_script_active) {
        _screenshot_script_active = true;
        godot::Dictionary start_result =
            FennaraScreenshotSceneTool::begin_script_session(
                callable_mp(
                    this,
                    &FennaraExecutor::_on_screenshot_script_capture_requested)
                    .bind(batch_generation),
                callable_mp(
                    this,
                    &FennaraExecutor::_on_screenshot_script_completed)
                    .bind(batch_generation));
        if ((bool)start_result.get("success", false)) {
            godot::SceneTree *tree = get_tree();
            if (tree) {
                godot::Ref<godot::SceneTreeTimer> timeout =
                    tree->create_timer(SCREENSHOT_SCRIPT_TIMEOUT_SECONDS);
                timeout->connect(
                    "timeout",
                    callable_mp(
                        this,
                        &FennaraExecutor::_on_screenshot_script_timeout)
                        .bind(batch_generation, _screenshot_capture_owner));
            }
            return;
        }

        int idx = _screenshot_tool_index;
        godot::Dictionary args = _screenshot_args;
        _screenshot_tool_index = -1;
        _screenshot_args = godot::Dictionary();
        FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
        _screenshot_capture_owner = 0;
        _screenshot_running = false;
        _screenshot_script_active = false;
        _on_async_tool_complete(start_result, idx, "screenshot_scene", args,
                                batch_generation);
        _start_next_screenshot_scene();
        return;
    }

    godot::Dictionary nav_result = FennaraScreenshotSceneTool::navigate(
        _screenshot_args, _screenshot_capture_index);
    if ((bool)nav_result.get("success", false)) {
        int reported_count = int(nav_result.get("capture_count", 1));
        if (_screenshot_capture_index == 0) {
            _screenshot_capture_count = reported_count;
        }
        if (reported_count != _screenshot_capture_count ||
            _screenshot_capture_count < 1 || _screenshot_capture_count > 6) {
            nav_result["success"] = false;
            nav_result["error"] =
                "Screenshot capture queue changed between deterministic runs.";
        }
    }
    if (!(bool)nav_result.get("success", false)) {
        int idx = _screenshot_tool_index;
        godot::Dictionary args = _screenshot_args;
        _screenshot_tool_index = -1;
        _screenshot_args = godot::Dictionary();
        FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
        _screenshot_capture_owner = 0;
        _screenshot_running = false;
        _on_async_tool_complete(nav_result, idx, "screenshot_scene", args, batch_generation);
        _start_next_screenshot_scene();
        return;
    }

    _screenshot_nav_result = nav_result;

    double capture_delay = double(nav_result.get("capture_delay_seconds", 0.15));
    if (capture_delay < 0.0) capture_delay = 0.0;
    if (capture_delay > 10.0) capture_delay = 10.0;

    godot::SceneTree *tree = get_tree();
    if (tree) {
        godot::Ref<godot::SceneTreeTimer> timer = tree->create_timer(capture_delay);
        timer->connect("timeout", callable_mp(this, &FennaraExecutor::_on_screenshot_capture).bind(batch_generation));
    } else {
        _on_screenshot_capture(batch_generation);
    }
}

void FennaraExecutor::_on_screenshot_capture(uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation) {
        return;
    }

    godot::Dictionary capture_result =
        FennaraScreenshotSceneTool::capture_owned(_screenshot_capture_owner);

    if ((bool)capture_result.get("pending", false)) {
        call_deferred("_schedule_screenshot_capture", batch_generation);
        return;
    }

    godot::Dictionary merged = _screenshot_nav_result;
    if ((bool)capture_result.get("success", false)) {
        merged["image_base64"] = capture_result["image_base64"];
        merged["format"] = capture_result.get("format", "png");
        merged["mime_type"] = capture_result.get("mime_type", "image/png");
        merged["width"] = capture_result["width"];
        merged["height"] = capture_result["height"];
        merged["image_role"] = capture_result.get("image_role", "single");
        const char *camera_search_keys[] = {
            "selected_node_visibility", "camera_search",
            "camera_search_warning", "view", "projection",
            "camera_position", "camera_distance"};
        for (const char *key : camera_search_keys) {
            if (capture_result.has(key)) {
                merged[key] = capture_result[key];
            }
        }
        if (capture_result.has("image_res_path")) {
            merged["image_res_path"] = capture_result["image_res_path"];
            merged["image_path"] = capture_result["image_path"];
            merged["transport"] = capture_result.get("transport", "file_path");
        }
    } else {
        merged["success"] = false;
        merged["error"] = capture_result.get("error", "Capture failed");
    }
    if (capture_result.has("render_presence_validation")) {
        merged["render_presence_validation"] =
            capture_result["render_presence_validation"];
    }
    if (capture_result.has("render_presence_coverage")) {
        merged["render_presence_coverage"] =
            capture_result["render_presence_coverage"];
    }
    if (capture_result.has("render_presence_max_span")) {
        merged["render_presence_max_span"] =
            capture_result["render_presence_max_span"];
    }
    if (capture_result.has("render_presence_warning")) {
        merged["render_presence_warning"] =
            capture_result["render_presence_warning"];
    }

    if (!(bool)merged.get("success", false)) {
        int idx = _screenshot_tool_index;
        godot::Dictionary args = _screenshot_args;
        _screenshot_tool_index = -1;
        _screenshot_args = godot::Dictionary();
        _screenshot_nav_result = godot::Dictionary();
        _screenshot_primary_result = godot::Dictionary();
        _screenshot_additional_images = godot::Array();
        FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
        _screenshot_capture_owner = 0;
        _screenshot_running = false;
        _on_async_tool_complete(
            merged, idx, "screenshot_scene", args, batch_generation);
        _start_next_screenshot_scene();
        return;
    }

    merged["capture_index"] = _screenshot_capture_index;
    merged["capture_count"] = _screenshot_capture_count;
    if (_screenshot_capture_index == 0) {
        _screenshot_primary_result = merged;
    } else {
        _screenshot_additional_images.append(screenshot_image_record(merged));
    }

    if (_screenshot_capture_index + 1 < _screenshot_capture_count) {
        _screenshot_capture_index++;
        _screenshot_nav_result = godot::Dictionary();
        godot::SceneTree *tree = get_tree();
        if (tree) {
            godot::Ref<godot::SceneTreeTimer> timer = tree->create_timer(0.05);
            timer->connect(
                "timeout",
                callable_mp(
                    this, &FennaraExecutor::_on_screenshot_scene_opened)
                    .bind(batch_generation));
        } else {
            _on_screenshot_scene_opened(batch_generation);
        }
        return;
    }

    godot::Dictionary final_result = _screenshot_primary_result;
    final_result["capture_count"] = _screenshot_capture_count;
    final_result["captured_image_count"] =
        1 + _screenshot_additional_images.size();
    if (!_screenshot_additional_images.is_empty()) {
        final_result["images"] = _screenshot_additional_images;
    }

    int idx = _screenshot_tool_index;
    godot::Dictionary args = _screenshot_args;
    _screenshot_tool_index = -1;
    _screenshot_args = godot::Dictionary();
    _screenshot_nav_result = godot::Dictionary();
    _screenshot_primary_result = godot::Dictionary();
    _screenshot_additional_images = godot::Array();
    _screenshot_capture_index = 0;
    _screenshot_capture_count = 1;
    FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
    _screenshot_capture_owner = 0;
    _screenshot_running = false;

    _on_async_tool_complete(
        final_result, idx, "screenshot_scene", args, batch_generation);
    _start_next_screenshot_scene();
}

void FennaraExecutor::_on_screenshot_script_capture_requested(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running || !_screenshot_script_active) {
        return;
    }
    call_deferred("_begin_screenshot_script_capture", batch_generation);
}

void FennaraExecutor::_begin_screenshot_script_capture(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running || !_screenshot_script_active) {
        return;
    }

    godot::Dictionary nav_result =
        FennaraScreenshotSceneTool::navigate_pending_script_capture();
    if (!(bool)nav_result.get("success", false)) {
        FennaraScreenshotSceneTool::fail_script_capture(
            nav_result.get("error", "Could not frame ctx.capture()."));
        return;
    }
    _screenshot_nav_result = nav_result;

    double capture_delay = double(nav_result.get("capture_delay_seconds", 0.15));
    if (capture_delay < 0.0) capture_delay = 0.0;
    if (capture_delay > 10.0) capture_delay = 10.0;
    godot::SceneTree *tree = get_tree();
    if (tree) {
        godot::Ref<godot::SceneTreeTimer> timer =
            tree->create_timer(capture_delay);
        timer->connect(
            "timeout",
            callable_mp(this,
                        &FennaraExecutor::_on_screenshot_script_capture)
                .bind(batch_generation));
    } else {
        _on_screenshot_script_capture(batch_generation);
    }
}

void FennaraExecutor::_on_screenshot_script_capture(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running || !_screenshot_script_active) {
        return;
    }

    godot::Dictionary capture_result =
        FennaraScreenshotSceneTool::capture_image_owned(
            _screenshot_capture_owner);
    if ((bool)capture_result.get("pending", false)) {
        call_deferred("_schedule_screenshot_script_capture", batch_generation);
        return;
    }
    if (!(bool)capture_result.get("success", false)) {
        FennaraScreenshotSceneTool::fail_script_capture(
            capture_result.get("error", "ctx.capture() failed."));
        return;
    }

    godot::Ref<godot::Image> image =
        capture_result.get("image", godot::Variant());
    if (image.is_null()) {
        FennaraScreenshotSceneTool::fail_script_capture(
            "ctx.capture() rendered no image.");
        return;
    }
    FennaraScreenshotSceneTool::complete_script_capture(image);
}

void FennaraExecutor::_schedule_screenshot_script_capture(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running || !_screenshot_script_active) {
        return;
    }
    godot::RenderingServer::get_singleton()->request_frame_drawn_callback(
        callable_mp(this, &FennaraExecutor::_on_screenshot_script_capture)
            .bind(batch_generation));
}

void FennaraExecutor::_on_screenshot_script_completed(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running || !_screenshot_script_active) {
        return;
    }

    godot::Dictionary final_result =
        FennaraScreenshotSceneTool::finish_script_session();
    int idx = _screenshot_tool_index;
    godot::Dictionary args = _screenshot_args;
    _screenshot_tool_index = -1;
    _screenshot_args = godot::Dictionary();
    _screenshot_nav_result = godot::Dictionary();
    _screenshot_primary_result = godot::Dictionary();
    _screenshot_additional_images = godot::Array();
    _screenshot_capture_index = 0;
    _screenshot_capture_count = 1;
    _screenshot_script_active = false;
    FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
    _screenshot_capture_owner = 0;
    _screenshot_running = false;

    _on_async_tool_complete(final_result, idx, "screenshot_scene", args,
                            batch_generation);
    _start_next_screenshot_scene();
}

void FennaraExecutor::_on_screenshot_script_timeout(
    uint64_t batch_generation,
    uint64_t capture_owner) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running || !_screenshot_script_active ||
        _screenshot_capture_owner != capture_owner ||
        !FennaraScreenshotSceneTool::owns_capture(capture_owner) ||
        !FennaraScreenshotSceneTool::has_script_session()) {
        return;
    }

    FennaraScreenshotSceneTool::cancel_script_session(
        "Screenshot script exceeded the 300 second execution limit.");
    _on_screenshot_script_completed(batch_generation);
}

void FennaraExecutor::_schedule_screenshot_capture(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation ||
        !_screenshot_running) {
        return;
    }
    godot::RenderingServer::get_singleton()->request_frame_drawn_callback(
        callable_mp(this, &FennaraExecutor::_on_screenshot_capture)
            .bind(batch_generation));
}

} // namespace fennara
