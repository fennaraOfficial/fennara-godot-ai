<!-- fennara-i18n: locale=es source=CONTEXT.md sha256=ee0d279d8a4916d5cf894616b1c72658669a36bf0ec958efef5a09ee196c704e -->
<a id="fennara-context"></a>
# Contexto de Fennara

<!-- fennara-doc-nav:start -->
[Documentación](README.md)

🌐 **Idiomas**

[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · **Español** · [Português do Brasil](../pt-BR/CONTEXT.md) · [日本語](../ja/CONTEXT.md) · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · [Deutsch](../de/CONTEXT.md) · [Türkçe](../tr/CONTEXT.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

Este archivo define términos comunes utilizados en la documentación, las incidencias, las notas de versión y las instrucciones para agentes de Fennara.

<a id="product-terms"></a>
## Términos del producto

**Fennara**

El entorno para agentes que entiende Godot incluido en este repositorio. Fennara conecta herramientas de IA con información real de Godot, como diagnósticos, validación de escenas, errores de ejecución, capturas de pantalla e instrucciones del proyecto.

**Addon de Godot**

El plugin instalable que se copia en el proyecto de Godot de un usuario, bajo `res://addons/fennara/`. Es responsable de la interfaz del panel, las herramientas de inspección de Godot, la biblioteca nativa GDExtension, los recursos empaquetados de la interfaz de chat, los scripts auxiliares de ejecución y la versión local del addon en el proyecto.

**CLI de Fennara**

El comando `fennara` instalado en el equipo del usuario. Gestiona la instalación, las actualizaciones, la actualización de la propia CLI, las comprobaciones de doctor, la configuración de aplicaciones MCP, las advertencias sobre requisitos de la vista web, las comprobaciones de configuración de C# y las instrucciones de proyecto generadas.

**Paquete local**

El archivo ZIP de una versión que contiene los ejecutables locales de Fennara, como el servidor MCP, el daemon, los binarios de ejecución y los binarios de inicio para una plataforma y arquitectura concretas.

**Instrucciones del proyecto**

Archivos de instrucciones generados dentro de un proyecto de Godot, incluidos `AGENTS.md` y las referencias correspondientes en `addons/fennara/ai/`, para que los agentes de programación con IA sepan cuándo y cómo usar Fennara.

<a id="mcp-terms"></a>
## Términos de MCP

**Servidor MCP de Fennara**

El servidor MCP local mediante stdio que inicia una aplicación de programación con IA, como Claude Code, Cursor, Cline, Gemini CLI u otro cliente MCP. Expone las herramientas de Fennara a esa aplicación externa.

**Aplicación MCP**

Una aplicación externa de IA configurada mediante `fennara mcp-setup`. La configuración de aplicaciones MCP controla qué aplicación externa puede llamar a las herramientas de Fennara. No selecciona el modelo que utiliza el chat integrado de Fennara.

**Destino MCP**

El proyecto de Godot seleccionado actualmente para recibir llamadas MCP de Fennara.

**Esquema de herramienta**

La descripción de una herramienta MCP de Fennara que recibe el modelo, incluidos sus argumentos, límites y notas de uso.

**Envoltorio de resultado de herramienta**

El resultado conciso dirigido al modelo que se devuelve tras llamar a una herramienta. Los resultados de Fennara deben explicar el estado, los hallazgos importantes y el contexto útil siguiente sin volcar datos sin procesar innecesarios.

<a id="built-in-chat-terms"></a>
## Términos del chat integrado

**Chat integrado**

La interfaz de chat propia de Fennara dentro del addon de Godot o del navegador del sistema. Es independiente de las aplicaciones MCP externas. Un usuario puede configurar Claude Code para MCP y elegir otro proveedor o modelo para el chat integrado.

**Superficie de chat**

El modo de visualización del chat integrado. El modo incrustado utiliza la vista web nativa del panel de Godot. El modo navegador sirve la misma interfaz desde el daemon local y la abre en el navegador del sistema.

**Proveedor de chat**

Un backend capaz de generar respuestas para el chat integrado, como OpenAI, Anthropic,
OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax,
Ollama local o LM Studio.

**Referencia de modelo**

El identificador de modelo, cualificado por proveedor, seleccionado en el chat integrado. Los comandos con barra, como `/provider` y `/model`, ayudan a conectar proveedores y elegir referencias de modelo.

**Conexión de proveedor**

La configuración local y el estado de autenticación de un proveedor de chat administrados por el daemon, incluidas claves de API o URL base locales. Los secretos del proveedor deben permanecer en el almacenamiento local administrado por el daemon, no dentro del proyecto de Godot.

**Traza de generación**

Metadatos almacenados de una generación del chat integrado que relacionan los mensajes del asistente, las llamadas a herramientas, la elección de proveedor y modelo, el uso y los registros de costes con la generación que los produjo.

<a id="runtime-and-webview-terms"></a>
## Términos de ejecución y vista web

**Daemon de Fennara**

El servicio local que conecta las llamadas MCP y las solicitudes del chat integrado con el addon de Godot, almacena el estado local de ejecución y sirve rutas de chat alojadas por el daemon, como `/chat/`.

**Sesión de ejecución**

Una sesión de ejecución de Godot administrada por el daemon que se utiliza para inspección durante la ejecución, registros, validación, capturas de pantalla y futuros flujos de trabajo sobre escenas activas.

**Instantánea de Godot**

Una instantánea reversible del estado del proyecto que se toma antes de un turno asistido por Fennara que pueda modificar archivos. La configuración de la instantánea debe finalizar antes de guardar el turno del usuario, para que un fallo de configuración no deje prompts huérfanos.

**Runtime de vista web**

El soporte de plataforma necesario para mostrar el chat integrado dentro de Godot o junto a él. Windows utiliza WebView2, macOS utiliza WebKit/WKWebView y Linux utiliza un runtime CEF compartido instalado en los datos de aplicación de Fennara.

**Runtime CEF compartido de Linux**

El paquete externo del runtime CEF de Linux utilizado por la vista web del chat. Se instala una sola vez en el directorio de datos de aplicación de Fennara y no debe incluirse en cada ZIP del addon de Godot.

<a id="release-terms"></a>
## Términos de publicación

**Manifiesto de versión**

El recurso JSON llamado `fennara-release-manifest-v<version>.json`. Asigna recursos de la versión a plataformas, registra hashes SHA-256, declara recursos de runtime compartidos y define `minimum_cli_version`.

**Versión mínima de la CLI**

La versión más antigua de la CLI `fennara` que puede consumir un manifiesto de
versión. Si una versión necesita una lógica de instalación o actualización más
reciente, actualiza su canal en `scripts/release-policy.mjs`. El escritor del
manifiesto aplica esa política después de validar la identidad de la versión.
Los flujos de trabajo no eligen el valor.

**Versión más reciente**

El puntero Latest Release de GitHub hacia una versión exacta. Los instaladores y
las actualizaciones predeterminadas resuelven este puntero mediante la API de
GitHub. Fennara no utiliza una etiqueta o versión literal llamada `latest`.
Actualizar los archivos fuente después de publicar no modifica los recursos de
una versión. Los recursos de manifiesto ya publicados deben sustituirse de forma
explícita.
