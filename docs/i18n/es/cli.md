<!-- fennara-i18n: locale=es source=docs/cli.md sha256=561efbe26f75accb949050e7af9d06ae607040b86d061c04c4dcf8732ba61135 -->
<a id="fennara-cli"></a>
# CLI de Fennara

<!-- fennara-doc-nav:start -->
[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · **Español** · [Português do Brasil](../pt-BR/cli.md) · [日本語](../ja/cli.md) · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · [Deutsch](../de/cli.md) · [Türkçe](../tr/cli.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../cli.md)
<!-- fennara-doc-nav:end -->

Utiliza la CLI cuando prefieras la terminal, necesites diagnósticos o recuperación,
o quieras automatizar una instalación con una versión exacta.

> [!TIP]
> La CLI es el método de instalación recomendado en macOS. Evita la notificación
> de seguridad de macOS que puede aparecer al extraer manualmente un ZIP del
> addon descargado desde un navegador y hacer que su biblioteca nativa herede la
> cuarentena de Finder.

<a id="common-flow"></a>
## Flujo habitual

```bash
cd path/to/your-godot-project
fennara install
```

Utiliza `fennara doctor` cuando necesites inspeccionar o reparar la instalación local.

Consulta [Configuración](setup.md) para el recorrido habitual en Godot. Conserva
esta página como referencia de comandos de terminal.

<a id="install-the-cli"></a>
## Instalar la CLI

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS y Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Si un addon de macOS extraído manualmente ya provoca una notificación sobre
`libfennara.macos.editor`, cierra Godot y elimina la carpeta `addons/fennara/`
copiada manualmente antes de ejecutar `fennara install`. De lo contrario, la CLI
conserva un addon completo ya existente.

Abre una terminal nueva si `fennara` no está disponible de inmediato y comprueba
la instalación:

```bash
fennara --version
fennara doctor
```

La CLI se instala por usuario. Los addons permanecen dentro de sus proyectos de
Godot. Los iniciadores compartidos, los runtimes versionados, los registros de
operaciones, los registros de texto y CEF para Linux permanecen en los datos de
aplicación de Fennara:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## Resumen de comandos

| Comando | Finalidad |
| --- | --- |
| `fennara install` | Instalar o adoptar el addon de un proyecto y sus componentes locales correspondientes |
| `fennara update` | Actualizar un proyecto y sus componentes locales |
| `fennara doctor` | Inspeccionar o reparar la instalación local |
| `fennara diagnostics` | Mostrar un informe de operación saneado |
| `fennara mcp-setup` | Conectar una aplicación MCP externa |
| `fennara prepare-export` | Eliminar el autoload de Fennara antes de una exportación de CI sin el addon |
| `fennara recover` | Restaurar una actualización nativa interrumpida |
| `fennara self-update` | Actualizar únicamente la CLI instalada |

Ejecuta `fennara --help` para ver el resumen de comandos instalado. Utiliza
`fennara mcp-setup --help` para ver los destinos compatibles de aplicaciones MCP.

<a id="install-a-project"></a>
## Instalar un proyecto

Ejecuta el comando dentro de una carpeta que contenga `project.godot`:

```bash
fennara install
```

O indica el proyecto de forma explícita:

```bash
fennara install --project path/to/project
```

Sin `--version`, la CLI selecciona el manifiesto de la versión actual. Utiliza
una versión exacta cuando necesites reproducibilidad:

```bash
fennara install --project path/to/project --version <version>
```

La instalación ofrece dos caminos seguros:

- Si no existe un addon completo, la CLI descarga y verifica la versión
  seleccionada, instala `addons/fennara`, instala los componentes locales
  correspondientes y escribe las instrucciones del proyecto de Fennara.
- Si ya existe un addon completo, la CLI lee su `VERSION`, valida la biblioteca
  de la plataforma actual e instala los componentes administrados por la CLI de
  esa versión exacta. No modifica el addon del proyecto. Un `--version`
  explícito debe coincidir con el addon existente.

<a id="prepare-an-addon-free-ci-export"></a>
## Preparar una exportación de CI sin el addon

Si `addons/fennara/` está excluido de un checkout de CI, elimina el autoload
persistente del runtime de Fennara antes de que se inicie Godot:

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

El comando solo modifica la entrada `_fennara_game_capture` de
`project.godot`. Conserva los demás autoload y ajustes, y es seguro ejecutarlo
de nuevo. Este paso debe realizarse antes de iniciar Godot porque el arranque
del proyecto valida las rutas de los autoload antes de que puedan ejecutarse
los plugins del editor o de exportación. Como alternativa, CI puede instalar
el addon de Fennara antes de iniciar Godot.

<a id="update-a-project"></a>
## Actualizar un proyecto

Para una actualización normal desde la terminal, cierra Godot para ese proyecto y ejecuta:

```bash
fennara update --project path/to/project
```

Sin `--version`, la CLI lee la identidad del addon instalado. Los addons
estables resuelven Latest Release de GitHub, mientras que los addons de staging
solo resuelven su canal `pr-<number>`. El selector se fija de inmediato en una
versión exacta, incluso durante la sustitución automática de la CLI. A
continuación, la CLI verifica los recursos de la versión, actualiza el addon y
los componentes locales versionados, actualiza las instrucciones del proyecto y
comprueba el requisito de la vista web de la plataforma. Utiliza
`--version <version>` para seleccionar de forma explícita una versión exacta.

`--no-self-update` está destinado a automatización controlada o a continuar
después de que la CLI ya se haya sustituido. No lo utilices para eludir el
requisito de versión mínima de la CLI.

> [!IMPORTANT]
> Si vas a actualizar desde Fennara v0.3.8 o anterior, vuelve a instalar una vez
> la CLI con el comando de tu plataforma indicado en [Configuración](setup.md#instalar-desde-la-terminal-recomendado-en-macos)
> antes de ejecutar `fennara update`. Esas versiones de la CLI consultan una
> etiqueta de versión retirada y no pueden encontrar versiones actuales.
> Reinstalar la CLI no elimina el addon ni la configuración del proyecto.

> [!IMPORTANT]
> En macOS, vuelve a instalar una vez la CLI antes de actualizar desde Fennara
> v0.3.11. Esa CLI rechaza el paquete de framework existente antes de poder
> actualizarse a sí misma. La reinstalación solo sustituye la CLI y conserva el
> addon y la configuración del proyecto.

<a id="prepare-while-godot-is-open"></a>
### Preparar mientras Godot está abierto

El botón de actualización del editor utiliza el modo de staging:

```bash
fennara update --prepare --project path/to/project
```

La preparación descarga, verifica y deja listo de forma duradera el addon. No
cierra Godot, sustituye el addon activo, cambia el manifiesto del runtime activo
ni reinicia el daemon. El panel de Godot observa el recibo de la operación y
pide confirmación al usuario antes de iniciar en segundo plano el cierre, la
sustitución, la reapertura y la validación. El panel transmite la versión exacta
que ya encontró, por lo que un cambio del puntero no puede modificar una
actualización en curso.

Fennara admite una sola versión de runtime compartido activa a la vez. La
activación se bloquea si otro editor de Godot con Fennara permanece conectado al
daemon compartido. Cierra el otro editor y vuelve a intentarlo. La versión local
anterior y el puntero de runtime permanecen disponibles para recuperarse sin
acceso a la red.

`--prepare` es una primitiva de bajo nivel para la integración con Godot. Los
usuarios de terminal suelen utilizar `fennara update` con Godot ya cerrado.

<a id="recover-an-interrupted-update"></a>
## Recuperar una actualización interrumpida

Si el addon actualizado no puede cargarse lo suficiente para mostrar el panel de
recuperación, cierra Godot y ejecuta:

```bash
fennara recover --project path/to/project
```

La CLI solo restaura operaciones que se encuentren en un estado recuperable.
Restaura el addon anterior, los iniciadores compartidos y el manifiesto del
runtime activo, y después intenta volver a abrir el ejecutable de Godot
registrado. Selecciona una transacción concreta cuando el equipo de soporte te
proporcione su identificador de operación:

```bash
fennara recover --project path/to/project --operation <operation-id>
```

Las operaciones completadas, únicamente preparadas o ya revertidas se rechazan.

<a id="inspect-health-and-failures"></a>
## Inspeccionar el estado y los fallos

`doctor` informa de la plataforma detectada, la disposición de los datos de
aplicación, la versión activa, los iniciadores, los runtimes, el estado del
daemon y el requisito de la vista web:

```bash
fennara doctor
```

Si informa de un daemon o runtime MCP en ejecución anterior a `current.json`,
reinicia Godot o la aplicación MCP afectada para que inicie el runtime seleccionado.

Utiliza `--repair` para volver a crear los directorios base de datos de aplicación
que falten. En Linux también limpia perfiles de procesos CEF obsoletos y repara
el marcador del runtime actual si ya hay instalado un runtime administrado completo:

```bash
fennara doctor --repair
```

Las operaciones de instalación, actualización, recuperación y actualización de
la propia CLI escriben estado y eventos duraderos. Muestra el informe saneado más
reciente con:

```bash
fennara diagnostics
```

Para una operación anterior o una salida procesable por máquinas:

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

Los informes incluyen códigos de error estables, fases, versiones de los
componentes, nombres de recursos seleccionados y resultados de verificación de
hashes. Ocultan las rutas del proyecto, del directorio personal y de los datos
de aplicación de Fennara, así como credenciales, tokens bearer y consultas de
URL. No incluyen mensajes del chat, claves de proveedores ni contenido de los
archivos del proyecto.

<a id="configure-an-external-mcp-app"></a>
## Configurar una aplicación MCP externa

El panel de chat de Godot expone estos comandos en **Chat Settings > MCP Apps**.
Su botón Set Up pide al daemon local que invoque la CLI instalada, por lo que
los flujos del panel y la terminal utilizan la misma implementación de
configuración y copias de seguridad.

Ejecuta `fennara mcp-setup --help` para elegir un destino compatible. Reinicia la
aplicación MCP después de cambiar su configuración. Este comando conecta una
aplicación externa al servidor MCP de Fennara. No selecciona el proveedor de
modelos utilizado por el panel de chat integrado de Godot. [Configuración de
MCP](mcp-setup.md) contiene la lista de destinos, las ubicaciones de
configuración y los ejemplos de configuración manual.

<a id="update-only-the-cli"></a>
## Actualizar únicamente la CLI

Las actualizaciones normales de proyectos gestionan automáticamente la
actualización de la CLI. Para actualizar solo la CLI instalada:

```bash
fennara self-update
fennara self-update --version <version>
```

Sin `--version`, la actualización conserva el canal de la instalación activa:
estable utiliza Latest Release de GitHub y staging utiliza únicamente su canal
de PR registrado.

Staging nunca cambia a estable de forma automática. Para abandonar staging
deliberadamente, cierra Godot y ejecuta
`fennara update --version <stable-version> --project <path>`. Esa versión
estable exacta se valida antes de cambiar la versión compartida activa.

Utiliza este comando cuando el equipo de soporte lo solicite o cuando una
actualización del proyecto indique que la CLI instalada es demasiado antigua
para continuar de forma segura.

<a id="automation-guidance"></a>
## Recomendaciones para automatización

- Indica `--project` en lugar de depender del directorio actual.
- Fija `--version` cuando una compilación deba ser reproducible.
- Conserva el identificador de operación y la ruta de registro mostrados si se produce un fallo.
- Utiliza `fennara diagnostics --operation <id> --json` para generar informes estructurados.
- No edites manualmente `current.json`, los directorios de versiones, los recibos de actualización ni las carpetas de addons preparados.
- No ejecutes una actualización normal que sustituya el addon mientras el proyecto esté abierto en Godot. Utiliza el flujo de actualización del editor o cierra Godot primero.
