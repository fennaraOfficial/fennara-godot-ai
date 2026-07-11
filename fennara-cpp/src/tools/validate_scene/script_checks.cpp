#include "fennara/tools/validate_scene.hpp"

#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/packed_scene.hpp>
#include <godot_cpp/classes/reg_ex.hpp>
#include <godot_cpp/classes/reg_ex_match.hpp>
#include <godot_cpp/classes/resource.hpp>
#include <godot_cpp/classes/resource_loader.hpp>
#include <godot_cpp/classes/script.hpp>

namespace fennara {

namespace {

godot::Ref<godot::Script> s_script_from_state(
    const godot::Ref<godot::SceneState> &state,
    int node_idx,
    godot::String *script_path = nullptr) {
    int prop_count = state->get_node_property_count(node_idx);
    for (int p = 0; p < prop_count; p++) {
        godot::String prop_name =
            godot::String(state->get_node_property_name(node_idx, p));
        if (prop_name != "script") {
            continue;
        }
        godot::Variant val = state->get_node_property_value(node_idx, p);
        if (val.get_type() != godot::Variant::OBJECT) {
            return godot::Ref<godot::Script>();
        }
        godot::Object *obj = val;
        auto *script = godot::Object::cast_to<godot::Script>(obj);
        if (!script) {
            return godot::Ref<godot::Script>();
        }
        if (script_path) {
            *script_path = script->get_path();
        }
        return godot::Ref<godot::Script>(script);
    }
    return godot::Ref<godot::Script>();
}

godot::Ref<godot::Script> s_instanced_root_script(
    const godot::Ref<godot::SceneState> &state,
    int node_idx,
    godot::String &script_path,
    godot::String &instance_scene_path) {
    godot::Ref<godot::PackedScene> instance = state->get_node_instance(node_idx);
    if (!instance.is_valid()) {
        return godot::Ref<godot::Script>();
    }

    instance_scene_path = instance->get_path();
    godot::Ref<godot::SceneState> instance_state = instance->get_state();
    if (!instance_state.is_valid() || instance_state->get_node_count() <= 0) {
        return godot::Ref<godot::Script>();
    }
    return s_script_from_state(instance_state, 0, &script_path);
}

godot::Dictionary s_scene_props_for_node(
    const godot::Ref<godot::SceneState> &state,
    int node_idx) {
    godot::Dictionary scene_props;
    int prop_count = state->get_node_property_count(node_idx);
    for (int p = 0; p < prop_count; p++) {
        godot::String pname =
            godot::String(state->get_node_property_name(node_idx, p));
        scene_props[pname] = state->get_node_property_value(node_idx, p);
    }
    return scene_props;
}

godot::String s_join_samples(const godot::Array &samples) {
    godot::PackedStringArray parts;
    for (int i = 0; i < samples.size(); i++) {
        parts.append(samples[i]);
    }
    return godot::String(", ").join(parts);
}

godot::String s_format_unset_properties(const godot::Array &properties) {
    godot::PackedStringArray parts;
    for (int i = 0; i < properties.size(); i++) {
        if (properties[i].get_type() != godot::Variant::DICTIONARY) {
            continue;
        }
        godot::Dictionary prop = properties[i];
        godot::String name = prop.get("name", "");
        godot::String type = prop.get("type", "");
        if (name.is_empty()) {
            continue;
        }
        parts.append(type.is_empty() ? name : name + " (" + type + ")");
    }
    return godot::String(", ").join(parts);
}

void s_append_unique_string(
    godot::Array &values,
    godot::Dictionary &seen,
    const godot::String &value,
    int limit) {
    if (value.is_empty() || seen.has(value)) {
        return;
    }
    seen[value] = true;
    if (limit <= 0 || values.size() < limit) {
        values.append(value);
    }
}

void s_record_unset_export_group(
    godot::Dictionary &groups,
    godot::Array &group_order,
    const godot::String &node_path,
    const godot::String &script_path,
    const godot::String &instance_scene_path,
    const godot::String &prop_name,
    const godot::String &type_label) {
    godot::String key = script_path;

    godot::Dictionary group;
    if (groups.has(key)) {
        group = groups[key];
    } else {
        group["script_path"] = script_path;
        group["properties"] = godot::Array();
        group["property_seen"] = godot::Dictionary();
        group["node_count"] = 0;
        group["sample_seen"] = godot::Dictionary();
        group["instance_scenes"] = godot::Array();
        group["instance_scene_seen"] = godot::Dictionary();
        group_order.append(key);
    }

    godot::String prop_key = prop_name + godot::String("\n") + type_label;
    godot::Dictionary property_seen =
        group.get("property_seen", godot::Dictionary());
    if (!property_seen.has(prop_key)) {
        property_seen[prop_key] = true;
        godot::Dictionary prop;
        prop["name"] = prop_name;
        prop["type"] = type_label;
        godot::Array properties = group.get("properties", godot::Array());
        properties.append(prop);
        group["properties"] = properties;
        group["property_seen"] = property_seen;
    }

    godot::Dictionary sample_seen = group.get("sample_seen", godot::Dictionary());
    int node_count = static_cast<int>(group.get("node_count", 0));
    if (!sample_seen.has(node_path)) {
        sample_seen[node_path] = true;
        group["node_count"] = node_count + 1;
        group["sample_seen"] = sample_seen;
    }

    godot::Array instance_scenes =
        group.get("instance_scenes", godot::Array());
    godot::Dictionary instance_scene_seen =
        group.get("instance_scene_seen", godot::Dictionary());
    if (!instance_scene_path.is_empty() &&
        !instance_scene_seen.has(instance_scene_path)) {
        int total = static_cast<int>(group.get("instance_scene_total", 0));
        group["instance_scene_total"] = total + 1;
    }
    s_append_unique_string(
        instance_scenes, instance_scene_seen, instance_scene_path, 5);
    group["instance_scenes"] = instance_scenes;
    group["instance_scene_seen"] = instance_scene_seen;

    groups[key] = group;
}

godot::String s_count_label(int count, const godot::String &singular,
                            const godot::String &plural) {
    return godot::String::num_int64(count) + " " +
           (count == 1 ? singular : plural);
}

godot::String s_omitted_label(int total, int shown) {
    int omitted = total - shown;
    if (omitted <= 0) {
        return "";
    }
    return ", " + s_count_label(omitted, "omitted", "omitted");
}

godot::String s_script_cache_key(
    const godot::Ref<godot::Script> &script,
    const godot::String &script_path) {
    if (!script_path.is_empty()) {
        return "path:" + script_path;
    }
    return "instance:" +
           godot::String::num_uint64(script->get_instance_id());
}

godot::Array s_unset_export_candidates(
    const godot::Ref<godot::Script> &script) {
    godot::Array candidates;
    godot::TypedArray<godot::Dictionary> prop_list =
        script->get_script_property_list();
    for (int p = 0; p < prop_list.size(); p++) {
        godot::Dictionary prop_info = prop_list[p];
        int usage = prop_info.get("usage", 0);
        if (!(usage & godot::PROPERTY_USAGE_STORAGE)) continue;

        int type = prop_info.get("type", 0);
        int hint = prop_info.get("hint", 0);
        bool is_resource_export =
            (hint == 17) || (type == godot::Variant::OBJECT);
        if (!is_resource_export) continue;

        godot::String prop_name = prop_info.get("name", "");
        if (prop_name.is_empty()) continue;

        godot::Variant default_val =
            script->get_property_default_value(godot::StringName(prop_name));
        if (default_val.get_type() != godot::Variant::NIL) continue;

        godot::String hint_string = prop_info.get("hint_string", "");
        godot::Dictionary candidate;
        candidate["name"] = prop_name;
        candidate["type"] =
            hint_string.is_empty() ? "Object" : hint_string;
        candidates.append(candidate);
    }
    return candidates;
}

} // namespace

void FennaraValidateSceneTool::_check_unset_export_vars(
    const godot::Ref<godot::SceneState> &state, godot::Array &issues) {
    int count = state->get_node_count();
    godot::Dictionary groups;
    godot::Array group_order;
    godot::Dictionary candidate_cache;

    for (int i = 0; i < count; i++) {
        godot::String script_path;
        godot::Ref<godot::Script> script =
            s_script_from_state(state, i, &script_path);
        godot::String instance_scene_path;
        if (!script.is_valid()) {
            script = s_instanced_root_script(
                state, i, script_path, instance_scene_path);
        }

        if (!script.is_valid()) continue;

        godot::String cache_key = s_script_cache_key(script, script_path);
        godot::Array candidates;
        if (candidate_cache.has(cache_key)) {
            candidates = candidate_cache[cache_key];
        } else {
            candidates = s_unset_export_candidates(script);
            candidate_cache[cache_key] = candidates;
        }
        if (candidates.is_empty()) continue;

        godot::Dictionary scene_props = s_scene_props_for_node(state, i);
        for (int p = 0; p < candidates.size(); p++) {
            godot::Dictionary candidate = candidates[p];
            godot::String prop_name = candidate.get("name", "");
            if (scene_props.has(prop_name)) {
                godot::Variant val = scene_props[prop_name];
                if (val.get_type() != godot::Variant::NIL) continue;
            }

            s_record_unset_export_group(
                groups,
                group_order,
                _build_node_path(state, i),
                script_path,
                instance_scene_path,
                prop_name,
                candidate.get("type", "Object"));
        }
    }

    for (int i = 0; i < group_order.size(); i++) {
        godot::String key = group_order[i];
        godot::Dictionary group = groups[key];
        godot::String script_path = group.get("script_path", "");
        godot::Array properties = group.get("properties", godot::Array());
        int node_count = static_cast<int>(group.get("node_count", 0));
        godot::Array instance_scenes =
            group.get("instance_scenes", godot::Array());

        godot::String severity = "info";
        godot::String message =
            godot::String("Script ") +
            (script_path.is_empty() ? godot::String("<unknown>") : script_path) +
            " has " +
            s_count_label(properties.size(), "unset exported Object/Resource var",
                          "unset exported Object/Resource vars");
        godot::String prop_text = s_format_unset_properties(properties);
        if (!prop_text.is_empty()) {
            message += ": " + prop_text;
        }
        message += " on " + s_count_label(node_count, "node", "nodes");
        if (!script_path.is_empty()) {
            message += " using this script";
        }
        if (!instance_scenes.is_empty()) {
            message += ". Instanced scene samples: " +
                       s_join_samples(instance_scenes);
            message += s_omitted_label(
                static_cast<int>(group.get("instance_scene_total",
                                           instance_scenes.size())),
                instance_scenes.size());
        }
        message += ". Ignore this note if these references are intentionally optional or assigned at runtime";

        godot::Dictionary extra;
        extra["properties"] = properties;
        extra["script_path"] = script_path;
        extra["node_count"] = node_count;
        if (!instance_scenes.is_empty()) {
            extra["instance_scenes"] = instance_scenes;
        }

        _add_issue(issues, "", "unset_export_var", severity, message, extra);
    }
}

namespace {

void s_collect_regex_matches(
    const godot::Ref<godot::RegEx> &re, const godot::String &line,
    int group, godot::Array &out) {
    godot::TypedArray<godot::RegExMatch> matches = re->search_all(line);
    for (int m = 0; m < matches.size(); m++) {
        godot::Ref<godot::RegExMatch> match = matches[m];
        godot::String val = match->get_string(group);
        if (!val.is_empty()) out.append(val);
    }
}

godot::String s_strip_inline_comment(const godot::String &line) {
    int hash_pos = line.find("#");
    if (hash_pos <= 0) return line;

    int quote_count = 0;
    for (int i = 0; i < hash_pos; i++) {
        if (line[i] == '"') quote_count++;
    }
    if (quote_count % 2 != 0) return line;

    return line.substr(0, hash_pos);
}

godot::Node *s_get_receiver_node(
    godot::Node *script_node, godot::Node *root,
    const godot::Dictionary &alias_nodes,
    const godot::String &receiver_name) {
    if (receiver_name.is_empty() || receiver_name == "self") {
        return script_node;
    }

    if (!alias_nodes.has(receiver_name)) return nullptr;

    godot::Variant alias_value = alias_nodes[receiver_name];
    if (alias_value.get_type() != godot::Variant::OBJECT) return nullptr;

    godot::Object *obj = alias_value;
    godot::Node *alias_node = godot::Object::cast_to<godot::Node>(obj);
    if (!alias_node) return nullptr;

    godot::Node *cursor = alias_node;
    while (cursor) {
        if (cursor == root) return alias_node;
        cursor = cursor->get_parent();
    }

    return nullptr;
}

bool s_ref_goes_above_root(
    godot::Node *base_node, godot::Node *root, const godot::String &ref) {
    godot::PackedStringArray segs = ref.split("/");
    int up = 0;
    for (int i = 0; i < segs.size(); i++) {
        if (segs[i] == "..") up++;
        else break;
    }

    int depth = 0;
    godot::Node *cursor = base_node;
    while (cursor != root && cursor != nullptr) {
        depth++;
        cursor = cursor->get_parent();
    }

    return up > depth;
}

void s_try_register_alias(
    const godot::Ref<godot::RegEx> &re_assign_dollar,
    const godot::Ref<godot::RegEx> &re_assign_dollar_quoted,
    const godot::Ref<godot::RegEx> &re_assign_get_node,
    const godot::String &line,
    godot::Node *script_node,
    godot::Node *root,
    godot::Dictionary &alias_nodes) {
    godot::Ref<godot::RegExMatch> match = re_assign_dollar->search(line);
    if (match.is_null()) {
        match = re_assign_dollar_quoted->search(line);
    }

    if (!match.is_null()) {
        godot::String alias_name = match->get_string(1);
        godot::String ref_path = match->get_string(2);
        godot::Node *target =
            script_node->get_node_or_null(godot::NodePath(ref_path));
        if (target) alias_nodes[alias_name] = target;
        return;
    }

    match = re_assign_get_node->search(line);
    if (match.is_null()) return;

    godot::String alias_name = match->get_string(1);
    godot::String receiver_name = match->get_string(2);
    godot::String ref_path = match->get_string(3);

    godot::Node *receiver =
        s_get_receiver_node(script_node, root, alias_nodes, receiver_name);
    if (!receiver) return;

    godot::Node *target = receiver->get_node_or_null(godot::NodePath(ref_path));
    if (target) alias_nodes[alias_name] = target;
}

void s_check_script_refs_recursive(
    godot::Node *node, godot::Node *root,
    const godot::Ref<godot::RegEx> &re_dollar,
    const godot::Ref<godot::RegEx> &re_dollar_quoted,
    const godot::Ref<godot::RegEx> &re_get_node,
    const godot::Ref<godot::RegEx> &re_assign_dollar,
    const godot::Ref<godot::RegEx> &re_assign_dollar_quoted,
    const godot::Ref<godot::RegEx> &re_assign_get_node,
    godot::Array &issues) {

    godot::Ref<godot::Script> script = node->get_script();
    if (script.is_valid()) {
        godot::String spath = script->get_path();
        if (spath.get_extension() == "gd" && !spath.contains("::") &&
            godot::FileAccess::file_exists(spath)) {
            godot::String node_scene_path =
                godot::String(root->get_path_to(node));

            godot::Ref<godot::FileAccess> f =
                godot::FileAccess::open(spath, godot::FileAccess::READ);
            if (f.is_valid()) {
                godot::String content = f->get_as_text();
                f.unref();

                godot::PackedStringArray lines = content.split("\n");
                godot::Dictionary alias_nodes;
                for (int li = 0; li < lines.size(); li++) {
                    godot::String line =
                        s_strip_inline_comment(lines[li]).strip_edges();
                    if (line.is_empty() || line.begins_with("#")) continue;

                    s_try_register_alias(
                        re_assign_dollar, re_assign_dollar_quoted,
                        re_assign_get_node, line, node, root, alias_nodes);

                    godot::Array paths;
                    s_collect_regex_matches(re_dollar, line, 1, paths);
                    s_collect_regex_matches(re_dollar_quoted, line, 1, paths);
                    for (int pi = 0; pi < paths.size(); pi++) {
                        godot::String ref = paths[pi];
                        godot::Node *target =
                            node->get_node_or_null(godot::NodePath(ref));
                        if (target) continue;
                        if (s_ref_goes_above_root(node, root, ref)) continue;

                        godot::Dictionary issue;
                        issue["node"] = node_scene_path;
                        issue["node_path"] = node_scene_path;
                        issue["check"] = "invalid_script_node_ref";
                        issue["severity"] = "warning";
                        issue["message"] =
                            godot::String("Script '") + spath +
                            "' line " +
                            godot::String::num_int64(li + 1) +
                            ": node path '" + ref +
                            "' does not resolve to any node";
                        issue["script"] = spath;
                        issue["line"] = li + 1;
                        issue["ref_path"] = ref;
                        issues.append(issue);
                    }

                    godot::TypedArray<godot::RegExMatch> get_node_matches =
                        re_get_node->search_all(line);
                    for (int mi = 0; mi < get_node_matches.size(); mi++) {
                        godot::Ref<godot::RegExMatch> match =
                            get_node_matches[mi];
                        godot::String receiver_name = match->get_string(1);
                        godot::String ref = match->get_string(2);

                        godot::Node *receiver = s_get_receiver_node(
                            node, root, alias_nodes, receiver_name);
                        if (!receiver) continue;

                        godot::Node *target =
                            receiver->get_node_or_null(godot::NodePath(ref));
                        if (target) continue;
                        if (s_ref_goes_above_root(receiver, root, ref)) continue;

                        godot::Dictionary issue;
                        issue["node"] = node_scene_path;
                        issue["node_path"] = node_scene_path;
                        issue["check"] = "invalid_script_node_ref";
                        issue["severity"] = "warning";
                        issue["message"] =
                            godot::String("Script '") + spath +
                            "' line " +
                            godot::String::num_int64(li + 1) +
                            ": node path '" + ref +
                            "' does not resolve to any node";
                        issue["script"] = spath;
                        issue["line"] = li + 1;
                        issue["ref_path"] = ref;
                        issues.append(issue);
                    }
                }
            }
        }
    }

    for (int c = 0; c < node->get_child_count(); c++) {
        s_check_script_refs_recursive(
            node->get_child(c), root,
            re_dollar, re_dollar_quoted, re_get_node,
            re_assign_dollar, re_assign_dollar_quoted, re_assign_get_node,
            issues);
    }
}

} // namespace

void FennaraValidateSceneTool::_check_script_node_references(
    const godot::String &scene_path, godot::Array &issues) {
    godot::Ref<godot::PackedScene> packed =
        godot::ResourceLoader::get_singleton()->load(
            scene_path, "PackedScene",
            godot::ResourceLoader::CACHE_MODE_IGNORE);
    if (!packed.is_valid()) return;

    godot::Node *root = packed->instantiate();
    if (!root) return;

    godot::Ref<godot::RegEx> re_dollar;
    re_dollar.instantiate();
    re_dollar->compile("\\$(%?[A-Za-z_]\\w*(?:/\\w+)*)");

    godot::Ref<godot::RegEx> re_dollar_quoted;
    re_dollar_quoted.instantiate();
    re_dollar_quoted->compile("\\$\"([^\"]+)\"");

    godot::Ref<godot::RegEx> re_get_node;
    re_get_node.instantiate();
    re_get_node->compile(
        "(?:\\b([A-Za-z_]\\w*|self)\\s*\\.\\s*)?"
        "get_node(?:_or_null)?\\(\\s*[\"']([^\"']+)[\"']\\s*\\)");

    godot::Ref<godot::RegEx> re_assign_dollar;
    re_assign_dollar.instantiate();
    re_assign_dollar->compile(
        "(?:@onready\\s+)?var\\s+([A-Za-z_]\\w*)\\s*(?::[^=]+)?="
        "\\s*\\$(%?[A-Za-z_]\\w*(?:/\\w+)*)");

    godot::Ref<godot::RegEx> re_assign_dollar_quoted;
    re_assign_dollar_quoted.instantiate();
    re_assign_dollar_quoted->compile(
        "(?:@onready\\s+)?var\\s+([A-Za-z_]\\w*)\\s*(?::[^=]+)?="
        "\\s*\\$\"([^\"]+)\"");

    godot::Ref<godot::RegEx> re_assign_get_node;
    re_assign_get_node.instantiate();
    re_assign_get_node->compile(
        "(?:@onready\\s+)?var\\s+([A-Za-z_]\\w*)\\s*(?::[^=]+)?="
        "\\s*(?:([A-Za-z_]\\w*|self)\\s*\\.\\s*)?"
        "get_node(?:_or_null)?\\(\\s*[\"']([^\"']+)[\"']\\s*\\)");

    s_check_script_refs_recursive(
        root, root, re_dollar, re_dollar_quoted, re_get_node,
        re_assign_dollar, re_assign_dollar_quoted, re_assign_get_node,
        issues);

    root->queue_free();
}

} // namespace fennara
