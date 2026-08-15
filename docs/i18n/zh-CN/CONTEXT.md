<!-- fennara-i18n: locale=zh-CN source=CONTEXT.md sha256=7d76acbada75ade69b43dc52fcd543f90d678c04b3e9b50fc11601b8b1853fd4 -->
<a id="fennara-context"></a>
# Fennara 术语

<!-- fennara-doc-nav:start -->
[English](../../../CONTEXT.md) · **简体中文** · [Español](../es/CONTEXT.md) · [Português do Brasil](../pt-BR/CONTEXT.md) · [日本語](../ja/CONTEXT.md) · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · [Deutsch](../de/CONTEXT.md) · [Türkçe](../tr/CONTEXT.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

本文定义 Fennara 文档、issue、发布说明和面向智能体的指南中使用的常见术语。

<a id="product-terms"></a>
## 产品术语

**Fennara**

本仓库中的 Godot 感知智能体环境。Fennara 将 AI 工具连接到真实的 Godot 反馈，例如诊断、场景验证、运行时错误、截图和项目指南。

**Godot 插件**

复制到用户 Godot 项目 `res://addons/fennara/` 下的可安装插件。它负责面板 UI、面向 Godot 的检查工具、原生 GDExtension 库、打包后的聊天 UI 资源、运行时辅助脚本以及项目本地的插件版本。

**Fennara CLI**

安装在用户机器上的 `fennara` 命令。它负责安装、更新、CLI 自我更新、doctor 检查、MCP 应用设置、WebView 前置条件警告、C# 设置检查和生成项目指南。

**本地包**

针对一个平台和架构的发布 ZIP，其中包含 MCP 服务器、守护进程、运行时二进制文件和启动器等本地 Fennara 可执行文件。

**项目指南**

写入 Godot 项目的指南文件，包括 `AGENTS.md` 以及 `addons/fennara/ai/` 下按需路由的参考资料，让 AI 编程智能体知道何时以及如何使用 Fennara。

<a id="mcp-terms"></a>
## MCP 术语

**Fennara MCP 服务器**

由 Claude Code、Cursor、Cline、Gemini CLI 或其他 MCP 客户端等 AI 编程应用启动的本地 stdio MCP 服务器。它向外部应用公开 Fennara 工具。

**MCP 应用**

由 `fennara mcp-setup` 配置的外部 AI 应用。MCP 应用设置决定哪些外部应用可以调用 Fennara 工具，不会选择 Fennara 内置聊天使用的模型。

**MCP 目标**

由停靠面板选定、在守护进程中全局生效的兼容性目标，供没有 MCP 项目绑定的外部
MCP 连接使用。已绑定的 MCP 连接不会读取或更改此目标。

**MCP 项目绑定**

Fennara MCP 进程启动时一次性选定的稳定项目根目录。它会将该进程的调用路由到
匹配的 Godot 编辑器会话，不使用守护进程的全局 MCP 目标。

**项目根目录**

包含某个 Godot 项目 `project.godot` 的规范化文件系统目录。Fennara 使用文件系统身份而非
项目名称来区分仓库和工作树。

**Godot 编辑器会话**

一个当前已连接的 Fennara 插件与 Godot 编辑器实例。它具有项目路径和 Godot 进程 ID，
并且可以断开后重新连接，而不改变 MCP 进程的项目绑定。

**工具 Schema**

面向模型的 Fennara MCP 工具说明，包括参数、限制和工作流提示。

**工具结果封装**

工具调用后返回给模型的精简结果。Fennara 结果应说明状态、重要发现和接下来有用的上下文，不应倾倒不必要的原始数据。

<a id="built-in-chat-terms"></a>
## 内置聊天术语

**内置聊天**

Fennara 在 Godot 插件或系统浏览器中提供的聊天界面。它与外部 MCP 应用相互独立。用户可以为 MCP 配置 Claude Code，同时为内置聊天选择另一个提供商或模型。

**聊天界面**

内置聊天的显示模式。嵌入模式使用原生 Godot 面板 WebView，浏览器模式则由本地守护进程提供同一套 UI，并在系统浏览器中打开。

**聊天提供商**

可以生成内置聊天回复的后端，例如 OpenAI、Anthropic、OpenRouter、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、本地 Ollama 或 LM Studio。

**模型引用**

内置聊天中选定的、包含提供商限定符的模型标识。`/provider` 和 `/model` 等斜杠命令可帮助用户连接提供商并选择模型引用。

**提供商连接**

由守护进程管理的聊天提供商本地设置和认证状态，包括 API 密钥或本地基础 URL。提供商密钥应留在守护进程管理的本地存储中，而不是 Godot 项目内。

**生成追踪**

内置聊天生成过程的存储元数据，它把助手消息、工具调用、提供商和模型选择、用量及成本日志关联到产生这些内容的具体生成过程。

<a id="runtime-and-webview-terms"></a>
## 运行时与 WebView 术语

**Fennara 守护进程**

将 MCP 调用和内置聊天请求连接到 Godot 插件的本地服务。它存储本地运行时状态，并提供 `/chat/` 等由守护进程托管的聊天路由。

**运行时会话**

由守护进程管理的交互式 Godot 游戏进程，用于运行中场景检查、日志和运行时捕获。
即使所属项目的编辑器重新连接，其规范项目根目录仍保留控制权。有界场景验证和独立截图调用
使用不同路径，不占用运行时槽位。

**运行时槽位**

在整台机器范围内生效的准入状态，确保所有已连接项目中最多只能启动或运行一个由守护进程管理的
运行时会话。

**运行时租约**

所属项目根目录占用运行时槽位的可续期、有时限权利。所有者活动会续期其非活动截止时间，
而绝对截止时间始终强制执行。

**Godot 快照**

在可能修改文件的 Fennara 辅助轮次前创建的可恢复项目状态快照。快照设置应在持久化用户轮次前完成，避免设置失败后留下悬空提示词。

**WebView 运行时**

在 Godot 内或旁边显示内置聊天所需的平台支持。Windows 使用 WebView2，macOS 使用 WebKit/WKWebView，Linux 使用安装在 Fennara 应用数据目录中的共享 CEF 运行时。

**共享 Linux CEF 运行时**

Linux 聊天 WebView 使用的外部 CEF 运行时负载。它仅在用户的 Fennara 应用数据目录中安装一次，不得打包进每个 Godot 插件 ZIP。

<a id="release-terms"></a>
## 发布术语

**发布清单**

名为 `fennara-release-manifest-v<version>.json` 的 JSON 资源。它将发布资源映射到平台、记录 SHA-256 哈希、声明共享运行时资源，并设置 `minimum_cli_version`。

**最低 CLI 版本**

允许使用某个发布清单的最低 `fennara` CLI 版本。如果发布版需要更新的安装或更新逻辑，请更新 `scripts/release-policy.mjs` 中对应的轨道。清单写入器会在验证发布身份后应用该策略，工作流不会自行选择此值。

**最新发布版**

GitHub 的 Latest Release 指针，指向一个精确的版本化发布。安装程序和默认更新通过 GitHub API 解析此指针。Fennara 不使用字面意义上的 `latest` 标签或发布。发布后更新源文件不会改变发布资源，已发布的清单资源必须明确替换。
