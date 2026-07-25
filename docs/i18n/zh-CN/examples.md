<!-- fennara-i18n: locale=zh-CN source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# 示例

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../examples.md) · **简体中文** · [Español](../es/examples.md) · [Português do Brasil](../pt-BR/examples.md) · [日本語](../ja/examples.md) · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · [Deutsch](../de/examples.md) · [Türkçe](../tr/examples.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../examples.md)
<!-- fennara-doc-nav:end -->

复制一段提示词，替换其中的项目细节，然后从 MCP 应用或 Fennara
内置聊天中发送它。

| 目标 | 示例 |
| --- | --- |
| 确认已连接的编辑器 | [检查连接](#检查连接) |
| 了解现有项目 | [编辑前检查](#编辑项目前先检查) |
| 进行一项聚焦的更改 | [考虑架构的更改](#进行一项考虑架构的小改动) |
| 诊断正在运行的项目 | [运行时错误](#调试运行时错误) |
| 检查渲染输出 | [视觉反馈](#视觉反馈) |

<a id="check-connection"></a>
## 检查连接

```text
使用 Fennara MCP 运行 fennara_status，并告诉我连接的是哪个 Godot 项目。
```

<a id="inspect-a-project-before-editing"></a>
## 编辑项目前先检查

```text
使用 Fennara MCP 检查这个 Godot 项目。在建议更改之前，先查看场景树、相关文件、诊断信息和项目结构。
```

<a id="make-a-small-architecture-aware-change"></a>
## 进行一项考虑架构的小改动

```text
像一名谨慎的贡献者一样在这个现有 Godot 项目中工作。检查相关系统是如何组织的，进行最小但有用的更改，并说明哪些文件或资源发生了变化以及我该如何测试。
```

<a id="debug-a-runtime-error"></a>
## 调试运行时错误

```text
使用 Fennara MCP 检查最新的 Godot 运行时错误，找出可能的来源，修复问题，并解释修复方法。
```

<a id="visual-feedback"></a>
## 视觉反馈

```text
使用 Fennara MCP 捕获当前场景的截图，检查 UI 布局，如果有明显问题，请建议或进行一项小修复。
```

<a id="built-in-chat-provider-setup"></a>
## 内置聊天提供方设置

在 Godot 内的 Fennara 停靠面板中：

```text
/provider
```

连接一个云端提供方或本地提供方。

然后：

```text
/model
```

选择停靠面板应使用的模型。

<a id="existing-project-demo-prompt"></a>
## 现有项目演示提示词

下面是 Open RPG 演示中所用提示词的类型：

```text
我希望你像一名谨慎的项目贡献者一样，在这个现有 Godot RPG 项目中工作。在进行更改之前，先了解相关系统是如何组织的。尽可能复用现有架构和命名风格。以最小且整洁的方式添加所请求的功能，然后告诉我更改了什么以及如何在游戏中试用。
```
