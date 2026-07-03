#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara::get_class_info {

godot::String fallback_docs_branch();
godot::String docs_branch_from_version_info(const godot::Dictionary &version_info);
godot::String docs_branch_for_running_godot();

} // namespace fennara::get_class_info
