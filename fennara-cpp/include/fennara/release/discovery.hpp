#pragma once

#include "fennara/release/identity.hpp"

#include <godot_cpp/variant/string.hpp>

namespace fennara::release_discovery {

struct Result {
    bool success = false;
    bool update_available = false;
    release_identity::Identity current;
    godot::String target_version;
    godot::String target_release_tag;
    godot::String target_source_commit;
    godot::String target_manifest_sha256;
    godot::String detail;
    godot::String error;
};

Result check(int timeout_ms);

} // namespace fennara::release_discovery
