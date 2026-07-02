#pragma once

#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara {

// Legacy tool kept for backward compatibility with older clients.
// Do not advertise for new model/tool selection; prefer run_scene_edit_script
// with Godot ResourceLoader/ResourceSaver APIs for resource workflows.
class FennaraSaveCustomResourceTool : public godot::RefCounted {
    GDCLASS(FennaraSaveCustomResourceTool, godot::RefCounted)

  protected:
    static void _bind_methods();

  public:
    static godot::Dictionary execute(const godot::Dictionary &args);

  private:
    static godot::Dictionary _stamp_result(godot::Dictionary result,
                                           const godot::Dictionary &args);
    static godot::String
    _get_script_path_for_class_name(const godot::String &class_type);
};

} // namespace fennara
