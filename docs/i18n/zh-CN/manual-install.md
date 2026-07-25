<!-- fennara-i18n: locale=zh-CN source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# 手动安装

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../manual-install.md) · **简体中文** · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../manual-install.md)
<!-- fennara-doc-nav:end -->

只有在你需要绕过 Godot 设置流程或 `fennara install`
自行组装 Fennara 时，才使用本页面。

> [!TIP]
> 在 Windows 和 Linux 上，大多数用户应该把 `addons/fennara` 添加到项目，
> 打开 Fennara 停靠面板，然后按 **Set Up Fennara**。在 macOS 上，请使用 CLI。
> 请参阅[设置](setup.md)。

> [!IMPORTANT]
> 不建议在 macOS 上手动安装插件 ZIP。插件包含
> 当前尚未经过 Apple 公证的原生库，而浏览器下载
> 加 Finder 解压可能会导致 macOS 报告无法验证
> `libfennara.macos.editor` 不含恶意软件。请使用
> [CLI 安装](setup.md#install-from-the-terminal-recommended-on-macos)
> 以避免此通知。如果通知已经出现，请关闭 Godot，
> 移除手动复制的 `addons/fennara/` 文件夹，然后运行 `fennara install`。

手动安装包含四个部分：CLI、项目插件、共享本地
运行时软件包，以及可选的 MCP 应用配置。

<a id="1-download-release-files"></a>
## 1. 下载发布文件

打开最新的 GitHub 发布：

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

下载发布清单、适合你平台的文件以及共享插件 zip。

| 用途 | 资产 |
| --- | --- |
| 发布计划和 SHA-256 值 | `fennara-release-manifest-v<version>.json` |
| Windows x86_64 CLI | `fennara-cli-windows-x86_64-v<version>.zip` |
| Windows x86_64 本地运行时 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 CLI | `fennara-cli-linux-x86_64-v<version>.zip` |
| Linux x86_64 本地运行时 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Linux x86_64 嵌入式 webview | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 CLI | `fennara-cli-macos-arm64-v<version>.zip` |
| macOS arm64 本地运行时 | `fennara-release-local-macos-arm64-v<version>.zip` |
| 带版本的全平台插件 | `fennara-release-addon-v<version>.zip` |

发布还包含这个用于文档和
手动下载的稳定名称插件别名：

```text
fennara-addon-latest.zip
```

清单记录本地运行时、插件和
共享运行时资产的预期 SHA-256。检查手动
下载时，请将其作为规范源。

<a id="2-install-the-cli"></a>
## 2. 安装 CLI

解压 `fennara-cli` zip。

将其 `bin` 目录添加到 PATH，或把 `fennara` 二进制文件复制到你现有的某个 PATH 文件夹中。

检查它：

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. 安装 Godot 插件

解压 `fennara-addon` zip。

复制：

```text
addons/fennara
```

到你的 Godot 项目中，使项目包含：

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. 安装本地运行时软件包

CLI 通常会替你管理此项。只有在你避免使用 `fennara install` 时，才需要手动设置运行时。

Fennara 的默认数据文件夹：

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

预期布局是：

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

在 Windows 上，二进制文件使用 `.exe`。

`current.json` 使启动器二进制文件指向活动的运行时版本。正常的 `fennara install` 和 `fennara update` 命令会自动创建此文件。

Linux 嵌入式聊天使用共享的 `webview/cef/linux-x64/<cef-version>/`
运行时位置。正常运行 `fennara install` / `fennara update` 会从发布清单和资产中
自动安装由发布管理的 CEF 运行时。
如果你完全手动安装，请将
`fennara-webview-cef-linux-x64-<cef-version>.zip` 解压到该共享运行时
位置，并写入匹配的 `webview/cef/linux-x64/current.json` 标记。
请将该载荷保留在 Godot 项目插件之外，`addons/fennara` 不应
包含 `libcef.so` 或其他 CEF 运行时文件。

此 CEF 载荷仅用于嵌入式 Linux 聊天。用户可以在 Chat Settings 中选择 **Open chat
in my system browser next time**，通过系统浏览器中的本地守护进程显示同一个内置
聊天，而不是使用嵌入式 Godot webview。

最终的 Linux CEF 布局应如下所示：

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json` 必须是：

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` 必须是
CEF 资产对应的发布清单，例如：

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

不要把可写浏览器状态放在 CEF 版本目录中。正常使用时，
每个编辑器的配置文件和日志会写入 Fennara 应用数据的缓存或日志根目录，
而运行时载荷保持共享和只读。

<a id="5-configure-your-mcp-app"></a>
## 5. 配置你的 MCP 应用

安装本地运行时软件包后，配置你的 MCP 应用：

```bash
fennara mcp-setup --claude
```

其他目标：

```bash
fennara mcp-setup --help
```

设置后重新启动 MCP 应用。

如果你的应用未列出，或你在此次安装中要手动编辑 MCP 配置，
请参阅 [MCP 设置](mcp-setup.md)，了解稳定启动器路径和
JSON/TOML 示例。

这只会把外部 MCP 应用连接到 Fennara 的 Godot 工具。它不会
配置 Fennara 内置聊天停靠面板的模型提供方。如果你想使用内置聊天，请在
Godot 内配置停靠面板，或参阅 [MCP 应用与内置聊天](chat-vs-mcp.md)。

<a id="6-verify"></a>
## 6. 验证

打开 Godot 项目，然后向你的 MCP 应用询问：

```text
使用 Fennara MCP 运行 fennara_status，并告诉我连接的是哪个 Godot 项目。
```

如果路径正确，手动安装就已正常工作。

<a id="recommended-shortcut"></a>
## 推荐快捷方式

即使你手动安装了 CLI，也可以让它安装插件和本地运行时软件包：

```bash
cd path/to/your-godot-project
fennara install
```

CLI 还会为 AI 编程智能体写入项目指导：

```text
AGENTS.md
addons/fennara/ai/
```

AI 目录包含精简的始终读取指南、一个索引，以及仅在相关时加载的专门页面。手动复制的插件 ZIP 可以包含此打包目录，但它不会创建或刷新项目根目录的 `AGENTS.md`。当 Fennara 应管理并刷新完整项目指导时，请使用 `fennara install` 和 `fennara update`。
