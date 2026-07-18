#pragma once

#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>

#include <cstdint>

namespace fennara {

class EditorFilesystemState {
public:
    static EditorFilesystemState &get_singleton();

    godot::Dictionary snapshot() const;
    void on_resources_reimporting(const godot::PackedStringArray &paths);
    void on_resources_reimported(const godot::PackedStringArray &paths);

    bool begin_owned_import(const godot::String &asset_path,
                            godot::String &error);
    void finish_owned_import(bool success);

private:
    bool _signal_import_active = false;
    int64_t _active_import_count = 0;
    int64_t _last_imported_count = 0;
    bool _owned_import_active = false;
    bool _last_owned_import_success = false;
    uint64_t _owned_import_started_ms = 0;
    uint64_t _last_owned_import_duration_ms = 0;
    godot::String _owned_import_asset_path;
};

} // namespace fennara
