#include "fennara/executor.hpp"

#include "fennara/tools/screenshot_scene.hpp"

#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/scene_tree_timer.hpp>

namespace fennara {

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

    godot::Dictionary open_result = FennaraScreenshotSceneTool::execute(pending.args);
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

    godot::Dictionary nav_result =
        FennaraScreenshotSceneTool::navigate(_screenshot_args);
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

    godot::Dictionary merged = _screenshot_nav_result;
    if ((bool)capture_result.get("success", false)) {
        merged["image_base64"] = capture_result["image_base64"];
        merged["format"] = capture_result.get("format", "png");
        merged["mime_type"] = capture_result.get("mime_type", "image/png");
        merged["width"] = capture_result["width"];
        merged["height"] = capture_result["height"];
        merged["image_role"] = capture_result.get("image_role", "single");
        if (capture_result.has("image_res_path")) {
            merged["image_res_path"] = capture_result["image_res_path"];
            merged["image_path"] = capture_result["image_path"];
            merged["transport"] = capture_result.get("transport", "file_path");
        }
    } else {
        merged["success"] = false;
        merged["error"] = capture_result.get("error", "Capture failed");
    }
    if (capture_result.has("content_validation")) {
        merged["content_validation"] = capture_result["content_validation"];
    }
    if (capture_result.has("content_coverage")) {
        merged["content_coverage"] = capture_result["content_coverage"];
    }
    if (capture_result.has("content_max_span")) {
        merged["content_max_span"] = capture_result["content_max_span"];
    }
    if (capture_result.has("content_warning")) {
        merged["content_warning"] = capture_result["content_warning"];
    }

    int idx = _screenshot_tool_index;
    godot::Dictionary args = _screenshot_args;
    _screenshot_tool_index = -1;
    _screenshot_args = godot::Dictionary();
    _screenshot_nav_result = godot::Dictionary();
    FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
    _screenshot_capture_owner = 0;
    _screenshot_running = false;

    _on_async_tool_complete(merged, idx, "screenshot_scene", args, batch_generation);
    _start_next_screenshot_scene();
}

} // namespace fennara
