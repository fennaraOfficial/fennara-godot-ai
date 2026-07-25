<!-- fennara-i18n: locale=es source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
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
- `POST /tools/call`: reenvía una llamada y espera el resultado.
- `WS /godot/ws`: puente local. El plugin envía `hello` al conectarse.

Binario de desarrollo:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## Servidor MCP

`crates/fennara-mcp` es el servidor local. Habla JSON-RPC mediante stdio para que los clientes lo inicien como proceso local.

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

- `fennara_status` verifica la instalación y accesibilidad, y muestra el estado del daemon y del puente.
- Herramientas como `write_or_update_file`, `run_scene_edit_script`, `get_scene_tree`, `script_diagnostics` y `screenshot_scene` se reenvían al daemon y al plugin.

Ruta futura instalada en Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
