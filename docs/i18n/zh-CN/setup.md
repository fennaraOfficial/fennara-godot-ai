<!-- fennara-i18n: locale=zh-CN source=docs/setup.md sha256=ab1b11ff7dd3472ab14185e920004b6504fa14eb1c29e7c7b1d7a322780af1dd -->
<a id="setup"></a>
# 设置

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../setup.md) · **简体中文** · [Español](../es/setup.md) · [Português do Brasil](../pt-BR/setup.md) · [日本語](../ja/setup.md) · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · [Deutsch](../de/setup.md) · [Türkçe](../tr/setup.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../setup.md)
<!-- fennara-doc-nav:end -->

安装 Fennara，选择你想在哪里聊天，并连接你的 Godot 项目。

> [!TIP]
> 大多数用户只需要添加插件、打开 Fennara 停靠面板，然后按
> **Set Up Fennara**。在 macOS 上，请使用下面的 CLI 安装，以避免
> 手动下载插件 ZIP 后可能出现的安全通知。

<a id="before-you-start"></a>
## 开始之前

| 要求 | 何时需要 |
| --- | --- |
| Godot 4.5 或更高版本 | 始终需要 |
| Windows x86_64、Linux x86_64 或 macOS arm64 | 始终需要 |
| 支持 MCP 的 AI 应用 | 仅用于外部 MCP |
| 云端 API 密钥、Ollama 或 LM Studio | 仅用于内置聊天 |
| 可作为 `dotnet` 使用的 .NET SDK | 仅用于 C# 诊断和运行时预检 |

<a id="install-from-godot"></a>
## 从 Godot 安装

