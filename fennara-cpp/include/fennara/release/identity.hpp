#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

#include <optional>

namespace fennara::release_identity {

struct Identity {
    godot::String track;
    godot::String channel;
    godot::String version;
    godot::String release_tag;
    godot::String source_commit;

    bool is_staging() const;
};

std::optional<Identity> load_addon(const godot::String &expected_version,
                                   godot::String &error);
std::optional<Identity> parse(const godot::Dictionary &value,
                              const godot::String &expected_version,
                              godot::String &error);
bool manifest_matches(const godot::Dictionary &manifest, const Identity &expected,
                      godot::String &error);
godot::String channel_pointer_ref(const Identity &identity);
godot::String channel_pointer_name(const Identity &identity);

} // namespace fennara::release_identity
