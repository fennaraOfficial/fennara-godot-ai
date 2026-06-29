# Built-In Chat Providers

This page is for the Fennara chat dock inside Godot.

External MCP apps are different. Claude Code, Claude Desktop, Codex, Cursor, Gemini, and Antigravity use their own model setup when they call Fennara MCP tools. See [MCP Apps And Built-In Chat](chat-vs-mcp.md) for that distinction.

## Supported Providers

| Provider | Type | Model Id Shape | Notes |
| --- | --- | --- | --- |
| OpenAI | Cloud API key | `openai/<model>` | Uses OpenAI's official API with `OPENAI_API_KEY`. |
| Anthropic | Cloud API key | `anthropic/<model>` | Uses Anthropic's official Messages API with `ANTHROPIC_API_KEY`. |
| OpenRouter | Cloud API key | `openrouter/<model>` or legacy `<provider>/<model>` | Existing OpenRouter model ids such as `google/gemini...` remain accepted. |
| Ollama Cloud | Cloud API key | `ollama-cloud/<model>` | Uses Ollama's hosted OpenAI-compatible API. |
| DeepSeek | Cloud API key | `deepseek/<model>` | Uses DeepSeek's OpenAI-compatible API. |
| Z.AI | Cloud API key | `zai/<model>` | Uses Z.AI's OpenAI-compatible API. |
| Moonshot AI | Cloud API key | `moonshotai/<model>` | Uses Moonshot's OpenAI-compatible API. |
| Moonshot AI (China) | Cloud API key | `moonshotai-cn/<model>` | Uses Moonshot China's OpenAI-compatible API. |
| Kimi For Coding | Cloud API key | `kimi-for-coding/<model>` | Uses Kimi's Anthropic-compatible Messages API. |
| MiniMax | Cloud API key | `minimax/<model>` | Uses MiniMax's Anthropic-compatible Messages API. |
| MiniMax Token Plan | Cloud API key | `minimax-coding-plan/<model>` | Uses MiniMax's Anthropic-compatible Messages API. |
| MiniMax (China) | Cloud API key | `minimax-cn/<model>` | Uses MiniMax China's Anthropic-compatible Messages API. |
| MiniMax Token Plan (China) | Cloud API key | `minimax-cn-coding-plan/<model>` | Uses MiniMax China's Anthropic-compatible Messages API. |
| Ollama | Local server | `ollama/<local-model>` | Defaults to `http://127.0.0.1:11434`. |
| LM Studio | Local server | `lmstudio/<local-model>` | Defaults to `http://127.0.0.1:1234/v1`. |

Cloud providers need your own API key. Local providers need the local server running with a model available.

Native provider prefixes take precedence. For example, `openai/gpt-...` uses the official OpenAI provider and `anthropic/claude-...` uses the official Anthropic provider. To use those vendors through OpenRouter, select the explicit `openrouter/openai/...` or `openrouter/anthropic/...` model id.

## Where Settings Live

Fennara stores built-in chat settings locally through the daemon, outside the Godot project:

- provider API keys
- local provider base URLs
- selected model
- reasoning effort
- chat display mode, either embedded in Godot or opened in the system browser
- chat history

These settings are not written into `res://addons/fennara/` and are not shared with Claude, Codex, Cursor, Gemini, or other external MCP apps.

## Chat Display Setting

The Chat Settings dialog includes **Open chat in my system browser next time**.

When this is off, Fennara tries to render the built-in chat inside the Godot dock. When it is on, the dock shows an **Open chat** button and launches the same built-in chat through the local daemon at `127.0.0.1`. This can reduce Godot editor GPU and memory usage and is also the fallback path if the native webview cannot start.

Changing this setting takes effect the next time Godot starts. It only changes where the built-in chat UI is displayed; it does not change the selected provider, model, API keys, chat history, MCP app setup, or which model Claude/Codex/Cursor use externally.

## Choosing A Provider And Model

Inside the Fennara dock:

1. Use `/provider` to open the provider picker.
2. Add an API key for a cloud provider, or configure a local provider base URL.
3. Use `/model` to choose a model from the connected provider.

You can also open the same provider and model pickers from the dock controls.

See [Built-In Chat Slash Commands](slash-commands.md) for command palette behavior.

## Local Providers

For Ollama:

```bash
ollama serve
ollama pull llama3.1:8b
```

Then choose:

```text
ollama/llama3.1:8b
```

For LM Studio, start the local server from LM Studio and choose a model id shaped like:

```text
lmstudio/<loaded-model-id>
```

## Model Catalog

The daemon keeps a local model catalog for cloud providers and asks local servers for their currently available models. If a catalog or local server changes while Godot is open, refresh the model picker or reopen the provider/model picker.

Fennara checks basic model capabilities before sending a request:

- text output is required
- tool calling is required for Fennara tool use
- image input is required before image attachments are sent as image context

Ollama image input is not enabled yet in Fennara chat.
