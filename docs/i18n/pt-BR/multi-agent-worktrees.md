<!-- fennara-i18n: locale=pt-BR source=docs/multi-agent-worktrees.md sha256=7b266e260017a37b18e3d8e36a6bed75e76c3bcc4ead88c49bec146302495014 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# Vários agentes e worktrees do Godot

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · [Español](../es/multi-agent-worktrees.md) · **Português do Brasil** · [日本語](../ja/multi-agent-worktrees.md) · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · [Deutsch](../de/multi-agent-worktrees.md) · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

Execute vários agentes de programação em repositórios ou worktrees separados
na mesma máquina sem permitir que a escolha de destino de um agente redirecione
outro. Cada projeto recebe seu próprio processo e conexão MCP do Fennara; todos
os projetos compartilham o mesmo daemon por usuário.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

Chamadas de edição, inspeção, validação e captura de tela podem ser executadas
simultaneamente. Execuções de jogos gerenciadas pelo daemon são serializadas por
meio de um único Slot de execução para toda a máquina.

<a id="one-mcp-connection-per-project"></a>
## Uma conexão MCP por projeto

Um processo MCP seleciona uma Raiz do projeto estável quando é iniciado. Essa
Vinculação de projeto MCP é uma identidade canônica do sistema de arquivos para
o diretório que contém `project.godot`; ela não é o nome de um projeto nem o ID
de um processo do Godot.

Use um processo e uma conexão MCP separados para cada repositório ou worktree.
Uma conexão pode atender vários agentes somente quando todos trabalham
intencionalmente no mesmo projeto. As ferramentas do Fennara não expõem um
seletor de projeto por chamada, portanto o modelo não pode mudar acidentalmente
um processo para outro projeto.

Cada projeto também precisa de um editor Godot conectado com o Fennara ativado.
Se um editor for fechado e se reconectar com um novo ID de processo, o processo
MCP existente voltará a encaminhar chamadas quando a mesma Raiz do projeto se
reconectar.

<a id="how-a-process-chooses-its-project"></a>
## Como um processo escolhe seu projeto

O runtime MCP captura seu diretório de trabalho de inicialização e seleciona sua
vinculação uma única vez, nesta ordem:

