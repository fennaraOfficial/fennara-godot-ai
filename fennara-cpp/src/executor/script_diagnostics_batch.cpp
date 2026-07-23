#include "fennara/executor.hpp"
#include "fennara/file_utils.hpp"
#include "fennara/lsp/gdscript_lsp.hpp"

#include "fennara/tools/run_asset_import_script.hpp"
#include "fennara/tools/run_scene_edit_script.hpp"
#include "fennara/tools/validate_scene.hpp"

#include <godot_cpp/classes/file_access.hpp>

namespace fennara {
namespace {

void merge_per_file(godot::Dictionary &into, const godot::Dictionary &from) {
    godot::Array keys = from.keys();
    for (int i = 0; i < keys.size(); i++) {
        into[keys[i]] = from[keys[i]];
    }
}

godot::Dictionary run_gdscript_diagnostics(const godot::Array &files_to_check) {
    godot::Array gd_files;
    for (int i = 0; i < files_to_check.size(); i++) {
        godot::String path = files_to_check[i];
        if (path.ends_with(".gd")) {
            gd_files.append(path);
        }
    }

    godot::Dictionary per_file;
    if (!gd_files.is_empty()) {
        godot::Dictionary gdscript_result =
            gdscript_lsp::diagnose_files(gd_files, "fennara-batch-diagnostics");
        if (!(bool)gdscript_result.get("success", false)) {
            return gdscript_result;
        }
        merge_per_file(
            per_file,
            gdscript_result.get("per_file", godot::Dictionary()));
    }

    godot::Dictionary result;
    result["success"] = true;
    result["per_file"] = per_file;
    return result;
}

void apply_batch_script_diagnostics(
    godot::Dictionary &result,
    const godot::Dictionary &per_file,
    const godot::String &resolved_script_path,
    bool batch_success) {
    if (per_file.has(resolved_script_path)) {
        godot::Dictionary file_diag = per_file[resolved_script_path];
        result["script_diagnostics"] =
            file_diag.get("diagnostics", godot::Array());
        result["total_errors"] = file_diag.get("total_errors", 0);
        result["total_warnings"] = file_diag.get("total_warnings", 0);
    } else {
        result["script_diagnostics"] = godot::Array();
        result["total_errors"] = 0;
        result["total_warnings"] = 0;
    }
    result["diagnostic_success"] = batch_success;
}

} // namespace

void FennaraExecutor::_maybe_append_scene_validation(godot::Dictionary &res,
                                              const godot::String &scene_path) {
    if (res.has("validation")) {
        return;
    }

    godot::Dictionary val_args;
    godot::Array scene_paths;
    scene_paths.append(scene_path);
    val_args["scene_paths"] = scene_paths;
    godot::Dictionary validation = FennaraValidateSceneTool::execute(val_args);
    godot::Array validation_results =
        validation.get("scenes", godot::Array());
    if (!validation_results.is_empty()) {
        godot::Dictionary first = validation_results[0];
        if (godot::String(first.get("status", "")) != "success") return;
        godot::Dictionary val_summary;
        val_summary["issues"] = first.get("issues", godot::Array());
        val_summary["checks_run"] = first.get("checks_run", 0);
        val_summary["total_issues"] = first.get("total_issues", 0);
        val_summary["errors"] = first.get("errors", 0);
        val_summary["warnings"] = first.get("warnings", 0);
        res["validation"] = val_summary;
    }
}

void FennaraExecutor::_track_modified_scene(const godot::String &scene_path,
                                     int tool_index) {
    _modified_scenes.push_back({scene_path, tool_index});
}

void FennaraExecutor::_finish_run_scene_edit_script(
    godot::Dictionary &result,
    const godot::Dictionary &prepared_args,
    int tool_index,
    uint64_t batch_generation) {
    if ((bool)result.get("success", false) &&
        (bool)result.get("scene_saved", false)) {
        godot::String scene_path =
            result.get("scene_path", prepared_args.get("scene_path", ""));
        if (!scene_path.is_empty()) {
            _track_modified_scene(scene_path, tool_index);
            _maybe_append_scene_validation(result, scene_path);
        }
    }
    _on_async_tool_complete(
        result, tool_index, "run_scene_edit_script", godot::Dictionary(),
        batch_generation);
}

void FennaraExecutor::_run_batch_diagnostics(
    uint64_t batch_generation, const godot::Array &diagnostic_targets) {
    godot::Dictionary per_file_results;
    bool batch_success = false;
    godot::String batch_error;

    godot::Array files_to_check;
    for (int i = 0; i < diagnostic_targets.size(); i++) {
        godot::Dictionary target = diagnostic_targets[i];
        godot::String diagnostic_path = target.get("diagnostic_path", "");
        if (!diagnostic_path.is_empty() &&
            godot::FileAccess::file_exists(diagnostic_path) &&
            !files_to_check.has(diagnostic_path)) {
            files_to_check.append(diagnostic_path);
        }
    }

    if (files_to_check.is_empty()) {
        for (int i = 0; i < diagnostic_targets.size(); i++) {
            godot::Dictionary target = diagnostic_targets[i];
            godot::String result_path = target.get("result_path", "");
            if (result_path.is_empty()) continue;
            godot::Dictionary file_result;
            file_result["diagnostics"] = godot::Array();
            file_result["total_errors"] = 0;
            file_result["total_warnings"] = 0;
            file_result["total_info"] = 0;
            file_result["total_hints"] = 0;
            file_result["total_diagnostics"] = 0;
            per_file_results[result_path] = file_result;
        }
        batch_success = true;
        goto done;
    }

    {
        godot::Dictionary diag_result =
            run_gdscript_diagnostics(files_to_check);
        if (!(bool)diag_result.get("success", false)) {
            batch_success = false;
            batch_error = diag_result.get("error", "Diagnostics failed");
            goto done;
        }

        godot::Dictionary abs_per_file =
            diag_result.get("per_file", godot::Dictionary());

        for (int i = 0; i < diagnostic_targets.size(); i++) {
            godot::Dictionary target = diagnostic_targets[i];
            godot::String diagnostic_path = target.get("diagnostic_path", "");
            godot::String result_path = target.get("result_path", "");
            if (result_path.is_empty()) continue;
            godot::Dictionary file_result;
            if (abs_per_file.has(diagnostic_path)) {
                file_result = abs_per_file[diagnostic_path];
            } else {
                file_result["diagnostics"] = godot::Array();
                file_result["total_errors"] = 0;
                file_result["total_warnings"] = 0;
                file_result["total_info"] = 0;
                file_result["total_hints"] = 0;
                file_result["total_diagnostics"] = 0;
            }
            per_file_results[result_path] = file_result;
        }

        batch_success = true;
    }

done:
    {
        std::lock_guard<std::mutex> lock(_batch_diag_mutex);
        _batch_diag_results = godot::Dictionary();
        _batch_diag_results["success"] = batch_success;
        _batch_diag_results["per_file"] = per_file_results;
        if (!batch_error.is_empty()) {
            _batch_diag_results["error"] = batch_error;
        }
    }
    godot::Dictionary diag_done = _batch_log_context();
    diag_done["success"] = batch_success;
    diag_done["target_count"] = diagnostic_targets.size();
    if (!batch_error.is_empty()) {
        diag_done["error"] = batch_error;
    }
    _log_tool_event("Batch diagnostics finished", diag_done);
    call_deferred("_on_batch_diagnostics_complete", batch_generation);
}

void FennaraExecutor::_on_batch_diagnostics_complete(uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation) {
        return;
    }

