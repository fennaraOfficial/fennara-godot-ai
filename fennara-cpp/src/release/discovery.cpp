#include "fennara/release/discovery.hpp"

#include "fennara/app_paths.hpp"
#include "fennara/local_bridge.hpp"
#include "fennara/release/version.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/http_client.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/classes/tls_options.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

namespace fennara::release_discovery {
namespace {

constexpr const char *kLatestReleasePath =
    "/repos/fennaraOfficial/fennara-godot-ai/releases/latest";
constexpr uint64_t kChannelCacheSeconds = 300;

struct HttpResponse {
    int code = 0;
    bool cancelled = false;
    godot::String body;
    godot::String etag;
    godot::String error;
};

struct ChannelCache {
    bool valid = false;
    int code = 0;
    uint64_t checked_at = 0;
    godot::String etag;
    godot::String body;
};

bool is_cancelled(const std::atomic_bool *cancelled) {
    return cancelled != nullptr && cancelled->load(std::memory_order_acquire);
}

godot::String addon_version() {
    const godot::String path = "res://addons/fennara/VERSION";
    const godot::String value =
        godot::FileAccess::file_exists(path)
            ? godot::FileAccess::get_file_as_string(path).strip_edges()
            : godot::String();
    return value.is_empty() ? godot::String(FennaraLocalBridge::PLUGIN_VERSION) : value;
}

HttpResponse request_github(const godot::String &path, const godot::String &accept,
                            int timeout_ms, const godot::String &etag,
                            const std::atomic_bool *cancelled) {
    HttpResponse result;
    if (is_cancelled(cancelled)) {
        result.cancelled = true;
        result.error = "Update check cancelled.";
        return result;
    }
    godot::PackedByteArray response_body;
    godot::Ref<godot::HTTPClient> http;
    http.instantiate();
    if (http->connect_to_host("api.github.com", 443,
                              godot::TLSOptions::client()) != godot::OK) {
        result.error = "Failed to connect to GitHub.";
        return result;
    }
    godot::PackedStringArray headers;
    headers.append("Accept: " + accept);
    headers.append("User-Agent: fennara-godot-ai");
    if (!etag.is_empty()) {
        headers.append("If-None-Match: " + etag);
    }
    const uint64_t deadline =
        godot::Time::get_singleton()->get_ticks_msec() + static_cast<uint64_t>(timeout_ms);
    bool sent = false;
    bool response_complete = false;
    while (godot::Time::get_singleton()->get_ticks_msec() < deadline) {
        if (is_cancelled(cancelled)) {
            result.cancelled = true;
            result.error = "Update check cancelled.";
            return result;
        }
        http->poll();
        const godot::HTTPClient::Status status = http->get_status();
        if (status == godot::HTTPClient::STATUS_CANT_CONNECT ||
            status == godot::HTTPClient::STATUS_TLS_HANDSHAKE_ERROR ||
            status == godot::HTTPClient::STATUS_CONNECTION_ERROR) {
            result.error = "Failed to connect to GitHub.";
            return result;
        }
        if (status == godot::HTTPClient::STATUS_CONNECTED && !sent) {
            if (http->request(godot::HTTPClient::METHOD_GET, path, headers) != godot::OK) {
                result.error = "Failed to send the GitHub release request.";
                return result;
            }
            sent = true;
        }
        if (status == godot::HTTPClient::STATUS_BODY) {
            const godot::PackedByteArray chunk = http->read_response_body_chunk();
            if (!chunk.is_empty()) {
                response_body.append_array(chunk);
            }
            if (http->get_response_body_length() >= 0 &&
                response_body.size() >= http->get_response_body_length()) {
                response_complete = true;
                break;
            }
        }
        if (sent && http->has_response() &&
            (http->get_response_body_length() == 0 ||
             status == godot::HTTPClient::STATUS_CONNECTED)) {
            response_complete = true;
            break;
        }
        godot::OS::get_singleton()->delay_usec(10000);
    }
    if (!response_complete) {
        result.error = http->has_response() ? "Timed out reading the GitHub response."
                                            : "Timed out waiting for GitHub.";
        return result;
    }
    result.body = response_body.get_string_from_utf8();
    result.code = http->get_response_code();
    const godot::PackedStringArray response_headers = http->get_response_headers();
    for (int index = 0; index < response_headers.size(); index++) {
        const godot::String header = response_headers[index];
        if (header.to_lower().begins_with("etag:")) {
            result.etag = header.substr(header.find(":") + 1).strip_edges();
            break;
        }
    }
    return result;
}

godot::String channel_cache_path(const godot::String &channel) {
    const godot::String root = app_paths::app_dir();
    return root.is_empty()
               ? godot::String()
               : root.path_join("cache")
                     .path_join("update-channels")
                     .path_join(channel + godot::String(".json"));
}

ChannelCache load_channel_cache(const godot::String &channel) {
    ChannelCache result;
    const godot::String path = channel_cache_path(channel);
    if (path.is_empty() || !godot::FileAccess::file_exists(path)) {
        return result;
    }
    const godot::Variant parsed =
        godot::JSON::parse_string(godot::FileAccess::get_file_as_string(path));
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        return result;
    }
    const godot::Dictionary value = parsed;
    if ((int64_t)value.get("schema_version", 0) != 1 ||
        godot::String(value.get("channel", "")) != channel) {
        return result;
    }
    result.code = value.get("response_code", 0);
    result.checked_at = value.get("checked_at_unix", 0);
    result.etag = value.get("etag", "");
    result.body = value.get("body", "");
    result.valid = (result.code == 200 && !result.body.is_empty()) || result.code == 404;
    return result;
}

void save_channel_cache(const godot::String &channel, int code, const godot::String &etag,
                        const godot::String &body) {
    godot::Dictionary value;
    value["schema_version"] = 1;
    value["channel"] = channel;
    value["response_code"] = code;
    value["checked_at_unix"] = static_cast<int64_t>(
        godot::Time::get_singleton()->get_unix_time_from_system());
    value["etag"] = etag;
    value["body"] = body;
    app_paths::write_json(channel_cache_path(channel), value);
}

bool cache_is_fresh(const ChannelCache &cache) {
    const uint64_t now = static_cast<uint64_t>(
        godot::Time::get_singleton()->get_unix_time_from_system());
    return cache.valid && now >= cache.checked_at &&
           now - cache.checked_at < kChannelCacheSeconds;
}

godot::String extract_asset_version(const godot::String &name,
                                    const godot::String &prefix,
                                    const godot::String &suffix) {
    if (!name.begins_with(prefix) || !name.ends_with(suffix)) {
        return "";
    }
    const int count = name.length() - prefix.length() - suffix.length();
    const godot::String version =
        count > 0 ? release_version::normalize(name.substr(prefix.length(), count))
                  : godot::String();
    return release_version::is_valid(version) ? version : godot::String();
}

godot::String stable_version(const godot::Dictionary &release) {
    const godot::String tag =
        release_version::normalize(godot::String(release.get("tag_name", "")));
    if (release_version::is_valid(tag)) {
        return tag;
    }
    const godot::Variant assets_value = release.get("assets", godot::Array());
    if (assets_value.get_type() != godot::Variant::ARRAY) {
        return "";
    }
    const godot::String prefixes[] = {
        "fennara-release-manifest-v", "fennara-release-addon-v",
        "fennara-release-local-linux-x86_64-v",
        "fennara-release-local-windows-x86_64-v",
        "fennara-release-local-macos-arm64-v", "fennara-cli-linux-x86_64-v",
        "fennara-cli-windows-x86_64-v", "fennara-cli-macos-arm64-v",
    };
    const godot::String suffixes[] = {".json", ".zip", ".zip", ".zip",
                                      ".zip",  ".zip", ".zip", ".zip"};
    const godot::Array assets = assets_value;
    for (int prefix_index = 0; prefix_index < 8; prefix_index++) {
        for (int asset_index = 0; asset_index < assets.size(); asset_index++) {
            const godot::Variant asset_value = assets[asset_index];
            if (asset_value.get_type() != godot::Variant::DICTIONARY) {
                continue;
            }
            const godot::String version = extract_asset_version(
                godot::Dictionary(asset_value).get("name", ""), prefixes[prefix_index],
                suffixes[prefix_index]);
            if (!version.is_empty()) {
                return version;
            }
        }
    }
    return "";
}

Result stable_result(const release_identity::Identity &current, int timeout_ms,
                     const std::atomic_bool *cancelled) {
    Result result;
    result.current = current;
    const HttpResponse response =
        request_github(kLatestReleasePath, "application/vnd.github+json", timeout_ms, "",
                       cancelled);
    result.cancelled = response.cancelled;
    if (!response.error.is_empty() || response.code != 200) {
        result.error = response.error.is_empty() ? "Stable release lookup failed." : response.error;
        return result;
    }
    const godot::Variant parsed = godot::JSON::parse_string(response.body);
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        result.error = "Stable release metadata was not valid JSON.";
        return result;
    }
    result.target_version = stable_version(parsed);
    if (result.target_version.is_empty()) {
        result.error = "Stable release metadata did not contain a version.";
        return result;
    }
    result.target_release_tag = "v" + result.target_version;
    result.update_available =
        release_version::compare(result.target_version, current.version).value_or(0) > 0;
    result.success = true;
    return result;
}

