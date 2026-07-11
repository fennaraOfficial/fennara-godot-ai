#include "fennara/update_notice.hpp"

#include "fennara/local_bridge.hpp"
#include "fennara/logger.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

#include <string>

namespace fennara::update_notice {
namespace {

bool g_checked = false;
bool g_update_available = false;
bool g_check_failed = false;
std::string g_current_version;
std::string g_latest_version;
std::string g_error;

int parse_part(const godot::String &part) {
    godot::String digits;
    for (int i = 0; i < part.length(); i++) {
        char32_t c = part[i];
        if (c < '0' || c > '9') {
            break;
        }
        digits += godot::String::chr(c);
    }
    return digits.is_empty() ? 0 : digits.to_int();
}

godot::String normalize_version(godot::String version) {
    version = version.strip_edges();
    if (version.begins_with("v")) {
        version = version.substr(1);
    }
    return version;
}

bool is_version_candidate(godot::String version) {
    version = normalize_version(version);
    godot::PackedStringArray parts = version.split(".");
    if (parts.size() < 2) {
        return false;
    }
    for (int i = 0; i < parts.size(); i++) {
        godot::String part = parts[i];
        if (part.is_empty()) {
            return false;
        }
        char32_t c = part[0];
        if (c < '0' || c > '9') {
            return false;
        }
    }
    return true;
}

godot::String extract_asset_version(const godot::String &name,
                                    const godot::String &prefix,
                                    const godot::String &suffix) {
    if (!name.begins_with(prefix) || !name.ends_with(suffix)) {
        return "";
    }
    int start = prefix.length();
    int count = name.length() - start - suffix.length();
    if (count <= 0) {
        return "";
    }
    godot::String version = normalize_version(name.substr(start, count));
    return is_version_candidate(version) ? version : godot::String();
}

godot::String latest_version_from_release(const godot::Dictionary &response) {
    godot::String tag = normalize_version(godot::String(response.get("tag_name", "")));
    if (is_version_candidate(tag)) {
        return tag;
    }

    godot::Variant assets_value = response.get("assets", godot::Array());
    if (assets_value.get_type() != godot::Variant::ARRAY) {
        return "";
    }

    godot::Array assets = assets_value;
    const godot::String prefixes[] = {
        "fennara-release-manifest-v",
        "fennara-release-addon-v",
        "fennara-release-local-linux-x86_64-v",
        "fennara-release-local-windows-x86_64-v",
        "fennara-release-local-macos-arm64-v",
        "fennara-cli-linux-x86_64-v",
        "fennara-cli-windows-x86_64-v",
        "fennara-cli-macos-arm64-v",
    };
    const godot::String suffixes[] = {
        ".json",
        ".zip",
        ".zip",
        ".zip",
        ".zip",
        ".zip",
        ".zip",
        ".zip",
    };

    for (int prefix_index = 0; prefix_index < 8; prefix_index++) {
        for (int asset_index = 0; asset_index < assets.size(); asset_index++) {
            godot::Variant asset_value = assets[asset_index];
            if (asset_value.get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            godot::Dictionary asset = asset_value;
            godot::String name = asset.get("name", "");
            godot::String version =
                extract_asset_version(name, prefixes[prefix_index], suffixes[prefix_index]);
            if (!version.is_empty()) {
                return version;
            }
        }
    }

    return "";
}

bool version_is_newer(const godot::String &latest, const godot::String &current) {
    godot::PackedStringArray latest_parts = latest.split(".");
    godot::PackedStringArray current_parts = current.split(".");
    int count = latest_parts.size() > current_parts.size()
                    ? latest_parts.size()
                    : current_parts.size();
    for (int i = 0; i < count; i++) {
        int latest_part = i < latest_parts.size() ? parse_part(latest_parts[i]) : 0;
        int current_part = i < current_parts.size() ? parse_part(current_parts[i]) : 0;
        if (latest_part > current_part) {
            return true;
        }
        if (latest_part < current_part) {
            return false;
        }
    }
    return false;
}

godot::String read_addon_version() {
    godot::String path = "res://addons/fennara/VERSION";
    if (!godot::FileAccess::file_exists(path)) {
        return FennaraLocalBridge::PLUGIN_VERSION;
    }
    godot::Ref<godot::FileAccess> file = godot::FileAccess::open(path, godot::FileAccess::READ);
    if (file.is_null()) {
        return FennaraLocalBridge::PLUGIN_VERSION;
    }
    godot::String version = file->get_as_text().strip_edges();
    return version.is_empty() ? godot::String(FennaraLocalBridge::PLUGIN_VERSION) : version;
}

void set_error(const godot::String &message) {
    g_check_failed = true;
    g_error = message.utf8().get_data();
}

} // namespace

bool begin_check() {
    if (g_checked) {
        return false;
    }
    g_checked = true;

    godot::String current = normalize_version(read_addon_version());
    g_current_version = current.utf8().get_data();
    return true;
}

void complete_check(bool success,
                    int response_code,
                    const godot::PackedByteArray &body,
                    const godot::String &error) {
    if (!success) {
        set_error(error.is_empty()
                      ? godot::String("Latest version request failed with HTTP status ") +
                            godot::String::num_int64(response_code)
                      : error);
        FLOG_TOOL("Update check skipped: latest release request failed");
        return;
    }

    const godot::Variant parsed = godot::JSON::parse_string(body.get_string_from_utf8());
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        set_error("GitHub response was not JSON.");
        return;
    }
    godot::Dictionary response = parsed;

    godot::String latest = latest_version_from_release(response);
    if (latest.is_empty()) {
        set_error("Latest release did not include a versioned tag or asset.");
        return;
    }

    g_latest_version = latest.utf8().get_data();
    g_update_available =
        version_is_newer(latest, godot::String(g_current_version.c_str()));
}

bool is_update_available() {
    return g_update_available;
}

godot::String current_version() {
    return godot::String(g_current_version.c_str());
}

godot::String latest_version() {
    return godot::String(g_latest_version.c_str());
}

godot::String warning_text() {
    if (!g_update_available) {
        return "";
    }
    return "Fennara is out of date. Current addon: " + current_version() +
           ". Latest release: " + latest_version() +
           ". Ask the user to run `fennara update` inside this Godot project or pass `--project <path>`.";
}

godot::Dictionary status() {
    godot::Dictionary result;
    result["checked"] = g_checked;
    result["check_failed"] = g_check_failed;
    result["current_version"] = current_version();
    result["latest_version"] = latest_version();
    result["outdated"] = g_update_available;
    result["message"] = g_update_available ? warning_text() : "";
    if (g_check_failed) {
        result["error"] = godot::String(g_error.c_str());
    }
    return result;
}

} // namespace fennara::update_notice
