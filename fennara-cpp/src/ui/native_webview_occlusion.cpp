#if defined(_WIN32) || defined(__APPLE__)

#include "native_webview_occlusion.hpp"

#include <godot_cpp/classes/canvas_item.hpp>
#include <godot_cpp/classes/canvas_layer.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/display_server.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/object.hpp>
#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/window.hpp>
#include <godot_cpp/variant/rect2.hpp>
#include <godot_cpp/variant/rect2i.hpp>
#include <godot_cpp/variant/transform2d.hpp>
#include <godot_cpp/variant/vector2.hpp>
#include <godot_cpp/variant/vector2i.hpp>

#include <algorithm>

namespace fennara::webview_backend {
namespace {

constexpr double kCandidateRefreshSeconds = 0.5;

godot::Control *control_from_id(uint64_t id) {
    return godot::Object::cast_to<godot::Control>(
        godot::ObjectDB::get_instance(id));
}

godot::CanvasLayer *canvas_layer_from_id(uint64_t id) {
    return godot::Object::cast_to<godot::CanvasLayer>(
        godot::ObjectDB::get_instance(id));
}

godot::Window *window_from_id(uint64_t id) {
    return godot::Object::cast_to<godot::Window>(
        godot::ObjectDB::get_instance(id));
}

godot::Rect2 transformed_control_rect(godot::Control *control) {
    if (control == nullptr) {
        return godot::Rect2();
    }
    return control->get_global_transform_with_canvas().xform(
        godot::Rect2(godot::Vector2(), control->get_size()));
}

bool rect_has_area(const godot::Rect2 &rect) {
    return rect.size.x > 0.0 && rect.size.y > 0.0;
}

bool controls_overlap(godot::Control *candidate, godot::Control *owner) {
    if (candidate == nullptr ||
        owner == nullptr ||
        !candidate->is_visible_in_tree() ||
        candidate->get_window() != owner->get_window()) {
        return false;
    }

    const godot::Rect2 candidate_rect = transformed_control_rect(candidate);
    const godot::Rect2 owner_rect = transformed_control_rect(owner);
    return rect_has_area(candidate_rect) &&
           rect_has_area(owner_rect) &&
           candidate_rect.intersects(owner_rect);
}

bool visible_control_overlaps(godot::Node *node, godot::Control *owner) {
    if (node == nullptr) {
        return false;
    }

    if (auto *control = godot::Object::cast_to<godot::Control>(node)) {
        if (controls_overlap(control, owner)) {
            return true;
        }
        if (!control->is_visible_in_tree()) {
            return false;
        }
    }

    const int32_t child_count = node->get_child_count();
    for (int32_t index = 0; index < child_count; index++) {
        if (visible_control_overlaps(node->get_child(index), owner)) {
            return true;
        }
    }
    return false;
}

bool embedded_window_overlaps(godot::Window *candidate, godot::Control *owner) {
    if (candidate == nullptr ||
        owner == nullptr ||
        candidate == owner->get_window() ||
        !candidate->is_visible() ||
        !candidate->is_embedded()) {
        return false;
    }

    const godot::Vector2i position = candidate->get_position();
    const godot::Vector2i size = candidate->get_size();
    const godot::Rect2 window_rect(
        godot::Vector2(position.x, position.y),
        godot::Vector2(size.x, size.y));
    const godot::Rect2 owner_rect = transformed_control_rect(owner);
    return rect_has_area(window_rect) &&
           rect_has_area(owner_rect) &&
           window_rect.intersects(owner_rect);
}

bool active_popup_overlaps(godot::Control *owner) {
    godot::DisplayServer *display = godot::DisplayServer::get_singleton();
    if (display == nullptr || owner == nullptr || owner->get_window() == nullptr) {
        return false;
    }

    const int32_t popup_id = display->window_get_active_popup();
    const int32_t owner_window_id = owner->get_window()->get_window_id();
    if (popup_id == godot::DisplayServer::INVALID_WINDOW_ID ||
        popup_id == owner_window_id) {
        return false;
    }

    const godot::Vector2i popup_position = display->window_get_position(popup_id);
    const godot::Vector2i popup_size = display->window_get_size(popup_id);
    const godot::Vector2i owner_window_position =
        display->window_get_position(owner_window_id);
    const godot::Rect2 popup_rect(
        godot::Vector2(popup_position.x, popup_position.y),
        godot::Vector2(popup_size.x, popup_size.y));
    const godot::Rect2 local_owner_rect = transformed_control_rect(owner);
    const godot::Rect2 owner_rect(
        local_owner_rect.position +
            godot::Vector2(owner_window_position.x, owner_window_position.y),
        local_owner_rect.size);
    return rect_has_area(popup_rect) &&
           rect_has_area(owner_rect) &&
           popup_rect.intersects(owner_rect);
}

template <typename T>
void append_unique(std::vector<uint64_t> &ids, T *object) {
    if (object == nullptr) {
        return;
    }
    const uint64_t id = object->get_instance_id();
    if (std::find(ids.begin(), ids.end(), id) == ids.end()) {
        ids.push_back(id);
    }
}

} // namespace

void NativeWebviewOcclusionTracker::set_owner(godot::Control *owner) {
    const uint64_t next_owner_id =
        owner != nullptr ? owner->get_instance_id() : 0;
    if (owner_id == next_owner_id) {
        return;
    }

    reset();
    owner_id = next_owner_id;
}

bool NativeWebviewOcclusionTracker::update(double delta) {
    godot::Control *owner = control_from_id(owner_id);
    if (owner == nullptr || !owner->is_visible_in_tree()) {
        return true;
    }

    godot::SceneTree *tree = owner->get_tree();
    const int32_t node_count = tree != nullptr ? tree->get_node_count() : -1;
    refresh_remaining -= std::max(0.0, delta);
    if (node_count != last_node_count || refresh_remaining <= 0.0) {
        refresh_candidates(owner);
        refresh_remaining = kCandidateRefreshSeconds;
    }

    if (active_popup_overlaps(owner)) {
        return true;
    }

    for (uint64_t id : window_ids) {
        if (embedded_window_overlaps(window_from_id(id), owner)) {
            return true;
        }
    }

    for (uint64_t id : canvas_layer_ids) {
        godot::CanvasLayer *layer = canvas_layer_from_id(id);
        if (layer != nullptr &&
            layer->is_visible() &&
            layer->get_layer() >= 0 &&
            visible_control_overlaps(layer, owner)) {
            return true;
        }
    }

    for (uint64_t id : top_level_control_ids) {
        godot::Control *candidate = control_from_id(id);
        if (candidate != nullptr &&
            candidate->is_set_as_top_level() &&
            candidate->get_z_index() >= 0 &&
            controls_overlap(candidate, owner)) {
            return true;
        }
    }

    return false;
}

void NativeWebviewOcclusionTracker::reset() {
    owner_id = 0;
    last_node_count = -1;
    refresh_remaining = 0.0;
    top_level_control_ids.clear();
    canvas_layer_ids.clear();
    window_ids.clear();
}

void NativeWebviewOcclusionTracker::refresh_candidates(godot::Control *owner) {
    top_level_control_ids.clear();
    canvas_layer_ids.clear();
    window_ids.clear();

    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::Control *base_control =
        editor != nullptr ? editor->get_base_control() : nullptr;
    godot::Node *edited_scene_root =
        editor != nullptr ? editor->get_edited_scene_root() : nullptr;
    if (base_control != nullptr) {
        collect_candidates(base_control, owner, edited_scene_root);
    }

    godot::SceneTree *tree = owner != nullptr ? owner->get_tree() : nullptr;
    last_node_count = tree != nullptr ? tree->get_node_count() : -1;
    godot::Window *root = tree != nullptr ? tree->get_root() : nullptr;
    if (root == nullptr) {
        return;
    }

    append_unique(window_ids, root);
    const int32_t child_count = root->get_child_count();
    for (int32_t index = 0; index < child_count; index++) {
        godot::Node *child = root->get_child(index);
        if (child == nullptr ||
            child == base_control ||
            child == edited_scene_root) {
            continue;
        }
        if (godot::Object::cast_to<godot::CanvasLayer>(child) != nullptr ||
            godot::Object::cast_to<godot::Window>(child) != nullptr) {
            collect_candidates(child, owner, edited_scene_root);
        }
    }
}

void NativeWebviewOcclusionTracker::collect_candidates(
        godot::Node *node,
        godot::Control *owner,
        godot::Node *edited_scene_root) {
    if (node == nullptr ||
        node == edited_scene_root ||
        node == owner ||
        (owner != nullptr && owner->is_ancestor_of(node))) {
        return;
    }

    if (auto *window = godot::Object::cast_to<godot::Window>(node)) {
        append_unique(window_ids, window);
    }
    if (auto *layer = godot::Object::cast_to<godot::CanvasLayer>(node)) {
        append_unique(canvas_layer_ids, layer);
    }
    if (auto *control = godot::Object::cast_to<godot::Control>(node)) {
        if (control->is_set_as_top_level()) {
            append_unique(top_level_control_ids, control);
        }
    }

    const int32_t child_count = node->get_child_count();
    for (int32_t index = 0; index < child_count; index++) {
        collect_candidates(node->get_child(index), owner, edited_scene_root);
    }
}

} // namespace fennara::webview_backend

#endif
