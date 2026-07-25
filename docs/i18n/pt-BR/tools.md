<!-- fennara-i18n: locale=pt-BR source=docs/tools.md sha256=4cf72381fada4fec347f29da5995d9768b39235f71b437dd698088ac0acb3518 -->
<a id="tools"></a>
# Ferramentas

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../tools.md) · [简体中文](../zh-CN/tools.md) · [Español](../es/tools.md) · **Português do Brasil** · [日本語](../ja/tools.md) · [한국어](../ko/tools.md) · [Русский](../ru/tools.md) · [Français](../fr/tools.md) · [Deutsch](../de/tools.md) · [Türkçe](../tr/tools.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../tools.md)
<!-- fennara-doc-nav:end -->

O Fennara oferece aos agentes de programação inspeção, edição, validação,
capturas de tela e feedback de runtime com conhecimento do Godot. Ele complementa
as ferramentas normais de repositório e shell, em vez de substituí-las.

Esta página explica o que cada ferramenta pode fazer, o que significa uma
chamada bem-sucedida e as limitações ou casos de falha importantes. Os esquemas
ativos das ferramentas continuam sendo a fonte oficial para argumentos, campos
de resultado, limites e instruções ao agente. Projetos instalados também recebem
orientações compactas e conhecimento sob demanda em `addons/fennara/ai/`.

<a id="tool-surfaces"></a>
## Interfaces das ferramentas

Clientes MCP externos, incluindo Codex, Claude Code, Cursor e Gemini, conectam-se
pelo processo local `fennara-mcp`. Eles usam sua própria conta de modelo e suas
ferramentas normais de arquivos, pesquisa, diff e shell junto com o Fennara.

O chat integrado usa o mesmo daemon e a mesma ponte do Godot. Ele pode chamar as
mesmas ferramentas Godot e também oferece `read_file` e `exec_command` limitados
ao projeto. A configuração de provedor e modelo pertence ao chat integrado, não
ao servidor MCP.

`fennara_status` está disponível para clientes MCP externos. O chat integrado já
recebe do daemon o estado da conexão e do projeto ativo.

<a id="typical-workflow"></a>
## Fluxo de trabalho típico

1. Confirme o projeto conectado ao usar um cliente MCP externo.
2. Inspecione a cena, o recurso, a classe, o estado de importação ou a configuração relevante.
3. Faça a menor edição útil.
4. Execute diagnósticos ou validação de cena.
5. Use capturas de tela ou ferramentas de runtime quando evidências visuais ou comportamentais forem importantes.

O sistema de arquivos do editor pode ficar temporariamente ocupado verificando
ou importando. Use ferramentas de recursos depois que ele informar que está pronto.

<a id="connection"></a>
## Conexão

<a id="fennarastatus"></a>
### `fennara_status`

Relata servidor MCP, daemon, projeto Godot ativo, sessões de editor conectadas,
versões dos componentes, contexto de renderização, ferramentas anunciadas e
prontidão do sistema de arquivos do editor.

Comportamento funcional:

- Retorna um bloco de status em texto simples.
- Distingue um sistema de arquivos pronto de outro que está verificando ou importando.
- Informa se ferramentas voltadas a recursos estão prontas.
- Mostra diferenças de versão para diagnosticar instalações incompatíveis.

Limites e falhas importantes:

- Relata prontidão no nível do projeto, não de um caminho específico.
- Um daemon desconectado, projeto ativo ausente ou plugin Godot desconectado é relatado diretamente, em vez de tratado como pronto.
- A prontidão pode mudar brevemente enquanto o Godot reimporta arquivos.

<a id="inspection"></a>
## Inspeção

<a id="getscenetree"></a>
### `get_scene_tree`

Carrega uma cena pelo Godot e retorna sua hierarquia de nós, classes, scripts
anexados e subcenas instanciadas. Os caminhos retornados podem ser usados por outras ferramentas.

Comportamento funcional:

- Lê cenas criadas sem regravá-las.
- Torna visível a estrutura de nós e instâncias antes da edição.
- Mantém o resultado focado na hierarquia, sem expandir todos os recursos.

Limites e falhas importantes:

- Não é um relatório completo de recursos 3D, malhas, materiais, esqueletos ou animações.
- Uma cena que o Godot não consegue carregar retorna falha, não uma árvore presumida.
- Detalhes grandes de recursos pertencem a inspeções direcionadas de propriedades ou scripts.

<a id="getnodeproperties"></a>
### `get_node_properties`

