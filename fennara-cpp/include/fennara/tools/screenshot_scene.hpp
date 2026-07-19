#pragma once

#include <cstdint>

#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/variant/aabb.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/transform3d.hpp>

namespace godot {
class SubViewport;
}

namespace fennara {

class FennaraScreenshotSceneTool : public godot::RefCounted {
    GDCLASS(FennaraScreenshotSceneTool, godot::RefCounted)

protected:
    static void _bind_methods();

public:
    static godot::Dictionary open_scene(const godot::String &scene_path);
    static godot::Dictionary navigate(const godot::Dictionary &args);
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
        const godot::Dictionary &capture_options);
    static godot::Dictionary _frame_2d_script_capture(
        godot::Node *root, const godot::Array &capture_nodes,
        const godot::Dictionary &capture_options);
    static bool _prepare_capture_script(const godot::Dictionary &args,
                                        godot::Dictionary &result);
    static bool _has_capture_script();
    static bool _run_capture_script(godot::Node *root,
                                    godot::Dictionary &result,
                                    godot::Array &capture_nodes,
                                    godot::Dictionary &capture_options);
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
    static uint64_t &_active_capture_owner_ref();
    static uint64_t &_next_capture_owner_ref();
    static void _discard_temporary_viewport();
    static bool _is_3d_scene;
};

} // namespace fennara
