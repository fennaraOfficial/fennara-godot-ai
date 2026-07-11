#include "fennara/lsp/csharp_lsp.hpp"

#include "fennara/lsp/csharp_lsp_internal.hpp"
#include "fennara/lsp/csharp_build.hpp"
#include "fennara/lsp/csharp_project_graph.hpp"
#include "fennara/lsp/csharp_support.hpp"
#include "fennara/file_utils.hpp"
#include "fennara/logger.hpp"
#include "fennara/process_tree.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

#include <atomic>
#include <chrono>
#include <condition_variable>
#include <mutex>
#include <thread>

namespace fennara::csharp_lsp {
namespace {

struct SessionState {
    godot::Ref<godot::FileAccess> stdio;
    int pid = -1;
    godot::String lsp_path;
    godot::String project_path;
    godot::String root_uri;
    int64_t graph_generation = 0;
};

SessionState &session_state() {
    static SessionState *state = new SessionState();
    return *state;
}

std::mutex &session_mutex() {
    static std::mutex *mutex = new std::mutex();
    return *mutex;
}

std::atomic_bool &warmup_in_progress() {
    static std::atomic_bool *warming = new std::atomic_bool(false);
    return *warming;
}

std::atomic_bool &preparation_reserved() {
    static std::atomic_bool *reserved = new std::atomic_bool(false);
    return *reserved;
}

std::mutex &preparation_wait_mutex() {
    static std::mutex *mutex = new std::mutex();
    return *mutex;
}

std::condition_variable &preparation_wait_condition() {
    static std::condition_variable *condition = new std::condition_variable();
    return *condition;
}

std::thread &warmup_thread() {
    static std::thread *thread = new std::thread();
    return *thread;
}

std::atomic_bool &shutting_down() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::atomic_int &diagnostics_priority_count() {
    static std::atomic_int *count = new std::atomic_int(0);
    return *count;
}

struct DiagnosticsPriorityScope {
    DiagnosticsPriorityScope() {
        diagnostics_priority_count().fetch_add(1);
    }

    ~DiagnosticsPriorityScope() {
        diagnostics_priority_count().fetch_sub(1);
    }
};

struct RequestCancellationScope {
    explicit RequestCancellationScope(const std::atomic_bool *cancelled) {
        internal::set_request_cancellation(cancelled);
    }

