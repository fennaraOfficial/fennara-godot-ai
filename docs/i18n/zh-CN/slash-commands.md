<!-- fennara-i18n: locale=zh-CN source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# 内置聊天斜杠命令

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · **简体中文** · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

斜杠命令是 Godot 内 Fennara 聊天面板的快捷操作。它们是 UI 命令，不是 MCP 工具，也不会作为提示词发送给模型。

在编辑框中输入 `/` 可打开命令面板。

| 命令 | 打开 | 用途 |
| --- | --- | --- |
| `/provider` | 提供商选择器 | 连接云端提供商、配置本地提供商 URL 或切换提供商。 |
| `/model` | 模型选择器 | 从当前或已连接提供商中选择模型。 |

<a id="how-they-behave"></a>
## 操作方式

- 使用方向键在命令建议中移动。
- 按 Enter 执行所选命令。
- 按 Escape 关闭命令面板。
- 聊天消息发送前，斜杠命令文字会从编辑框中移除。

<a id="common-flow"></a>
## 常见流程

在内置聊天面板中输入：

```text
/provider
```

连接 OpenAI、Anthropic、OpenRouter、Ollama Cloud、DeepSeek、Z.AI、Moonshot AI、Kimi For Coding、MiniMax、本地 Ollama 或 LM Studio。

然后输入：

```text
/model
```

选择面板要使用的模型。

外部 MCP 应用不要使用这些斜杠命令。请通过 `fennara mcp-setup` 配置应用，然后让应用使用 Fennara MCP 工具。
