# Setup

This guide walks through a normal Fennara setup from a clean machine to a Godot project connected through an external MCP app, the built-in chat dock, or both.

## Requirements

- Godot 4.5+ project
- An MCP client that can run local stdio MCP servers, only if you want external AI app integration
- Windows, macOS, or Linux
- For C# projects: .NET SDK installed and available as `dotnet`
- Optional for built-in chat: a configured chat provider, such as a cloud API key or a local Ollama/LM Studio server
- Optional for embedded Windows chat: Microsoft Edge WebView2 Runtime
- Windows troubleshooting only: Microsoft Visual C++ Redistributable 2015-2022 x64 if the Fennara CLI/runtime fails to start with missing `VCRUNTIME` / `MSVCP` DLLs or exit code `-1073741515`

## Default: Set Up From Godot

Add Fennara to the project using one of the download options in the
[README Quick Start](../README.md#quick-start). Open the project and select the
Fennara dock. If the matching local components are missing, Fennara shows a
native setup panel that does not depend on chat, the daemon, or a webview.

Select **Set Up Fennara**. The addon reads its own `VERSION`, downloads the
release manifest and matching CLI archive, verifies the archive SHA-256, and
installs the CLI under Fennara app data. It then runs the same
`fennara install --project <path> --version <addon-version>` flow used by the
terminal installer.

The panel follows the CLI's durable operation state and shows installation
progress. A failure includes a stable error code, operation ID when one was
created, **Retry**, **Copy Report**, and **Open Logs** actions. Copied reports
use the sanitized operation state and do not include API keys, chat content, or
project files.

The bootstrap only installs the exact verified CLI and then delegates the rest
of setup to that CLI. It does not place the daemon, MCP server, browser runtime,
or updater inside the project addon.

## Terminal Setup Alternative

Use this flow for non-interactive setup or when the native setup panel cannot
download or launch the CLI. It uses the same CLI that the Godot panel
bootstraps.

Install the CLI using the platform command in
[Fennara CLI](cli.md#install-the-cli), then install the Godot project:

```bash
fennara install --project path/to/your-godot-project
```

Running inside the project works too:

```bash
cd path/to/your-godot-project
fennara install
```

For a C# Godot project, use the same `fennara install` command. Ensure the .NET
SDK required by the project is available through `dotnet`; Fennara uses project
builds for C# diagnostics and runtime preflight.

If the project already has a complete Asset Library or release addon, the CLI
keeps it and installs its exact matching local components. Otherwise it installs
the selected release addon. See [Fennara CLI](cli.md#install-a-project) for
version selection, repeat-install behavior, and automation options.

## Built-In Chat Webview Prerequisites

Fennara MCP tools work without the built-in Godot chat dock. The chat dock needs
the platform webview:

```text
Windows: Microsoft Edge WebView2 Runtime
macOS: WKWebView from the system WebKit.framework
Linux: Fennara-managed shared CEF runtime
```

`fennara install`, `fennara update`, and `fennara doctor` check the current
platform. On Windows, missing WebView2 prints the official Microsoft WebView2
Runtime link. On macOS, WebKit is normally part of the OS, so Fennara only
reports if the framework cannot be found. On Linux, the CEF runtime is selected
from the release manifest and installed under Fennara app data.

On Linux, Fennara also uses a shared browser runtime location for the embedded
chat CEF runtime:

```text
~/.local/share/fennara/webview/cef/linux-x64/<cef-version>
```

The CEF browser payload is installed once per user and shared across Godot
projects/editors. It is not copied into `addons/fennara`. The Linux chat dock
renders through that shared runtime when it is present. Release manifests point
at the matching CEF runtime asset; `fennara install`, `fennara update`, and
`fennara doctor --repair` validate or repair the shared runtime layout.

The shared CEF runtime directory is read-only during normal browser use. Each
open Godot editor gets its own writable CEF profile/cache/log directories under
the Fennara app-data `cache/webview/profiles/cef/` and `logs/webview/cef/`
roots, so multiple editors can keep embedded chat open at the same time.

The built-in chat has its own provider settings. It can use OpenAI, Anthropic,
OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax,
local Ollama, or LM Studio. These settings are separate
from Claude Code, Codex, Cursor, Gemini, or any other external MCP app.
Provider keys and local base URLs are stored locally outside the Godot project.

Chat Settings also has **Open chat in my system browser next time**. Leave it
off to use the embedded Godot dock webview. Turn it on to have the dock show an
**Open chat** button that launches the same built-in chat in your system browser
through the local daemon. Restart Godot after changing this display setting.

Inside the dock, use `/provider` to connect or switch providers and `/model` to
choose a model. See [Built-In Chat Providers](providers.md) and [Built-In Chat Slash Commands](slash-commands.md).

To add focused code context to a built-in chat request, select code in Godot's
script editor, open the script editor context menu, and choose **Add to Chat**.
The selected script range is attached to the next chat message as removable code
context.

## Configure Your MCP App

Claude Code and Claude Desktop:

```bash
fennara mcp-setup --claude
```

Codex:

```bash
fennara mcp-setup --codex
```

Cursor:

```bash
fennara mcp-setup --cursor
```

Gemini and Antigravity:

```bash
fennara mcp-setup --gemini
```

Other supported targets:

```bash
fennara mcp-setup --help
```

Restart the MCP app after running `mcp-setup`.

If your app is not listed, or if `mcp-setup` cannot find that app's config file,
see [MCP Setup](mcp-setup.md) for manual JSON/TOML config examples.

`mcp-setup` only connects that external app to Fennara's MCP tools. For example,
`fennara mcp-setup --claude` lets Claude call Fennara tools, but it does not make
the built-in Fennara dock use Claude or your Claude subscription. The dock uses
the provider configured in Fennara chat settings. See [MCP Apps And Built-In Chat](chat-vs-mcp.md).

## Verify The Connection

Open the Godot project, then ask your MCP app:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

The result should show the project path you expect.

If the wrong project is shown, use the Fennara dock in Godot to set the current project as the MCP target.

## Update Fennara

When the dock shows **Update**, select it to download and verify the release
while Godot stays open. The dock shows progress, then asks whether to close
Godot and install. Choosing **Not Now** leaves the current installation running.

After confirmation, Fennara closes the editor normally, installs the staged
release, and reopens the same project. It keeps the previous working version
until the new addon and daemon validate. If validation fails, the dock offers
**Restore Previous Version**, **Open Logs**, and **Copy Report**.

Terminal users can close Godot and run:

```bash
fennara update --project path/to/your-godot-project
```

See [Fennara CLI](cli.md#update-a-project) for exact-version updates, the
prepare primitive, interrupted-update recovery, and diagnostics.

## Troubleshooting

### An Install Or Update Failed

`fennara install`, `fennara update`, and CLI self-update operations print an
operation ID and durable event-log path. Show the latest sanitized report with:

```bash
fennara diagnostics
```

The report is safe to copy into a support request. See
[CLI diagnostics](cli.md#inspect-health-and-failures) for operation selection,
JSON output, recorded fields, and redaction guarantees.

### `fennara` Is Not Found

Open a new terminal and try again:

```bash
fennara doctor
```

`doctor` also reports when a running Fennara daemon or MCP runtime appears to be
older than the version selected by `current.json`; restart Godot or the MCP app
when it prints that warning.

If it still fails, add the Fennara `bin` directory to PATH manually.
The platform paths are listed in [Fennara CLI](cli.md#install-the-cli).

### Windows CLI Fails Before It Starts

If `fennara`, `fennara-mcp`, or `fennara-daemon` fails before printing normal
output with a missing `VCRUNTIME` / `MSVCP` DLL error, exit code
`-1073741515`, or `0xc0000135`, install the Microsoft Visual C++ Redistributable
2015-2022 x64:

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

This is not a hard requirement for every Windows user. It only means that
machine is missing the Microsoft runtime DLLs used by the current Fennara
Windows binaries. The installer and self-update path already print clearer
messages for the known `-1073741515` case, but the docs call it out here for
manual troubleshooting.

Long term, Fennara should either ship Windows binaries in a way that avoids
this footgun or detect the missing runtime earlier and point directly at the
fix.

### Release Requires A Newer CLI

If `fennara install` or `fennara update` says the release requires a newer
Fennara CLI and self-update could not run, rerun the install script from step 1,
then run the command again. This should be rare; normal package, runtime, and
CLI changes are handled by the release manifest.

### The Addon Is Not Visible In Godot

Check that the project contains:

```text
addons/fennara/fennara.gdextension
```

Then reopen the project or refresh the plugin list in Godot.

### `fennara_status` Shows The Wrong Project

Open the intended Godot project and use the Fennara dock to set it as the MCP target.

### C# Diagnostics Are Missing

Make sure the project contains an unambiguous `.csproj`, `.sln`, or `.slnx` and
that `dotnet` works from your terminal:

```bash
dotnet --version
```
