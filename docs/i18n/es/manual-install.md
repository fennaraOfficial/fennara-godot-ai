<!-- fennara-i18n: locale=es source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# Instalación manual

<!-- fennara-doc-nav:start -->
[Documentación](README.md)

🌐 **Idiomas**

[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · **Español** · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Utiliza esta página únicamente cuando necesites montar Fennara sin el flujo de
configuración de Godot ni `fennara install`.

> [!TIP]
> En Windows y Linux, la mayoría de los usuarios deben añadir `addons/fennara`
> al proyecto, abrir el panel de Fennara y pulsar **Set Up Fennara**. En macOS,
> utiliza la CLI. Consulta [Configuración](setup.md).

> [!IMPORTANT]
> No se recomienda instalar manualmente el ZIP del addon en macOS. El addon
> contiene una biblioteca nativa que actualmente no está notarizada por Apple.
> Descargarla desde un navegador y extraerla mediante Finder puede hacer que
> macOS indique que no puede verificar que `libfennara.macos.editor` esté libre
> de software malicioso. Utiliza la [instalación mediante CLI](setup.md#instalar-desde-la-terminal-recomendado-en-macos)
> para evitar esta notificación. Si ya aparece, cierra Godot, elimina la carpeta
> `addons/fennara/` copiada manualmente y ejecuta `fennara install`.

La instalación manual consta de cuatro partes: la CLI, el addon del proyecto, el
paquete de runtime local compartido y la configuración opcional de aplicaciones MCP.

<a id="1-download-release-files"></a>
## 1. Descargar los archivos de la versión

Abre la versión más reciente de GitHub:

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

Descarga el manifiesto de la versión, los archivos de tu plataforma y el ZIP compartido del addon.

| Finalidad | Recurso |
| --- | --- |
| Plan de la versión y valores SHA-256 | `fennara-release-manifest-v<version>.json` |
| CLI para Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip` |
| Runtime local para Windows x86_64 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| CLI para Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip` |
| Runtime local para Linux x86_64 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Vista web incrustada para Linux x86_64 | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| CLI para macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip` |
| Runtime local para macOS arm64 | `fennara-release-local-macos-arm64-v<version>.zip` |
| Addon versionado para todas las plataformas | `fennara-release-addon-v<version>.zip` |

La versión también incluye este alias estable del addon para la documentación y
las descargas manuales:

```text
fennara-addon-latest.zip
```

El manifiesto registra los SHA-256 esperados para el runtime local, el addon y
los recursos de runtime compartidos. Utilízalo como fuente de verdad al comprobar
las descargas manuales.

<a id="2-install-the-cli"></a>
## 2. Instalar la CLI

Extrae el ZIP de `fennara-cli`.

Añade su directorio `bin` a PATH o copia el binario `fennara` en una de las
carpetas que ya estén incluidas en PATH.

Compruébalo:

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Instalar el addon de Godot

Extrae el ZIP de `fennara-addon`.

Copia:

```text
addons/fennara
```

dentro de tu proyecto de Godot, de modo que el proyecto contenga:

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. Instalar el paquete de runtime local

Normalmente, la CLI se encarga de esto. Solo necesitas configurar manualmente el runtime si no vas a utilizar `fennara install`.

Directorios predeterminados de datos de Fennara:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

La disposición esperada es:

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

En Windows, los binarios utilizan `.exe`.

`current.json` dirige los binarios de inicio a la versión de runtime activa. Los
comandos habituales `fennara install` y `fennara update` crean este archivo
automáticamente.

El chat incrustado en Linux utiliza la ubicación compartida
`webview/cef/linux-x64/<cef-version>/`. Las ejecuciones normales de
`fennara install` y `fennara update` instalan automáticamente el runtime CEF
administrado por la versión a partir del manifiesto y el recurso publicados. Si
instalas todo manualmente, extrae
`fennara-webview-cef-linux-x64-<cef-version>.zip` en esa ubicación compartida y
escribe el marcador `webview/cef/linux-x64/current.json` correspondiente.
Mantén este paquete fuera del addon del proyecto de Godot. `addons/fennara` no
debe contener `libcef.so` ni otros archivos del runtime CEF.

Este paquete CEF solo se utiliza para el chat incrustado en Linux. Los usuarios
pueden elegir **Open chat in my system browser next time** en Chat Settings para
mostrar el mismo chat integrado mediante el daemon local en el navegador del
sistema, en lugar de la vista web incrustada de Godot.

La disposición final de CEF en Linux debe ser:

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json` debe contener:

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` debe ser el
manifiesto de versión correspondiente al recurso CEF. Por ejemplo:

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

No coloques estado modificable del navegador dentro del directorio versionado
de CEF. El uso normal escribe perfiles y registros de cada editor bajo las
raíces de caché y registros de los datos de aplicación de Fennara, mientras que
el paquete de runtime permanece compartido y en modo de solo lectura.

<a id="5-configure-your-mcp-app"></a>
## 5. Configurar tu aplicación MCP

Después de instalar el paquete de runtime local, configura tu aplicación MCP:

```bash
fennara mcp-setup --claude
```

Otros destinos:

```bash
fennara mcp-setup --help
```

Reinicia la aplicación MCP después de configurarla.

Si tu aplicación no aparece en la lista, o si editas manualmente su configuración
MCP como parte de esta instalación, consulta [Configuración de MCP](mcp-setup.md)
para ver la ruta estable del iniciador y ejemplos JSON y TOML.

Esto solo conecta la aplicación MCP externa con las herramientas para Godot de
Fennara. No configura el proveedor de modelos del panel de chat integrado.
Configura el panel dentro de Godot si quieres utilizarlo, o consulta
[Aplicaciones MCP y chat integrado](chat-vs-mcp.md).

<a id="6-verify"></a>
## 6. Verificar

Abre el proyecto de Godot y pregunta a tu aplicación MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Si la ruta es correcta, la instalación manual funciona.

<a id="recommended-shortcut"></a>
## Atajo recomendado

Aunque instales la CLI de forma manual, puedes dejar que esta instale el addon y el paquete de runtime local:

```bash
cd path/to/your-godot-project
fennara install
```

La CLI también escribe instrucciones del proyecto para los agentes de programación con IA:

```text
AGENTS.md
addons/fennara/ai/
```

El directorio de IA contiene instrucciones compactas que se leen siempre, un
índice y páginas especializadas que solo se cargan cuando son relevantes. Un
ZIP del addon copiado manualmente puede incluir este directorio empaquetado,
pero no crea ni actualiza el archivo `AGENTS.md` en la raíz del proyecto.
Utiliza `fennara install` y `fennara update` cuando Fennara deba administrar y
actualizar todas las instrucciones del proyecto.
