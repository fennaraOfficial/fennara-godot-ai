#pragma once

#include <cstdint>

#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/variant/aabb.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/transform3d.hpp>

namespace godot {
class Image;
class SubViewport;
}

namespace fennara {

inline constexpr int SCREENSHOT_CAMERA_SEARCH_SUBJECT_LIMIT = 8;

class FennaraScreenshotSceneTool : public godot::RefCounted {
    GDCLASS(FennaraScreenshotSceneTool, godot::RefCounted)

protected:
    static void _bind_methods();

public:
    static godot::Dictionary prepare_execution(const godot::Dictionary &args);
    static godot::Dictionary execute_prepared(
        const godot::Dictionary &prepared_args);
    static godot::Dictionary open_scene(const godot::String &scene_path);
    static godot::Dictionary navigate(const godot::Dictionary &args,
                                        int capture_index = 0);
    static godot::Dictionary capture_owned(uint64_t owner);
    static godot::Dictionary execute(const godot::Dictionary &args);
    static uint64_t try_reserve_capture();
    static void release_capture(uint64_t owner);

private:
    static godot::String _save_png_data(const godot::PackedByteArray &png_data,
                                        const godot::String &name_hint,
                                        godot::Dictionary &result);
    static godot::Transform3D _local_tree_3d_transform(godot::Node *node);
    static void _accumulate_3d_bounds(godot::Node *node, godot::AABB &bounds,
                                      bool &has_bounds);
    static godot::Dictionary _frame_3d_editor_camera(
        godot::Node *root, const godot::Array &capture_nodes,
        const godot::Dictionary &capture_options,
        bool use_default_camera_search);
    static godot::Dictionary _frame_2d_script_capture(
        godot::Node *root, const godot::Array &capture_nodes,
        const godot::Dictionary &capture_options);
    static godot::Ref<godot::Image> _capture_camera_searched_3d(
        godot::SubViewport *viewport,
        godot::Dictionary &result);
    static bool _configure_capture_script(const godot::Dictionary &args,
                                          godot::Dictionary &result);
    static bool _has_capture_script();
    static bool _run_capture_script(godot::Node *root,
                                    godot::Dictionary &result,
                                    godot::Array &capture_nodes,
                                    godot::Dictionary &capture_options,
                                    int capture_index);
    static void _append_capture_script_receipt(godot::Dictionary &result);
    static void _clear_capture_script();
    static godot::String _make_name_hint(const godot::String &scene_path,
                                         const godot::String &subject_label,
                                         const godot::String &view);
    static godot::String &_current_scene_path_ref();
    static godot::String &_capture_name_hint_ref();
    static godot::String &_artifact_dir_ref();
    static godot::SubViewport *&_camera_capture_viewport_ref();
    static godot::Node *&_camera_capture_root_ref();
    static bool &_capture_requires_content_ref();
    static godot::Dictionary &_camera_search_capture_state_ref();
    static void _clear_camera_search_capture_state();
    static void _reset_camera_search_job();
    static godot::Node *&_script_capture_root_ref();
    static godot::Array &_script_capture_requests_ref();
    static godot::Dictionary &_script_capture_receipt_ref();
    static bool &_preserve_script_root_after_capture_ref();
    static void _clear_script_capture_session(bool free_detached_root);
    static uint64_t &_active_capture_owner_ref();
    static uint64_t &_next_capture_owner_ref();
    static void _discard_temporary_viewport(bool preserve_script_root = false);
    static bool _is_3d_scene;
};

} // namespace fennara
