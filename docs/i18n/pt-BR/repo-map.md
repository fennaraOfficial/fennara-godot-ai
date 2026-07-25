<!-- fennara-i18n: locale=pt-BR source=docs/repo-map.md sha256=a6f2ff719d03e067423cd68229e6941f701a52bfc4ed84361327716d5d9e9c0d -->
<a id="repo-map"></a>
# Mapa do repositório

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · **Português do Brasil** · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../repo-map.md)
<!-- fennara-doc-nav:end -->

Este é o mapa rápido para contribuidores e agentes de programação que trabalham neste repositório.

<a id="find-the-right-area"></a>
## Encontre a área correta

| Alteração | Local principal |
| --- | --- |
| Configuração do usuário ou comportamento da CLI | `local/crates/fennara-cli/` |
| Protocolo MCP externo ou esquemas | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Chat integrado ou comportamento do daemon | `local/crates/fennara-daemon/` |
| Integração com o editor Godot | `fennara-cpp/` |
| UI de chat | `ui/chat/` |
| Scripts auxiliares de runtime | `runtime/` |
| Empacotamento ou lançamentos | `scripts/`, `.github/workflows/` |
| Documentação do usuário | `README.md`, `docs/` |

<a id="top-level"></a>
## Nível superior

| Caminho | Responsabilidade |
| --- | --- |
| `.github/` | Modelo de pull request, modelos de issues e workflows do GitHub Actions. |
| `docs/` | Documentação do projeto, guias de configuração, notas de arquitetura, exemplos, demos e notas de lançamento. |
| `docs/i18n/` | Manifesto de localidades e árvores completas da documentação traduzida. |
| `fennara-cpp/` | Código-fonte da GDExtension C++ para Godot e ponto de entrada do build SCons. |
| `godot_demo/addons/fennara/` | Payload instalável do addon Godot copiado para os projetos dos usuários. |
| `local/` | CLI Rust, servidor MCP, daemon, esquemas e código do runtime local. |
| `media/` | Imagens e mídia pública usadas pela documentação. |
| `runtime/` | Código-fonte dos scripts auxiliares de runtime do Godot usados por `runtime_session` e `runtime_script`. |
| `scripts/` | Scripts auxiliares de versionamento, empacotamento e lançamento. |
| `ui/chat/` | Código-fonte da UI opcional do chat integrado em webview. |
| `local/templates/` | Orientações compactas do projeto e páginas de conhecimento de IA sob demanda, gravadas nos projetos Godot por `fennara install` e atualizadas por `fennara update`. |
| `local/webview-runtimes/` | Arquivos de manifesto/configuração para runtimes externos de webview instalados nos dados compartilhados do aplicativo Fennara, como o payload CEF para Linux. |
| `install.ps1` / `install.sh` | Scripts de inicialização que instalam a CLI Fennara a partir de lançamentos do GitHub. |
| `VERSION` | Fonte da verdade da versão. |
| `README.md` | Visão geral curta voltada a pessoas e início rápido. |
| `docs/README.md` | Índice da documentação orientado por tarefas. |
| `docs/setup.md` | Configuração voltada ao usuário com o addon primeiro, pré-requisitos do chat, conexão MCP, fluxo de atualização e solução de problemas. |
| `docs/cli.md` | Referência de comandos do terminal, comportamento de instalação/atualização de responsabilidade da CLI, recuperação, diagnósticos, layout dos dados do aplicativo e orientações para automação. |
| `docs/telemetry.md` | Payload de atividade anônima, estado nos dados do aplicativo, comportamento de entrega, definição de atividade mensal e controles de desativação. |
| `CONTRIBUTING.md` | Regras de contribuição. |
| `SECURITY.md` | Política de comunicação de problemas de segurança. |
| `LICENSE.md` | Licença do projeto. |

<a id="local-rust-packages"></a>
## Pacotes Rust locais

