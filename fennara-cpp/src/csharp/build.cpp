#include "fennara/csharp/build.hpp"

#include "fennara/csharp/build_issues.hpp"
#include "fennara/csharp/project.hpp"
#include "fennara/file_utils.hpp"
#include "fennara/logger.hpp"
#include "fennara/process_tree.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

#include <chrono>
#include <condition_variable>
#include <mutex>
#include <thread>

namespace fennara::csharp_build {
namespace {

constexpr int kRetainedBuildLogRuns = 20;

std::atomic_bool &background_build_running() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::atomic_bool &force_next_explicit_build() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::atomic_bool &build_shutdown_requested() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::atomic_bool &preparation_in_progress() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::atomic_bool &preparation_reserved() {
    static std::atomic_bool *value = new std::atomic_bool(false);
    return *value;
}

std::mutex &preparation_wait_mutex() {
    static std::mutex *mutex = new std::mutex();
    return *mutex;
}

std::condition_variable &preparation_wait_condition() {
    static std::condition_variable *condition = new std::condition_variable();
    return *condition;
}

std::thread &preparation_thread() {
    static std::thread *thread = new std::thread();
    return *thread;
}

std::mutex &preparation_thread_mutex() {
    static std::mutex *mutex = new std::mutex();
    return *mutex;
}

std::mutex &build_coordinator_mutex() {
    static std::mutex *mutex = new std::mutex();
    return *mutex;
}

std::condition_variable &build_coordinator_condition() {
    static std::condition_variable *condition = new std::condition_variable();
    return *condition;
}

bool &build_coordinator_busy() {
    static bool *busy = new bool(false);
    return *busy;
}

std::atomic_uint64_t &build_log_sequence() {
    static std::atomic_uint64_t *sequence = new std::atomic_uint64_t(0);
    return *sequence;
}

bool remove_directory_recursive(const godot::String &path) {
    godot::Ref<godot::DirAccess> dir = godot::DirAccess::open(path);
    if (dir.is_null()) {
        return false;
    }

    bool removed = true;
    dir->list_dir_begin();
    godot::String name = dir->get_next();
    while (!name.is_empty()) {
        if (name != "." && name != "..") {
            godot::String child = path.path_join(name);
            bool child_removed = dir->current_is_dir()
                ? remove_directory_recursive(child)
                : godot::DirAccess::remove_absolute(child) == godot::OK;
            removed = child_removed && removed;
        }
        name = dir->get_next();
    }
    dir->list_dir_end();
    return godot::DirAccess::remove_absolute(path) == godot::OK && removed;
}

void prune_build_log_runs(const godot::String &root, int retain_count) {
    godot::Ref<godot::DirAccess> dir = godot::DirAccess::open(root);
    if (dir.is_null()) {
        return;
    }

    godot::PackedStringArray run_names;
    dir->list_dir_begin();
    godot::String name = dir->get_next();
    while (!name.is_empty()) {
        int separator = name.find("-");
        if (dir->current_is_dir() && separator > 0 &&
            name.left(separator).is_valid_int() &&
            name.substr(separator + 1).is_valid_int()) {
            run_names.append(name);
        }
        name = dir->get_next();
    }
    dir->list_dir_end();

    run_names.sort();
    int remove_count = run_names.size() - retain_count;
    for (int i = 0; i < remove_count; i++) {
        // Retention is best effort. A locked log must not fail diagnostics.
        remove_directory_recursive(root.path_join(run_names[i]));
    }
}

class BuildCoordinatorLease {
public:
    ~BuildCoordinatorLease() {
        release();
    }

    bool acquire(const std::atomic_bool *cancelled) {
        std::unique_lock<std::mutex> lock(build_coordinator_mutex());
        build_coordinator_condition().wait(lock, [cancelled]() {
            return !build_coordinator_busy() ||
                   build_shutdown_requested().load() ||
                   (cancelled != nullptr && cancelled->load());
        });
        if (build_shutdown_requested().load() ||
            (cancelled != nullptr && cancelled->load())) {
            return false;
        }
        build_coordinator_busy() = true;
        acquired = true;
        return true;
    }

private:
    void release() {
        if (!acquired) {
            return;
        }
        {
            std::lock_guard<std::mutex> lock(build_coordinator_mutex());
            build_coordinator_busy() = false;
            acquired = false;
        }
        build_coordinator_condition().notify_one();
    }

