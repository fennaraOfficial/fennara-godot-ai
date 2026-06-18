# Architecture

Fennara connects AI coding agents to real Godot feedback.

```text
AI coding app
  -> Fennara MCP server
  -> Fennara daemon
  -> Godot plugin
  -> Godot editor/project
```

## Responsibilities

| Area | Responsibility |
| --- | --- |
| AI coding app | Starts the local MCP server and calls Fennara tools. |
| Fennara MCP server | Speaks MCP over stdio and forwards tool calls locally. |
| Fennara daemon | Maintains the local bridge, runtime helpers, and Godot-facing coordination. |
| Godot plugin | Inspects and edits Godot state through Godot APIs, then returns concise feedback. |
| Godot project | The user's actual scenes, scripts, resources, settings, and runtime state. |

## Design Principles

- Inspect first, edit second, validate third.
- Keep tools primitive and game-agnostic.
- Prefer Godot-aware inspection over guessing from files.
- Return useful feedback to the agent, including diagnostics, validation results, runtime errors, screenshots, and relevant context.
- Keep model-facing results concise.
