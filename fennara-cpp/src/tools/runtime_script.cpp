#include "fennara/tools/runtime_script.hpp"

#include "fennara/logger.hpp"
#include "fennara/runtime/runtime_script_diagnostics.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/http_client.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/classes/time.hpp>

#include <algorithm>
#include <chrono>
#include <thread>

namespace fennara {
namespace {

constexpr const char *kResultVersion = "runtime-script-result-v1";
constexpr const char *kRuntimeScriptDir = "res://.fennara/tmp/runtime_scripts";
constexpr const char *kLocalDaemonHost = "127.0.0.1";
constexpr int kLocalDaemonPort = 41287;
constexpr int64_t kDefaultScriptTimeoutMs = 30000;
constexpr int64_t kMinScriptTimeoutMs = 500;
constexpr int64_t kMaxScriptTimeoutMs = 120000;
constexpr int64_t kHttpTimeoutMarginMs = 5000;

godot::Dictionary make_error(const godot::String &message) {
    godot::Dictionary result;
    result["success"] = false;
    result["tool_name"] = "runtime_script";
    result["format_version"] = kResultVersion;
    result["status"] = "blocked";
    result["error"] = message;
    return result;
}

godot::String make_script_run_id() {
    godot::Time *time = godot::Time::get_singleton();
    uint64_t ticks = time == nullptr ? 0 : time->get_ticks_usec();
    return "script_" + godot::String::num_uint64(ticks);
}

bool ensure_tmp_dir(godot::Dictionary &error) {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        error = make_error("ProjectSettings is unavailable.");
        return false;
    }
    godot::String abs_dir = settings->globalize_path(kRuntimeScriptDir);
    godot::Error dir_err = godot::DirAccess::make_dir_recursive_absolute(abs_dir);
    if (dir_err != godot::OK) {
        error = make_error("Could not create runtime script directory: " + abs_dir);
        return false;
    }
    return true;
}

godot::Dictionary save_inline_code(const godot::String &code,
                                   godot::String &script_path) {
    godot::Dictionary error;
    if (!ensure_tmp_dir(error)) {
        return error;
    }
    script_path = godot::String(kRuntimeScriptDir).path_join(
        make_script_run_id() + ".gd");
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(script_path, godot::FileAccess::WRITE);
    if (file.is_null()) {
        return make_error("Could not write runtime script: " + script_path);
    }
    file->store_string(code);
    if (!code.ends_with("\n")) {
        file->store_string("\n");
    }
    return godot::Dictionary();
}

godot::String read_script_text(const godot::String &script_path) {
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(script_path, godot::FileAccess::READ);
    return file.is_null() ? godot::String() : file->get_as_text();
}

godot::Dictionary validate_contract(const godot::String &script_path) {
    godot::String text = read_script_text(script_path);
    if (text.is_empty()) {
        return make_error("Runtime script is empty or could not be read: " + script_path);
    }
    if (text.find("@tool") >= 0) {
        return make_error("Runtime scripts must not use @tool.");
    }
    if (text.find("extends RefCounted") < 0) {
        return make_error("Runtime script must extend RefCounted.");
    }
    if (text.find("func run(ctx)") < 0 && text.find("func run(ctx:") < 0) {
        return make_error("Runtime script must define func run(ctx) -> void.");
    }
    return godot::Dictionary();
}

uint64_t now_ms() {
    godot::Time *time = godot::Time::get_singleton();
    return time == nullptr ? 0 : static_cast<uint64_t>(time->get_ticks_msec());
}

godot::Dictionary post_daemon(const godot::String &path,
                              const godot::Dictionary &payload,
                              int timeout_ms) {
    godot::Dictionary result;
    godot::Ref<godot::HTTPClient> http;
    http.instantiate();
    if (http.is_null() ||
        http->connect_to_host(kLocalDaemonHost, kLocalDaemonPort) != godot::OK) {
        return make_error("Failed to connect to local Fennara daemon.");
    }

    godot::PackedStringArray headers;
    headers.append("Content-Type: application/json");
    headers.append("Accept: application/json");
    godot::PackedByteArray body = godot::JSON::stringify(payload).to_utf8_buffer();
    uint64_t deadline = now_ms() + static_cast<uint64_t>(timeout_ms);
    bool sent = false;
    godot::String response_body;
    while (now_ms() < deadline) {
        http->poll();
        godot::HTTPClient::Status status = http->get_status();
        if (status == godot::HTTPClient::STATUS_CONNECTED && !sent) {
            if (http->request_raw(godot::HTTPClient::METHOD_POST, path, headers, body) != godot::OK) {
                return make_error("Failed to send daemon request.");
            }
            sent = true;
        }
        if (status == godot::HTTPClient::STATUS_BODY) {
            godot::PackedByteArray chunk = http->read_response_body_chunk();
            if (!chunk.is_empty()) {
                response_body += chunk.get_string_from_utf8();
            }
            if (http->get_status() != godot::HTTPClient::STATUS_BODY && http->has_response()) {
                break;
            }
        } else if (sent && status == godot::HTTPClient::STATUS_CONNECTED && http->has_response()) {
            break;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }

    godot::Variant parsed = godot::JSON::parse_string(response_body);
    if (parsed.get_type() == godot::Variant::DICTIONARY) {
        result = parsed;
    } else {
        result["response_body"] = response_body;
    }
    result["success"] = (bool)result.get("ok", false);
    return result;
}

} // namespace

void FennaraRuntimeScriptTool::_bind_methods() {
    godot::ClassDB::bind_static_method(
        "FennaraRuntimeScriptTool",
        godot::D_METHOD("submit", "args"),
        &FennaraRuntimeScriptTool::submit);
}

godot::Dictionary FennaraRuntimeScriptTool::submit(const godot::Dictionary &args) {
    godot::String requested_session_id =
        godot::String(args.get("session_id", "")).strip_edges();
    if (requested_session_id.is_empty()) {
        return make_error("`session_id` is required. Use the session id returned by runtime_session.start/status.");
    }

    bool has_code = args.has("code") &&
                    !godot::String(args.get("code", "")).strip_edges().is_empty();
    bool has_script_path = args.has("script_path") &&
                           !godot::String(args.get("script_path", "")).strip_edges().is_empty();
    if (has_code == has_script_path) {
        return make_error("Pass exactly one of `code` or `script_path`.");
    }

    godot::String script_path;
    if (has_code) {
        godot::Dictionary save_error =
            save_inline_code(godot::String(args.get("code", "")), script_path);
        if (!save_error.is_empty()) {
            return save_error;
        }
    } else {
        script_path = godot::String(args.get("script_path", "")).strip_edges();
        if (!script_path.begins_with("res://") || !script_path.ends_with(".gd")) {
            return make_error("`script_path` must be a res:// path to a .gd file.");
        }
        if (!godot::FileAccess::file_exists(script_path)) {
            return make_error("Runtime script does not exist: " + script_path);
        }
    }

    godot::Dictionary contract_error = validate_contract(script_path);
    if (!contract_error.is_empty()) {
        contract_error["script_path"] = script_path;
        return contract_error;
    }

    godot::Dictionary diagnostics = runtime_script_diagnostics::check(script_path);
    if (runtime_script_diagnostics::has_blocking_error(diagnostics)) {
        godot::Dictionary blocked =
            make_error("Runtime script diagnostics failed. Patch the saved script_path and rerun.");
        blocked["script_path"] = script_path;
        runtime_script_diagnostics::apply_to_result(diagnostics, blocked);
        return blocked;
    }

    godot::String script_run_id = make_script_run_id();
    int64_t requested_timeout_ms = static_cast<int64_t>(
        args.get("timeout_ms", kDefaultScriptTimeoutMs));
    requested_timeout_ms = std::clamp(
        requested_timeout_ms, kMinScriptTimeoutMs, kMaxScriptTimeoutMs);
    godot::Dictionary payload;
    payload["session_id"] = requested_session_id;
    payload["script_run_id"] = script_run_id;
    payload["script_path"] = script_path;
    payload["timeout_ms"] = requested_timeout_ms;

    int http_timeout_ms = static_cast<int>(
        std::min(requested_timeout_ms + kHttpTimeoutMarginMs,
                 kMaxScriptTimeoutMs + kHttpTimeoutMarginMs));
    godot::Dictionary result =
        post_daemon("/runtime/session/script", payload, http_timeout_ms);
    result["tool_name"] = "runtime_script";
    result["format_version"] = kResultVersion;
    result["session_id"] = requested_session_id;
    result["script_run_id"] = script_run_id;
    result["script_path"] = script_path;
    if (result.has("artifact_dir")) {
        result["runtime_session_artifact_dir"] = result["artifact_dir"];
    }
    if (result.has("status_path")) {
        result["script_status_path"] = result["status_path"];
    }
    godot::Dictionary script_result = result.get("result", godot::Dictionary());
    if (!script_result.is_empty()) {
        if (script_result.has("scene_closed")) {
            result["scene_closed"] = script_result["scene_closed"];
        }
        if (script_result.has("session_active")) {
            result["session_active"] = script_result["session_active"];
        }
        if (script_result.has("captures")) {
            result["captures"] = script_result["captures"];
        }
    }
    godot::String raw_log_path = result.get("raw_log_path", "");
    if (!raw_log_path.is_empty()) {
        result["log_path"] = raw_log_path;
    }
    runtime_script_diagnostics::apply_to_result(diagnostics, result);
    return result;
}

} // namespace fennara
