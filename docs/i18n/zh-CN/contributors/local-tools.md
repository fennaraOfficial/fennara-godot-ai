<!-- fennara-i18n: locale=zh-CN source=local/README.md sha256=29a4563cb548ac4612f1881d66af9e72f4de9b1c118920e0d14ba00d0279edec -->
<a id="fennara-local-tools"></a>
# Fennara 本地工具

<!-- fennara-doc-nav:start -->
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
- `POST /status/bound`：特权绑定状态。将一个 MCP 进程的规范项目根目录与已连接 Godot 编辑器会话进行解析匹配。
- `POST /tools/call`：把工具调用转发给已连接的 Godot 插件并等待结果。
- `WS /godot/ws`：本地 Godot 插件桥接。插件连接后会发送 `hello` 消息。

当前用户的所有启用 Fennara 的编辑器和外部 MCP 进程都共享一个守护进程。已绑定的外部请求按规范项目根目录路由；内部的内置聊天请求仍绑定到其 Godot 编辑器会话，
而旧式未绑定 MCP 请求使用停靠面板选定的兼容性目标。

守护进程还拥有一个整台机器范围的运行时槽位。运行时会话的所有权和可续期租约状态与项目根目录关联，因此编辑器可以重新连接而不会转移控制权。

开发二进制文件：

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP 服务器

`crates/fennara-mcp` 是本地 MCP 服务器。它通过 stdio 使用 JSON-RPC，让 MCP 客户端能够把它作为本地进程启动。

每个 MCP 进程启动时都会固定一个可选的项目绑定。选择优先级为 `--project-path`、`FENNARA_PROJECT_PATH`，然后是启动目录中包含 `project.godot` 的最近祖先目录。未找到项目时会自动进入旧式未绑定兼容模式；
无效的显式路径会导致启动失败。如需跨项目隔离，请为每个项目使用一个 MCP 进程和连接。

`crates/fennara-project-identity` 由 MCP 运行时和守护进程共享。它负责项目根目录的发现、验证、规范化、无损协议转换和实时文件系统相等性。

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

- `fennara_status`：确认 MCP 服务器已安装且可访问，然后在守护进程运行时报告路由模式、绑定来源或根目录、所选编辑器状态和 Godot 桥接就绪情况。
- `write_or_update_file`、`run_scene_edit_script`、`get_scene_tree`、`script_diagnostics` 和 `screenshot_scene` 等 Godot 项目工具会转发给守护进程，再由守护进程转发给已连接的 Godot 插件。

后续在 Windows 上安装到的用户路径：

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
