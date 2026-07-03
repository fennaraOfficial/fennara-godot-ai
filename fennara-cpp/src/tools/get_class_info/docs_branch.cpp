#include "fennara/tools/get_class_info/docs_branch.hpp"

#include <godot_cpp/classes/engine.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara::get_class_info {

namespace {

constexpr const char *kFallbackDocsBranch = "master";

bool variant_to_int(const godot::Variant &value, int64_t &out) {
    const godot::Variant::Type type = value.get_type();
    if (type != godot::Variant::INT && type != godot::Variant::FLOAT) {
        return false;
    }

    out = static_cast<int64_t>(value);
    return true;
}

godot::String branch_from_major_minor(int64_t major, int64_t minor) {
    if (major <= 0 || minor < 0) {
        return fallback_docs_branch();
    }
    return godot::String::num_int64(major) + "." + godot::String::num_int64(minor);
}

godot::String branch_from_version_string(const godot::String &version_string) {
    const godot::String clean = version_string.strip_edges();
    if (clean.is_empty()) {
        return fallback_docs_branch();
    }

    godot::PackedStringArray parts = clean.split(".");
    if (parts.size() < 2 || !parts[0].is_valid_int() ||
        !parts[1].is_valid_int()) {
        return fallback_docs_branch();
    }

    return branch_from_major_minor(parts[0].to_int(), parts[1].to_int());
}

} // namespace

godot::String fallback_docs_branch() {
    return kFallbackDocsBranch;
}

godot::String docs_branch_from_version_info(const godot::Dictionary &version_info) {
    int64_t major = 0;
    int64_t minor = -1;
    if (variant_to_int(version_info.get("major", godot::Variant()), major) &&
        variant_to_int(version_info.get("minor", godot::Variant()), minor) &&
        major > 0 && minor >= 0) {
        return branch_from_major_minor(major, minor);
    }

    return branch_from_version_string(version_info.get("string", ""));
}

godot::String docs_branch_for_running_godot() {
    godot::Engine *engine = godot::Engine::get_singleton();
    if (engine == nullptr) {
        return fallback_docs_branch();
    }

    return docs_branch_from_version_info(engine->get_version_info());
}

} // namespace fennara::get_class_info
