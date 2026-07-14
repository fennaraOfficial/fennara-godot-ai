# Setup

Install Fennara, choose where you want to chat, and connect your Godot project.

> [!TIP]
> Most users only need to add the addon, open the Fennara dock, and press
> **Set Up Fennara**. On macOS, use the CLI installation below to avoid the
> security notification that can follow a manually downloaded addon ZIP.

## Before You Start

| Requirement | When you need it |
| --- | --- |
| Godot 4.5 or newer | Always |
| Windows x86_64, Linux x86_64, or macOS arm64 | Always |
| An MCP-capable AI app | Only for external MCP use |
| A cloud API key, Ollama, or LM Studio | Only for the built-in chat |
| The .NET SDK available as `dotnet` | Only for C# diagnostics and runtime preflight |

## Install From Godot

> [!IMPORTANT]
> On macOS, the release addon contains a native library that is not currently
> Apple-notarized. Downloading the addon ZIP through a browser and extracting it
> manually can make macOS report that it cannot verify
> `libfennara.macos.editor` is free of malware. Use
> [Install From The Terminal](#install-from-the-terminal-recommended-on-macos)
> to avoid this notification.

1. Download `fennara-addon-latest.zip` from the
   [latest release](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)
   and copy `addons/fennara/` into your project.
2. Open the project and select the Fennara dock.
3. Press **Set Up Fennara**.

Fennara installs the matching local components and connects the open project.
If an older shared daemon is idle, setup stops it before activating the matching
version. A version switch requires zero connected projects. The project being
set up normally stays disconnected while the versions differ. If setup reports
a connected project, close every other Fennara-enabled editor and retry. If a
stale connection remains for the current project, close and reopen this editor,
then retry.
If setup fails, the dock provides **Retry**, **Copy Report**, and **Open Logs**.
Copied reports are sanitized and do not include API keys, chat content, or
project files.

> [!NOTE]
> The addon stays in your project. The CLI, daemon, MCP server, logs, and shared
> browser runtime live in Fennara app data outside the project.

## Install From The Terminal (Recommended On macOS)

The CLI installs the same addon and is the recommended installation method on
macOS. It avoids the browser and Finder quarantine path that causes the native
library notification described above.

Install the CLI on Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Or on macOS and Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Then run Fennara inside the project:

```bash
cd path/to/your-godot-project
fennara install
```

If you already extracted the addon manually on macOS and see the notification,
close Godot and remove the manually copied `addons/fennara/` folder before
running `fennara install`. This matters because the CLI preserves an existing
complete addon instead of replacing it.

If the project already contains a complete Fennara addon, the CLI keeps it and
installs the matching local components. Otherwise, it installs the current
release addon too. See the [CLI install reference](cli.md#install-a-project) for
version pinning and automation.

## Choose How You Use Fennara

| Path | Model account | Setup |
| --- | --- | --- |
| Built-in chat | A provider connected in Fennara Chat Settings | [Connect a provider](#connect-the-built-in-chat) |
| External MCP app | The app's own model account or subscription | [Connect an MCP app](#connect-an-mcp-app) |
| Both | Each path keeps its own model settings | Complete both sections |

### Connect The Built-In Chat

1. Open **Chat Settings > Chat**.
2. Select **Open providers**.
3. Connect a cloud provider with your own key, or connect a local Ollama or
   LM Studio server.
4. Choose a model.

See [Built-In Chat Providers](providers.md) for supported providers, keys, local
server URLs, and model IDs. Use `/provider` and `/model` for the same actions
from the composer.

The embedded chat uses the platform webview:

| Platform | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | System WKWebView/WebKit |
| Linux | Fennara-managed shared CEF runtime |

`fennara install`, `fennara update`, and `fennara doctor` check these
prerequisites. MCP tools continue to work if the optional embedded chat cannot
start.

To use the system browser instead, enable **Open chat in my system browser next
time** in Chat Settings and restart Godot. This changes only where the built-in
chat appears. It keeps the same provider, history, and project connection.

To attach code to the next built-in chat message, select code in Godot's script
editor, open the context menu, and choose **Add to Chat**.

### Connect An MCP App

Open **Chat Settings > MCP Apps**, find your app, and press **Set Up**. Restart
the app so it can load Fennara.

You can also connect an app from the terminal:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

If your app is not listed, see [MCP Setup](mcp-setup.md) for every supported
target and manual configuration formats.

External MCP apps use their own model accounts. The built-in chat uses the
provider selected in Fennara Chat Settings. See
[MCP Apps And Built-In Chat](chat-vs-mcp.md) for the distinction.

## Verify The Connection

Open the Godot project, then ask your MCP app:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

If it reports the wrong project, select the correct MCP target from the Fennara
dock.

## Update Fennara

When the dock shows **Update**, press it and follow the prompts. Fennara
downloads and verifies the update before asking to close Godot. It reopens the
same project after installation and keeps the previous working version until
the update validates.

To update from the terminal, close Godot and run:

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> If you are upgrading from Fennara v0.3.8 or older, reinstall the CLI once
> with the platform installation command above before running `fennara update`.
> Those CLIs query a retired release tag and cannot discover current releases.
> Reinstalling the CLI switches future updates to GitHub's Latest Release
> endpoint without removing your project addon or settings.

If validation fails, use **Restore Previous Version**, **Open Logs**, or
**Copy Report** in the dock. See the [CLI update reference](cli.md#update-a-project)
for exact versions, preparation, and interrupted-update recovery.

## Troubleshooting

### An Install Or Update Failed

Copy the sanitized report from the dock, or show the latest report in a
terminal:

```bash
fennara diagnostics
```

See [CLI diagnostics](cli.md#inspect-health-and-failures) for operation IDs,
JSON output, recorded fields, and redaction guarantees.

### `fennara` Is Not Found

Open a new terminal and run:

```bash
fennara doctor
```

If the command is still unavailable, add the Fennara `bin` directory to PATH.
The [CLI installation page](cli.md#install-the-cli) lists the platform paths.

### Windows Binaries Fail Before Starting

If a Fennara binary reports a missing `VCRUNTIME` or `MSVCP` DLL, exit code
`-1073741515`, or `0xc0000135`, install the Microsoft Visual C++ Redistributable
2015-2022 x64:

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

This is required only on Windows machines missing those Microsoft runtime DLLs.

### A Release Requires A Newer CLI

If CLI self-update cannot install the required version, rerun the install script
from [Install The CLI](cli.md#install-the-cli), then retry the command.

### The Addon Is Not Visible In Godot

Confirm this file exists, then reopen the project:

```text
addons/fennara/fennara.gdextension
```

### `fennara_status` Shows The Wrong Project

Open the intended project and select it with the MCP target control in the
Fennara dock.

### C# Diagnostics Are Missing

Confirm the project contains one clear `.csproj`, `.sln`, or `.slnx`, then run:

```bash
dotnet --version
```

For browser runtime layouts, manual recovery, and implementation details, see
[Architecture](architecture.md), [Manual Install](manual-install.md), and the
[FAQ](faq.md).
