# Built-In Chat Providers

This page is for the Fennara chat dock inside Godot.

External MCP apps are different. Claude Code, Claude Desktop, Codex, Cursor, Gemini, and Antigravity use their own model setup when they call Fennara MCP tools. See [MCP Apps And Built-In Chat](chat-vs-mcp.md) for that distinction.

## Supported Providers

| Provider | How To Connect | Model Id Shape | Notes |
| --- | --- | --- | --- |
| OpenAI | Create a key in [OpenAI API keys](https://platform.openai.com/api-keys). Fennara key/env: `OPENAI_API_KEY`. | `openai/<model>` | Uses OpenAI's official API. |
| Anthropic | Create a key in [Claude Console API keys](https://console.anthropic.com/settings/keys). Fennara key/env: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Uses Anthropic's official Messages API. |
| OpenRouter | Create a key in [OpenRouter Keys](https://openrouter.ai/settings/keys). Fennara key/env: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | Use this when you want OpenRouter to route the request. See [OpenRouter Model Ids](#openrouter-model-ids). |
| Ollama Cloud | Create a key in [Ollama API keys](https://ollama.com/settings/keys). Fennara key/env: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Uses Ollama's hosted API, not the local Ollama server. |
| DeepSeek | Create a key in [DeepSeek API keys](https://platform.deepseek.com/api_keys). Fennara key/env: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | Uses DeepSeek's OpenAI-compatible API. |
| Z.AI | Create a key in [Z.AI API keys](https://z.ai/manage-apikey/apikey-list). Fennara key/env: `ZHIPU_API_KEY`. | `zai/<model>` | Uses Z.AI's OpenAI-compatible API. |
| Moonshot AI | Create a key in [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys). Fennara key/env: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Uses Moonshot's OpenAI-compatible API. |
| Moonshot AI (China) | Create a key in [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys). Fennara key/env: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Uses Moonshot China's OpenAI-compatible API. |
| Kimi For Coding | Create a key in the [Kimi Code Console](https://www.kimi.com/code/console). Fennara key/env: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Uses Kimi's Anthropic-compatible Messages API. Requires Kimi Code access. |
| MiniMax | Create a pay-as-you-go key from [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) **API Keys > Create new secret key**. Fennara key/env: `MINIMAX_API_KEY`. | `minimax/<model>` | Uses MiniMax's Anthropic-compatible Messages API at `minimax.io`. |
| MiniMax Token Plan | Use the Subscription Key from [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) **Billing > Token Plan**. Fennara key/env: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Token Plan Subscription Keys are separate from pay-as-you-go API keys. |
| MiniMax (China) | Create a pay-as-you-go key from the [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) API key page. Fennara key/env: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | Uses MiniMax China's Anthropic-compatible Messages API at `minimaxi.com`. |
| MiniMax Token Plan (China) | Use the Subscription Key from the [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) Token Plan page. Fennara key/env: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | China Token Plan Subscription Keys are separate from pay-as-you-go API keys. |
| Ollama | Run a local Ollama server. No cloud API key is required. | `ollama/<local-model>` | Defaults to `http://127.0.0.1:11434`. |
| LM Studio | Start LM Studio's local server. No key is required by default. | `lmstudio/<local-model>` | Defaults to `http://127.0.0.1:1234/v1`. If your LM Studio server requires auth, set `LMSTUDIO_API_KEY` in the daemon environment. |

Cloud providers need your own API key or subscription key. Local providers need the local server running with a model available.

Fennara can store keys from the provider picker in the dock. Chat Settings includes an **Open providers** button for discovering the same picker. The key/env names above are the same names Fennara understands if you prefer environment variables. Stored keys live in the daemon's local app data, outside the Godot project.

## OpenRouter Model Ids

OpenRouter model ids often already contain a provider slug. In Fennara, prefer the explicit OpenRouter prefix:

```text
openrouter/google/gemini-...
openrouter/anthropic/claude-...
openrouter/openai/gpt-...
```

If you paste a raw OpenRouter slug such as `google/gemini-...`, Fennara still sends it to OpenRouter for compatibility. Native Fennara prefixes win, though: `openai/gpt-...` uses the official OpenAI provider, and `anthropic/claude-...` uses the official Anthropic provider. To use those vendors through OpenRouter, choose `openrouter/openai/...` or `openrouter/anthropic/...`.

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

1. Open **Chat Settings** and click **Open providers**, or use `/provider` directly.
2. Choose a provider and add the connection details it requires, such as an API key for a cloud provider or a base URL for a local provider.
3. Use the model picker to choose a model from the connected provider.

Chat Settings, the dock controls, and `/provider` all open the same registry-backed provider picker. Use `/model` or the dock model control to open the model picker.

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
