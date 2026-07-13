# Repo Map

This is the quick map for contributors and coding agents working in this repository.

## Find The Right Area

| Change | Primary Location |
| --- | --- |
| User setup or CLI behavior | `local/crates/fennara-cli/` |
| External MCP protocol or schemas | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Built-in chat or daemon behavior | `local/crates/fennara-daemon/` |
| Godot editor integration | `fennara-cpp/` |
| Chat UI | `ui/chat/` |
| Runtime helper scripts | `runtime/` |
| Packaging or releases | `scripts/`, `.github/workflows/` |
| User documentation | `README.md`, `docs/` |

## Top Level

| Path | Owns |
| --- | --- |
| `.github/` | Pull request template, issue templates, and GitHub Actions workflows. |
| `docs/` | Project docs, setup guides, architecture notes, examples, demos, and release notes. |
| `fennara-cpp/` | C++ Godot GDExtension source and SCons build entrypoint. |
| `godot_demo/addons/fennara/` | Installable Godot addon payload copied into user projects. |
| `local/` | Rust CLI, MCP server, daemon, schemas, and local runtime code. |
| `media/` | Images and public media used by docs. |
| `runtime/` | Source Godot runtime helper scripts used by `runtime_session` and `runtime_script`. |
| `scripts/` | Versioning, packaging, and release helper scripts. |
| `ui/chat/` | Source for the optional in-editor web chat UI. |
| `local/templates/` | Markdown templates written into Godot projects by `fennara install` and refreshed by `fennara update`. |
| `local/webview-runtimes/` | Manifest/config files for external webview runtimes installed into shared Fennara app data, such as the Linux CEF payload. |
| `install.ps1` / `install.sh` | Bootstrap scripts that install the Fennara CLI from GitHub releases. |
| `VERSION` | Version source of truth. |
| `README.md` | Short human-facing overview and quick start. |
| `docs/README.md` | Task-oriented documentation index. |
| `docs/setup.md` | User-facing addon-first setup, chat prerequisites, MCP connection, update flow, and troubleshooting. |
| `docs/cli.md` | Terminal command reference, CLI-owned install/update behavior, recovery, diagnostics, app-data layout, and automation guidance. |
| `CONTRIBUTING.md` | Contribution rules. |
| `SECURITY.md` | Security reporting policy. |
| `LICENSE.md` | Project license. |

## Local Rust Packages

| Path | Owns |
| --- | --- |
| `local/crates/fennara-cli/` | `fennara` command: install, update, CLI self-update, doctor, operation diagnostics, webview prerequisite checks, C# support, MCP app setup, and generated project guidance. |
| `local/crates/fennara-cli/src/operation.rs` | Public install/update operation coordinator, phases, and CLI handoff entry points. |
| `local/crates/fennara-cli/src/operation/` | Focused operation journal, durable storage, diagnostic redaction, and test modules. |
| `local/crates/fennara-cli/src/project_addon.rs` | Existing project-addon version and current-platform GDExtension library validation. |
| `local/crates/fennara-cli/src/release_identity.rs` | Stable/staging addon identity, exact release selectors, pull-request channel validation, and legacy stable compatibility. |
| `local/crates/fennara-cli/src/release_channel.rs` | Per-channel staging pointer validation and resolution to an immutable exact release. |
| `local/crates/fennara-cli/src/release_version.rs` | Shared CLI SemVer parsing and precedence used by manifests and release selection. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Exact-version adoption of an existing Asset Library or release addon without replacing project addon files. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Shared daemon health check, exact-version readiness, and startup used by install and doctor. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Process-level failure, durable diagnostics, redaction, and fail-closed operation-log tests. |
| `local/crates/fennara-cli/src/diagnostics.rs` | User-facing access to the latest or a named sanitized operation report. |
| `local/crates/fennara-mcp/` | Local stdio MCP server and tool schema forwarding. |
| `local/crates/fennara-daemon/` | Local daemon used for runtime sessions and Godot bridge work. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Built-in chat approval modes, tool-risk classification, permission decisions, and pending approval request types. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Daemon-owned built-in chat `exec_command` implementation: shell detection, cwd validation, process spawn, timeout/tree-kill, output capture, result artifact logging, and result formatting. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Built-in chat context compaction planner: exact-tail protection, OpenCode-style old tool-result pressure pruning, summary chunk selection/storage/replay, summary prompt serialization, token budgets, and placeholder rendering. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | Built-in chat PromptBuilder and generated runtime environment context. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Local-only built-in chat trace recorder, SQLite event rows, retention, and debug query helpers. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | Built-in chat provider runtime primitives, catalog/resolution, context preflight hooks, normalized stream/error types, and OpenAI-compatible or Anthropic-compatible adapters for OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, Ollama/local, and LM Studio. |
| `local/schemas/tools/` | Shared tool JSON schemas. The external MCP server and built-in chat embed their own allowed subsets. |
| `local/webview-runtimes/linux-cef.json` | Linux CEF runtime placeholder/generated manifest used for release manifest generation, doctor output, and legacy fallback. It records the shared app-data layout and archive metadata without placing CEF inside the addon zip. |
| `local/Cargo.toml` | Rust workspace config. |
| `local/Cargo.lock` | Locked Rust dependency graph. |

