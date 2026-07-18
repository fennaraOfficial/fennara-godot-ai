#pragma once

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/ref_counted.hpp>
#include <godot_cpp/classes/resource.hpp>
#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>
#include <godot_cpp/variant/variant.hpp>

namespace fennara {

class FennaraRunAssetImportScriptContext : public godot::RefCounted {
    GDCLASS(FennaraRunAssetImportScriptContext, godot::RefCounted)

protected:
    static void _bind_methods();

public:
    void configure(const godot::String &asset_path,
                   const godot::String &importer,
                   const godot::Dictionary &options,
                   const godot::Array &generated_files,
                   const godot::Array &dependencies,
                   const godot::Ref<godot::Resource> &imported_resource,
                   bool import_valid,
                   bool read_only);
    void cleanup();

    godot::String get_asset_path() const;
    godot::String get_mode() const;
    bool is_read_only() const;
    godot::Dictionary get_import_info() const;
    bool has_import_option(const godot::String &name) const;
    godot::Variant get_import_option(const godot::String &name) const;
    godot::Array list_import_options(const godot::String &prefix = "") const;
    bool set_import_option(const godot::String &name,
                           const godot::Variant &value);
    godot::Array get_staged_changes() const;
    void discard_import_option_change(const godot::String &name);

    godot::Ref<godot::Resource> get_imported_resource() const;
    godot::Node *instantiate_imported_scene();
    godot::Array get_generated_files() const;
    godot::Array get_dependencies() const;
    godot::Dictionary get_subresource_summary();

    void log(const godot::Variant &value);
    void error(const godot::String &message);
    void require(bool condition, const godot::String &message);
    godot::Array get_logs() const;
    godot::Array get_edit_errors() const;

#ifdef FENNARA_SETUP_TEST_HOOKS
    void configure_for_test(const godot::String &importer,
                            const godot::Dictionary &options,
                            const godot::Ref<godot::Resource> &imported_resource,
                            bool read_only);
#endif

private:
    bool _option_is_editable(const godot::String &name,
                             godot::String *reason = nullptr) const;
    void _add_error(const godot::String &message,
                    const godot::String &source = "context");
    godot::Node *_ensure_temporary_host();

    godot::String _asset_path;
    godot::String _importer;
    godot::Dictionary _options;
    godot::Dictionary _staged_values;
    godot::Array _generated_files;
    godot::Array _dependencies;
    godot::Ref<godot::Resource> _imported_resource;
    bool _import_valid = false;
    bool _read_only = true;
    bool _failed = false;
    godot::Array _logs;
    godot::Array _errors;
    godot::Node *_temporary_host = nullptr;
};

class FennaraRunAssetImportScriptTool : public godot::RefCounted {
    GDCLASS(FennaraRunAssetImportScriptTool, godot::RefCounted)

protected:
    static void _bind_methods();

public:
    static godot::Dictionary execute(const godot::Dictionary &args);
    static godot::Dictionary prepare_execution(const godot::Dictionary &args);
    static godot::Dictionary execute_prepared(
        const godot::Dictionary &prepared_args);
    static void finalize_result(godot::Dictionary &result);
#ifdef FENNARA_SETUP_TEST_HOOKS
    static godot::Dictionary apply_reimport_result_for_test(
        const godot::Dictionary &import_result,
        int change_count);
    static godot::Dictionary verify_generated_outputs_for_test(
        const godot::Variant &dest_files);
#endif
};

} // namespace fennara