    if (_batch_diag_thread.joinable()) {
        _batch_diag_thread.join();
    }

    godot::Dictionary batch_results;
    {
        std::lock_guard<std::mutex> lock(_batch_diag_mutex);
        batch_results = _batch_diag_results;
    }

    bool batch_success = batch_results.get("success", false);
    godot::Dictionary per_file =
        batch_results.get("per_file", godot::Dictionary());
    godot::String batch_error =
        batch_results.get("error", "Diagnostics failed");
    _focus_best_edited_script(per_file);

    for (const auto &pw : _pending_script_writes) {
        godot::Dictionary merged = pw.write_result;

        if (per_file.has(pw.file_path)) {
            godot::Dictionary file_diag = per_file[pw.file_path];
            merged["diagnostics"] = file_diag.get("diagnostics", godot::Array());
            merged["total_errors"] = file_diag.get("total_errors", 0);
            merged["total_warnings"] = file_diag.get("total_warnings", 0);
            merged["total_info"] = file_diag.get("total_info", 0);
            merged["total_hints"] = file_diag.get("total_hints", 0);
            merged["total_diagnostics"] = file_diag.get(
                "total_diagnostics",
                godot::Array(file_diag.get("diagnostics", godot::Array())).size());
        } else {
            merged["diagnostics"] = godot::Array();
            merged["total_errors"] = 0;
            merged["total_warnings"] = 0;
            merged["total_info"] = 0;
            merged["total_hints"] = 0;
            merged["total_diagnostics"] = 0;
        }
        merged["diagnostic_success"] = batch_success;
        merged["diagnostic_mode"] = "lsp";
        godot::Dictionary summary = merged.get("summary", godot::Dictionary());
        summary["diagnostic_success"] = batch_success;
        summary["diagnostic_mode"] = "lsp";
        summary["total_errors"] = merged.get("total_errors", 0);
        summary["total_warnings"] = merged.get("total_warnings", 0);
        summary["total_info"] = merged.get("total_info", 0);
        summary["total_hints"] = merged.get("total_hints", 0);
        summary["diagnostic_count"] = merged.get("total_diagnostics", 0);
        merged["summary"] = summary;

        _on_async_tool_complete(merged, pw.tool_index, "write_or_update_file", pw.tool_args, batch_generation);
    }

    _pending_script_writes.clear();

