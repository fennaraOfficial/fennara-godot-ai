#pragma once

#include "fennara/release/identity.hpp"

#include <godot_cpp/variant/string.hpp>

#include <atomic>

namespace fennara::release_discovery {

struct Result {
    bool success = false;
    bool cancelled = false;
    bool update_available = false;
    release_identity::Identity current;
    godot::String target_version;
    godot::String target_release_tag;
    godot::String target_source_commit;
    godot::String target_manifest_sha256;
    godot::String detail;
    godot::String error;
};

Result check(int timeout_ms, const std::atomic_bool *cancelled = nullptr);

} // namespace fennara::release_discovery
