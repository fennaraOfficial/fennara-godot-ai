#pragma once

#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara::control_auth {

godot::String verified_daemon_header();
void request_legacy_daemon_shutdown();
bool verify_daemon_and_append_header(godot::PackedStringArray &headers);

} // namespace fennara::control_auth
