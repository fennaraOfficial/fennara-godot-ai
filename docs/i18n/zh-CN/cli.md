<!-- fennara-i18n: locale=zh-CN source=docs/cli.md sha256=561efbe26f75accb949050e7af9d06ae607040b86d061c04c4dcf8732ba61135 -->
<a id="fennara-cli"></a>
# Fennara CLI

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../cli.md) · **简体中文** · [Español](../es/cli.md) · [Português do Brasil](../pt-BR/cli.md) · [日本語](../ja/cli.md) · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · [Deutsch](../de/cli.md) · [Türkçe](../tr/cli.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../cli.md)
<!-- fennara-doc-nav:end -->

如果你更喜欢终端、需要诊断或恢复，或者希望按精确版本自动安装，请使用 CLI。

> [!TIP]
> CLI 是 macOS 上推荐的安装方式。它可以避免浏览器下载的插件 ZIP 被手动解压、原生库继承 Finder 隔离属性时可能出现的安全通知。

<a id="common-flow"></a>
## 常见流程

```bash
cd path/to/your-godot-project
fennara install
```

需要检查或修复本地安装时使用 `fennara doctor`。

常规 Godot 使用流程请参阅[设置](setup.md)。本页作为终端命令参考。

<a id="install-the-cli"></a>
## 安装 CLI

Windows：

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS 和 Linux：

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

如果手动解压的 macOS 插件已触发有关 `libfennara.macos.editor` 的通知，请关闭 Godot 并删除手动复制的 `addons/fennara/`，再运行 `fennara install`。否则 CLI 会保留已有的完整插件。

如果 `fennara` 未立即可用，请打开新终端，然后检查安装：

```bash
fennara --version
fennara doctor
```

CLI 按用户安装。项目插件留在各自 Godot 项目中，共享启动器、版本化运行时、操作记录、日志和 Linux CEF 位于 Fennara 应用数据目录：

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## 命令摘要

| 命令 | 用途 |
| --- | --- |
| `fennara install` | 安装或接管项目插件及其匹配的本地组件 |
| `fennara update` | 更新项目及其本地组件 |
| `fennara doctor` | 检查或修复本地安装 |
| `fennara diagnostics` | 显示经过清理的操作报告 |
| `fennara mcp-setup` | 连接外部 MCP 应用 |
| `fennara prepare-export` | 在不含插件的 CI 导出前移除 Fennara 的自动加载项 |
| `fennara recover` | 恢复中断的原生更新 |
| `fennara self-update` | 仅更新已安装 CLI |

运行 `fennara --help` 查看已安装命令摘要，运行 `fennara mcp-setup --help` 查看支持的 MCP 应用目标。

<a id="install-a-project"></a>
## 安装项目

在包含 `project.godot` 的文件夹中运行：

```bash
fennara install
```

也可明确指定项目：

```bash
fennara install --project path/to/project
```

不提供 `--version` 时，CLI 选择当前发布清单。需要可复现性时使用精确发布版：

```bash
fennara install --project path/to/project --version <version>
```

安装有两条安全路径：

- 如果不存在完整插件，CLI 会下载并验证所选发布版，安装 `addons/fennara` 和匹配的本地组件，再写入 Fennara 项目指南。
- 如果已存在完整插件，CLI 会读取其 `VERSION`、验证当前平台库并安装该精确版本的 CLI 管理组件。项目插件保持不变，明确的 `--version` 必须与现有插件匹配。

<a id="prepare-an-addon-free-ci-export"></a>
## 准备不含插件的 CI 导出

如果 CI 检出排除了 `addons/fennara/`，请在 Godot 启动前移除 Fennara
持久化的运行时自动加载项：

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

该命令只编辑 `project.godot` 中的 `_fennara_game_capture` 条目。
它会保留其他自动加载项和设置，并且可以安全地重复运行。此步骤必须在
Godot 启动前运行，因为项目启动过程会在编辑器插件或导出插件能够执行前
验证自动加载路径。CI 也可以选择在启动 Godot 前安装 Fennara 插件。

<a id="update-a-project"></a>
## 更新项目

常规终端更新时，请关闭该项目的 Godot 并运行：

```bash
fennara update --project path/to/project
```

不提供 `--version` 时，CLI 会读取已安装插件身份。稳定版插件解析 GitHub Latest Release，预发布插件只解析自己的 `pr-<number>` 频道。选择器会立即冻结为一个精确版本，在 CLI 自我替换期间也不会改变。之后 CLI 验证发布资源、刷新插件和版本化本地组件、更新项目指南并检查平台 WebView 前置条件。要明确选择发布版，请使用 `--version <version>`。

`--no-self-update` 用于受控自动化或 CLI 已替换后的续接。不要用它绕过发布版最低 CLI 要求。