    bool acquired = false;
};

godot::Dictionary csharp_source_fingerprints() {
    godot::Dictionary fingerprints;
    godot::Array files = file_utils::find_all_diagnostic_files();
    for (int i = 0; i < files.size(); i++) {
        godot::String path = files[i];
        if (!path.to_lower().ends_with(".cs")) {
            continue;
        }
        godot::Dictionary fingerprint;
        fingerprint["modified"] = static_cast<int64_t>(
            godot::FileAccess::get_modified_time(path));
        fingerprint["size"] = godot::FileAccess::get_size(path);
        fingerprints[path.replace("\\", "/").simplify_path()] = fingerprint;
    }
    return fingerprints;
}

struct BackgroundBuildScope {
    explicit BackgroundBuildScope(bool active) : active(active) {
        if (active) {
            background_build_running().store(true);
            source_fingerprints = csharp_source_fingerprints();
        }
    }

    ~BackgroundBuildScope() {
        if (active) {
            if (source_fingerprints != csharp_source_fingerprints()) {
                force_next_explicit_build().store(true);
            }
            background_build_running().store(false);
        }
    }

    bool active;
    godot::Dictionary source_fingerprints;
};

uint64_t now_ms() {
    godot::Time *time = godot::Time::get_singleton();
    return time == nullptr ? 0 : static_cast<uint64_t>(time->get_ticks_msec());
}

godot::String project_root() {
    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    return settings == nullptr ? godot::String() : settings->globalize_path("res://");
}

godot::String shell_quote(const godot::String &value) {
    return "\"" + value.replace("\"", "\\\"") + "\"";
}

godot::String join_args(const godot::PackedStringArray &args) {
    godot::PackedStringArray quoted;
    for (int i = 0; i < args.size(); i++) {
        godot::String arg = args[i];
        quoted.append(arg.find(" ") >= 0 ? shell_quote(arg) : arg);
    }
    return godot::String(" ").join(quoted);
}

godot::String read_available_output(
    const godot::Ref<godot::FileAccess> &pipe);

godot::Dictionary run_command_blocking(const godot::String &command,
                                       const godot::PackedStringArray &args,
                                       const godot::String &display_command,
                                       const godot::String &working_directory,
                                       const std::atomic_bool *cancelled) {
    uint64_t start = now_ms();
    godot::Dictionary result;
    result["command"] = display_command.is_empty()
                            ? command + (args.is_empty() ? "" : " " + join_args(args))
                            : display_command;
    result["working_directory"] = working_directory;
    godot::Dictionary process =
        godot::OS::get_singleton()->execute_with_pipe(command, args, false);
    if (!process.has("stdio")) {
        result["exit_code"] = -1;
        result["duration_seconds"] = (double)(now_ms() - start) / 1000.0;
        result["status"] = "failed";
        result["output"] = "Failed to start " + command + ".";
        return result;
    }

    godot::Ref<godot::FileAccess> stdio = process["stdio"];
    godot::Ref<godot::FileAccess> stderr_pipe = process.get(
        "stderr", godot::Ref<godot::FileAccess>());
    int pid = static_cast<int>(process.get("pid", -1));
    godot::String output;
    godot::OS *os = godot::OS::get_singleton();
    while (pid > 0 && os->is_process_running(pid)) {
        output += read_available_output(stdio);
        output += read_available_output(stderr_pipe);
        if (build_shutdown_requested().load() ||
            (cancelled != nullptr && cancelled->load())) {
            process_tree::terminate_and_wait(pid);
            result["exit_code"] = -1;
            result["duration_seconds"] = (double)(now_ms() - start) / 1000.0;
            result["status"] = "failed";
            result["cancelled"] = true;
            result["output"] = output;
            return result;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    output += read_available_output(stdio);
    output += read_available_output(stderr_pipe);
    int exit_code = pid > 0 ? os->get_process_exit_code(pid) : -1;
    result["exit_code"] = exit_code;
    result["duration_seconds"] = (double)(now_ms() - start) / 1000.0;
    result["output"] = output;
    result["status"] = exit_code == 0 ? "success" : "failed";
    return result;
}

godot::String godot_build_logger_path() {
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr) {
        return "";
    }
    godot::String path = os->get_executable_path()
        .get_base_dir()
        .path_join("GodotSharp")
        .path_join("Tools")
        .path_join("GodotTools.BuildLogger.dll");
    if (os->get_name() == "macOS" && !godot::FileAccess::file_exists(path)) {
        path = os->get_executable_path()
            .get_base_dir()
            .path_join("../Resources")
            .simplify_path()
            .path_join("GodotSharp")
            .path_join("Tools")
            .path_join("GodotTools.BuildLogger.dll");
    }
    return path;
}

godot::String platform_name() {
    godot::OS *os = godot::OS::get_singleton();
    if (os == nullptr) {
        return "";
    }
    godot::String name = os->get_name();
    if (name == "Windows") {
        return "windows";
    }
    if (name == "macOS") {
        return "macos";
    }
    if (name == "Linux") {
        return "linuxbsd";
    }
    return "";
}

godot::String read_available_output(const godot::Ref<godot::FileAccess> &pipe) {
    godot::PackedByteArray bytes;
    while (pipe.is_valid()) {
        godot::PackedByteArray chunk = pipe->get_buffer(4096);
        if (chunk.is_empty()) {
            break;
        }
        bytes.append_array(chunk);
        if (chunk.size() < 4096) {
            break;
        }
    }
    return bytes.get_string_from_utf8();
}

godot::Dictionary empty_file_diagnostics() {
    godot::Dictionary file;
    file["diagnostics"] = godot::Array();
    file["total_errors"] = 0;
    file["total_warnings"] = 0;
    file["total_info"] = 0;
    file["total_hints"] = 0;
    file["total_diagnostics"] = 0;
    return file;
}

godot::Dictionary group_issues_by_file(const godot::Array &issues,
                                       const godot::String &fallback_project) {
    godot::Dictionary per_file;
    for (int i = 0; i < issues.size(); i++) {
        if (issues[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary issue = issues[i];
        godot::String path = issue.get("file", "");
        if (path.is_empty()) {
            path = issue.get("project", fallback_project);
        }
        if (path.is_empty()) {
            path = fallback_project;
        }
        path = path.replace("\\", "/").simplify_path();
        godot::Dictionary file = per_file.get(path, empty_file_diagnostics());
        godot::String severity = godot::String(issue.get("severity", "error")).to_lower();
        if (severity != "error" && severity != "warning" &&
            severity != "info" && severity != "hint") {
            severity = "error";
        }

        godot::Dictionary diagnostic;
        diagnostic["line"] = issue.get("line", 0);
        diagnostic["column"] = issue.get("column", 0);
        diagnostic["severity"] = severity;
        diagnostic["code"] = issue.get("code", "");
        diagnostic["message"] = issue.get("message", "");
        diagnostic["source"] = "dotnet_build";
        godot::Array diagnostics = file.get("diagnostics", godot::Array());
        diagnostics.append(diagnostic);
        file["diagnostics"] = diagnostics;

        godot::String counter = severity == "error" ? "total_errors" :
                                severity == "warning" ? "total_warnings" :
                                severity == "hint" ? "total_hints" : "total_info";
        file[counter] = static_cast<int>(file.get(counter, 0)) + 1;
        file["total_diagnostics"] =
            static_cast<int>(file.get("total_diagnostics", 0)) + 1;
        per_file[path] = file;
    }
    return per_file;
}

godot::Dictionary resolve_godot_csproj() {
    godot::Dictionary status = csharp_support::inspect_project();
    godot::Array projects = status.get("projects", godot::Array());
    godot::Array all_csproj;

    for (int i = 0; i < projects.size(); i++) {
        if (projects[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary project = projects[i];
        if (godot::String(project.get("type", "")) != "project") {
            continue;
        }
        godot::String path = godot::String(
            project.get("absolute_path", "")).replace("\\", "/");
        all_csproj.append(path);
    }

    godot::Dictionary result;
    godot::Dictionary selected =
        status.get("selected_project", godot::Dictionary());
    if (godot::String(selected.get("type", "")) == "project") {
        result["success"] = true;
        result["path"] = selected.get("absolute_path", "");
        result["selection_reason"] =
            selected.get("selection_reason", "selected_project");
        return result;
    }
    result["success"] = false;
    result["candidates"] = all_csproj;
    result["error"] = all_csproj.is_empty()
        ? "No C# project file was found for this Godot project."
        : "Multiple C# project files were found and the Godot project settings "
          "did not identify one unambiguously.";
    return result;
}

} // namespace

void begin_build_lifecycle() {
    build_shutdown_requested().store(false);
}

void request_build_shutdown() {
    build_shutdown_requested().store(true);
    preparation_wait_condition().notify_all();
    build_coordinator_condition().notify_all();
}

void notify_build_waiters() {
    preparation_wait_condition().notify_all();
    build_coordinator_condition().notify_all();
}

void reserve_background_preparation() {
    if (build_shutdown_requested().load()) {
        return;
    }
    preparation_reserved().store(true);
    preparation_in_progress().store(true);
}

void cancel_reserved_background_preparation() {
    if (!preparation_reserved().exchange(false)) {
        return;
    }
    preparation_in_progress().store(false);
    preparation_wait_condition().notify_all();
}

void start_background_preparation_async() {
    if (build_shutdown_requested().load()) {
        return;
    }
    bool was_reserved = preparation_reserved().exchange(false);
    if (!was_reserved) {
        bool expected = false;
        if (!preparation_in_progress().compare_exchange_strong(expected, true)) {
            return;
        }
    }

    std::lock_guard<std::mutex> thread_lock(preparation_thread_mutex());
    if (build_shutdown_requested().load()) {
        return;
    }
    if (preparation_thread().joinable()) {
        preparation_thread().join();
    }
    preparation_thread() = std::thread([]() {
        Logger::log_activity("C# background preparation started");
        godot::Dictionary support = csharp_support::inspect_project();
        godot::Dictionary selected =
            support.get("selected_project", godot::Dictionary());
        godot::String selected_type = selected.get("type", "");
        bool build_ready =
            !godot::String(selected.get("absolute_path", "")).is_empty() &&
            (selected_type == "project" || selected_type == "solution");

        if (build_ready) {
            godot::Dictionary build =
                run_background_diagnostics(&build_shutdown_requested());
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
        } else {
            Logger::log_activity(
                "C# background isolated build skipped: " +
                godot::String(support.get(
                    "message", "no unambiguous C# project")));
        }

        preparation_in_progress().store(false);
        preparation_wait_condition().notify_all();
        Logger::log_activity("C# background preparation complete");
    });
}

bool wait_for_background_preparation(
    const godot::String &activity,
    const std::atomic_bool *cancelled) {
    if (!preparation_in_progress().load()) {
        return true;
    }
    Logger::log_activity(
        activity + godot::String(" waiting for C# background preparation"));
    std::unique_lock<std::mutex> lock(preparation_wait_mutex());
    while (preparation_in_progress().load()) {
        if ((cancelled != nullptr && cancelled->load()) ||
            build_shutdown_requested().load()) {
            return false;
        }
        preparation_wait_condition().wait_for(
            lock, std::chrono::milliseconds(50));
    }
    Logger::log_activity(
        activity + godot::String(" continuing after C# background preparation"));
    return true;
}

void shutdown_background_preparation() {
    preparation_reserved().store(false);
    {
        std::lock_guard<std::mutex> thread_lock(preparation_thread_mutex());
        if (preparation_thread().joinable() &&
            preparation_thread().get_id() != std::this_thread::get_id()) {
            preparation_thread().join();
            preparation_thread() = std::thread();
        }
    }
    preparation_in_progress().store(false);
    preparation_wait_condition().notify_all();
}

godot::String find_root_csproj() {
    godot::Dictionary resolved = resolve_godot_csproj();
    return (bool)resolved.get("success", false)
        ? godot::String(resolved.get("path", ""))
        : godot::String();
}

godot::Dictionary run_dotnet_build_if_needed(
    const std::atomic_bool *cancelled) {
    if (!wait_for_background_preparation("C# runtime build", cancelled)) {
        godot::Dictionary cancelled_result;
        cancelled_result["needed"] = true;
        cancelled_result["status"] = "failed";
        cancelled_result["message"] =
            "C# runtime build cancelled while waiting for background preparation.";
        cancelled_result["updates_godot_assembly"] = false;
        return cancelled_result;
    }
    BuildCoordinatorLease build_lease;
    if (!build_lease.acquire(cancelled)) {
        godot::Dictionary cancelled_result;
        cancelled_result["needed"] = true;
        cancelled_result["status"] = "failed";
        cancelled_result["message"] = "C# runtime build cancelled during plugin shutdown.";
        cancelled_result["updates_godot_assembly"] = false;
        return cancelled_result;
    }
    godot::Dictionary result;
    godot::Dictionary resolved_project = resolve_godot_csproj();
    if (!(bool)resolved_project.get("success", false)) {
        godot::Array candidates = resolved_project.get(
            "candidates", godot::Array());
        bool has_csharp_project = !candidates.is_empty();
        result["needed"] = has_csharp_project;
        result["status"] = has_csharp_project ? "failed" : "skipped";
        result["message"] = resolved_project.get(
            "error", "Could not identify the Godot C# project.");
        result["candidates"] = candidates;
        result["output_mode"] = "none";
        result["updates_godot_assembly"] = false;
        return result;
    }
    godot::String csproj_path = resolved_project.get("path", "");

    godot::String root = project_root();
    godot::PackedStringArray args;
    args.append("build");
    args.append(csproj_path);
    args.append("-c");
    args.append("Debug");
    args.append("-v");
    args.append("minimal");
    args.append("-nologo");
    args.append("-nodeReuse:false");
    args.append("-m:1");
    godot::String platform = platform_name();
    if (!platform.is_empty()) {
        args.append("-p:GodotTargetPlatform=" + platform);
    }
#ifdef REAL_T_IS_DOUBLE
    args.append("-p:GodotFloat64=true");
#endif

    result = run_command_blocking(
        "dotnet", args,
        "dotnet build " + shell_quote(csproj_path) + " -c Debug", root,
        cancelled);
    result["needed"] = true;
    result["project"] = csproj_path;
    result["project_selection_reason"] =
        resolved_project.get("selection_reason", "");
    result["output_mode"] = "godot_runtime";
    result["godot_output_root"] = root
        .path_join(".godot")
        .path_join("mono")
        .path_join("temp")
        .path_join("bin")
        .path_join("Debug");
    result["updates_godot_assembly"] = true;
    result["may_trigger_editor_reload"] = true;
    return result;
}

godot::Dictionary run_diagnostics_impl(const std::atomic_bool *cancelled,
                                       bool background) {
    if (!background && !wait_for_background_preparation(
                           "C# project diagnostics", cancelled)) {
        godot::Dictionary cancelled_result;
        cancelled_result["success"] = false;
        cancelled_result["cancelled"] = true;
        cancelled_result["error"] =
            "C# project diagnostics cancelled while waiting for background preparation.";
        return cancelled_result;
    }
    BuildCoordinatorLease build_lease;
    if (!build_lease.acquire(cancelled)) {
        godot::Dictionary cancelled_result;
        cancelled_result["success"] = false;
        cancelled_result["cancelled"] = true;
        cancelled_result["error"] =
            "C# project diagnostics cancelled while waiting for another C# build.";
        return cancelled_result;
    }
    BackgroundBuildScope background_scope(background);
    bool force_refresh = !background && force_next_explicit_build().load();

    godot::Dictionary status = csharp_support::inspect_project();
    godot::Dictionary selected = status.get("selected_project", godot::Dictionary());
    godot::String selected_path = selected.get("absolute_path", "");
    godot::Dictionary result;
    if (selected_path.is_empty()) {
        result["success"] = false;
        result["error"] = status.get(
            "message", "No unambiguous C# project or solution was found.");
        return result;
    }

    godot::String logger_path = godot_build_logger_path();
    if (!godot::FileAccess::file_exists(logger_path)) {
        result["success"] = false;
        result["error"] = "Godot's .NET build logger was not found at " + logger_path;
        return result;
    }

    godot::String build_logs_root = project_root()
        .path_join(".godot")
        .path_join("fennara")
        .path_join("build_logs");
    godot::DirAccess::make_dir_recursive_absolute(build_logs_root);
    prune_build_log_runs(build_logs_root, kRetainedBuildLogRuns - 1);
    godot::String logs_dir = build_logs_root.path_join(
        godot::String::num_uint64(now_ms()) + "-" +
        godot::String::num_uint64(build_log_sequence().fetch_add(1) + 1));
    godot::DirAccess::make_dir_recursive_absolute(logs_dir);

    godot::String diagnostic_output_root = project_root()
        .path_join(".godot")
        .path_join("fennara")
        .path_join("diagnostic_build");
    godot::DirAccess::make_dir_recursive_absolute(diagnostic_output_root);

    godot::String output_targets_path = project_root()
        .path_join(".godot")
        .path_join("fennara")
        .path_join("diagnostic_output.targets");
    godot::String output_targets_content =
        "<Project>\n"
        "  <PropertyGroup>\n"
        "    <OutputPath>$(FennaraDiagnosticsRoot)/$(MSBuildProjectName)/$(Configuration)/</OutputPath>\n"
        "  </PropertyGroup>\n"
        "</Project>\n";
    bool write_output_targets = true;
    if (godot::FileAccess::file_exists(output_targets_path)) {
        godot::Ref<godot::FileAccess> existing = godot::FileAccess::open(
            output_targets_path, godot::FileAccess::READ);
        if (existing.is_valid()) {
            write_output_targets =
                existing->get_as_text() != output_targets_content;
        }
    }
    if (write_output_targets) {
        godot::Ref<godot::FileAccess> output_targets = godot::FileAccess::open(
            output_targets_path, godot::FileAccess::WRITE);
        if (output_targets.is_null()) {
            result["success"] = false;
            result["error"] =
                "Failed to create the isolated C# diagnostic build targets file.";
            return result;
        }
        output_targets->store_string(output_targets_content);
        output_targets->close();
    }

    godot::PackedStringArray args;
    args.append("build");
    args.append(selected_path);
    args.append("-c");
    args.append("Debug");
    args.append("-v");
    args.append("minimal");
    args.append("-nologo");
    args.append("-nodeReuse:false");
    if (force_refresh) {
        args.append("--no-incremental");
    }
    godot::String platform = platform_name();
    if (!platform.is_empty()) {
        args.append("-p:GodotTargetPlatform=" + platform);
    }
#ifdef REAL_T_IS_DOUBLE
    args.append("-p:GodotFloat64=true");
#endif
    args.append("-p:CustomBeforeMicrosoftCommonTargets=" + output_targets_path);
    args.append("-p:FennaraDiagnosticsRoot=" + diagnostic_output_root);
    args.append("-l:GodotTools.BuildLogger.GodotBuildLogger," + logger_path + ";" + logs_dir);

    uint64_t started = now_ms();
    godot::Dictionary process =
        godot::OS::get_singleton()->execute_with_pipe("dotnet", args, false);
    if (!process.has("stdio")) {
        result["success"] = false;
        result["error"] = "Failed to start dotnet build.";
        return result;
    }

    godot::Ref<godot::FileAccess> stdio = process["stdio"];
    godot::Ref<godot::FileAccess> stderr_pipe = process.get(
        "stderr", godot::Ref<godot::FileAccess>());
    int pid = static_cast<int>(process.get("pid", -1));
    godot::String output;
    godot::OS *os = godot::OS::get_singleton();
    while (pid > 0 && os->is_process_running(pid)) {
        output += read_available_output(stdio);
        output += read_available_output(stderr_pipe);
        if (build_shutdown_requested().load() ||
            (cancelled != nullptr && cancelled->load())) {
            process_tree::terminate_and_wait(pid);
            result["success"] = false;
            result["cancelled"] = true;
            result["error"] = "C# project diagnostics cancelled.";
            result["duration_seconds"] = (double)(now_ms() - started) / 1000.0;
            return result;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    output += read_available_output(stdio);
    output += read_available_output(stderr_pipe);

    int exit_code = pid > 0 ? os->get_process_exit_code(pid) : -1;
    godot::Dictionary snapshot =
        csharp_build_issues::snapshot_from_directory(logs_dir);
    result["success"] = true;
    result["build_succeeded"] = exit_code == 0;
    result["exit_code"] = exit_code;
    result["project_path"] = selected_path;
    result["duration_seconds"] = (double)(now_ms() - started) / 1000.0;
    result["issues"] = snapshot.get("issues", godot::Array());
    result["issue_count"] = snapshot.get("issue_count", 0);
    result["per_file"] = group_issues_by_file(
        snapshot.get("issues", godot::Array()), selected_path);
    result["output"] = output;
    result["logs_dir"] = logs_dir;
    result["output_mode"] = "isolated_diagnostics";
    result["diagnostic_output_root"] = diagnostic_output_root;
    result["updates_godot_assembly"] = false;
    result["triggers_editor_reload"] = false;
    result["forced_refresh"] = force_refresh;
    result["background_preparation"] = background;
    if (force_refresh) {
        force_next_explicit_build().store(false);
    }
    return result;
}

godot::Dictionary run_diagnostics(const std::atomic_bool *cancelled) {
    return run_diagnostics_impl(cancelled, false);
}

godot::Dictionary run_background_diagnostics(
    const std::atomic_bool *cancelled) {
    return run_diagnostics_impl(cancelled, true);
}

void note_csharp_source_changed() {
    if (background_build_running().load()) {
        force_next_explicit_build().store(true);
    }
}

} // namespace fennara::csharp_build
