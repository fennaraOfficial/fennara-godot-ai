<!-- fennara-i18n: locale=pt-BR source=docs/chat-vs-mcp.md sha256=03cb522aed8f8e305feaca0c2ed51f7ba29b2657a721df4196b15bc6ccf12c9c -->
<a id="mcp-apps-or-built-in-chat"></a>
# Aplicativos MCP ou chat integrado?

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · [Español](../es/chat-vs-mcp.md) · **Português do Brasil** · [日本語](../ja/chat-vs-mcp.md) · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · [Deutsch](../de/chat-vs-mcp.md) · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

O Fennara oferece suporte aos dois. Escolha onde deseja que a conversa aconteça.

| | Aplicativo MCP externo | Chat integrado do Fennara |
| --- | --- | --- |
| Onde você conversa | Codex, Claude, Cursor, Gemini ou outro aplicativo MCP | O dock do Fennara ou o navegador do sistema |
| Conta do modelo | A conta ou assinatura do aplicativo externo | Um provedor conectado em Fennara Chat Settings |
| O que o Fennara acrescenta | Ferramentas MCP com conhecimento do Godot | Interface de chat, as mesmas ferramentas principais do Godot e ferramentas de arquivo e shell exclusivas do chat |
| Configuração | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> Você pode usar os dois caminhos. As configurações de modelo permanecem separadas.

<a id="external-mcp-apps"></a>
## Aplicativos MCP externos

Conectar um aplicativo MCP permite que ele inicie o servidor MCP local do Fennara
e chame ferramentas do Godot. Isso não compartilha a assinatura ou o login do
aplicativo com o chat integrado.

Configure um aplicativo em **Chat Settings > MCP Apps** ou use a CLI:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Não é necessária uma chave de provedor do chat do Fennara. Reinicie o aplicativo
externo após a configuração. Consulte [Configuração de MCP](mcp-setup.md) para
ver todos os destinos e a configuração manual.

<a id="built-in-chat"></a>
## Chat integrado

O chat integrado precisa de um provedor conectado em Fennara Chat Settings. Use
sua própria chave para um provedor na nuvem ou conecte um servidor Ollama ou LM Studio local.

O mesmo chat pode aparecer dentro do dock do Godot ou no navegador do sistema.
Essa escolha de exibição não altera o provedor, o modelo, o histórico nem o projeto.

Para anexar código, selecione-o no editor de scripts do Godot, abra o menu de
contexto e escolha **Add to Chat**. Consulte [Provedores do chat integrado](providers.md)
para configurar o provedor e o modelo.

<a id="project-routing"></a>
## Roteamento de projetos

Os dois caminhos usam o daemon local do Fennara para obter feedback do Godot.

- Chamadas MCP externas vão para o projeto selecionado pelo controle **MCP target**
  do dock.
- O chat integrado permanece vinculado ao editor Godot que abriu o chat.

Para verificar uma conexão MCP externa, pergunte:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```
