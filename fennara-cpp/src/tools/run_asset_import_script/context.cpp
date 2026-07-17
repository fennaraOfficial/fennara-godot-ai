#include "fennara/tools/run_asset_import_script.hpp"

#include "fennara/tools/run_asset_import_script/internal.hpp"

#include <godot_cpp/classes/animation_player.hpp>
#include <godot_cpp/classes/control.hpp>
#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/classes/mesh_instance3d.hpp>
#include <godot_cpp/classes/packed_scene.hpp>
#include <godot_cpp/classes/skeleton3d.hpp>
#include <godot_cpp/core/class_db.hpp>

namespace fennara {

namespace {

constexpr int kMaximumStagedChanges = 64;
constexpr int kMaximumListedOptions = 200;
constexpr int kMaximumLogEntries = 100;
constexpr int kMaximumLogCharacters = 16000;

void count_scene_nodes(godot::Node *node, godot::Dictionary &summary) {
    if (node == nullptr) {
        return;
    }
    summary["nodes"] = static_cast<int64_t>(summary.get("nodes", 0)) + 1;
    if (godot::Object::cast_to<godot::MeshInstance3D>(node) != nullptr) {
        summary["mesh_instances"] =
            static_cast<int64_t>(summary.get("mesh_instances", 0)) + 1;
    }
    if (godot::Object::cast_to<godot::Skeleton3D>(node) != nullptr) {
        summary["skeletons"] =
            static_cast<int64_t>(summary.get("skeletons", 0)) + 1;
    }
    if (auto *player = godot::Object::cast_to<godot::AnimationPlayer>(node)) {
        summary["animation_players"] =
            static_cast<int64_t>(summary.get("animation_players", 0)) + 1;
        summary["animations"] =
            static_cast<int64_t>(summary.get("animations", 0)) +
            player->get_animation_list().size();
    }
    for (int i = 0; i < node->get_child_count(); i++) {
        count_scene_nodes(node->get_child(i), summary);
    }
}

bool has_effective_value(const godot::Variant &value) {
    if (value.get_type() == godot::Variant::NIL) {
        return false;
    }
    if (value.get_type() == godot::Variant::STRING) {
        return !godot::String(value).strip_edges().is_empty();
    }
    return true;
}

} // namespace

void FennaraRunAssetImportScriptContext::_bind_methods() {
    godot::ClassDB::bind_method(godot::D_METHOD("get_asset_path"),
        &FennaraRunAssetImportScriptContext::get_asset_path);
    godot::ClassDB::bind_method(godot::D_METHOD("get_mode"),
        &FennaraRunAssetImportScriptContext::get_mode);
    godot::ClassDB::bind_method(godot::D_METHOD("is_read_only"),
        &FennaraRunAssetImportScriptContext::is_read_only);
    godot::ClassDB::bind_method(godot::D_METHOD("get_import_info"),
        &FennaraRunAssetImportScriptContext::get_import_info);
    godot::ClassDB::bind_method(godot::D_METHOD("has_import_option", "name"),
        &FennaraRunAssetImportScriptContext::has_import_option);
    godot::ClassDB::bind_method(godot::D_METHOD("get_import_option", "name"),
        &FennaraRunAssetImportScriptContext::get_import_option);
    godot::ClassDB::bind_method(
        godot::D_METHOD("list_import_options", "prefix"),
        &FennaraRunAssetImportScriptContext::list_import_options,
        DEFVAL(godot::String()));
    godot::ClassDB::bind_method(
        godot::D_METHOD("set_import_option", "name", "value"),
        &FennaraRunAssetImportScriptContext::set_import_option);
    godot::ClassDB::bind_method(godot::D_METHOD("get_staged_changes"),
        &FennaraRunAssetImportScriptContext::get_staged_changes);
    godot::ClassDB::bind_method(
        godot::D_METHOD("discard_import_option_change", "name"),
        &FennaraRunAssetImportScriptContext::discard_import_option_change);
    godot::ClassDB::bind_method(godot::D_METHOD("get_imported_resource"),
        &FennaraRunAssetImportScriptContext::get_imported_resource);
    godot::ClassDB::bind_method(godot::D_METHOD("instantiate_imported_scene"),
        &FennaraRunAssetImportScriptContext::instantiate_imported_scene);
    godot::ClassDB::bind_method(godot::D_METHOD("get_generated_files"),
        &FennaraRunAssetImportScriptContext::get_generated_files);
    godot::ClassDB::bind_method(godot::D_METHOD("get_dependencies"),
        &FennaraRunAssetImportScriptContext::get_dependencies);
    godot::ClassDB::bind_method(godot::D_METHOD("get_subresource_summary"),
        &FennaraRunAssetImportScriptContext::get_subresource_summary);
    godot::ClassDB::bind_method(godot::D_METHOD("log", "value"),
        &FennaraRunAssetImportScriptContext::log);
    godot::ClassDB::bind_method(godot::D_METHOD("error", "message"),
        &FennaraRunAssetImportScriptContext::error);
    godot::ClassDB::bind_method(
        godot::D_METHOD("require", "condition", "message"),
        &FennaraRunAssetImportScriptContext::require);
    godot::ClassDB::bind_method(godot::D_METHOD("get_logs"),
        &FennaraRunAssetImportScriptContext::get_logs);
    godot::ClassDB::bind_method(godot::D_METHOD("get_edit_errors"),
        &FennaraRunAssetImportScriptContext::get_edit_errors);
}

void FennaraRunAssetImportScriptContext::configure(
    const godot::String &asset_path,
    const godot::String &importer,
    const godot::Dictionary &options,
    const godot::Array &generated_files,
    const godot::Array &dependencies,
    const godot::Ref<godot::Resource> &imported_resource,
    bool import_valid,
    bool read_only) {
    cleanup();
    _asset_path = asset_path;
    _importer = importer;
    _options = options;
    _generated_files = generated_files;
    _dependencies = dependencies;
    _imported_resource = imported_resource;
    _import_valid = import_valid;
    _read_only = read_only;
    _staged_values.clear();
    _logs.clear();
    _errors.clear();
}

void FennaraRunAssetImportScriptContext::cleanup() {
    if (_temporary_host != nullptr) {
        _temporary_host->queue_free();
    }
    _temporary_host = nullptr;
}

godot::String FennaraRunAssetImportScriptContext::get_asset_path() const {
    return _asset_path;
}

godot::String FennaraRunAssetImportScriptContext::get_mode() const {
    return _read_only ? godot::String("inspect") : godot::String("edit");
}

bool FennaraRunAssetImportScriptContext::is_read_only() const {
    return _read_only;
}

godot::Dictionary FennaraRunAssetImportScriptContext::get_import_info() const {
    godot::Dictionary info;
    info["asset_path"] = _asset_path;
    info["importer"] = _importer;
    info["option_count"] = _options.size();
    info["generated_file_count"] = _generated_files.size();
    info["dependency_count"] = _dependencies.size();
    info["resource_loaded"] = _imported_resource.is_valid();
    info["import_valid"] = _import_valid;
    info["resource_class"] = _imported_resource.is_valid()
        ? _imported_resource->get_class()
        : godot::String();
    info["mode"] = get_mode();
    const godot::Dictionary version =
        godot::Engine::get_singleton()->get_version_info();
    info["godot_version"] = version.get("string", "");
    return info;
}

bool FennaraRunAssetImportScriptContext::has_import_option(
    const godot::String &name) const {
    return _options.has(name);
}

godot::Variant FennaraRunAssetImportScriptContext::get_import_option(
    const godot::String &name) const {
    if (_staged_values.has(name)) {
        return _staged_values[name];
    }
    if (_options.has(name)) {
        return _options[name];
    }
    return godot::Variant();
}

godot::Array FennaraRunAssetImportScriptContext::list_import_options(
    const godot::String &prefix) const {
    godot::Array listed;
    godot::Array keys = _options.keys();
    int matched_count = 0;
    for (int i = 0; i < keys.size(); i++) {
        const godot::String name = keys[i];
        if (!prefix.is_empty() && !name.begins_with(prefix)) {
            continue;
        }
        matched_count++;
        if (listed.size() >= kMaximumListedOptions) {
            continue;
        }
        godot::String reason;
        const bool editable = _option_is_editable(name, &reason);
        godot::Variant value = get_import_option(name);
        if (name == "_subresources" &&
            value.get_type() == godot::Variant::DICTIONARY) {
            godot::Dictionary bounded_value;
            bounded_value["entry_count"] = godot::Dictionary(value).size();
            bounded_value["omitted"] = true;
            value = bounded_value;
        }
        godot::Dictionary entry;
        entry["name"] = name;
        entry["value"] = value;
        entry["type"] = godot::Variant::get_type_name(value.get_type());
        entry["editable"] = editable;
        entry["effect"] = editable ? godot::String("generated_cache") : reason;
        entry["staged"] = _staged_values.has(name);
        listed.append(entry);
    }
    if (matched_count > listed.size()) {
        godot::Dictionary marker;
        marker["truncated"] = true;
        marker["omitted_count"] = matched_count - listed.size();
        listed.append(marker);
    }
    return listed;
}

void FennaraRunAssetImportScriptContext::set_import_option(
    const godot::String &name,
    const godot::Variant &value) {
    if (_read_only) {
        _add_error("set_import_option() is unavailable in inspect mode.");
        return;
    }
    if (!_options.has(name)) {
        _add_error("Unknown import option: " + name);
        return;
    }
    godot::String reason;
    if (!_option_is_editable(name, &reason)) {
        _add_error("Import option is inspect-only in this version: " + name +
                   " (" + reason + ")");
        return;
    }
    const godot::Variant before = _options[name];
    if (before.get_type() != value.get_type()) {
        _add_error(
            "Import option type mismatch for " + name + ": expected " +
            godot::Variant::get_type_name(before.get_type()) + ", received " +
            godot::Variant::get_type_name(value.get_type()) + ".");
        return;
    }
    if (!_staged_values.has(name) && _staged_values.size() >= kMaximumStagedChanges) {
        _add_error("A maximum of 64 import option changes may be staged per call.");
        return;
    }
    if (before == value) {
        _staged_values.erase(name);
        return;
    }
    _staged_values[name] = value;
}

godot::Array FennaraRunAssetImportScriptContext::get_staged_changes() const {
    godot::Array changes;
    godot::Array keys = _staged_values.keys();
    for (int i = 0; i < keys.size(); i++) {
        const godot::String name = keys[i];
        const godot::Variant before = _options[name];
        const godot::Variant after = _staged_values[name];
        godot::Dictionary change;
        change["name"] = name;
        change["before"] = before;
        change["after"] = after;
        change["type"] = godot::Variant::get_type_name(after.get_type());
        changes.append(change);
    }
    return changes;
}

void FennaraRunAssetImportScriptContext::discard_import_option_change(
    const godot::String &name) {
    _staged_values.erase(name);
}

godot::Ref<godot::Resource>
FennaraRunAssetImportScriptContext::get_imported_resource() const {
    return _imported_resource;
}

godot::Node *FennaraRunAssetImportScriptContext::_ensure_temporary_host() {
    if (_temporary_host != nullptr) {
        return _temporary_host;
    }
    godot::EditorInterface *editor = godot::EditorInterface::get_singleton();
    godot::Control *base = editor != nullptr ? editor->get_base_control() : nullptr;
    if (base == nullptr || !base->is_inside_tree()) {
        _add_error("The live editor SceneTree is unavailable.", "instantiate");
        return nullptr;
    }
    _temporary_host = memnew(godot::Node);
    _temporary_host->set_name("FennaraAssetImportScriptHost");
    _temporary_host->set_process_mode(godot::Node::PROCESS_MODE_DISABLED);
    base->add_child(_temporary_host);
    return _temporary_host;
}

godot::Node *FennaraRunAssetImportScriptContext::instantiate_imported_scene() {
    godot::Ref<godot::PackedScene> packed = _imported_resource;
    if (!packed.is_valid()) {
        _add_error("The imported resource is not a PackedScene.", "instantiate");
        return nullptr;
    }
    const godot::Variant root_script = _options.get("nodes/root_script", godot::Variant());
    if (has_effective_value(root_script)) {
        _add_error(
            "Live imported-scene instantiation is refused because nodes/root_script is set.",
            "instantiate");
        return nullptr;
    }
    godot::Node *host = _ensure_temporary_host();
    if (host == nullptr) {
        return nullptr;
    }
    godot::Node *instance = packed->instantiate(godot::PackedScene::GEN_EDIT_STATE_DISABLED);
    if (instance == nullptr) {
        _add_error("PackedScene instantiation returned null.", "instantiate");
        return nullptr;
    }
    host->add_child(instance);
    return instance;
}

godot::Array FennaraRunAssetImportScriptContext::get_generated_files() const {
    return _generated_files;
}

godot::Array FennaraRunAssetImportScriptContext::get_dependencies() const {
    return _dependencies;
}

godot::Dictionary FennaraRunAssetImportScriptContext::get_subresource_summary() {
    godot::Dictionary summary;
    summary["resource_class"] = _imported_resource.is_valid()
        ? _imported_resource->get_class()
        : godot::String();
    summary["nodes"] = 0;
    summary["mesh_instances"] = 0;
    summary["skeletons"] = 0;
    summary["animation_players"] = 0;
    summary["animations"] = 0;
    const godot::Variant subresources =
        _options.get("_subresources", godot::Dictionary());
    summary["import_override_entries"] =
        subresources.get_type() == godot::Variant::DICTIONARY
            ? godot::Dictionary(subresources).size()
            : 0;

    godot::Ref<godot::PackedScene> packed = _imported_resource;
    if (!packed.is_valid()) {
        return summary;
    }
    godot::Node *instance =
        packed->instantiate(godot::PackedScene::GEN_EDIT_STATE_DISABLED);
    if (instance == nullptr) {
        _add_error("PackedScene summary instantiation returned null.", "summary");
        return summary;
    }
    count_scene_nodes(instance, summary);
    instance->queue_free();
    return summary;
}

void FennaraRunAssetImportScriptContext::log(const godot::Variant &value) {
    godot::String text = value.stringify();
    if (text.length() > kMaximumLogCharacters) {
        text = text.left(kMaximumLogCharacters) +
               "\n[truncated by Fennara]";
    }
    if (_logs.size() < kMaximumLogEntries) {
        _logs.append(text);
        return;
    }
    _logs[kMaximumLogEntries - 1] =
        "[additional log entries omitted by Fennara]";
}

void FennaraRunAssetImportScriptContext::error(const godot::String &message) {
    _add_error(message);
}

void FennaraRunAssetImportScriptContext::require(
    bool condition,
    const godot::String &message) {
    if (!condition) {
        _add_error(message, "require");
    }
}

godot::Array FennaraRunAssetImportScriptContext::get_logs() const {
    return _logs;
}

godot::Array FennaraRunAssetImportScriptContext::get_edit_errors() const {
    return _errors;
}

bool FennaraRunAssetImportScriptContext::_option_is_editable(
    const godot::String &name,
    godot::String *reason) const {
    auto reject = [reason](const godot::String &value) {
        if (reason != nullptr) {
            *reason = value;
        }
        return false;
    };

    if (name == "_subresources") {
        return reject("subresource_overrides");
    }
    if (name.begins_with("import_script/") || name == "nodes/root_script") {
        return reject("executes_or_attaches_code");
    }
    if (name.begins_with("materials/") || name.contains("save_to_file") ||
        name.contains("extract_path")) {
        return reject("external_project_file_write");
    }
    if (_importer == "texture") {
        return true;
    }
    if (_importer == "scene") {
        if (name == "nodes/apply_root_scale" || name == "nodes/root_scale" ||
            name == "nodes/import_as_skeleton_bones" ||
            name == "nodes/use_name_suffixes" ||
            name == "nodes/use_node_type_suffixes" ||
            name.begins_with("meshes/") || name.begins_with("skins/") ||
            name.begins_with("animation/")) {
            return true;
        }
        return reject("unsupported_scene_import_effect");
    }
    return reject("unknown_importer_effect");
}

void FennaraRunAssetImportScriptContext::_add_error(
    const godot::String &message,
    const godot::String &source) {
    _errors.append(run_asset_import_script_internal::make_runtime_error(message, source));
}

} // namespace fennara
