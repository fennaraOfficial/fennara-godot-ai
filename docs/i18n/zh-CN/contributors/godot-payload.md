<!-- fennara-i18n: locale=zh-CN source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Godot 负载

<!-- fennara-doc-nav:start -->
[文档](../README.md)

🌐 **语言**

[English](../../../../godot_demo/README.md) · **简体中文** · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

此目录是面向 Godot 的插件负载源代码树。它会被复制到用户项目并打包进发布归档。

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` 必须始终可以作为普通 Godot 插件目录安装。提交到这里的内容应当能够直接放进用户项目的 `res://addons/fennara/`。

<a id="what-belongs-here"></a>
## 这里应包含什么

- Godot 加载的 `addons/fennara/fennara.gdextension` 和 `.uid` 文件。
- 平台构建生成的 `addons/fennara/bin/` 编辑器 GDExtension 二进制文件。
- 原生聊天 WebView 使用的 `addons/fennara/dist/` 生成版 Web UI 资源。
- 从 `runtime/` 同步的 `addons/fennara/runtime/` Godot 端运行时辅助脚本。
- 与打包时仓库 `VERSION` 一致的 `addons/fennara/VERSION`。

<a id="what-does-not-belong-here"></a>
## 这里不应包含什么

- `.godot/`、`.import/`、日志、临时文件或编辑器缓存等本地 Godot 用户状态。
- 工作流产生的根包输出。它们应位于 `dist/` 或 `.package-preview/` 等忽略的构建目录。
- Fennara 守护进程和 MCP 可执行文件或 Linux CEF 运行时等共享本地运行时负载。CLI 会把它们安装到用户的 Fennara 应用数据目录，不会复制进每个 Godot 项目插件。

<a id="generated-files"></a>
## 生成文件

聊天 UI 源代码位于 `ui/chat/`。更改后运行：

```powershell
node scripts\sync-chat-ui.mjs
```

这会把构建后的 WebView 文件同步到 `godot_demo/addons/fennara/dist/`。该目录有意提交到仓库，因为插件用户不应需要 Node.js 或前端构建步骤。

运行时辅助源代码位于 `runtime/`。更改后运行：

```powershell
node scripts\sync-runtime.mjs
```

这会把 Godot 端运行时辅助脚本同步到 `godot_demo/addons/fennara/runtime/`。该目录有意提交到仓库，以便插件用户通过发布 ZIP 获得这些脚本。
