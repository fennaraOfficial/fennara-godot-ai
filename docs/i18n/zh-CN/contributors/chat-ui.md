<!-- fennara-i18n: locale=zh-CN source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Fennara 聊天 UI

<!-- fennara-doc-nav:start -->
[English](../../../../ui/chat/README.md) · **简体中文** · [Español](../../es/contributors/chat-ui.md) · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · [日本語](../../ja/contributors/chat-ui.md) · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · [Deutsch](../../de/contributors/chat-ui.md) · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

此文件夹包含可选编辑器内聊天界面的源代码。

第一版有意不使用构建流程，只使用普通 HTML、CSS 和 JavaScript。这样可让开源仓库便于检查，也避免在 WebView 宿主和守护进程聊天桥接尚未稳定前引入前端工具链。

打包后的副本位于 `godot_demo/addons/fennara/dist/`。

编辑此文件夹后运行：

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## 设计说明

- 与 Godot 编辑器界面保持一致：紧凑控件、柔和对比、小圆角、清晰的焦点状态，不使用营销式首屏。
- 只使用本地 Fennara 守护进程和聊天 API，不依赖托管服务。
- OpenRouter 支持应使用由用户提供、存储在 Godot 项目外本地位置的密钥。
- 即使没有连接模型，UI 也应有用，状态、设置、对话记录和编辑框状态仍应可见。
