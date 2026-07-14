# Fennara CLI

Use the CLI when you prefer the terminal, need diagnostics or recovery, or want
an automated install with an exact version.

> [!TIP]
> You do not need to install the CLI manually if **Set Up Fennara** already
> completed in the Godot dock.

## Common Flow

```bash
cd path/to/your-godot-project
fennara install
```

Use `fennara doctor` when you need to inspect or repair the local installation.

Use [Setup](setup.md) for the normal Godot journey. Keep this page as the
terminal command reference.

## Install The CLI

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS and Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Open a new terminal if `fennara` is not immediately available, then check the
installation:

```bash
fennara --version
fennara doctor
```

The CLI is installed per user. Project addons stay inside their Godot projects;
shared launchers, versioned runtimes, operation records, logs, and Linux CEF
stay in Fennara app data:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

## Command Summary

| Command | Purpose |
| --- | --- |
| `fennara install` | Install or adopt a project addon and its matching local components |
| `fennara update` | Update a project and its local components |
| `fennara doctor` | Inspect or repair the local installation |
| `fennara diagnostics` | Show a sanitized operation report |
| `fennara mcp-setup` | Connect an external MCP app |
| `fennara recover` | Restore an interrupted native update |
| `fennara self-update` | Update only the installed CLI |

Run `fennara --help` for the installed command summary. Use
`fennara mcp-setup --help` for the supported MCP app targets.

## Install A Project

Run inside a folder containing `project.godot`:

```bash
fennara install
```

Or identify the project explicitly:

```bash
fennara install --project path/to/project
```

Without `--version`, the CLI selects the current release manifest. Use an exact
release when reproducibility matters:

```bash
fennara install --project path/to/project --version <version>
```

Installation has two safe paths:

- If no complete addon exists, the CLI downloads and verifies the selected
  release, installs `addons/fennara`, installs the matching local components,
  and writes Fennara project guidance.
- If a complete addon already exists, the CLI reads its `VERSION`, validates
  the current platform library, and installs that exact version's CLI-managed
  components. It keeps the project addon unchanged. An explicit `--version`
  must match the existing addon.

## Update A Project

For a normal terminal update, close Godot for that project and run:

```bash
fennara update --project path/to/project
```

Without `--version`, the CLI reads the installed addon identity. Stable addons
resolve stable latest, while staging addons resolve only their `pr-<number>`
channel. The moving selector is immediately frozen to one exact immutable
version, including across CLI self-replacement. The CLI then verifies the
release assets, refreshes the addon and versioned local components, updates
project guidance, and checks the platform webview prerequisite. Use
`--version <version>` to select an exact release explicitly.

`--no-self-update` is intended for controlled automation or continuation after
the CLI has already been replaced. Do not use it to bypass a release's minimum
CLI requirement.

### Prepare While Godot Is Open

The in-editor update button uses the staging form:

```bash
fennara update --prepare --project path/to/project
```

Preparation downloads, verifies, and durably stages the addon. It does not
close Godot, replace the live addon, switch the active runtime manifest, or
restart the daemon. The Godot dock observes the operation receipt and asks the
user before starting the detached close, replace, reopen, and validation step.
The dock passes the exact version it already discovered, so pointer movement
cannot change an in-progress update.

Fennara supports one active shared runtime version at a time. Activation is
blocked if another Fennara-enabled Godot editor remains connected to the shared
daemon. Close the other editor, then retry. The previous local version and
runtime pointer remain available for recovery without network access.

`--prepare` is a low-level primitive for the Godot integration. Terminal users
normally use `fennara update` with Godot already closed.

## Recover An Interrupted Update

If the updated addon cannot load far enough to show the recovery panel, close
Godot and run:

```bash
fennara recover --project path/to/project
```

The CLI restores only operations in a recoverable state. It restores the
previous addon, shared launchers, and active runtime manifest, then attempts to
reopen the recorded Godot executable. Select a particular transaction when
support gives you its operation ID:

```bash
fennara recover --project path/to/project --operation <operation-id>
```

Completed, merely prepared, and already rolled-back operations are rejected.

## Inspect Health And Failures

`doctor` reports the detected platform, app-data layout, active version,
launchers, runtimes, daemon state, and webview prerequisite:

```bash
fennara doctor
```

If it reports a running daemon or MCP runtime older than `current.json`, restart
Godot or the affected MCP app so it launches the selected runtime.

Use `--repair` to recreate missing base app-data directories. On Linux it also
cleans stale CEF process profiles and repairs the current-runtime marker when a
complete managed runtime is already installed:

```bash
fennara doctor --repair
```

Install, update, recovery, and self-update operations write durable state and
events. Show the newest sanitized report with:

```bash
fennara diagnostics
```

For an older operation or machine-readable output:

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

Reports include stable error codes, phases, component versions, selected asset
names, and hash verification results. They redact project, home, and Fennara
app-data paths, credentials, bearer tokens, and URL queries. They do not include
chat messages, provider keys, or project file contents.

## Configure An External MCP App

The Godot chat dock exposes these commands under **Chat Settings > MCP Apps**.
Its Set Up button asks the local daemon to invoke the installed CLI, so the dock
and terminal workflows use the same configuration and backup implementation.

Run `fennara mcp-setup --help` to choose a supported target. Restart the MCP app
after changing its configuration. This command connects an external app to the
Fennara MCP server; it does not select the model provider used by the built-in
Godot chat dock. [MCP Setup](mcp-setup.md) owns the target list, config
locations, and manual configuration examples.

## Update Only The CLI

Normal project updates handle CLI self-update automatically. To update only the
installed CLI:

```bash
fennara self-update
fennara self-update --version <version>
```

Without `--version`, self-update preserves the active installation track:
stable uses stable latest, and staging uses only its recorded PR channel.

Staging never crosses into stable automatically. To leave staging deliberately,
close Godot and run `fennara update --version <stable-version> --project <path>`.
That exact stable release is validated before the shared active version changes.

Use this when support requests it or when a project update reports that the
installed CLI is too old to continue safely.

## Automation Guidance

- Pass `--project` instead of relying on the current directory.
- Pin `--version` when a build must be reproducible.
- Preserve the printed operation ID and log path on failure.
- Use `fennara diagnostics --operation <id> --json` for structured reporting.
- Do not edit `current.json`, version directories, update receipts, or staged
  addon folders by hand.
- Do not run a normal addon-replacing update while that project is open in
  Godot. Use the in-editor update flow or close Godot first.
