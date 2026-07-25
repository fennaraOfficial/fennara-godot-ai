<!-- fennara-i18n: locale=de source=docs/chat-vs-mcp.md sha256=03cb522aed8f8e305feaca0c2ed51f7ba29b2657a721df4196b15bc6ccf12c9c -->
<a id="mcp-apps-or-built-in-chat"></a>
# MCP-Apps oder integrierter Chat?

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · [Español](../es/chat-vs-mcp.md) · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · [日本語](../ja/chat-vs-mcp.md) · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · **Deutsch** · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara unterstützt beides. Wähle, wo die Unterhaltung stattfinden soll.

| | Externe MCP-App | Integrierter Fennara-Chat |
| --- | --- | --- |
| Wo du chattest | Codex, Claude, Cursor, Gemini oder eine andere MCP-App | Das Fennara-Dock oder dein Systembrowser |
| Modellkonto | Das Konto oder Abonnement der externen App | Ein in den Fennara Chat Settings verbundener Anbieter |
| Was Fennara ergänzt | Godot-spezifische MCP-Werkzeuge | Chat-Benutzeroberfläche, dieselben zentralen Godot-Werkzeuge sowie Datei- und Shell-Werkzeuge nur für den Chat |
| Einrichtung | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> Du kannst beide Wege verwenden. Ihre Modelleinstellungen bleiben getrennt.

<a id="external-mcp-apps"></a>
## Externe MCP-Apps

Wenn du eine MCP-App verbindest, kann diese App den lokalen Fennara-MCP-Server starten und
Godot-Werkzeuge aufrufen. Das Abonnement oder die Anmeldung der App wird nicht mit dem
integrierten Chat geteilt.

Richte eine App unter **Chat Settings > MCP Apps** ein oder verwende die CLI:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Es ist kein Anbieterschlüssel für den Fennara-Chat erforderlich. Starte die externe App nach der Einrichtung neu.
Unter [MCP-Einrichtung](mcp-setup.md) findest du alle Ziele und die manuelle Konfiguration.

<a id="built-in-chat"></a>
## Integrierter Chat

Für den integrierten Chat muss ein Anbieter in den Fennara Chat Settings verbunden sein. Verwende deinen
eigenen Schlüssel für einen Cloud-Anbieter oder verbinde einen lokalen Ollama- oder LM-Studio-Server.

Derselbe Chat kann im Godot-Dock oder in deinem Systembrowser angezeigt werden. Diese
Anzeigeauswahl ändert weder Anbieter, Modell und Verlauf noch das Projekt.

Um Code anzuhängen, markiere ihn im Skripteditor von Godot, öffne das Kontextmenü und
wähle **Add to Chat**. Unter [Anbieter für den integrierten Chat](providers.md) findest du die Einrichtung
von Anbieter und Modell.

<a id="project-routing"></a>
## Projektweiterleitung

Beide Wege verwenden den lokalen Fennara-Daemon für Godot-Feedback.

- Externe MCP-Aufrufe gehen an das Projekt, das im Dock über **MCP target**
  ausgewählt wurde.
- Der integrierte Chat bleibt an den Godot-Editor gebunden, der den Chat geöffnet hat.

Um eine externe MCP-Verbindung zu überprüfen, frage:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```
