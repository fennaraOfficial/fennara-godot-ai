# Multiple Agents And Godot Worktrees

<!-- fennara-doc-nav:start -->
**English** · [简体中文](i18n/zh-CN/multi-agent-worktrees.md) · [Español](i18n/es/multi-agent-worktrees.md) · [Português do Brasil](i18n/pt-BR/multi-agent-worktrees.md) · [日本語](i18n/ja/multi-agent-worktrees.md) · [한국어](i18n/ko/multi-agent-worktrees.md) · [Русский](i18n/ru/multi-agent-worktrees.md) · [Français](i18n/fr/multi-agent-worktrees.md) · [Deutsch](i18n/de/multi-agent-worktrees.md) · [Türkçe](i18n/tr/multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

Run several coding agents against separate repositories or worktrees on one
machine without letting one agent's target choice redirect another. Each
project gets its own Fennara MCP process and connection; all projects share the
same per-user daemon.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

Editing, inspection, validation, and screenshot calls may run concurrently.
Daemon-managed game runs are serialized through one machine-wide Runtime Slot.

## One MCP Connection Per Project

An MCP process selects one stable Project Root when it starts. That MCP Project
Binding is a canonical filesystem identity for the directory containing
`project.godot`; it is not a project name or a Godot process ID.

Use a separate MCP process and connection for each repository or worktree. One
connection may serve multiple agents only when all of them intentionally work
on the same project. Fennara tools do not expose a per-call project selector, so
the model cannot accidentally switch a process to another project.

Each project also needs a connected Fennara-enabled Godot editor. If an editor
closes and reconnects under a new process ID, the existing MCP process resumes
routing when the same Project Root reconnects.

## How A Process Chooses Its Project

The MCP runtime captures its startup working directory and selects its binding
once, in this order:

1. `--project-path <path>` or `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. The nearest ancestor of the startup directory that contains `project.godot`.
4. Legacy-unbound compatibility mode when automatic discovery finds no Godot
   project.

Command-line and environment paths are explicit assertions. An empty,
inaccessible, missing, non-directory, non-Godot, or unsupported path prevents
the MCP server from starting; it never falls back to another project. Relative
paths resolve from the captured startup directory. Prefer an absolute path when
the MCP host's launch directory is unclear.

Fennara does not implicitly consume host-specific workspace variables. An MCP
host may map its own workspace value into `--project-path` or
`FENNARA_PROJECT_PATH`.

## Configure A Project-Bound Connection

`fennara mcp-setup` remains global and project-neutral. Running it inside a
project does not bind every future MCP process to that project. Keep its stable
launcher path, then use the MCP host's project or workspace configuration to
add a binding.

For JSON-style configuration:

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

Or use the environment:

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

For Codex-style TOML:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Configure the next agent in its own project/workspace config with
`/absolute/path/to/worktree-b`. If a host reliably starts a separate MCP
process from each project directory, ancestor discovery can provide the same
binding without an explicit path.

## MCP Host Boundaries

Project-local configuration and startup-directory behavior differ by host:

- VS Code single-folder workspaces may rely on the host's documented child
  working directory, but an explicit project binding is still the clearest
  configuration.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro, and Codex
  can use project/workspace configuration. Use an explicit binding or a
  documented project startup directory when isolation must be guaranteed.
- Claude Desktop and legacy Windsurf/Cascade configuration is global. Their
  default Fennara entry remains legacy-unbound and cannot provide automatic
  project-local isolation. Advanced users may create separately named global
  entries with distinct explicit paths, but must choose the correct entry.

Automatic project-local config generation and new Windsurf/Devin Local support
are outside this workflow.

## Start And Verify The Editors

Each worktree needs its own Fennara-enabled Godot editor. Headless editors can
use separate Godot LSP ports while sharing Fennara's daemon:

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

The LSP ports belong to Godot. Fennara continues to use one shared daemon on
its normal loopback address.

Run `fennara_status` from every agent before concurrent work. Confirm that it
reports:

- routing mode `bound`
- the expected binding source and canonical Project Root
- bound-editor state `connected`
- editor filesystem readiness for that editor

If automatic discovery found no project, status reports `legacy_unbound` and a
concurrency warning. In that compatibility mode, the dock-selected MCP Target
is used first, followed by the sole connected editor. Do not use an unbound
connection for isolated concurrent work.

## Missing And Duplicate Editors

A valid Project Binding stays alive when its editor is absent. Tool calls return
retryable `bound_project_not_connected` until that Project Root reconnects;
they never fall through to the dock target.

Two editors resolving to the same Project Root produce
`ambiguous_project_binding`. Close the duplicate editor or give it a distinct
worktree. Fennara does not choose by process ID, connection order, project name,
or dock target.

Symlink aliases to the same project resolve to the same live filesystem
identity. Retargeting a symlink after MCP startup does not change a binding;
restart that MCP process to bind again.

## Serialized Runtime Sessions

All projects share one machine-wide Runtime Slot for daemon-managed game runs.
When another project is starting or running a session,
`runtime_session.start` returns a successful `busy` domain result with
`availability: "busy"`, `slot_acquired: false`, and a suggested
`retry_after_ms`. It does not expose the
owner, session ID, process ID, scene, logs, queue position, or expected duration.

There is no FIFO queue. Poll with jitter near the suggested retry delay and
treat each `runtime_session.start` as the final atomic claim. A free status is
only advisory because another agent may win the race after preflight.

The owning Project Root alone may inspect, renew, script, or stop its Runtime
Session. Owner status renews a 120-second inactivity deadline. A bounded owner
runtime operation suspends inactivity expiry while active and renews the deadline
only after returning a terminal script result; timeout, setup error, or
cancellation does not renew it. Agents should poll owner status about every 30
seconds with jitter while a run proceeds.

The default absolute Runtime Lease is 900 seconds. `max_run_seconds` accepts a
positive integer up to 86,400 seconds; for example, an expected one-hour
regression can request 4,500 seconds for a safety margin. The absolute deadline
is never suspended. Natural exit, explicit stop, startup failure, inactivity,
or absolute expiry stops or reaps the game and releases the Runtime Slot.

## Safe Multi-Agent Checklist

1. Create a distinct repository or worktree for each project.
2. Install Fennara and open one Godot editor for each Project Root.
3. Configure one project-bound MCP process per project.
4. Run `fennara_status` from every agent and verify its canonical root.
5. Let editing, inspection, bounded scene validation, and standalone screenshots
   proceed concurrently.
6. Poll and retry non-error `busy` results for playtests; keep the winning
   session alive with owner status while it runs.
