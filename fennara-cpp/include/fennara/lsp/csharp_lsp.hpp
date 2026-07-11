#pragma once

#include <godot_cpp/variant/array.hpp>
#include <godot_cpp/variant/dictionary.hpp>
#include <godot_cpp/variant/string.hpp>

#include <atomic>

namespace fennara::csharp_lsp {

void begin_session_lifecycle();
void reserve_background_preparation();
void cancel_reserved_background_preparation();

godot::Dictionary warmup(const godot::String &client_name);

void warmup_async(const godot::String &lsp_path,
                  const godot::String &project_path,
                  const godot::String &project_root,
                  const godot::String &client_name);

bool wait_for_background_preparation(
    const godot::String &activity,
    const std::atomic_bool *cancelled = nullptr);
bool background_preparation_in_progress();

godot::Dictionary diagnose_files(const godot::Array &files,
                                 const godot::String &client_name,
                                 const std::atomic_bool *cancelled = nullptr);

godot::Dictionary document_symbols(const godot::Array &files,
                                   const godot::String &client_name);

void shutdown_warm_server();

} // namespace fennara::csharp_lsp