> [!IMPORTANT]
> 在 macOS 上，发布插件包含一个当前尚未
> 经过 Apple 公证的原生库。通过浏览器下载插件 ZIP 并
> 手动解压，可能会让 macOS 报告无法验证
> `libfennara.macos.editor` 不含恶意软件。请使用
> [从终端安装](#install-from-the-terminal-recommended-on-macos)
> 以避免此通知。

1. 从[最新发布](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)
   下载 `fennara-addon-latest.zip`
   并将 `addons/fennara/` 复制到你的项目中。
2. 打开项目并选择 Fennara 停靠面板。
3. 按 **Set Up Fennara**。

Fennara 会安装匹配的本地组件并连接已打开的项目。
如果较旧的共享守护进程处于空闲状态，设置会在激活匹配
版本之前停止它。版本切换要求连接的项目数量为零。正在
设置的项目在版本不同时通常保持断开连接。如果设置报告
已有连接的项目，请关闭所有其他启用了 Fennara 的编辑器，然后重试。如果
当前项目仍有陈旧连接，请关闭并重新打开此编辑器，
然后重试。
如果设置失败，停靠面板会提供 **Retry**、**Copy Report** 和 **Open Logs**。
复制的报告已经净化，不包含 API 密钥、聊天内容或
项目文件。

> [!NOTE]
> 插件保留在你的项目中。CLI、守护进程、MCP 服务器、日志和共享
> 浏览器运行时位于项目之外的 Fennara 应用数据中。

<a id="install-from-the-terminal-recommended-on-macos"></a>
## 从终端安装（macOS 推荐）

CLI 会安装同一个插件，也是 macOS 上推荐的安装方法。
它避开了导致上述原生库通知的浏览器和 Finder 隔离路径。

在 Windows 上安装 CLI：

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

或者在 macOS 和 Linux 上：

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

然后在项目内运行 Fennara：

```bash
cd path/to/your-godot-project
fennara install
```

如果你已经在 macOS 上手动解压插件并看到了该通知，
请关闭 Godot，移除手动复制的 `addons/fennara/` 文件夹，然后再
运行 `fennara install`。这一点很重要，因为 CLI 会保留现有的
完整插件，而不是替换它。

如果项目已经包含完整的 Fennara 插件，CLI 会保留它并
安装匹配的本地组件。否则，它也会安装当前
发布的插件。关于版本固定和自动化，请参阅 [CLI 安装参考](cli.md#install-a-project)。

<a id="choose-how-you-use-fennara"></a>
## 选择如何使用 Fennara

| 路径 | 模型账户 | 设置 |
| --- | --- | --- |
| 内置聊天 | 在 Fennara Chat Settings 中连接的提供方 | [连接提供方](#连接内置聊天) |
| 外部 MCP 应用 | 该应用自己的模型账户或订阅 | [连接 MCP 应用](#连接-mcp-应用) |
| 两者 | 每条路径保留自己的模型设置 | 完成两个部分 |

<a id="connect-the-built-in-chat"></a>
### 连接内置聊天

1. 打开 **Chat Settings > Chat**。
2. 选择 **Open providers**。
3. 使用你自己的密钥连接云端提供方，或连接本地 Ollama 或
   LM Studio 服务器。
4. 选择一个模型。

请参阅[内置聊天提供方](providers.md)，了解支持的提供方、密钥、本地
服务器 URL 和模型 ID。在输入框中使用 `/provider` 和 `/model` 可执行相同
操作。

嵌入式聊天使用平台 webview：

| 平台 | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | 系统 WKWebView/WebKit |
| Linux | 由 Fennara 管理的共享 CEF 运行时 |

`fennara install`、`fennara update` 和 `fennara doctor` 会检查这些
先决条件。如果可选的嵌入式聊天无法启动，MCP 工具仍可继续
工作。

若要改用系统浏览器，请在 Chat Settings 中启用 **Open chat in my system browser next
time**，然后重新启动 Godot。这只会更改内置
聊天的显示位置。它会保留相同的提供方、历史和项目连接。

若要将代码附加到下一条内置聊天消息，请在 Godot 脚本
编辑器中选中代码，打开上下文菜单，然后选择 **Add to Chat**。

<a id="connect-an-mcp-app"></a>
### 连接 MCP 应用

打开 **Chat Settings > MCP Apps**，找到你的应用，然后按 **Set Up**。重新启动
应用，使其能够加载 Fennara。

你也可以从终端连接应用：

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

如果你的应用未列出，请参阅 [MCP 设置](mcp-setup.md)，了解所有受支持的
目标和手动配置格式。

外部 MCP 应用使用它们自己的模型账户。内置聊天使用
Fennara Chat Settings 中选择的提供方。关于两者的区别，请参阅
[MCP 应用与内置聊天](chat-vs-mcp.md)。

<a id="verify-the-connection"></a>
## 验证连接

打开 Godot 项目，然后向你的 MCP 应用询问：

```text
使用 Fennara MCP 运行 fennara_status，并告诉我连接的是哪个 Godot 项目。
```

如果它报告了错误的项目，请从 Fennara
停靠面板中选择正确的 MCP 目标。

<a id="update-fennara"></a>
## 更新 Fennara

当停靠面板显示 **Update** 时，请按下它并按照提示操作。Fennara
会在请求关闭 Godot 之前下载并验证更新。安装后，它会重新打开
同一个项目，并保留先前可工作的版本，直到
更新通过验证。

要从终端更新，请关闭 Godot 并运行：

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> 如果你正从 Fennara v0.3.8 或更早版本升级，请先用上方的平台安装命令
> 重新安装一次 CLI，再运行 `fennara update`。
> 这些 CLI 会查询一个已经停用的发布标签，无法发现当前发布。
> 重新安装 CLI 会让未来的更新改用 GitHub 的 Latest Release
> 端点，而不会移除你的项目插件或设置。

> [!IMPORTANT]
> 在 macOS 上，从 Fennara v0.3.11 升级之前，请重新安装一次 CLI。该
> CLI 会在到达自更新之前拒绝现有的框架捆绑。此次
> 重新安装只替换 CLI，并保留项目插件和设置。

如果验证失败，请在停靠面板中使用 **Restore Previous Version**、**Open Logs** 或
**Copy Report**。关于精确版本、准备和中断更新恢复，请参阅 [CLI 更新参考](cli.md#update-a-project)。

<a id="troubleshooting"></a>
## 故障排除

<a id="an-install-or-update-failed"></a>
### 安装或更新失败

从停靠面板复制净化报告，或在
终端中显示最新报告：

```bash
fennara diagnostics
```

请参阅 [CLI 诊断](cli.md#inspect-health-and-failures)，了解操作 ID、
JSON 输出、记录字段和脱敏保证。

<a id="fennara-is-not-found"></a>
### 找不到 `fennara`

打开一个新终端并运行：

```bash
fennara doctor
```

如果命令仍不可用，请将 Fennara `bin` 目录添加到 PATH。
[CLI 安装页面](cli.md#install-the-cli)列出了各平台路径。

<a id="windows-binaries-fail-before-starting"></a>
### Windows 二进制文件在启动前失败

如果 Fennara 二进制文件报告缺少 `VCRUNTIME` 或 `MSVCP` DLL、退出代码
`-1073741515` 或 `0xc0000135`，请安装 Microsoft Visual C++ Redistributable
2015-2022 x64：

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

只有缺少这些 Microsoft 运行时 DLL 的 Windows 机器才需要此项。

<a id="a-release-requires-a-newer-cli"></a>
### 发布要求更新的 CLI

如果 CLI 自更新无法安装所需版本，请重新运行
[安装 CLI](cli.md#install-the-cli)中的安装脚本，然后重试该命令。

<a id="the-addon-is-not-visible-in-godot"></a>
### 插件在 Godot 中不可见

确认此文件存在，然后重新打开项目：

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` 显示了错误的项目

打开预期项目，并使用 Fennara
停靠面板中的 MCP 目标控件选择它。

<a id="c-diagnostics-are-missing"></a>
### 缺少 C# 诊断

确认项目中有一个明确的 `.csproj`、`.sln` 或 `.slnx`，然后运行：

```bash
dotnet --version
```

关于浏览器运行时布局、手动恢复和实现细节，请参阅
[架构](architecture.md)、[手动安装](manual-install.md)和
[常见问题](faq.md)。
