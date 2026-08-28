<!-- fennara-i18n: locale=de source=docs/multi-agent-worktrees.md sha256=7b266e260017a37b18e3d8e36a6bed75e76c3bcc4ead88c49bec146302495014 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# Mehrere Agenten und Godot-Worktrees

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · [Español](../es/multi-agent-worktrees.md) · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · [日本語](../ja/multi-agent-worktrees.md) · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · **Deutsch** · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

Führe mehrere Coding-Agenten auf demselben Rechner gegen getrennte Repositorys
oder Worktrees aus, ohne dass die Zielauswahl eines Agenten einen anderen
umleitet. Jedes Projekt erhält einen eigenen Fennara-MCP-Prozess und eine eigene
Verbindung; alle Projekte teilen sich denselben benutzerbezogenen Daemon.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

Bearbeitung, Inspektion, Validierung und Screenshot-Aufrufe können gleichzeitig
ausgeführt werden. Vom Daemon verwaltete Spielausführungen werden über einen
rechnerweiten Laufzeit-Slot serialisiert.

<a id="one-mcp-connection-per-project"></a>
## Eine MCP-Verbindung pro Projekt

Ein MCP-Prozess wählt beim Start einen stabilen Projektstamm aus. Diese
MCP-Projektbindung ist die kanonische Dateisystemidentität des Verzeichnisses,
das `project.godot` enthält; sie ist weder ein Projektname noch eine
Godot-Prozess-ID.

Verwende für jedes Repository oder jeden Worktree einen eigenen MCP-Prozess und
eine eigene Verbindung. Eine Verbindung darf nur dann mehrere Agenten bedienen,
wenn alle absichtlich am selben Projekt arbeiten. Fennara-Werkzeuge bieten keine
Projektauswahl pro Aufruf, sodass das Modell einen Prozess nicht versehentlich
auf ein anderes Projekt umstellen kann.

Jedes Projekt benötigt außerdem einen verbundenen Godot-Editor mit aktiviertem
Fennara. Wenn ein Editor geschlossen und mit einer neuen Prozess-ID erneut
verbunden wird, nimmt der vorhandene MCP-Prozess seine Weiterleitung wieder auf,
sobald derselbe Projektstamm verbunden ist.

<a id="how-a-process-chooses-its-project"></a>
## So wählt ein Prozess sein Projekt aus

Die MCP-Laufzeit erfasst beim Start ihr Arbeitsverzeichnis und wählt ihre Bindung
einmalig in dieser Reihenfolge aus:

