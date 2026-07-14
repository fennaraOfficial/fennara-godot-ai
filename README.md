# Fennara Godot AI

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/demos.md)
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
      <strong>Watch the featured demo</strong><br />
      Comparing Fennara with other Godot MCPs.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Play this video</a><br />
      <a href="docs/demos.md">Browse all demo videos</a>
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

## What Setup Adds

- the Fennara addon kept in `res://addons/fennara/`
- a small `fennara` CLI installed in Fennara app data
- a local MCP server used by AI coding apps
- a local daemon that bridges MCP/chat requests to the open Godot editor
- generated project guidance for AI agents

The built-in chat dock uses the platform webview: Microsoft Edge WebView2 on Windows, WKWebView/WebKit on macOS, and a Fennara-managed shared CEF runtime on Linux. MCP tools still work if the optional chat dock cannot start.

## Install

On Windows and Linux, choose either the addon or CLI install. On macOS, use the
CLI install below if you want to avoid the macOS security notification that can
appear after manually downloading and extracting the addon ZIP.

### Add The Addon To Your Project

- Open the [Latest Release](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest), download `fennara-addon-latest.zip`, and extract its `addons/fennara/` folder into your project.

Open the project, select the Fennara dock, and press **Set Up Fennara**.

> **macOS:** The release addon contains a native library that is not currently
> Apple-notarized. If you download the addon ZIP through a browser and extract
> it manually, macOS may report that it cannot verify
> `libfennara.macos.editor` is free of malware. To avoid this notification, use
> the CLI installation below. If you already see the notification, close Godot,
> remove the manually copied `addons/fennara/` folder, then install Fennara with
> the CLI.

### Install With The CLI (Recommended On macOS)

The CLI installs the same Fennara addon. It is the recommended installation
method on macOS because it avoids the browser and Finder quarantine path that
causes the notification described above.

Install the CLI on Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Or on macOS and Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Then run Fennara from your Godot project:

```bash
cd path/to/your-godot-project
fennara install
```

See [Setup](docs/setup.md) for troubleshooting and [Fennara CLI](docs/cli.md)
for the complete command reference.

## Set Up A Provider Or Connect An MCP App

### Built-In Chat

Open **Chat Settings > Chat**, select **Open providers**, and connect a provider.
Fennara uses your own key for cloud providers (BYOK). You can also use a local
Ollama or LM Studio server. See the [supported provider list](docs/providers.md).

### MCP Apps

Open **Chat Settings > MCP Apps**, find your app, and press **Set Up**.

You can also connect an app from the terminal:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

If your MCP app is not listed in Chat Settings, see [MCP Setup](docs/mcp-setup.md)
for the complete app list and manual configuration instructions.

## Update

When the Fennara dock shows **Update**, press it and follow the prompts.

> **Upgrading from Fennara v0.3.8 or older:** Reinstall the CLI once with the
> platform install command above before running `fennara update`. Those CLI
> versions resolve a retired release tag and cannot discover current releases.
> Reinstalling the CLI switches future updates to GitHub's Latest Release
> endpoint and does not remove your existing project addon or settings.

To update from the terminal, close Godot and run:

```bash
cd path/to/your-godot-project
fennara update
```

See [Update Fennara](docs/setup.md#update-fennara) for recovery and diagnostics.

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

## Demos

Watch a hands-on Fennara walkthrough:

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

More videos:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

See [Demos](docs/demos.md) for more videos from the Fennara channel.

## Star History
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

## Documentation

| Start with... | When you need... |
| --- | --- |
| [Documentation home](docs/README.md) | Every guide and reference page |
| [Setup](docs/setup.md) | Installation, updates, and troubleshooting |
| [Chat providers](docs/providers.md) | Built-in chat models and keys |
| [MCP setup](docs/mcp-setup.md) | Codex, Claude, Cursor, and other MCP apps |
| [Tools](docs/tools.md) | The Godot feedback available to agents |
| [Contributing](CONTRIBUTING.md) | Development and pull request guidance |

## Community

Questions, setup help, and early feedback are welcome on Discord:

https://discord.com/invite/3fF4ft9PTk

## License

See [LICENSE.md](LICENSE.md).
