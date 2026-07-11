#include "fennara/lsp/csharp_project_graph.hpp"

#include "fennara/lsp/csharp_support.hpp"
#include "fennara/process_tree.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/project_settings.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>

#include <atomic>
#include <chrono>
#include <mutex>
#include <string>
#include <thread>
#include <unordered_set>

namespace fennara::csharp_project_graph {
namespace {

std::mutex &cache_mutex() {
    static std::mutex *value = new std::mutex();
    return *value;
}

godot::Dictionary &cached_graph() {
    static godot::Dictionary *value = new godot::Dictionary();
    return *value;
}

std::atomic_int64_t &graph_generation() {
    static std::atomic_int64_t *value = new std::atomic_int64_t(0);
    return *value;
}

std::atomic_int64_t &cache_epoch() {
    static std::atomic_int64_t *value = new std::atomic_int64_t(0);
    return *value;
}

constexpr uint64_t kEvaluationTimeoutMs = 30000;

godot::String canonical_path(const godot::String &path) {
    godot::String clean = path.replace("\\", "/").simplify_path();
    godot::OS *os = godot::OS::get_singleton();
    return os != nullptr && os->get_name() == "Windows" ? clean.to_lower() : clean;
}

godot::String read_available_output(
    const godot::Ref<godot::FileAccess> &pipe) {
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

godot::Dictionary run_command(
    const godot::String &command,
    const godot::PackedStringArray &args,
    const std::atomic_bool *cancelled,
    uint64_t deadline_ms) {
    godot::Dictionary process =
        godot::OS::get_singleton()->execute_with_pipe(command, args, false);
    godot::Dictionary result;
    if (!process.has("stdio")) {
        result["exit_code"] = -1;
        result["error"] = "Failed to start " + command + ".";
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
        bool was_cancelled = cancelled != nullptr && cancelled->load();
        bool timed_out =
            godot::Time::get_singleton()->get_ticks_msec() >= deadline_ms;
        if (was_cancelled || timed_out) {
            process_tree::terminate_and_wait(pid);
            result["exit_code"] = -1;
            result["cancelled"] = was_cancelled;
            result["timed_out"] = timed_out;
            result["error"] = was_cancelled
                ? godot::String("C# project graph evaluation cancelled.")
                : godot::String("C# project graph evaluation timed out after 30 seconds.");
            result["output"] = output;
            return result;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    output += read_available_output(stdio);
    output += read_available_output(stderr_pipe);
    result["exit_code"] = pid > 0 ? os->get_process_exit_code(pid) : -1;
    result["output"] = output;
    return result;
}

godot::Array solution_projects(const godot::String &solution_path,
                               const std::atomic_bool *cancelled,
                               uint64_t deadline_ms,
                               godot::String &error) {
    godot::PackedStringArray args;
    args.append("sln");
    args.append(solution_path);
    args.append("list");
    godot::Dictionary command = run_command(
        "dotnet", args, cancelled, deadline_ms);
    godot::Array projects;
    if (int(command.get("exit_code", -1)) != 0) {
        error = command.get(
            "error", godot::String(command.get("output", "")).strip_edges());
        return projects;
    }

    const godot::String base_dir = solution_path.get_base_dir();
    godot::PackedStringArray lines = godot::String(command.get("output", "")).split("\n");
    for (int i = 0; i < lines.size(); i++) {
        godot::String line = lines[i].strip_edges();
        if (!line.ends_with(".csproj") && !line.ends_with(".fsproj") &&
            !line.ends_with(".vbproj")) {
            continue;
        }
        godot::String path = line.is_absolute_path() ? line : base_dir.path_join(line);
        projects.append(path.simplify_path());
    }
    return projects;
}

godot::Dictionary evaluate_project(const godot::String &project_path,
                                   const std::atomic_bool *cancelled,
                                   uint64_t deadline_ms) {
    godot::PackedStringArray args;
    args.append("msbuild");
    args.append(project_path);
    args.append("-nologo");
    args.append("-getItem:Compile,ProjectReference");
    args.append("-getProperty:MSBuildProjectFullPath,MSBuildAllProjects");
    args.append("-nodeReuse:false");
    args.append("-m:1");
    godot::Dictionary command = run_command(
        "dotnet", args, cancelled, deadline_ms);

    godot::Dictionary result;
    result["project"] = project_path;
    result["exit_code"] = command.get("exit_code", -1);
    if (int(command.get("exit_code", -1)) != 0) {
        result["error"] = command.get(
            "error", godot::String(command.get("output", "")).strip_edges());
        return result;
    }

    godot::String output = command.get("output", "");
    int json_start = output.find("{");
    godot::Variant parsed = json_start < 0
                                ? godot::Variant()
                                : godot::JSON::parse_string(output.substr(json_start));
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        result["error"] = "MSBuild returned invalid project evaluation JSON.";
        return result;
    }

    godot::Dictionary data = parsed;
    godot::Dictionary properties = data.get("Properties", godot::Dictionary());
    godot::Dictionary items = data.get("Items", godot::Dictionary());
    result["project"] = godot::String(properties.get("MSBuildProjectFullPath", project_path));
    result["compile"] = items.get("Compile", godot::Array());
    result["references"] = items.get("ProjectReference", godot::Array());
    result["imports"] = properties.get("MSBuildAllProjects", "");
    result["ok"] = true;
    return result;
}

godot::Dictionary dependency_fingerprint(const godot::String &path) {
    godot::Dictionary fingerprint;
    fingerprint["modified"] =
        static_cast<int64_t>(godot::FileAccess::get_modified_time(path));
    godot::Ref<godot::FileAccess> file = godot::FileAccess::open(
        path, godot::FileAccess::READ);
    fingerprint["size"] = file.is_valid() ? file->get_length() : -1;
    fingerprint["sha256"] = file.is_valid()
        ? godot::FileAccess::get_file_as_string(path).sha256_text()
        : godot::String();
    return fingerprint;
}

void append_dependency(godot::Dictionary &fingerprints,
                       const godot::String &path) {
    godot::String canonical = canonical_path(path);
    if (canonical.is_empty() || fingerprints.has(canonical)) {
        return;
    }
    if (canonical.contains("/obj/") || canonical.contains("/.godot/") ||
        canonical.contains("/program files/dotnet/sdk/") ||
        canonical.contains("/.nuget/packages/")) {
        return;
    }
    fingerprints[canonical] = dependency_fingerprint(path);
}

bool cache_is_current(const godot::Dictionary &graph,
                      const godot::String &selected_path) {
    if (graph.is_empty() || canonical_path(graph.get("selected_path", "")) !=
                                canonical_path(selected_path)) {
        return false;
    }
    godot::Dictionary fingerprints = graph.get(
        "dependency_fingerprints", godot::Dictionary());
    godot::Array paths = fingerprints.keys();
    for (int i = 0; i < paths.size(); i++) {
        godot::String path = paths[i];
        if (dependency_fingerprint(path) !=
            godot::Dictionary(fingerprints[path])) {
            return false;
        }
    }
    return true;
}

godot::String reference_path(const godot::Dictionary &item,
                             const godot::String &project_dir) {
    godot::String path = item.get("FullPath", "");
    if (path.is_empty()) {
        path = item.get("Identity", "");
        if (!path.is_absolute_path()) {
            path = project_dir.path_join(path);
        }
    }
    return path.simplify_path();
}

} // namespace

godot::Dictionary evaluate_selected(const std::atomic_bool *cancelled) {
    godot::Dictionary status = csharp_support::inspect_project();
    godot::Dictionary selected = status.get("selected_project", godot::Dictionary());
    godot::String selected_path = selected.get("absolute_path", "");
    godot::Dictionary graph;
    graph["selected_path"] = selected_path;
    if (status.get("state", "") != godot::String("ready") || selected_path.is_empty()) {
        graph["ok"] = false;
        graph["error"] = csharp_support::diagnostics_unavailable_message(status);
        return graph;
    }

    {
        std::lock_guard<std::mutex> lock(cache_mutex());
        if (cache_is_current(cached_graph(), selected_path)) {
            return cached_graph();
        }
    }
    int64_t evaluation_epoch = cache_epoch().load();
    uint64_t deadline_ms =
        godot::Time::get_singleton()->get_ticks_msec() + kEvaluationTimeoutMs;

    godot::Array pending;
    godot::String selected_type = selected.get("type", "project");
    if (selected_type == "solution") {
        godot::String solution_error;
        pending = solution_projects(
            selected_path, cancelled, deadline_ms, solution_error);
        if (pending.is_empty()) {
            graph["ok"] = false;
            graph["error"] = solution_error.is_empty()
                ? godot::String(
                    "The selected solution contains no supported projects or could not be read.")
                : solution_error;
            return graph;
        }
    } else {
        pending.append(selected_path);
    }

    godot::Array projects;
    godot::Dictionary owners;
    godot::Dictionary fingerprints;
    std::unordered_set<std::string> visited;
    append_dependency(fingerprints, selected_path);

    for (int index = 0; index < pending.size(); index++) {
        godot::String project_path = godot::String(pending[index]).simplify_path();
        godot::String canonical = canonical_path(project_path);
        std::string key = canonical.utf8().get_data();
        if (!visited.insert(key).second) {
            continue;
        }

        godot::Dictionary project = evaluate_project(
            project_path, cancelled, deadline_ms);
        if (!bool(project.get("ok", false))) {
            graph["ok"] = false;
            graph["error"] = "Failed to evaluate " + project_path + ": " +
                             godot::String(project.get("error", "Unknown MSBuild error."));
            return graph;
        }
        projects.append(project);
        append_dependency(fingerprints, project_path);

        godot::PackedStringArray imports =
            godot::String(project.get("imports", "")).split(";", false);
        for (int i = 0; i < imports.size(); i++) {
            append_dependency(fingerprints, imports[i]);
        }

        godot::Array compile = project.get("compile", godot::Array());
        for (int i = 0; i < compile.size(); i++) {
            godot::Dictionary item = compile[i];
            godot::String file = item.get("FullPath", "");
            if (file.is_empty()) {
                file = project_path.get_base_dir().path_join(item.get("Identity", ""));
            }
            godot::String file_key = canonical_path(file);
            godot::Array file_owners = owners.get(file_key, godot::Array());
            file_owners.append(project.get("project", project_path));
            owners[file_key] = file_owners;
        }

        godot::Array references = project.get("references", godot::Array());
        for (int i = 0; i < references.size(); i++) {
            pending.append(reference_path(references[i], project_path.get_base_dir()));
        }
    }

    godot::ProjectSettings *settings = godot::ProjectSettings::get_singleton();
    if (settings != nullptr) {
        append_dependency(
            fingerprints, settings->globalize_path("res://global.json"));
    }
    graph["ok"] = true;
    graph["generation"] = graph_generation().fetch_add(1) + 1;
    graph["projects"] = projects;
    graph["owners"] = owners;
    graph["dependency_fingerprints"] = fingerprints;
    if (cache_epoch().load() == evaluation_epoch) {
        std::lock_guard<std::mutex> lock(cache_mutex());
        if (cache_epoch().load() == evaluation_epoch) {
            cached_graph() = graph;
        }
    }
    return graph;
}

godot::Array owners_for_file(const godot::Dictionary &graph,
                             const godot::String &absolute_file_path) {
    godot::Dictionary owners = graph.get("owners", godot::Dictionary());
    return owners.get(canonical_path(absolute_file_path), godot::Array());
}

void invalidate() {
    cache_epoch().fetch_add(1);
    std::lock_guard<std::mutex> lock(cache_mutex());
    cached_graph().clear();
}

} // namespace fennara::csharp_project_graph
