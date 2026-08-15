<!-- fennara-i18n: locale=zh-CN source=docs/multi-agent-worktrees.md sha256=037571da5f2479b968f6b74e8994d0272042d4a26616c50af5f62f3d2a9bbcf9 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# 多智能体与 Godot 工作树

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · **简体中文** · [Español](../es/multi-agent-worktrees.md) · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · [日本語](../ja/multi-agent-worktrees.md) · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · [Deutsch](../de/multi-agent-worktrees.md) · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

在同一台机器上，让多个编程智能体分别面向不同仓库或工作树运行，而不会因为某个智能体的目标选择将另一个智能体重定向。每个项目都有自己的 Fennara MCP 进程和连接；所有项目共享同一个每用户守护进程。

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

编辑、检查、有界场景验证和独立截图调用可以并发运行。由守护进程管理的交互式游戏运行会通过一个整台机器共享的运行时槽位串行执行。

<a id="one-mcp-connection-per-project"></a>
## 每个项目一个 MCP 连接

MCP 进程启动时会选定一个稳定的项目根目录。该 MCP 项目绑定是包含 `project.godot` 的目录所对应的规范文件系统身份；它不是项目名称，也不是 Godot 进程 ID。

请为每个仓库或工作树使用不同的 MCP 进程和连接。只有当多个智能体有意共同处理同一个项目时，才应让一个连接服务于多个智能体。Fennara 工具不公开按调用选择项目的参数，因此模型无法意外地将进程切换到其他项目。

每个项目还需要一个已连接、启用 Fennara 的 Godot 编辑器。如果编辑器关闭后以新的进程 ID 重新连接，现有 MCP 进程会在同一项目根目录重新连接时恢复路由。

<a id="how-a-process-chooses-its-project"></a>
## 进程如何选择项目

MCP 运行时会捕获其启动工作目录，并且只按以下优先级选择一次绑定：

1. `--project-path <path>` 或 `--project-path=<path>`。
2. `FENNARA_PROJECT_PATH`。
3. 启动目录中包含 `project.godot` 的最近祖先目录。
4. 自动发现未找到 Godot 项目时，进入旧式未绑定兼容模式。

命令行和环境路径是显式断言。空路径、无法访问、缺失、非目录、非 Godot 或不受支持的路径会阻止 MCP 服务器启动；它绝不会回退到其他项目。相对路径从已捕获的启动目录解析。如果不清楚 MCP 主机的启动目录，请优先使用绝对路径。

Fennara 不会隐式读取特定于主机的工作区变量。MCP 主机可以将自身的工作区值映射到 `--project-path` 或 `FENNARA_PROJECT_PATH`。

<a id="configure-a-project-bound-connection"></a>
## 配置已绑定项目的连接

`fennara mcp-setup` 仍然是全局且与项目无关的。在项目内运行它不会将今后每个 MCP 进程都绑定到该项目。请保留其稳定启动器路径，然后使用 MCP 主机的项目或工作区配置添加绑定。

对于 JSON 风格的配置：

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

也可使用环境变量：

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

对于 Codex 风格的 TOML：

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

请在下一个智能体自己的项目或工作区配置中使用 `/absolute/path/to/worktree-b` 进行配置。如果主机能够可靠地从每个项目目录启动独立的 MCP 进程，祖先目录发现无需显式路径即可提供相同绑定。

<a id="mcp-host-boundaries"></a>
## MCP 主机边界

项目本地配置和启动目录行为因主机而异：

- VS Code 单文件夹工作区可以依赖主机文档中说明的子进程工作目录，但显式项目绑定仍然是最清晰的配置。
- Claude Code、Gemini CLI、Antigravity、Cline、Cursor、OpenCode、Kiro 和 Codex 可以使用项目或工作区配置。必须保证隔离时，请使用显式绑定或有文档说明的项目启动目录。
- Claude Desktop 和旧版 Windsurf/Cascade 使用全局配置。它们默认的 Fennara 条目仍处于旧式未绑定模式，无法提供自动项目本地隔离。高级用户可以创建名称各异、显式路径各不相同的全局条目，但必须选择正确的条目。

