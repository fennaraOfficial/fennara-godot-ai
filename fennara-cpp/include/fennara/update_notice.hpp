#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/packed_byte_array.hpp>
#include <godot_cpp/variant/string.hpp>

namespace fennara::update_notice {

bool begin_check();
void complete_check(bool success,
                    int response_code,
                    const godot::PackedByteArray &body,
                    const godot::String &error = "");
bool is_update_available();
godot::String current_version();
godot::String latest_version();
godot::String warning_text();
godot::Dictionary status();

} // namespace fennara::update_notice