| Caminho | Responsabilidade |
| --- | --- |
| `local/crates/fennara-cli/` | Comando `fennara`: instalação, atualização, autoatualização da CLI, doctor, diagnósticos de operações, verificações de pré-requisitos de webview, suporte a C#, configuração de aplicativos MCP e orientações do projeto geradas. |
| `local/crates/fennara-cli/src/operation.rs` | Coordenador público de operações de instalação/atualização, fases e pontos de entrada da transferência da CLI. |
| `local/crates/fennara-cli/src/operation/` | Diário de operações especializado, armazenamento durável, redação de diagnósticos e módulos de teste. |
| `local/crates/fennara-cli/src/project_addon.rs` | Validação da versão do addon já presente no projeto e da biblioteca GDExtension da plataforma atual. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Preparação da exportação de CI sem o addon, que remove somente o autoload persistente de runtime do Fennara antes de iniciar o Godot. |
| `local/crates/fennara-cli/src/release_identity.rs` | Identidade estável/de staging do addon, seletores de lançamento exato, validação do canal de pull request e compatibilidade com versões estáveis legadas. |
| `local/crates/fennara-cli/src/release_channel.rs` | Validação do ponteiro de staging por canal e resolução para um lançamento versionado exato. |
| `local/crates/fennara-cli/src/release_manifest.rs` | Análise do manifesto de lançamento, validação dos hashes dos ativos, vinculação da identidade e seleção do pacote de plataforma. |
| `local/crates/fennara-cli/src/release_version.rs` | Análise e precedência compartilhadas do SemVer da CLI, usadas por manifestos e pela seleção de lançamentos. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Adoção de versão exata de um addon completo já existente sem substituir os arquivos do addon do projeto. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Verificação de integridade compartilhada do daemon, prontidão da versão exata e inicialização usadas pela instalação e pelo doctor. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Testes no nível do processo para falhas, diagnósticos duráveis, redação e registro de operações com falha segura. |
| `local/crates/fennara-cli/src/diagnostics.rs` | Acesso voltado ao usuário ao relatório de operação sanitizado mais recente ou a um relatório nomeado. |
| `local/crates/fennara-mcp/` | Servidor MCP stdio local e encaminhamento dos esquemas das ferramentas. |
| `local/crates/fennara-daemon/` | Daemon local usado para sessões de runtime e trabalho da ponte do Godot. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | Agendador anônimo de atividade diária, fila limitada, entrega HTTP e integração com o ciclo de vida do daemon. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | Validação de identidade aleatória da instalação, persistência atômica nos dados do aplicativo, estado de recibo diário e limpeza na desativação. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Modos de aprovação do chat integrado, classificação de risco das ferramentas, decisões de permissão e tipos de solicitação de aprovação pendente. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Implementação de `exec_command` do chat integrado sob responsabilidade do daemon: detecção do shell, validação do cwd, criação do processo, limite de tempo/encerramento da árvore, captura da saída, registro do artefato de resultado e formatação do resultado. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Planejador de compactação de contexto do chat integrado: proteção exata da cauda, poda por pressão de resultados antigos de ferramentas no estilo OpenCode, seleção/armazenamento/reprodução de trechos de resumo, serialização do prompt de resumo, orçamentos de tokens e renderização de espaços reservados. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | PromptBuilder do chat integrado e contexto gerado do ambiente de runtime. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Registrador de rastreamento do chat integrado somente local, linhas de eventos SQLite, retenção e auxiliares de consulta para depuração. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | Primitivas do runtime de provedores do chat integrado, catálogo/resolução, hooks de verificação prévia de contexto, tipos normalizados de stream/erro e adaptadores compatíveis com OpenAI ou Anthropic para OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, endpoints personalizados, Ollama/local e LM Studio. |
| `local/schemas/tools/` | Esquemas JSON compartilhados das ferramentas. O servidor MCP externo e o chat integrado incorporam seus próprios subconjuntos permitidos. |
| `local/webview-runtimes/linux-cef.json` | Manifesto substituto/gerado do runtime CEF para Linux, usado para geração do manifesto de lançamento, saída do doctor e fallback legado. Ele registra o layout compartilhado nos dados do aplicativo e os metadados do arquivo compactado sem colocar o CEF dentro do zip do addon. |
| `local/Cargo.toml` | Configuração do workspace Rust. |
| `local/Cargo.lock` | Grafo bloqueado de dependências Rust. |

<a id="gdextension-source"></a>
## Código-fonte da GDExtension

