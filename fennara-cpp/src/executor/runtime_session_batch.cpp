#include "fennara/executor.hpp"

#include "fennara/csharp/build.hpp"
#include "fennara/runtime/runtime_scene_preflight.hpp"
#include "fennara/tools/runtime_session.hpp"
#include "fennara/tools/validate_scene.hpp"

#include <godot_cpp/classes/scene_tree.hpp>
#include <godot_cpp/classes/scene_tree_timer.hpp>

#include <utility>

namespace fennara {
namespace {

constexpr double kPollSeconds = 0.1;

godot::Dictionary make_runtime_session_error(const godot::String &message) {
    godot::Dictionary result;
    result["success"] = false;
    result["tool_name"] = "runtime_session";
    result["format_version"] = "runtime-session-result-v1";
    result["status"] = "blocked";
    result["error"] = message;
    return result;
}

} // namespace

bool FennaraExecutor::_schedule_runtime_session_poll(uint64_t batch_generation) {
    godot::SceneTree *tree = get_tree();
    if (!tree) {
        return false;
    }
    godot::Ref<godot::SceneTreeTimer> timer = tree->create_timer(kPollSeconds);
    timer->connect(
        "timeout",
        callable_mp(this, &FennaraExecutor::_on_runtime_session_complete)
            .bind(batch_generation));
    return true;
}

void FennaraExecutor::_begin_runtime_session_phase(
    RuntimeSessionPhase phase,
    std::function<godot::Dictionary()> task,
    uint64_t batch_generation) {
    if (_runtime_session_thread.joinable()) {
        _runtime_session_thread.join();
    }

    _runtime_session_phase = phase;
    {
        std::lock_guard<std::mutex> lock(_runtime_session_mutex);
        _runtime_session_thread_done = false;
        _runtime_session_thread_result = godot::Dictionary();
    }
    _runtime_session_thread = std::thread([this, task = std::move(task)]() mutable {
        godot::Dictionary result = task();
        std::lock_guard<std::mutex> lock(_runtime_session_mutex);
        _runtime_session_thread_result = result;
        _runtime_session_thread_done = true;
    });

    if (!_schedule_runtime_session_poll(batch_generation)) {
        _on_runtime_session_complete(batch_generation);
    }
}

void FennaraExecutor::_complete_runtime_session(const godot::Dictionary &result,
                                                uint64_t batch_generation) {
    const int tool_index = _runtime_session_tool_index;
    const godot::Dictionary args = _runtime_session_args;

    _runtime_session_running = false;
    _runtime_session_tool_index = -1;
    _runtime_session_args = godot::Dictionary();
    _runtime_session_thread_done = false;
    _runtime_session_thread_result = godot::Dictionary();
    _runtime_session_phase = RuntimeSessionPhase::Idle;
    _runtime_session_build_result = godot::Dictionary();
    _runtime_session_preflight_result = godot::Dictionary();

    _on_async_tool_complete(
        result, tool_index, "runtime_session", args, batch_generation);
    _start_next_runtime_session();
}

void FennaraExecutor::_start_next_runtime_session() {
    if (_batch_cancelled || _runtime_session_running) {
        return;
    }
    if (_pending_runtime_sessions.empty()) {
        _start_next_runtime_script();
        return;
    }

    const PendingRuntimeSession pending = _pending_runtime_sessions.front();
    _pending_runtime_sessions.erase(_pending_runtime_sessions.begin());
    const uint64_t batch_generation = _async_batch_generation;

    _runtime_session_running = true;
    _runtime_session_tool_index = pending.tool_index;
    _runtime_session_args = pending.args;
    _runtime_session_cancelled.store(false);
    _runtime_session_build_result = godot::Dictionary();
    _runtime_session_preflight_result = godot::Dictionary();

    const godot::String action =
        godot::String(pending.args.get("action", "status")).strip_edges().to_lower();
    if (action == "start") {
        const godot::String scene_path =
            godot::String(pending.args.get("scene_path", "")).strip_edges();
        if (scene_path.is_empty()) {
            _complete_runtime_session(
                make_runtime_session_error("`scene_path` is required."),
                batch_generation);
            return;
        }

        _begin_runtime_session_phase(
            RuntimeSessionPhase::SlotStatus,
            []() { return FennaraRuntimeSessionTool::query_slot_status(); },
            batch_generation);
        return;
    }

    _begin_runtime_session_phase(
        RuntimeSessionPhase::Execute,
        [args = pending.args]() { return FennaraRuntimeSessionTool::execute(args); },
        batch_generation);
}

void FennaraExecutor::_on_runtime_session_complete(uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation) {
        return;
    }

