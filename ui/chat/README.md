# Fennara Chat UI

<!-- fennara-doc-nav:start -->
[Documentation](../../docs/README.md)

🌐 **Languages**

**English** · [简体中文](../../docs/i18n/zh-CN/contributors/chat-ui.md) · [Español](../../docs/i18n/es/contributors/chat-ui.md) · [Português do Brasil](../../docs/i18n/pt-BR/contributors/chat-ui.md) · [日本語](../../docs/i18n/ja/contributors/chat-ui.md) · [한국어](../../docs/i18n/ko/contributors/chat-ui.md) · [Русский](../../docs/i18n/ru/contributors/chat-ui.md) · [Français](../../docs/i18n/fr/contributors/chat-ui.md) · [Deutsch](../../docs/i18n/de/contributors/chat-ui.md) · [Türkçe](../../docs/i18n/tr/contributors/chat-ui.md)
<!-- fennara-doc-nav:end -->

This folder contains the source for the optional in-editor chat surface.

The first version is buildless on purpose: plain HTML, CSS, and JavaScript.
That keeps the OSS repo easy to inspect and avoids adding a frontend toolchain
before the webview host and daemon chat bridge are settled.

The packaged copy lives in `godot_demo/addons/fennara/dist/`.

After editing this folder, run:

```bash
node scripts/sync-chat-ui.mjs
```

## Design Notes

- Match Godot editor surfaces: compact controls, quiet contrast, small radii,
  clear focus states, and no marketing-style hero treatment.
- Use only local Fennara daemon/chat APIs; do not require hosted services.
- OpenRouter support should use a user-provided key stored locally outside the
  Godot project.
- Keep the UI useful without a model connection: status, settings, transcript,
  and composer states should still be visible.