| Caminho | Responsabilidade |
| --- | --- |
| `fennara-cpp/SConstruct` | Ponto de entrada do build da GDExtension. |
| `fennara-cpp/include/` | Cabeçalhos C++ públicos. |
| `fennara-cpp/src/` | Implementação em C++. |
| `fennara-cpp/src/setup/` | Estado nativo da configuração inicial, inicialização da CLI pelo manifesto de lançamento, verificação de hashes, inicialização da CLI e leitor durável do progresso das operações. |
| `fennara-cpp/src/release/version.cpp` | Validação e precedência SemVer nativas usadas pela descoberta de lançamento/atualização. |
| `fennara-cpp/src/release/identity.cpp` | Validação da identidade estável/de staging empacotada e compatibilidade estável legada. |
| `fennara-cpp/src/release/discovery.cpp` | Descoberta de atualizações pelo GitHub Latest e por canais de staging isolados. |
| `fennara-cpp/src/update/` | Coordenação de atualização para um destino exato, descoberta de recibos duráveis, transferência de encerramento/instalação e estado da UI de recuperação. |
| `fennara-cpp/src/ui/setup_panel.cpp` | Painel de configuração inicial independente de webview, com progresso, nova tentativa, logs e ações de relatório sanitizado. |
| `fennara-cpp/vendor/cef/` | Snapshot dos cabeçalhos oficiais do CEF 139 usado pela ponte OSR do Linux. Os binários do runtime ficam fora do addon. |
| `fennara-cpp/src/ui/webview_host*` | Host nativo da webview do chat integrado e backends de plataforma. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Descoberta do runtime CEF compartilhado somente para Linux, validação do marcador e base do carregador dinâmico de `libcef.so`. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Superfície de renderização fora da tela do CEF somente para Linux, encaminhamento de entrada do Godot, carregamento da ABI da ponte e atualizações de textura do Godot para a webview do chat integrado. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Pequena biblioteca de ponte somente para Linux, gerada a partir do código-fonte fixado do `libcef_dll_wrapper` oficial do CEF 139 e do adaptador CEF OSR da Fennara. A GDExtension principal carrega essa biblioteca dinamicamente depois que o runtime externo `libcef.so` é carregado. |
| `fennara-cpp/src/tools/` | Implementações das ferramentas voltadas ao Godot. |
| `fennara-cpp/src/lsp/` | Diagnósticos de scripts e auxiliares do servidor de linguagem. |
| `fennara-cpp/src/csharp/` | Seleção de projeto C# somente para build, preparação em segundo plano, diagnósticos isolados e verificação prévia do runtime. |
| `fennara-cpp/src/runtime/` | Suporte nativo de runtime usado pelas ferramentas, incluindo verificação prévia de cenas em runtime, diagnósticos de scripts e snapshots do depurador. |
| `fennara-cpp/godot-cpp/` | Submódulo dos bindings C++ do Godot. |

<a id="addon-payload"></a>
## Payload do addon

| Caminho | Responsabilidade |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Arquivo de registro da GDExtension no Godot. |
| `godot_demo/addons/fennara/VERSION` | Versão do pacote do addon. |
| `godot_demo/addons/fennara/release.json` | Identidade estável ou de staging empacotada, incluindo versão exata, tag de lançamento, canal e commit de origem do staging. |
| `godot_demo/addons/fennara/bin/` | Bibliotecas geradas por plataforma. |
| `godot_demo/addons/fennara/dist/` | Ativos empacotados da UI web usados pela webview do chat integrado. |
| `godot_demo/addons/fennara/runtime/` | Cópia empacotada e sincronizada de `runtime/` distribuída dentro do addon. |
| `godot_demo/tests/first_run_setup_test.gd` | Teste headless do estado nativo da configuração inicial e de falha determinística. |
| `godot_demo/tests/export_plugin_test.gd` | Teste headless de regressão da exclusão na exportação e da restauração do autoload. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | Teste headless de regressão do contrato de argumentos de captura de tela nativa. |
| `godot_demo/tests/image_sheet_test.gd` | Teste headless de regressão da composição compartilhada de folhas de captura de tela/runtime. |
| `godot_demo/tests/runtime_image_context_test.gd` | Teste headless de regressão da saída de frames brutos, folhas e Image arbitrária no runtime. |

<a id="runtime-helper-source"></a>
## Código-fonte dos auxiliares de runtime

