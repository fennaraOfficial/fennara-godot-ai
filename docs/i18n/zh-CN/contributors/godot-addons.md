<!-- fennara-i18n: locale=zh-CN source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Godot 插件

<!-- fennara-doc-nav:start -->
[文档](../README.md)

🌐 **语言**

[English](../../../../godot_demo/addons/README.md) · **简体中文** · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

此目录与 Godot 在项目中要求的结构一致：

```text
res://addons/
  fennara/
```

把仓库负载放在 `godot_demo/addons/` 下，可以让打包和本地测试脚本无需重组路径就把插件复制到项目。

<a id="current-addon"></a>
## 当前插件

`fennara/` 是可安装的 Fennara Godot AI 插件，包含：

- `fennara.gdextension`，原生扩展的 Godot 入口。
- `bin/`，从 `fennara-cpp/` 构建的平台编辑器二进制文件。
- `dist/`，从 `ui/chat/` 同步的原生聊天 WebView 生成资源。
- `runtime/`，从仓库根目录 `runtime/` 源代码同步的 Godot 端辅助脚本。
- `debugger/`，面向调试器的插件资源。
- `VERSION`，打包后的插件版本标记。

<a id="rules"></a>
## 规则

- 保持插件相对路径稳定。用户项目会以 `res://addons/fennara/` 接收此文件夹。
- 不要把包预览 ZIP、发布 ZIP、下载的 CEF 归档、日志或本地测试输出放在这里。
- 不要手动编辑 `fennara/dist/` 中生成的 WebView 文件，除非也同步修改源文件。
- 不要在未更新 `runtime/` 并运行 `node scripts/sync-runtime.mjs` 的情况下手动编辑 `fennara/runtime/` 中同步的运行时辅助文件。
- 只有需要复制进 Godot 项目的新插件负载才能放在这里。
