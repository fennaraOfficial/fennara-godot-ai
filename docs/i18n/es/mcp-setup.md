<!-- fennara-i18n: locale=es source=docs/mcp-setup.md sha256=86c9fe3fc7a69c2ade417dd01a0ccabb05ddaa91cf417fa8559c28d4b01811bd -->
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

La configuración escribe una entrada global y neutral respecto al proyecto.
Ejecutar `fennara mcp-setup` dentro de un proyecto no vincula todas las
conexiones futuras a ese proyecto.

<a id="bind-a-connection-to-one-project"></a>
## Vincular una conexión a un proyecto

Para varios repositorios o árboles de trabajo en el mismo equipo, ejecuta un
proceso y una conexión MCP por proyecto. Configura ese proceso en los ajustes de
proyecto o espacio de trabajo del host MCP mediante una de estas opciones:

```text
--project-path /absolute/path/to/godot-project
```

o:

```text
FENNARA_PROJECT_PATH=/absolute/path/to/godot-project
```

El runtime selecciona una vez su Vinculación de proyecto durante el inicio, en
este orden:

1. `--project-path`
2. `FENNARA_PROJECT_PATH`
3. el ancestro más cercano del directorio de inicio que contenga `project.godot`
4. modo de compatibilidad heredado sin vinculación cuando la detección no
   encuentra ningún proyecto

Una ruta explícita no válida impide que el servidor MCP se inicie. Nunca recurre
al destino del panel ni a otro editor. Una vinculación válida permanece activa
si su editor está ausente temporalmente y se recupera cuando esa Raíz del
proyecto vuelve a conectarse. No hay una anulación de proyecto por herramienta
visible para el modelo.

Consulta [Varios agentes y árboles de trabajo](multi-agent-worktrees.md) para
ver ejemplos de configuración, los límites de compatibilidad de los hosts, la
verificación del estado, el comportamiento ante editores duplicados y las
pruebas de juego serializadas.

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

Para una entrada local del proyecto que deba permanecer aislada, añade su
vinculación a `args`:

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

Para vincular una entrada al estilo de Codex, añade el argumento sin cambiar su
iniciador estable:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/godot-project"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

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

Los espacios de trabajo de una sola carpeta de VS Code pueden proporcionar el
proyecto como directorio de inicio del proceso MCP hijo. Claude Code, Gemini
CLI, Antigravity, Cline, Cursor, OpenCode, Kiro y Codex pueden utilizar
configuración de proyecto o espacio de trabajo; usa una vinculación explícita o
un directorio de inicio de proyecto documentado cuando debas garantizar el
aislamiento.

Claude Desktop y las versiones heredadas de Windsurf/Cascade utilizan
configuración global para este flujo de trabajo. Su configuración predeterminada
permanece en modo heredado sin vinculación. Los usuarios avanzados pueden crear
entradas globales con nombres distintos y rutas de proyecto explícitas
diferentes, pero esas aplicaciones no ofrecen aislamiento local del proyecto
automático.

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

Para trabajo aislado, confirma que el estado indique el modo de enrutamiento
`bound`, la fuente de vinculación y la Raíz del proyecto canónica esperadas, el
estado del editor vinculado `connected` y la disponibilidad del sistema de
archivos de ese editor.

Si el estado indica `legacy_unbound`, la conexión no encontró ninguna Raíz del
proyecto automática. Utiliza la ruta de compatibilidad del **MCP target** del
panel y advierte que este modo no es seguro para trabajo concurrente aislado.

<a id="troubleshooting"></a>
## Resolución de problemas

Si Fennara no aparece en la aplicación MCP:

- comprueba que la ruta del iniciador sea absoluta y exista
- comprueba que la sintaxis de la configuración sea JSON, JSON5 o TOML válido, según requiera la aplicación
- comprueba que el servidor se llame `fennara`
- comprueba que la aplicación lea el archivo de configuración que editaste
- cierra por completo y vuelve a abrir la aplicación MCP
- comprueba que el proyecto de Godot tenga instalado el addon de Fennara
- para una conexión vinculada, comprueba que su ruta explícita o directorio de
  inicio sea la Raíz del proyecto de Godot prevista
- si el estado indica `bound_project_not_connected`, abre ese proyecto en Godot
  y espera a que el addon se conecte
- si el estado indica `ambiguous_project_binding`, cierra el editor duplicado o
  ábrelo desde un árbol de trabajo distinto
- para una conexión heredada sin vinculación, comprueba que el proyecto previsto
  esté seleccionado como MCP target en el panel

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