Mostra propriedades diferentes dos padrões da classe para nós selecionados e
expande resumos úteis de recursos incorporados.

Comportamento funcional:

- Oferece suporte a até cinco nós-alvo por chamada.
- Lê propriedades exportadas de GDScript e metadados C# disponíveis.
- Resume recursos como animações, temas, dados de tiles, bibliotecas de malhas, sprite frames e grafos de animação, em vez de despejar valores opacos.

Limites e falhas importantes:

- É direcionado a nós, não um inventário completo da cena.
- Recursos-fonte importados podem expor menos informações que nós `.tscn`. Use `run_asset_import_script` quando precisar inspecionar diretamente o recurso importado gerado.
- Caminhos de nós inválidos são relatados, não ignorados.

<a id="getclassinfo"></a>
### `get_class_info`

Retorna a superfície de API real de uma classe Godot, incluindo herança,
propriedades, métodos, sinais, enums, constantes e documentação disponível.

Comportamento funcional:

- As informações ClassDB de runtime vêm do editor conectado.
- Classes integradas usam a documentação XML oficial do Godot correspondente às versões principal e secundária conectadas, com fallback explícito para `master`.
- Classes GDExtension e de addons nativos retornam informações disponíveis de classe e propriedades sem fingir que têm documentação oficial do Godot.

Limites e falhas importantes:

- A consulta pode ficar incompleta quando o XML correspondente não está disponível ou a resposta não chega por inteiro.
- Comportamento apenas de runtime ainda pode exigir um pequeno script de sondagem no editor.
- Um nome de classe inexistente é relatado como ausente.

<a id="editing"></a>
## Edição

<a id="writeorupdatefile"></a>
### `write_or_update_file`

Cria, regrava ou faz uma substituição exata em um arquivo de texto do projeto.

Comportamento funcional:

- `write` cria ou substitui um arquivo pelo conteúdo completo.
- `update` substitui um bloco de texto exato e exclusivo.
- Edições de GDScript e shader retornam automaticamente diagnósticos do Godot.
- Edições de shader também tentam resserializar cenas e recursos que os referenciam, para evitar dados de material incorporado obsoletos.
- Gravações C# podem formar uma edição de vários arquivos antes de solicitar uma compilação de diagnóstico do projeto.

Limites e falhas importantes:

- Texto de atualização ausente ou ambíguo falha, em vez de alterar uma correspondência arbitrária.
- Caminhos protegidos do Fennara, Git, cache do Godot, manifesto do plugin e configurações do projeto não podem ser editados.
- Não se destina à manipulação bruta de `.tscn`, `.tres` ou `.res`.
- A validação C# não ocorre após cada gravação. Faça uma varredura do projeto após concluir as edições relacionadas.
- Proprietários de shaders que não possam ser resserializados com segurança são relatados como ignorados ou como aviso.

<a id="runsceneeditscript"></a>
### `run_scene_edit_script`

Executa um worker GDScript em tempo de editor em uma cena criada ou grafo de
recursos. É a forma estruturada de inspecionar ou editar cenas pelo modelo de
objetos e serializador do Godot.

Comportamento funcional:

- O modo inspect carrega um grafo separado somente leitura e nunca o salva.
- O modo edit pode adicionar, remover, renomear ou reparentear nós, atribuir recursos, alterar propriedades, criar cenas e salvar pela serialização do Godot.
- Cenas existentes só são salvas quando o worker marca o contexto como modificado.
- Novos nós e instâncias PackedScene usam auxiliares explícitos de propriedade para que o Godot serialize a estrutura desejada.
- Diagnósticos de script ocorrem antes da execução, e cenas salvas recebem validação posterior.
- Raízes de cenas herdadas são preservadas quando o Godot consegue serializar as substituições com segurança.
- Toda chamada retorna o caminho efetivo do worker temporário, para corrigir uma falha sem recriá-lo.

Limites e falhas importantes:

- O grafo carregado não equivale a pressionar Run Scene. APIs de jogabilidade dependentes de SceneTree, timers, processamento de frames e transformações globais podem agir diferente ou falhar em nós separados.
- O modo inspect bloqueia auxiliares de alteração do contexto, mas GDScript arbitrário ainda deve evitar efeitos colaterais de sistema de arquivos, editor, SO e salvamento de recursos.
- Fontes importadas, como `.glb` e `.gltf`, não são salvas por essa ferramenta. Configurações de importação pertencem a `run_asset_import_script`.
- Propriedade incorreta dos conteúdos de PackedScene é rejeitada, pois pode achatar ou duplicar instâncias.
- Se o salvamento achatar uma raiz herdada, o Fennara restaura o arquivo original e relata falha.
- Diagnósticos ou erros de runtime interrompem a edição. Uma falha não cria nem atualiza a cena-alvo, embora o worker temporário possa permanecer para nova tentativa.

