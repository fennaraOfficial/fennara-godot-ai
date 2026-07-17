#include "fennara/tools/run_asset_import_script/internal.hpp"

#include "fennara/editor_filesystem_state.hpp"
#include "fennara/warning_capture.hpp"

#include <godot_cpp/classes/button.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/editor_file_system.hpp>
#include <godot_cpp/classes/editor_file_system_directory.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/file_system_dock.hpp>
#include <godot_cpp/classes/os.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/classes/time.hpp>

namespace fennara::run_asset_import_script_internal {

namespace {

constexpr int kMaximumCollectedPaths = 200;
constexpr int kMaximumImportMessages = 50;

godot::String read_text_file(const godot::String &path) {
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(path, godot::FileAccess::READ);
    return file.is_valid() ? file->get_as_text() : godot::String();
}

godot::Array variant_paths(const godot::Variant &value, int maximum_paths = -1) {
    godot::Array result;
    if (value.get_type() == godot::Variant::ARRAY) {
        godot::Array values = value;
        const int count = maximum_paths < 0 || values.size() < maximum_paths
            ? values.size()
            : maximum_paths;
        for (int i = 0; i < count; i++) {
            result.append(values[i]);
        }
    } else if (value.get_type() == godot::Variant::PACKED_STRING_ARRAY) {
        godot::PackedStringArray values = value;
        const int count = maximum_paths < 0 || values.size() < maximum_paths
            ? values.size()
            : maximum_paths;
        for (int i = 0; i < count; i++) {
            result.append(values[i]);
        }
    }
    return result;
}

bool editor_filesystem_import_valid(const godot::String &asset_path) {
    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::EditorFileSystem *filesystem =
        editor != nullptr ? editor->get_resource_filesystem() : nullptr;
    if (filesystem == nullptr) {
        return false;
    }
    godot::EditorFileSystemDirectory *directory =
        filesystem->get_filesystem_path(asset_path.get_base_dir());
    if (directory == nullptr) {
        return false;
    }
    const int index = directory->find_file_index(asset_path.get_file());
    return index >= 0 && directory->get_file_import_is_valid(index);
}

bool contains_import_error(const godot::Array &captured) {
    for (int i = 0; i < captured.size(); i++) {
        if (captured[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary entry = captured[i];
        const godot::String type = entry.get("type", "");
        if (type == "error" || type == "script_error" || type == "shader_error") {
            return true;
        }
    }
    return false;
}

godot::Array bounded_messages(const godot::Array &captured) {
    godot::Array result;
    for (int i = 0; i < captured.size() && i < kMaximumImportMessages; i++) {
        result.append(captured[i]);
    }
    return result;
}

godot::Dictionary verify_import(const ImportSnapshot &snapshot,
                                const godot::Array &staged_changes) {
    godot::Dictionary verification;
    verification["success"] = false;
    godot::Ref<godot::ConfigFile> config;
    config.instantiate();
    const godot::Error load_error = config->load(snapshot.sidecar_path);
    verification["sidecar_load_error"] = static_cast<int64_t>(load_error);
    if (load_error != godot::OK) {
        verification["error"] = "Godot could not reload the canonical .import sidecar.";
        return verification;
    }

    const godot::String importer = config->get_value("remap", "importer", "");
    const bool importer_matches = importer == snapshot.importer;
    const bool sidecar_valid = config->get_value("remap", "valid", true);
    const bool filesystem_valid = editor_filesystem_import_valid(snapshot.asset_path);
    verification["importer"] = importer;
    verification["importer_matches"] = importer_matches;
    verification["sidecar_valid"] = sidecar_valid;
    verification["editor_filesystem_valid"] = filesystem_valid;

    godot::Array mismatched_options;
    for (int i = 0; i < staged_changes.size(); i++) {
        godot::Dictionary change = staged_changes[i];
        const godot::String name = change.get("name", "");
        const godot::Variant expected = change.get("after", godot::Variant());
        const godot::Variant actual =
            config->get_value("params", name, godot::Variant());
        if (actual != expected) {
            godot::Dictionary mismatch;
            mismatch["name"] = name;
            mismatch["expected"] = expected;
            mismatch["actual"] = actual;
            mismatched_options.append(mismatch);
        }
    }
    verification["mismatched_options"] = mismatched_options;

    godot::Array generated_files = variant_paths(
        config->get_value("deps", "dest_files", godot::Array()));
    godot::Array reported_generated_files;
    godot::Array missing_outputs;
    int missing_output_count = 0;
    for (int i = 0; i < generated_files.size(); i++) {
        const godot::String path = generated_files[i];
        if (reported_generated_files.size() < kMaximumCollectedPaths) {
            reported_generated_files.append(path);
        }
        if (!godot::FileAccess::file_exists(path)) {
            missing_output_count++;
            if (missing_outputs.size() < kMaximumCollectedPaths) {
                missing_outputs.append(path);
            }
        }
    }
    verification["generated_files"] = reported_generated_files;
    verification["generated_file_count"] = generated_files.size();
    verification["generated_files_omitted_count"] =
        generated_files.size() - reported_generated_files.size();
    verification["missing_outputs"] = missing_outputs;
    verification["missing_output_count"] = missing_output_count;
    verification["missing_outputs_omitted_count"] =
        missing_output_count - missing_outputs.size();

    godot::Ref<godot::Resource> loaded =
        godot::ResourceLoader::get_singleton()->load(
            snapshot.asset_path, "",
            godot::ResourceLoader::CACHE_MODE_REPLACE_DEEP);
    verification["resource_load_valid"] = loaded.is_valid();
    verification["resource_class"] =
        loaded.is_valid() ? loaded->get_class() : godot::String();

    const bool success = importer_matches && sidecar_valid && filesystem_valid &&
                         mismatched_options.is_empty() && missing_output_count == 0 &&
                         loaded.is_valid();
    verification["success"] = success;
    if (!success) {
        verification["error"] = "Post-reimport verification failed.";
    }
    return verification;
}

godot::Dictionary restore_previous_import(const ImportSnapshot &snapshot) {
    godot::Dictionary recovery;
    godot::String write_error;
    if (!write_text_file(snapshot.sidecar_path, snapshot.sidecar_text, write_error)) {
        recovery["success"] = false;
        recovery["error"] = write_error;
        return recovery;
    }
    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::EditorFileSystem *filesystem =
        editor != nullptr ? editor->get_resource_filesystem() : nullptr;
    if (filesystem == nullptr) {
        recovery["success"] = false;
        recovery["error"] = "Godot's EditorFileSystem is unavailable during recovery.";
        return recovery;
    }
    godot::PackedStringArray paths;
    paths.append(snapshot.asset_path);
    filesystem->reimport_files(paths);
    godot::Ref<godot::ConfigFile> restored_config;
    restored_config.instantiate();
    bool restored =
        restored_config->load(snapshot.sidecar_path) == godot::OK &&
        restored_config->get_value("remap", "importer", "") == snapshot.importer &&
        editor_filesystem_import_valid(snapshot.asset_path);
    godot::Array option_keys = snapshot.options.keys();
    for (int i = 0; restored && i < option_keys.size(); i++) {
        const godot::String name = option_keys[i];
        restored = restored_config->get_value(
                       "params", name, godot::Variant()) == snapshot.options[name];
    }
    restored = restored &&
        godot::ResourceLoader::get_singleton()->load(
            snapshot.asset_path, "",
            godot::ResourceLoader::CACHE_MODE_REPLACE_DEEP).is_valid();
    recovery["success"] = restored;
    if (!restored) {
        recovery["error"] = "The previous import configuration could not be verified after recovery.";
    }
    return recovery;
}

} // namespace

bool load_import_snapshot(const godot::String &asset_path,
                          ImportSnapshot &snapshot,
                          godot::Dictionary &result) {
    if (!asset_path.begins_with("res://") || asset_path.contains("..")) {
        result["success"] = false;
        result["error"] = "asset_path must be a normalized res:// source path.";
        return false;
    }
    if (!godot::FileAccess::file_exists(asset_path)) {
        result["success"] = false;
        result["error"] = "Asset source not found: " + asset_path;
        return false;
    }

    snapshot.asset_path = asset_path;
    snapshot.sidecar_path = asset_path + godot::String(".import");
    if (!godot::FileAccess::file_exists(snapshot.sidecar_path)) {
        result["success"] = false;
        result["error"] = "Godot import sidecar not found: " + snapshot.sidecar_path;
        return false;
    }
    snapshot.sidecar_text = read_text_file(snapshot.sidecar_path);
    if (snapshot.sidecar_text.is_empty()) {
        result["success"] = false;
        result["error"] = "Godot import sidecar was empty or unreadable.";
        return false;
    }
    snapshot.sidecar_hash = snapshot.sidecar_text.md5_text();
    snapshot.config.instantiate();
    const godot::Error load_error = snapshot.config->load(snapshot.sidecar_path);
    if (load_error != godot::OK) {
        result["success"] = false;
        result["error"] = "Godot failed to parse the import sidecar.";
        result["sidecar_load_error"] = static_cast<int64_t>(load_error);
        return false;
    }
    snapshot.importer = snapshot.config->get_value("remap", "importer", "");
    if (snapshot.importer.is_empty()) {
        result["success"] = false;
        result["error"] = "Import sidecar did not declare an importer.";
        return false;
    }

    godot::PackedStringArray option_keys =
        snapshot.config->get_section_keys("params");
    for (int i = 0; i < option_keys.size(); i++) {
        snapshot.options[option_keys[i]] =
            snapshot.config->get_value("params", option_keys[i]);
    }
    snapshot.generated_files = variant_paths(
        snapshot.config->get_value("deps", "dest_files", godot::Array()),
        kMaximumCollectedPaths);
    godot::PackedStringArray dependencies =
        godot::ResourceLoader::get_singleton()->get_dependencies(asset_path);
    for (int i = 0; i < dependencies.size() && i < kMaximumCollectedPaths; i++) {
        snapshot.dependencies.append(dependencies[i]);
    }
    snapshot.import_valid = editor_filesystem_import_valid(asset_path);
    return true;
}

bool sidecar_matches_snapshot(const ImportSnapshot &snapshot) {
    return read_text_file(snapshot.sidecar_path).md5_text() == snapshot.sidecar_hash;
}

bool write_text_file(const godot::String &path,
                     const godot::String &text,
                     godot::String &error) {
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(path, godot::FileAccess::WRITE);
    if (!file.is_valid()) {
        error = "Could not open file for writing: " + path;
        return false;
    }
    file->store_string(text);
    file->flush();
    if (file->get_error() != godot::OK) {
        error = "Failed while writing file: " + path;
        return false;
    }
    return true;
}

bool selected_import_dock_is_safe(const godot::String &asset_path,
                                  bool &target_selected,
                                  godot::String &error) {
    target_selected = false;
    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    if (editor == nullptr) {
        error = "Godot's EditorInterface is unavailable.";
        return false;
    }
    godot::PackedStringArray selected = editor->get_selected_paths();
    for (int i = 0; i < selected.size(); i++) {
        if (selected[i] == asset_path) {
            target_selected = true;
            break;
        }
    }
    if (!target_selected) {
        return true;
    }

    godot::Control *base = editor->get_base_control();
    if (base == nullptr) {
        error = "The selected asset's Import dock state could not be inspected.";
        return false;
    }
    godot::Array controls = base->find_children("*", "", true, false);
    godot::Node *import_dock = nullptr;
    for (int i = 0; i < controls.size(); i++) {
        godot::Object *object = godot::Object::cast_to<godot::Object>(controls[i]);
        if (object != nullptr && godot::String(object->get_class()) == "ImportDock") {
            import_dock = godot::Object::cast_to<godot::Node>(object);
            break;
        }
    }
    if (import_dock == nullptr) {
        error = "The selected asset's Import dock state could not be determined safely.";
        return false;
    }
    godot::Array descendants = import_dock->find_children("*", "Button", true, false);
    for (int i = 0; i < descendants.size(); i++) {
        godot::Button *button = godot::Object::cast_to<godot::Button>(descendants[i]);
        if (button != nullptr && button->get_text().strip_edges().ends_with("(*)")) {
            error = "The selected asset has pending Import dock changes. Apply or revert them before using edit mode.";
            return false;
        }
    }
    return true;
}

void refresh_selected_import_dock(const godot::String &asset_path,
                                  bool target_selected) {
    if (!target_selected) {
        return;
    }
    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::FileSystemDock *dock =
        editor != nullptr ? editor->get_file_system_dock() : nullptr;
    if (dock != nullptr) {
        dock->navigate_to_path(asset_path);
    }
}

godot::Dictionary apply_and_reimport(
    const ImportSnapshot &snapshot,
    const godot::Array &staged_changes) {
    godot::Dictionary result;
    result["success"] = false;
    result["reimported"] = false;
    result["recovery_attempted"] = false;

    if (!sidecar_matches_snapshot(snapshot)) {
        result["error"] =
            "The import sidecar changed before the staged transaction could be saved.";
        return result;
    }

    godot::String state_error;
    EditorFilesystemState &state = EditorFilesystemState::get_singleton();
    if (!state.begin_owned_import(snapshot.asset_path, state_error)) {
        result["error"] = state_error;
        return result;
    }
    const uint64_t started_ms = godot::Time::get_singleton()->get_ticks_msec();

    godot::Ref<godot::ConfigFile> edited;
    edited.instantiate();
    godot::Error load_error = edited->load(snapshot.sidecar_path);
    if (load_error != godot::OK) {
        state.finish_owned_import(false);
        result["error"] = "The import sidecar changed or became unreadable before saving.";
        return result;
    }
    for (int i = 0; i < staged_changes.size(); i++) {
        godot::Dictionary change = staged_changes[i];
        edited->set_value("params", change.get("name", ""),
                          change.get("after", godot::Variant()));
    }
    const godot::Error save_error = edited->save(snapshot.sidecar_path);
    result["sidecar_save_error"] = static_cast<int64_t>(save_error);
    if (save_error != godot::OK) {
        godot::String restore_error;
        const bool restored = write_text_file(
            snapshot.sidecar_path, snapshot.sidecar_text, restore_error);
        result["recovery_attempted"] = true;
        godot::Dictionary recovery;
        recovery["success"] = restored;
        if (!restored) {
            recovery["error"] = restore_error;
            result["recovery_error"] = restore_error;
        }
        result["recovery"] = recovery;
        state.finish_owned_import(false);
        result["error"] = "Godot failed to save the staged import settings.";
        if (!restored) {
            result["error"] = godot::String(result["error"]) +
                " Recovery also failed: " + restore_error;
        }
        return result;
    }

    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::EditorFileSystem *filesystem =
        editor != nullptr ? editor->get_resource_filesystem() : nullptr;
    if (filesystem == nullptr) {
        godot::String restore_error;
        const bool restored = write_text_file(
            snapshot.sidecar_path, snapshot.sidecar_text, restore_error);
        result["recovery_attempted"] = true;
        godot::Dictionary recovery;
        recovery["success"] = restored;
        if (!restored) {
            recovery["error"] = restore_error;
            result["recovery_error"] = restore_error;
        }
        result["recovery"] = recovery;
        state.finish_owned_import(false);
        result["error"] = "Godot's EditorFileSystem became unavailable before reimport.";
        if (!restored) {
            result["error"] = godot::String(result["error"]) +
                " Recovery also failed: " + restore_error;
        }
        return result;
    }

    godot::Ref<FennaraWarningCapture> capture;
    capture.instantiate();
    godot::OS::get_singleton()->add_logger(capture);
    godot::PackedStringArray paths;
    paths.append(snapshot.asset_path);
    filesystem->reimport_files(paths);
    godot::OS::get_singleton()->remove_logger(capture);
    result["reimported"] = true;
    result["import_messages"] = bounded_messages(capture->get_captured());
    result["import_message_count"] = capture->get_captured().size();

    godot::Dictionary verification = verify_import(snapshot, staged_changes);
    if (contains_import_error(capture->get_captured())) {
        verification["success"] = false;
        verification["error"] = "Godot reported an error during reimport.";
    }
    result["verification"] = verification;
    const bool success = verification.get("success", false);
    if (!success) {
        result["recovery_attempted"] = true;
        const godot::Dictionary recovery = restore_previous_import(snapshot);
        result["recovery"] = recovery;
        result["error"] = verification.get("error", "Asset reimport failed verification.");
        if (!(bool)recovery.get("success", false)) {
            const godot::String recovery_error = recovery.get(
                "error", "The previous import configuration could not be restored.");
            result["recovery_error"] = recovery_error;
            result["error"] = godot::String(result["error"]) +
                " Recovery also failed: " + recovery_error;
        }
    }

    const uint64_t finished_ms = godot::Time::get_singleton()->get_ticks_msec();
    result["duration_ms"] = static_cast<int64_t>(finished_ms - started_ms);
    result["success"] = success;
    state.finish_owned_import(success);
    return result;
}

} // namespace fennara::run_asset_import_script_internal
