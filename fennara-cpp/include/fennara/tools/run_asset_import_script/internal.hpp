#pragma once

#include <godot_cpp/classes/config_file.hpp>
#include <godot_cpp/classes/gd_script.hpp>
#include <godot_cpp/classes/resource.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara::run_asset_import_script_internal {

struct ImportSnapshot {
    godot::String asset_path;
    godot::String sidecar_path;
    godot::String sidecar_text;
    godot::String sidecar_hash;
    godot::String importer;
    godot::Dictionary options;
    godot::Array generated_files;
    godot::Array dependencies;
    bool import_valid = false;
    godot::Ref<godot::ConfigFile> config;
};

godot::Dictionary make_runtime_error(const godot::String &message,
                                     const godot::String &source);
godot::String normalize_asset_path(const godot::String &path);
bool load_import_snapshot(const godot::String &asset_path,
                          ImportSnapshot &snapshot,
                          godot::Dictionary &result);
godot::String write_or_resolve_script_path(const godot::String &asset_path,
                                           const godot::String &code,
                                           const godot::String &script_path,
                                           godot::Dictionary &result);
godot::Ref<godot::GDScript> load_script(const godot::String &script_path,
                                        godot::Dictionary &result);
bool validate_script_contract(const godot::Ref<godot::GDScript> &script,
                              godot::Dictionary &result);
godot::Variant instantiate_runner(const godot::Ref<godot::GDScript> &script,
                                  godot::Dictionary &result);
bool sidecar_matches_snapshot(const ImportSnapshot &snapshot);
bool write_text_file(const godot::String &path,
                     const godot::String &text,
                     godot::String &error);
godot::Dictionary apply_and_reimport(
    const ImportSnapshot &snapshot,
    const godot::Array &staged_changes);
bool selected_import_dock_is_safe(const godot::String &asset_path,
                                  bool &target_selected,
                                  godot::String &error);
void refresh_selected_import_dock(const godot::String &asset_path,
                                  bool target_selected);

} // namespace fennara::run_asset_import_script_internal
