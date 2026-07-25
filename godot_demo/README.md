# Godot Payload

<!-- fennara-doc-nav:start -->
**English** · [简体中文](../docs/i18n/zh-CN/contributors/godot-payload.md) · [Español](../docs/i18n/es/contributors/godot-payload.md) · [Português do Brasil](../docs/i18n/pt-BR/contributors/godot-payload.md) · [日本語](../docs/i18n/ja/contributors/godot-payload.md) · [한국어](../docs/i18n/ko/contributors/godot-payload.md) · [Русский](../docs/i18n/ru/contributors/godot-payload.md) · [Français](../docs/i18n/fr/contributors/godot-payload.md) · [Deutsch](../docs/i18n/de/contributors/godot-payload.md) · [Türkçe](../docs/i18n/tr/contributors/godot-payload.md)
<!-- fennara-doc-nav:end -->

This directory is the source tree for the Godot-facing addon payload that is copied into user projects and packaged into release archives.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` must stay installable as a normal Godot addon directory. Anything committed here should be something a user project can receive directly under `res://addons/fennara/`.

## What Belongs Here

- `addons/fennara/fennara.gdextension` and `.uid` files that Godot loads.
- `addons/fennara/bin/` editor GDExtension binaries produced by platform builds.
- `addons/fennara/dist/` generated web chat assets used by the native chat webview.
- `addons/fennara/runtime/` synced Godot-side runtime helper scripts from `runtime/`.
- `addons/fennara/VERSION`, matching the repo `VERSION` during packaging.

## What Does Not Belong Here

- Local Godot user state such as `.godot/`, `.import/`, logs, temp files, or editor caches.
- Root package outputs from workflows. Those belong under ignored build folders such as `dist/` or `.package-preview/`.
- Shared local runtime payloads such as the Fennara daemon/MCP executables or Linux CEF runtime. Those are installed under the user's Fennara app-data directory by the CLI, not copied into every Godot project addon.

## Generated Files

The chat UI source lives under `ui/chat/`. After changing it, run:

```powershell
node scripts\sync-chat-ui.mjs
```

That syncs the built webview files into `godot_demo/addons/fennara/dist/`, which is intentionally committed because addon users should not need Node.js or a frontend build step.

The runtime helper source lives under `runtime/`. After changing it, run:

```powershell
node scripts\sync-runtime.mjs
```

That syncs the Godot-side runtime helpers into `godot_demo/addons/fennara/runtime/`, which is intentionally committed because addon users should receive those scripts with the release zip.
