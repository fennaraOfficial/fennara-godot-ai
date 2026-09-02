<!-- fennara-i18n: locale=es source=docs/chat-vs-mcp.md sha256=b6f27b2c7e905515aba56b75bf6736644a9c36c885f4cab61555c82cd6c47fda -->
<a id="mcp-apps-or-built-in-chat"></a>
# ¿Aplicaciones MCP o chat integrado?

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · **Español** · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · [日本語](../ja/chat-vs-mcp.md) · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · [Deutsch](../de/chat-vs-mcp.md) · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara admite ambas opciones. Elige dónde quieres que tenga lugar la conversación.

| | Aplicación MCP externa | Chat integrado de Fennara |
| --- | --- | --- |
| Dónde conversas | Codex, Claude, Cursor, Gemini u otra aplicación MCP | El panel de Fennara o el navegador del sistema |
| Cuenta del modelo | La cuenta o suscripción de la aplicación externa | Un proveedor conectado en Fennara Chat Settings |
| Qué añade Fennara | Herramientas MCP que entienden Godot | Interfaz de chat, las mismas herramientas principales de Godot y herramientas de archivos y terminal exclusivas del chat |
| Configuración | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> Puedes utilizar ambas opciones. Sus configuraciones de modelo permanecen separadas.

<a id="external-mcp-apps"></a>
## Aplicaciones MCP externas

Conectar una aplicación MCP permite que esta inicie el servidor MCP local de
Fennara y llame a herramientas de Godot. No comparte la suscripción ni el inicio
de sesión de la aplicación con el chat integrado.

Configura una aplicación desde **Chat Settings > MCP Apps** o utiliza la CLI:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

No se necesita ninguna clave de proveedor de chat de Fennara. Reinicia la
aplicación externa después de configurarla. Consulta [Configuración de MCP](mcp-setup.md)
para ver todos los destinos y la configuración manual.

<a id="built-in-chat"></a>
## Chat integrado

El chat integrado necesita un proveedor conectado en Fennara Chat Settings.
Utiliza tu propia clave para un proveedor en la nube o conecta un servidor local
de Ollama o LM Studio.

El mismo chat puede aparecer dentro del panel de Godot o en el navegador del
sistema. Esta elección de visualización no cambia su proveedor, modelo, historial
ni proyecto.

Para adjuntar código, selecciónalo en el editor de scripts de Godot, abre el menú
contextual y elige **Add to Chat**. Consulta [Proveedores del chat integrado](providers.md)
para configurar proveedores y modelos.

<a id="project-routing"></a>
## Enrutamiento de proyectos

Ambas opciones utilizan el daemon local de Fennara para obtener información de Godot.

- Un proceso MCP externo puede vincularse una vez durante el inicio a una Raíz
  de proyecto de Godot canónica. Sus llamadas se enrutan al editor
  correspondiente sin leer ni cambiar el **MCP target** del panel.
- Un proceso MCP externo sin vinculación mantiene el comportamiento de
  compatibilidad: utiliza el Destino MCP seleccionado en el panel o el único
  editor conectado cuando no hay un destino válido seleccionado.
- El chat integrado permanece vinculado al editor de Godot que lo abrió.

Utiliza un proceso y una conexión MCP por proyecto para agentes aislados que
trabajen en repositorios o árboles de trabajo distintos. Consulta
[Varios agentes y árboles de trabajo](multi-agent-worktrees.md) para conocer la
configuración y el comportamiento de la Ranura de ejecución.

Para verificar una conexión MCP externa, pregunta:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Antes de trabajar en paralelo, comprueba que el estado indique el modo de
enrutamiento `bound` y la Raíz del proyecto canónica esperada. El modo heredado
sin vinculación incluye una advertencia de concurrencia.