    ~RequestCancellationScope() {
        internal::set_request_cancellation(nullptr);
    }
};

void wait_for_diagnostics_priority_to_clear(const godot::String &reason) {
    bool logged = false;
    while (diagnostics_priority_count().load() > 0) {
        if (!logged) {
            Logger::log_activity(reason);
            logged = true;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(50));
    }
}

void clear_session() {
    SessionState &state = session_state();
    state.stdio.unref();
    state.pid = -1;
    state.lsp_path = "";
    state.project_path = "";
    state.root_uri = "";
    state.graph_generation = 0;
    internal::set_server_pid(-1);
    internal::set_stderr_pipe(godot::Ref<godot::FileAccess>());
    internal::clear_open_documents();
}

void terminate_session_locked() {
    SessionState &state = session_state();
    godot::OS *os = godot::OS::get_singleton();
    if (state.pid > 0 && os != nullptr && os->is_process_running(state.pid)) {
        process_tree::terminate_and_wait(state.pid);
    }
    clear_session();
}

bool session_alive() {
    SessionState &state = session_state();
    return state.stdio.is_valid() && state.pid > 0 &&
           godot::OS::get_singleton()->is_process_running(state.pid);
}

void shutdown_warm_server_locked() {
    SessionState &state = session_state();
    if (session_alive()) {
        internal::shutdown(state.stdio, state.pid);
    }
    clear_session();
}

void flatten_symbols(godot::Array symbols, godot::Array &out) {
    for (int i = 0; i < symbols.size(); i++) {
        if (symbols[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary symbol = symbols[i];
        out.append(symbol);
        godot::Variant children_var = symbol.get("children", godot::Array());
        if (children_var.get_type() == godot::Variant::ARRAY) {
            flatten_symbols(children_var, out);
        }
    }
}

godot::Dictionary failed_file_result(const godot::String &error) {
    godot::Dictionary file = internal::empty_file_result();
    file["status"] = "failed";
    file["error"] = error;
    return file;
}

godot::String matching_solution(const godot::Dictionary &status,
                                const godot::Dictionary &selected) {
    if (godot::String(selected.get("type", "")) == "solution") {
        return selected.get("absolute_path", "");
    }
    godot::String project_name =
        godot::String(selected.get("absolute_path", "")).get_file().get_basename();
    godot::Array candidates = status.get("projects", godot::Array());
    godot::String match;
    for (int i = 0; i < candidates.size(); i++) {
        godot::Dictionary candidate = candidates[i];
        if (godot::String(candidate.get("type", "")) != "solution") {
            continue;
        }
        godot::String path = candidate.get("absolute_path", "");
        if (path.get_file().get_basename().nocasecmp_to(project_name) != 0) {
            continue;
        }
        if (!match.is_empty()) {
            return "";
        }
        match = path;
    }
    return match;
}

godot::String ensure_generated_solution(const godot::String &project_path) {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings == nullptr) {
        return "";
    }
    godot::String dir = settings->globalize_path("res://.godot/fennara/csharp_lsp");
    godot::DirAccess::make_dir_recursive_absolute(dir);
    godot::String solution_path = dir.path_join("fennara-csharp-lsp.sln");

    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(solution_path, godot::FileAccess::WRITE);
    if (file.is_null()) {
        return "";
    }
    godot::String project_name = project_path.get_file().get_basename();
    godot::OS *os = godot::OS::get_singleton();
    godot::String solution_project_path =
        os != nullptr && os->get_name() == "Windows"
            ? project_path.replace("/", "\\")
            : project_path.replace("\\", "/");
    const godot::String project_guid = "{47D63888-1487-4A3D-A93C-0A520E41A9A1}";
    godot::String content =
        "Microsoft Visual Studio Solution File, Format Version 12.00\r\n"
        "# Visual Studio Version 17\r\n"
        "VisualStudioVersion = 17.0.31903.59\r\n"
        "MinimumVisualStudioVersion = 10.0.40219.1\r\n"
        "Project(\"{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}\") = \"" +
        project_name + "\", \"" + solution_project_path + "\", \"" + project_guid +
        "\"\r\nEndProject\r\nGlobal\r\n"
        "\tGlobalSection(SolutionConfigurationPlatforms) = preSolution\r\n"
        "\t\tDebug|Any CPU = Debug|Any CPU\r\n"
        "\tEndGlobalSection\r\n"
        "\tGlobalSection(ProjectConfigurationPlatforms) = postSolution\r\n"
        "\t\t" + project_guid + ".Debug|Any CPU.ActiveCfg = Debug|Any CPU\r\n"
        "\t\t" + project_guid + ".Debug|Any CPU.Build.0 = Debug|Any CPU\r\n"
        "\tEndGlobalSection\r\nEndGlobal\r\n";
    file->store_string(content);
    return solution_path;
}

godot::Dictionary start_session_for_paths(const godot::String &lsp_path,
                                          const godot::String &project_path,
                                          const godot::String &root_uri,
                                          const godot::String &client_name,
                                          const std::atomic_bool *cancelled = nullptr,
                                          int64_t requested_graph_generation = 0) {
    if (lsp_path.is_empty() || project_path.is_empty() || root_uri.is_empty()) {
        return internal::failure("C# LSP warmup skipped: missing project path.");
    }

    SessionState &state = session_state();
    if (session_alive() && state.lsp_path == lsp_path &&
        state.project_path == project_path && state.root_uri == root_uri &&
        (requested_graph_generation == 0 || state.graph_generation == 0 ||
         state.graph_generation == requested_graph_generation)) {
        if (state.graph_generation == 0 && requested_graph_generation > 0) {
            state.graph_generation = requested_graph_generation;
        }
        godot::Dictionary result;
        result["success"] = true;
        result["reused"] = true;
        result["project_path"] = state.project_path;
        result["lsp_path"] = state.lsp_path;
        return result;
    }

    if (session_alive()) {
        shutdown_warm_server_locked();
    }
    clear_session();

    if (shutting_down().load()) {
        return internal::failure("C# LSP session is shutting down.");
    }
    internal::clear_abort();
    godot::PackedStringArray args;
    args.append("--solution");
    args.append(project_path);
    godot::Dictionary process =
        godot::OS::get_singleton()->execute_with_pipe(lsp_path, args, false);
    if (!process.has("stdio")) {
        return internal::failure("Failed to start csharp-ls.");
    }

    godot::Ref<godot::FileAccess> stdio = process["stdio"];
    godot::Ref<godot::FileAccess> stderr_pipe = process.get(
        "stderr", godot::Ref<godot::FileAccess>());
    int pid = static_cast<int>(process.get("pid", -1));
    internal::set_server_pid(pid);
    internal::set_stderr_pipe(stderr_pipe);
    if (stdio.is_null()) {
        if (pid > 0) {
            process_tree::terminate_and_wait(pid);
        }
        internal::set_server_pid(-1);
        return internal::failure("Failed to open csharp-ls stdio pipe.");
    }

    godot::Dictionary init =
        internal::initialize(stdio, client_name, root_uri, project_path.get_file(), cancelled);
    if (!init.has("result")) {
        internal::shutdown(stdio, pid);
        internal::set_server_pid(-1);
        return internal::failure("csharp-ls initialization timed out or failed.");
    }
    if (!internal::pump_until_workspace_ready(
            stdio, internal::kWorkspaceReadyTimeoutMs, cancelled)) {
        process_tree::terminate_and_wait(pid);
        internal::set_server_pid(-1);
        return internal::failure("csharp-ls project loading timed out, failed, or was cancelled.");
    }

    state.stdio = stdio;
    state.pid = pid;
    state.lsp_path = lsp_path;
    state.project_path = project_path;
    state.root_uri = root_uri;
    state.graph_generation = requested_graph_generation;

    godot::Dictionary result;
    result["success"] = true;
    result["reused"] = false;
    result["project_path"] = state.project_path;
    result["lsp_path"] = state.lsp_path;
    return result;
}

godot::Dictionary start_session_from_project_scan(
    const godot::String &client_name,
    const std::atomic_bool *cancelled = nullptr,
    int64_t requested_graph_generation = 0) {
    godot::Dictionary csharp_status = csharp_support::inspect_project();
    if (godot::String(csharp_status.get("state", "")) != "ready") {
        return internal::failure(
            csharp_support::diagnostics_unavailable_message(csharp_status));
    }

    godot::String lsp_path = csharp_status.get("lsp_path", "");
    godot::Dictionary selected_project =
        csharp_status.get("selected_project", godot::Dictionary());
    godot::String project_path =
        matching_solution(csharp_status, selected_project);
    if (project_path.is_empty()) {
        project_path = ensure_generated_solution(
            selected_project.get("absolute_path", ""));
        if (project_path.is_empty()) {
            return internal::failure(
                "Could not create a managed solution for csharp-ls.");
        }
    }
    godot::String project_root =
        godot::ProjectSettings::get_singleton()->globalize_path("res://");
    godot::String root_uri = internal::file_uri(project_root);
    return start_session_for_paths(
        lsp_path, project_path, root_uri, client_name, cancelled,
        requested_graph_generation);
}

void launch_preparation_async(const godot::String &client_name,
                              bool include_build) {
    if (shutting_down().load()) {
        return;
    }
    bool was_reserved = preparation_reserved().exchange(false);
    if (!was_reserved) {
        bool expected = false;
        if (!warmup_in_progress().compare_exchange_strong(expected, true)) {
            return;
        }
    }
    if (warmup_thread().joinable()) {
        warmup_thread().join();
    }
    warmup_thread() = std::thread([client_name, include_build]() {
        Logger::log_activity(include_build
            ? godot::String("C# background preparation started")
            : godot::String("C# LSP recovery warmup started"));

        godot::Dictionary support = csharp_support::inspect_project();
        godot::Dictionary selected =
            support.get("selected_project", godot::Dictionary());
        godot::String selected_type = selected.get("type", "");
        bool build_ready =
            !godot::String(selected.get("absolute_path", "")).is_empty() &&
            (selected_type == "project" || selected_type == "solution");
        bool lsp_ready = godot::String(support.get("state", "")) == "ready";

        if (include_build && build_ready) {
            godot::Dictionary build =
                csharp_build::run_background_diagnostics(&shutting_down());
            double duration = build.get("duration_seconds", 0.0);
            if ((bool)build.get("success", false) &&
                (bool)build.get("build_succeeded", false)) {
                Logger::log_activity(
                    "C# background isolated build ready in " +
                    godot::String::num(duration, 3) + "s");
            } else if ((bool)build.get("cancelled", false)) {
                Logger::log_activity(
                    "C# background isolated build cancelled after " +
                    godot::String::num(duration, 3) + "s");
            } else {
                Logger::log_activity(
                    "C# background isolated build completed with diagnostics in " +
                    godot::String::num(duration, 3) + "s");
            }
        } else if (include_build) {
            Logger::log_activity(
                "C# background isolated build skipped: no unambiguous C# project");
        }

        if (lsp_ready && !shutting_down().load()) {
            std::lock_guard<std::mutex> lock(session_mutex());
            godot::Dictionary result =
                start_session_from_project_scan(client_name, &shutting_down());
            if ((bool)result.get("success", false)) {
                Logger::log_activity(
                    godot::String("C# LSP background warmup ready: ") +
                    godot::String(result.get("project_path", "")));
            } else {
                Logger::log_activity(
                    godot::String("C# LSP background warmup skipped: ") +
                    godot::String(result.get("error", "")));
            }
        } else if (!shutting_down().load()) {
            Logger::log_activity(
                godot::String("C# LSP background warmup skipped: ") +
                godot::String(support.get("message", "")));
        }

        warmup_in_progress().store(false);
        preparation_wait_condition().notify_all();
        Logger::log_activity(include_build
            ? godot::String("C# background preparation complete")
            : godot::String("C# LSP recovery warmup complete"));
    });
}

} // namespace

void begin_session_lifecycle() {
    shutting_down().store(false);
    internal::clear_abort();
}

void reserve_background_preparation() {
    if (shutting_down().load()) {
        return;
    }
    preparation_reserved().store(true);
    warmup_in_progress().store(true);
}

void cancel_reserved_background_preparation() {
    if (!preparation_reserved().exchange(false)) {
        return;
    }
    warmup_in_progress().store(false);
    preparation_wait_condition().notify_all();
}

godot::Dictionary warmup(const godot::String &client_name) {
    std::lock_guard<std::mutex> lock(session_mutex());
    godot::Dictionary result = start_session_from_project_scan(client_name);
    if ((bool)result.get("success", false)) {
        FLOG_SYS(godot::String("C# LSP warmup ready: ") +
                 godot::String(result.get("project_path", "")));
    } else {
        FLOG_SYS(godot::String("C# LSP warmup skipped: ") +
                 godot::String(result.get("error", "")));
    }
    return result;
}

void warmup_async(const godot::String &lsp_path,
                  const godot::String &project_path,
                  const godot::String &project_root,
                  const godot::String &client_name) {
    (void)lsp_path;
    (void)project_path;
    (void)project_root;
    launch_preparation_async(client_name, true);
}

bool wait_for_background_preparation(
    const godot::String &activity,
    const std::atomic_bool *cancelled) {
    if (!warmup_in_progress().load()) {
        return true;
    }
    Logger::log_activity(
        activity + godot::String(" waiting for C# background preparation"));
    std::unique_lock<std::mutex> lock(preparation_wait_mutex());
    while (warmup_in_progress().load()) {
        if ((cancelled != nullptr && cancelled->load()) ||
            shutting_down().load()) {
            return false;
        }
        preparation_wait_condition().wait_for(
            lock, std::chrono::milliseconds(50));
    }
    Logger::log_activity(
        activity + godot::String(" continuing after C# background preparation"));
    return true;
}

bool background_preparation_in_progress() {
    return warmup_in_progress().load();
}

godot::Dictionary diagnose_files(const godot::Array &files,
                                 const godot::String &client_name,
                                 const std::atomic_bool *cancelled) {
    DiagnosticsPriorityScope priority_scope;
    if (!wait_for_background_preparation("C# LSP diagnostics", cancelled)) {
        return internal::failure(
            "C# LSP diagnostics cancelled while waiting for background preparation.");
    }

    godot::Dictionary graph =
        csharp_project_graph::evaluate_selected(cancelled);
    if (!(bool)graph.get("ok", false)) {
        return internal::failure(graph.get("error", "Failed to evaluate C# project graph."));
    }

    godot::Dictionary per_file;
    godot::Array failed_files;
    godot::Array eligible_files;
    bool refreshed_after_miss = false;
    for (int i = 0; i < files.size(); i++) {
        godot::String abs_path = files[i];
        godot::Array owners =
            csharp_project_graph::owners_for_file(graph, abs_path);
        if (owners.is_empty() && !refreshed_after_miss) {
            csharp_project_graph::invalidate();
            graph = csharp_project_graph::evaluate_selected(cancelled);
            if (!(bool)graph.get("ok", false)) {
                return internal::failure(
                    graph.get("error", "Failed to refresh C# project graph."));
            }
            refreshed_after_miss = true;
            owners = csharp_project_graph::owners_for_file(graph, abs_path);
        }
        if (owners.is_empty()) {
            godot::String error =
                "File is not compiled by the selected C# project graph.";
            per_file[abs_path] = failed_file_result(error);
            godot::Dictionary failed;
            failed["path"] = file_utils::uri_to_res_path(internal::file_uri(abs_path));
            failed["error"] = error;
            failed_files.append(failed);
            continue;
        }
        eligible_files.append(abs_path);
    }

    if (eligible_files.is_empty()) {
        godot::Dictionary result;
        result["success"] = true;
        result["per_file"] = per_file;
        result["failed_files"] = failed_files;
        return result;
    }

    std::unique_lock<std::mutex> lock(session_mutex());
    RequestCancellationScope cancellation_scope(cancelled);

    godot::Dictionary session =
        start_session_from_project_scan(
            client_name, cancelled, graph.get("generation", 0));
    if (!(bool)session.get("success", false)) {
        return session;
    }
    bool reused = session.get("reused", false);
    FLOG_TOOL(godot::String("C# LSP diagnostics checking files=") +
              godot::String::num_int64(files.size()) +
              " reused=" + (reused ? "true" : "false"));

    SessionState &state = session_state();
    bool restart_unhealthy_session = false;
    for (int i = 0; i < eligible_files.size(); i++) {
        if ((cancelled != nullptr && cancelled->load()) || shutting_down().load()) {
            terminate_session_locked();
            break;
        }
        godot::String abs_path = eligible_files[i];
        per_file[abs_path] = internal::empty_file_result();

        // Keep all C# diagnostics on the one managed csharp-ls connection.
        // Opening and requesting one document at a time avoids a burst of
        // overlapping diagnostic requests while preserving the warm LSP state.
        internal::open_document(state.stdio, abs_path);
        godot::Dictionary response =
            internal::request_document_diagnostics(state.stdio, abs_path, cancelled);
        internal::close_document(state.stdio, abs_path);
        if (i + 1 < eligible_files.size()) {
            godot::OS::get_singleton()->delay_usec(25000);
        }
        if (!response.has("result")) {
            godot::String res_path =
                file_utils::uri_to_res_path(internal::file_uri(abs_path));
            godot::String error =
                "csharp-ls did not return textDocument/diagnostic for " +
                res_path;
            if (response.has("error")) {
                error = "csharp-ls returned a diagnostic request error for " +
                        res_path + ": " +
                        godot::JSON::stringify(response["error"]);
                FLOG_ERR(error);
                per_file[abs_path] = failed_file_result(error);
                godot::Dictionary failed;
                failed["path"] = res_path;
                failed["error"] = error;
                failed_files.append(failed);
                if ((bool)response.get("transport_error", false)) {
                    terminate_session_locked();
                    restart_unhealthy_session = true;
                    for (int remaining = i + 1;
                         remaining < eligible_files.size(); remaining++) {
                        godot::String skipped_path = eligible_files[remaining];
                        godot::String skipped_error =
                            "Skipped because the csharp-ls session became unhealthy.";
                        per_file[skipped_path] = failed_file_result(skipped_error);
                        godot::Dictionary skipped;
                        skipped["path"] = file_utils::uri_to_res_path(
                            internal::file_uri(skipped_path));
                        skipped["error"] = skipped_error;
                        failed_files.append(skipped);
                    }
                    break;
                }
                continue;
            }
            FLOG_ERR(error);
            per_file[abs_path] = failed_file_result(error);

            godot::Dictionary failed;
            failed["path"] = res_path;
            failed["error"] = error;
            failed_files.append(failed);
            terminate_session_locked();
            restart_unhealthy_session = true;
            for (int remaining = i + 1; remaining < eligible_files.size(); remaining++) {
                godot::String skipped_path = eligible_files[remaining];
                godot::String skipped_error =
                    "Skipped because the csharp-ls session became unhealthy.";
                per_file[skipped_path] = failed_file_result(skipped_error);
                godot::Dictionary skipped;
                skipped["path"] = file_utils::uri_to_res_path(
                    internal::file_uri(skipped_path));
                skipped["error"] = skipped_error;
                failed_files.append(skipped);
            }
            break;
        }
        per_file[abs_path] = internal::file_result_from_document_diagnostics(response);
    }

    godot::Dictionary result;
    result["success"] = true;
    result["per_file"] = per_file;
    result["failed_files"] = failed_files;
    result["project_path"] = session.get("project_path", "");
    result["lsp_path"] = session.get("lsp_path", "");
    result["reused_lsp"] = session.get("reused", false);
    lock.unlock();
    if (restart_unhealthy_session && !shutting_down().load()) {
        launch_preparation_async("fennara-csharp-recovery", false);
    }
    return result;
}

godot::Dictionary document_symbols(const godot::Array &files,
                                   const godot::String &client_name) {
    if (!wait_for_background_preparation("C# LSP indexing")) {
        return internal::failure(
            "C# LSP indexing cancelled while waiting for background preparation.");
    }

    godot::Dictionary per_file;
    godot::Array failed_files;
    godot::String project_path;
    godot::String lsp_path;
    bool first_file = true;
    bool reused_lsp = false;
    bool restart_unhealthy_session = false;

    for (int i = 0; i < files.size(); i++) {
        godot::String abs_path = files[i];
        wait_for_diagnostics_priority_to_clear(
            "C# LSP indexing yielded to diagnostics");

        godot::Dictionary response;
        {
            std::lock_guard<std::mutex> lock(session_mutex());
            godot::Dictionary session =
                start_session_from_project_scan(client_name);
            if (!(bool)session.get("success", false)) {
                return session;
            }
            if (first_file) {
                reused_lsp = session.get("reused", false);
                project_path = session.get("project_path", "");
                lsp_path = session.get("lsp_path", "");
                FLOG_TOOL(godot::String("C# LSP indexing symbols files=") +
                          godot::String::num_int64(files.size()) +
                          " reused=" + (reused_lsp ? "true" : "false"));
                first_file = false;
            }

            SessionState &state = session_state();

            // Keep C# indexing on the same managed csharp-ls connection as
            // diagnostics. The shared session mutex prevents indexing and
            // diagnostics from interleaving requests on the same stdio pipe,
            // while locking per file lets diagnostics cut ahead between files.
            internal::open_document(state.stdio, abs_path);
            response = internal::request_document_symbols(
                state.stdio, abs_path);
            internal::close_document(state.stdio, abs_path);
            bool request_error = response.has("error") &&
                !(bool)response.get("transport_error", false);
            if (!response.has("result") && !request_error) {
                terminate_session_locked();
                restart_unhealthy_session = true;
            }
        }

        if (!response.has("result")) {
            godot::Dictionary failed;
            failed["path"] = file_utils::uri_to_res_path(internal::file_uri(abs_path));
            failed["error"] = response.has("error")
                ? godot::String(response["error"])
                : godot::String(
                    "csharp-ls did not return textDocument/documentSymbol");
            failed_files.append(failed);
            if (restart_unhealthy_session) {
                launch_preparation_async("fennara-csharp-recovery", false);
                break;
            }
            continue;
        }

        godot::Array flat;
        godot::Variant result_var = response["result"];
        if (result_var.get_type() == godot::Variant::ARRAY) {
            flatten_symbols(result_var, flat);
        }
        per_file[abs_path] = flat;
    }

    godot::Dictionary result;
    result["success"] = true;
    result["per_file"] = per_file;
    result["failed_files"] = failed_files;
    result["project_path"] = project_path;
    result["lsp_path"] = lsp_path;
    result["reused_lsp"] = reused_lsp;
    return result;
}

void shutdown_warm_server() {
    shutting_down().store(true);
    preparation_reserved().store(false);
    internal::request_abort();
    if (warmup_thread().joinable() &&
        warmup_thread().get_id() != std::this_thread::get_id()) {
        warmup_thread().join();
    }
    std::lock_guard<std::mutex> lock(session_mutex());
    shutdown_warm_server_locked();
    csharp_project_graph::invalidate();
    warmup_in_progress().store(false);
    preparation_wait_condition().notify_all();
}

} // namespace fennara::csharp_lsp
