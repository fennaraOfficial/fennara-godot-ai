#include "fennara/executor.hpp"

#include "fennara/tools/screenshot_scene.hpp"

#include <algorithm>

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
    _screenshot_views = godot::Array();
    _screenshot_captures = godot::Array();
    _screenshot_view_index = 0;
    bool has_camera_path = pending.args.has("camera_path") &&
        !godot::String(pending.args.get("camera_path", "")).strip_edges().is_empty();
    if ((bool)open_result.get("is_3d", false) && !has_camera_path) {
        godot::String view = pending.args.get("view", "perspective");
        view = view.to_lower();
        if (view == "all") {
            _screenshot_views.append("front");
            _screenshot_views.append("back");
            _screenshot_views.append("left");
            _screenshot_views.append("right");
            _screenshot_views.append("top");
            _screenshot_views.append("perspective");
            _screenshot_views.append("isometric");
        } else {
            _screenshot_views.append(view);
        }
    }

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

    godot::Dictionary nav_args = _screenshot_args;
    if (_screenshot_views.size() > 0) {
        nav_args["view"] = _screenshot_views[_screenshot_view_index];
    }

    godot::Dictionary nav_result = FennaraScreenshotSceneTool::navigate(nav_args);
    if (!(bool)nav_result.get("success", false)) {
        int idx = _screenshot_tool_index;
        godot::Dictionary args = _screenshot_args;
        _screenshot_tool_index = -1;
        _screenshot_args = godot::Dictionary();
        _screenshot_views = godot::Array();
        _screenshot_captures = godot::Array();
        _screenshot_view_index = 0;
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

    if (_screenshot_views.size() > 1) {
        _screenshot_captures.append(merged);
        _screenshot_view_index++;
        if (_screenshot_view_index < _screenshot_views.size()) {
            godot::SceneTree *tree = get_tree();
            if (tree) {
                godot::Ref<godot::SceneTreeTimer> timer = tree->create_timer(0.05);
                timer->connect("timeout", callable_mp(this, &FennaraExecutor::_on_screenshot_scene_opened).bind(batch_generation));
            } else {
                _on_screenshot_scene_opened(batch_generation);
            }
            return;
        }

        godot::Dictionary all_result;
        all_result["success"] = true;
        all_result["is_3d"] = true;
        all_result["scene_path"] = _screenshot_args.get("scene_path", "");
        if (_screenshot_args.has("target_node_path")) {
            all_result["target_node_path"] = _screenshot_args["target_node_path"];
        }
        all_result["view"] = "all";
        bool all_content_passed = true;
        double minimum_coverage = 1.0;
        double minimum_span = 1.0;
        bool has_content_metrics = false;
        godot::String failed_views;
        godot::Dictionary collage =
            FennaraScreenshotSceneTool::make_collage(_screenshot_captures);
        godot::Array image_metadata;
        for (int i = 0; i < _screenshot_captures.size(); i++) {
            godot::Dictionary image = _screenshot_captures[i];
            if (image.has("content_validation")) {
                has_content_metrics = true;
                minimum_coverage = std::min(
                    minimum_coverage,
                    double(image.get("content_coverage", 0.0)));
                minimum_span = std::min(
                    minimum_span,
                    double(image.get("content_max_span", 0.0)));
                if (godot::String(image.get("content_validation", "failed")) !=
                    "passed") {
                    all_content_passed = false;
                    if (!failed_views.is_empty()) failed_views += ", ";
                    failed_views += godot::String(image.get("view", "unknown"));
                }
            }
            image.erase("image_base64");
            image_metadata.append(image);
        }
        all_result["images"] = image_metadata;
        if (has_content_metrics) {
            all_result["content_validation"] =
                all_content_passed ? "passed" : "failed";
            all_result["content_coverage"] = minimum_coverage;
            all_result["content_max_span"] = minimum_span;
            if (!all_content_passed) {
                all_result["content_warning"] =
                    "Captured collage was returned, but automatic framing was visually sparse in views: " +
                    failed_views + ".";
            }
        }
        if ((bool)collage.get("success", false)) {
            all_result["image_base64"] = collage["image_base64"];
            all_result["format"] = collage.get("format", "png");
            all_result["mime_type"] = collage.get("mime_type", "image/png");
            all_result["width"] = collage["width"];
            all_result["height"] = collage["height"];
            all_result["image_role"] = "collage";
            all_result["image_res_path"] = collage.get("image_res_path", "");
            all_result["image_path"] = collage.get("image_path", "");
            all_result["transport"] = collage.get("transport", "file_path");
            all_result["collage_columns"] = collage["columns"];
            all_result["collage_rows"] = collage["rows"];
        } else {
            all_result["collage_error"] =
                collage.get("error", "Failed to build collage");
        }
        merged = all_result;
    }

    int idx = _screenshot_tool_index;
    godot::Dictionary args = _screenshot_args;
    _screenshot_tool_index = -1;
    _screenshot_args = godot::Dictionary();
    _screenshot_nav_result = godot::Dictionary();
    _screenshot_views = godot::Array();
    _screenshot_captures = godot::Array();
    _screenshot_view_index = 0;
    FennaraScreenshotSceneTool::release_capture(_screenshot_capture_owner);
    _screenshot_capture_owner = 0;
    _screenshot_running = false;

    _on_async_tool_complete(merged, idx, "screenshot_scene", args, batch_generation);
    _start_next_screenshot_scene();
}

} // namespace fennara
