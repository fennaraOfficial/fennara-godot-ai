<!-- fennara-i18n: locale=pt-BR source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# Configuração de MCP

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

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

Se mais de um projeto Godot estiver aberto, use o controle **MCP target** do dock do Fennara para selecionar qual recebe chamadas MCP externas.

<a id="troubleshooting"></a>
## Solução de problemas

Se o Fennara não aparecer no aplicativo MCP:

- confirme que o caminho do inicializador é absoluto e existe
- confirme que a sintaxe é JSON, JSON5 ou TOML válida, conforme o aplicativo exige
- confirme que o servidor se chama `fennara`
- confirme que o aplicativo está lendo o arquivo que você editou
- encerre completamente e reabra o aplicativo MCP
- confirme que o projeto Godot tem o addon Fennara instalado
- confirme que o projeto Godot desejado está selecionado como destino MCP

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
