<!-- fennara-i18n: locale=es source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# Comandos con barra del chat integrado

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · **Español** · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

Los comandos con barra son accesos rápidos del panel de chat de Fennara dentro de Godot. Son comandos de la interfaz, no herramientas MCP ni prompts enviados al modelo.

Escribe `/` en el cuadro de redacción para abrir la paleta de comandos.

| Comando | Abre | Se utiliza para |
| --- | --- | --- |
| `/provider` | Selector de proveedor | Conectar un proveedor en la nube, configurar la URL de un proveedor local o cambiar de proveedor. |
| `/model` | Selector de modelo | Elegir un modelo del proveedor actual o conectado. |

<a id="how-they-behave"></a>
## Comportamiento

- Utiliza las teclas de flecha para desplazarte por las sugerencias de comandos.
- Pulsa Enter para ejecutar el comando seleccionado.
- Pulsa Escape para cerrar la paleta de comandos.
- El texto del comando con barra se elimina del cuadro antes de enviar el mensaje de chat.

<a id="common-flow"></a>
## Flujo habitual

Para el panel de chat integrado:

```text
/provider
```

Conecta OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, Ollama local o LM Studio.

Después:

```text
/model
```

Elige el modelo que quieres que utilice el panel.

No utilices estos comandos con barra para aplicaciones MCP externas. Configura la aplicación con `fennara mcp-setup` y después pídele que utilice las herramientas MCP de Fennara.
