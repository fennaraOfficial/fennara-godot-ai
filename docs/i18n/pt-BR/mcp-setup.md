<!-- fennara-i18n: locale=pt-BR source=docs/mcp-setup.md sha256=86c9fe3fc7a69c2ade417dd01a0ccabb05ddaa91cf417fa8559c28d4b01811bd -->
<a id="mcp-setup"></a>
# Configuração de MCP

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · **Português do Brasil** · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

Conecte um aplicativo externo de IA às ferramentas Godot do Fennara. O aplicativo
continua usando sua própria conta, assinatura ou configuração de API do modelo.

> [!NOTE]
> Isso não configura o chat integrado do Fennara. Consulte
> [Aplicativos MCP e chat integrado](chat-vs-mcp.md) se não souber qual caminho precisa.

<a id="quick-setup"></a>
## Configuração rápida

1. Conclua **Set Up Fennara** no dock do Godot.
2. Abra **Chat Settings > MCP Apps**.
3. Encontre seu aplicativo e pressione **Set Up**.
4. Reinicie o aplicativo.

O Fennara cria um backup antes de alterar a configuração MCP de um aplicativo.
A opção combinada **Claude** configura Claude Code e Claude Desktop. **Gemini
& Antigravity** configura os dois destinos compartilhados.

<a id="terminal-alternative"></a>
### Alternativa pelo terminal

Primeiro, execute `fennara install` dentro do projeto Godot e depois escolha um destino:

| Aplicativo | Comando |
| --- | --- |
| Claude Code e Claude Desktop | `fennara mcp-setup --claude` |
| Somente Claude Code | `fennara mcp-setup --claude-code` |
| Somente Claude Desktop | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini e Antigravity | `fennara mcp-setup --gemini` or `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

Execute `fennara mcp-setup --help` para ver a lista de destinos compatíveis com a CLI instalada.

A configuração grava uma entrada de inicializador global e neutra em relação
ao projeto. Executar `fennara mcp-setup` dentro de um projeto não vincula todas
as conexões futuras a esse projeto.

<a id="bind-a-connection-to-one-project"></a>
## Vincular uma conexão a um projeto

Para vários repositórios ou worktrees na mesma máquina, execute um processo e
uma conexão MCP por projeto. Configure esse processo nas configurações de projeto
ou workspace do host MCP usando:

```text
--project-path /absolute/path/to/godot-project
```

ou:

```text
FENNARA_PROJECT_PATH=/absolute/path/to/godot-project
```

O runtime seleciona sua Vinculação de projeto uma única vez na inicialização,
nesta ordem:

1. `--project-path`
2. `FENNARA_PROJECT_PATH`
3. o ancestral mais próximo do diretório de inicialização que contenha `project.godot`
4. modo de compatibilidade não vinculado legado quando a descoberta não encontra um projeto

Um caminho explícito inválido impede a inicialização do servidor MCP. Ele nunca
recorre ao destino do dock ou a outro editor. Uma vinculação válida permanece
ativa se o editor estiver temporariamente ausente e se recupera quando essa
Raiz do projeto se reconecta. Não há substituição de projeto por ferramenta
voltada ao modelo.

Consulte [Vários agentes e worktrees](multi-agent-worktrees.md) para ver exemplos
de configuração, limites de suporte dos hosts, verificação de status,
comportamento com editores duplicados e playtests serializados.

<a id="manual-setup"></a>
## Configuração manual

Use a configuração manual apenas quando o aplicativo não estiver listado, o
comando de configuração não conseguir encontrar o arquivo de configuração ou
você quiser intencionalmente editar a configuração MCP à mão.

Antes de editar, faça backup do arquivo. Depois, adicione um servidor MCP stdio
local chamado `fennara` que aponte para o inicializador MCP estável do Fennara.

Caminhos padrão do inicializador:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Use o caminho absoluto real da sua máquina. Não aponte aplicativos MCP para
`versions/<version>/fennara-mcp-runtime`. O inicializador estável em `bin/`
mantém as configurações funcionando entre atualizações do Fennara.

<a id="json-mcpservers"></a>
### JSON `mcpServers`

Muitos aplicativos MCP usam um objeto `mcpServers` de nível superior:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

Alguns usam a mesma chave `mcpServers`, mas exigem apenas `command`. Se a
configuração existente já tiver outros servidores, preserve essas entradas e
adicione somente o servidor `fennara`.

Para uma entrada local do projeto que precisa permanecer isolada, adicione a
vinculação a `args`:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/godot-project"],
      "env": {}
    }
  }
}
```

