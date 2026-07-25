<!-- fennara-i18n: locale=es source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# Configuración de MCP

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · **Español** · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

Conecta una aplicación externa de IA con las herramientas para Godot de Fennara.
La aplicación sigue utilizando su propia cuenta de modelo, suscripción o
configuración de API.

> [!NOTE]
> Esto no configura el chat integrado de Fennara. Consulta
> [Aplicaciones MCP y chat integrado](chat-vs-mcp.md) si no sabes qué opción
> necesitas.

<a id="quick-setup"></a>
## Configuración rápida

1. Completa **Set Up Fennara** en el panel de Godot.
2. Abre **Chat Settings > MCP Apps**.
3. Busca tu aplicación y pulsa **Set Up**.
4. Reinicia la aplicación.

Fennara crea una copia de seguridad antes de cambiar la configuración MCP de una
aplicación. La opción combinada **Claude** configura Claude Code y Claude
Desktop. **Gemini & Antigravity** configura ambos destinos compartidos.

<a id="terminal-alternative"></a>
### Alternativa mediante terminal

Primero ejecuta `fennara install` dentro del proyecto de Godot y después elige un destino:

| Aplicación | Comando |
| --- | --- |
| Claude Code y Claude Desktop | `fennara mcp-setup --claude` |
| Solo Claude Code | `fennara mcp-setup --claude-code` |
| Solo Claude Desktop | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini y Antigravity | `fennara mcp-setup --gemini` o `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

Ejecuta `fennara mcp-setup --help` para ver la lista de destinos compatibles con
la CLI que tienes instalada.

<a id="manual-setup"></a>
## Configuración manual

Utiliza la configuración manual únicamente cuando tu aplicación no aparezca en
la lista, el comando de configuración no encuentre su archivo de configuración
o quieras editar deliberadamente la configuración MCP a mano.

Antes de editar, crea una copia de seguridad del archivo de configuración.
Después añade un servidor MCP local mediante stdio llamado `fennara` que apunte
al iniciador MCP estable de Fennara.

Rutas predeterminadas del iniciador:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Utiliza la ruta absoluta real de tu equipo. No dirijas las aplicaciones MCP a
`versions/<version>/fennara-mcp-runtime`. El iniciador estable de `bin/` permite
que la configuración siga funcionando después de actualizar Fennara.

<a id="json-mcpservers"></a>
### JSON con `mcpServers`

Muchas aplicaciones MCP utilizan un objeto `mcpServers` en el nivel superior:

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

Algunas aplicaciones utilizan la misma clave `mcpServers`, pero solo requieren
`command`. Si la configuración existente ya contiene otros servidores,
consérvalos y añade únicamente el servidor `fennara`.

Las configuraciones al estilo de Cline también pueden incluir un tiempo de
espera mayor para las herramientas, expresado en segundos:

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
### JSON `servers` al estilo de VS Code

Algunos clientes, incluida la configuración MCP de usuario o proyecto de
VS Code, utilizan un objeto `servers` en el nivel superior y requieren
`type: "stdio"`:

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
### JSON `mcp` al estilo de OpenCode

La configuración JSON al estilo de OpenCode utiliza un objeto `mcp` en el nivel
superior. Su tiempo de espera se expresa en milisegundos:

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
### TOML al estilo de Codex

Codex utiliza TOML:

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

No pegues JSON en un archivo TOML ni TOML en un archivo JSON. Utiliza el formato
que ya emplee la aplicación.

<a id="common-config-locations"></a>
## Ubicaciones habituales de la configuración

Estas son ubicaciones habituales utilizadas por el auxiliar de configuración de
Fennara y por clientes MCP actuales. Las aplicaciones pueden cambiar sus rutas,
y algunas admiten configuración global y local del proyecto. Si una aplicación
incluye un comando como **Open MCP Config**, utilízalo en lugar de adivinar.

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
## Recomendaciones sobre tiempos de espera

Algunas herramientas de Fennara pueden tardar más que el pequeño tiempo de
espera MCP predeterminado, ya que pueden pedir a Godot que valide escenas,
inspeccione el estado de ejecución, capture imágenes o ejecute diagnósticos.

Utiliza un tiempo de espera mayor por herramienta cuando el cliente lo permita:

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

Si un cliente no admite tiempos de espera por servidor, utiliza su configuración
global documentada de tiempo de espera MCP.

<a id="verify-the-connection"></a>
## Verificar la conexión

Abre el proyecto de Godot y pregunta a tu aplicación MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Si hay más de un proyecto de Godot abierto, utiliza el control **MCP target** del
panel de Fennara para elegir cuál recibe las llamadas externas a herramientas MCP.

<a id="troubleshooting"></a>
## Resolución de problemas

Si Fennara no aparece en la aplicación MCP:

- comprueba que la ruta del iniciador sea absoluta y exista
- comprueba que la sintaxis de la configuración sea JSON, JSON5 o TOML válido, según requiera la aplicación
- comprueba que el servidor se llame `fennara`
- comprueba que la aplicación lea el archivo de configuración que editaste
- cierra por completo y vuelve a abrir la aplicación MCP
- comprueba que el proyecto de Godot tenga instalado el addon de Fennara
- comprueba que el proyecto de Godot deseado esté seleccionado como destino MCP

<a id="unsupported-mcp-apps"></a>
## Aplicaciones MCP no compatibles

Si tu aplicación MCP no aparece en la lista, busca primero su ubicación y formato
oficial de configuración MCP. Después pide a un LLM el cambio seguro más pequeño:

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

Revisa el resultado antes de guardarlo y después reinicia la aplicación MCP.
