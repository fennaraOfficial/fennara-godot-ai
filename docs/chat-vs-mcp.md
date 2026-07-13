# MCP Apps Or Built-In Chat?

Fennara supports both. Choose where you want the conversation to happen.

| | External MCP app | Built-in Fennara chat |
| --- | --- | --- |
| Where you chat | Codex, Claude, Cursor, Gemini, or another MCP app | The Fennara dock or your system browser |
| Model account | The external app's account or subscription | A provider connected in Fennara Chat Settings |
| What Fennara adds | Godot-aware MCP tools | Chat UI, the same core Godot tools, and chat-only file and shell tools |
| Setup | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> You can use both paths. Their model settings remain separate.

## External MCP Apps

Connecting an MCP app lets that app start the local Fennara MCP server and call
Godot tools. It does not share the app's subscription or login with the
built-in chat.

Set up an app from **Chat Settings > MCP Apps**, or use the CLI:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

No Fennara chat provider key is required. Restart the external app after setup.
See [MCP Setup](mcp-setup.md) for every target and manual configuration.

## Built-In Chat

The built-in chat needs a provider connected in Fennara Chat Settings. Use your
own key for a cloud provider, or connect a local Ollama or LM Studio server.

The same chat can appear inside the Godot dock or in your system browser. This
display choice does not change its provider, model, history, or project.

To attach code, select it in Godot's script editor, open the context menu, and
choose **Add to Chat**. See [Built-In Chat Providers](providers.md) for provider
and model setup.

## Project Routing

Both paths use the local Fennara daemon for Godot feedback.

- External MCP calls go to the project selected by the dock's **MCP target**
  control.
- Built-in chat stays bound to the Godot editor that opened the chat.

To verify an external MCP connection, ask:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```
