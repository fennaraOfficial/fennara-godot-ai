<!-- fennara-i18n: locale=zh-CN source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# MCP 设置

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../mcp-setup.md) · **简体中文** · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

将外部 AI 应用连接到 Fennara 的 Godot 工具。该应用会继续使用自己的
模型账户、订阅或 API 设置。

> [!NOTE]
> 这不会配置 Fennara 内置聊天。如果你不确定需要哪条路径，请参阅
> [MCP 应用与内置聊天](chat-vs-mcp.md)。

<a id="quick-setup"></a>
## 快速设置

1. 在 Godot 停靠面板中完成 **Set Up Fennara**。
2. 打开 **Chat Settings > MCP Apps**。
3. 找到你的应用，然后按 **Set Up**。
4. 重新启动该应用。

Fennara 会在更改应用的 MCP 配置前创建备份。组合的
**Claude** 选项会同时配置 Claude Code 和 Claude Desktop。**Gemini
& Antigravity** 会配置两个共享目标。

<a id="terminal-alternative"></a>
### 终端替代方案

先在 Godot 项目内运行 `fennara install`，然后选择一个目标：

| 应用 | 命令 |
| --- | --- |
| Claude Code 和 Claude Desktop | `fennara mcp-setup --claude` |
| 仅 Claude Code | `fennara mcp-setup --claude-code` |
| 仅 Claude Desktop | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini 和 Antigravity | `fennara mcp-setup --gemini` 或 `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

运行 `fennara mcp-setup --help`，查看已安装
CLI 所支持的目标列表。

<a id="manual-setup"></a>
## 手动设置

只有当你的应用不在列表中、设置命令找不到
应用配置文件，或你有意要手动编辑 MCP 配置时，才使用手动设置。

编辑前，请备份配置文件。然后添加一个名为 `fennara` 的本地 stdio MCP
服务器，使其指向稳定的 Fennara MCP 启动器。

默认启动器路径：

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

请使用你机器上的真实绝对路径。不要让 MCP 应用指向
`versions/<version>/fennara-mcp-runtime`，因为 `bin/` 中的稳定启动器可以让应用
配置在 Fennara 更新后继续工作。

<a id="json-mcpservers"></a>
### JSON `mcpServers`

许多 MCP 应用使用顶层 `mcpServers` 对象：

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

有些应用使用相同的 `mcpServers` 键，但只要求 `command`。如果
现有配置中已经有其他服务器，请保留这些条目，只添加
`fennara` 服务器。

Cline 风格的配置还可以包含以秒为单位的更长工具超时：

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### VS Code 风格的 JSON `servers`

包括 VS Code 用户或项目 MCP 配置在内的一些客户端使用顶层
`servers` 对象，并要求 `type: "stdio"`：

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### OpenCode 风格的 JSON `mcp`

OpenCode 风格的 JSON 配置使用顶层 `mcp` 对象。它的超时单位是
毫秒：

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### Codex 风格 TOML

Codex 使用 TOML：

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

不要把 JSON 粘贴到 TOML 文件中，也不要把 TOML 粘贴到 JSON 文件中。请匹配
应用已经使用的格式。

<a id="common-config-locations"></a>
## 常见配置位置

这些是 Fennara 设置辅助程序和当前 MCP
客户端使用的常见位置。应用可能会更改配置路径，有些应用同时支持全局和
项目本地配置。如果应用提供诸如 **Open MCP Config** 的命令，请使用
该命令，而不要猜测。

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

<a id="timeout-guidance"></a>
## 超时指南

一些 Fennara 工具可能比很短的默认 MCP 超时需要更长时间，因为它们
可能会要求 Godot 验证场景、检查运行时状态、捕获截图或
运行诊断。

当客户端支持时，请使用更长的单工具超时：

```text
服务器启动用 30 秒
工具调用用 300 秒
超时字段以毫秒为单位的客户端使用 300000 毫秒
```

如果客户端不支持单服务器超时，请使用该客户端文档中说明的
全局 MCP 超时设置。

<a id="verify-the-connection"></a>
## 验证连接

打开 Godot 项目，然后向你的 MCP 应用询问：

```text
使用 Fennara MCP 运行 fennara_status，并告诉我连接的是哪个 Godot 项目。
```

如果打开了多个 Godot 项目，请使用 Fennara 停靠面板的 **MCP target**
控件，选择哪个项目接收外部 MCP 工具调用。

<a id="troubleshooting"></a>
## 故障排除

如果 Fennara 未出现在 MCP 应用中：

- 确认启动器路径是绝对路径且确实存在
- 确认配置语法是应用所要求的有效 JSON、JSON5 或 TOML
- 确认服务器名为 `fennara`
- 确认应用正在读取你编辑的配置文件
- 完全退出并重新打开 MCP 应用
- 确认 Godot 项目已经安装 Fennara 插件
- 确认预期的 Godot 项目已被选为 MCP 目标

<a id="unsupported-mcp-apps"></a>
## 不受支持的 MCP 应用

如果你的 MCP 应用未列出，请先查找该应用官方的 MCP 配置位置和
格式。然后请 LLM 给出最小且安全的编辑：

```text
我有一个本地 stdio MCP 服务器可执行文件，路径为：
<paste the full path to fennara-mcp here>

我想把它添加到 <app name>。
该应用的 MCP 配置文件是：
<paste config path here>

配置格式是 <JSON/TOML/YAML/etc>。

请展示添加名为 "fennara" 的服务器所需的最小安全编辑。
保留所有现有配置。如果该应用需要 "mcpServers"、"servers"、"mcp"
或其他顶层键，请使用该应用官方文档所要求的键。
```

保存前请检查结果，然后重新启动 MCP 应用。
