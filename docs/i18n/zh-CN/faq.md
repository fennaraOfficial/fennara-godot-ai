<!-- fennara-i18n: locale=zh-CN source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# 常见问题

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../faq.md) · **简体中文** · [Español](../es/faq.md) · [Português do Brasil](../pt-BR/faq.md) · [日本語](../ja/faq.md) · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · [Deutsch](../de/faq.md) · [Türkçe](../tr/faq.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../faq.md)
<!-- fennara-doc-nav:end -->

安装和更新请从[设置](setup.md)开始。本页提供简短解答及详细参考链接。

| 问题 | 简短回答 |
| --- | --- |
| 需要提供商密钥吗？ | 仅内置聊天使用云端提供商时需要 |
| 能改用外部 MCP 应用吗？ | 可以，它使用自己的模型账户 |
| Fennara 会把项目上传到 Fennara 服务器吗？ | 不会 |
| 可以同时打开多个 Godot 编辑器吗？ | 可以，请在面板中选择外部 MCP 目标 |

<a id="is-fennara-only-a-code-generator"></a>
## Fennara 只是代码生成器吗？

不是。Fennara 是能够理解 Godot 的智能体工作流，可处理项目文件、场景、诊断、运行时错误、截图和 Godot 编辑器上下文。

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## Fennara 只是又一个 Godot MCP 命令服务器吗？

不是。MCP 只是通过 Codex、Claude、Cursor、Gemini 和 Antigravity 等应用使用 Fennara 的一种方式。Fennara 还提供可选的内置聊天面板。产品的核心理念是 Godot 反馈闭环，通过诊断、验证、运行时错误、截图和结构化工具结果让智能体修补错误。

<a id="does-fennara-replace-godot-knowledge"></a>
## Fennara 会取代 Godot 知识吗？

不会。Fennara 并不试图让 Godot 变得可有可无，而是让 AI 智能体对真实 Godot 引擎的结果负责。

<a id="how-should-i-install-fennara"></a>
## 应该如何安装 Fennara？

在 Windows 和 Linux 上，可以添加插件、打开 Fennara 面板并按 **Set Up Fennara**，也可以从终端安装。在 macOS 上，请使用 CLI 安装，以避免手动解压浏览器下载的插件 ZIP 时可能出现的安全通知。两条路径请参阅[设置](setup.md)。

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## 为什么 macOS 无法验证 `libfennara.macos.editor`？

发布版插件包含目前尚未经过 Apple 公证的原生库。通过浏览器下载并手动解压 ZIP 时，Finder 可能把隔离元数据传播到该库，从而触发通知。

要避免此问题，请使用 [CLI 安装](setup.md#从终端安装macos-推荐)。如果已经出现通知，请关闭 Godot，删除手动复制的 `addons/fennara/`，安装 CLI，并从项目目录运行 `fennara install`。CLI 安装同一插件，但不会经过浏览器和 Finder 隔离路径。

<a id="do-i-need-a-chat-provider-api-key"></a>
## 需要聊天提供商 API 密钥吗？

仅当要在 Fennara 内置聊天面板中使用云端提供商时需要。外部 MCP 客户端使用自己的模型和应用配置，不需要把提供商密钥放进 Fennara 聊天。

内置聊天也可以使用本地 Ollama 或 LM Studio，无需云端 API 密钥。请参阅[内置聊天提供商](providers.md)。

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## 已运行 `mcp-setup --claude`，为什么面板仍要求提供商？

`fennara mcp-setup --claude` 将 Claude 连接到 Fennara 的 Godot MCP 工具，不会把内置面板连接到 Claude，也不会与 Fennara 聊天共享 Claude 订阅。

外部 MCP 流程请使用 Claude Code 或 Claude Desktop。只有想在 Godot 的 Fennara 面板中聊天时才需另行配置提供商。请参阅 [MCP 应用与内置聊天](chat-vs-mcp.md)。

<a id="what-are-provider-and-model"></a>
## `/provider` 和 `/model` 是什么？

它们是内置聊天面板中的斜杠命令。`/provider` 打开提供商选择器，`/model` 打开模型选择器。它们是 UI 快捷方式，不是外部 MCP 工具，也不会作为文字发送给模型。请参阅[内置聊天斜杠命令](slash-commands.md)。

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## Fennara 会把 Godot 项目发送到 Fennara 服务器吗？

不会。在常规开源路径中，MCP 客户端、守护进程和 Godot 插件都在本地运行。内置聊天只会把模型请求发送到你配置的提供商，例如 OpenAI、Anthropic、OpenRouter、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax 或本地 Ollama/LM Studio。

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## 多个 Godot 编辑器打开时，哪个项目接收 MCP 工具调用？

守护进程将外部 MCP 调用路由到当前 MCP 目标。请在 Godot 的 Fennara 面板中使用 MCP 目标控件选择项目。内置聊天会话则绑定到打开该聊天的 Godot 编辑器。

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## 为什么 Linux 会安装单独的 CEF 运行时？

Linux 嵌入聊天使用 CEF 离屏渲染。CEF 负载很大，所以 Fennara 只在用户的 Fennara 应用数据目录中安装一次，不会复制进每个 Godot 项目插件。

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## 插件中应该包含 `libcef.so` 吗？

不应该。`libcef.so`、CEF 资源、语言包和 CEF 辅助程序属于共享 Linux CEF 运行时。插件只应包含 Godot 插件文件、GDExtension 二进制文件、聊天 UI 文件和 ripgrep 等小型辅助二进制文件。

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## 内置聊天 WebView 无法启动怎么办？

Fennara MCP 工具仍能工作，只有可选的编辑器内聊天面板依赖平台 WebView。Windows 上，如果 `fennara doctor` 报告缺失，请安装 Microsoft Edge WebView2 Runtime。macOS 的 WKWebView 来自系统 WebKit.framework。Linux 上，请运行 `fennara update` 安装或修复发布版管理的 CEF 运行时。

也可以在 Chat Settings 中启用 **Open chat in my system browser next time**。这会保留同一内置聊天和提供商设置，但通过本地守护进程在系统浏览器中打开 UI。更改后请重启 Godot。

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## 在浏览器打开聊天会使用 Claude 或 MCP 应用吗？

不会。浏览器显示只是内置聊天的 UI 和运行时选择，仍使用 Fennara 聊天设置中选定的提供商。`fennara mcp-setup --claude` 等命令配置的是外部 MCP 应用，不会配置内置聊天模型。

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## `fennara update` 会重写 MCP 应用配置吗？

不会。`fennara update` 会按需刷新 CLI、项目插件、本地运行时包、生成的项目指南和平台管理的运行时资源。只有在设置或修复 MCP 应用配置时才需再次运行 `fennara mcp-setup`。

<a id="where-does-chat-history-live"></a>
## 聊天历史存在哪里？

聊天历史由守护进程存储在本地，并限定在当前 Godot 项目范围内。提供商密钥和本地提供商 URL 也由守护进程存储在项目外的本地位置。

<a id="what-should-agents-use-fennara-tools-for"></a>
## 智能体应把 Fennara 工具用于什么？

请将 Fennara 用于需要 Godot 理解能力的反馈，例如场景树、发生变化的节点和资源属性、诊断、验证、运行时会话、截图和编辑器调试器状态。除非需要 Fennara 专用工具，MCP 客户端仍应使用自己的普通文件读取和搜索工具。
