<!-- fennara-i18n: locale=es source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# Preguntas frecuentes

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · **Español** · [Português do Brasil](../pt-BR/faq.md) · [日本語](../ja/faq.md) · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · [Deutsch](../de/faq.md) · [Türkçe](../tr/faq.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../faq.md)
<!-- fennara-doc-nav:end -->

Empieza por [Configuración](setup.md) para la instalación y las actualizaciones.
Utiliza esta página para consultar respuestas breves y enlaces a la referencia detallada.

| Pregunta | Respuesta breve |
| --- | --- |
| ¿Necesito una clave de proveedor? | Solo para un proveedor en la nube del chat integrado |
| ¿Puedo utilizar una aplicación MCP externa? | Sí, utiliza su propia cuenta de modelo |
| ¿Fennara sube mi proyecto a un servidor de Fennara? | No |
| ¿Puedo tener abiertos varios editores de Godot? | Sí, elige en el panel el destino para MCP externo |

<a id="is-fennara-only-a-code-generator"></a>
## ¿Fennara es solo un generador de código?

No. Fennara es un flujo de trabajo para agentes que entiende Godot. Puede trabajar con archivos del proyecto, escenas, diagnósticos, errores de ejecución, capturas de pantalla y el contexto del editor de Godot.

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## ¿Fennara es simplemente otro servidor MCP de comandos para Godot?

No. MCP es una de las formas de utilizar Fennara desde aplicaciones como Codex, Claude, Cursor, Gemini y Antigravity. Fennara también incluye un panel de chat integrado opcional. La idea principal del producto es el ciclo de información de Godot: diagnósticos, validación, errores de ejecución, capturas de pantalla y resultados estructurados de las herramientas para que los agentes puedan corregir sus errores.

<a id="does-fennara-replace-godot-knowledge"></a>
## ¿Fennara sustituye el conocimiento de Godot?

No. Fennara no pretende hacer que Godot sea opcional. Está diseñado para que los agentes de IA respondan ante el motor real de Godot.

<a id="how-should-i-install-fennara"></a>
## ¿Cómo debo instalar Fennara?

En Windows y Linux, añade el addon, abre el panel de Fennara y pulsa **Set Up
Fennara**, o realiza la instalación desde la terminal. En macOS, instala mediante
la CLI para evitar la notificación de seguridad que puede aparecer al extraer
manualmente un ZIP del addon descargado desde un navegador. Consulta
[Configuración](setup.md) para ver ambas opciones.

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## ¿Por qué macOS indica que no puede verificar `libfennara.macos.editor`?

El addon de la versión publicada contiene una biblioteca nativa que actualmente
no está notarizada por Apple. Cuando el ZIP del addon se descarga desde un
navegador y se extrae manualmente, Finder puede propagar los metadatos de
cuarentena a esa biblioteca, lo que provoca la notificación de macOS.

