<!-- fennara-i18n: locale=de source=local/README.md sha256=b63443a8ff7310d53044b20261e7f206a949bad07299656235ba39fe69f92d9d -->
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
- Privilegierter gebundener Status: löst den kanonischen Projektstamm eines
  MCP-Prozesses gegen die verbundenen Godot-Editor-Sitzungen auf.
- `POST /tools/call`: leitet einen Werkzeugaufruf an das verbundene Godot-Plugin weiter und wartet auf ein Werkzeugergebnis.
- `WS /godot/ws`: lokale Brücke zum Godot-Plugin. Das Plugin sendet nach der Verbindung eine `hello`-Nachricht.

Ein Daemon wird von allen Fennara-fähigen Editoren und externen MCP-Prozessen des
aktuellen Benutzers gemeinsam verwendet. Gebundene externe Anfragen werden anhand
des kanonischen Projektstamms weitergeleitet. Interne Anfragen des integrierten
Chats bleiben an ihre Godot-Editor-Sitzung gebunden, und ungebundene Legacy-MCP-
Anfragen verwenden das im Dock ausgewählte Kompatibilitätsziel.

Der Daemon besitzt außerdem einen rechnerweiten Laufzeit-Slot. Eigentümerschaft und
erneuerbarer Lease-Zustand einer Laufzeitsitzung sind einem Projektstamm zugeordnet,
sodass ein Editor erneut verbunden werden kann, ohne die Kontrolle zu übertragen.

Entwicklungsbinärdatei:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP-Server

`crates/fennara-mcp` ist der lokale MCP-Server. Er kommuniziert per JSON-RPC über stdio, damit MCP-Clients ihn als lokalen Prozess starten können.

Jeder MCP-Prozess fixiert beim Start eine optionale Projektbindung. Die Auswahl
erfolgt zuerst über `--project-path`, danach über `FENNARA_PROJECT_PATH` und
schließlich über den nächsten Vorfahren des Startverzeichnisses, der
`project.godot` enthält. Wenn kein Projekt gefunden wird, wechselt der Prozess
automatisch in den ungebundenen Legacy-Kompatibilitätsmodus; ein ungültiger
ausdrücklicher Pfad verhindert den Start. Verwende für projektübergreifende
Isolation je einen MCP-Prozess und eine Verbindung pro Projekt.

`crates/fennara-project-identity` wird gemeinsam von MCP-Laufzeit und Daemon
verwendet. Es ist für Ermittlung, Validierung und Kanonisierung von
Projektstämmen, verlustfreie Protokollkonvertierung und aktive
Dateisystemgleichheit zuständig.

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

- `fennara_status`: überprüft, ob der MCP-Server installiert und erreichbar ist,
  und meldet bei laufendem Daemon anschließend Routingmodus, Bindungsquelle und
  -stamm, Zustand des ausgewählten Editors und Bereitschaft der Godot-Bridge.
- Godot-Projektwerkzeuge wie `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` und `screenshot_scene` werden an
  den Daemon weitergeleitet, der sie an das verbundene Godot-Plugin weiterleitet.

Später installierter Benutzerpfad unter Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