| Caminho | Responsabilidade |
| --- | --- |
| `runtime/game_capture_helper.gd` | Ponto de entrada do auxiliar de runtime carregado pela GDExtension para sessões de cena e verificações de runtime. |
| `runtime/image_label.gd` | Rótulos compactos e determinísticos inseridos nas células compostas de Image depois da captura. |
| `runtime/image_sheet.gd` | Composição compartilhada usando somente Image, usada nos contextos de captura de tela e script de runtime. |
| `runtime/screenshot_script_context.gd` | Fachada pública do script de captura de tela que adiciona a composição compartilhada de Image ao contexto nativo de captura. |
| `runtime/runtime_script_context.gd` | Superfície pública de auxiliares `ctx` exposta a `runtime_script`, incluindo frames brutos, composição/saída de Image, esperas, entrada, snapshots, condições, raycasts e cliques. |
| `runtime/runtime_input_driver.gd` | Driver de baixo nível para eventos de entrada em runtime, incluindo teclas, botões do mouse, movimento absoluto do mouse, movimento relativo do mouse, modificadores e limpeza da entrada. |
| `runtime/runtime_node_snapshot.gd` | Busca de nós no runtime, verificações de existência, snapshots seguros contra referências obsoletas, leituras de propriedades e resumos de filhos. |
| `runtime/runtime_physics_query.gd` | Auxiliares de raycast e varredura exatos em 2D/3D no runtime, com recibos compactos de acerto. |
| `runtime/runtime_query_utils.gd` | Utilitários compartilhados para consultas de runtime, coerção de vetores, resolução segura de nó/caminho, identidade de objetos e correspondência genérica de destinos. |
| `runtime/runtime_capture_store.gd` | Gravador de artefatos de captura/status do runtime usado por sessões de runtime, scripts e verificações de ambiente. |
| `runtime/runtime_check_runner.gd` | Executor de verificações de runtime para especificações de execução de cena não interativas. |

<a id="scripts-and-workflows"></a>
## Scripts e workflows

| Caminho | Responsabilidade |
| --- | --- |
| `scripts/set-version.mjs` | Atualiza arquivos versionados em todo o repositório. |
| `scripts/check-version.mjs` | Verifica a sincronização da versão. |
| `scripts/release-identity.mjs` | Validação e geração compartilhadas em Node para a identidade SemVer do lançamento e ponteiros de staging por PR. |
| `scripts/release-policy.mjs` | Política da versão mínima compatível da CLI publicada para manifestos de lançamento estáveis e de staging. |
| `scripts/staging-candidate.mjs` | Geração confiável da identidade do candidato de staging e decisões monotônicas de ponteiro por PR. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | Validação especializada do addon de staging, arquivos compactados, manifesto, sistema de arquivos compartilhado e pacote de publicação. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | Pontos de entrada de validação rigorosa para resultados de build não confiáveis e para o pacote de publicação confiável. |
| `scripts/check-staging-channel-advance.mjs` | Aplica verificações monotônicas e de procedência antes que um ponteiro de canal de staging avance. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | Verificam os bytes dos ativos publicados e o comportamento público de download antes da promoção do ponteiro. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | Gera um projeto Godot temporário ignorado e faz um teste rápido da inspeção somente leitura de `PackedScene` importada em relação à GDExtension do editor. |
| `scripts/release-targets.mjs` | Define os destinos de plataforma compatíveis para lançamento e os nomes de seus ativos empacotados. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | Gravam a identidade congelada do candidato e o pequeno ponteiro de seu canal. |
| `scripts/sync-chat-ui.mjs` | Copia o código-fonte da UI de chat sem etapa de build para o payload do addon. |
| `scripts/sync-runtime.mjs` | Copia o código-fonte dos auxiliares de runtime da raiz do repositório para o payload do addon. |
| `scripts/sync-doc-navigation.mjs` | Adiciona navegação da documentação, hashes da fonte e âncoras estáveis sem traduzir a prosa. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | Validam cobertura da tradução, atualidade, estrutura Markdown, URLs e links. |
| `scripts/package-preview.mjs` | Monta os zips de prévia/lançamento do addon, CLI e runtime local depois dos builds por plataforma. |
| `scripts/prepare-linux-cef-runtime.mjs` | Prepara o zip separado do runtime CEF para Linux x64, remove símbolos dos binários ELF preparados, valida os arquivos obrigatórios e pode gravar o manifesto gerado do lançamento. |
| `scripts/prepare-linux-cef-sdk.mjs` | Baixa e extrai o SDK mínimo oficial fixado do CEF 139 para Linux, usado por builds de CI que precisam do código-fonte do wrapper em `libcef_dll/`. |
| `scripts/check-linux-cef-runtime-release.mjs` | Valida o ativo do lançamento do runtime CEF para Linux em relação ao manifesto gerado em `local/webview-runtimes/linux-cef.json`. |
| `scripts/write-release-manifest.mjs` | Grava e valida `fennara-release-manifest-v<version>.json` a partir dos ativos do lançamento, incluindo hashes do pacote local, do addon e dos runtimes compartilhados. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Código-fonte do auxiliar mínimo do CEF para Linux empacotado no zip separado do runtime CEF. |
| `.github/workflows/version-check.yml` | Verificação da consistência da versão. |
| `.github/workflows/gdextension-build.yml` | Verificação do build multiplataforma da GDExtension e teste headless do estado nativo da configuração inicial no Windows. |
| `.github/workflows/local-build.yml` | Verificação do build dos pacotes Rust locais. |
| `.github/workflows/package-preview.yml` | Artefatos manuais da prévia de pacotes, incluindo um artefato somente para teste do runtime CEF para testes rápidos do chat no Linux. |
| `.github/workflows/release.yml` | Publicação manual do lançamento no GitHub, incluindo o empacotamento gerado do runtime CEF para Linux, geração do manifesto de lançamento e validação final dos ativos. |
| `.github/workflows/staging-release.yml` | Build manual de staging com SHA exato, execução de teste somente de validação, publicação do pré-lançamento exato e avanço do ponteiro por PR. |

