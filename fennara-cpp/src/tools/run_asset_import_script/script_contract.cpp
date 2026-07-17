#include "fennara/tools/run_asset_import_script/internal.hpp"

#include "fennara/helpers.hpp"
#include "fennara/tools/write_or_update_file.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/classes/time.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara::run_asset_import_script_internal {

namespace {

godot::String safe_name_part(const godot::String &value) {
    godot::String safe = value.strip_edges().to_lower()
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
        .replace("@", "_")
        .replace(".", "_");
    return safe.is_empty() ? godot::String("asset") : safe;
}

godot::String temp_script_path(const godot::String &asset_path) {
    const uint64_t ticks = godot::Time::get_singleton()->get_ticks_usec();
    return "res://.fennara/tmp/editor_scripts/import_" +
           safe_name_part(asset_path.get_file().get_basename()) + "_" +
           godot::String::num_uint64(ticks) + ".gd";
}

} // namespace

godot::Dictionary make_runtime_error(const godot::String &message,
                                     const godot::String &source) {
    godot::Dictionary error;
    error["message"] = message;
    error["source"] = source;
    return error;
}

godot::String normalize_asset_path(const godot::String &path) {
    return normalize_path(path).strip_edges();
}

godot::String write_or_resolve_script_path(
    const godot::String &asset_path,
    const godot::String &code,
    const godot::String &script_path,
    godot::Dictionary &result) {
    if (!code.is_empty()) {
        const godot::String output_path = temp_script_path(asset_path);
        godot::Dictionary args;
        args["mode"] = "write";
        args["file_path"] = output_path;
        args["new_content"] = code;
        godot::Dictionary write_result =
            FennaraWriteOrUpdateFileTool::execute(args);
        if (!(bool)write_result.get("success", false)) {
            result = write_result;
            result["asset_path"] = asset_path;
            result["script_path"] = output_path;
            return godot::String();
        }
        return write_result.get("file_path", output_path);
    }

    const godot::String resolved = normalize_path(script_path);
    result["script_path"] = resolved;
    if (!resolved.ends_with(".gd")) {
        result["success"] = false;
        result["error"] = "script_path must point to a .gd file.";
        return godot::String();
    }
    if (!godot::FileAccess::file_exists(resolved)) {
        result["success"] = false;
        result["error"] = "Script file not found: " + resolved;
        return godot::String();
    }
    return resolved;
}

godot::Ref<godot::GDScript> load_script(const godot::String &script_path,
                                        godot::Dictionary &result) {
    godot::Ref<godot::GDScript> script =
        godot::ResourceLoader::get_singleton()->load(
            script_path, "GDScript", godot::ResourceLoader::CACHE_MODE_IGNORE);
    if (!script.is_valid()) {
        result["success"] = false;
        result["error"] = "Failed to load script: " + script_path;
        return script;
    }
    const godot::Error reload_error = script->reload();
    if (reload_error != godot::OK) {
        result["success"] = false;
        result["error"] = "Script reload failed before execution.";
        result["runtime_errors"] = godot::Array::make(
            make_runtime_error(
                "Script reload failed before execution. Patch the saved script_path and rerun.",
                "reload"));
        script.unref();
    }
    return script;
}

bool validate_script_contract(const godot::Ref<godot::GDScript> &script,
                              godot::Dictionary &result) {
    const godot::StringName base_type = script->get_instance_base_type();
    const godot::StringName ref_counted_type("RefCounted");
    if (base_type != ref_counted_type &&
        !godot::ClassDB::is_parent_class(base_type, ref_counted_type)) {
        result["success"] = false;
        result["error"] =
            "run_asset_import_script requires the script to extend RefCounted.";
        result["runtime_errors"] = godot::Array::make(
            make_runtime_error(
                "Expected `@tool extends RefCounted` for run_asset_import_script v1.",
                "contract"));
        return false;
    }
    return true;
}

godot::Variant instantiate_runner(const godot::Ref<godot::GDScript> &script,
                                  godot::Dictionary &result) {
    godot::Variant instance = script->new_();
    godot::Object *runner = instance;
    if (runner == nullptr) {
        result["success"] = false;
        result["error"] = "Failed to instantiate script.";
        result["runtime_errors"] = godot::Array::make(
            make_runtime_error("Script instantiation returned null.", "instantiate"));
        return godot::Variant();
    }
    if (!runner->has_method("run")) {
        result["success"] = false;
        result["error"] = "Script must define func run(ctx).";
        result["runtime_errors"] = godot::Array::make(
            make_runtime_error("Missing required run(ctx) entrypoint.", "contract"));
        return godot::Variant();
    }
    return instance;
}

} // namespace fennara::run_asset_import_script_internal