    for (const auto &pending : _pending_run_scene_edit_scripts) {
        godot::Dictionary merged = pending.prepared_args;
        apply_batch_script_diagnostics(
            merged, per_file, pending.resolved_script_path, batch_success);

        if (!batch_success) {
            merged["diagnostic_error"] = batch_error;
            merged["diagnostic_mode"] = "direct_script_load";
            merged["diagnostic_fallback"] = "direct_script_load";
            godot::Dictionary executed = FennaraRunSceneEditScriptTool::execute_prepared(merged);
            _finish_run_scene_edit_script(
                executed, merged, pending.tool_index, batch_generation);
            continue;
        }

        merged["diagnostic_mode"] = "lsp";

        if ((int)merged.get("total_errors", 0) > 0) {
            merged["success"] = false;
            merged["error"] = "Script diagnostics reported errors. Patch the saved script_path and rerun.";
            merged["runtime_errors"] = godot::Array();
            merged["logs"] = godot::Array();
            _on_async_tool_complete(merged, pending.tool_index, "run_scene_edit_script", godot::Dictionary(), batch_generation);
            continue;
        }

        godot::Dictionary executed = FennaraRunSceneEditScriptTool::execute_prepared(merged);
        _finish_run_scene_edit_script(
            executed, merged, pending.tool_index, batch_generation);
    }

    _pending_run_scene_edit_scripts.clear();

    std::vector<PendingScreenshotScene> runnable_screenshots;
    runnable_screenshots.reserve(_pending_screenshot_scenes.size());
    for (auto pending : _pending_screenshot_scenes) {
        godot::String script_path = pending.args.get(
            "_fennara_screenshot_script_path", "");
        if (script_path.is_empty()) {
            runnable_screenshots.push_back(pending);
            continue;
        }

        godot::String resolved = file_utils::resolve_path(script_path);
        apply_batch_script_diagnostics(
            pending.args, per_file, resolved, batch_success);
        if (!batch_success) {
            pending.args["diagnostic_error"] = batch_error;
            runnable_screenshots.push_back(pending);
            continue;
        }
        if ((int)pending.args.get("total_errors", 0) > 0) {
            godot::Dictionary failed = pending.args;
            failed["success"] = false;
            failed["error"] =
                "Screenshot script diagnostics reported errors. Patch script_path and rerun.";
            _on_async_tool_complete(
                failed, pending.tool_index, "screenshot_scene",
                godot::Dictionary(), batch_generation);
            continue;
        }
        runnable_screenshots.push_back(pending);
    }
    _pending_screenshot_scenes.swap(runnable_screenshots);
    if (!_pending_run_asset_import_scripts.empty()) {
        _asset_import_execution_pending = true;
        _asset_import_batch_generation = batch_generation;
        set_process(true);
        return;
    }
    _start_next_validate_scene();
}

void FennaraExecutor::_execute_pending_asset_import_scripts(
    uint64_t batch_generation) {
    if (_batch_cancelled || batch_generation != _async_batch_generation) {
        _pending_run_asset_import_scripts.clear();
        return;
    }

    godot::Dictionary batch_results;
    {
        std::lock_guard<std::mutex> lock(_batch_diag_mutex);
        batch_results = _batch_diag_results;
    }
    const bool batch_success = batch_results.get("success", false);
    const godot::Dictionary per_file =
        batch_results.get("per_file", godot::Dictionary());
    const godot::String batch_error =
        batch_results.get("error", "Diagnostics failed");

    for (const auto &pending : _pending_run_asset_import_scripts) {
        godot::Dictionary merged = pending.prepared_args;
        apply_batch_script_diagnostics(
            merged, per_file, pending.resolved_script_path, batch_success);

        if (!batch_success) {
            merged["diagnostic_error"] = batch_error;
            merged["diagnostic_mode"] = "direct_script_load";
            merged["diagnostic_fallback"] = "direct_script_load";
            godot::Dictionary executed =
                FennaraRunAssetImportScriptTool::execute_prepared(merged);
            _on_async_tool_complete(
                executed, pending.tool_index, "run_asset_import_script",
                godot::Dictionary(), batch_generation);
            continue;
        }

        merged["diagnostic_mode"] = "lsp";
        if ((int)merged.get("total_errors", 0) > 0) {
            merged["success"] = false;
            merged["error"] =
                "Script diagnostics reported errors. Patch the saved script_path and rerun.";
            merged["runtime_errors"] = godot::Array();
            merged["logs"] = godot::Array();
            FennaraRunAssetImportScriptTool::finalize_result(merged);
            _on_async_tool_complete(
                merged, pending.tool_index, "run_asset_import_script",
                godot::Dictionary(), batch_generation);
            continue;
        }

        godot::Dictionary executed =
            FennaraRunAssetImportScriptTool::execute_prepared(merged);
        _on_async_tool_complete(
            executed, pending.tool_index, "run_asset_import_script",
            godot::Dictionary(), batch_generation);
    }
    _pending_run_asset_import_scripts.clear();
    _start_next_validate_scene();
}

} // namespace fennara
