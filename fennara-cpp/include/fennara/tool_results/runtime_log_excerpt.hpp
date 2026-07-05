#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

namespace fennara::tool_results {

void append_runtime_log_excerpt(godot::PackedStringArray &lines,
                                const godot::Dictionary &raw_result);

} // namespace fennara::tool_results