Configurações no estilo Cline também podem incluir um tempo limite maior para ferramentas, em segundos:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### JSON `servers` no estilo VS Code

Alguns clientes, incluindo configurações MCP de usuário ou projeto do VS Code,
usam um objeto `servers` de nível superior e exigem `type: "stdio"`:

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### JSON `mcp` no estilo OpenCode

A configuração JSON no estilo OpenCode usa um objeto `mcp` de nível superior. O tempo limite é em milissegundos:

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### TOML no estilo Codex

O Codex usa TOML:

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Não cole JSON em um arquivo TOML nem TOML em um arquivo JSON. Use o formato já utilizado pelo aplicativo.

Para vincular uma entrada no estilo Codex, adicione o argumento sem alterar o
inicializador estável:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/godot-project"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

<a id="common-config-locations"></a>
## Locais comuns de configuração

Estes são locais comuns usados pelo auxiliar de configuração do Fennara e por
clientes MCP atuais. Os aplicativos podem alterar seus caminhos, e alguns
oferecem configurações globais e locais do projeto. Se o aplicativo tiver um
comando como **Open MCP Config**, use-o em vez de adivinhar.

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

Workspaces de pasta única do VS Code podem fornecer o projeto como diretório de
inicialização do processo filho MCP. Claude Code, Gemini CLI, Antigravity,
Cline, Cursor, OpenCode, Kiro e Codex podem usar configurações de projeto ou
workspace; use uma vinculação explícita ou um diretório de inicialização do
projeto documentado quando o isolamento precisar ser garantido.

Claude Desktop e Windsurf/Cascade legado usam configuração global neste fluxo
de trabalho. A configuração padrão deles permanece no modo não vinculado legado.
Usuários avançados podem criar entradas globais com nomes distintos e caminhos
de projeto explícitos diferentes, mas esses aplicativos não fornecem isolamento
local automático por projeto.

<a id="timeout-guidance"></a>
## Orientações sobre tempos limite

Algumas ferramentas do Fennara podem demorar mais que um pequeno tempo limite
MCP padrão, pois podem pedir ao Godot que valide cenas, inspecione o estado de
execução, capture telas ou execute diagnósticos.

Use um tempo limite maior por ferramenta quando o cliente oferecer suporte:

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

Se o cliente não oferecer tempos limite por servidor, use sua configuração global documentada de tempo limite MCP.

<a id="verify-the-connection"></a>
## Verificar a conexão

Abra o projeto Godot e pergunte ao aplicativo MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Para trabalho isolado, confirme que o status informa o modo de roteamento
`bound`, a fonte da vinculação e a Raiz do projeto canônica esperadas, o estado
do editor vinculado como `connected` e a prontidão do sistema de arquivos desse
editor.

Se o status informar `legacy_unbound`, a conexão não encontrou uma Raiz do
projeto automaticamente. Ela usa a rota de compatibilidade do **MCP target** do
dock e avisa que esse modo não é seguro para trabalho concorrente isolado.

<a id="troubleshooting"></a>
## Solução de problemas

Se o Fennara não aparecer no aplicativo MCP:

- confirme que o caminho do inicializador é absoluto e existe
- confirme que a sintaxe é JSON, JSON5 ou TOML válida, conforme o aplicativo exige
- confirme que o servidor se chama `fennara`
- confirme que o aplicativo está lendo o arquivo que você editou
- encerre completamente e reabra o aplicativo MCP
- confirme que o projeto Godot tem o addon Fennara instalado
- para uma conexão vinculada, confirme se o caminho explícito ou diretório de
  inicialização é a Raiz do projeto Godot desejada
- se o status informar `bound_project_not_connected`, abra esse projeto no
  Godot e aguarde a conexão do addon
- se o status informar `ambiguous_project_binding`, feche o editor duplicado ou
  abra-o a partir de uma worktree diferente
- para uma conexão não vinculada legada, confirme se o projeto desejado está
  selecionado como destino MCP no dock

<a id="unsupported-mcp-apps"></a>
## Aplicativos MCP sem suporte

Se seu aplicativo MCP não estiver listado, primeiro encontre na documentação
oficial desse aplicativo o local e o formato da configuração MCP. Depois, peça
a um LLM a menor edição segura:

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

Revise o resultado antes de salvar e reinicie o aplicativo MCP.