<a id="runassetimportscript"></a>
### `run_asset_import_script`

Executa um worker GDScript limitado em tempo de editor em um recurso-fonte
importado e sua configuração. Oferece suporte a modelos, texturas, áudio, fontes
e outros formatos que já tenham um sidecar `.import`.

Comportamento funcional no modo inspect:

- Relata importador, classe do recurso gerado, validade, opções atuais tipadas, arquivos gerados e dependências upstream.
- Carrega o recurso gerado sem reutilizar entradas aninhadas obsoletas do cache.
- Pode instanciar temporariamente uma PackedScene importada na SceneTree ativa para inspeção limitada e removê-la sem salvar.
- Fornece resumos limitados de sub-recursos gerados.
- Nunca persiste alterações das opções no modo inspect.

Comportamento funcional no modo edit:

- Prepara opções existentes compatíveis preservando seus tipos Variant nativos.
- Permite que o editor ativo faça a reimportação por `EditorFileSystem`.
- Só relata sucesso depois de verificar configurações canônicas, saídas geradas, estado do sistema de arquivos e um novo carregamento profundo.
- Tenta restaurar e reimportar a configuração anterior quando a verificação falha, e relata o sucesso da recuperação.

Limites e falhas importantes:

- O arquivo-fonte já deve estar importado e ter um sidecar `.import` válido.
- A versão um edita apenas opções classificadas como alterações seguras de cache gerado para importadores integrados compatíveis de textura e cena.
- Identidade do importador, scripts de importação, `_subresources`, caminhos externos de extração e opções com efeitos desconhecidos permanecem somente para inspeção.
- Opções desconhecidas ou sem suporte e valores com tipo Variant incorreto falham, sem coerção.
- Alteração direta de `.import` é detectada, restaurada e relatada como falha. O Fennara controla a persistência do sidecar.
- Cenas importadas com script de raiz configurado não são instanciadas temporariamente pelo auxiliar.
- Dependências descrevem arquivos necessários para importar o recurso. Não identificam consumidores downstream, como cenas que usam um modelo ou materiais que usam uma textura.
- Diagnósticos, erros de runtime ou reimportação, arquivos gerados ausentes, estado inválido ou falhas de recarga impedem um resultado bem-sucedido.
- Arrays grandes e detalhes de recursos são limitados ou resumidos. Um resultado limitado não promete que todos os vértices, chaves ou dependências foram impressos.

<a id="projectsettings"></a>
### `project_settings`

Lê e altera configurações estruturadas do projeto, autoloads, metadados do
aplicativo, configurações de renderização e exibição e ações de entrada.

Comportamento funcional:

- Usa operações estruturadas com conhecimento do Godot, não substituição textual bruta de `project.godot`.
- Lista ações de entrada com deadzones, contagem de eventos e resumos legíveis.
- Oferece eventos estruturados ao adicionar ou atualizar controles.

Limites e falhas importantes:

- Operações desconhecidas ou valores inválidos são relatados.
- Não substitui a edição de cenas ou scripts.
- Alterações que afetem inicialização, renderização, entrada ou addons ainda devem ser validadas.

<a id="checks"></a>
## Verificações

<a id="scriptdiagnostics"></a>
### `script_diagnostics`

Executa diagnósticos com conhecimento do Godot para scripts e shaders.

Comportamento funcional:

- Chamadas direcionadas de GDScript e shader aceitam até cinco arquivos.
- Diagnósticos GDScript vêm do servidor de linguagem do Godot.
- Diagnósticos de shader vêm do parser de shaders.
- Verificações GDScript direcionadas também carregam cenas relevantes na memória, associando erros de anexação ao script e à cena.
- Varreduras do projeto verificam GDScript e shaders e fazem uma compilação incremental C# isolada quando há projeto C#.
- Assemblies C# de diagnóstico ficam separadas das assemblies normais de runtime do editor.

Limites e falhas importantes:

- Diagnósticos direcionados de arquivos C# não têm suporte. C# usa varredura do projeto.
- Varreduras gerais não instanciam cada cena e podem perder problemas que só surgem em uma cena específica.
- Falhas do servidor de linguagem, parser ou compilação são retornadas como falhas, não resultados limpos.
- Diagnósticos provam que o código analisou ou compilou no contexto testado, não a correção da jogabilidade.

