#pragma once

#include <godot_cpp/variant/string.hpp>

namespace godot {
class ProjectSettings;
}

namespace fennara::project_root {

class Resolution {
public:
    bool is_resolved() const;
    const godot::String &path() const;
    godot::String globalize_path(const godot::String &path) const;
    godot::String error_message(bool distinguish_missing_settings = false) const;

private:
    friend Resolution resolve();

    Resolution(godot::ProjectSettings *settings, godot::String path);

    godot::ProjectSettings *_settings = nullptr;
    godot::String _path;
};

Resolution resolve();

} // namespace fennara::project_root
