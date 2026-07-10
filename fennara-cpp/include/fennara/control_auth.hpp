#pragma once

#include <atomic>

#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara::control_auth {

godot::String verified_daemon_header(const std::atomic_bool *cancelled = nullptr);
void request_legacy_daemon_shutdown(const std::atomic_bool *cancelled = nullptr);
bool verify_daemon_and_append_header(godot::PackedStringArray &headers);

} // namespace fennara::control_auth
