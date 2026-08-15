<!-- fennara-i18n: locale=es source=docs/setup.md sha256=d470da8cda3e69aacb89ee5f06f1d7831df5ab7f50c94a599bfab5bfe22157e3 -->
<a id="setup"></a>
# Configuración

<!-- fennara-doc-nav:start -->
[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · **Español** · [Português do Brasil](../pt-BR/setup.md) · [日本語](../ja/setup.md) · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · [Deutsch](../de/setup.md) · [Türkçe](../tr/setup.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../setup.md)
<!-- fennara-doc-nav:end -->

Instala Fennara, elige dónde quieres conversar y conecta tu proyecto de Godot.

> [!TIP]
> La mayoría de los usuarios solo necesitan añadir el addon, abrir el panel de
> Fennara y pulsar **Set Up Fennara**. En macOS, utiliza la instalación mediante
> CLI que aparece a continuación para evitar la notificación de seguridad que
> puede aparecer con un ZIP del addon descargado manualmente.

<a id="before-you-start"></a>
## Antes de empezar

| Requisito | Cuándo lo necesitas |
| --- | --- |
| Godot 4.5 o posterior | Siempre |
| Windows x86_64, Linux x86_64 o macOS arm64 | Siempre |
| Una aplicación de IA compatible con MCP | Solo para utilizar MCP externamente |
| Una clave de API en la nube, Ollama o LM Studio | Solo para el chat integrado |
| El SDK de .NET disponible como `dotnet` | Solo para diagnósticos de C# y comprobaciones previas de ejecución |

<a id="install-from-godot"></a>
## Instalar desde Godot

> [!IMPORTANT]
> En macOS, el addon de la versión publicada contiene una biblioteca nativa que
> actualmente no está notarizada por Apple. Descargar el ZIP del addon desde un
> navegador y extraerlo manualmente puede hacer que macOS indique que no puede
> verificar que `libfennara.macos.editor` esté libre de software malicioso.
> Utiliza [Instalar desde la terminal](#instalar-desde-la-terminal-recomendado-en-macos)
> para evitar esta notificación.

1. Descarga `fennara-addon-latest.zip` desde la
   [versión más reciente](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)
   y copia `addons/fennara/` dentro de tu proyecto.
2. Abre el proyecto y selecciona el panel de Fennara.
3. Pulsa **Set Up Fennara**.

Fennara instala los componentes locales correspondientes y conecta el proyecto
abierto. Si un daemon compartido más antiguo está inactivo, la configuración lo
detiene antes de activar la versión correspondiente. Para cambiar de versión no
puede haber ningún proyecto conectado. Normalmente, el proyecto que se está
configurando permanece desconectado mientras las versiones no coincidan. Si la
configuración indica que hay un proyecto conectado, cierra todos los demás
editores con Fennara y vuelve a intentarlo. Si permanece una conexión obsoleta
del proyecto actual, cierra y vuelve a abrir este editor antes de intentarlo de
nuevo.
Si la configuración falla, el panel ofrece **Retry**, **Copy Report** y
**Open Logs**. Los informes copiados están saneados y no incluyen claves de API,
contenido del chat ni archivos del proyecto.

> [!NOTE]
> El addon permanece en tu proyecto. La CLI, el daemon, el servidor MCP, los
> registros y el runtime compartido del navegador se encuentran en los datos de
> aplicación de Fennara, fuera del proyecto.

<a id="install-from-the-terminal-recommended-on-macos"></a>
## Instalar desde la terminal (recomendado en macOS)

La CLI instala el mismo addon y es el método de instalación recomendado en
macOS. Evita la ruta de cuarentena del navegador y Finder que provoca la
notificación de la biblioteca nativa descrita anteriormente.

Instala la CLI en Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

O en macOS y Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Después, ejecuta Fennara dentro del proyecto:

```bash
cd path/to/your-godot-project
fennara install
```

Si ya extrajiste manualmente el addon en macOS y aparece la notificación, cierra
Godot y elimina la carpeta `addons/fennara/` copiada manualmente antes de
ejecutar `fennara install`. Esto es importante porque la CLI conserva un addon
completo ya existente en lugar de sustituirlo.

Si el proyecto ya contiene un addon completo de Fennara, la CLI lo conserva e
instala los componentes locales correspondientes. De lo contrario, también
instala el addon de la versión actual. Consulta la [referencia de instalación de
la CLI](cli.md#instalar-un-proyecto) para conocer la fijación de versiones y la
automatización.

<a id="choose-how-you-use-fennara"></a>
## Elegir cómo utilizar Fennara

| Opción | Cuenta del modelo | Configuración |
| --- | --- | --- |
| Chat integrado | Un proveedor conectado en Fennara Chat Settings | [Conectar un proveedor](#conectar-el-chat-integrado) |
| Aplicación MCP externa | La cuenta o suscripción propia de la aplicación | [Conectar una aplicación MCP](#conectar-una-aplicación-mcp) |
| Ambas | Cada opción conserva su propia configuración de modelo | Completa ambas secciones |

<a id="connect-the-built-in-chat"></a>
### Conectar el chat integrado

1. Abre **Chat Settings > Chat**.
2. Selecciona **Open providers**.
3. Conecta un proveedor en la nube con tu propia clave o conecta un servidor
   local de Ollama o LM Studio.
4. Elige un modelo.

Consulta [Proveedores del chat integrado](providers.md) para conocer los
proveedores compatibles, sus claves, las URL de servidores locales y los
identificadores de modelo. Utiliza `/provider` y `/model` para realizar las
mismas acciones desde el cuadro de redacción.

El chat incrustado utiliza la vista web de la plataforma:

| Plataforma | Vista web |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | WKWebView/WebKit del sistema |
| Linux | Runtime CEF compartido administrado por Fennara |

`fennara install`, `fennara update` y `fennara doctor` comprueban estos
requisitos. Las herramientas MCP siguen funcionando aunque no se pueda iniciar
el chat incrustado opcional.

Para utilizar el navegador del sistema, activa **Open chat in my system browser
next time** en Chat Settings y reinicia Godot. Esto solo cambia dónde aparece el
chat integrado. Conserva el mismo proveedor, historial y conexión al proyecto.

Para adjuntar código al siguiente mensaje del chat integrado, selecciona código
en el editor de scripts de Godot, abre el menú contextual y elige **Add to Chat**.

<a id="connect-an-mcp-app"></a>
### Conectar una aplicación MCP

Abre **Chat Settings > MCP Apps**, busca tu aplicación y pulsa **Set Up**.
Reinicia la aplicación para que pueda cargar Fennara.

También puedes conectarla desde la terminal:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Si tu aplicación no aparece, consulta [Configuración de MCP](mcp-setup.md) para
ver todos los destinos compatibles y los formatos de configuración manual.

La entrada de configuración es global y neutral respecto al proyecto. Si varios
agentes utilizan repositorios o árboles de trabajo separados, configura después
una conexión MCP vinculada por cada Raíz del proyecto. Consulta
[Varios agentes y árboles de trabajo](multi-agent-worktrees.md).

Las aplicaciones MCP externas utilizan sus propias cuentas de modelo. El chat
integrado utiliza el proveedor seleccionado en Fennara Chat Settings. Consulta
[Aplicaciones MCP y chat integrado](chat-vs-mcp.md) para entender la diferencia.

<a id="verify-the-connection"></a>
## Verificar la conexión

Abre el proyecto de Godot y pregunta a tu aplicación MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Para trabajo aislado, confirma que indique el modo de enrutamiento `bound`, la
Raíz del proyecto canónica esperada, el estado del editor vinculado `connected`
y la disponibilidad del sistema de archivos de ese editor. Una conexión
vinculada sin un editor abierto correspondiente permanece activa e indica el
error reintentable `bound_project_not_connected` hasta que el editor vuelve a
conectarse.

Si el estado indica `legacy_unbound`, el MCP target del panel es la ruta de
compatibilidad. Selecciona el destino previsto para un uso con un solo cliente o
configura una Vinculación de proyecto explícita antes de trabajar
simultáneamente.

<a id="update-fennara"></a>
## Actualizar Fennara

Cuando el panel muestre **Update**, púlsalo y sigue las indicaciones. Fennara
descarga y verifica la actualización antes de pedirte que cierres Godot. Vuelve
a abrir el mismo proyecto tras la instalación y conserva la versión anterior
que funcionaba hasta que la actualización se haya validado.

Para actualizar desde la terminal, cierra Godot y ejecuta:

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Si vas a actualizar desde Fennara v0.3.8 o anterior, vuelve a instalar una vez
> la CLI con el comando correspondiente a tu plataforma antes de ejecutar
> `fennara update`. Esas versiones de la CLI consultan una etiqueta de versión
> retirada y no pueden encontrar las versiones actuales. Reinstalar la CLI hace
> que las actualizaciones futuras utilicen el punto de acceso Latest Release de
> GitHub, sin eliminar el addon ni la configuración del proyecto.

> [!IMPORTANT]
> En macOS, vuelve a instalar una vez la CLI antes de actualizar desde Fennara
> v0.3.11. Esa CLI rechaza el paquete de framework existente antes de llegar a
> actualizarse a sí misma. La reinstalación solo sustituye la CLI y conserva el
> addon y la configuración del proyecto.

Si la validación falla, utiliza **Restore Previous Version**, **Open Logs** o
**Copy Report** en el panel. Consulta la [referencia de actualización de la
CLI](cli.md#actualizar-un-proyecto) para conocer las versiones exactas, la
preparación y la recuperación tras una actualización interrumpida.

<a id="troubleshooting"></a>
## Resolución de problemas

<a id="an-install-or-update-failed"></a>
### Ha fallado una instalación o actualización

Copia el informe saneado desde el panel o muestra el informe más reciente en una
terminal:

```bash
fennara diagnostics
```

Consulta [Diagnósticos de la CLI](cli.md#inspeccionar-el-estado-y-los-fallos) para
conocer los identificadores de operación, la salida JSON, los campos registrados
y las garantías de ocultación de datos.

<a id="fennara-is-not-found"></a>
### No se encuentra `fennara`

Abre una terminal nueva y ejecuta:

```bash
fennara doctor
```

Si el comando sigue sin estar disponible, añade el directorio `bin` de Fennara
a PATH. La [página de instalación de la CLI](cli.md#instalar-la-cli) enumera las
rutas de cada plataforma.

<a id="windows-binaries-fail-before-starting"></a>
### Los binarios de Windows fallan antes de iniciarse

Si un binario de Fennara informa de que falta una DLL `VCRUNTIME` o `MSVCP`,
devuelve el código de salida `-1073741515` o `0xc0000135`, instala Microsoft
Visual C++ Redistributable 2015-2022 x64:

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

Solo es necesario en equipos Windows que no tengan esas DLL de runtime de Microsoft.

<a id="a-release-requires-a-newer-cli"></a>
### Una versión requiere una CLI más reciente

Si la actualización automática de la CLI no puede instalar la versión requerida,
vuelve a ejecutar el script de instalación de [Instalar la CLI](cli.md#instalar-la-cli)
y reintenta el comando.

<a id="the-addon-is-not-visible-in-godot"></a>
### El addon no aparece en Godot

Comprueba que exista este archivo y vuelve a abrir el proyecto:

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` muestra el proyecto equivocado

Lee primero el modo de enrutamiento indicado. Para `bound`, reinicia el proceso
MCP con el `--project-path`, `FENNARA_PROJECT_PATH` o directorio de inicio del
proyecto previstos. Para `legacy_unbound`, abre el proyecto deseado y
selecciónalo mediante el control MCP target del panel de Fennara.

Si una raíz vinculada figura como `not_connected`, abre ese proyecto y espera a
que se conecte su addon. Si figura como `ambiguous`, cierra el editor duplicado
o inícialo desde un árbol de trabajo distinto.

<a id="c-diagnostics-are-missing"></a>
### Faltan los diagnósticos de C#

Comprueba que el proyecto contenga un único `.csproj`, `.sln` o `.slnx` claro y ejecuta:

```bash
dotnet --version
```

Para conocer la disposición del runtime del navegador, la recuperación manual y
los detalles de implementación, consulta [Arquitectura](architecture.md),
[Instalación manual](manual-install.md) y las [Preguntas frecuentes](faq.md).