Result staging_result(const release_identity::Identity &current, int timeout_ms,
                      const std::atomic_bool *cancelled) {
    Result result;
    result.current = current;
    const godot::String pointer_name = release_identity::channel_pointer_name(current);
    const godot::String reference = release_identity::channel_pointer_ref(current);
    const godot::String encoded_reference = reference.replace("/", "%2F");
    const godot::String path = "/repos/fennaraOfficial/fennara-godot-ai/contents/" +
                               pointer_name + "?ref=" + encoded_reference;
    const ChannelCache cache = load_channel_cache(current.channel);
    HttpResponse response;
    bool refresh_cache = false;
    if (cache_is_fresh(cache)) {
        response.code = cache.code;
        response.body = cache.body;
        response.etag = cache.etag;
    } else {
        refresh_cache = true;
        response = request_github(path, "application/vnd.github.raw+json", timeout_ms,
                                  cache.valid ? cache.etag : godot::String(), cancelled);
        if (response.code == 304 && cache.valid) {
            response.code = cache.code;
            response.body = cache.body;
            response.etag = cache.etag;
        }
    }
    result.cancelled = response.cancelled;
    if (response.code == 404) {
        if (refresh_cache) {
            save_channel_cache(current.channel, 404, response.etag, "");
        }
        result.success = true;
        result.detail = "No staging updates are available for " + current.channel + ".";
        return result;
    }
    if (!response.error.is_empty() || response.code != 200) {
        result.error = response.error.is_empty() ? "Staging channel lookup failed." : response.error;
        return result;
    }
    const godot::Variant parsed = godot::JSON::parse_string(response.body);
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        result.error = "The staging channel pointer is not valid JSON.";
        return result;
    }
    const godot::Dictionary pointer = parsed;
    if ((int64_t)pointer.get("schema_version", 0) != 1 ||
        godot::String(pointer.get("channel", "")) != current.channel) {
        result.error = "The staging channel pointer does not match this addon channel.";
        return result;
    }
    godot::Dictionary identity_value;
    identity_value["schema_version"] = 1;
    identity_value["track"] = "staging";
    identity_value["channel"] = pointer.get("channel", "");
    identity_value["version"] = pointer.get("version", "");
    identity_value["release_tag"] = pointer.get("release_tag", "");
    identity_value["source_commit"] = pointer.get("source_commit", "");
    godot::String identity_error;
    const godot::String target_version = pointer.get("version", "");
    const std::optional<release_identity::Identity> target =
        release_identity::parse(identity_value, target_version, identity_error);
    const godot::String manifest_hash = pointer.get("release_manifest_sha256", "");
    if (!target.has_value() || target->channel != current.channel ||
        manifest_hash.length() != 64 || manifest_hash != manifest_hash.to_lower() ||
        !manifest_hash.is_valid_hex_number(false)) {
        result.error = identity_error.is_empty() ? "The staging channel pointer is invalid."
                                                : identity_error;
        return result;
    }
    result.target_version = target->version;
    result.target_release_tag = target->release_tag;
    result.target_source_commit = target->source_commit;
    result.target_manifest_sha256 = manifest_hash;
    result.update_available =
        release_version::compare(target->version, current.version).value_or(0) > 0;
    if (refresh_cache) {
        save_channel_cache(current.channel, 200, response.etag, response.body);
    }
    result.success = true;
    return result;
}

} // namespace

Result check(int timeout_ms, const std::atomic_bool *cancelled) {
    Result result;
    if (is_cancelled(cancelled)) {
        result.cancelled = true;
        result.error = "Update check cancelled.";
        return result;
    }
    const godot::String version = release_version::normalize(addon_version());
    godot::String identity_error;
    const std::optional<release_identity::Identity> identity =
        release_identity::load_addon(version, identity_error);
    if (!identity.has_value()) {
        result.error = identity_error;
        return result;
    }
    return identity->is_staging() ? staging_result(*identity, timeout_ms, cancelled)
                                  : stable_result(*identity, timeout_ms, cancelled);
}

} // namespace fennara::release_discovery
