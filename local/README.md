# Fennara Local Tools

<!-- fennara-doc-nav:start -->
**English** · [简体中文](../docs/i18n/zh-CN/contributors/local-tools.md) · [Español](../docs/i18n/es/contributors/local-tools.md) · [Português do Brasil](../docs/i18n/pt-BR/contributors/local-tools.md) · [日本語](../docs/i18n/ja/contributors/local-tools.md) · [한국어](../docs/i18n/ko/contributors/local-tools.md) · [Русский](../docs/i18n/ru/contributors/local-tools.md) · [Français](../docs/i18n/fr/contributors/local-tools.md) · [Deutsch](../docs/i18n/de/contributors/local-tools.md) · [Türkçe](../docs/i18n/tr/contributors/local-tools.md)
<!-- fennara-doc-nav:end -->

This folder contains local-native Fennara components.

## Daemon

`crates/fennara-daemon` runs the local Fennara daemon on:

```text
http://127.0.0.1:41287
```

Endpoints:

- `GET /health`: daemon health.
- `GET /status`: daemon status plus connected Godot plugin metadata.
- `POST /status/bound`: privileged bound status. Resolves one MCP process's
  canonical Project Root against connected Godot editor sessions.
- `POST /tools/call`: forwards a tool call to the connected Godot plugin and waits for a tool result.
- `WS /godot/ws`: local Godot plugin bridge. The plugin sends a `hello` message after connecting.

One daemon is shared by every Fennara-enabled editor and external MCP process
for the current user. Bound external requests route by canonical Project Root;
internal built-in-chat requests remain bound to their Godot Editor Session, and
legacy-unbound MCP requests use the dock-selected compatibility target.

The daemon also owns one machine-wide Runtime Slot. Runtime Session ownership
and renewable lease state are associated with a Project Root so an editor can
reconnect without transferring control.

Development binary:

```text
local/target/debug/fennara-daemon.exe
```

## MCP Server

`crates/fennara-mcp` is the local MCP server. It speaks JSON-RPC over stdio so MCP clients can launch it as a local process.

Each MCP process freezes one optional Project Binding at startup. Selection is
`--project-path`, then `FENNARA_PROJECT_PATH`, then nearest `project.godot`
ancestor of the startup directory. Finding no project automatically enters
legacy-unbound compatibility mode; an invalid explicit path fails startup. Use
one MCP process and connection per project for cross-project isolation.

`crates/fennara-project-identity` is shared by the MCP runtime and daemon. It
owns Project Root discovery, validation, canonicalization, lossless protocol
conversion, and live filesystem equality.

`fennara-mcp` embeds its selected MCP-facing schemas from `local/schemas/tools/`
at build time and forwards those tool calls to the local daemon. It does not
need an external schema service at runtime. The built-in chat selects a related
but different tool set from the same schema directory.

`fennara install` also writes generated project guidance from `local/templates/`
into the Godot project:

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

Build:

```powershell
cd local
cargo build
```

On Windows, if a terminal has not refreshed the Rust PATH yet:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Development binary:

```text
local/target/debug/fennara-mcp.exe
```

Current tools:

- `fennara_status`: verifies that the MCP server is installed and reachable,
  then reports routing mode, binding source/root, selected editor state, and
  Godot bridge readiness when the daemon is running.
- Godot project tools such as `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics`, and `screenshot_scene` are forwarded
  to the daemon, which forwards them to the connected Godot plugin.

Later installed user path on Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
