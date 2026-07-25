<!-- fennara-i18n: locale=zh-CN source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# 参与贡献

<!-- fennara-doc-nav:start -->
[English](../../../CONTRIBUTING.md) · **简体中文** · [Español](../es/CONTRIBUTING.md) · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · [日本語](../ja/CONTRIBUTING.md) · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · [Deutsch](../de/CONTRIBUTING.md) · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

感谢你帮助改进 Fennara Godot AI。

<a id="good-contributions"></a>
## 合适的贡献

- 文档修正
- 可复现的错误修复
- 平台兼容性修复
- 构建和打包改进
- 对设置说明的小幅改进

<a id="design-discussion-required"></a>
## 需要先讨论的设计

在开始以下工作前，请先创建 issue 或 discussion：

- 新增 MCP 工具
- 更改工具 schema
- 更改发布工作流
- 大规模架构更改
- 影响自动生成项目指南的更改

<a id="pull-requests"></a>
## 拉取请求

- 保持拉取请求小而聚焦。
- 说明改了什么以及为什么改。
- 说明如何验证更改。
- 对可见的 UI 或文档渲染更改附上截图或录屏。
- 不要包含无关的格式调整或清理。
- 不要在 issue 或拉取请求中粘贴大段自动生成的说明。

<a id="commit-and-pr-titles"></a>
## 提交与 PR 标题

使用 Conventional Commit 格式：

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

常用类型：

- `feat`：面向用户的功能
- `fix`：错误修复
- `docs`：文档
- `ci`：GitHub Actions 和自动化
- `build`：构建或打包
- `refactor`：不改变行为的代码重构
- `test`：测试
- `chore`：维护

<a id="project-boundaries"></a>
## 项目边界

Fennara 应保持与具体游戏无关。请避免假定某款游戏的控制、目标、经济系统、物品栏、战斗、寻路、任务或 UI 流程的 API 和指南。

智能体应检查 Godot 项目的真实场景、脚本、资源、设置、运行时状态、诊断和截图，然后针对该项目组合通用的 Fennara 工具。

<a id="documentation-translations"></a>
## 文档翻译

英文是规范源。请先修正英文，再更新所有受影响的 locale。翻译集合和 locale 元数据位于 `docs/i18n/languages.json`。

- 完整阅读英文页面，然后直接编写翻译。不要使用批量机器翻译服务或正文生成脚本。
- 保持代码块、行内代码、命令、路径、配置键、URL 和产品名称准确不变。
- 保留文档脚本维护的源标记和明确的英文锚点别名。
- 除非流利的审阅者已经检查，否则不要把翻译标记为已由母语使用者审阅。
- 不要把法律文本、内部智能体提示、生成的项目指南、供应商文件或测试夹具作为独立翻译源。

更改规范或翻译文档后运行：

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

这些命令维护导航元数据并验证结构，不会编写翻译正文。

常规导航同步会保留所有现有源哈希。更改英文源后，请直接更新该页面
在全部九种语言版本中的内容，然后有意只确认该规范源：

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

对于每个已审阅并更新翻译的英文页面，重复使用
`--accept-source <path>`。在全部九种翻译都包含新含义之前，
绝不能接受源哈希。