## GDExtension Source

| Path | Owns |
| --- | --- |
| `fennara-cpp/SConstruct` | GDExtension build entrypoint. |
| `fennara-cpp/include/` | Public C++ headers. |
| `fennara-cpp/src/` | C++ implementation. |
| `fennara-cpp/src/setup/` | Native first-run setup state, release-manifest CLI bootstrap, hash verification, CLI launch, and durable operation progress reader. |
| `fennara-cpp/src/release/version.cpp` | Native SemVer validation and precedence used by release/update discovery. |
| `fennara-cpp/src/ui/setup_panel.cpp` | Webview-independent first-run setup panel with progress, retry, logs, and sanitized report actions. |
| `fennara-cpp/vendor/cef/` | Official CEF 139 header snapshot used by the Linux OSR bridge. Runtime binaries stay outside the addon. |
| `fennara-cpp/src/ui/webview_host*` | Native in-editor chat webview host and platform backends. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Linux-only shared CEF runtime discovery, marker validation, and dynamic `libcef.so` loader foundation. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Linux-only CEF off-screen rendering surface, Godot input forwarding, bridge ABI loading, and Godot texture updates for the internal chat webview. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Small Linux-only bridge library built from the pinned official CEF 139 `libcef_dll_wrapper` source and Fennara's CEF OSR adapter. The main GDExtension dlopens this after the external `libcef.so` runtime is loaded. |
| `fennara-cpp/src/tools/` | Godot-facing tool implementations. |
| `fennara-cpp/src/lsp/` | Script diagnostics and language-server helpers. |
| `fennara-cpp/src/csharp/` | Build-only C# project selection, background preparation, isolated diagnostics, and runtime preflight. |
| `fennara-cpp/src/runtime/` | Native runtime support used by tools, including runtime scene preflight, script diagnostics, and debugger snapshots. |
| `fennara-cpp/godot-cpp/` | Godot C++ bindings submodule. |

## Addon Payload

| Path | Owns |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Godot GDExtension registration file. |
| `godot_demo/addons/fennara/VERSION` | Addon package version. |
| `godot_demo/addons/fennara/release.json` | Packaged stable or staging identity, including exact version, release tag, channel, and staging source commit. |
| `godot_demo/addons/fennara/bin/` | Built platform libraries. |
| `godot_demo/addons/fennara/dist/` | Packaged web UI assets used by the in-editor chat webview. |
| `godot_demo/addons/fennara/runtime/` | Synced packaged copy of `runtime/` shipped inside the addon. |
| `godot_demo/tests/first_run_setup_test.gd` | Headless native first-run setup state and deterministic failure test. |

## Runtime Helper Source

| Path | Owns |
| --- | --- |
| `runtime/game_capture_helper.gd` | Runtime helper entrypoint loaded by the GDExtension for scene sessions and runtime checks. |
| `runtime/runtime_script_context.gd` | Public `ctx` helper surface exposed to `runtime_script`, including wait/capture/action/input/snapshot/until/raycast/click helpers. |
| `runtime/runtime_input_driver.gd` | Low-level runtime input event driver for keys, mouse buttons, absolute mouse motion, relative mouse motion, modifiers, and input cleanup. |
| `runtime/runtime_node_snapshot.gd` | Runtime node lookup, existence checks, stale-reference-safe snapshots, property reads, and child summaries. |
| `runtime/runtime_physics_query.gd` | Runtime 2D/3D exact raycast and scan helpers with compact hit receipts. |
| `runtime/runtime_query_utils.gd` | Shared runtime query utilities for vector coercion, safe node/path resolution, object identity, and generic target matching. |
| `runtime/runtime_capture_store.gd` | Runtime capture/status artifact writer used by runtime sessions, scripts, and environment checks. |
| `runtime/runtime_check_runner.gd` | Runtime check runner for non-interactive scene execution specs. |

