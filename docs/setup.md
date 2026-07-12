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

Install the Fennara addon from the Godot Asset Library or copy its
`addons/fennara/` folder into the project. Open the project and select the
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
download or launch the CLI.

### 1. Install The Fennara CLI

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS / Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Check that the CLI is available:

```bash
fennara doctor
```

If `fennara` is not found, open a new terminal. The installer may have updated your shell PATH for future sessions.

Fennara installs the CLI here by default:

```text
Windows: %LOCALAPPDATA%\Fennara\bin
macOS: ~/Library/Application Support/Fennara/bin
Linux: ~/.local/share/fennara/bin
```

The install script installs the small outer CLI. In normal releases,
`fennara update` can update that CLI itself before refreshing project assets.
Rerun the install script only when CLI self-update is not available for the
selected release or install location.

### 2. Install Fennara In A Godot Project

Run this inside the Godot project folder:

```bash
cd path/to/your-godot-project
fennara install
```

If you keep MCP or agent config in a separate tooling repo, pass the Godot
project path explicitly instead:

```bash
fennara install --project path/to/your-godot-project
fennara update --project path/to/your-godot-project
```

Your MCP app can point at the global Fennara launcher from anywhere. It does not
need config files inside the Godot project. `--project` tells Fennara which
Godot project to install or update.

For a C# Godot project, use the same `fennara install` command. Ensure the .NET
SDK required by the project is available through `dotnet`; Fennara uses project
builds for C# diagnostics and runtime preflight.

When the project does not have a complete addon, `fennara install` copies the
selected Godot addon into:

```text
addons/fennara
```

If that directory exists from a failed or partial installation but does not
contain `fennara.gdextension`, `fennara install` treats it as incomplete and
replaces it with the selected addon package.

If the directory already contains a complete addon from the Godot Asset
Library or a release archive, install adopts it instead. The CLI reads its
`VERSION`, validates the matching editor library, resolves that exact release,
checks the release's minimum CLI version, and installs the matching daemon, MCP
server, local runtime, and optional shared webview runtime. It does not replace
or rewrite the existing addon. It starts the daemon when needed and confirms
that the running daemon version matches the addon. Repeating the same install
is safe.

Install also reads the release manifest, downloads and verifies the local
Fennara runtime package into your user app-data folder, and writes project
guidance for AI coding agents. Fresh addon installs write both paths below.
Existing-addon adoption only writes the project-level `AGENTS.md`, because the
addon already contains its matching guidance:

```text
AGENTS.md
addons/fennara/ai/guidelines.md
```

If `AGENTS.md` already exists, Fennara only updates the generated block between its own markers.
Existing-addon adoption may also update an existing project `.gitignore` with
Fennara-managed local state entries.

### Built-In Chat Webview Prerequisites

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

## 3. Configure Your MCP App

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

## 4. Verify The Connection

Open the Godot project, then ask your MCP app:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

The result should show the project path you expect.

If the wrong project is shown, use the Fennara dock in Godot to set the current project as the MCP target.

## 5. Update Fennara

Run this inside the Godot project folder:

```bash
cd path/to/your-godot-project
fennara update
```

This reads the release manifest, updates the installed CLI when needed, and
then updates the project addon, the local runtime package, any shared runtime
assets needed by your platform, and the generated Fennara guidance files.

If an MCP app is currently running a Fennara launcher, `fennara update` may keep that launcher and continue. That is okay. The versioned runtime package is still updated.

When an update has to replace the running CLI before continuing, Fennara prints
the updater log path. Use that log to inspect the resumed project-update output
in CI or agent-driven runs.

The native update flow prepares verified files before asking Godot to close. Its
CLI staging primitive is:

```bash
fennara update --prepare --project path/to/your-godot-project
```

Preparation downloads and verifies manifest-backed release assets, validates
the packaged addon, and copies it to an operation-specific directory under
`.godot/fennara-update/`. It records `ready_to_close` only after the staging
receipt and a digest covering every staged addon file are durable. Preparation does not replace `addons/fennara`, switch
`current.json`, or restart the running daemon.

The chat update action runs this preparation command and displays native
progress in the Godot dock. Once the operation reaches `ready_to_close`, the
dock asks the user to choose **Close Godot and Install** or **Not Now**. On
confirmation, a detached CLI waits for the exact Godot process to exit, moves
the current addon into the operation directory as `previous-addon`, renames the
verified staged addon into `addons/fennara`, activates the matching runtime and
shared launchers, and reopens the same executable and project. Immediately
before replacement, the CLI recomputes the full staged-addon digest and rejects
any changed or missing file.

The previous addon, shared launchers, and runtime manifest remain available until the reopened
GDExtension writes its activation handshake and the CLI confirms the matching
daemon. Failed validation records `recovery_required`; the dock then offers
**Restore Previous Version**, **Open Logs**, and **Copy Report**.

If a machine loses power during the brief addon replacement and the addon
cannot load far enough to show the recovery panel, close Godot and run:

```bash
fennara recover --project path/to/your-godot-project
```

The installed CLI finds the newest interrupted operation, restores its addon,
shared launchers, and runtime manifest, then reopens the recorded Godot
executable when it is still available. Pass `--operation <operation-id>` to
select a specific interrupted operation.

## Troubleshooting

### An Install Or Update Failed

`fennara install`, `fennara update`, and CLI self-update operations print an
operation ID and durable event-log path. Show the latest sanitized report with:

```bash
fennara diagnostics
```

Use a specific ID when reporting or revisiting an older failure:

```bash
fennara diagnostics --operation <operation-id>
```

Operation state is stored under the Fennara app-data `operations/` directory,
with JSONL events under `logs/operations/`. Reports include the phase, stable
error code, platform, architecture, and component versions known to the CLI.
Downloaded artifacts also record their selected asset name, expected hash,
actual hash, and verification status. Operation failures use stable typed codes
so support does not depend on matching the wording of an error message.
They replace the project, home, and Fennara app-data paths with placeholders
and redact common credential fields, bearer tokens, and URL query strings.
They do not collect chat messages, provider keys, or project file contents.

### `fennara` Is Not Found

Open a new terminal and try again:

```bash
fennara doctor
```

`doctor` also reports when a running Fennara daemon or MCP runtime appears to be
older than the version selected by `current.json`; restart Godot or the MCP app
when it prints that warning.

If it still fails, add the Fennara `bin` directory to PATH manually.

Default paths:

```text
Windows: %LOCALAPPDATA%\Fennara\bin
macOS: ~/Library/Application Support/Fennara/bin
Linux: ~/.local/share/fennara/bin
```

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
