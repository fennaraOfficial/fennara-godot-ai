<!-- fennara-i18n: locale=pt-BR source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# Comandos de barra do chat integrado

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · **Português do Brasil** · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

Comandos de barra são atalhos no dock de chat do Fennara dentro do Godot. São comandos da interface, não ferramentas MCP nem prompts enviados ao modelo.

Digite `/` no compositor para abrir a paleta de comandos.

| Comando | Abre | Use para |
| --- | --- | --- |
| `/provider` | Seletor de provedores | Conectar um provedor na nuvem, configurar a URL de um provedor local ou trocar de provedor. |
| `/model` | Seletor de modelos | Escolher um modelo do provedor atual ou conectado. |

<a id="how-they-behave"></a>
## Como se comportam

- Use as teclas de seta para percorrer as sugestões de comandos.
- Pressione Enter para executar o comando selecionado.
- Pressione Escape para fechar a paleta de comandos.
- O texto do comando de barra é removido do compositor antes que a mensagem de chat seja enviada.

<a id="common-flow"></a>
## Fluxo comum

Para o dock de chat integrado:

```text
/provider
```

Conecte OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, Ollama local ou LM Studio.

Depois:

```text
/model
```

Escolha o modelo que deseja que o dock use.

Para aplicativos MCP externos, não use esses comandos de barra. Configure o aplicativo com `fennara mcp-setup` e depois peça a ele que use as ferramentas MCP do Fennara.