    bool done = false;
    godot::Dictionary result;
    {
        std::lock_guard<std::mutex> lock(_runtime_session_mutex);
        done = _runtime_session_thread_done;
        result = _runtime_session_thread_result;
    }

    if (!done) {
        _schedule_runtime_session_poll(batch_generation);
        return;
    }

    if (_runtime_session_thread.joinable()) {
        _runtime_session_thread.join();
    }

    switch (_runtime_session_phase) {
        case RuntimeSessionPhase::SlotStatus:
            if ((bool)result.get("success", false) &&
                godot::String(result.get("availability", "")) == "free") {
                _begin_runtime_session_phase(
                    RuntimeSessionPhase::Build,
                    [this]() {
                        return csharp_build::run_dotnet_build_if_needed(
                            &_runtime_session_cancelled);
                    },
                    batch_generation);
                return;
            }
            _complete_runtime_session(result, batch_generation);
            return;

        case RuntimeSessionPhase::Build: {
            _runtime_session_build_result = result;
            if ((bool)result.get("needed", false) &&
                godot::String(result.get("status", "")) != "success") {
                godot::Dictionary blocked = make_runtime_session_error(
                    "C# project build failed. Runtime session was not started.");
                blocked["csharp_build"] = result;
                _complete_runtime_session(blocked, batch_generation);
                return;
            }

            const godot::String scene_path =
                godot::String(_runtime_session_args.get("scene_path", ""))
                    .strip_edges();
            godot::Dictionary validate_args;
            godot::Array scene_paths;
            scene_paths.append(scene_path);
            validate_args["scene_paths"] = scene_paths;
            validate_args["skip_runtime"] = true;
            if (_runtime_session_args.has("_fennara_tool_artifact_dir")) {
                validate_args["_fennara_tool_artifact_dir"] =
                    godot::String(
                        _runtime_session_args["_fennara_tool_artifact_dir"])
                        .path_join("preflight");
            }
            _runtime_session_preflight_result =
                FennaraValidateSceneTool::execute(validate_args);
            const godot::Dictionary summary =
                _runtime_session_preflight_result.get("summary", godot::Dictionary());
            if (!(bool)_runtime_session_preflight_result.get("success", false) ||
                static_cast<int>(summary.get("errors", 0)) > 0) {
                godot::Dictionary blocked = make_runtime_session_error(
                    "Scene preflight failed. Runtime session was not started.");
                blocked["csharp_build"] = _runtime_session_build_result;
                blocked["preflight"] = _runtime_session_preflight_result;
                _complete_runtime_session(blocked, batch_generation);
                return;
            }

            const godot::Dictionary script_context =
                runtime_scene_preflight::collect_scene_script_context(scene_path);
            _begin_runtime_session_phase(
                RuntimeSessionPhase::ScriptPreflight,
                [script_context]() {
                    return runtime_scene_preflight::diagnose_collected_scripts(
                        script_context);
                },
                batch_generation);
            return;
        }

        case RuntimeSessionPhase::ScriptPreflight:
            if (!(bool)result.get("success", false)) {
                godot::Dictionary blocked = make_runtime_session_error(
                    "Scene/autoload script diagnostics failed. Runtime session was not "
                    "started.");
                blocked["csharp_build"] = _runtime_session_build_result;
                blocked["preflight"] = _runtime_session_preflight_result;
                blocked["script_preflight"] = result;
                _complete_runtime_session(blocked, batch_generation);
                return;
            }

            _begin_runtime_session_phase(
                RuntimeSessionPhase::StartDaemon,
                [args = _runtime_session_args,
                 build_result = _runtime_session_build_result,
                 preflight = _runtime_session_preflight_result,
                 script_preflight = result]() {
                    return FennaraRuntimeSessionTool::execute_start_after_preflight(
                        args, build_result, preflight, script_preflight);
                },
                batch_generation);
            return;

        case RuntimeSessionPhase::StartDaemon:
        case RuntimeSessionPhase::Execute:
        case RuntimeSessionPhase::Idle:
            _complete_runtime_session(result, batch_generation);
            return;
    }
}

} // namespace fennara