<a id="validatescene"></a>
### `validate_scene`

Verifica problemas estruturais em uma ou mais cenas e, quando possível, executa uma breve inicialização headless.

Comportamento funcional:

- Aceita até dez caminhos de cenas.
- Verifica scripts e recursos ausentes, caminhos inválidos, nomes irmãos duplicados, dependências cíclicas e referências exportadas relevantes.
- Referências opcionais ou atribuídas em runtime são observações, não falhas incondicionais.
- Cenas criadas com estrutura limpa recebem uma inicialização headless de três segundos, com logs e artefatos preservados.
- Descobertas repetidas são agrupadas para não inundar o resultado.

Limites e falhas importantes:

- Cenas-fonte importadas recebem apenas validação estrutural, pois não podem ser iniciadas diretamente como cenas criadas do projeto.
- O Fennara interrompe o processo intencionalmente após a janela de validação. Esse código de parada sozinho não é falha.
- Uma inicialização breve não valida todos os caminhos, visuais, desempenho, animações ou interações.
- Erros estruturais impedem a execução da cena.

<a id="visual-and-runtime-feedback"></a>
## Feedback visual e de runtime

<a id="screenshotscene"></a>
### `screenshot_scene`

Captura evidência visual de cenas criadas e recursos 3D importados compatíveis.

Comportamento funcional:

- Toda cena é instanciada em um SubViewport isolado. A captura não abre nem modifica a cena criada.
- O enquadramento 3D automático pode adicionar iluminação neutra quando não há ambiente ou luzes.
- `scene_path` é a única entrada obrigatória. Sem `code` e `script_path`, o Fennara captura a raiz separada com enquadramento automático.
- GDScript pode selecionar um nó ou array de nós, agrupar sujeitos, mostrar ou ocultar partes, alterar temporariamente a cena e pedir capturas com `ctx.capture(...)`. As mudanças são renderizadas, nunca salvas.
- `await ctx.capture(...)` renderiza o estado naquele ponto e retorna um `Image` comum. O worker pode inspecionar, comparar, redimensionar, descartar ou combinar imagens antes de publicar com `ctx.output(image, description)`.
- Para até oito sujeitos, quando uma captura 3D por script omite `view` e `camera`, o Fennara testa 17 pontos de vista determinísticos e escolhe um que favoreça visibilidade, tamanho legível, distância das bordas e pouca sobreposição. Use vista ou câmera explícita quando a direção for conhecida, e várias capturas quando sujeitos distantes ficarem pequenos.
- Um worker recebe apenas `ctx.root`, `await ctx.capture(...)`, `ctx.sheet(...)`, `ctx.output(...)`, `ctx.log(...)` e `ctx.error(...)`. `ctx.sheet(...)` compõe Images ordenadas pelo chamador em páginas determinísticas e opcionalmente rotuladas, sem escolher estados nem publicá-las. Pode passar uma Camera2D ou Camera3D temporária sob `ctx.root` nas opções.
- Caminhos de câmera ou alvo, retângulos de vista e parâmetros superiores de enquadramento não são aceitos. Seleção e enquadramento ficam no script.
- Toda imagem publicada é salva e listada. Clientes MCP e modelos integrados capazes de ver imagens recebem as primeiras seis como contexto separado na ordem. As demais ficam disponíveis pelo caminho, e o recibo informa a contagem omitida.
- Capturas esparsas retornam métricas de enquadramento e status parcial, sem ocultar a imagem.

Limites e falhas importantes:

- O enquadramento automático nem sempre deduz a vista artística de interiores, salas, níveis ou recursos incomuns.
- Uma imagem válida pode vir com validação indicando enquadramento esparso ou incerto.
- Modelos somente texto recebem recibo e caminhos, mas não veem os pixels.
- Falhas de carregamento, renderização, propriedade de captura ou salvamento são relatadas.
- Argumentos antigos desconhecidos são rejeitados com erro de migração.
- Erros de análise ou runtime, ausência de capturas, nós fora da raiz e câmeras temporárias inválidas são relatados sem captura.

<a id="runtimesession"></a>
### `runtime_session`

Inicia, verifica ou interrompe uma cena Godot em janela gerenciada pelo daemon.

Comportamento funcional:

- Barreiras de inicialização são executadas antes do processo da cena.
- Um início bem-sucedido retorna ID da sessão, estado do processo, caminhos de logs, descobertas iniciais e informações de captura.
- Status retorna nova saída sem descartar o log completo.
- Stop retorna informações finais do processo e log.
- Projetos C# recebem compilação real no Debug normal do Godot antes da inicialização, usando assemblies atuais.
- O log de runtime é a fonte oficial para saída do Godot, erros, marcadores auxiliares, capturas e eventos de parada.