1. `--project-path <path>` ou `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. O ancestral mais próximo do diretório de inicialização que contenha `project.godot`.
4. Modo de compatibilidade não vinculado legado quando a descoberta automática
   não encontra um projeto Godot.

Os caminhos da linha de comando e do ambiente são declarações explícitas. Um
caminho vazio, inacessível, ausente, que não seja um diretório ou projeto Godot,
ou que não seja compatível impede a inicialização do servidor MCP; ele nunca
recorre a outro projeto. Caminhos relativos são resolvidos a partir do diretório
de inicialização capturado. Prefira um caminho absoluto quando o diretório de
inicialização do host MCP não estiver claro.

O Fennara não consome implicitamente variáveis de workspace específicas do host.
Um host MCP pode mapear seu próprio valor de workspace para `--project-path` ou
`FENNARA_PROJECT_PATH`.

<a id="configure-a-project-bound-connection"></a>
## Configurar uma conexão vinculada a um projeto

`fennara mcp-setup` permanece global e neutro em relação ao projeto. Executá-lo
dentro de um projeto não vincula todos os processos MCP futuros a esse projeto.
Mantenha o caminho estável do inicializador e use a configuração de projeto ou
workspace do host MCP para adicionar uma vinculação.

Para configuração no estilo JSON:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

Ou use o ambiente:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Para TOML no estilo Codex:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Configure o próximo agente em sua própria configuração de projeto/workspace com
`/absolute/path/to/worktree-b`. Se um host iniciar de forma confiável um processo
MCP separado a partir do diretório de cada projeto, a descoberta por ancestrais
pode fornecer a mesma vinculação sem um caminho explícito.

<a id="mcp-host-boundaries"></a>
## Limites dos hosts MCP

A configuração local do projeto e o comportamento do diretório de inicialização
variam entre os hosts:

- Workspaces de pasta única do VS Code podem depender do diretório de trabalho
  filho documentado pelo host, mas uma vinculação de projeto explícita ainda é
  a configuração mais clara.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro e Codex
  podem usar configurações de projeto/workspace. Use uma vinculação explícita
  ou um diretório de inicialização do projeto documentado quando o isolamento
  precisar ser garantido.
- A configuração do Claude Desktop e do Windsurf/Cascade legado é global. A
  entrada padrão do Fennara permanece no modo não vinculado legado e não pode
  fornecer isolamento local automático por projeto. Usuários avançados podem
  criar entradas globais com nomes distintos e caminhos explícitos diferentes,
  mas precisam escolher a entrada correta.

<a id="worktree-isolated-subagents"></a>
### Subagentes isolados em worktree

Alguns hosts iniciam um agente filho em um worktree Git separado herdando as
conexões MCP do pai. Claude Code `isolation: worktree` e o isolamento de
worktree do Grok Build `spawn_subagent` fazem isso.

As ferramentas nativas de arquivo e de shell passam a operar no worktree
filho. O Fennara permanece vinculado ao projeto pai, então o filho pode
editar uma árvore e inspecionar ou alterar outra.

Dê a esse subagente a própria conexão Fennara MCP vinculada ao worktree
filho, ou mantenha-o no projeto pai sem isolamento de worktree. Os
subagentes padrão do Codex e do OpenCode não estão documentados como
herdando o Fennara dessa forma.

A geração automática de configuração local do projeto e o novo suporte ao
Windsurf/Devin Local estão fora deste fluxo de trabalho.

<a id="start-and-verify-the-editors"></a>
## Iniciar e verificar os editores

Cada worktree precisa de seu próprio editor Godot com o Fennara ativado.
Editores headless podem usar portas LSP do Godot separadas enquanto
compartilham o daemon do Fennara:

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

As portas LSP pertencem ao Godot. O Fennara continua usando um único daemon
compartilhado em seu endereço de loopback normal.

Execute `fennara_status` em cada agente antes do trabalho concorrente. Confirme
se ele informa:

- modo de roteamento `bound`
- a fonte da vinculação e a Raiz do projeto canônica esperadas
- estado do editor vinculado `connected`
- prontidão do sistema de arquivos desse editor

Se a descoberta automática não encontrar um projeto, o status informa
`legacy_unbound` e um aviso de concorrência. Nesse modo de compatibilidade, o
Destino MCP selecionado no dock é usado primeiro, seguido pelo único editor
conectado. Não use uma conexão não vinculada para trabalho concorrente isolado.

<a id="missing-and-duplicate-editors"></a>
## Editores ausentes e duplicados

Uma Vinculação de projeto válida permanece ativa quando seu editor está ausente.
As chamadas de ferramentas retornam `bound_project_not_connected`, permitindo
nova tentativa, até que essa Raiz do projeto se reconecte; elas nunca recorrem
ao destino do dock.

Dois editores resolvidos para a mesma Raiz do projeto produzem
`ambiguous_project_binding`. Feche o editor duplicado ou atribua a ele uma
worktree diferente. O Fennara não escolhe pelo ID do processo, pela ordem das
conexões, pelo nome do projeto nem pelo destino do dock.

Aliases de links simbólicos para o mesmo projeto são resolvidos como a mesma
identidade ativa no sistema de arquivos. Redirecionar um link simbólico depois
da inicialização do MCP não altera uma vinculação; reinicie esse processo MCP
para vinculá-lo novamente.

<a id="serialized-runtime-sessions"></a>
## Sessões de execução serializadas

Todos os projetos compartilham um único Slot de execução para toda a máquina
para jogos gerenciados pelo daemon. Quando outro projeto está iniciando ou
executando uma sessão, `runtime_session.start` retorna um resultado de domínio
`busy` bem-sucedido com `availability: "busy"`, `slot_acquired: false` e um
`retry_after_ms` sugerido. Ele não expõe o proprietário, ID da sessão, ID do
processo, cena, logs, posição na fila nem duração esperada.

Não há fila FIFO. Consulte com jitter próximo ao atraso sugerido para nova
tentativa e trate cada `runtime_session.start` como a reivindicação atômica
final. Um status livre é apenas informativo, pois outro agente pode vencer a
disputa depois da verificação preliminar.

Somente a Raiz do projeto proprietária pode inspecionar, renovar, executar
scripts ou interromper sua Sessão de execução. O status do proprietário renova um
prazo de inatividade de 120 segundos. Uma operação limitada de runtime do
proprietário suspende a expiração por inatividade enquanto está ativa e renova o
prazo somente depois de retornar um resultado terminal do script; tempo limite,
erro de preparação ou cancelamento não o renovam. Os agentes devem consultar o
status do proprietário aproximadamente a cada 30 segundos, com jitter, enquanto
uma execução continua.

O Lease de execução absoluto padrão é de 900 segundos. `max_run_seconds` aceita
um inteiro positivo de até 86.400 segundos; por exemplo, uma regressão estimada
em uma hora pode solicitar 4.500 segundos como margem de segurança. O prazo
absoluto nunca é suspenso. Saída natural, interrupção explícita, falha de
inicialização, inatividade ou expiração absoluta interrompe ou recolhe o jogo e
libera o Slot de execução.

<a id="safe-multi-agent-checklist"></a>
## Checklist seguro para vários agentes

1. Crie um repositório ou worktree diferente para cada projeto.
2. Instale o Fennara e abra um editor Godot para cada Raiz do projeto.
3. Configure um processo MCP vinculado ao projeto para cada projeto.
4. Execute `fennara_status` em cada agente e verifique sua raiz canônica.
5. Permita que a edição, a inspeção, a validação limitada de cenas e as capturas
   de tela independentes prossigam simultaneamente.
6. Consulte e tente novamente resultados `busy` sem erro para playtests;
   mantenha a sessão vencedora ativa consultando o status do proprietário
   durante sua execução.
