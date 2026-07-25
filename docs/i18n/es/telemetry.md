<!-- fennara-i18n: locale=es source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# Telemetría anónima

<!-- fennara-doc-nav:start -->
[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · **Español** · [Português do Brasil](../pt-BR/telemetry.md) · [日本語](../ja/telemetry.md) · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · [Deutsch](../de/telemetry.md) · [Türkçe](../tr/telemetry.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara envía como máximo un pequeño evento anónimo de actividad por día UTC.
El evento solo se envía después de que un editor de Godot compatible se conecte
al daemon local. Ayuda a los responsables a medir las instalaciones activas, el
uso de las plataformas compatibles y la adopción de versiones.

La telemetría está activada de forma predeterminada. Abre **Chat Settings > Chat
> Anonymous telemetry** para desactivarla. Los entornos sin interfaz gráfica y
automatizados pueden establecer cualquiera de estas variables:

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

Una variable de entorno tiene prioridad sobre la preferencia guardada en la
interfaz. Desactivar la telemetría detiene los eventos futuros y elimina la
identidad de telemetría local y el estado del último envío. Al volver a activarla,
se crea una nueva identidad aleatoria la próxima vez que Godot se conecte.

<a id="event-contents"></a>
## Contenido del evento

El evento `fennara_active_installation` solo contiene:

| Campo | Finalidad |
| --- | --- |
| `schema_version` | Versión del pequeño contrato de datos de telemetría |
| `event` | Nombre fijo del evento |
| `installation_id` | UUID aleatorio generado localmente, no derivado del hardware ni de cuentas |
| `fennara_version` | Versión del daemon en ejecución |
| `godot_version` | Versión numérica de Godot, como `4.6.3` |
| `platform` | `windows`, `macos` o `linux` |
| `architecture` | `x86_64` o `aarch64` |

Fennara no envía nombres ni rutas de proyectos, información de cuentas, prompts,
mensajes de chat, claves de proveedores, nombres de modelos, nombres, argumentos
o resultados de herramientas, registros, capturas de pantalla, contenido de
escenas, nombres de archivos ni texto de errores.

<a id="storage-and-transport"></a>
## Almacenamiento y transporte

El daemon almacena su identidad aleatoria y el último día UTC enviado correctamente
en el directorio compartido de datos de aplicación de Fennara:

```text
Fennara/
  telemetry/
    state.json
```

El daemon envía el evento mediante HTTPS a
`https://fennara.io/api/telemetry`. El receptor valida una lista exacta de campos
permitidos y sustituye el UUID de instalación original por un HMAC del servidor
antes de reenviar el evento a PostHog. Los perfiles de persona y la geolocalización
por IP de PostHog están desactivados para este evento.

El receptor de Vercel observa necesariamente los metadatos normales de red al
procesar la solicitud HTTPS. Esos metadatos no se copian en los datos del evento
de PostHog.

<a id="delivery-behavior"></a>
## Comportamiento del envío

La telemetría se ejecuta fuera de las rutas de llamadas a herramientas de Godot:

- Una cola limitada acepta señales de actividad sin esperar.
- Un único proceso de trabajo en segundo plano reutiliza un solo cliente HTTP.
- Las solicitudes tienen un tiempo de espera breve.
- Una cola llena, un problema del sistema de archivos, un fallo de red o el
  rechazo del servidor se toleran silenciosamente y nunca hacen fallar una
  herramienta de Fennara.
- El día UTC solo se registra después de que el servidor acepte un evento, de
  modo que una conexión posterior de Godot pueda reintentar un envío fallido.
- Al cerrar, espera brevemente y después cancela el proceso de telemetría en
  lugar de retrasar el daemon.

Una instalación corresponde a un UUID aleatorio guardado. Utilizar Fennara en
dos equipos cuenta como dos instalaciones. Borrar los datos de aplicación de
Fennara, o desactivar y volver a activar la telemetría, crea una identidad nueva.

Las instalaciones activas mensuales se cuentan como identidades de instalación
anónimas distintas que enviaron al menos un evento `fennara_active_installation`
durante el mes natural.
