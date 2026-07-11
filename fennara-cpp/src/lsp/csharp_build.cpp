#include "fennara/lsp/csharp_build.hpp"

#include "fennara/lsp/csharp_build_issues.hpp"
#include "fennara/lsp/csharp_lsp.hpp"
#include "fennara/lsp/csharp_support.hpp"
#include "fennara/file_utils.hpp"
#include "fennara/process_tree.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

#include <chrono>
#include <mutex>
#include <thread>

namespace fennara::csharp_build {
namespace {

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

std::mutex &build_coordinator_mutex() {
    static std::mutex *mutex = new std::mutex();
    return *mutex;
}

std::atomic_uint64_t &build_log_sequence() {
    static std::atomic_uint64_t *sequence = new std::atomic_uint64_t(0);
    return *sequence;
}

bool acquire_build_coordinator(
    std::unique_lock<std::mutex> &lock,
    const std::atomic_bool *cancelled) {
    while (!lock.try_lock()) {
        if (build_shutdown_requested().load() ||
            (cancelled != nullptr && cancelled->load())) {
            return false;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(25));
    }
    return true;
}

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
        godot::Ref<godot::FileAccess> file = godot::FileAccess::open(
            path, godot::FileAccess::READ);
        fingerprint["size"] = file.is_valid() ? file->get_length() : -1;
        fingerprint["sha256"] = file.is_valid()
            ? godot::FileAccess::get_file_as_string(path).sha256_text()
            : godot::String();
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

godot::String absolute_under_root(const godot::String &root,
                                  const godot::String &relative) {
    godot::String clean = relative.strip_edges().replace("\\", "/");
    if (clean.is_empty() || clean == ".") {
        return root;
    }
    if (clean.begins_with("res://")) {
        godot::ProjectSettings *settings =
            godot::ProjectSettings::get_singleton();
        return settings == nullptr ? clean : settings->globalize_path(clean);
    }
    if (clean.is_absolute_path()) {
        return clean;
    }
    return root.path_join(clean);
}

godot::Dictionary resolve_godot_csproj() {
    godot::Dictionary status = csharp_support::inspect_project();
    godot::String root = status.get("project_root", project_root());
    godot::String configured_dir = absolute_under_root(
        root, status.get("dotnet_project_directory", ""));
    configured_dir = configured_dir.replace("\\", "/").trim_suffix("/");
    godot::String assembly_name =
        godot::String(status.get("dotnet_assembly_name", "")).strip_edges();
    godot::Array projects = status.get("projects", godot::Array());
    godot::Array directory_matches;
    godot::Array named_matches;
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
        if (path.get_base_dir().trim_suffix("/").to_lower() ==
            configured_dir.to_lower()) {
            directory_matches.append(path);
        }
        if (!assembly_name.is_empty() &&
            path.get_file().get_basename().to_lower() ==
                assembly_name.to_lower()) {
            named_matches.append(path);
        }
    }

    godot::Dictionary result;
    for (int i = 0; i < directory_matches.size(); i++) {
        godot::String path = directory_matches[i];
        if (!assembly_name.is_empty() &&
            path.get_file().get_basename().to_lower() ==
                assembly_name.to_lower()) {
            result["success"] = true;
            result["path"] = path;
            result["selection_reason"] =
                "dotnet_project_directory_and_assembly_name";
            return result;
        }
    }
    if (directory_matches.size() == 1) {
        result["success"] = true;
        result["path"] = directory_matches[0];
        result["selection_reason"] = "dotnet_project_directory";
        return result;
    }
    if (named_matches.size() == 1) {
        result["success"] = true;
        result["path"] = named_matches[0];
        result["selection_reason"] = "dotnet_assembly_name";
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
}

godot::String find_root_csproj() {
    godot::Dictionary resolved = resolve_godot_csproj();
    return (bool)resolved.get("success", false)
        ? godot::String(resolved.get("path", ""))
        : godot::String();
}

godot::Dictionary run_dotnet_build_if_needed() {
    if (!csharp_lsp::wait_for_background_preparation("C# runtime build")) {
        godot::Dictionary cancelled_result;
        cancelled_result["needed"] = true;
        cancelled_result["status"] = "failed";
        cancelled_result["message"] =
            "C# runtime build cancelled while waiting for background preparation.";
        cancelled_result["updates_godot_assembly"] = false;
        return cancelled_result;
    }
    std::unique_lock<std::mutex> build_lock(
        build_coordinator_mutex(), std::defer_lock);
    if (!acquire_build_coordinator(build_lock, nullptr)) {
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
        &build_shutdown_requested());
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
    if (!background && !csharp_lsp::wait_for_background_preparation(
                           "C# project diagnostics", cancelled)) {
        godot::Dictionary cancelled_result;
        cancelled_result["success"] = false;
        cancelled_result["cancelled"] = true;
        cancelled_result["error"] =
            "C# project diagnostics cancelled while waiting for background preparation.";
        return cancelled_result;
    }
    std::unique_lock<std::mutex> build_lock(
        build_coordinator_mutex(), std::defer_lock);
    if (!acquire_build_coordinator(build_lock, cancelled)) {
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

    godot::String logs_dir = project_root()
        .path_join(".godot")
        .path_join("fennara")
        .path_join("build_logs")
        .path_join(
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
