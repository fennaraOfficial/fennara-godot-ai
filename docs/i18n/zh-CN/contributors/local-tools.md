<!-- fennara-i18n: locale=zh-CN source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
<a id="fennara-local-tools"></a>
# Fennara 本地工具

<!-- fennara-doc-nav:start -->
[文档](../README.md)

🌐 **语言**

[English](../../../../local/README.md) · **简体中文** · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

此文件夹包含 Fennara 的本地原生组件。

<a id="daemon"></a>
## 守护进程

`crates/fennara-daemon` 在以下地址运行本地 Fennara 守护进程：

```text
http://127.0.0.1:41287
```

端点：

- `GET /health`：守护进程健康状态。
- `GET /status`：守护进程状态和已连接 Godot 插件的元数据。
- `POST /tools/call`：把工具调用转发给已连接的 Godot 插件并等待结果。
- `WS /godot/ws`：本地 Godot 插件桥接。插件连接后会发送 `hello` 消息。

开发二进制文件：

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP 服务器

`crates/fennara-mcp` 是本地 MCP 服务器。它通过 stdio 使用 JSON-RPC，让 MCP 客户端能够把它作为本地进程启动。

`fennara-mcp` 在构建时嵌入从 `local/schemas/tools/` 中选出的面向 MCP 的 schema，并将工具调用转发给本地守护进程。运行时无需外部 schema 服务。内置聊天会从同一 schema 目录选择一组相关但不同的工具。

`fennara install` 还会把 `local/templates/` 中生成的项目指南写入 Godot 项目：

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

构建：

```powershell
cd local
cargo build
```

在 Windows 上，如果终端尚未刷新 Rust PATH：

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

开发二进制文件：

```text
local/target/debug/fennara-mcp.exe
```

当前工具：

- `fennara_status`：确认 MCP 服务器已安装且可访问，然后在守护进程运行时报告守护进程和 Godot 桥接状态。
- `write_or_update_file`、`run_scene_edit_script`、`get_scene_tree`、`script_diagnostics` 和 `screenshot_scene` 等 Godot 项目工具会转发给守护进程，再由守护进程转发给已连接的 Godot 插件。

后续在 Windows 上安装到的用户路径：

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
