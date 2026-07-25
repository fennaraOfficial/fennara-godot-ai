<!-- fennara-i18n: locale=zh-CN source=docs/providers.md sha256=cb9c3750ebffd837ab24519959fdac8f39ba41cc32b1a8e0c467c500b56f7cb5 -->
<a id="built-in-chat-providers"></a>
# 内置聊天提供方

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · **简体中文** · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../providers.md)
<!-- fennara-doc-nav:end -->

将模型提供方连接到 Godot 内的 Fennara 聊天停靠面板。

> [!NOTE]
> 外部 MCP 应用使用各自的模型设置。要从 Codex、Claude、Cursor 或其他
> MCP 应用使用 Fennara，无需在这里连接提供方。
> 请参阅 [MCP 应用与内置聊天](chat-vs-mcp.md)。

<a id="quick-setup"></a>
## 快速设置

1. 在 Fennara 停靠面板中打开 **Chat Settings > Chat**。
2. 选择 **Open providers**。
3. 选择一个云端提供方并输入你自己的密钥，或选择 Ollama 或 LM
   Studio 以使用本地模型。
4. 选择一个模型。

你也可以在输入框中键入 `/provider` 和 `/model`。

<a id="provider-reference"></a>
## 提供方参考

| 提供方 | 连接方式 | 模型 ID 形式 | 说明 |
| --- | --- | --- | --- |
| OpenAI | 在 [OpenAI API keys](https://platform.openai.com/api-keys) 中创建密钥。Fennara 密钥或环境变量：`OPENAI_API_KEY`。 | `openai/<model>` | 使用 OpenAI 官方 API。 |
| Anthropic | 在 [Claude Console API keys](https://console.anthropic.com/settings/keys) 中创建密钥。Fennara 密钥或环境变量：`ANTHROPIC_API_KEY`。 | `anthropic/<model>` | 使用 Anthropic 官方 Messages API。 |
| OpenRouter | 在 [OpenRouter Keys](https://openrouter.ai/settings/keys) 中创建密钥。Fennara 密钥或环境变量：`OPENROUTER_API_KEY`。 | `openrouter/<provider>/<model>` | 使用 OpenRouter API。 |
| Ollama Cloud | 在 [Ollama API keys](https://ollama.com/settings/keys) 中创建密钥。Fennara 密钥或环境变量：`OLLAMA_API_KEY`。 | `ollama-cloud/<model>` | 使用 Ollama 托管 API，而不是本地 Ollama 服务器。 |
| DeepSeek | 在 [DeepSeek API keys](https://platform.deepseek.com/api_keys) 中创建密钥。Fennara 密钥或环境变量：`DEEPSEEK_API_KEY`。 | `deepseek/<model>` | 使用 DeepSeek 的 OpenAI 兼容 API。 |
| Z.AI | 在 [Z.AI API keys](https://z.ai/manage-apikey/apikey-list) 中创建密钥。Fennara 密钥或环境变量：`ZHIPU_API_KEY`。 | `zai/<model>` | 使用 Z.AI 的 OpenAI 兼容 API。 |
| Moonshot AI | 在 [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys) 中创建密钥。Fennara 密钥或环境变量：`MOONSHOT_API_KEY`。 | `moonshotai/<model>` | 使用 Moonshot 的 OpenAI 兼容 API。 |
| Moonshot AI (China) | 在 [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys) 中创建密钥。Fennara 密钥或环境变量：`MOONSHOT_API_KEY`。 | `moonshotai-cn/<model>` | 使用 Moonshot China 的 OpenAI 兼容 API。 |
| Kimi For Coding | 在 [Kimi Code Console](https://www.kimi.com/code/console) 中创建密钥。Fennara 密钥或环境变量：`KIMI_API_KEY`。 | `kimi-for-coding/<model>` | 使用 Kimi 的 Anthropic 兼容 Messages API。需要 Kimi Code 访问权限。 |
| MiniMax | 在 [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) 的 **API Keys > Create new secret key** 中创建按量付费密钥。Fennara 密钥或环境变量：`MINIMAX_API_KEY`。 | `minimax/<model>` | 使用位于 `minimax.io` 的 MiniMax Anthropic 兼容 Messages API。 |
| MiniMax Token Plan | 使用 [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) 中 **Billing > Token Plan** 下的 Subscription Key。Fennara 密钥或环境变量：`MINIMAX_API_KEY`。 | `minimax-coding-plan/<model>` | Token Plan Subscription Key 与按量付费 API 密钥彼此独立。 |
| MiniMax (China) | 从 [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) API 密钥页面创建按量付费密钥。Fennara 密钥或环境变量：`MINIMAX_API_KEY`。 | `minimax-cn/<model>` | 使用位于 `minimaxi.com` 的 MiniMax China Anthropic 兼容 Messages API。 |
| MiniMax Token Plan (China) | 使用 [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) Token Plan 页面中的 Subscription Key。Fennara 密钥或环境变量：`MINIMAX_API_KEY`。 | `minimax-cn-coding-plan/<model>` | 中国区 Token Plan Subscription Key 与按量付费 API 密钥彼此独立。 |
| NVIDIA | 在 [build.nvidia.com](https://build.nvidia.com/) 创建密钥。Fennara 密钥或环境变量：`NVIDIA_API_KEY`。 | `nvidia/<publisher>/<model>` | 使用 NVIDIA 的 OpenAI 兼容托管 NIM API。 |
| Ollama | 运行本地 Ollama 服务器。不需要云端 API 密钥。 | `ollama/<local-model>` | 默认地址为 `http://127.0.0.1:11434`。 |
| LM Studio | 启动 LM Studio 本地服务器。默认不需要密钥。 | `lmstudio/<local-model>` | 默认地址为 `http://127.0.0.1:1234/v1`。如果你的 LM Studio 服务器需要身份验证，请在守护进程环境中设置 `LMSTUDIO_API_KEY`。 |

云端提供方需要你自己的 API 密钥或订阅密钥。本地提供方需要
本地服务器正在运行且有可用模型。

OpenRouter 选择始终使用明确的 `openrouter/<provider>/<model>`
形式。设置加载时，较早保存的 `<provider>/<model>` OpenRouter 选择会迁移一次，
但新的路由不再使用这种旧形式。

Fennara 可以存储你在停靠面板的提供方选择器中输入的密钥。Chat Settings 中有一个 **Open providers** 按钮，用于打开同一个选择器。若你更愿意使用环境变量，上方列出的密钥或环境变量名称就是 Fennara 能识别的名称。存储的密钥位于守护进程的本地应用数据中，在 Godot 项目之外。

<a id="custom-openai-compatible-providers"></a>
## 自定义 OpenAI 兼容提供方

在提供方选择器底部选择 **Custom**，即可添加一个 OpenAI 兼容
端点，例如本地路由器或内部 API 网关。请输入：

- 唯一的小写提供方 ID
- Fennara 中显示的名称
- 以 API 版本结尾的基础 URL，例如 `http://localhost:20128/v1`
- 可选的 API 密钥
- 一个或多个模型 ID、显示名称、上下文长度和最大输出 token 限制
- 可选的请求标头

模型 ID 必须与端点所期望的值一致。Fennara 在模型选择器中将它们显示为
`<provider-id>/<model-id>`，同时只向提供方发送 `<model-id>`。
端点必须实现 OpenAI 兼容的
`/chat/completions` 请求和流式响应格式。

API 密钥和自定义标头值使用 Fennara 受保护的守护进程身份验证存储。
提供方定义保存在 Godot 项目之外、由守护进程管理的本地应用数据中。
准确的模型限制可让 Fennara
在请求超过模型上下文窗口之前压缩对话历史，
并使生成的摘要保持在模型输出限制之内。在这些字段出现之前保存的
现有自定义模型会使用兼容性默认值加载，即 64,000 个上下文 token 和 4,096 个输出 token。

保存后，自定义提供方会连同其模型数量一起出现在提供方选择器中。
选择该提供方即可重新打开表单，并添加模型或重命名模型。将
API 密钥留空会保留已保存的密钥，任何新输入的标头都会按名称与
已保存的标头合并。

<a id="where-settings-live"></a>
## 设置存储位置

Fennara 通过守护进程在 Godot 项目之外本地存储内置聊天设置：

- 提供方 API 密钥
- 自定义提供方标头值
- 自定义 OpenAI 兼容提供方定义
- 本地提供方基础 URL
- 选定模型
- 推理强度
- 提供方响应超时
- 聊天显示模式，即嵌入 Godot 或在系统浏览器中打开
- 聊天历史

这些设置不会写入 `res://addons/fennara/`，也不会与 Claude、Codex、Cursor、Gemini 或其他外部 MCP 应用共享。

<a id="provider-response-timeout"></a>
## 提供方响应超时

**Provider response timeout** 设置控制内置聊天等待每个模型请求完成的时间。默认值为 120 秒，可设置为 30 至 3600 秒。增大该值有助于较慢的本地模型或大量使用工具的长轮次顺利完成。守护进程会将选定的超时应用于提供方请求，并在达到限制时取消请求。

<a id="chat-display-setting"></a>
## 聊天显示设置

Chat Settings 对话框包含 **Open chat in my system browser next time**。

关闭时，Fennara 会尝试在 Godot 停靠面板中渲染内置聊天。开启时，停靠面板会显示 **Open chat** 按钮，并通过位于 `127.0.0.1` 的本地守护进程启动相同的内置聊天。这可以降低 Godot 编辑器的 GPU 和内存用量，也是在原生 webview 无法启动时的回退路径。

更改此设置会在下次启动 Godot 时生效。它只会更改内置聊天 UI 的显示位置，不会更改选定的提供方、模型、API 密钥、聊天历史、MCP 应用设置，也不会更改 Claude、Codex 或 Cursor 在外部使用的模型。

<a id="picker-shortcuts"></a>
## 选择器快捷方式

Chat Settings、停靠面板控件和 `/provider` 会打开同一个提供方选择器。
使用 `/model` 或停靠面板的模型控件打开模型选择器。

关于命令面板行为，请参阅[内置聊天斜杠命令](slash-commands.md)。

<a id="local-providers"></a>
## 本地提供方

对于 Ollama：

```bash
ollama serve
ollama pull llama3.1:8b
```

然后选择：

```text
ollama/llama3.1:8b
```

较早的 `local/<model>` 选择仍会作为 Ollama 兼容
别名被接受。对于新设置，请优先使用明确的 `ollama/<model>` 形式。

对于 LM Studio，请从 LM Studio 启动本地服务器，并选择一个形式如下的模型 ID：

```text
lmstudio/<loaded-model-id>
```

<a id="model-catalog"></a>
## 模型目录

守护进程为云端提供方维护一个本地模型目录，并向本地服务器查询其当前可用的模型。如果 Godot 运行期间目录或本地服务器发生变化，请刷新模型选择器，或重新打开提供方或模型选择器。

Fennara 会在发送请求之前检查基本模型能力：

- 必须支持文本输出
- 使用 Fennara 工具时必须支持工具调用
- 将图像附件作为图像上下文发送之前，必须支持图像输入

Fennara 聊天尚未启用 Ollama 图像输入。
