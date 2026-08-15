<!-- fennara-i18n: locale=es source=local/README.md sha256=b63443a8ff7310d53044b20261e7f206a949bad07299656235ba39fe69f92d9d -->
<a id="fennara-local-tools"></a>
# Herramientas locales de Fennara

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · **Español** · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Esta carpeta contiene los componentes nativos locales de Fennara.

<a id="daemon"></a>
## Daemon

`crates/fennara-daemon` ejecuta el daemon local en:

```text
http://127.0.0.1:41287
```

Puntos de acceso:

- `GET /health`: estado del daemon.
- `GET /status`: estado y metadatos del plugin conectado.
- estado vinculado privilegiado: resuelve la Raíz del proyecto canónica de un
  proceso MCP entre las sesiones conectadas del editor de Godot.
- `POST /tools/call`: reenvía una llamada y espera el resultado.
- `WS /godot/ws`: puente local. El plugin envía `hello` al conectarse.

Todos los editores habilitados para Fennara y todos los procesos MCP externos
del usuario actual comparten un daemon. Las solicitudes externas vinculadas se
enrutan mediante la Raíz del proyecto canónica; las solicitudes internas del
chat integrado permanecen vinculadas a su Sesión del editor de Godot, y las
solicitudes MCP heredadas sin vinculación utilizan el destino de compatibilidad
seleccionado en el panel.

El daemon también controla una Ranura de ejecución para todo el equipo. La
propiedad de la Sesión de ejecución y el estado renovable de la concesión se
asocian a una Raíz del proyecto para que un editor pueda volver a conectarse sin
transferir el control.

Binario de desarrollo:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## Servidor MCP

`crates/fennara-mcp` es el servidor local. Habla JSON-RPC mediante stdio para que los clientes lo inicien como proceso local.

Cada proceso MCP fija una Vinculación de proyecto opcional al iniciarse. La
selección sigue este orden: `--project-path`, `FENNARA_PROJECT_PATH` y el
ancestro más cercano del directorio de inicio que contenga `project.godot`. Si
no encuentra ningún proyecto, entra automáticamente en el modo de compatibilidad
heredado sin vinculación; una ruta explícita no válida impide el inicio. Utiliza
un proceso y una conexión MCP por proyecto para el aislamiento entre proyectos.

`crates/fennara-project-identity` es compartido por el runtime MCP y el daemon.
Se ocupa de la detección, validación y canonización de la Raíz del proyecto, de
la conversión sin pérdidas para el protocolo y de la igualdad activa del sistema
de archivos.

`fennara-mcp` incorpora al compilar los esquemas elegidos de
`local/schemas/tools/` y reenvía las llamadas al daemon. No necesita un servicio
externo de esquemas. El chat integrado elige un conjunto relacionado pero distinto.

`fennara install` también escribe desde `local/templates/`:

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

Compilar:

```powershell
cd local
cargo build
```

En Windows, si la terminal aún no actualizó PATH:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Binario de desarrollo:

```text
local/target/debug/fennara-mcp.exe
```

Herramientas actuales:

- `fennara_status` verifica que el servidor MCP esté instalado y sea accesible y,
  cuando el daemon está en ejecución, indica el modo de enrutamiento, la
  fuente/raíz de vinculación, el estado del editor seleccionado y la
  disponibilidad del puente de Godot.
- Herramientas como `write_or_update_file`, `run_scene_edit_script`, `get_scene_tree`, `script_diagnostics` y `screenshot_scene` se reenvían al daemon y al plugin.

Ruta futura instalada en Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
