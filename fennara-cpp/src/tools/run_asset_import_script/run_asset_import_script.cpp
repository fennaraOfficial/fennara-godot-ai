#include "fennara/tools/run_asset_import_script.hpp"

#include "fennara/logger.hpp"
#include "fennara/tools/run_asset_import_script/internal.hpp"
#include "fennara/tools/run_scene_edit_script/internal.hpp"
#include "fennara/warning_capture.hpp"

#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

using namespace run_asset_import_script_internal;

namespace {

void stamp_result(godot::Dictionary &result) {
    result["tool_name"] = "run_asset_import_script";
    result["format_version"] = "run-asset-import-script-result-v1";
}

void append_captured_errors(const godot::Array &captured,
                            godot::Array &runtime_errors) {
    for (int i = 0; i < captured.size(); i++) {
        if (captured[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary item = captured[i];
        const godot::String type = item.get("type", "");
        if (type == "error" || type == "script_error" || type == "shader_error") {
            godot::Dictionary entry;
            entry["source"] = "engine";
            entry["message"] = item.get("message", "Godot reported an error.");
            entry["file"] = item.get("file", "");
            entry["line"] = item.get("line", 0);
            runtime_errors.append(entry);
        }
    }
}

} // namespace

void FennaraRunAssetImportScriptTool::finalize_result(
    godot::Dictionary &result) {
    const bool success = result.get("success", false);
    result["status"] = success ? "success" : "failed";
    godot::Dictionary summary;
    summary["status"] = result["status"];
    summary["asset_path"] = result.get("asset_path", "");
    summary["script_path"] = result.get("script_path", "");
    summary["mode"] = result.get("mode", "inspect");
    summary["importer"] = result.get("importer", "");
    summary["modified"] = result.get("modified", false);
    summary["reimported"] = result.get("reimported", false);
    summary["diagnostic_success"] =
        result.get("diagnostic_success", false);
    summary["diagnostic_mode"] = result.get("diagnostic_mode", "");
    summary["diagnostic_fallback"] =
        result.get("diagnostic_fallback", "");
    summary["total_errors"] = result.get("total_errors", 0);
    summary["total_warnings"] = result.get("total_warnings", 0);
    summary["duration_ms"] = result.get("duration_ms", 0);
    summary["change_count"] =
        godot::Array(result.get("changes", godot::Array())).size();
    summary["runtime_error_count"] =
        godot::Array(result.get("runtime_errors", godot::Array())).size();
    summary["log_count"] =
        godot::Array(result.get("logs", godot::Array())).size();
    result["summary"] = summary;
}

void FennaraRunAssetImportScriptTool::_bind_methods() {
    godot::ClassDB::bind_static_method(
        "FennaraRunAssetImportScriptTool",
        godot::D_METHOD("execute", "args"),
        &FennaraRunAssetImportScriptTool::execute);
}

godot::Dictionary FennaraRunAssetImportScriptTool::execute(
    const godot::Dictionary &args) {
    godot::Dictionary prepared = prepare_execution(args);
    if (!(bool)prepared.get("success", false)) {
        finalize_result(prepared);
        return prepared;
    }

    const godot::String script_path = prepared.get("script_path", "");
    godot::Dictionary diagnostics =
        run_scene_edit_script_internal::collect_script_diagnostics(script_path);
    run_scene_edit_script_internal::apply_diagnostics_to_result(
        diagnostics, prepared);

    if (!(bool)diagnostics.get("diagnostic_success", false)) {
        prepared["diagnostic_mode"] = "direct_script_load";
        prepared["diagnostic_fallback"] = "direct_script_load";
    } else {
        prepared["diagnostic_mode"] = "lsp";
    }

    if ((bool)diagnostics.get("diagnostic_success", false) &&
        (int)diagnostics.get("total_errors", 0) > 0) {
        prepared["success"] = false;
        prepared["error"] =
            "Script diagnostics reported errors. Patch the saved script_path and rerun.";
        finalize_result(prepared);
        return prepared;
    }

    return execute_prepared(prepared);
}

godot::Dictionary FennaraRunAssetImportScriptTool::prepare_execution(
    const godot::Dictionary &args) {
    godot::Dictionary result;
    stamp_result(result);
    result["success"] = false;
    result["modified"] = false;
    result["reimported"] = false;
    result["changes"] = godot::Array();
    result["logs"] = godot::Array();
    result["runtime_errors"] = godot::Array();

    const godot::String raw_asset_path = args.get("asset_path", "");
    const godot::String mode =
        godot::String(args.get("mode", "inspect")).strip_edges().to_lower();
    const godot::String code = args.get("code", "");
    const godot::String provided_script_path = args.get("script_path", "");
    if (raw_asset_path.is_empty()) {
        result["error"] = "asset_path required";
        finalize_result(result);
        return result;
    }
    if (code.is_empty() == provided_script_path.is_empty()) {
        result["error"] = "Provide exactly one of code or script_path.";
        finalize_result(result);
        return result;
    }
    if (mode != "inspect" && mode != "edit") {
        result["error"] = "mode must be either 'inspect' or 'edit'.";
        finalize_result(result);
        return result;
    }

    const godot::String asset_path = normalize_asset_path(raw_asset_path);
    result["asset_path"] = asset_path;
    result["mode"] = mode;
    ImportSnapshot snapshot;
    if (!load_import_snapshot(asset_path, snapshot, result)) {
        finalize_result(result);
        return result;
    }
    result["importer"] = snapshot.importer;
    result["generated_files"] = snapshot.generated_files;
    result["dependencies"] = snapshot.dependencies;

    const godot::String script_path = write_or_resolve_script_path(
        asset_path, code, provided_script_path, result);
    if (script_path.is_empty()) {
        finalize_result(result);
        return result;
    }
    result["script_path"] = script_path;

    result["success"] = true;
    return result;
}

godot::Dictionary FennaraRunAssetImportScriptTool::execute_prepared(
    const godot::Dictionary &prepared_args) {
    godot::Dictionary result = prepared_args;
    stamp_result(result);
    result["success"] = false;
    const godot::String asset_path = result.get("asset_path", "");
    const godot::String mode = result.get("mode", "inspect");
    const godot::String script_path = result.get("script_path", "");

    ImportSnapshot snapshot;
    if (!load_import_snapshot(asset_path, snapshot, result)) {
        finalize_result(result);
        return result;
    }
    result["importer"] = snapshot.importer;
    result["generated_files"] = snapshot.generated_files;
    result["dependencies"] = snapshot.dependencies;

    godot::Ref<godot::GDScript> script = load_script(script_path, result);
    if (!script.is_valid() || !validate_script_contract(script, result)) {
        finalize_result(result);
        return result;
    }
    godot::Variant runner_variant = instantiate_runner(script, result);
    godot::Object *runner = runner_variant;
    if (runner == nullptr) {
        finalize_result(result);
        return result;
    }

    godot::Ref<godot::Resource> imported_resource =
        godot::ResourceLoader::get_singleton()->load(
            asset_path, "", godot::ResourceLoader::CACHE_MODE_IGNORE_DEEP);
    godot::Ref<FennaraRunAssetImportScriptContext> context;
    context.instantiate();
    context->configure(asset_path, snapshot.importer, snapshot.options,
                       snapshot.generated_files, snapshot.dependencies,
                       imported_resource, snapshot.import_valid,
                       mode == "inspect");
    result["import_info"] = context->get_import_info();

    godot::Ref<FennaraWarningCapture> capture;
    capture.instantiate();
    godot::OS::get_singleton()->add_logger(capture);
    runner->call("run", context.ptr());
    godot::OS::get_singleton()->remove_logger(capture);

    godot::Array runtime_errors = context->get_edit_errors();
    append_captured_errors(capture->get_captured(), runtime_errors);
    result["logs"] = context->get_logs();
    result["runtime_errors"] = runtime_errors;
    result["changes"] = context->get_staged_changes();
    result["modified"] = false;
    context->cleanup();

    if (!sidecar_matches_snapshot(snapshot)) {
        godot::String restore_error;
        const bool restored = write_text_file(
            snapshot.sidecar_path, snapshot.sidecar_text, restore_error);
        result["success"] = false;
        result["sidecar_restored"] = restored;
        result["recovery_attempted"] = true;
        godot::Dictionary recovery;
        recovery["success"] = restored;
        result["error"] =
            "The worker changed the .import sidecar directly. Only ctx.set_import_option() is supported.";
        if (!restored) {
            result["recovery_error"] = restore_error;
            recovery["error"] = restore_error;
            result["error"] = godot::String(result["error"]) +
                " Recovery also failed: " + restore_error;
        }
        result["recovery"] = recovery;
        finalize_result(result);
        return result;
    }
    if (!runtime_errors.is_empty()) {
        result["error"] = "Asset import script execution failed.";
        finalize_result(result);
        return result;
    }
    if (mode == "inspect") {
        result["success"] = true;
        result["modified"] = false;
        result["changes"] = godot::Array();
        result["note"] = "Imported asset inspected without changing import settings.";
        finalize_result(result);
        return result;
    }

    const godot::Array changes = result["changes"];
    if (changes.is_empty()) {
        result["success"] = true;
        result["note"] = "No import option changes were staged, so the asset was not reimported.";
        finalize_result(result);
        return result;
    }

    bool target_selected = false;
    godot::String dock_error;
    if (!selected_import_dock_is_safe(asset_path, target_selected, dock_error)) {
        result["error"] = dock_error;
        finalize_result(result);
        return result;
    }

    godot::Dictionary import_result = apply_and_reimport(snapshot, changes);
    godot::Array keys = import_result.keys();
    for (int i = 0; i < keys.size(); i++) {
        result[keys[i]] = import_result[keys[i]];
    }
    if ((bool)result.get("success", false)) {
        result["modified"] = true;
        refresh_selected_import_dock(asset_path, target_selected);
        result["import_dock_refreshed"] = target_selected;
        FLOG_TOOL("run_asset_import_script: asset=" + asset_path +
                  " importer=" + snapshot.importer +
                  " changes=" + godot::String::num_int64(changes.size()));
    }
    finalize_result(result);
    return result;
}

} // namespace fennara
