<!-- fennara-i18n: locale=zh-CN source=README.md sha256=bb9720891f1a14c9d6ae542665829e5a6d736f56c0b4afd6160890b8efba398a -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[English](README.md) · **简体中文** · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Русский](README.ru.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![演示](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/zh-CN/demos.md)
[![许可证](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Godot 开发者和团队正在使用 Fennara，其中包括 [Somni Game Studios](https://somnigamestudios.com/)。

Fennara 为 AI 助手提供到 Godot 的实时连接。你可以从 Codex、Claude、Cursor、Gemini 和 Antigravity 等支持 MCP 的应用中使用它，也可以使用可选的编辑器内聊天停靠面板。

智能体可以检查场景、检查脚本、捕获截图、读取运行时错误并在编辑器内验证更改，而不是仅根据项目文件进行猜测。

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="比较 Fennara 与其他 Godot MCP" width="100%" />
      </a>
    </td>
    <td>
      <strong>观看精选演示</strong><br />
      比较 Fennara 与其他 Godot MCP。<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">播放此视频</a><br />
      <a href="docs/i18n/zh-CN/demos.md">浏览所有演示视频</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## 它能做什么

- 通过 MCP 向外部 AI 应用暴露理解 Godot 的工具
- 在 Godot 编辑器内添加一个可选的本地聊天停靠面板
- 返回真实的 Godot 反馈：场景树、诊断、截图、运行时日志和验证结果
- 让智能体对已打开的编辑器负责，而不是只对文件系统负责

外部 MCP 应用和内置聊天使用彼此独立的模型设置。请参阅 [MCP 应用与内置聊天](docs/i18n/zh-CN/chat-vs-mcp.md)和[内置聊天提供方](docs/i18n/zh-CN/providers.md)。

<a id="requirements"></a>
## 要求

- Godot 4.5 或更高版本。
- 受支持的桌面操作系统：Windows x86_64、Linux x86_64 或 macOS arm64。
- 只有当你想从 Claude、Codex、Cursor、Gemini、Antigravity 或其他外部 AI 应用中使用 Fennara 时，才需要支持 MCP 的编程应用。
- 只有当你想使用 Fennara 内置聊天停靠面板时，才需要聊天提供方。它可以是云端提供方密钥，也可以是 Ollama / LM Studio 等本地提供方。

完整安装演练请参阅[设置](docs/i18n/zh-CN/setup.md)。

<a id="what-setup-adds"></a>
## 设置会添加什么

- 保留在 `res://addons/fennara/` 中的 Fennara 插件
- 安装在 Fennara 应用数据中的小型 `fennara` CLI
- AI 编程应用使用的本地 MCP 服务器
- 在 MCP 或聊天请求与已打开的 Godot 编辑器之间进行桥接的本地守护进程
- 为 AI 智能体生成的项目指导

内置聊天停靠面板使用平台 webview：Windows 上使用 Microsoft Edge WebView2，macOS 上使用 WKWebView/WebKit，Linux 上使用由 Fennara 管理的共享 CEF 运行时。如果可选的聊天停靠面板无法启动，MCP 工具仍可工作。

<a id="install"></a>
## 安装

在 Windows 和 Linux 上，可以选择插件或 CLI 安装。在 macOS 上，如果你想避免手动下载并解压插件 ZIP 后可能出现的 macOS 安全通知，请使用
下面的 CLI 安装。

<a id="add-the-addon-to-your-project"></a>
### 将插件添加到项目

- 打开[最新发布](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)，下载 `fennara-addon-latest.zip`，并将其中的 `addons/fennara/` 文件夹解压到你的项目中。

打开项目，选择 Fennara 停靠面板，然后按 **Set Up Fennara**。

Fennara 是编辑器依赖项，而不是游戏运行时依赖项。在导出期间，
编辑器插件会从导出的项目中移除其运行时自动加载项，并跳过
`res://addons/fennara/` 和 `res://.fennara/`。导出完成后，
编辑器项目会恢复原状。如果 CI 检出通过 `.gitignore` 排除了插件，
请在启动 Godot 前运行 `fennara prepare-export --project path/to/project`，
或在该检出中安装插件。Godot 会在导出插件运行前验证自动加载路径，
因此必须先完成这项准备工作。

> **macOS：** 发布插件包含一个当前尚未
> 经过 Apple 公证的原生库。如果通过浏览器下载插件 ZIP 并
> 手动解压，macOS 可能会报告无法验证
> `libfennara.macos.editor` 不含恶意软件。为避免此通知，请使用
> 下面的 CLI 安装。如果你已经看到该通知，请关闭 Godot，
> 移除手动复制的 `addons/fennara/` 文件夹，然后使用
> CLI 安装 Fennara。

<a id="install-with-the-cli-recommended-on-macos"></a>
### 使用 CLI 安装（macOS 推荐）

CLI 会安装同一个 Fennara 插件。它是 macOS 上推荐的安装
方法，因为它避开了导致上述通知的浏览器和 Finder 隔离
路径。

在 Windows 上安装 CLI：

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

或者在 macOS 和 Linux 上：

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

然后从你的 Godot 项目运行 Fennara：

```bash
cd path/to/your-godot-project
fennara install
```

故障排除请参阅[设置](docs/i18n/zh-CN/setup.md)，完整命令参考请参阅
[Fennara CLI](docs/i18n/zh-CN/cli.md)。

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## 设置提供方或连接 MCP 应用

<a id="built-in-chat"></a>
### 内置聊天

打开 **Chat Settings > Chat**，选择 **Open providers**，然后连接提供方。
Fennara 对云端提供方使用你自己的密钥（BYOK）。你也可以使用本地
Ollama 或 LM Studio 服务器。请参阅[支持的提供方列表](docs/i18n/zh-CN/providers.md)。

<a id="mcp-apps"></a>
### MCP 应用

打开 **Chat Settings > MCP Apps**，找到你的应用，然后按 **Set Up**。

你也可以从终端连接应用：

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

如果你的 MCP 应用未列在 Chat Settings 中，请参阅 [MCP 设置](docs/i18n/zh-CN/mcp-setup.md)，
了解完整应用列表和手动配置说明。

<a id="update"></a>
## 更新

当 Fennara 停靠面板显示 **Update** 时，请按下它并按照提示操作。

> **从 Fennara v0.3.8 或更早版本升级：** 请先使用上方的平台安装命令
> 重新安装一次 CLI，再运行 `fennara update`。这些 CLI
> 版本会解析一个已经停用的发布标签，无法发现当前发布。
> 重新安装 CLI 会让未来的更新改用 GitHub 的 Latest Release
> 端点，并且不会移除你现有的项目插件或设置。

> **从 Fennara v0.3.11 升级的 macOS 用户：** 请先使用
> 上方的 macOS 安装命令重新安装一次 CLI，再进行更新。v0.3.11 CLI 会在
> 能够自更新之前拒绝现有的 macOS 框架捆绑。重新安装只会
> 替换 CLI，不会移除你的项目插件或设置。

要从终端更新，请关闭 Godot 并运行：

```bash
cd path/to/your-godot-project
fennara update
```

关于恢复和诊断，请参阅[更新 Fennara](docs/i18n/zh-CN/setup.md#update-fennara)。

<a id="tools"></a>
## 工具

Fennara 暴露一组精简、理解 Godot 的工具：

- 写入或更新项目文件并返回诊断
- 运行一次性场景编辑脚本
- 检查场景树、节点、资源和 Godot 类
- 验证场景
- 捕获截图
- 启动运行时会话并读取运行时日志
- 对实时场景运行小型运行时脚本

目标并不是替代智能体的普通文件工具。Fennara 提供缺失的 Godot 反馈闭环。

<a id="privacy"></a>
## 隐私

Godot 连接后，Fennara 每个 UTC 日最多发送一次匿名活跃安装事件。
其中包含随机安装 UUID、Fennara 和 Godot
版本、操作系统及 CPU 架构。它不包含项目
数据、路径、提示词、工具活动、日志、截图或账户信息。

可以在 **Chat Settings > Chat > Anonymous telemetry** 中禁用遥测，
也可以使用 `FENNARA_DISABLE_TELEMETRY=true` 或 `DO_NOT_TRACK=1`。关于完整载荷、存储、传输和
退出契约，请参阅[匿名遥测](docs/i18n/zh-CN/telemetry.md)。

<a id="demos"></a>
## 演示

观看 Fennara 实操演练：

[![这个 Godot 插件彻底改变 AI 游戏开发](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

更多视频：

- [我给 Codex 一张 AI 游戏图像，它在 Godot 中构建出了这个](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP 构建类 Katamari Godot 游戏](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [这个 Godot 插件彻底改变 AI 游戏开发](https://www.youtube.com/watch?v=wKln8248y2M)

请参阅[演示](docs/i18n/zh-CN/demos.md)，观看 Fennara 频道的更多视频。

<a id="star-history"></a>
## Star 历史
<a href="https://github.com/fennaraOfficial/fennara-godot-ai/stargazers">
  <img alt="Star History Chart" src="https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/star-history/star-history.svg" width="700">
</a>

<a id="documentation"></a>
## 文档

| 从这里开始... | 当你需要... |
| --- | --- |
| [文档首页](docs/i18n/zh-CN/README.md) | 所有指南和参考页面 |
| [设置](docs/i18n/zh-CN/setup.md) | 安装、更新和故障排除 |
| [聊天提供方](docs/i18n/zh-CN/providers.md) | 内置聊天模型和密钥 |
| [MCP 设置](docs/i18n/zh-CN/mcp-setup.md) | Codex、Claude、Cursor 和其他 MCP 应用 |
| [工具](docs/i18n/zh-CN/tools.md) | 智能体可用的 Godot 反馈 |
| [匿名遥测](docs/i18n/zh-CN/telemetry.md) | 收集的数据、传递行为和退出控制 |
| [贡献](docs/i18n/zh-CN/CONTRIBUTING.md) | 开发和拉取请求指南 |

<a id="community"></a>
## 社区

欢迎在 Discord 上提问、获取设置帮助并提供早期反馈：

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## 许可证

请参阅 [LICENSE.md](LICENSE.md)。
