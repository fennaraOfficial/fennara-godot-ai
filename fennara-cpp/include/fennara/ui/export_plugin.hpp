#pragma once

#include <godot_cpp/classes/editor_export_platform.hpp>
#include <godot_cpp/classes/editor_export_plugin.hpp>
#include <godot_cpp/variant/packed_string_array.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

#include <cstdint>

namespace fennara {

class FennaraExportPlugin : public godot::EditorExportPlugin {
    GDCLASS(FennaraExportPlugin, godot::EditorExportPlugin)

protected:
    static void _bind_methods();

private:
    bool export_active = false;
    bool had_runtime_autoload = false;
    godot::Variant saved_runtime_autoload;
    bool extension_list_changed = false;
    godot::String extension_list_path;
    godot::PackedByteArray saved_extension_list;

    static bool _should_skip_path(const godot::String &path);
    static godot::String _filter_extension_list(
        const godot::String &contents);
    void _resolve_extension_list_path();
    bool _replace_extension_list(
        const godot::PackedByteArray &contents);
    bool _restore_extension_list_backup();
    void _suppress_extension_list_entry();
    void _restore_export_state();

public:
    FennaraExportPlugin();
    ~FennaraExportPlugin();

    godot::String _get_name() const override;
    bool _supports_platform(
        const godot::Ref<godot::EditorExportPlatform> &platform) const override;
    void _export_begin(const godot::PackedStringArray &features,
                       bool is_debug,
                       const godot::String &path,
                       uint32_t flags) override;
    void _export_file(const godot::String &path,
                      const godot::String &type,
                      const godot::PackedStringArray &features) override;
    void _export_end() override;

#ifdef FENNARA_SETUP_TEST_HOOKS
    bool test_should_skip_path(const godot::String &path) const;
    godot::String test_filter_extension_list(
        const godot::String &contents) const;
    void test_suppress_extension_list_entry();
    void test_export_begin();
    void test_export_end();
#endif
};

} // namespace fennara
