#pragma once

#if defined(_WIN32) || defined(__APPLE__)

#include <cstdint>
#include <vector>

namespace godot {
class Control;
class Node;
}

namespace fennara::webview_backend {

class NativeWebviewOcclusionTracker {
public:
    void set_owner(godot::Control *owner);
    bool update(double delta);
    void reset();

private:
    void refresh_candidates(godot::Control *owner);
    void collect_candidates(godot::Node *node,
                            godot::Control *owner,
                            godot::Node *edited_scene_root,
                            bool inside_canvas_layer = false);

    uint64_t owner_id = 0;
    int32_t last_node_count = -1;
    double refresh_remaining = 0.0;
    double structure_refresh_remaining = 0.0;
    std::vector<uint64_t> top_level_control_ids;
    std::vector<uint64_t> canvas_layer_control_ids;
    std::vector<uint64_t> window_ids;
};

} // namespace fennara::webview_backend

#endif
