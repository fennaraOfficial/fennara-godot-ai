#include "fennara/control_auth.hpp"

#include "fennara/app_paths.hpp"

#include <godot_cpp/classes/crypto.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/http_client.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/marshalls.hpp>
#include <godot_cpp/classes/time.hpp>

#include <chrono>
#include <thread>

namespace fennara::control_auth {
namespace {

constexpr const char *kDaemonHost = "127.0.0.1";
constexpr int kDaemonPort = 41287;
constexpr int kChallengeBytes = 32;
constexpr int kChallengeTimeoutMs = 2000;
constexpr int kMaxChallengeResponseBytes = 4096;

godot::String base64_url_encode(const godot::PackedByteArray &bytes) {
    return godot::Marshalls::get_singleton()
        ->raw_to_base64(bytes)
        .replace("+", "-")
        .replace("/", "_")
        .replace("=", "");
}

godot::PackedByteArray base64_url_decode(godot::String value) {
    value = value.replace("-", "+").replace("_", "/");
    while (value.length() % 4 != 0) {
        value += "=";
    }
    return godot::Marshalls::get_singleton()->base64_to_raw(value);
}

godot::Dictionary request_json(godot::HTTPClient::Method method,
                               const godot::String &path) {
    godot::Ref<godot::HTTPClient> http;
    http.instantiate();
    if (http.is_null() || http->connect_to_host(kDaemonHost, kDaemonPort) != godot::OK) {
        return godot::Dictionary();
    }

    const uint64_t deadline = godot::Time::get_singleton()->get_ticks_msec() + kChallengeTimeoutMs;
    bool request_sent = false;
    godot::String response_body;
    while (godot::Time::get_singleton()->get_ticks_msec() < deadline) {
        http->poll();
        const godot::HTTPClient::Status status = http->get_status();
        if (status == godot::HTTPClient::STATUS_CANT_CONNECT ||
            status == godot::HTTPClient::STATUS_CONNECTION_ERROR) {
            return godot::Dictionary();
        }
        if (status == godot::HTTPClient::STATUS_CONNECTED && !request_sent) {
            godot::PackedStringArray headers;
            headers.append("Accept: application/json");
            if (http->request(method, path, headers) != godot::OK) {
                return godot::Dictionary();
            }
            request_sent = true;
        }
        if (status == godot::HTTPClient::STATUS_BODY) {
            const godot::PackedByteArray chunk = http->read_response_body_chunk();
            if (!chunk.is_empty()) {
                if (response_body.to_utf8_buffer().size() + chunk.size() >
                    kMaxChallengeResponseBytes) {
                    return godot::Dictionary();
                }
                response_body += chunk.get_string_from_utf8();
            }
            if (http->get_status() != godot::HTTPClient::STATUS_BODY && http->has_response()) {
                break;
            }
        } else if (request_sent && status == godot::HTTPClient::STATUS_CONNECTED &&
                   http->has_response()) {
            break;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(5));
    }
    if (http->get_response_code() != 200) {
        return godot::Dictionary();
    }

    const godot::Variant parsed = godot::JSON::parse_string(response_body);
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        return godot::Dictionary();
    }
    return parsed;
}

bool verify_daemon(const godot::PackedByteArray &key) {
    godot::Ref<godot::Crypto> crypto;
    crypto.instantiate();
    if (crypto.is_null()) {
        return false;
    }
    const godot::PackedByteArray nonce = crypto->generate_random_bytes(kChallengeBytes);
    if (nonce.size() != kChallengeBytes) {
        return false;
    }

    const godot::String path = "/control/challenge?nonce=" + base64_url_encode(nonce);
    const godot::Dictionary response = request_json(godot::HTTPClient::METHOD_GET, path);
    const godot::PackedByteArray proof =
        base64_url_decode(godot::String(response.get("proof", "")));
    const godot::PackedByteArray expected = crypto->hmac_digest(
        godot::HashingContext::HASH_SHA256, key, nonce);
    return expected.size() == kChallengeBytes &&
           crypto->constant_time_compare(expected, proof);
}

} // namespace

godot::String verified_daemon_header() {
    const godot::String path = app_paths::daemon_control_token_path();
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(path, godot::FileAccess::READ);
    if (file.is_null()) {
        return "";
    }
    const godot::String token = file->get_as_text().strip_edges();
    if (token.is_empty() || token.contains("\r") || token.contains("\n")) {
        return "";
    }
    const godot::PackedByteArray key = base64_url_decode(token);
    if (key.size() != kChallengeBytes || !verify_daemon(key)) {
        return "";
    }
    return "X-Fennara-Control-Token: " + token;
}

void request_legacy_daemon_shutdown() {
    const godot::Dictionary health =
        request_json(godot::HTTPClient::METHOD_GET, "/health");
    if (godot::String(health.get("daemon", "")) != "fennara-daemon") {
        return;
    }
    request_json(godot::HTTPClient::METHOD_POST, "/shutdown");
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
}

bool verify_daemon_and_append_header(godot::PackedStringArray &headers) {
    const godot::String header = verified_daemon_header();
    if (header.is_empty()) {
        return false;
    }
    headers.append(header);
    return true;
}

} // namespace fennara::control_auth
