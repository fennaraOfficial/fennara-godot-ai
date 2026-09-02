<!-- fennara-i18n: locale=pt-BR source=CONTEXT.md sha256=7d76acbada75ade69b43dc52fcd543f90d678c04b3e9b50fc11601b8b1853fd4 -->
<a id="fennara-context"></a>
# Contexto do Fennara

<!-- fennara-doc-nav:start -->
[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · [Español](../es/CONTEXT.md) · **Português do Brasil** · [日本語](../ja/CONTEXT.md) · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · [Deutsch](../de/CONTEXT.md) · [Türkçe](../tr/CONTEXT.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

Este arquivo define termos comuns usados na documentação, nas issues, nas notas de lançamento e nas orientações do Fennara voltadas para agentes.

<a id="product-terms"></a>
## Termos do produto

**Fennara**

O ambiente para agentes com conhecimento do Godot neste repositório. O Fennara conecta ferramentas de IA a feedback real do Godot, como diagnósticos, validação de cenas, erros de execução, capturas de tela e orientações do projeto.

**Godot Addon**

O plugin instalável copiado para o projeto Godot do usuário em `res://addons/fennara/`. Ele é responsável pela interface do dock, pelas ferramentas de inspeção voltadas ao Godot, pela biblioteca GDExtension nativa, pelos recursos empacotados da interface de chat, pelos scripts auxiliares de execução e pela versão local do addon no projeto.

**Fennara CLI**

O comando `fennara` instalado na máquina do usuário. Ele cuida da instalação, atualização, autoatualização da CLI, verificações do doctor, configuração de aplicativos MCP, avisos de pré-requisitos da webview, verificações da configuração de C# e orientações de projeto geradas.

**Pacote local**

O arquivo zip de lançamento que contém os executáveis locais do Fennara, como o servidor MCP, o daemon, os binários de execução e os binários de inicialização para uma plataforma e arquitetura.

**Orientações do projeto**

Arquivos de orientação gerados e colocados em um projeto Godot, incluindo `AGENTS.md` e as referências direcionadas em `addons/fennara/ai/`, para que agentes de programação com IA saibam quando e como usar o Fennara.

<a id="mcp-terms"></a>
## Termos de MCP

**Servidor MCP do Fennara**

O servidor MCP local via stdio iniciado por um aplicativo de programação com IA, como Claude Code, Cursor, Cline, Gemini CLI ou outro cliente MCP. Ele expõe as ferramentas do Fennara a esse aplicativo externo.

**Aplicativo MCP**

Um aplicativo de IA externo configurado por `fennara mcp-setup`. A configuração do aplicativo MCP controla qual aplicativo externo pode chamar as ferramentas do Fennara. Ela não seleciona o modelo usado pelo chat integrado do Fennara.

**Destino MCP**

O destino de compatibilidade global do daemon, selecionado no dock, usado por
uma conexão MCP externa que não tem uma Vinculação de projeto MCP. Conexões MCP
vinculadas não leem nem alteram esse destino.

**Vinculação de projeto MCP**

A Raiz do projeto estável, selecionada uma única vez quando um processo MCP do
Fennara é iniciado. Ela encaminha as chamadas desse processo para a Sessão do
editor Godot correspondente sem usar o Destino MCP global do daemon.

**Raiz do projeto**

O diretório canônico do sistema de arquivos que contém o `project.godot` de um
projeto Godot. O Fennara usa a identidade do sistema de arquivos, e não o nome
do projeto, para distinguir repositórios e worktrees.

**Sessão do editor Godot**

Uma instância atualmente conectada do addon Fennara e do editor Godot. Ela tem
um caminho de projeto e um ID de processo do Godot, e pode desconectar e se
reconectar sem alterar a Vinculação de projeto de um processo MCP.

**Esquema de ferramenta**

A descrição de uma ferramenta MCP do Fennara voltada ao modelo, incluindo argumentos, limites e observações sobre o fluxo de trabalho.

**Envelope de resultado da ferramenta**

O resultado conciso voltado ao modelo, retornado após uma chamada de ferramenta. Os resultados do Fennara devem explicar o status, as descobertas importantes e o próximo contexto útil sem despejar dados brutos desnecessários.

<a id="built-in-chat-terms"></a>
## Termos do chat integrado

**Chat integrado**

A própria interface de chat do Fennara no addon Godot ou no navegador do sistema. Ela é separada dos aplicativos MCP externos. Um usuário pode configurar o Claude Code para MCP e ainda escolher outro provedor ou modelo para o chat integrado.

**Interface de chat**

O modo de exibição do chat integrado. O modo incorporado usa a webview nativa do dock do Godot. O modo navegador disponibiliza a mesma interface a partir do daemon local e a abre no navegador do sistema.

**Provedor de chat**

Um backend capaz de gerar respostas do chat integrado, como OpenAI, Anthropic,
OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax,
Ollama local ou LM Studio.

**Referência de modelo**

O identificador de modelo qualificado pelo provedor selecionado no chat integrado. Comandos de barra, como `/provider` e `/model`, ajudam os usuários a conectar provedores e escolher referências de modelo.

**Conexão de provedor**

Configurações locais e estado de autenticação de um provedor de chat gerenciados pelo daemon, incluindo chaves de API ou URLs-base locais. Os segredos do provedor devem permanecer no armazenamento local gerenciado pelo daemon, e não dentro do projeto Godot.

**Rastro de geração**

Metadados armazenados de uma geração do chat integrado, que associam mensagens do assistente, chamadas de ferramentas, escolha de provedor ou modelo, uso e registros de custo à geração que os produziu.

<a id="runtime-and-webview-terms"></a>
## Termos de execução e webview

**Daemon do Fennara**

O serviço local que conecta chamadas MCP e solicitações do chat integrado ao addon Godot, armazena o estado de execução local e disponibiliza rotas de chat hospedadas pelo daemon, como `/chat/`.

**Sessão de execução**

Um processo de jogo interativo do Godot gerenciado pelo daemon, usado para
inspecionar cenas em execução, coletar logs e fazer capturas em runtime. Sua Raiz
do projeto proprietária canônica mantém o controle mesmo que o editor desse
projeto se reconecte. A validação limitada de cenas e as chamadas independentes
de captura de tela usam caminhos separados e não ocupam o Slot de execução.

**Slot de execução**

O estado de admissão de toda a máquina que permite iniciar ou executar no
máximo uma Sessão de execução gerenciada pelo daemon entre todos os projetos
conectados.

**Lease de execução**

O direito renovável e limitado no tempo da Raiz do projeto proprietária de
ocupar o Slot de execução. A atividade do proprietário renova seu prazo de
inatividade, enquanto o prazo absoluto continua sempre em vigor.

**Snapshot do Godot**

Um snapshot reversível do estado do projeto, criado antes de um turno assistido pelo Fennara que possa modificar arquivos. A preparação do snapshot deve terminar antes que o turno do usuário seja persistido, para que uma falha na preparação não deixe prompts pendentes.

**Runtime de webview**

O suporte de plataforma necessário para exibir o chat integrado no Godot ou próximo dele. O Windows usa WebView2, o macOS usa WebKit/WKWebView e o Linux usa um runtime CEF compartilhado instalado nos dados de aplicativo do Fennara.

**Runtime CEF compartilhado do Linux**

O payload externo do runtime CEF usado pela webview de chat no Linux. Ele é instalado uma vez no diretório de dados de aplicativo do Fennara e não deve ser incluído em cada arquivo zip do addon Godot.

<a id="release-terms"></a>
## Termos de lançamento

**Manifesto de lançamento**

O recurso JSON chamado `fennara-release-manifest-v<version>.json`. Ele associa recursos de lançamento às plataformas, registra hashes SHA-256, declara recursos de runtime compartilhados e define `minimum_cli_version`.

**Versão mínima da CLI**

A versão mais antiga da CLI `fennara` autorizada a consumir um manifesto de
lançamento. Se um lançamento exigir uma lógica de instalação ou atualização
mais recente, atualize sua faixa em `scripts/release-policy.mjs`. O gerador do
manifesto aplica essa política depois de validar a identidade do lançamento.
Os fluxos de trabalho não escolhem esse valor.

**Lançamento mais recente**

O ponteiro Latest Release do GitHub para um lançamento exato com versão. Os instaladores e as atualizações padrão resolvem esse ponteiro pela API do GitHub. O Fennara não usa uma tag ou um lançamento literal chamado `latest`. Atualizar arquivos-fonte depois da publicação não altera os recursos do lançamento. Recursos de manifesto já publicados devem ser substituídos explicitamente.
