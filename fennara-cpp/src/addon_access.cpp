#include "fennara/addon_access.hpp"

#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/json.hpp>
#include <godot_cpp/classes/project_settings.hpp>

#include <filesystem>
#include <system_error>

namespace fennara::addon_access {

namespace {

constexpr const char *kConfigPath = "user://.fennara/addon_access.json";
constexpr const char *kFennaraAddonRoot = "res://addons/fennara";
constexpr const char *kAiGuidanceRoot = "res://addons/fennara/ai/";

godot::String config_path_abs() {
    return godot::ProjectSettings::get_singleton()->globalize_path(kConfigPath);
}

godot::Array load_allowed_roots() {
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(config_path_abs(), godot::FileAccess::READ);
    if (!file.is_valid()) {
        return godot::Array();
    }

    const godot::String contents = file->get_as_text().strip_edges();
    if (contents.is_empty()) {
        return godot::Array();
    }
    godot::Variant parsed = godot::JSON::parse_string(contents);
    if (parsed.get_type() != godot::Variant::DICTIONARY) {
        return godot::Array();
    }

    godot::Dictionary data = parsed;
    godot::Variant roots = data.get("allowed_addon_roots", godot::Array());
    if (roots.get_type() != godot::Variant::ARRAY) {
        return godot::Array();
    }
    return roots;
}

godot::String display_name_for_addon(const godot::String &root) {
    godot::String plugin_cfg = root.path_join("plugin.cfg");
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(plugin_cfg, godot::FileAccess::READ);
    if (!file.is_valid()) {
        return root.get_file();
    }

    godot::PackedStringArray lines = file->get_as_text().split("\n");
    for (int i = 0; i < lines.size(); i++) {
        godot::String line = lines[i].strip_edges();
        if (!line.begins_with("name=")) {
            continue;
        }
        godot::String name = line.substr(5).strip_edges();
        if ((name.begins_with("\"") && name.ends_with("\"")) ||
            (name.begins_with("'") && name.ends_with("'"))) {
            name = name.substr(1, name.length() - 2);
        }
        if (!name.is_empty()) {
            return name;
        }
    }
    return root.get_file();
}

std::filesystem::path native_path_for(const godot::String &path) {
    const godot::String globalized =
        godot::ProjectSettings::get_singleton()->globalize_path(path);
    return std::filesystem::u8path(globalized.utf8().get_data());
}

bool is_canonical_descendant(const std::filesystem::path &path,
                             const std::filesystem::path &root) {
    std::error_code error;
    const std::filesystem::path canonical_root =
        std::filesystem::canonical(root, error);
    if (error) {
        return false;
    }

    const std::filesystem::path canonical_path =
        std::filesystem::canonical(path, error);
    if (error) {
        return false;
    }

    const std::filesystem::path relative =
        canonical_path.lexically_relative(canonical_root);
    if (relative.empty() || relative.is_absolute()) {
        return false;
    }
    for (const std::filesystem::path &part : relative) {
        if (part == "..") {
            return false;
        }
    }
    return true;
}

godot::Dictionary addon_entry(const godot::String &root,
                              bool allowed,
                              bool missing = false) {
    godot::Dictionary entry;
    entry["path"] = root;
    entry["allowed"] = allowed;
    entry["display_name"] = display_name_for_addon(root);
    entry["has_plugin_cfg"] = godot::FileAccess::file_exists(root.path_join("plugin.cfg"));
    if (missing) {
        entry["missing"] = true;
    }
    return entry;
}

} // namespace

godot::String normalize_res_path(const godot::String &path) {
    godot::String normalized = path.replace("\\", "/").strip_edges();
    if (normalized.begins_with("res://")) {
        return normalized.trim_suffix("/");
    }

    godot::String project_root =
        godot::ProjectSettings::get_singleton()->globalize_path("res://").replace("\\", "/");
    if (normalized.begins_with(project_root)) {
        return (godot::String("res://") + normalized.substr(project_root.length()))
            .trim_suffix("/");
    }

    if (normalized.begins_with("./")) {
        normalized = normalized.substr(2);
    }
    if (normalized.begins_with("/")) {
        normalized = normalized.substr(1);
    }
    return (godot::String("res://") + normalized).trim_suffix("/");
}

godot::String addon_root_for_path(const godot::String &path) {
    godot::String normalized = normalize_res_path(path);
    if (normalized == "res://addons" || !normalized.begins_with("res://addons/")) {
        return "";
    }

    godot::String rest = normalized.substr(godot::String("res://addons/").length());
    int slash = rest.find("/");
    godot::String addon_name = slash < 0 ? rest : rest.substr(0, slash);
    if (addon_name.is_empty()) {
        return "";
    }
    return godot::String("res://addons/") + addon_name;
}

bool is_addons_root(const godot::String &path) {
    return normalize_res_path(path) == "res://addons";
}

bool is_locked_addon_root(const godot::String &addon_root) {
    return normalize_res_path(addon_root) == godot::String(kFennaraAddonRoot);
}

bool is_ai_guidance_path(const godot::String &path) {
    const godot::String normalized = normalize_res_path(path);
    if (!normalized.begins_with(kAiGuidanceRoot) ||
        !normalized.to_lower().ends_with(".md")) {
        return false;
    }

    const godot::String relative =
        normalized.substr(godot::String(kAiGuidanceRoot).length());
    const godot::PackedStringArray parts = relative.split("/");
    for (int i = 0; i < parts.size(); i++) {
        if (parts[i].is_empty() || parts[i] == "." || parts[i] == "..") {
            return false;
        }
    }
    return !parts.is_empty() &&
           is_canonical_descendant(
               native_path_for(normalized),
               native_path_for(kAiGuidanceRoot));
}

godot::Array allowed_addon_roots() {
    godot::Array raw = load_allowed_roots();
    godot::Array cleaned;
    for (int i = 0; i < raw.size(); i++) {
        godot::String root = addon_root_for_path(godot::String(raw[i]));
        if (root.is_empty() || is_locked_addon_root(root) || cleaned.has(root)) {
            continue;
        }
        cleaned.append(root);
    }
    return cleaned;
}

void set_allowed_addon_roots(const godot::Array &roots) {
    godot::Array cleaned;
    for (int i = 0; i < roots.size(); i++) {
        godot::String root = addon_root_for_path(godot::String(roots[i]));
        if (root.is_empty() || is_locked_addon_root(root) || cleaned.has(root)) {
            continue;
        }
        cleaned.append(root);
    }

    godot::String abs_path = config_path_abs();
    godot::String dir_path = abs_path.get_base_dir();
    if (!godot::DirAccess::dir_exists_absolute(dir_path)) {
        godot::DirAccess::make_dir_recursive_absolute(dir_path);
    }

    godot::Dictionary data;
    data["version"] = 1;
    data["allowed_addon_roots"] = cleaned;
    godot::Ref<godot::FileAccess> file =
        godot::FileAccess::open(abs_path, godot::FileAccess::WRITE);
    if (file.is_valid()) {
        file->store_string(godot::JSON::stringify(data, "  "));
    }
}

bool is_addon_root_allowed(const godot::String &addon_root) {
    return !is_locked_addon_root(addon_root);
}

godot::Dictionary status() {
    godot::Array allowed = allowed_addon_roots();
    godot::Array discoverable;
    godot::Array locked;
    godot::Array missing;

    godot::Ref<godot::DirAccess> dir = godot::DirAccess::open("res://addons");
    if (dir.is_valid()) {
        dir->list_dir_begin();
        godot::String name = dir->get_next();
        while (!name.is_empty()) {
            if (dir->current_is_dir() && name != "." && name != ".." && !name.begins_with(".")) {
                godot::String root = godot::String("res://addons/") + name;
                if (is_locked_addon_root(root)) {
                    godot::Dictionary entry;
                    entry["path"] = root;
                    entry["display_name"] = display_name_for_addon(root);
                    entry["reason"] = "fennara_plugin_protected";
                    locked.append(entry);
                } else {
                    discoverable.append(addon_entry(root, true));
                }
            }
            name = dir->get_next();
        }
        dir->list_dir_end();
    }

    for (int i = 0; i < allowed.size(); i++) {
        godot::String root = allowed[i];
        if (!godot::DirAccess::dir_exists_absolute(root)) {
            missing.append(addon_entry(root, true, true));
        }
    }

    godot::Dictionary result;
    result["discoverable"] = discoverable;
    result["locked"] = locked;
    result["missing"] = missing;
    result["policy_note"] =
        "Fennara tools can access the whole project except Fennara's own protected addon folder.";
    return result;
}

godot::Dictionary blocked_result(const godot::String &path) {
    godot::String normalized = normalize_res_path(path);
    godot::String root = addon_root_for_path(normalized);
    godot::Dictionary result;
    result["success"] = false;
    result["status"] = "blocked";
    result["blocked_path"] = normalized;
    result["blocked_addon_root"] = root;

    if (is_locked_addon_root(root)) {
        result["block_reason"] = "fennara_plugin_protected";
        result["error"] = "Fennara plugin files are protected: " + root;
        return result;
    }

    result["block_reason"] = "blocked_path";
    result["error"] = "Path is blocked: " + normalized;
    return result;
}

bool is_path_allowed(const godot::String &path,
                     bool allow_addons_root_discovery,
                     godot::Dictionary &blocked_out) {
    godot::String normalized = normalize_res_path(path);
    if (allow_addons_root_discovery && normalized == "res://addons") {
        return true;
    }

    godot::String root = addon_root_for_path(normalized);
    if (root.is_empty()) {
        return true;
    }
    if (is_locked_addon_root(root)) {
        blocked_out = blocked_result(normalized);
        return false;
    }
    return true;
}

} // namespace fennara::addon_access
