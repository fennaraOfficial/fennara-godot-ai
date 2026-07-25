<!-- fennara-i18n: locale=zh-CN source=docs/README.md sha256=2f8fb6a711c8bb56af570d1657f802c63cbdf2ced6b2c620339c588c9c9211cb -->
<a id="fennara-documentation"></a>
# Fennara 文档

<!-- fennara-doc-nav:start -->
[English](../../README.md) · **简体中文** · [Español](../es/README.md) · [Português do Brasil](../pt-BR/README.md) · [日本語](../ja/README.md) · [한국어](../ko/README.md) · [Русский](../ru/README.md) · [Français](../fr/README.md) · [Deutsch](../de/README.md) · [Türkçe](../tr/README.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../README.md)
<!-- fennara-doc-nav:end -->

从你想完成的任务开始。每个页面先介绍常规路径，再在后面提供高级细节。

<a id="languages"></a>
## 语言

使用上方语言菜单可以在其他语言中停留在同一页面。覆盖率、审阅状态和规范源策略请参阅[语言与翻译状态](languages.md)。

<a id="start-here"></a>
## 从这里开始

| 我想要 | 阅读 |
| --- | --- |
| 安装 Fennara | [设置](setup.md) |
| 连接内置聊天 | [聊天提供商](providers.md) |
| 连接 Codex、Claude、Cursor 或其他 MCP 应用 | [MCP 设置](mcp-setup.md) |
| 更新或恢复 Fennara | [更新 Fennara](setup.md#更新-fennara) |
| 解决设置问题 | [故障排除](setup.md#故障排除) |

<a id="use-fennara"></a>
## 使用 Fennara

| 指南 | 内容 |
| --- | --- |
| [MCP 应用与内置聊天](chat-vs-mcp.md) | 两条路径分别使用哪个模型账户 |
| [工具](tools.md) | 能够理解 Godot 的工具及其使用时机 |
| [示例](examples.md) | 常见 Godot 工作流的提示词 |
| [斜杠命令](slash-commands.md) | 聊天面板中的 `/provider` 和 `/model` |
| [常见问题](faq.md) | 常见问题的简短解答 |
| [演示](demos.md) | 视频和项目演示 |
| [匿名遥测](telemetry.md) | 收集的数据、发送行为和退出控制 |

<a id="reference-and-recovery"></a>
## 参考与恢复

| 参考 | 适用情况 |
| --- | --- |
| [Fennara CLI](cli.md) | 需要终端命令、诊断或自动化 |
| [手动安装](manual-install.md) | 无法使用常规安装程序 |
| [MCP 设置参考](mcp-setup.md) | 需要特定应用或手动配置 |
| [提供商参考](providers.md) | 需要密钥、模型 ID 或本地服务器信息 |

<a id="for-contributors"></a>
## 面向贡献者

| 文档 | 用途 |
| --- | --- |
| [贡献指南](CONTRIBUTING.md) | 贡献和拉取请求规范 |
| [架构](architecture.md) | 系统边界和运行时流程 |
| [仓库地图](repo-map.md) | 代码和生成文件的位置 |
| [发布流程](release.md) | 打包、清单、验证和发布 |
| [项目术语](CONTEXT.md) | 代码和文档共用的名称 |
| [安全](SECURITY.md) | 报告漏洞 |
| [GitHub 元数据](github-metadata.md) | 仓库描述和主题 |
| [Godot 负载](contributors/godot-payload.md) | 打包插件源代码边界 |
| [Godot 插件](contributors/godot-addons.md) | 插件目录结构和规则 |
| [本地工具](contributors/local-tools.md) | CLI、守护进程、MCP 服务器和本地运行时 |
| [运行时辅助脚本](contributors/runtime-helpers.md) | Godot 端运行时辅助源代码 |
| [仓库脚本](contributors/scripts.md) | 构建、同步、验证和打包自动化 |
| [聊天 UI](contributors/chat-ui.md) | 可选编辑器内聊天源代码和设计规则 |

<a id="learn-from-examples"></a>
## 从示例中学习

- [Fennara 与传统 Godot MCP 对比](fennara-vs-traditional-godot-mcp.md)
- [Open RPG 演示解析](open-rpg-demo.md)
- [提示词示例](examples.md)
