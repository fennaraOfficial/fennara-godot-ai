# Godot Addons

<!-- fennara-doc-nav:start -->
[Documentation](../../docs/README.md)

🌐 **Languages**

**English** · [简体中文](../../docs/i18n/zh-CN/contributors/godot-addons.md) · [Español](../../docs/i18n/es/contributors/godot-addons.md) · [Português do Brasil](../../docs/i18n/pt-BR/contributors/godot-addons.md) · [日本語](../../docs/i18n/ja/contributors/godot-addons.md) · [한국어](../../docs/i18n/ko/contributors/godot-addons.md) · [Русский](../../docs/i18n/ru/contributors/godot-addons.md) · [Français](../../docs/i18n/fr/contributors/godot-addons.md) · [Deutsch](../../docs/i18n/de/contributors/godot-addons.md) · [Türkçe](../../docs/i18n/tr/contributors/godot-addons.md)
<!-- fennara-doc-nav:end -->

This directory mirrors the shape Godot expects inside a project:

```text
res://addons/
  fennara/
```

Keeping the repository payload under `godot_demo/addons/` lets packaging and local test scripts copy the addon into a project without reshaping paths.

## Current Addon

`fennara/` is the installable Fennara Godot AI addon. It contains:

- `fennara.gdextension`, the Godot entry point for the native extension.
- `bin/`, platform editor binaries built from `fennara-cpp/`.
- `dist/`, generated native chat webview assets synced from `ui/chat/`.
- `runtime/`, synced Godot-side helper scripts from the repo-root `runtime/` source.
- `debugger/`, debugger-facing addon assets.
- `VERSION`, the packaged addon version marker.

## Rules

- Keep addon-relative paths stable. User projects receive this folder as `res://addons/fennara/`.
- Do not put package-preview zips, release zips, downloaded CEF archives, logs, or local test output here.
- Do not hand-edit generated webview files in `fennara/dist/` unless you are intentionally patching generated output and then syncing the source change too.
- Do not hand-edit synced runtime helper files in `fennara/runtime/` without also updating `runtime/` and running `node scripts/sync-runtime.mjs`.
- Add new addon payloads here only if they are meant to be copied into Godot projects.
