<!-- fennara-i18n: locale=es source=docs/README.md sha256=ab01a3fd8024bccc8ad878eb0ec4cb15defa770ed3feccd0eef4c56270c7e763 -->
<a id="fennara-documentation"></a>
# Documentación de Fennara

<!-- fennara-doc-nav:start -->
[English](../../README.md) · [简体中文](../zh-CN/README.md) · **Español** · [Português do Brasil](../pt-BR/README.md) · [日本語](../ja/README.md) · [한국어](../ko/README.md) · [Русский](../ru/README.md) · [Français](../fr/README.md) · [Deutsch](../de/README.md) · [Türkçe](../tr/README.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../README.md)
<!-- fennara-doc-nav:end -->

Empieza por la tarea que quieres completar. Cada página comienza con el camino
habitual y deja los detalles avanzados para más adelante.

<a id="languages"></a>
## Idiomas

Utiliza el menú de idiomas de la parte superior para permanecer en la misma
página en otro idioma. Consulta [Idiomas y estado de las traducciones](languages.md)
para conocer la cobertura, el estado de revisión y la política de fuente canónica.

<a id="start-here"></a>
## Primeros pasos

| Quiero... | Consultar... |
| --- | --- |
| Instalar Fennara | [Configuración](setup.md) |
| Conectar el chat integrado | [Proveedores de chat](providers.md) |
| Conectar Codex, Claude, Cursor u otra aplicación MCP | [Configuración de MCP](mcp-setup.md) |
| Actualizar o recuperar Fennara | [Actualizar Fennara](setup.md#actualizar-fennara) |
| Resolver un problema de configuración | [Resolución de problemas](setup.md#resolución-de-problemas) |

<a id="use-fennara"></a>
## Usar Fennara

| Guía | Contenido |
| --- | --- |
| [Aplicaciones MCP y chat integrado](chat-vs-mcp.md) | Qué cuenta de modelo utiliza cada opción |
| [Varios agentes y árboles de trabajo](multi-agent-worktrees.md) | Vincular cada conexión MCP con su propio proyecto de Godot mientras comparten un daemon |
| [Herramientas](tools.md) | Herramientas que entienden Godot y cuándo utilizarlas |
| [Ejemplos](examples.md) | Prompts para flujos de trabajo habituales en Godot |
| [Comandos con barra](slash-commands.md) | `/provider` y `/model` en el panel de chat |
| [Preguntas frecuentes](faq.md) | Respuestas breves a preguntas habituales |
| [Demostraciones](demos.md) | Videos y recorridos de proyectos |
| [Telemetría anónima](telemetry.md) | Datos recopilados, comportamiento del envío y controles de exclusión |

<a id="reference-and-recovery"></a>
## Referencia y recuperación

| Referencia | Úsala cuando... |
| --- | --- |
| [CLI de Fennara](cli.md) | Necesites comandos de terminal, diagnósticos o automatización |
| [Instalación manual](manual-install.md) | No puedas utilizar el instalador habitual |
| [Referencia de configuración de MCP](mcp-setup.md) | Necesites configuración manual o específica para una aplicación |
| [Referencia de proveedores](providers.md) | Necesites claves, identificadores de modelo o detalles de servidores locales |

<a id="for-contributors"></a>
## Para colaboradores

| Documento | Finalidad |
| --- | --- |
| [Contribuir](CONTRIBUTING.md) | Expectativas para contribuciones y solicitudes de incorporación de cambios |
| [Arquitectura](architecture.md) | Límites del sistema y flujos de ejecución |
| [Mapa del repositorio](repo-map.md) | Ubicación del código y de los archivos generados |
| [Proceso de publicación](release.md) | Empaquetado, manifiestos, validación y publicación |
| [Vocabulario del proyecto](CONTEXT.md) | Nombres comunes utilizados en el código y la documentación |
| [Seguridad](SECURITY.md) | Cómo informar de vulnerabilidades |
| [Metadatos de GitHub](github-metadata.md) | Descripción y temas del repositorio |
| [Paquete de Godot](contributors/godot-payload.md) | Límites de la fuente empaquetada del addon |
| [Addons de Godot](contributors/godot-addons.md) | Forma y reglas del directorio de addons |
| [Herramientas locales](contributors/local-tools.md) | CLI, daemon, servidor MCP y runtime local |
| [Auxiliares de ejecución](contributors/runtime-helpers.md) | Fuente de auxiliares del runtime de Godot |
| [Scripts del repositorio](contributors/scripts.md) | Automatización de compilación, sincronización, validación y empaquetado |
| [Interfaz de chat](contributors/chat-ui.md) | Fuente y reglas de diseño del chat opcional |

<a id="learn-from-examples"></a>
## Aprender mediante ejemplos

- [Fennara frente a un MCP tradicional para Godot](fennara-vs-traditional-godot-mcp.md)
- [Análisis de la demostración Open RPG](open-rpg-demo.md)
- [Ejemplos de prompts](examples.md)
