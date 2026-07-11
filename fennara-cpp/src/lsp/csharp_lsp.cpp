#include "fennara/lsp/csharp_lsp.hpp"

#include "fennara/lsp/csharp_lsp_internal.hpp"
#include "fennara/process_tree.hpp"
#include "fennara/file_utils.hpp"
#include "fennara/logger.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/variant/packed_byte_array.hpp>

#include <atomic>

namespace fennara::csharp_lsp::internal {

constexpr int kPollSleepUsec = 1000;
constexpr int kInitializeTimeoutMs = 15000;
constexpr int kWriteTimeoutMs = 15000;
thread_local const std::atomic_bool *active_request_cancellation = nullptr;

std::atomic_bool &abort_requested() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::atomic_int64_t &request_sequence() {
    static std::atomic_int64_t *value = new std::atomic_int64_t(0);
    return *value;
}

std::atomic_int &server_pid() {
    static std::atomic_int *value = new std::atomic_int(-1);
    return *value;
}

godot::Ref<godot::FileAccess> &server_stderr() {
    static godot::Ref<godot::FileAccess> *value =
        new godot::Ref<godot::FileAccess>();
    return *value;
}

godot::String &stderr_tail() {
    static godot::String *value = new godot::String();
    return *value;
}

godot::String &last_message_summary() {
    static godot::String *value = new godot::String();
    return *value;
}

void drain_server_stderr() {
    godot::Ref<godot::FileAccess> &pipe = server_stderr();
    if (pipe.is_null()) {
        return;
    }
    while (true) {
        godot::PackedByteArray chunk = pipe->get_buffer(4096);
        if (!chunk.is_empty()) {
            stderr_tail() += chunk.get_string_from_utf8();
            if (stderr_tail().length() > 4000) {
                stderr_tail() = stderr_tail().right(4000);
            }
        }
        if (chunk.is_empty() || chunk.size() < 4096) {
            return;
        }
    }
}

int next_request_id() {
    return static_cast<int>(request_sequence().fetch_add(1) + 1);
}

bool is_cancelled(const std::atomic_bool *cancelled) {
    if (abort_requested().load() ||
        (cancelled != nullptr && cancelled->load()) ||
        (active_request_cancellation != nullptr &&
         active_request_cancellation->load())) {
        return true;
    }
    int pid = server_pid().load();
    godot::OS *os = godot::OS::get_singleton();
    return pid > 0 && os != nullptr && !os->is_process_running(pid);
}

godot::Dictionary &open_versions() {
    static godot::Dictionary *versions = new godot::Dictionary();
    return *versions;
}

godot::String file_uri(const godot::String &abs_path) {
    return file_utils::path_to_uri(abs_path);
}

godot::Dictionary empty_file_result() {
    godot::Dictionary file;
    file["diagnostics"] = godot::Array();
    file["total_errors"] = 0;
    file["total_warnings"] = 0;
    file["total_info"] = 0;
    file["total_hints"] = 0;
    file["total_diagnostics"] = 0;
    return file;
}

godot::Dictionary failure(const godot::String &error) {
    godot::Dictionary result;
    result["success"] = false;
    result["error"] = error;
    result["per_file"] = godot::Dictionary();
    return result;
}

godot::Dictionary csharp_config() {
    godot::Dictionary config;
    return config;
}

godot::Variant normalize_rpc_id(const godot::Variant &id) {
    if (id.get_type() == godot::Variant::INT ||
        id.get_type() == godot::Variant::FLOAT) {
        return static_cast<int64_t>(id);
    }
    return id;
}

bool write_message(const godot::Ref<godot::FileAccess> &stdio,
                   const godot::Dictionary &message);

void send_response(const godot::Ref<godot::FileAccess> &stdio,
                   const godot::Variant &id,
                   const godot::Variant &result) {
    godot::Dictionary message;
    message["jsonrpc"] = "2.0";
    godot::Variant normalized_id = normalize_rpc_id(id);
    message["id"] = normalized_id;
    message["result"] = result;
    write_message(stdio, message);
}

bool write_message(const godot::Ref<godot::FileAccess> &stdio,
                   const godot::Dictionary &message) {
    if (stdio.is_null()) {
        return false;
    }

    godot::String body = godot::JSON::stringify(message);
    godot::PackedByteArray body_bytes = body.to_utf8_buffer();
    godot::String header =
        godot::String("Content-Length: {0}\r\n\r\n")
            .format(godot::Array::make(body_bytes.size()));
    godot::PackedByteArray bytes = header.to_ascii_buffer();
    bytes.append_array(body_bytes);
    godot::PackedByteArray one_byte;
    one_byte.resize(1);
    int64_t write_started = godot::Time::get_singleton()->get_ticks_msec();
    for (int i = 0; i < bytes.size(); i++) {
        one_byte.set(0, bytes[i]);
        while (!stdio->store_buffer(one_byte)) {
            drain_server_stderr();
            if (is_cancelled(nullptr) ||
                godot::Time::get_singleton()->get_ticks_msec() - write_started >=
                    kWriteTimeoutMs) {
                return false;
            }
            godot::OS::get_singleton()->delay_usec(kPollSleepUsec);
        }
    }
    stdio->flush();
    return true;
}

void send_notification(const godot::Ref<godot::FileAccess> &stdio,
                       const godot::String &method,
                       const godot::Dictionary &params) {
    godot::Dictionary message;
    message["jsonrpc"] = "2.0";
    message["method"] = method;
    message["params"] = params;
    write_message(stdio, message);
}

void handle_server_request(const godot::Ref<godot::FileAccess> &stdio,
                           const godot::Dictionary &message) {
    if (!message.has("id") || !message.has("method")) {
        return;
    }

    godot::String method = message.get("method", "");
    if (method == "client/registerCapability" ||
        method == "client/unregisterCapability") {
        send_response(stdio, message["id"], godot::Dictionary());
        return;
    }

    if (method == "workspace/configuration") {
        godot::Array configs;
        godot::Dictionary params = message.get("params", godot::Dictionary());
        godot::Array items = params.get("items", godot::Array());
        godot::Dictionary config = csharp_config();
        for (int i = 0; i < items.size(); i++) {
            configs.append(config);
        }
        send_response(stdio, message["id"], configs);
        return;
    }

    if (method == "workspace/diagnostic/refresh" ||
        method == "window/workDoneProgress/create") {
        send_response(stdio, message["id"], godot::Dictionary());
        return;
    }

    if (method == "workspace/workspaceFolders" ||
        method == "window/showMessageRequest") {
        send_response(stdio, message["id"], godot::Variant());
        return;
    }

    if (method == "workspace/applyEdit") {
        godot::Dictionary result;
        result["applied"] = false;
        send_response(stdio, message["id"], result);
    }
}

bool read_byte(const godot::Ref<godot::FileAccess> &stdio, char &out) {
    if (stdio.is_null()) {
        return false;
    }
    godot::PackedByteArray bytes = stdio->get_buffer(1);
    if (bytes.size() != 1) {
        return false;
    }
    out = static_cast<char>(bytes[0]);
    return true;
}

godot::Dictionary read_one_message(const godot::Ref<godot::FileAccess> &stdio,
                                   int timeout_ms,
                                   const std::atomic_bool *cancelled = nullptr) {
    const int64_t start = godot::Time::get_singleton()->get_ticks_msec();
    godot::String header;

    while (godot::Time::get_singleton()->get_ticks_msec() - start < timeout_ms) {
        drain_server_stderr();
        if (is_cancelled(cancelled)) {
            return godot::Dictionary();
        }
        char ch = 0;
        if (read_byte(stdio, ch)) {
            header += godot::String::chr(ch);
            if (header.ends_with("\r\n\r\n")) {
                break;
            }
        } else {
            godot::OS::get_singleton()->delay_usec(kPollSleepUsec);
        }
    }

    if (!header.ends_with("\r\n\r\n")) {
        return godot::Dictionary();
    }

    int content_length = -1;
    godot::PackedStringArray lines = header.split("\r\n");
    for (int i = 0; i < lines.size(); i++) {
        if (lines[i].to_lower().begins_with("content-length:")) {
            godot::PackedStringArray parts = lines[i].split(":");
            if (parts.size() >= 2) {
                content_length = parts[1].strip_edges().to_int();
            }
            break;
        }
    }
    if (content_length <= 0) {
        return godot::Dictionary();
    }

    godot::PackedByteArray body;
    while (body.size() < content_length &&
           godot::Time::get_singleton()->get_ticks_msec() - start < timeout_ms) {
        drain_server_stderr();
        if (is_cancelled(cancelled)) {
            return godot::Dictionary();
        }
        int remaining = content_length - body.size();
        godot::PackedByteArray chunk = stdio->get_buffer(remaining);
        if (!chunk.is_empty()) {
            body.append_array(chunk);
        } else {
            godot::OS::get_singleton()->delay_usec(kPollSleepUsec);
        }
    }

    if (body.size() < content_length) {
        return godot::Dictionary();
    }

    godot::Variant parsed = godot::JSON::parse_string(body.get_string_from_utf8());
    return parsed.get_type() == godot::Variant::DICTIONARY
               ? godot::Dictionary(parsed)
               : godot::Dictionary();
}

godot::Dictionary wait_for_response(const godot::Ref<godot::FileAccess> &stdio,
                                    int id,
                                    int timeout_ms,
                                    const std::atomic_bool *cancelled = nullptr) {
    const int64_t start = godot::Time::get_singleton()->get_ticks_msec();
    while (godot::Time::get_singleton()->get_ticks_msec() - start < timeout_ms) {
        if (is_cancelled(cancelled)) {
            return godot::Dictionary();
        }
        int remaining =
            timeout_ms -
            static_cast<int>(godot::Time::get_singleton()->get_ticks_msec() - start);
        if (remaining <= 0) {
            break;
        }
        godot::Dictionary message = read_one_message(stdio, remaining, cancelled);
        if (!message.is_empty()) {
            last_message_summary() =
                "method=" + godot::String(message.get("method", "")) +
                " id=" + godot::String(message.get("id", godot::Variant())) +
                " has_result=" + (message.has("result") ? "true" : "false") +
                " has_error=" + (message.has("error") ? "true" : "false");
        }
        handle_server_request(stdio, message);
        if (message.has("id") &&
            (message.has("result") || message.has("error")) &&
            static_cast<int>(message["id"]) == id) {
            return message;
        }
    }
    FLOG_ERR(godot::String("C# LSP response timed out id=") +
             godot::String::num_int64(id) + " timeout_ms=" +
             godot::String::num_int64(timeout_ms) + " " + timeout_context());
    return godot::Dictionary();
}

bool pump_until_workspace_ready(const godot::Ref<godot::FileAccess> &stdio,
                                int timeout_ms,
                                const std::atomic_bool *cancelled) {
    const int64_t start = godot::Time::get_singleton()->get_ticks_msec();
    bool saw_load_start = false;
    while (godot::Time::get_singleton()->get_ticks_msec() - start < timeout_ms) {
        if (is_cancelled(cancelled)) {
            return false;
        }
        godot::Dictionary message = read_one_message(stdio, 1000, cancelled);
        if (message.is_empty()) {
            continue;
        }

        godot::String method = message.get("method", "");
        if (method == "window/logMessage") {
            godot::Dictionary params = message.get("params", godot::Dictionary());
            godot::String text = params.get("message", "");
            if (text.find("loading project") >= 0) {
                saw_load_start = true;
            }
            if (text.find("project file(s) loaded") >= 0) {
                return true;
            }
        } else if (method == "$/progress") {
            godot::Dictionary params = message.get("params", godot::Dictionary());
            godot::Dictionary value = params.get("value", godot::Dictionary());
            godot::String kind = value.get("kind", "");
            if (kind == "end") {
                return true;
            }
            if (kind == "begin" || kind == "report") {
                saw_load_start = true;
            }
        }

        handle_server_request(stdio, message);
    }
    FLOG_ERR(godot::String("C# LSP workspace-ready wait ended by timeout") +
              (saw_load_start ? " after load start" : " before load start"));
    return false;
}

godot::Dictionary initialize(const godot::Ref<godot::FileAccess> &stdio,
                             const godot::String &client_name,
                             const godot::String &root_uri,
                             const godot::String &workspace_name,
                             const std::atomic_bool *cancelled) {
    godot::Dictionary client_info;
    client_info["name"] = client_name;
    client_info["version"] = "1.0";

    godot::Dictionary workspace_folder;
    workspace_folder["uri"] = root_uri;
    workspace_folder["name"] = workspace_name;
    godot::Array workspace_folders;
    workspace_folders.append(workspace_folder);

    godot::Dictionary text_document;
    text_document["documentSymbol"] = godot::Dictionary();
    godot::Dictionary diagnostic_capability;
    diagnostic_capability["dynamicRegistration"] = false;
    text_document["diagnostic"] = diagnostic_capability;
    text_document["publishDiagnostics"] = godot::Dictionary();

    godot::Dictionary workspace_diagnostics;
    workspace_diagnostics["refreshSupport"] = true;
    godot::Dictionary workspace;
    workspace["configuration"] = true;
    workspace["diagnostics"] = workspace_diagnostics;

    godot::Dictionary capabilities;
    capabilities["textDocument"] = text_document;
    capabilities["workspace"] = workspace;

    godot::Dictionary params;
    params["processId"] = godot::OS::get_singleton()->get_process_id();
    params["clientInfo"] = client_info;
    params["rootUri"] = root_uri;
    params["workspaceFolders"] = workspace_folders;
    params["capabilities"] = capabilities;

    godot::Dictionary message;
    message["jsonrpc"] = "2.0";
    int request_id = next_request_id();
    message["id"] = request_id;
    message["method"] = "initialize";
    message["params"] = params;
    write_message(stdio, message);

    godot::Dictionary response =
        wait_for_response(stdio, request_id, kInitializeTimeoutMs, cancelled);
    if (response.has("result")) {
        send_notification(stdio, "initialized", godot::Dictionary());
    }
    return response;
}

godot::String severity_name(int severity) {
    switch (severity) {
        case 1:
            return "error";
        case 2:
            return "warning";
        case 3:
            return "info";
        case 4:
            return "hint";
        default:
            return "info";
    }
}

void add_diagnostic(godot::Dictionary &file_result,
                    const godot::Dictionary &raw) {
    godot::Dictionary range = raw.get("range", godot::Dictionary());
    godot::Dictionary start = range.get("start", godot::Dictionary());
    int severity = static_cast<int>(raw.get("severity", 3));
    godot::String severity_text = severity_name(severity);

    godot::Dictionary diagnostic;
    diagnostic["line"] = static_cast<int>(start.get("line", 0)) + 1;
    diagnostic["column"] = static_cast<int>(start.get("character", 0)) + 1;
    diagnostic["severity"] = severity_text;
    diagnostic["message"] = raw.get("message", "");
    if (raw.has("code")) {
        diagnostic["code"] = godot::String(raw["code"]);
    }
    if (raw.has("source")) {
        diagnostic["source"] = raw["source"];
    }

    godot::Array diagnostics = file_result.get("diagnostics", godot::Array());
    diagnostics.append(diagnostic);
    file_result["diagnostics"] = diagnostics;

    godot::String counter =
        severity_text == "error" ? "total_errors" :
        severity_text == "warning" ? "total_warnings" :
        severity_text == "hint" ? "total_hints" : "total_info";
    file_result[counter] = static_cast<int>(file_result.get(counter, 0)) + 1;
    file_result["total_diagnostics"] =
        static_cast<int>(file_result.get("total_diagnostics", 0)) + 1;
}

void notify_file_changed(const godot::Ref<godot::FileAccess> &stdio,
                         const godot::String &abs_path) {
    godot::Dictionary file_event;
    file_event["uri"] = file_uri(abs_path);
    file_event["type"] = 2;

    godot::Array changes;
    changes.append(file_event);

    godot::Dictionary params;
    params["changes"] = changes;
    send_notification(stdio, "workspace/didChangeWatchedFiles", params);
}

void open_document(const godot::Ref<godot::FileAccess> &stdio,
                   const godot::String &abs_path) {
    godot::String uri = file_uri(abs_path);
    godot::Dictionary &versions = open_versions();
    int version = static_cast<int>(versions.get(uri, 0)) + 1;
    godot::Dictionary text_document;
    text_document["version"] = 1;
    text_document["uri"] = uri;

    if (versions.has(uri)) {
        text_document["version"] = version;
        godot::Dictionary change;
        change["text"] = file_utils::read_file_content(abs_path);
        godot::Array changes;
        changes.append(change);

        godot::Dictionary params;
        params["textDocument"] = text_document;
        params["contentChanges"] = changes;
        send_notification(stdio, "textDocument/didChange", params);
        versions[uri] = version;
        return;
    }

    text_document["languageId"] = "csharp";
    text_document["text"] = file_utils::read_file_content(abs_path);

    godot::Dictionary params;
    params["workDoneToken"] = godot::Variant();
    params["partialResultToken"] = godot::Variant();
    params["textDocument"] = text_document;
    params["identifier"] = godot::Variant();
    params["previousResultId"] = godot::Variant();
    send_notification(stdio, "textDocument/didOpen", params);
    versions[uri] = 1;
}

void close_document(const godot::Ref<godot::FileAccess> &stdio,
                    const godot::String &abs_path) {
    godot::String uri = file_uri(abs_path);
    if (!open_versions().has(uri)) {
        return;
    }
    godot::Dictionary text_document;
    text_document["uri"] = uri;
    godot::Dictionary params;
    params["textDocument"] = text_document;
    send_notification(stdio, "textDocument/didClose", params);
    open_versions().erase(uri);
}

godot::Dictionary request_document_diagnostics(
    const godot::Ref<godot::FileAccess> &stdio,
    const godot::String &abs_path,
    const std::atomic_bool *cancelled) {
    int request_id = next_request_id();
    godot::Dictionary text_document;
    text_document["uri"] = file_uri(abs_path);

    godot::Dictionary params;
    params["workDoneToken"] = godot::Variant();
    params["partialResultToken"] = godot::Variant();
    params["textDocument"] = text_document;
    params["identifier"] = godot::Variant();
    params["previousResultId"] = godot::Variant();

    godot::Dictionary message;
    message["jsonrpc"] = "2.0";
    message["id"] = request_id;
    message["method"] = "textDocument/diagnostic";
    message["params"] = params;
    if (!write_message(stdio, message)) {
        godot::Dictionary response;
        response["error"] = "Failed to write textDocument/diagnostic request to csharp-ls.";
        response["transport_error"] = true;
        return response;
    }
    return wait_for_response(stdio, request_id, kDiagnosticsTimeoutMs, cancelled);
}

godot::Dictionary request_document_symbols(
    const godot::Ref<godot::FileAccess> &stdio,
    const godot::String &abs_path,
    const std::atomic_bool *cancelled) {
    int request_id = next_request_id();
    godot::Dictionary text_document;
    text_document["uri"] = file_uri(abs_path);

    godot::Dictionary params;
    params["textDocument"] = text_document;

    godot::Dictionary message;
    message["jsonrpc"] = "2.0";
    message["id"] = request_id;
    message["method"] = "textDocument/documentSymbol";
    message["params"] = params;
    if (!write_message(stdio, message)) {
        godot::Dictionary response;
        response["error"] = "Failed to write textDocument/documentSymbol request to csharp-ls.";
        response["transport_error"] = true;
        return response;
    }
    return wait_for_response(stdio, request_id, kSymbolsTimeoutMs, cancelled);
}

godot::Dictionary file_result_from_document_diagnostics(
    const godot::Dictionary &response) {
    godot::Dictionary file_result = empty_file_result();
    godot::Dictionary result = response.get("result", godot::Dictionary());
    godot::Array items = result.get("items", godot::Array());
    for (int i = 0; i < items.size(); i++) {
        if (items[i].get_type() == godot::Variant::DICTIONARY) {
            add_diagnostic(file_result, items[i]);
        }
    }
    return file_result;
}

void shutdown(const godot::Ref<godot::FileAccess> &stdio, int pid) {
    godot::Dictionary message;
    message["jsonrpc"] = "2.0";
    int request_id = next_request_id();
    message["id"] = request_id;
    message["method"] = "shutdown";
    write_message(stdio, message);
    wait_for_response(stdio, request_id, kShutdownTimeoutMs);
    send_notification(stdio, "exit", godot::Dictionary());

    godot::OS *os = godot::OS::get_singleton();
    if (pid > 0 && os->is_process_running(pid)) {
        process_tree::terminate_and_wait(pid);
    }
}

void clear_open_documents() {
    open_versions().clear();
}

void request_abort() {
    abort_requested().store(true);
}

void clear_abort() {
    abort_requested().store(false);
}

void set_server_pid(int pid) {
    server_pid().store(pid);
}

void set_stderr_pipe(const godot::Ref<godot::FileAccess> &stderr_pipe) {
    server_stderr() = stderr_pipe;
    stderr_tail() = "";
    last_message_summary() = "";
    drain_server_stderr();
}

void set_request_cancellation(const std::atomic_bool *cancelled) {
    active_request_cancellation = cancelled;
}

godot::String timeout_context() {
    godot::String stderr_text = stderr_tail().strip_edges().replace("\r", " ").replace("\n", " | ");
    return "last_message={" + last_message_summary() + "} stderr_tail={" +
           stderr_text + "}";
}

} // namespace fennara::csharp_lsp::internal
