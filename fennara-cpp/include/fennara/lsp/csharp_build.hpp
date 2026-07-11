#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

#include <atomic>

namespace fennara::csharp_build {

void begin_build_lifecycle();
void request_build_shutdown();
godot::String find_root_csproj();
godot::Dictionary run_dotnet_build_if_needed();
godot::Dictionary run_diagnostics(const std::atomic_bool *cancelled = nullptr);
godot::Dictionary run_background_diagnostics(
    const std::atomic_bool *cancelled = nullptr);

// Records a C# source change only while the initial background diagnostic
// build is running. The next explicit project scan then performs one forced
// refresh to close the incremental-build timestamp race.
void note_csharp_source_changed();

} // namespace fennara::csharp_build
