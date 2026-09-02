<!-- fennara-i18n: locale=de source=docs/mcp-setup.md sha256=86c9fe3fc7a69c2ade417dd01a0ccabb05ddaa91cf417fa8559c28d4b01811bd -->
<a id="mcp-setup"></a>
# MCP-Einrichtung

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · **Deutsch** · [Türkçe](../tr/mcp-setup.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

Verbinde eine externe KI-App mit den Godot-Werkzeugen von Fennara. Die App verwendet weiterhin ihr eigenes Modellkonto, Abonnement oder ihre eigene API-Einrichtung.

> [!NOTE]
> Dies konfiguriert nicht den integrierten Fennara-Chat. Lies [MCP-Apps und integrierter Chat](chat-vs-mcp.md), wenn du nicht sicher bist, welchen Weg du benötigst.

<a id="quick-setup"></a>
## Schnelle Einrichtung

1. Schließe **Set Up Fennara** im Godot-Dock ab.
2. Öffne **Chat Settings > MCP Apps**.
3. Suche deine App und drücke **Set Up**.
4. Starte die App neu.

Fennara erstellt eine Sicherung, bevor es die MCP-Konfiguration einer App ändert. Die zusammengefasste Option **Claude** konfiguriert Claude Code und Claude Desktop. **Gemini & Antigravity** konfiguriert beide gemeinsam verwendeten Ziele.

<a id="terminal-alternative"></a>
### Alternative im Terminal

Führe zuerst `fennara install` im Godot-Projekt aus und wähle anschließend ein Ziel:

| App | Befehl |
| --- | --- |
| Claude Code und Claude Desktop | `fennara mcp-setup --claude` |
| Nur Claude Code | `fennara mcp-setup --claude-code` |
| Nur Claude Desktop | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini und Antigravity | `fennara mcp-setup --gemini` or `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

Führe `fennara mcp-setup --help` aus, um die von deiner installierten CLI unterstützte Zielliste anzuzeigen.

Die Einrichtung schreibt einen globalen, projektneutralen Launcher-Eintrag. Wenn
`fennara mcp-setup` innerhalb eines Projekts ausgeführt wird, bindet dies nicht alle
künftigen Verbindungen an dieses Projekt.

<a id="bind-a-connection-to-one-project"></a>
## Eine Verbindung an ein Projekt binden

Führe für mehrere Repositorys oder Worktrees auf demselben Rechner je einen
MCP-Prozess und eine Verbindung pro Projekt aus. Konfiguriere diesen Prozess in den
Projekt- oder Workspace-Einstellungen des MCP-Hosts entweder mit:

```text
--project-path /absolute/path/to/godot-project
```

oder:

```text
FENNARA_PROJECT_PATH=/absolute/path/to/godot-project
```

Die Laufzeit wählt ihre Projektbindung einmal beim Start in dieser Reihenfolge aus:

1. `--project-path`
2. `FENNARA_PROJECT_PATH`
3. der nächste Vorfahr des Startverzeichnisses, der `project.godot` enthält
4. ungebundener Legacy-Kompatibilitätsmodus, wenn die Ermittlung kein Projekt findet

Ein ungültiger ausdrücklicher Pfad verhindert den Start des MCP-Servers. Er fällt
niemals auf das Dock-Ziel oder einen anderen Editor zurück. Eine gültige Bindung
bleibt aktiv, wenn ihr Editor vorübergehend fehlt, und nimmt den Betrieb wieder auf,
sobald dieser Projektstamm erneut verbunden wird. Es gibt keine modellseitige
Projektüberschreibung pro Werkzeugaufruf.

Unter [Mehrere Agenten und Worktrees](multi-agent-worktrees.md) findest du
Konfigurationsbeispiele, Grenzen der Host-Unterstützung, Statusprüfung, Verhalten
bei doppelten Editoren und serialisierte Spieltests.

<a id="manual-setup"></a>
## Manuelle Einrichtung

Verwende die manuelle Einrichtung nur, wenn deine App nicht aufgeführt ist, der Einrichtungsbefehl die Konfigurationsdatei der App nicht finden kann oder du die MCP-Konfiguration bewusst von Hand bearbeiten möchtest.

Erstelle vor dem Bearbeiten eine Sicherung der Konfigurationsdatei. Füge dann einen lokalen stdio-MCP-Server mit dem Namen `fennara` hinzu, der auf den stabilen Fennara-MCP-Starter verweist.

Standardpfade der Starter:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Verwende den tatsächlichen absoluten Pfad auf deinem Computer. Verweise MCP-Apps nicht auf `versions/<version>/fennara-mcp-runtime`; durch den stabilen Starter in `bin/` funktionieren App-Konfigurationen auch nach Fennara-Aktualisierungen weiter.

<a id="json-mcpservers"></a>
### JSON `mcpServers`

Viele MCP-Apps verwenden ein Objekt `mcpServers` auf der obersten Ebene:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

Einige Apps verwenden denselben Schlüssel `mcpServers`, benötigen jedoch nur `command`. Falls die vorhandene Konfiguration bereits andere Server enthält, behalte diese Einträge bei und füge ausschließlich den Server `fennara` hinzu.

Füge für einen projektlokalen Eintrag, der isoliert bleiben muss, dessen Bindung
zu `args` hinzu:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/godot-project"],
      "env": {}
    }
  }
}
```

Konfigurationen im Stil von Cline können außerdem ein längeres Werkzeug-Zeitlimit in Sekunden enthalten:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### JSON `servers` im Stil von VS Code

Einige Clients, darunter die MCP-Konfiguration von VS Code auf Benutzer- oder Projektebene, verwenden ein Objekt `servers` auf der obersten Ebene und erfordern `type: "stdio"`:

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### JSON `mcp` im Stil von OpenCode

Eine JSON-Konfiguration im Stil von OpenCode verwendet ein Objekt `mcp` auf der obersten Ebene. Das Zeitlimit wird in Millisekunden angegeben:

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### TOML im Stil von Codex

Codex verwendet TOML:

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Füge kein JSON in eine TOML-Datei und kein TOML in eine JSON-Datei ein. Verwende das Format, das die App bereits nutzt.

Füge das Argument hinzu, um einen Codex-Eintrag zu binden, ohne dessen stabilen
Launcher zu ändern:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/godot-project"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

<a id="common-config-locations"></a>
## Übliche Speicherorte der Konfiguration

Dies sind übliche Speicherorte, die Fennaras Einrichtungshelfer und aktuelle MCP-Clients verwenden. Apps können ihre Konfigurationspfade ändern, und einige unterstützen sowohl globale als auch projektbezogene Konfigurationen. Wenn eine App einen Befehl wie **Open MCP Config** besitzt, verwende ihn, statt den Pfad zu erraten.

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

VS-Code-Workspaces mit einem einzelnen Ordner können das Projekt über das
Startverzeichnis des MCP-Kindprozesses bereitstellen. Claude Code, Gemini CLI,
Antigravity, Cline, Cursor, OpenCode, Kiro und Codex können Projekt- oder
Workspace-Konfiguration verwenden. Nutze eine ausdrückliche Bindung oder ein
dokumentiertes Projekt-Startverzeichnis, wenn die Isolation gewährleistet sein
muss.

Claude Desktop und ältere Windsurf-/Cascade-Versionen verwenden für diesen
Arbeitsablauf eine globale Konfiguration. Ihre Standardeinrichtung bleibt
im ungebundenen Legacy-Modus. Fortgeschrittene Benutzer können getrennt benannte Einträge mit
unterschiedlichen ausdrücklichen Projektpfaden erstellen, diese Apps bieten jedoch
keine automatische projektlokale Isolation.

<a id="timeout-guidance"></a>
## Hinweise zu Zeitlimits

Einige Fennara-Werkzeuge können länger als ein kleines standardmäßiges MCP-Zeitlimit benötigen, da sie Godot anweisen können, Szenen zu überprüfen, den Laufzeitzustand zu untersuchen, Screenshots aufzunehmen oder Diagnosen auszuführen.

Verwende ein längeres Zeitlimit pro Werkzeug, wenn der Client dies unterstützt:

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

Wenn ein Client keine Zeitlimits pro Server unterstützt, verwende die dokumentierte globale Einstellung dieses Clients für das MCP-Zeitlimit.

<a id="verify-the-connection"></a>
## Die Verbindung überprüfen

Öffne das Godot-Projekt und bitte dann deine MCP-App:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Bestätige für isolierte Arbeit, dass der Status den Routingmodus `bound`, die
erwartete Bindungsquelle und den kanonischen Projektstamm, den Zustand des
gebundenen Editors `connected` und die Dateisystembereitschaft dieses Editors
meldet.

Wenn der Status `legacy_unbound` meldet, wurde für die Verbindung kein
automatischer Projektstamm gefunden. Sie verwendet den Kompatibilitätsweg über
das **MCP target** im Dock und warnt, dass dieser Modus für isolierte gleichzeitige
Arbeit unsicher ist.

<a id="troubleshooting"></a>
## Fehlerbehebung

Wenn Fennara nicht in der MCP-App erscheint:

- Prüfe, ob der Starterpfad absolut ist und existiert.
- Prüfe, ob die Konfigurationssyntax gültiges JSON, JSON5 oder TOML entsprechend den Anforderungen der App ist.
- Prüfe, ob der Server `fennara` heißt.
- Prüfe, ob die App die von dir bearbeitete Konfigurationsdatei liest.
- Beende die MCP-App vollständig und öffne sie erneut.
- Prüfe, ob im Godot-Projekt das Fennara-Addon installiert ist.
- Prüfe bei einer gebundenen Verbindung, ob ihr ausdrücklicher Pfad oder
  Startverzeichnis dem vorgesehenen Godot-Projektstamm entspricht.
- Wenn der Status `bound_project_not_connected` meldet, öffne dieses Projekt in
  Godot und warte, bis sich das Addon verbindet.
- Wenn der Status `ambiguous_project_binding` meldet, schließe den doppelten
  Editor oder öffne ihn aus einem anderen Worktree.
- Prüfe bei einer ungebundenen Legacy-Verbindung, ob das vorgesehene Projekt als
  MCP-Ziel im Dock ausgewählt ist.

<a id="unsupported-mcp-apps"></a>
## Nicht unterstützte MCP-Apps

Wenn deine MCP-App nicht aufgeführt ist, ermittle zuerst anhand der offiziellen Dokumentation dieser App den Speicherort und das Format ihrer MCP-Konfiguration. Bitte anschließend ein LLM um die kleinste sichere Änderung:

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

Überprüfe das Ergebnis vor dem Speichern und starte anschließend die MCP-App neu.
