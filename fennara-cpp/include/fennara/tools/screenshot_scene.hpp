#pragma once

#include <cstdint>

#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/variant/aabb.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/transform3d.hpp>

namespace godot {
class Callable;
class RefCounted;
class Image;
class SubViewport;
struct Vector2i;
}

namespace fennara {

class FennaraScreenshotSceneScriptContext;
class FennaraWarningCapture;

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
    static godot::Dictionary capture_image_owned(uint64_t owner);
    static godot::Dictionary begin_script_session(
        const godot::Callable &capture_requested,
        const godot::Callable &script_completed);
    static godot::Dictionary navigate_pending_script_capture();
    static void complete_script_capture(
        const godot::Ref<godot::Image> &image);
    static void fail_script_capture(const godot::String &message);
    static void cancel_script_session(const godot::String &message);
    static godot::Dictionary finish_script_session();
    static bool has_script_session();
    static godot::Dictionary publish_image(
        const godot::Ref<godot::Image> &image,
        const godot::String &description,
        int output_index);
    static godot::Dictionary execute(const godot::Dictionary &args);
    static uint64_t try_reserve_capture();
    static bool owns_capture(uint64_t owner);
    static void release_capture(uint64_t owner);
#ifdef FENNARA_SETUP_TEST_HOOKS
    static godot::Dictionary test_script_viewport_reuse();
#endif

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
    static godot::SubViewport *_prepare_capture_viewport(
        godot::Node *root, const godot::String &name,
        const godot::Vector2i &size, bool use_own_world_3d,
        godot::Dictionary &result);
    static void _cleanup_failed_capture_setup();
    static bool &_capture_requires_content_ref();
    static godot::Dictionary &_camera_search_capture_state_ref();
    static void _clear_camera_search_capture_state();
    static void _reset_camera_search_job();
    static godot::Node *&_script_capture_root_ref();
    static godot::Ref<FennaraScreenshotSceneScriptContext>
        &_script_context_ref();
    static godot::Ref<godot::RefCounted> &_script_runner_ref();
    static godot::Ref<godot::RefCounted> &_script_instance_ref();
    static godot::Ref<FennaraWarningCapture> &_script_warning_capture_ref();
    static bool &_preserve_script_root_after_capture_ref();
    static void _clear_script_capture_session(bool free_detached_root);
    static uint64_t &_active_capture_owner_ref();
    static uint64_t &_next_capture_owner_ref();
    static void _discard_temporary_viewport(bool preserve_script_root = false);
    static bool _is_3d_scene;
};

} // namespace fennara