<a id="where-to-change-things"></a>
## Onde alterar cada coisa

| Tarefa | Comece aqui |
| --- | --- |
| Adicionar ou alterar uma ferramenta do Godot | `fennara-cpp/src/tools/` e `local/schemas/tools/` |
| Alterar o texto do esquema MCP | `local/schemas/tools/` |
| Alterar `fennara install` ou `fennara update` | `local/crates/fennara-cli/src/`; o staging nativo e a aplicação/reversão desacoplada são responsabilidade de `release_update.rs`, `update_stage.rs`, `update_stage/` e `update_apply/` |
| Alterar comandos da CLI ou comportamento do terminal | `local/crates/fennara-cli/src/` e `docs/cli.md` |
| Alterar progresso de atualização nativo, confirmação de encerramento, handshake de ativação ou recuperação | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` e `ui/chat/` |
| Alterar a configuração inicial nativa ou a inicialização da CLI | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp` e `fennara-cpp/src/ui/dock.cpp` |
| Alterar a exclusão do addon durante a exportação | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp` e `godot_demo/tests/export_plugin_test.gd` |
| Alterar logs de operações de instalação/atualização, fases, códigos de erro ou relatórios de diagnóstico | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/` e `local/crates/fennara-cli/src/diagnostics.rs` |
| Alterar verificações de pré-requisitos de webview | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs` e `fennara-cpp/src/ui/webview_host*` |
| Alterar orientações do projeto geradas | `local/templates/` e `local/crates/fennara-cli/src/project_guidance.rs` |
| Sincronizar orientações geradas do addon de demonstração | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs` e `godot_demo/addons/fennara/ai/` |
| Alterar a configuração de aplicativos MCP | `local/crates/fennara-cli/src/mcp_setup.rs` e `docs/mcp-setup.md` |
| Alterar o comportamento de processos/logs das sessões de runtime | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/` e `fennara-cpp/src/tool_results/` |
| Alterar auxiliares ctx de `runtime_script`, entrada, snapshots, esperas, raycasts, capturas ou limpeza | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json` e `docs/tools.md` |
| Alterar a UI do chat integrado, comandos de barra ou seletor de modelo/provedor | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp` e `fennara-cpp/src/ui/webview_host*` |
| Alterar provedores do chat integrado | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` e `ui/chat/` |
| Alterar campos de telemetria anônima, agendamento ou controles de privacidade | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` e `docs/telemetry.md` |
| Alterar bibliotecas da UI de chat vendorizadas | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/` e `THIRD_PARTY_NOTICES.md` |
| Alterar o suporte a C# | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/` e os esquemas e orientações das ferramentas de C# |
| Alterar pacotes de lançamento, política da versão mínima da CLI ou autoatualização da CLI | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs` e `.github/workflows/release.yml` |
| Incrementar a versão | `node scripts/set-version.mjs <version>` |
| Atualizar configuração/documentação de chat vs MCP, provedores ou comandos de barra | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md` e `llms.txt` |
| Atualizar traduções da documentação | Página canônica em inglês, `docs/i18n/languages.json`, páginas correspondentes da localidade, `scripts/sync-doc-navigation.mjs` e `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## Observações

- Mantenha este arquivo atualizado ao adicionar ou mover áreas importantes do código-fonte.
- Mantenha as etapas de lançamento em [release.md](release.md).
- Mantenha as etapas de configuração em [setup.md](setup.md).
- Mantenha o comportamento dos comandos do terminal em [cli.md](cli.md).
