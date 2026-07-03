# MCP Setup

Use this guide when connecting Fennara to an external MCP app such as Claude
Code, Claude Desktop, Codex, Cursor, Cline, VS Code, Gemini, Antigravity,
OpenCode, Windsurf, Kiro, or another MCP client.

External MCP apps use their own model account, subscription, or API setup.
Fennara supplies the local Godot-aware tools. The built-in Fennara chat dock is
configured separately inside Godot.

## Preferred Setup

Run `fennara install` inside your Godot project first. This installs the Godot
addon, downloads the local runtime package, and creates the stable MCP launcher
that external apps should call.

Then configure your MCP app:

```bash
fennara mcp-setup --claude
fennara mcp-setup --codex
fennara mcp-setup --cursor
fennara mcp-setup --gemini
```

Other supported targets:

```bash
fennara mcp-setup --claude-code
fennara mcp-setup --claude-desktop
fennara mcp-setup --cline
fennara mcp-setup --vscode
fennara mcp-setup --opencode
fennara mcp-setup --windsurf
fennara mcp-setup --kiro
fennara mcp-setup --help
```

Restart the MCP app after setup so it reloads Fennara.

## Manual Setup

Use manual setup only when your app is not listed, the setup command cannot find
the app's config file, or you intentionally want to edit MCP config by hand.

Before editing, make a backup of the config file. Then add a local stdio MCP
server named `fennara` that points at the stable Fennara MCP launcher.

Default launcher paths:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Use the real absolute path on your machine. Do not point MCP apps at
`versions/<version>/fennara-mcp-runtime`; the stable launcher in `bin/` keeps app
configs working across Fennara updates.

### JSON `mcpServers`

Many MCP apps use a top-level `mcpServers` object:

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

Some apps use the same `mcpServers` key but only require `command`. If the
existing config already has other servers, preserve those entries and add only
the `fennara` server.

Cline-style configs can also include a longer tool timeout in seconds:

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

### VS Code-Style JSON `servers`

Some clients, including VS Code user or project MCP config, use a top-level
`servers` object and require `type: "stdio"`:

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

### OpenCode-Style JSON `mcp`

OpenCode-style JSON config uses a top-level `mcp` object. Its timeout is in
milliseconds:

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

### Codex-Style TOML

Codex uses TOML:

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Do not paste JSON into a TOML file or TOML into a JSON file. Match the format
already used by the app.

## Common Config Locations

These are common locations used by Fennara's setup helper and by current MCP
clients. Apps can change their config paths, and some support both global and
project-local configs. If an app has a command such as **Open MCP Config**, use
that instead of guessing.

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

## Timeout Guidance

Some Fennara tools may take longer than a small default MCP timeout because they
can ask Godot to validate scenes, inspect runtime state, capture screenshots, or
run diagnostics.

Use a longer per-tool timeout when the client supports it:

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

If a client does not support per-server timeouts, use that client's documented
global MCP timeout setting.

## Verify The Connection

Open the Godot project, then ask your MCP app:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

If more than one Godot project is open, use the Fennara dock's **MCP target**
control to select which project receives external MCP tool calls.

## Troubleshooting

If Fennara does not appear in the MCP app:

- confirm the launcher path is absolute and exists
- confirm the config syntax is valid JSON, JSON5, or TOML as required by the app
- confirm the server is named `fennara`
- confirm the app is reading the config file you edited
- fully quit and reopen the MCP app
- confirm the Godot project has the Fennara addon installed
- confirm the intended Godot project is selected as the MCP target

## Unsupported MCP Apps

If your MCP app is not listed, find that app's official MCP config location and
format first. Then ask an LLM for the smallest safe edit:

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

Review the result before saving, then restart the MCP app.
