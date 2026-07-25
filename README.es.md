<!-- fennara-i18n: locale=es source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[Documentación](docs/i18n/es/README.md)

🌐 **Idiomas**

[English](README.md) · [简体中文](README.zh-CN.md) · **Español** · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Русский](README.ru.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/es/demos.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Utilizado por desarrolladores y equipos de Godot, incluido [Somni Game Studios](https://somnigamestudios.com/).

Fennara ofrece a los asistentes de IA una conexión en vivo con Godot. Puedes usarlo desde aplicaciones compatibles con MCP, como Codex, Claude, Cursor, Gemini y Antigravity, o desde el panel de chat opcional integrado en el editor.

Los agentes pueden inspeccionar escenas, comprobar scripts, capturar imágenes, leer errores de ejecución y validar cambios dentro del editor, en lugar de depender de suposiciones basadas únicamente en los archivos del proyecto.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Comparación de Fennara con otros MCP para Godot" width="100%" />
      </a>
    </td>
    <td>
      <strong>Mira la demostración destacada</strong><br />
      Comparación de Fennara con otros MCP para Godot.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Reproducir este video</a><br />
      <a href="docs/i18n/es/demos.md">Ver todos los videos de demostración</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## Qué hace

- expone herramientas que entienden Godot a aplicaciones externas de IA mediante MCP
- añade un panel de chat local opcional dentro del editor de Godot
- devuelve información real de Godot: árboles de escenas, diagnósticos, capturas de pantalla, registros de ejecución y resultados de validación
- hace que el agente responda ante el editor abierto, no solo ante el sistema de archivos

Las aplicaciones MCP externas y el chat integrado utilizan configuraciones de modelo independientes. Consulta [Aplicaciones MCP y chat integrado](docs/i18n/es/chat-vs-mcp.md) y [Proveedores del chat integrado](docs/i18n/es/providers.md).

<a id="requirements"></a>
## Requisitos

- Godot 4.5 o posterior.
- Un sistema operativo de escritorio compatible: Windows x86_64, Linux x86_64 o macOS arm64.
- Una aplicación de programación compatible con MCP solo si quieres usar Fennara desde Claude, Codex, Cursor, Gemini, Antigravity u otra aplicación externa de IA.
- Un proveedor de chat solo si quieres usar el panel de chat integrado de Fennara. Puede ser una clave de un proveedor en la nube o un proveedor local, como Ollama o LM Studio.

Para consultar el proceso completo de instalación, visita [Configuración](docs/i18n/es/setup.md).

<a id="what-setup-adds"></a>
## Qué añade la configuración

- el addon de Fennara, ubicado en `res://addons/fennara/`
- una pequeña CLI `fennara`, instalada en los datos de aplicación de Fennara
- un servidor MCP local utilizado por las aplicaciones de programación con IA
- un daemon local que conecta las solicitudes de MCP y del chat con el editor de Godot abierto
- instrucciones de proyecto generadas para los agentes de IA

El panel de chat integrado utiliza la vista web de cada plataforma: Microsoft Edge WebView2 en Windows, WKWebView/WebKit en macOS y un runtime CEF compartido, administrado por Fennara, en Linux. Las herramientas MCP siguen funcionando aunque el panel de chat opcional no pueda iniciarse.

<a id="install"></a>
## Instalación

En Windows y Linux, elige entre instalar el addon o utilizar la CLI. En macOS, usa la instalación mediante CLI que aparece a continuación si quieres evitar la notificación de seguridad de macOS que puede aparecer tras descargar y extraer manualmente el archivo ZIP del addon.

<a id="add-the-addon-to-your-project"></a>
### Añadir el addon a tu proyecto

- Abre la [versión más reciente](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest), descarga `fennara-addon-latest.zip` y extrae su carpeta `addons/fennara/` dentro de tu proyecto.

Abre el proyecto, selecciona el panel de Fennara y pulsa **Set Up Fennara**.

Fennara es una dependencia del editor, no una dependencia del runtime del
juego. Durante la exportación, el plugin del editor elimina su autoload del
runtime del proyecto exportado y omite `res://addons/fennara/` y
`res://.fennara/`. El proyecto del editor se restaura cuando finaliza la
exportación. Si un checkout de CI excluye el addon mediante `.gitignore`,
ejecuta `fennara prepare-export --project path/to/project` antes de iniciar
Godot, o instala el addon en ese checkout. Godot valida las rutas de los
autoload antes de que puedan ejecutarse los plugins de exportación, por lo que
esta preparación debe realizarse primero.

> **macOS:** El addon de la versión publicada contiene una biblioteca nativa que
> actualmente no está notarizada por Apple. Si descargas el ZIP del addon desde
> un navegador y lo extraes de forma manual, macOS puede indicar que no puede
> verificar que `libfennara.macos.editor` esté libre de software malicioso. Para
> evitar esta notificación, utiliza la instalación mediante CLI que aparece a
> continuación. Si la notificación ya se muestra, cierra Godot, elimina la
> carpeta `addons/fennara/` copiada manualmente e instala Fennara mediante la CLI.

<a id="install-with-the-cli-recommended-on-macos"></a>
### Instalar mediante la CLI (recomendado en macOS)

La CLI instala el mismo addon de Fennara. Es el método de instalación recomendado
en macOS porque evita la ruta de cuarentena del navegador y Finder que provoca la
notificación descrita anteriormente.

Instala la CLI en Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

O en macOS y Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Después, ejecuta Fennara desde tu proyecto de Godot:

```bash
cd path/to/your-godot-project
fennara install
```

Consulta [Configuración](docs/i18n/es/setup.md) para resolver problemas y
[CLI de Fennara](docs/i18n/es/cli.md) para ver la referencia completa de comandos.

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## Configurar un proveedor o conectar una aplicación MCP

<a id="built-in-chat"></a>
### Chat integrado

Abre **Chat Settings > Chat**, selecciona **Open providers** y conecta un proveedor.
Fennara utiliza tu propia clave para los proveedores en la nube (BYOK). También
puedes usar un servidor local de Ollama o LM Studio. Consulta la [lista de
proveedores compatibles](docs/i18n/es/providers.md).

<a id="mcp-apps"></a>
### Aplicaciones MCP

Abre **Chat Settings > MCP Apps**, busca tu aplicación y pulsa **Set Up**.

También puedes conectar una aplicación desde la terminal:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Si tu aplicación MCP no aparece en Chat Settings, consulta [Configuración de MCP](docs/i18n/es/mcp-setup.md)
para ver la lista completa de aplicaciones y las instrucciones de configuración manual.

<a id="update"></a>
## Actualización

Cuando el panel de Fennara muestre **Update**, púlsalo y sigue las indicaciones.

> **Actualización desde Fennara v0.3.8 o anterior:** Vuelve a instalar una vez la
> CLI con el comando de instalación correspondiente a tu plataforma antes de
> ejecutar `fennara update`. Esas versiones de la CLI resuelven una etiqueta de
> versión retirada y no pueden encontrar las versiones actuales. Reinstalar la
> CLI hace que las actualizaciones futuras utilicen el punto de acceso Latest
> Release de GitHub, sin eliminar el addon ni la configuración existentes del
> proyecto.

> **Usuarios de macOS que actualizan desde Fennara v0.3.11:** Vuelvan a instalar
> una vez la CLI con el comando de instalación de macOS antes de actualizar. La
> CLI v0.3.11 rechaza el paquete de framework de macOS existente antes de poder
> actualizarse a sí misma. La reinstalación solo sustituye la CLI, no elimina el
> addon ni la configuración del proyecto.

Para actualizar desde la terminal, cierra Godot y ejecuta:

```bash
cd path/to/your-godot-project
fennara update
```

Consulta [Actualizar Fennara](docs/i18n/es/setup.md#actualizar-fennara) para ver los pasos de recuperación y diagnóstico.

<a id="tools"></a>
## Herramientas

Fennara expone un conjunto reducido de herramientas que entienden Godot:

- escribir o actualizar archivos del proyecto y devolver diagnósticos
- ejecutar scripts puntuales de edición de escenas
- inspeccionar árboles de escenas, nodos, recursos y clases de Godot
- validar escenas
- capturar imágenes
- iniciar sesiones de ejecución y leer sus registros
- ejecutar pequeños scripts durante la ejecución sobre una escena activa

El objetivo no es sustituir las herramientas de archivos habituales de un agente. Fennara proporciona el ciclo de información de Godot que faltaba.

<a id="privacy"></a>
## Privacidad

Fennara envía como máximo un evento anónimo de instalación activa por día UTC
después de que Godot se conecte. Contiene un UUID aleatorio de instalación, las
versiones de Fennara y Godot, el sistema operativo y la arquitectura de CPU. No
contiene datos ni rutas del proyecto, prompts, actividad de herramientas,
registros, capturas de pantalla ni información de cuentas.

La telemetría puede desactivarse en **Chat Settings > Chat > Anonymous telemetry**,
con `FENNARA_DISABLE_TELEMETRY=true` o con `DO_NOT_TRACK=1`. Consulta
[Telemetría anónima](docs/i18n/es/telemetry.md) para conocer el contenido completo,
el almacenamiento, el transporte y las opciones de exclusión.

<a id="demos"></a>
## Demostraciones

Mira una demostración práctica de Fennara:

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

Más videos:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

Consulta [Demostraciones](docs/i18n/es/demos.md) para ver más videos del canal de Fennara.

<a id="star-history"></a>
## Historial de estrellas
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Gráfico del historial de estrellas" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## Documentación

| Empieza por... | Cuando necesites... |
| --- | --- |
| [Inicio de la documentación](docs/i18n/es/README.md) | Todas las guías y páginas de referencia |
| [Configuración](docs/i18n/es/setup.md) | Instalación, actualizaciones y resolución de problemas |
| [Proveedores de chat](docs/i18n/es/providers.md) | Modelos y claves del chat integrado |
| [Configuración de MCP](docs/i18n/es/mcp-setup.md) | Codex, Claude, Cursor y otras aplicaciones MCP |
| [Herramientas](docs/i18n/es/tools.md) | La información de Godot disponible para los agentes |
| [Telemetría anónima](docs/i18n/es/telemetry.md) | Datos recopilados, comportamiento del envío y controles de exclusión |
| [Contribuir](docs/i18n/es/CONTRIBUTING.md) | Orientación para el desarrollo y las solicitudes de incorporación de cambios |

<a id="community"></a>
## Comunidad

Las preguntas, la ayuda con la configuración y los comentarios iniciales son bienvenidos en Discord:

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## Licencia

Consulta [LICENSE.md](LICENSE.md).