> [!IMPORTANT]
> 从 Fennara v0.3.8 或更早版本升级时，请先用[设置](setup.md#从终端安装macos-推荐)中的平台安装命令重新安装一次 CLI，再运行 `fennara update`。这些 CLI 查询的是已停用的发布标签，无法发现当前发布版。重新安装不会删除项目插件或设置。

> [!IMPORTANT]
> 在 macOS 上从 Fennara v0.3.11 升级前，请重新安装一次 CLI。该 CLI 会在进入自我更新前拒绝现有 framework bundle。重新安装只替换 CLI，保留项目插件和设置。

<a id="prepare-while-godot-is-open"></a>
### Godot 打开时进行准备

编辑器内更新按钮使用暂存形式：

```bash
fennara update --prepare --project path/to/project
```

准备流程会下载、验证并持久暂存插件，不会关闭 Godot、替换活动插件、切换活动运行时清单或重启守护进程。Godot 面板观察操作收据，并在开始独立的关闭、替换、重新打开和验证步骤前询问用户。面板传递已发现的精确版本，因此指针变化不会改变进行中的更新。

Fennara 一次只支持一个活动共享运行时版本。如果另一个启用了 Fennara 的 Godot 编辑器仍连接到共享守护进程，激活会被阻止。关闭其他编辑器后重试。上一个本地版本和运行时指针仍可用于无网络恢复。

`--prepare` 是供 Godot 集成使用的低层原语。终端用户通常在关闭 Godot 后直接运行 `fennara update`。

<a id="recover-an-interrupted-update"></a>
## 恢复中断的更新

如果新插件无法加载到足以显示恢复面板，请关闭 Godot 并运行：

```bash
fennara recover --project path/to/project
```

CLI 只恢复处于可恢复状态的操作。它会还原上一个插件、共享启动器和活动运行时清单，然后尝试重新打开记录的 Godot 可执行文件。支持人员提供操作 ID 时可指定事务：

```bash
fennara recover --project path/to/project --operation <operation-id>
```

已经完成、仅准备或已经回滚的操作会被拒绝。

<a id="inspect-health-and-failures"></a>
## 检查健康状态和故障

`doctor` 报告检测到的平台、应用数据布局、活动版本、启动器、运行时、守护进程状态和 WebView 前置条件：

```bash
fennara doctor
```

如果它报告正在运行的守护进程或 MCP 运行时比 `current.json` 更旧，请重启 Godot 或相关 MCP 应用，以启动所选运行时。

使用 `--repair` 重建缺失的基础应用数据目录。在 Linux 上，它还会清理过期 CEF 进程配置，并在完整的受管理运行时已经安装时修复当前运行时标记：

```bash
fennara doctor --repair
```

安装、更新、恢复和自我更新操作会写入持久状态和事件。显示最新的清理报告：

```bash
fennara diagnostics
```

查看较旧操作或机器可读输出：

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

报告包含稳定错误代码、阶段、组件版本、所选资源名称和哈希验证结果。项目路径、主目录、Fennara 应用数据路径、凭据、bearer token 和 URL 查询会被脱敏。报告不包含聊天消息、提供商密钥或项目文件内容。

<a id="configure-an-external-mcp-app"></a>
## 配置外部 MCP 应用

Godot 聊天面板在 **Chat Settings > MCP Apps** 下公开这些命令。**Set Up** 按钮会让本地守护进程调用已安装 CLI，因此面板和终端使用同一套配置及备份实现。

运行 `fennara mcp-setup --help` 选择支持目标。更改配置后重启 MCP 应用。该命令把外部应用连接到 Fennara MCP 服务器，不会选择 Godot 内置聊天面板使用的模型提供商。[MCP 设置](mcp-setup.md)包含目标列表、配置位置和手动配置示例。

<a id="update-only-the-cli"></a>
## 仅更新 CLI

常规项目更新会自动处理 CLI 自我更新。只更新已安装 CLI：

```bash
fennara self-update
fennara self-update --version <version>
```

不提供 `--version` 时，自我更新保留活动安装轨道，稳定版使用 GitHub Latest Release，预发布使用其记录的 PR 频道。

预发布不会自动跨到稳定版。要有意离开预发布，请关闭 Godot 并运行 `fennara update --version <stable-version> --project <path>`。共享活动版本改变前会验证该精确稳定发布版。

当支持人员要求，或项目更新报告已安装 CLI 太旧而无法安全继续时使用此命令。

<a id="automation-guidance"></a>
## 自动化建议

- 传递 `--project`，不要依赖当前目录。
- 构建需要可复现时固定 `--version`。
- 失败时保留输出的操作 ID 和日志路径。
- 使用 `fennara diagnostics --operation <id> --json` 获取结构化报告。
- 不要手动编辑 `current.json`、版本目录、更新收据或暂存插件目录。
- 项目在 Godot 中打开时，不要运行常规的插件替换更新。请使用编辑器内更新流程或先关闭 Godot。