Para evitarla, utiliza la [instalación mediante CLI](setup.md#instalar-desde-la-terminal-recomendado-en-macos).
Si la notificación ya aparece, cierra Godot, elimina la carpeta
`addons/fennara/` copiada manualmente, instala la CLI y ejecuta `fennara install`
desde el directorio del proyecto. La CLI instala el mismo addon sin pasar por la
ruta de cuarentena del navegador y Finder.

<a id="do-i-need-a-chat-provider-api-key"></a>
## ¿Necesito una clave de API de un proveedor de chat?

Solo si quieres utilizar un proveedor en la nube en el panel de chat integrado de Fennara. Los clientes MCP externos utilizan su propia configuración de modelo y aplicación, y pueden usar las herramientas MCP de Fennara sin introducir ninguna clave de proveedor en el chat de Fennara.

El chat integrado también puede utilizar Ollama o LM Studio de forma local sin
una clave de API en la nube. Consulta [Proveedores del chat integrado](providers.md).

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## ¿Por qué el panel solicita un proveedor si ya ejecuté `mcp-setup --claude`?

`fennara mcp-setup --claude` conecta Claude a las herramientas MCP para Godot de Fennara. No conecta el panel integrado de Fennara con Claude ni comparte tu suscripción de Claude con el chat de Fennara.

Utiliza Claude Code o Claude Desktop para el flujo MCP externo. Configura un proveedor independiente solo si quieres conversar dentro del panel de Fennara en Godot. Consulta [Aplicaciones MCP y chat integrado](chat-vs-mcp.md).

<a id="what-are-provider-and-model"></a>
## ¿Qué son `/provider` y `/model`?

Son comandos con barra del panel de chat integrado de Fennara. `/provider` abre el selector de proveedor. `/model` abre el selector de modelo. Son accesos directos de la interfaz, no herramientas MCP externas ni texto enviado al modelo. Consulta [Comandos con barra del chat integrado](slash-commands.md).

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## ¿Fennara envía mi proyecto de Godot a un servidor de Fennara?

No. En el flujo OSS normal, el cliente MCP, el daemon y el addon de Godot se
ejecutan localmente. El chat integrado solo envía solicitudes del modelo al
proveedor que configures, como OpenAI, Anthropic, OpenRouter, Ollama Cloud,
DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax o un servidor local de
Ollama o LM Studio.

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## ¿Qué proyecto recibe las llamadas a herramientas MCP si hay varios editores de Godot abiertos?

El daemon dirige las llamadas MCP externas al destino MCP activo. Utiliza el
control MCP target del panel de Fennara en Godot para elegir el proyecto. Las
sesiones de chat integrado permanecen vinculadas al editor de Godot que abrió
ese chat.

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## ¿Por qué Linux instala un runtime CEF independiente?

El chat incrustado en Linux utiliza el renderizado fuera de pantalla de CEF. El
paquete de CEF es grande, por lo que Fennara lo instala una sola vez en el
directorio de datos de aplicación de Fennara del usuario, en lugar de copiarlo
en el addon de cada proyecto de Godot.

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## ¿Debe contener el addon `libcef.so`?

No. `libcef.so`, los recursos de CEF, los paquetes de idioma y el auxiliar de
CEF pertenecen al runtime CEF compartido de Linux. El addon solo debe contener
los archivos del addon de Godot, los binarios de GDExtension, los archivos de la
interfaz de chat y pequeños binarios auxiliares incluidos, como ripgrep.

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## ¿Qué ocurre si no se puede iniciar la vista web del chat integrado?

Las herramientas MCP de Fennara siguen funcionando. Solo el panel de chat
opcional dentro del editor necesita la vista web de la plataforma. En Windows,
instala Microsoft Edge WebView2 Runtime si `fennara doctor` indica que falta. En
macOS, WKWebView procede del WebKit.framework del sistema. En Linux, ejecuta
`fennara update` para instalar o reparar el runtime CEF administrado por la
versión publicada.

También puedes utilizar la opción **Open chat in my system browser next time**
de Chat Settings. Conserva el mismo chat integrado de Fennara y la misma
configuración del proveedor, pero abre la interfaz mediante el daemon local en
el navegador del sistema en lugar de la vista web incrustada de Godot. Reinicia
Godot después de cambiar esta opción.

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## ¿Abrir el chat en el navegador utiliza Claude o mi aplicación MCP?

No. Mostrarlo en el navegador es únicamente una elección de interfaz y runtime
para el chat integrado de Fennara. Sigue utilizando el proveedor seleccionado en
la configuración del chat de Fennara. `fennara mcp-setup --claude` y comandos
similares configuran aplicaciones MCP externas, no el modelo del chat integrado.

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## ¿`fennara update` vuelve a escribir la configuración de las aplicaciones MCP?

No. `fennara update` actualiza, cuando sea necesario, la CLI instalada, el addon
del proyecto, el paquete de runtime local, las instrucciones de proyecto
generadas y los recursos de runtime administrados por la plataforma. Vuelve a
ejecutar `fennara mcp-setup` únicamente para configurar o reparar la
configuración de una aplicación MCP.

<a id="where-does-chat-history-live"></a>
## ¿Dónde se guarda el historial del chat?

El daemon almacena localmente el historial del chat y lo limita al proyecto de
Godot actual. Las claves de proveedores y sus URL locales también se almacenan
localmente mediante el daemon, fuera del proyecto de Godot.

<a id="what-should-agents-use-fennara-tools-for"></a>
## ¿Para qué deben utilizar los agentes las herramientas de Fennara?

Utiliza Fennara para obtener información que entienda Godot: árboles de escenas,
propiedades modificadas de nodos y recursos, diagnósticos, validación, sesiones
de ejecución, capturas de pantalla y estado del depurador del editor. Los
clientes MCP deben seguir utilizando sus propias herramientas habituales de
lectura y búsqueda de archivos, salvo que necesiten una herramienta específica
de Fennara.
