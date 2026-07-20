#include "fennara/tools/screenshot_scene.hpp"

#include "fennara/logger.hpp"

#include <algorithm>
#include <cmath>
#include <vector>

#include <godot_cpp/classes/array_mesh.hpp>
#include <godot_cpp/classes/camera3d.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/directional_light3d.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/environment.hpp>
#include <godot_cpp/classes/light3d.hpp>
#include <godot_cpp/classes/mesh_instance3d.hpp>
#include <godot_cpp/classes/node3d.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/skeleton3d.hpp>
#include <godot_cpp/classes/skin_reference.hpp>
#include <godot_cpp/classes/sub_viewport.hpp>
#include <godot_cpp/classes/visual_instance3d.hpp>
#include <godot_cpp/classes/world_environment.hpp>
#include <godot_cpp/variant/node_path.hpp>

namespace fennara {

namespace {

bool contains_light(godot::Node *node) {
    if (!node) return false;
    if (auto *light = godot::Object::cast_to<godot::Light3D>(node)) {
        if (light->is_visible_in_tree() &&
            light->get_param(godot::Light3D::PARAM_ENERGY) > 0.0) {
            return true;
        }
    }
    for (int i = 0; i < node->get_child_count(); i++) {
        if (contains_light(node->get_child(i))) return true;
    }
    return false;
}

bool contains_environment(godot::Node *node) {
    if (!node) return false;
    if (auto *world = godot::Object::cast_to<godot::WorldEnvironment>(node)) {
        if (world->get_environment().is_valid()) return true;
    }
    for (int i = 0; i < node->get_child_count(); i++) {
        if (contains_environment(node->get_child(i))) return true;
    }
    return false;
}

void force_skeleton_updates(godot::Node *node) {
    if (!node) return;
    if (auto *skeleton = godot::Object::cast_to<godot::Skeleton3D>(node)) {
        skeleton->force_update_all_bone_transforms();
    }
    for (int i = 0; i < node->get_child_count(); i++) {
        force_skeleton_updates(node->get_child(i));
    }
}

} // namespace

godot::Transform3D FennaraScreenshotSceneTool::_local_tree_3d_transform(godot::Node *node) {
    godot::Transform3D transform;
    if (!node) {
        return transform;
    }

    std::vector<godot::Node *> chain;
    godot::Node *current = node;
    while (current) {
        chain.push_back(current);
        godot::Node3D *node_3d = godot::Object::cast_to<godot::Node3D>(current);
        if (node_3d && node_3d->is_set_as_top_level()) {
            break;
        }
        current = current->get_parent();
    }

    for (int i = static_cast<int>(chain.size()) - 1; i >= 0; i--) {
        godot::Node3D *node_3d = godot::Object::cast_to<godot::Node3D>(chain[i]);
        if (node_3d) {
            transform = transform * node_3d->get_transform();
        }
    }
    return transform;
}

void FennaraScreenshotSceneTool::_accumulate_3d_bounds(godot::Node *node,
                                                godot::AABB &bounds,
                                                bool &has_bounds) {
    if (!node) return;

    godot::VisualInstance3D *visual =
        godot::Object::cast_to<godot::VisualInstance3D>(node);
    if (visual && visual->is_visible()) {
        godot::AABB local_bounds = visual->get_aabb();
        godot::MeshInstance3D *mesh_instance =
            godot::Object::cast_to<godot::MeshInstance3D>(visual);
        if (mesh_instance && mesh_instance->get_skin_reference().is_valid()) {
            godot::Ref<godot::ArrayMesh> baked_mesh =
                mesh_instance->bake_mesh_from_current_skeleton_pose();
            if (baked_mesh.is_valid()) {
                local_bounds = baked_mesh->get_aabb();
            }
        }
        if (local_bounds.has_surface()) {
            godot::AABB global_bounds =
                _local_tree_3d_transform(visual).xform(local_bounds).abs();
            if (has_bounds) {
                bounds.merge_with(global_bounds);
            } else {
                bounds = global_bounds;
                has_bounds = true;
            }
        }
    }

    for (int i = 0; i < node->get_child_count(); i++) {
        _accumulate_3d_bounds(node->get_child(i), bounds, has_bounds);
    }
}

godot::Dictionary FennaraScreenshotSceneTool::_frame_3d_editor_camera(
    godot::Node *root, const godot::Array &capture_nodes,
    const godot::Dictionary &capture_options,
    bool use_default_camera_search) {
    godot::Dictionary result;

    if (use_default_camera_search &&
        capture_nodes.size() > SCREENSHOT_CAMERA_SEARCH_SUBJECT_LIMIT) {
        result["success"] = false;
        result["error"] =
            "Automatic 3D camera search accepts at most eight selected subjects per capture. Select a shared parent, split them across captures, or provide an explicit view or camera.";
        return result;
    }

    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    if (!editor) {
        result["success"] = false;
        result["error"] = "EditorInterface not available";
        return result;
    }

    godot::Node *base = godot::Object::cast_to<godot::Node>(editor->get_base_control());
    if (!base) {
        result["success"] = false;
        result["error"] = "Editor base control not available";
        return result;
    }

    godot::SubViewport *previous = _camera_capture_viewport_ref();
    if (previous) {
        _discard_temporary_viewport();
    }
    _capture_requires_content_ref() = false;
    _clear_camera_search_capture_state();

    godot::SubViewport *viewport = memnew(godot::SubViewport);
    viewport->set_name("FennaraFramedScreenshotViewport");
    viewport->set_update_mode(godot::SubViewport::UPDATE_ALWAYS);
    viewport->set_clear_mode(godot::SubViewport::CLEAR_MODE_ALWAYS);
    viewport->set_transparent_background(false);
    viewport->set_use_own_world_3d(true);
    base->add_child(viewport);
    viewport->add_child(root);
    force_skeleton_updates(root);

    godot::Camera3D *script_camera = nullptr;
    if (capture_options.has("camera")) {
        godot::Object *camera_object = capture_options["camera"];
        godot::Node *camera_node = godot::Object::cast_to<godot::Node>(camera_object);
        script_camera = godot::Object::cast_to<godot::Camera3D>(camera_node);
        if (!script_camera ||
            (camera_node != root && !root->is_ancestor_of(camera_node))) {
            viewport->queue_free();
            result["success"] = false;
            result["error"] =
                "Script capture option `camera` must be a Camera3D under ctx.root for this scene.";
            return result;
        }
    }

    if (script_camera) {
        godot::ProjectSettings *ps = godot::ProjectSettings::get_singleton();
        int width = std::max(
            int(ps->get_setting("display/window/size/viewport_width", 1920)), 64);
        int height = std::max(
            int(ps->get_setting("display/window/size/viewport_height", 1080)), 64);
        viewport->set_size(godot::Vector2i(width, height));
        script_camera->make_current();
        _camera_capture_viewport_ref() = viewport;
        _camera_capture_root_ref() = root;
        _capture_requires_content_ref() = true;
        _capture_name_hint_ref() = _make_name_hint(
            _current_scene_path_ref(), "script", "camera_3d");
        result["success"] = true;
        result["is_3d"] = true;
        result["scene_path"] = _current_scene_path_ref();
        result["view"] = "camera_3d";
        result["capture_delay_seconds"] = 0.15;
        result["note"] =
            "3D scene captured from the Camera3D supplied to ctx.capture.";
        godot::Dictionary viewport_dict;
        viewport_dict["width"] = width;
        viewport_dict["height"] = height;
        result["viewport_size"] = viewport_dict;
        _append_capture_script_receipt(result);
        return result;
    }

    godot::AABB bounds;
    bool has_bounds = false;
    for (int i = 0; i < capture_nodes.size(); i++) {
        godot::Object *object = capture_nodes[i];
        godot::Node *node = godot::Object::cast_to<godot::Node>(object);
        _accumulate_3d_bounds(node, bounds, has_bounds);
    }
    if (!has_bounds) {
        for (int i = 0; i < capture_nodes.size(); i++) {
            godot::Object *object = capture_nodes[i];
            godot::Node3D *target_3d =
                godot::Object::cast_to<godot::Node3D>(object);
            if (!target_3d) continue;
            godot::Vector3 target_position =
                _local_tree_3d_transform(target_3d).origin;
            godot::AABB point_bounds(
                target_position - godot::Vector3(1, 1, 1),
                godot::Vector3(2, 2, 2));
            if (has_bounds) {
                bounds.merge_with(point_bounds);
            } else {
                bounds = point_bounds;
                has_bounds = true;
            }
        }
        if (!has_bounds) {
            viewport->queue_free();
            result["success"] = false;
            result["error"] = "No visible 3D geometry bounds found for isolated capture";
            return result;
        }
    }
    godot::String view = capture_options.get("view", "perspective");
    view = view.to_lower();

    godot::Vector3 center = bounds.get_center();
    godot::Vector3 size = bounds.get_size();
    double diagonal = std::sqrt(double(size.x * size.x + size.y * size.y + size.z * size.z));
    double radius = std::max(diagonal * 0.5, 1.0);
    double margin = double(capture_options.get("context_margin", 1.1));
    margin = std::max(margin, 0.25);

    godot::Vector3 view_dir = godot::Vector3(1.0, 0.65, 1.0).normalized();
    godot::Vector3 up = godot::Vector3(0.0, 1.0, 0.0);
    if (view == "front") {
        view_dir = godot::Vector3(0.0, 0.0, 1.0);
    } else if (view == "back") {
        view_dir = godot::Vector3(0.0, 0.0, -1.0);
    } else if (view == "left") {
        view_dir = godot::Vector3(-1.0, 0.0, 0.0);
    } else if (view == "right") {
        view_dir = godot::Vector3(1.0, 0.0, 0.0);
    } else if (view == "top") {
        view_dir = godot::Vector3(0.0, 1.0, 0.0);
        up = godot::Vector3(0.0, 0.0, -1.0);
    } else if (view == "isometric") {
        view_dir = godot::Vector3(1.0, 0.85, 1.0).normalized();
    } else if (view == "perspective") {
        view_dir = godot::Vector3(1.0, 0.65, 1.0).normalized();
    } else {
        viewport->queue_free();
        result["success"] = false;
        result["error"] = "Unsupported 3D view: " + view;
        return result;
    }

    godot::Vector3 right = up.cross(view_dir).normalized();
    godot::Vector3 camera_up = view_dir.cross(right).normalized();

    const double fov_degrees = 70.0;
    double fov_rad = fov_degrees * 3.14159265358979323846 / 180.0;
    double tan_vertical = std::tan(fov_rad * 0.5);
    godot::ProjectSettings *ps = godot::ProjectSettings::get_singleton();
    int width = int(ps->get_setting("display/window/size/viewport_width", 1920));
    int height = int(ps->get_setting("display/window/size/viewport_height", 1080));
    width = std::max(width, 64);
    height = std::max(height, 64);
    godot::Vector2i viewport_size(width, height);
    double aspect = viewport_size.y > 0 ? double(viewport_size.x) / double(viewport_size.y) : 1.0;
    double tan_horizontal = tan_vertical * aspect;

    godot::Vector3 half = size * 0.5;
    double fit_distance = 0.0;
    for (int xi = -1; xi <= 1; xi += 2) {
        for (int yi = -1; yi <= 1; yi += 2) {
            for (int zi = -1; zi <= 1; zi += 2) {
                godot::Vector3 corner = center + godot::Vector3(
                    half.x * double(xi),
                    half.y * double(yi),
                    half.z * double(zi));
                godot::Vector3 offset = corner - center;
                double x = std::abs(double(offset.dot(right)));
                double y = std::abs(double(offset.dot(camera_up)));
                double toward_camera = double(offset.dot(view_dir));
                double required = toward_camera + std::max(
                    x / std::max(tan_horizontal, 0.0001),
                    y / std::max(tan_vertical, 0.0001));
                fit_distance = std::max(fit_distance, required);
            }
        }
    }

    double effective_margin = margin;
    double distance = std::max(fit_distance * effective_margin, 0.75);
    godot::Vector3 camera_position = center + view_dir * distance;

    double orthographic_size = 0.0;
    if (view == "isometric") {
        double projected_half_width = 0.0;
        double projected_half_height = 0.0;
        for (int xi = -1; xi <= 1; xi += 2) {
            for (int yi = -1; yi <= 1; yi += 2) {
                for (int zi = -1; zi <= 1; zi += 2) {
                    godot::Vector3 corner = center + godot::Vector3(
                        half.x * double(xi),
                        half.y * double(yi),
                        half.z * double(zi));
                    godot::Vector3 offset = corner - center;
                    projected_half_width =
                        std::max(projected_half_width, std::abs(double(offset.dot(right))));
                    projected_half_height =
                        std::max(projected_half_height, std::abs(double(offset.dot(camera_up))));
                }
            }
        }
        orthographic_size = std::max(projected_half_height * 2.0,
                                     (projected_half_width * 2.0) /
                                         std::max(aspect, 0.0001));
        orthographic_size = std::max(orthographic_size * effective_margin, 1.0);
        distance = std::max(radius * 3.0, orthographic_size + radius + 1.0);
        camera_position = center + view_dir * distance;
    }

    viewport->set_size(viewport_size);

    godot::Camera3D *camera = memnew(godot::Camera3D);
    camera->set_name("FennaraFramedScreenshotCamera");
    if (view == "isometric") {
        camera->set_projection(godot::Camera3D::PROJECTION_ORTHOGONAL);
        camera->set_size(float(orthographic_size));
    } else {
        camera->set_projection(godot::Camera3D::PROJECTION_PERSPECTIVE);
        camera->set_fov(float(fov_degrees));
    }
    camera->set_near(0.05f);
    camera->set_far(float(std::max(distance + radius * 10.0, 1000.0)));

    viewport->add_child(camera);

    if (!contains_environment(root)) {
        godot::Ref<godot::Environment> environment;
        environment.instantiate();
        environment->set_background(godot::Environment::BG_COLOR);
        environment->set_bg_color(godot::Color(0.11, 0.12, 0.14, 1.0));
        environment->set_ambient_source(godot::Environment::AMBIENT_SOURCE_COLOR);
        environment->set_ambient_light_color(godot::Color(1.0, 1.0, 1.0, 1.0));
        environment->set_ambient_light_energy(0.65f);

        godot::WorldEnvironment *world_environment =
            memnew(godot::WorldEnvironment);
        world_environment->set_name("FennaraPreviewEnvironment");
        world_environment->set_environment(environment);
        viewport->add_child(world_environment);
    }
    if (!contains_light(root)) {
        godot::DirectionalLight3D *light = memnew(godot::DirectionalLight3D);
        light->set_name("FennaraPreviewKeyLight");
        light->set_rotation_degrees(godot::Vector3(-45.0, -35.0, 0.0));
        light->set_param(godot::Light3D::PARAM_ENERGY, 1.1f);
        viewport->add_child(light);
    }

    camera->look_at_from_position(camera_position, center, up);
    camera->make_current();

    _camera_capture_viewport_ref() = viewport;
    _camera_capture_root_ref() = root;
    _capture_requires_content_ref() = true;

    if (use_default_camera_search) {
        godot::Dictionary &state = _camera_search_capture_state_ref();
        state["enabled"] = true;
        state["root"] = root;
        state["camera"] = camera;
        state["capture_nodes"] = capture_nodes;
        state["bounds_center"] = center;
        state["bounds_size"] = size;
        state["primary_view"] = view;
    }

    godot::Dictionary bounds_dict;
    bounds_dict["center"] = center;
    bounds_dict["size"] = size;
    result["success"] = true;
    result["is_3d"] = true;
    result["scene_path"] = _current_scene_path_ref();
    result["view"] = use_default_camera_search
        ? godot::String("camera_search") : view;
    result["note"] = use_default_camera_search
        ? godot::String(
            "3D scene: deterministic camera search around ctx.capture subjects")
        : godot::String(
            "3D scene: isolated capture auto-framed around ctx.capture subjects");
    result["framed_bounds"] = bounds_dict;
    result["camera_distance"] = distance;
    result["camera_position"] = camera_position;
    result["projection"] = view == "isometric" ? "orthogonal" : "perspective";
    if (view == "isometric") {
        result["orthographic_size"] = orthographic_size;
    }
    result["context_margin"] = margin;
    result["effective_context_margin"] = effective_margin;
    result["capture_delay_seconds"] = 0.15;
    godot::Dictionary viewport_dict;
    viewport_dict["width"] = width;
    viewport_dict["height"] = height;
    result["viewport_size"] = viewport_dict;
    _append_capture_script_receipt(result);
    _capture_name_hint_ref() = _make_name_hint(
        _current_scene_path_ref(), "selection",
        use_default_camera_search ? godot::String("camera_search") : view);

    FLOG_TOOL(godot::String("SS: auto-framed 3D bounds center=") +
              godot::String(center) + " size=" + godot::String(size) +
              " view=" + view + " distance=" + godot::String::num(distance, 2));
    return result;
}

} // namespace fennara
