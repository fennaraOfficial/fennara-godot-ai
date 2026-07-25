<!-- fennara-i18n: locale=pt-BR source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# Perguntas frequentes

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · [Español](../es/faq.md) · **Português do Brasil** · [日本語](../ja/faq.md) · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · [Deutsch](../de/faq.md) · [Türkçe](../tr/faq.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../faq.md)
<!-- fennara-doc-nav:end -->

Comece por [Configuração](setup.md) para instalação e atualizações. Use esta
página para respostas curtas e links para a referência detalhada.

| Pergunta | Resposta curta |
| --- | --- |
| Preciso de uma chave de provedor? | Apenas para um provedor na nuvem no chat integrado |
| Posso usar um aplicativo MCP externo? | Sim, ele usa sua própria conta de modelo |
| O Fennara envia meu projeto para um servidor do Fennara? | Não |
| Posso deixar vários editores Godot abertos? | Sim, escolha o destino MCP externo no dock |

<a id="is-fennara-only-a-code-generator"></a>
## O Fennara é apenas um gerador de código?

Não. O Fennara é um fluxo de trabalho para agentes com conhecimento do Godot.
Ele pode trabalhar com arquivos do projeto, cenas, diagnósticos, erros de
execução, capturas de tela e o contexto do editor Godot.

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## O Fennara é apenas mais um servidor de comandos MCP para Godot?

Não. MCP é uma das formas de usar o Fennara em aplicativos como Codex, Claude,
Cursor, Gemini e Antigravity. O Fennara também tem um dock de chat integrado
opcional. A principal tese do produto é o ciclo de feedback do Godot:
diagnósticos, validação, erros de execução, capturas de tela e resultados
estruturados das ferramentas para que agentes possam corrigir erros.

<a id="does-fennara-replace-godot-knowledge"></a>
## O Fennara substitui o conhecimento sobre Godot?

Não. O Fennara não tenta tornar o Godot opcional. Ele foi criado para manter os
agentes de IA responsáveis perante o motor Godot real.

<a id="how-should-i-install-fennara"></a>
## Como devo instalar o Fennara?

No Windows e no Linux, adicione o addon, abra o dock do Fennara e pressione
**Set Up Fennara**, ou instale pelo terminal. No macOS, instale pela CLI para
evitar a notificação de segurança que pode ocorrer quando um ZIP do addon
baixado pelo navegador é extraído manualmente. Consulte [Configuração](setup.md)
para ver os dois caminhos.

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## Por que o macOS diz que não consegue verificar `libfennara.macos.editor`?

O addon de lançamento contém uma biblioteca nativa que atualmente não é
notarizada pela Apple. Quando o ZIP do addon é baixado pelo navegador e extraído
manualmente, o Finder pode propagar metadados de quarentena para essa biblioteca,
causando a notificação do macOS.

