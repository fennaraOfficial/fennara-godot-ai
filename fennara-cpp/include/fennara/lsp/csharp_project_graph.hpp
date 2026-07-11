#pragma once

#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

#include <atomic>

namespace fennara::csharp_project_graph {

// Evaluates the selected MSBuild project or solution and all ProjectReference
// edges. The returned graph contains the compiler's actual Compile items.
godot::Dictionary evaluate_selected(
    const std::atomic_bool *cancelled = nullptr);

// Returns the project paths in graph that compile absolute_file_path.
godot::Array owners_for_file(const godot::Dictionary &graph,
                             const godot::String &absolute_file_path);

// Drops the cached graph. Session teardown calls this when project context may
// have changed.
void invalidate();

} // namespace fennara::csharp_project_graph
