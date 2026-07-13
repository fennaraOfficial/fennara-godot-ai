#pragma once

#include <godot_cpp/variant/string.hpp>

#include <optional>

namespace fennara::release_version {

godot::String normalize(godot::String version);
bool is_valid(const godot::String &version);
std::optional<int> compare(const godot::String &left, const godot::String &right);

} // namespace fennara::release_version
