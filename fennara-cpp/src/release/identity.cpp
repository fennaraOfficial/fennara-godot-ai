#include "fennara/release/identity.hpp"

#include "fennara/release/version.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>

namespace fennara::release_identity {
namespace {

constexpr const char *kAddonIdentityPath = "res://addons/fennara/release.json";

bool valid_channel(const godot::String &channel) {
    if (!channel.begins_with("pr-")) {
        return false;
    }
    const godot::String number = channel.substr(3);
    return !number.is_empty() && !number.begins_with("0") && number.is_valid_int() &&
           number.to_int() > 0;
}

bool valid_source_commit(const godot::String &value) {
    if (value.length() != 40 || value != value.to_lower()) {
        return false;
    }
    for (int index = 0; index < value.length(); index++) {
        const char32_t character = value[index];
        if (!((character >= '0' && character <= '9') ||
              (character >= 'a' && character <= 'f'))) {
            return false;
        }
    }
    return true;
}

Identity legacy_stable(const godot::String &version) {
    Identity identity;
    identity.track = "stable";
    identity.version = version;
    identity.release_tag = "v" + version;
    return identity;
}

} // namespace

bool Identity::is_staging() const {
    return track == "staging";
}

std::optional<Identity> load_addon(const godot::String &expected_version,
                                   godot::String &error) {
    if (!godot::FileAccess::file_exists(kAddonIdentityPath)) {
        if (!release_version::is_valid(expected_version) || expected_version.contains("-")) {
            error = "A prerelease addon requires release.json identity metadata.";
            return std::nullopt;
        }
        return legacy_stable(expected_version);
    }
    const godot::Variant parsed = godot::JSON::parse_string(
        godot::FileAccess::get_file_as_string(kAddonIdentityPath));
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        error = "The addon release identity is not valid JSON.";
        return std::nullopt;
    }
    return parse(parsed, expected_version, error);
}

std::optional<Identity> parse(const godot::Dictionary &value,
                              const godot::String &expected_version,
                              godot::String &error) {
    Identity identity;
    const godot::Variant schema_version = value.get("schema_version", godot::Variant());
    const godot::Variant::Type schema_type = schema_version.get_type();
    if ((schema_type != godot::Variant::INT && schema_type != godot::Variant::FLOAT) ||
        (double)schema_version != 1.0) {
        error = "The addon release identity uses an unsupported schema.";
        return std::nullopt;
    }
    identity.track = value.get("track", "");
    identity.channel = value.get("channel", "");
    identity.version = value.get("version", "");
    identity.release_tag = value.get("release_tag", "");
    identity.source_commit = value.get("source_commit", "");
    if (!release_version::is_valid(identity.version) || identity.version != expected_version ||
        identity.release_tag != "v" + identity.version) {
        error = "The addon release identity does not match its VERSION and release tag.";
        return std::nullopt;
    }
    if (identity.track == "stable") {
        if (identity.version.contains("-") || !identity.channel.is_empty()) {
            error = "Stable addon release identity contains staging fields.";
            return std::nullopt;
        }
        if (!identity.source_commit.is_empty() && !valid_source_commit(identity.source_commit)) {
            error = "Stable addon release identity contains an invalid source commit.";
            return std::nullopt;
        }
        return identity;
    }
    if (identity.track != "staging" || !valid_channel(identity.channel) ||
        !valid_source_commit(identity.source_commit)) {
        error = "Staging addon release identity is incomplete or invalid.";
        return std::nullopt;
    }
    const godot::String pull_request = identity.channel.substr(3);
    const int prerelease_index = identity.version.find("-");
    const godot::String prefix = "pr." + pull_request + ".";
    const godot::String prerelease = prerelease_index >= 0
                                         ? identity.version.substr(prerelease_index + 1)
                                         : godot::String();
    if (!prerelease.begins_with(prefix) || prerelease.length() <= prefix.length()) {
        error = "The staging addon version does not belong to its channel.";
        return std::nullopt;
    }
    const godot::String candidate = prerelease.substr(prefix.length());
    if (!candidate.is_valid_int() || candidate.begins_with("0") || candidate.to_int() <= 0) {
        error = "The staging addon candidate number is invalid.";
        return std::nullopt;
    }
    return identity;
}

bool manifest_matches(const godot::Dictionary &manifest, const Identity &expected,
                      godot::String &error) {
    const godot::Variant release_value = manifest.get("release", godot::Variant());
    if (release_value.get_type() == godot::Variant::NIL && !expected.is_staging()) {
        return true;
    }
    if (release_value.get_type() != godot::Variant::DICTIONARY) {
        error = "The staging release manifest is missing release identity metadata.";
        return false;
    }
    std::optional<Identity> actual = parse(release_value, expected.version, error);
    if (!actual.has_value() || actual->track != expected.track ||
        actual->channel != expected.channel || actual->version != expected.version ||
        actual->release_tag != expected.release_tag ||
        actual->source_commit != expected.source_commit) {
        if (error.is_empty()) {
            error = "The release manifest identity does not match this addon.";
        }
        return false;
    }
    return true;
}

godot::String channel_pointer_ref(const Identity &identity) {
    return identity.is_staging() ? "fennara-staging/" + identity.channel : godot::String();
}

godot::String channel_pointer_name(const Identity &identity) {
    return identity.is_staging()
               ? "fennara-staging-channel-" + identity.channel + ".json"
               : godot::String();
}

} // namespace fennara::release_identity