Limites e falhas importantes:

- Apenas uma sessão de runtime gerenciada fica ativa globalmente.
- Falhas nas barreiras impedem a abertura.
- Uma compilação C# pode acionar a recarga normal de assemblies no editor aberto.
- Marcadores de prontidão podem chegar após a resposta inicial e aparecer em um status posterior.
- Sessões gerenciadas são processos separados do Godot, não a cena executada manualmente no editor.

<a id="runtimescript"></a>
### `runtime_script`

Executa uma sondagem GDScript limitada ou controlador de entrada dentro de uma sessão gerenciada ativa.

Comportamento funcional:

- Pode inspecionar nós ativos, registrar descobertas, aguardar estados, enviar entrada mapeada ou de baixo nível, fazer raycasts, interagir com UI básica e capturar frames.
- Pode coletar Images não salvas com `ctx.frame()`, compor sheets controladas pelo chamador com `ctx.sheet()` e publicar Images derivadas com `ctx.output()` sem exibi-las no jogo.
- Um script pode terminar mantendo a cena aberta para outra sondagem.
- Resultados incluem diagnósticos, descobertas de runtime, caminhos de capturas e logs e estado da sessão.

Limites e falhas importantes:

- Exige um ID válido de `runtime_session` ativa.
- Scripts de runtime não são scripts `@tool` e não servem como workers de edição.
- Diagnósticos inválidos, tempos limite, erros, sessões fechadas e nós indisponíveis são relatados.
- Sondagens devem permanecer limitadas. Não substituem uma estrutura permanente de automação da jogabilidade.

<a id="scrapeeditor"></a>
### `scrape_editor`

Lê um snapshot compacto do depurador depois que o usuário executa manualmente uma cena pelo editor.

Comportamento funcional:

- Agrupa problemas repetidos e limita detalhes ruidosos.
- Ajuda a inspecionar saída executada pelo editor que não pertence a uma sessão gerenciada.

Limites e falhas importantes:

- É intencionalmente mais limitado que ler cada elemento ou linha de log do editor.
- Não deve ser usado para cenas iniciadas por `runtime_session`, pois o log gerenciado é mais completo.
- Pode não haver estado útil quando nada foi executado manualmente.

<a id="built-in-chat-tools-and-controls"></a>
## Ferramentas e controles do chat integrado

<a id="readfile"></a>
### `read_file`

Lê arquivos de texto e imagens compatíveis limitados ao projeto usando caminhos
do Godot. É útil quando a normalização `res://` ou imagens importam. A navegação
ampla do código continua pertencendo às ferramentas normais do repositório.

<a id="execcommand"></a>
### `exec_command`

Executa um comando não interativo com a raiz do projeto ativo como diretório padrão.

Comportamento funcional:

- Captura saída e erro padrão com limites de tempo e tamanho.
- Rejeita diretórios de trabalho fora da raiz do projeto ativo.
- Armazena um recibo bruto no daemon para que saídas grandes não precisem permanecer na conversa.

Limites e falhas importantes:

- É restrição à raiz e controle de aprovação, não um sandbox do sistema operacional.
- Não oferece terminal interativo, PTY, sessão em segundo plano, entrada padrão nem configuração arbitrária de ambiente.
- Saídas diferentes de zero, tempos limite e truncamento são relatados.

<a id="chat-controls"></a>
### Controles do chat

O chat integrado oferece modos de aprovação para chamadas que alteram o projeto
ou executam runtime. Inspeções somente leitura podem ser imediatas, enquanto
alterações ou execução podem exigir aprovação. Full access remove esses prompts,
mas não contorna verificações rígidas de segurança.

Código selecionado no editor de scripts pode ser anexado com **Add to Chat**.
O compositor mostra o anexo antes do envio. `/provider` abre a configuração do
provedor e `/model` a seleção do modelo. São comandos de chat, não ferramentas MCP.

<a id="what-fennara-does-not-replace"></a>
## O que o Fennara não substitui

Use ferramentas normais de desenvolvimento para:

- pesquisa e navegação ampla no repositório
- leitura comum de arquivos de texto
- diffs e controle de versão
- edições que não precisam de feedback do Godot
- trabalho geral de shell

Use o Fennara quando a resposta depender do Godot entender, importar,
serializar, renderizar, validar ou executar o projeto.