1. `--project-path <path>` oder `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. Der nächste Vorfahr des Startverzeichnisses, der `project.godot` enthält.
4. Ungebundener Legacy-Kompatibilitätsmodus, wenn die automatische Ermittlung kein
   Godot-Projekt findet.

Pfade aus Befehlszeile und Umgebung sind ausdrückliche Zusicherungen. Ein leerer,
nicht zugänglicher, fehlender, nicht als Verzeichnis vorliegender, nicht zu einem
Godot-Projekt gehörender oder nicht unterstützter Pfad verhindert den Start des
MCP-Servers; er fällt niemals auf ein anderes Projekt zurück. Relative Pfade werden
ausgehend vom erfassten Startverzeichnis aufgelöst. Verwende vorzugsweise einen
absoluten Pfad, wenn das Startverzeichnis des MCP-Hosts unklar ist.

Fennara verwendet keine hostspezifischen Workspace-Variablen implizit. Ein
MCP-Host kann seinen eigenen Workspace-Wert `--project-path` oder
`FENNARA_PROJECT_PATH` zuordnen.

<a id="configure-a-project-bound-connection"></a>
## Eine projektgebundene Verbindung konfigurieren

`fennara mcp-setup` bleibt global und projektneutral. Wenn der Befehl in einem
Projekt ausgeführt wird, bindet er nicht alle künftigen MCP-Prozesse an dieses
Projekt. Behalte den stabilen Launcher-Pfad bei und füge dann über die Projekt-
oder Workspace-Konfiguration des MCP-Hosts eine Bindung hinzu.

Für eine JSON-Konfiguration:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

Alternativ über die Umgebung:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Für TOML im Codex-Stil:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Konfiguriere den nächsten Agenten in seiner eigenen Projekt- oder
Workspace-Konfiguration mit `/absolute/path/to/worktree-b`. Wenn ein Host
zuverlässig aus jedem Projektverzeichnis einen getrennten MCP-Prozess startet,
kann die Vorfahrenermittlung dieselbe Bindung ohne ausdrücklichen Pfad erzeugen.

<a id="mcp-host-boundaries"></a>
## Grenzen der MCP-Hosts

Projektlokale Konfiguration und Verhalten des Startverzeichnisses unterscheiden
sich je nach Host:

- VS-Code-Workspaces mit einem einzelnen Ordner können sich auf das dokumentierte
  Arbeitsverzeichnis des Kindprozesses verlassen; eine ausdrückliche Projektbindung
  bleibt jedoch die eindeutigste Konfiguration.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro und Codex
  können Projekt- oder Workspace-Konfiguration verwenden. Nutze eine
  ausdrückliche Bindung oder ein dokumentiertes Projekt-Startverzeichnis, wenn die
  Isolation gewährleistet sein muss.
- Die Konfiguration von Claude Desktop und älteren Windsurf-/Cascade-Versionen ist
  global. Ihr standardmäßiger Fennara-Eintrag bleibt im ungebundenen Legacy-Modus und kann keine
  automatische projektlokale Isolation bereitstellen. Fortgeschrittene Benutzer
  können getrennt benannte globale Einträge mit unterschiedlichen ausdrücklichen
  Pfaden erstellen, müssen jedoch den richtigen Eintrag auswählen.

<a id="worktree-isolated-subagents"></a>
### Worktree-isolierte Subagenten

Manche Hosts starten einen Kind-Agenten in einem getrennten Git-Worktree und
vererben dabei die MCP-Verbindungen des Elternprozesses. Claude Code
`isolation: worktree` und die Worktree-Isolation von Grok Build
`spawn_subagent` tun das.

Native Datei- und Shell-Werkzeuge arbeiten dann im Kind-Worktree. Fennara
bleibt an das Elternprojekt gebunden, sodass das Kind einen Baum bearbeiten
und einen anderen untersuchen oder verändern kann.

Gib diesem Subagenten eine eigene Fennara-MCP-Verbindung, die an den
Kind-Worktree gebunden ist, oder behalte ihn ohne Worktree-Isolation im
Elternprojekt. Codex- und OpenCode-Standard-Subagenten sind nicht so
dokumentiert, dass sie Fennara auf diese Weise erben.

Die automatische Erzeugung projektlokaler Konfiguration und neue Unterstützung für
Windsurf/Devin Local gehören nicht zu diesem Arbeitsablauf.

<a id="start-and-verify-the-editors"></a>
## Editoren starten und überprüfen

Jeder Worktree benötigt einen eigenen Godot-Editor mit aktiviertem Fennara.
Headless-Editoren können getrennte Godot-LSP-Ports verwenden und trotzdem
Fennaras Daemon gemeinsam nutzen:

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

Die LSP-Ports gehören zu Godot. Fennara verwendet weiterhin einen gemeinsamen
Daemon an seiner normalen Loopback-Adresse.

Führe vor gleichzeitiger Arbeit bei jedem Agenten `fennara_status` aus. Bestätige,
dass Folgendes gemeldet wird:

- Routingmodus `bound`
- die erwartete Bindungsquelle und der kanonische Projektstamm
- Zustand des gebundenen Editors `connected`
- Dateisystembereitschaft dieses Editors

Wenn die automatische Ermittlung kein Projekt gefunden hat, meldet der Status
`legacy_unbound` und eine Warnung zur gleichzeitigen Nutzung. In diesem
Kompatibilitätsmodus wird zuerst das im Dock ausgewählte MCP-Ziel und danach der
einzige verbundene Editor verwendet. Nutze für isolierte gleichzeitige Arbeit
keine ungebundene Verbindung.

<a id="missing-and-duplicate-editors"></a>
## Fehlende und doppelte Editoren

Eine gültige Projektbindung bleibt aktiv, wenn ihr Editor fehlt. Werkzeugaufrufe
geben den wiederholbaren Fehler `bound_project_not_connected` zurück, bis dieser
Projektstamm erneut verbunden wird; sie fallen niemals auf das Dock-Ziel zurück.

Zwei Editoren, die zum selben Projektstamm aufgelöst werden, erzeugen
`ambiguous_project_binding`. Schließe den doppelten Editor oder gib ihm einen
eigenen Worktree. Fennara wählt weder nach Prozess-ID, Verbindungsreihenfolge,
Projektname noch Dock-Ziel aus.

Symlink-Aliasse desselben Projekts werden zur selben aktiven Dateisystemidentität
aufgelöst. Wenn ein Symlink nach dem MCP-Start auf ein anderes Ziel gesetzt wird,
ändert dies die Bindung nicht; starte den MCP-Prozess neu, um ihn erneut zu binden.

<a id="serialized-runtime-sessions"></a>
## Serialisierte Laufzeitsitzungen

Alle Projekte teilen sich für vom Daemon verwaltete Spielausführungen einen
rechnerweiten Laufzeit-Slot. Wenn ein anderes Projekt gerade eine Sitzung startet
oder ausführt, gibt `runtime_session.start` das erfolgreiche Domänenergebnis `busy`
mit `availability: "busy"`, `slot_acquired: false` und einem vorgeschlagenen
`retry_after_ms` zurück. Es legt weder Eigentümer, Sitzungs-ID, Prozess-ID, Szene, Protokolle,
Warteschlangenposition noch erwartete Dauer offen.

Es gibt keine FIFO-Warteschlange. Frage mit Jitter ungefähr zum vorgeschlagenen
Zeitpunkt erneut ab und behandle jeden Aufruf von `runtime_session.start` als den
endgültigen atomaren Anspruch. Ein freier Status ist nur ein Hinweis, weil ein
anderer Agent das Rennen nach der Vorprüfung gewinnen kann.

Nur der zugehörige Projektstamm darf seine Laufzeitsitzung inspizieren, erneuern,
mit Skripten steuern oder beenden. Statusabfragen des Eigentümers erneuern eine
Inaktivitätsfrist von 120 Sekunden. Eine begrenzte Laufzeitoperation des
Eigentümers setzt den Inaktivitätsablauf während ihrer Ausführung aus und erneuert
die Frist erst, nachdem sie ein abschließendes Skriptergebnis zurückgegeben hat;
bei Zeitüberschreitung, Einrichtungsfehler oder Abbruch wird sie nicht erneuert.
Agenten sollten den Eigentümerstatus während einer Ausführung etwa alle 30
Sekunden mit Jitter abfragen.

Die absolute Standarddauer der Laufzeit-Lease beträgt 900 Sekunden.
`max_run_seconds` akzeptiert eine positive Ganzzahl von bis zu 86.400 Sekunden;
eine voraussichtlich einstündige Regression kann beispielsweise 4.500 Sekunden
als Sicherheitspuffer anfordern. Die absolute Frist wird niemals angehalten.
Natürliches Prozessende, ausdrücklicher Stopp, Startfehler, Inaktivität oder
absoluter Ablauf beenden oder bereinigen das Spiel und geben den Laufzeit-Slot frei.

<a id="safe-multi-agent-checklist"></a>
## Checkliste für sichere Arbeit mit mehreren Agenten

1. Erstelle für jedes Projekt ein eigenes Repository oder einen eigenen Worktree.
2. Installiere Fennara und öffne für jeden Projektstamm einen Godot-Editor.
3. Konfiguriere je einen projektgebundenen MCP-Prozess pro Projekt.
4. Führe bei jedem Agenten `fennara_status` aus und überprüfe dessen kanonischen Stamm.
5. Lass Bearbeitung, Inspektion, begrenzte Szenenvalidierung und eigenständige
   Screenshots gleichzeitig fortfahren.
6. Frage normale `busy`-Ergebnisse für Spieltests ab und versuche es erneut; halte
   die siegreiche Sitzung während der Ausführung durch Eigentümerstatus aktiv.
