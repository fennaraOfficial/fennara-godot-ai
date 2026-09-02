<!-- fennara-i18n: locale=zh-CN source=docs/chat-vs-mcp.md sha256=b6f27b2c7e905515aba56b75bf6736644a9c36c885f4cab61555c82cd6c47fda -->
<a id="mcp-apps-or-built-in-chat"></a>
# MCP 应用还是内置聊天？

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · **简体中文** · [Español](../es/chat-vs-mcp.md) · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · [日本語](../ja/chat-vs-mcp.md) · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · [Deutsch](../de/chat-vs-mcp.md) · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara 两者都支持。你可以选择对话发生在哪里。

| | 外部 MCP 应用 | Fennara 内置聊天 |
| --- | --- | --- |
| 聊天位置 | Codex、Claude、Cursor、Gemini 或其他 MCP 应用 | Fennara 面板或系统浏览器 |
| 模型账户 | 外部应用的账户或订阅 | 在 Fennara Chat Settings 中连接的提供商 |
| Fennara 提供的能力 | 能够理解 Godot 的 MCP 工具 | 聊天 UI、同一组核心 Godot 工具，以及仅限聊天的文件和 shell 工具 |
| 设置 | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> 两条路径可以同时使用，其模型设置保持独立。

<a id="external-mcp-apps"></a>
## 外部 MCP 应用

连接 MCP 应用后，该应用可以启动本地 Fennara MCP 服务器并调用 Godot 工具。应用的订阅或登录不会与内置聊天共享。

可在 **Chat Settings > MCP Apps** 中设置应用，也可使用 CLI：

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

不需要 Fennara 聊天提供商密钥。设置后请重启外部应用。所有目标和手动配置请参阅 [MCP 设置](mcp-setup.md)。

<a id="built-in-chat"></a>
## 内置聊天

内置聊天需要在 Fennara Chat Settings 中连接提供商。云端提供商使用你自己的密钥，也可以连接本地 Ollama 或 LM Studio 服务器。

同一个聊天既可显示在 Godot 面板中，也可显示在系统浏览器中。这个显示选择不会改变提供商、模型、历史记录或项目。

要附加代码，请在 Godot 脚本编辑器中选中代码，打开上下文菜单，然后选择 **Add to Chat**。提供商和模型设置请参阅[内置聊天提供商](providers.md)。

<a id="project-routing"></a>
## 项目路由

两条路径都通过本地 Fennara 守护进程获取 Godot 反馈。

- 外部 MCP 进程可在启动时一次性绑定到一个规范的 Godot 项目根目录。其调用会路由到匹配的编辑器，不会读取或更改
  停靠面板的 **MCP target**。
- 未绑定的外部 MCP 进程保留兼容性行为：它使用由停靠面板选定的 MCP 目标；如果未选定有效目标且仅有一个已连接编辑器，
  则使用该编辑器。
- 内置聊天始终绑定到打开该聊天的 Godot 编辑器。

对于在不同仓库或工作树中隔离工作的智能体，请为每个项目使用一个 MCP 进程和连接。有关设置和运行时槽位行为，请参阅
[多智能体与工作树](multi-agent-worktrees.md)。

要验证外部 MCP 连接，请询问：

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

开始并发工作前，请确认状态报告的路由模式为 `bound`，并且规范项目根目录符合预期。旧式未绑定模式会包含并发性警告。