自动生成项目本地配置以及新的 Windsurf/Devin Local 支持不在此工作流范围内。

<a id="start-and-verify-the-editors"></a>
## 启动并验证编辑器

每个工作树都需要自己的、启用 Fennara 的 Godot 编辑器。无头编辑器可以使用不同的 Godot LSP 端口，同时共享 Fennara 的守护进程：

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

LSP 端口属于 Godot。Fennara 继续在常规回环地址上使用一个共享守护进程。

并发工作前，请从每个智能体运行 `fennara_status`。确认它报告：

- 路由模式 `bound`
- 预期的绑定来源和规范项目根目录
- 绑定编辑器状态 `connected`
- 该编辑器的编辑器文件系统就绪情况

如果自动发现未找到项目，状态会报告 `legacy_unbound` 和并发性警告。在该兼容模式中，首先使用由停靠面板选定的 MCP 目标，其次使用唯一已连接的编辑器。请勿将未绑定连接用于隔离的并发工作。

<a id="missing-and-duplicate-editors"></a>
## 缺少和重复的编辑器

编辑器不在时，有效的项目绑定仍会保持运行。工具调用会返回可重试的 `bound_project_not_connected`，直到该项目根目录重新连接；它们绝不会回退到停靠面板目标。

两个编辑器解析到同一项目根目录时，会产生 `ambiguous_project_binding`。请关闭重复编辑器，或为它提供不同的工作树。Fennara 不会根据进程 ID、连接顺序、项目名称或停靠面板目标进行选择。

指向同一项目的符号链接别名会解析为同一个实时文件系统身份。MCP 启动后重新定向符号链接不会改变绑定；请重启该 MCP 进程以重新绑定。

<a id="serialized-runtime-sessions"></a>
## 串行化运行时会话

所有项目共享一个整台机器范围的运行时槽位，供守护进程管理的游戏运行使用。当其他项目正在启动或运行会话时，`runtime_session.start` 会返回成功的 `busy` 领域结果，其中包含 `availability: "busy"`、`slot_acquired: false` 和建议的 `retry_after_ms`。它不暴露所有者、会话 ID、进程 ID、场景、日志、队列位置或预计持续时间。

没有 FIFO 队列。请在建议的重试延迟附近带抖动地轮询，并将每次 `runtime_session.start` 视为最终的原子申领。空闲状态仅供参考，因为另一个智能体可能在预检后抢先申领。

只有所属项目根目录才能检查、续期、运行脚本或停止自己的运行时会话。所有者状态查询会对 120 秒的非活动截止时间续期。有界的所有者运行时操作在活动期间会暂停非活动过期，并且只有在返回终态脚本结果后才会对截止时间续期；超时、设置错误或取消都不会续期。运行过程中，智能体应每约 30 秒带抖动地轮询一次所有者状态。

默认的运行时租约绝对时限为 900 秒。`max_run_seconds` 接受不超过 86,400 秒的正整数；例如，预计用时一小时的回归测试可请求 4,500 秒，以留出安全余量。绝对截止时间绝不暂停。自然退出、显式停止、启动失败、非活动过期或绝对过期会停止或回收游戏，并释放运行时槽位。

<a id="safe-multi-agent-checklist"></a>
## 安全多智能体检查清单

1. 为每个项目创建不同的仓库或工作树。
2. 为每个项目根目录安装 Fennara 并打开一个 Godot 编辑器。
3. 为每个项目配置一个已绑定项目的 MCP 进程。
4. 从每个智能体运行 `fennara_status` 并验证其规范根目录。
5. 让编辑、检查、有界场景验证和独立截图并发进行。
6. 对试玩中的非错误 `busy` 结果进行轮询和重试；获得槽位的会话运行时，通过所有者状态使其保持活跃。
