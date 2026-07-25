<!-- fennara-i18n: locale=de source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
<a id="fennara-local-tools"></a>
# Lokale Fennara-Werkzeuge

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · **Deutsch** · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Dieser Ordner enthält lokal-native Fennara-Komponenten.

<a id="daemon"></a>
## Daemon

`crates/fennara-daemon` führt den lokalen Fennara-Daemon unter folgender Adresse aus:

```text
http://127.0.0.1:41287
```

Endpunkte:

- `GET /health`: Zustand des Daemons.
- `GET /status`: Daemonstatus sowie Metadaten zum verbundenen Godot-Plugin.
- `POST /tools/call`: leitet einen Werkzeugaufruf an das verbundene Godot-Plugin weiter und wartet auf ein Werkzeugergebnis.
- `WS /godot/ws`: lokale Brücke zum Godot-Plugin. Das Plugin sendet nach der Verbindung eine `hello`-Nachricht.

Entwicklungsbinärdatei:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP-Server

`crates/fennara-mcp` ist der lokale MCP-Server. Er kommuniziert per JSON-RPC über stdio, damit MCP-Clients ihn als lokalen Prozess starten können.

`fennara-mcp` bettet seine ausgewählten MCP-seitigen Schemas aus `local/schemas/tools/`
zur Build-Zeit ein und leitet diese Werkzeugaufrufe an den lokalen Daemon weiter. Zur Laufzeit
benötigt er keinen externen Schemadienst. Der integrierte Chat wählt einen verwandten,
aber abweichenden Werkzeugsatz aus demselben Schemaverzeichnis.

`fennara install` schreibt außerdem generierte Projektanweisungen aus `local/templates/`
in das Godot-Projekt:

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

Build:

```powershell
cd local
cargo build
```

Unter Windows, falls ein Terminal den Rust-PATH noch nicht aktualisiert hat:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Entwicklungsbinärdatei:

```text
local/target/debug/fennara-mcp.exe
```

Aktuelle Werkzeuge:

- `fennara_status`: überprüft, ob der MCP-Server installiert und erreichbar ist, und meldet bei laufendem Daemon anschließend den Status von Daemon und Godot-Brücke.
- Godot-Projektwerkzeuge wie `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` und `screenshot_scene` werden an
  den Daemon weitergeleitet, der sie an das verbundene Godot-Plugin weiterleitet.

Später installierter Benutzerpfad unter Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
