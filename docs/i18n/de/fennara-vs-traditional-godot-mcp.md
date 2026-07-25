<!-- fennara-i18n: locale=de source=docs/fennara-vs-traditional-godot-mcp.md sha256=e45a741b1db7c20e40b4a311c198af216172dfa024ca9c123db4f9336c9a6e7f -->
<a id="fennara-vs-traditional-godot-mcp"></a>
# Fennara im Vergleich zu traditionellem Godot MCP

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../fennara-vs-traditional-godot-mcp.md) · [简体中文](../zh-CN/fennara-vs-traditional-godot-mcp.md) · [Español](../es/fennara-vs-traditional-godot-mcp.md) · [Português do Brasil](../pt-BR/fennara-vs-traditional-godot-mcp.md) · [日本語](../ja/fennara-vs-traditional-godot-mcp.md) · [한국어](../ko/fennara-vs-traditional-godot-mcp.md) · [Русский](../ru/fennara-vs-traditional-godot-mcp.md) · [Français](../fr/fennara-vs-traditional-godot-mcp.md) · **Deutsch** · [Türkçe](../tr/fennara-vs-traditional-godot-mcp.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../fennara-vs-traditional-godot-mcp.md)
<!-- fennara-doc-nav:end -->

| Traditionelle Befehlsbrücke | Fennara-Feedbackschleife |
| --- | --- |
| Stellt Editoraktionen bereit | Stellt Godot-spezifische Inspektionen, Aktionen und Prüfungen bereit |
| Ein erfolgreicher Befehl kann den Ablauf beenden | Diagnosen, Validierung, Laufzeitprotokolle und Screenshots bestimmen den nächsten Schritt |
| Am besten für direkte, bekannte Änderungen | Am besten, wenn ein Agent untersuchen, ändern, überprüfen und wiederherstellen muss |

Die meisten Godot-MCP-Server stellen KI-Clients Editorbefehle bereit.

Beispiele:

- Knoten erstellen
- Eigenschaft festlegen
- Szene öffnen
- Szene speichern
- Protokolle lesen
- Screenshot aufnehmen
- Projekt ausführen
- Signal verbinden
- Input Map bearbeiten
- Materialien verwalten
- Tests ausführen

Das ist nützlich. Es macht Godot zu einer API-Oberfläche.

Bei echter KI-Spieleentwicklung besteht der schwierige Teil jedoch nicht darin, ob eine KI `set_property` aufrufen kann.

Der schwierige Teil besteht darin, ob die KI erkennen kann, dass das Projekt defekt ist.

<a id="traditional-mcp-pattern"></a>
## Traditionelles MCP-Muster

```text
KI ruft einen Editorbefehl auf.
Editor gibt ein Ergebnis zurück.
KI errät den nächsten Schritt.
```

Das funktioniert gut bei kleinen, direkten Änderungen.

Beispiel:

```text
Rename Camera3D to MainCamera.
```

Bei größeren Projektaufgaben, in denen der Agent die Architektur untersuchen, Skripte, Ressourcen und Szenen bearbeiten, Fehler erkennen und sich davon erholen muss, ist es jedoch schwächer.

<a id="fennara-pattern"></a>
## Fennara-Muster

```text
KI ändert das Projekt.
Godot-Feedback kommt zurück.
KI korrigiert und führt erneut aus, bis es funktioniert.
```

Fennara konzentriert sich auf Feedback:

- GDScript-Diagnosen
- Szenenvalidierung
- Laufzeitfehler
- Untersuchung des Szenenbaums
- Knoteneigenschaften
- Untersuchung von Klassen und APIs
- Screenshots
- generierte Projektanweisungen
- Arbeitsabläufe mit Korrektur und erneuter Ausführung

<a id="the-difference"></a>
## Der Unterschied

Traditionelles Godot MCP fragt:

```text
Welche Editorbefehle sollen wir bereitstellen?
```

Fennara fragt:

```text
Welches Feedback benötigt das Modell, um erfolgreich innerhalb von Godot zu entwickeln?
```

Befehle sind eine Grundvoraussetzung.

Feedback ist der Wettbewerbsvorteil.
