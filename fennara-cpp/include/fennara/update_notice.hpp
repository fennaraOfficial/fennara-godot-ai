#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

#include <atomic>

namespace fennara::update_notice {

void check_once(const std::atomic_bool *cancelled = nullptr);
bool is_update_available();
godot::String current_version();
godot::String latest_version();
godot::String channel();
godot::String track();
godot::String target_release_tag();
godot::String source_commit();
godot::String warning_text();
godot::Dictionary status();

} // namespace fennara::update_notice