Para evitá-la, use a [instalação pela CLI](setup.md#instalar-pelo-terminal-recomendado-no-macos).
Se a notificação já aparecer, feche o Godot, remova a pasta `addons/fennara/`
copiada manualmente, instale a CLI e execute `fennara install` no diretório do
projeto. A CLI instala o mesmo addon sem passar pela quarentena do navegador e do Finder.

<a id="do-i-need-a-chat-provider-api-key"></a>
## Preciso de uma chave de API de provedor de chat?

Apenas se quiser usar um provedor na nuvem no dock de chat integrado do Fennara.
Clientes MCP externos usam sua própria configuração de modelo ou aplicativo e
podem usar ferramentas MCP do Fennara sem fornecer uma chave de provedor ao chat.

O chat integrado também pode usar Ollama ou LM Studio local sem uma chave de API
na nuvem. Consulte [Provedores do chat integrado](providers.md).

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## Por que o dock pede um provedor se eu já executei `mcp-setup --claude`?

`fennara mcp-setup --claude` conecta o Claude às ferramentas MCP do Godot do
Fennara. Ele não conecta o dock integrado do Fennara ao Claude e não compartilha
sua assinatura do Claude com o chat do Fennara.

Use Claude Code ou Claude Desktop para o fluxo MCP externo. Configure um
provedor separado apenas se quiser conversar dentro do dock do Fennara no Godot.
Consulte [Aplicativos MCP e chat integrado](chat-vs-mcp.md).

<a id="what-are-provider-and-model"></a>
## O que são `/provider` e `/model`?

São comandos de barra no dock de chat integrado do Fennara. `/provider` abre o
seletor de provedores. `/model` abre o seletor de modelos. São atalhos da
interface, não ferramentas MCP externas nem texto enviado ao modelo. Consulte
[Comandos de barra do chat integrado](slash-commands.md).

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## O Fennara envia meu projeto Godot para um servidor do Fennara?

Não. No fluxo OSS normal, o cliente MCP, o daemon e o addon Godot são executados
localmente. O chat integrado envia solicitações de modelo apenas ao provedor que
você configurar, como OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek,
Z.AI, Moonshot AI, Kimi For Coding, MiniMax ou um servidor Ollama/LM Studio local.

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## Qual projeto recebe chamadas de ferramentas MCP quando vários editores Godot estão abertos?

O daemon encaminha chamadas MCP externas para o destino MCP ativo. Use o controle
de destino MCP do dock do Fennara no Godot para escolher o projeto. As sessões
do chat integrado permanecem vinculadas ao editor Godot que abriu o chat.

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## Por que o Linux instala um runtime CEF separado?

O chat incorporado no Linux usa renderização off-screen do CEF. O payload do CEF
é grande, portanto o Fennara o instala uma vez no diretório de dados de aplicativo
do usuário em vez de copiá-lo para o addon de cada projeto Godot.

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## O addon deve conter `libcef.so`?

Não. `libcef.so`, recursos do CEF, pacotes de localidade e o auxiliar do CEF
pertencem ao runtime CEF compartilhado do Linux. O addon deve conter apenas os
arquivos do addon Godot, os binários GDExtension, os arquivos da interface de
chat e pequenos binários auxiliares incluídos, como ripgrep.

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## E se a webview do chat integrado não conseguir iniciar?

As ferramentas MCP do Fennara continuam funcionando. Apenas o dock de chat
opcional dentro do editor precisa da webview da plataforma. No Windows, instale
o Microsoft Edge WebView2 Runtime se `fennara doctor` informar que ele está
ausente. No macOS, o WKWebView vem do WebKit.framework do sistema. No Linux,
execute `fennara update` para que o runtime CEF gerenciado pelo lançamento seja
instalado ou reparado.

Você também pode usar a opção **Open chat in my system browser next time** em
Chat Settings. Ela mantém o mesmo chat integrado e as configurações de provedor,
mas abre a interface pelo daemon local no navegador do sistema em vez da webview
incorporada ao Godot. Reinicie o Godot após alterar a configuração.

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## Abrir o chat no navegador usa o Claude ou meu aplicativo MCP?

Não. A exibição no navegador é apenas uma escolha de interface e runtime para o
chat integrado do Fennara. Ele continua usando o provedor selecionado nas
configurações de chat do Fennara. `fennara mcp-setup --claude` e comandos
semelhantes configuram aplicativos MCP externos. Eles não configuram o modelo
do chat integrado.

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## `fennara update` reescreve a configuração dos aplicativos MCP?

Não. `fennara update` atualiza, quando necessário, a CLI instalada, o addon do
projeto, o pacote de runtime local, as orientações de projeto geradas e os
recursos de runtime gerenciados pela plataforma. Execute `fennara mcp-setup`
novamente apenas ao configurar ou reparar a configuração de um aplicativo MCP.

<a id="where-does-chat-history-live"></a>
## Onde fica o histórico do chat?

O histórico do chat é armazenado localmente pelo daemon e limitado ao projeto
Godot atual. Chaves de provedores e URLs de provedores locais também são
armazenadas localmente pelo daemon, fora do projeto Godot.

<a id="what-should-agents-use-fennara-tools-for"></a>
## Para que os agentes devem usar as ferramentas do Fennara?

Use o Fennara para feedback com conhecimento do Godot: árvores de cena,
propriedades alteradas de nós e recursos, diagnósticos, validação, sessões de
execução, capturas de tela e estado do depurador do editor. Clientes MCP ainda
devem usar suas próprias ferramentas comuns de leitura e pesquisa de arquivos,
a menos que uma ferramenta específica do Fennara seja necessária.
