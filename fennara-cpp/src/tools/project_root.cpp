#include "fennara/tools/project_root.hpp"

#include <godot_cpp/classes/project_settings.hpp>

#include <utility>

namespace fennara::project_root {

Resolution::Resolution(godot::ProjectSettings *settings, godot::String path) :
        _settings(settings), _path(std::move(path)) {}

bool Resolution::is_resolved() const {
    return _settings != nullptr && !_path.is_empty();
}

const godot::String &Resolution::path() const {
    return _path;
}

godot::String Resolution::globalize_path(const godot::String &path) const {
    return _settings == nullptr ? godot::String() : _settings->globalize_path(path);
}

godot::String Resolution::error_message(bool distinguish_missing_settings) const {
    if (distinguish_missing_settings && _settings == nullptr) {
        return "ProjectSettings is unavailable.";
    }
    return "Could not resolve the current Godot project root.";
}

Resolution resolve() {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    return Resolution(
        settings,
        settings == nullptr ? godot::String() : settings->globalize_path("res://"));
}

} // namespace fennara::project_root
