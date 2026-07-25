<!-- fennara-i18n: locale=de source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# MCP-Einrichtung

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

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

Wenn mehr als ein Godot-Projekt geöffnet ist, wähle mit dem Steuerelement **MCP target** im Fennara-Dock aus, welches Projekt externe MCP-Werkzeugaufrufe empfangen soll.

<a id="troubleshooting"></a>
## Fehlerbehebung

Wenn Fennara nicht in der MCP-App erscheint:

- Prüfe, ob der Starterpfad absolut ist und existiert.
- Prüfe, ob die Konfigurationssyntax gültiges JSON, JSON5 oder TOML entsprechend den Anforderungen der App ist.
- Prüfe, ob der Server `fennara` heißt.
- Prüfe, ob die App die von dir bearbeitete Konfigurationsdatei liest.
- Beende die MCP-App vollständig und öffne sie erneut.
- Prüfe, ob im Godot-Projekt das Fennara-Addon installiert ist.
- Prüfe, ob das vorgesehene Godot-Projekt als MCP-Ziel ausgewählt ist.

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
