# Fennara CLI

The Fennara CLI is the shared installer and maintenance layer behind both the
Godot setup panel and terminal workflows. The addon bootstrap downloads the
matching CLI, verifies it, and asks it to finish setup. Users who prefer a
terminal can run the same operations directly.

Use [Setup](setup.md) for the normal Godot user journey. This page is the CLI
reference.

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

```text
fennara doctor [--repair]
fennara diagnostics [--operation <operation-id>] [--json]
fennara install [--project <path>] [--version <version>]
fennara mcp-setup <target flags>
fennara update [--project <path>] [--version <version>] [--no-self-update] [--prepare]
fennara recover --project <path> [--operation <operation-id>]
fennara self-update [--version <version>]
```

Run `fennara <command> --help` for the options supported by the installed CLI.

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
fennara install --project path/to/project --version 0.3.8
```

Installation has two safe paths:

- If no complete addon exists, the CLI downloads and verifies the selected
  release, installs `addons/fennara`, installs the matching local components,
  and writes Fennara project guidance.
- If a complete addon already exists, the CLI reads its `VERSION`, validates
  the current platform library, and installs that exact version's CLI-managed
  components. It keeps the project addon unchanged. An explicit `--version`
  must match the existing addon.

This second path is what the Godot **Set Up Fennara** panel uses after it has
bootstrapped the CLI.

## Update A Project

For a normal terminal update, close Godot for that project and run:

```bash
fennara update --project path/to/project
```

The CLI resolves the selected release, self-updates when the release requires a
newer CLI, verifies the release assets, refreshes the addon and versioned local
components, updates project guidance, and checks the platform webview
prerequisite. Use `--version <version>` to select an exact release.

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
