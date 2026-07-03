# Fennara Godot AI

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Used by Godot developers and teams, including [Somni Game Studios](https://somnigamestudios.com/).

Fennara gives AI assistants a live connection to Godot. Use it from MCP-capable apps like Codex, Claude, Cursor, Gemini, and Antigravity, or from the optional in-editor chat dock.

Agents can inspect scenes, check scripts, capture screenshots, read runtime errors, and validate changes inside the editor instead of guessing from project files alone.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Comparing Fennara with other Godot MCPs" width="100%" />
      </a>
    </td>
    <td>
      <strong>Watch: Comparing Fennara with other Godot MCPs</strong><br />
      See how Fennara's Godot feedback loop compares with command-only MCP workflows.
    </td>
  </tr>
</table>

## What It Does

- exposes Godot-aware tools to external AI apps through MCP
- adds an optional local chat dock inside the Godot editor
- returns real Godot feedback: scene trees, diagnostics, screenshots, runtime logs, and validation results
- keeps the agent accountable to the open editor instead of only the filesystem

External MCP apps and the built-in chat use separate model settings. See [MCP Apps And Built-In Chat](docs/chat-vs-mcp.md) and [Built-In Chat Providers](docs/providers.md).

## Requirements

- Godot 4.5 or newer.
- A supported desktop OS: Windows x86_64, Linux x86_64, or macOS arm64.
- An MCP-capable coding app only if you want to use Fennara from Claude, Codex, Cursor, Gemini, Antigravity, or another external AI app.
- A chat provider only if you want to use the built-in Fennara chat dock. This can be a cloud provider key or a local provider such as Ollama / LM Studio.

For the full install walkthrough, see [Setup](docs/setup.md).

## What Fennara Installs

- a small `fennara` CLI
- a local MCP server used by AI coding apps
- a local daemon that bridges MCP/chat requests to the open Godot editor
- a Godot addon copied into `res://addons/fennara/`
- generated project guidance for AI agents

The built-in chat dock uses the platform webview: Microsoft Edge WebView2 on Windows, WKWebView/WebKit on macOS, and a Fennara-managed shared CEF runtime on Linux. MCP tools still work if the optional chat dock cannot start.

## Quick Start

Install the CLI and Godot addon first, then choose the MCP app path, the built-in chat path, or both.

### 1. Install The CLI

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS / Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Check the install:

```bash
fennara doctor
```

### 2. Add Fennara To A Godot Project

Run this from the Godot project folder:

```bash
cd path/to/your-godot-project
fennara install
```

For a C# project:

```bash
fennara install --csharp
```

`--csharp` installs Fennara's managed `csharp-ls` language server support so
`script_diagnostics`, runtime preflight checks, and C# feedback can report real
C# parser/type issues.

Then open the project in Godot.

`fennara install` also writes project guidance for AI coding agents:

```text
AGENTS.md
addons/fennara/ai/guidelines.md
```

### 3. Optional: Configure Your MCP App

Skip this step if you only want to use the built-in Fennara chat dock.

Claude Code and Claude Desktop:

```bash
fennara mcp-setup --claude
```

Gemini and Antigravity:

```bash
fennara mcp-setup --gemini
```

Cursor:

```bash
fennara mcp-setup --cursor
```

Codex:

```bash
fennara mcp-setup --codex
```

More targets:

```bash
fennara mcp-setup --help
```

Restart the MCP app after setup so it reloads the Fennara server.

If your app is not listed, or if you need to edit an MCP config by hand, see
[MCP Setup](docs/mcp-setup.md).

This step only configures the external MCP app. It does not configure the built-in Fennara chat model. See [MCP Apps And Built-In Chat](docs/chat-vs-mcp.md) if you are wondering why the dock asks for a provider even after `mcp-setup --claude`.

### 4. Optional: Verify External MCP Works

With the Godot project open, ask your MCP app:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

If the project path is correct, the MCP server and Godot plugin are talking.

If more than one Godot project is open, use the Fennara dock's MCP target control to choose which project receives external MCP tool calls.

### 5. Update Later

Run this from the Godot project folder:

```bash
cd path/to/your-godot-project
fennara update
```

`fennara update` reads the release manifest, updates the installed CLI when a newer release requires it, then refreshes the project addon, local runtime package, generated Fennara guidance files, and any release-managed shared webview runtime needed by the current platform. On Windows/macOS it also checks the platform webview prerequisite and warns if the built-in chat dock may not start. Rerun the install script only if CLI self-update is not available for the selected release or install location.

## Tools

Fennara exposes a small set of Godot-aware tools:

- write or update project files and return diagnostics
- run one-off scene edit scripts
- inspect scene trees, nodes, resources, and Godot classes
- validate scenes
- capture screenshots
- start runtime sessions and read runtime logs
- run small runtime scripts against a live scene

The goal is not to replace an agent's normal file tools. Fennara gives the missing Godot feedback loop.

## Built-In Chat

The Fennara dock includes a native web chat surface inside Godot. It talks to the local daemon, not a hosted Fennara backend.

- bring your own model provider key, or use local Ollama / LM Studio
- use `/provider` and `/model` to switch models from inside Godot
- attach selected script ranges and supported image context
- keep chat history, provider keys, and local URLs on your machine
- open the chat embedded in Godot or in your system browser

More detail: [Built-In Chat Providers](docs/providers.md), [Built-In Chat Slash Commands](docs/slash-commands.md).

## Demos

Watch a hands-on Fennara walkthrough:

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

More videos:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

See [Demos](docs/demos.md) for more videos from the Fennara channel.

## Star History

<a href="https://www.star-history.com/#fennaraOfficial/fennara-godot-ai&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=fennaraOfficial/fennara-godot-ai&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=fennaraOfficial/fennara-godot-ai&type=Date" />
    <img alt="Fennara Godot AI Star History" src="https://api.star-history.com/svg?repos=fennaraOfficial/fennara-godot-ai&type=Date" />
  </picture>
</a>

## Repository

Useful starting points:

- [Setup](docs/setup.md)
- [MCP setup](docs/mcp-setup.md)
- [Repo map](docs/repo-map.md)
- [Architecture](docs/architecture.md)
- [Tools](docs/tools.md)
- [FAQ](docs/faq.md)
- [Demos](docs/demos.md)
- [Manual install notes](docs/manual-install.md)
- [Release process](docs/release.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

## Community

Questions, setup help, and early feedback are welcome on Discord:

https://discord.com/invite/3fF4ft9PTk

## License

See [LICENSE.md](LICENSE.md).
