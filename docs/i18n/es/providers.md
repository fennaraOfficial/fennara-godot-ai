<!-- fennara-i18n: locale=es source=docs/providers.md sha256=cb9c3750ebffd837ab24519959fdac8f39ba41cc32b1a8e0c467c500b56f7cb5 -->
<a id="built-in-chat-providers"></a>
# Proveedores del chat integrado

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · **Español** · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../providers.md)
<!-- fennara-doc-nav:end -->

Conecta un proveedor de modelos al panel de chat de Fennara dentro de Godot.

> [!NOTE]
> Las aplicaciones MCP externas utilizan su propia configuración de modelos. No
> necesitas conectar aquí un proveedor para utilizar Fennara desde Codex, Claude,
> Cursor u otra aplicación MCP. Consulta [Aplicaciones MCP y chat integrado](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Configuración rápida

1. Abre **Chat Settings > Chat** en el panel de Fennara.
2. Selecciona **Open providers**.
3. Elige un proveedor en la nube e introduce tu propia clave, o elige Ollama o
   LM Studio para utilizar un modelo local.
4. Selecciona un modelo.

También puedes escribir `/provider` y `/model` en el cuadro de redacción.

<a id="provider-reference"></a>
## Referencia de proveedores

| Proveedor | Cómo conectarlo | Formato del identificador de modelo | Notas |
| --- | --- | --- | --- |
| OpenAI | Crea una clave en [OpenAI API keys](https://platform.openai.com/api-keys). Clave o variable de Fennara: `OPENAI_API_KEY`. | `openai/<model>` | Utiliza la API oficial de OpenAI. |
| Anthropic | Crea una clave en [Claude Console API keys](https://console.anthropic.com/settings/keys). Clave o variable de Fennara: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Utiliza la API oficial Messages de Anthropic. |
| OpenRouter | Crea una clave en [OpenRouter Keys](https://openrouter.ai/settings/keys). Clave o variable de Fennara: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | Utiliza la API de OpenRouter. |
| Ollama Cloud | Crea una clave en [Ollama API keys](https://ollama.com/settings/keys). Clave o variable de Fennara: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Utiliza la API alojada de Ollama, no el servidor local de Ollama. |
| DeepSeek | Crea una clave en [DeepSeek API keys](https://platform.deepseek.com/api_keys). Clave o variable de Fennara: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | Utiliza la API compatible con OpenAI de DeepSeek. |
| Z.AI | Crea una clave en [Z.AI API keys](https://z.ai/manage-apikey/apikey-list). Clave o variable de Fennara: `ZHIPU_API_KEY`. | `zai/<model>` | Utiliza la API compatible con OpenAI de Z.AI. |
| Moonshot AI | Crea una clave en [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys). Clave o variable de Fennara: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Utiliza la API compatible con OpenAI de Moonshot. |
| Moonshot AI (China) | Crea una clave en [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys). Clave o variable de Fennara: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Utiliza la API compatible con OpenAI de Moonshot China. |
| Kimi For Coding | Crea una clave en [Kimi Code Console](https://www.kimi.com/code/console). Clave o variable de Fennara: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Utiliza la API Messages compatible con Anthropic de Kimi. Requiere acceso a Kimi Code. |
| MiniMax | Crea una clave de pago por uso en [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview), **API Keys > Create new secret key**. Clave o variable de Fennara: `MINIMAX_API_KEY`. | `minimax/<model>` | Utiliza la API Messages compatible con Anthropic de MiniMax en `minimax.io`. |
| MiniMax Token Plan | Utiliza la Subscription Key de [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview), **Billing > Token Plan**. Clave o variable de Fennara: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Las Subscription Keys de Token Plan son distintas de las claves de API de pago por uso. |
| MiniMax (China) | Crea una clave de pago por uso en la página de claves de API de [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Clave o variable de Fennara: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | Utiliza la API Messages compatible con Anthropic de MiniMax China en `minimaxi.com`. |
| MiniMax Token Plan (China) | Utiliza la Subscription Key de la página Token Plan de [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Clave o variable de Fennara: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | Las Subscription Keys de China Token Plan son distintas de las claves de API de pago por uso. |
| NVIDIA | Crea una clave en [build.nvidia.com](https://build.nvidia.com/). Clave o variable de Fennara: `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | Utiliza la API NIM alojada de NVIDIA, compatible con OpenAI. |
| Ollama | Ejecuta un servidor local de Ollama. No requiere una clave de API en la nube. | `ollama/<local-model>` | De forma predeterminada utiliza `http://127.0.0.1:11434`. |
| LM Studio | Inicia el servidor local de LM Studio. De forma predeterminada no requiere clave. | `lmstudio/<local-model>` | De forma predeterminada utiliza `http://127.0.0.1:1234/v1`. Si tu servidor de LM Studio requiere autenticación, establece `LMSTUDIO_API_KEY` en el entorno del daemon. |

Los proveedores en la nube necesitan tu propia clave de API o clave de
suscripción. Los proveedores locales necesitan que el servidor local esté en
ejecución y tenga un modelo disponible.

Las selecciones de OpenRouter siempre utilizan el formato explícito
`openrouter/<provider>/<model>`. Las selecciones antiguas guardadas con el
formato `<provider>/<model>` se migran una vez al cargar la configuración, pero
ese formato heredado no se utiliza para nuevas rutas.

Fennara puede guardar desde el selector de proveedores las claves introducidas
en el panel. Chat Settings incluye un botón **Open providers** que abre el mismo
selector. Los nombres de clave y variable anteriores son los que reconoce
Fennara si prefieres utilizar variables de entorno. Las claves guardadas se
encuentran en los datos de aplicación locales del daemon, fuera del proyecto de
Godot.

<a id="custom-openai-compatible-providers"></a>
## Proveedores personalizados compatibles con OpenAI

Elige **Custom** al final del selector de proveedores para añadir un punto de
acceso compatible con OpenAI, como un enrutador local o una puerta de enlace de
API interna. Introduce:

- un identificador de proveedor único en minúsculas
- el nombre que se mostrará en Fennara
- una URL base que termine en la versión de la API, por ejemplo `http://localhost:20128/v1`
- una clave de API opcional
- uno o varios identificadores y nombres de modelos, longitudes de contexto y límites máximos de tokens de salida
- encabezados opcionales para las solicitudes

Los identificadores de modelo deben coincidir con lo que espera el punto de
acceso. Fennara los muestra como `<provider-id>/<model-id>` en el selector de
modelos, pero solo envía `<model-id>` al proveedor. El punto de acceso debe
implementar la solicitud `/chat/completions` compatible con OpenAI y el formato
de respuesta en streaming.

Las claves de API y los valores de encabezados personalizados utilizan el
almacén protegido de autenticación del daemon de Fennara. Las definiciones de
proveedores permanecen en los datos de aplicación locales administrados por el
daemon, fuera del proyecto de Godot. Unos límites de modelo precisos permiten
que Fennara compacte el historial de la conversación antes de que una solicitud
supere el contexto del modelo y mantenga los resúmenes generados dentro del
límite de salida. Los modelos personalizados guardados antes de que estos
campos estuvieran disponibles se cargan con valores de compatibilidad de 64.000
tokens de contexto y 4.096 tokens de salida.

Después de guardar, el proveedor personalizado aparece en el selector junto con
su número de modelos. Selecciona ese proveedor para volver a abrir el formulario
y añadir o cambiar el nombre de los modelos. Dejar vacía la clave de API
conserva la clave guardada. Los encabezados nuevos se combinan por nombre con
los encabezados guardados.

<a id="where-settings-live"></a>
## Ubicación de la configuración

Fennara almacena localmente mediante el daemon la configuración del chat integrado, fuera del proyecto de Godot:

- claves de API de proveedores
- valores de encabezados de proveedores personalizados
- definiciones de proveedores personalizados compatibles con OpenAI
- URL base de proveedores locales
- modelo seleccionado
- esfuerzo de razonamiento
- tiempo de espera de respuesta del proveedor
- modo de visualización del chat, incrustado en Godot o abierto en el navegador del sistema
- historial del chat

Esta configuración no se escribe en `res://addons/fennara/` ni se comparte con Claude, Codex, Cursor, Gemini u otras aplicaciones MCP externas.

<a id="provider-response-timeout"></a>
## Tiempo de espera de respuesta del proveedor

La opción **Provider response timeout** controla cuánto tiempo puede ejecutarse cada solicitud al modelo en el chat integrado. El valor predeterminado es de 120 segundos y admite valores de 30 a 3600 segundos. Aumentarlo puede ayudar a que los modelos locales más lentos o los turnos largos con muchas herramientas finalicen. El daemon aplica el tiempo de espera seleccionado a la solicitud del proveedor y la cancela si se alcanza ese límite.

<a id="chat-display-setting"></a>
## Configuración de visualización del chat

El diálogo Chat Settings incluye **Open chat in my system browser next time**.

Cuando esta opción está desactivada, Fennara intenta renderizar el chat integrado
dentro del panel de Godot. Cuando está activada, el panel muestra un botón
**Open chat** e inicia el mismo chat integrado mediante el daemon local en
`127.0.0.1`. Esto puede reducir el uso de GPU y memoria del editor de Godot y
también sirve como alternativa si no se puede iniciar la vista web nativa.

El cambio se aplica la próxima vez que se inicia Godot. Solo modifica dónde se
muestra la interfaz del chat integrado. No cambia el proveedor ni el modelo
seleccionados, las claves de API, el historial del chat, la configuración de
aplicaciones MCP ni el modelo que Claude, Codex o Cursor utilizan externamente.

<a id="picker-shortcuts"></a>
## Accesos directos de los selectores

Chat Settings, los controles del panel y `/provider` abren el mismo selector de
proveedores. Utiliza `/model` o el control de modelo del panel para abrir el
selector de modelos.

Consulta [Comandos con barra del chat integrado](slash-commands.md) para conocer el comportamiento de la paleta de comandos.

<a id="local-providers"></a>
## Proveedores locales

Para Ollama:

```bash
ollama serve
ollama pull llama3.1:8b
```

Después elige:

```text
ollama/llama3.1:8b
```

Las selecciones antiguas `local/<model>` se siguen aceptando como alias de
compatibilidad con Ollama. Para configuraciones nuevas, utiliza el formato
explícito `ollama/<model>`.

Para LM Studio, inicia el servidor local desde LM Studio y elige un identificador de modelo con este formato:

```text
lmstudio/<loaded-model-id>
```

<a id="model-catalog"></a>
## Catálogo de modelos

El daemon mantiene un catálogo local de modelos para los proveedores en la nube
y solicita a los servidores locales los modelos que tienen disponibles. Si un
catálogo o servidor local cambia mientras Godot está abierto, actualiza el
selector de modelos o vuelve a abrir el selector de proveedor o modelo.

Fennara comprueba las capacidades básicas del modelo antes de enviar una solicitud:

- se requiere salida de texto
- se requiere llamada a herramientas para utilizar herramientas de Fennara
- se requiere entrada de imágenes antes de enviar archivos de imagen adjuntos como contexto

La entrada de imágenes de Ollama todavía no está habilitada en el chat de Fennara.