## Scripts And Workflows

| Path | Owns |
| --- | --- |
| `scripts/set-version.mjs` | Updates versioned files across the repo. |
| `scripts/check-version.mjs` | Checks version sync. |
| `scripts/release-identity.mjs` | Shared Node validation and generation for SemVer release identity and per-PR staging pointers. |
| `scripts/sync-chat-ui.mjs` | Copies the buildless chat UI source into the addon payload. |
| `scripts/sync-runtime.mjs` | Copies repo-root runtime helper source into the addon payload. |
| `scripts/package-preview.mjs` | Assembles addon, CLI, and local runtime preview/release zips after platform builds. |
| `scripts/prepare-linux-cef-runtime.mjs` | Stages the separate Linux x64 CEF runtime zip, strips staged ELF binaries, validates required files, and can write the generated release manifest. |
| `scripts/prepare-linux-cef-sdk.mjs` | Downloads and extracts the pinned official CEF 139 Linux minimal SDK for CI builds that need `libcef_dll/` wrapper source. |
| `scripts/check-linux-cef-runtime-release.mjs` | Validates the Linux CEF runtime release asset against the generated `local/webview-runtimes/linux-cef.json` manifest. |
| `scripts/write-release-manifest.mjs` | Writes and validates `fennara-release-manifest-v<version>.json` from release assets, including local package, addon, and shared runtime hashes. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Minimal Linux CEF subprocess helper source packaged inside the separate CEF runtime zip. |
| `.github/workflows/version-check.yml` | Version consistency check. |
| `.github/workflows/gdextension-build.yml` | Cross-platform GDExtension build check plus the Windows headless native first-run setup state test. |
| `.github/workflows/local-build.yml` | Rust local package build check. |
| `.github/workflows/package-preview.yml` | Manual package preview artifacts, including a test-only Linux CEF runtime artifact for Linux chat smoke tests. |
| `.github/workflows/release.yml` | Manual GitHub release publishing, including generated Linux CEF runtime packaging, release manifest generation, and final asset validation. |

## Where To Change Things

| Task | Start here |
| --- | --- |
| Add or change a Godot tool | `fennara-cpp/src/tools/` and `local/schemas/tools/` |
| Change MCP schema text | `local/schemas/tools/` |
| Change `fennara install` or `fennara update` | `local/crates/fennara-cli/src/`; native staging and detached apply/rollback are owned by `release_update.rs`, `update_stage.rs`, `update_stage/`, and `update_apply/` |
| Change CLI commands or terminal behavior | `local/crates/fennara-cli/src/` and `docs/cli.md` |
| Change native update progress, shutdown confirmation, activation handshake, or recovery | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs`, and `ui/chat/` |
| Change native first-run setup or CLI bootstrap | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp`, and `fennara-cpp/src/ui/dock.cpp` |
| Change install/update operation logs, phases, error codes, or diagnostic reports | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/`, and `local/crates/fennara-cli/src/diagnostics.rs` |
| Change webview prerequisite checks | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs`, and `fennara-cpp/src/ui/webview_host*` |
| Change generated project guidance | `local/templates/` and `local/crates/fennara-cli/src/project_guidance.rs` |
| Sync generated demo addon guidance | `local/templates/fennara-guidelines.md`, `scripts/sync-guidance.mjs`, and `godot_demo/addons/fennara/ai/guidelines.md` |
| Change MCP app setup | `local/crates/fennara-cli/src/mcp_setup.rs` and `docs/mcp-setup.md` |
| Change runtime session process/log behavior | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/`, and `fennara-cpp/src/tool_results/` |
| Change `runtime_script` ctx helpers, input, snapshots, waits, raycasts, captures, or cleanup | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json`, and `docs/tools.md` |
| Change in-editor chat UI, slash commands, or model/provider picker | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp`, and `fennara-cpp/src/ui/webview_host*` |
| Change built-in chat providers | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, and `ui/chat/` |
| Change vendored chat UI libraries | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/`, and `THIRD_PARTY_NOTICES.md` |
| Change C# support | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/`, and the C# tool schemas and guidance |
| Change release packages or CLI self-update | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/write-release-manifest.mjs`, and `.github/workflows/release.yml` |
| Bump version | `node scripts/set-version.mjs <version>` |
| Update setup/docs for chat vs MCP, providers, or slash commands | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md`, and `llms.txt` |

## Notes

- Keep this file current when adding or moving major source areas.
- Keep release steps in [release.md](release.md).
- Keep setup steps in [setup.md](setup.md).
- Keep terminal command behavior in [cli.md](cli.md).
