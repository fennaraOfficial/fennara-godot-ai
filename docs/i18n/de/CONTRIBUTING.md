<!-- fennara-i18n: locale=de source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# Mitwirken

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · [Español](../es/CONTRIBUTING.md) · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · [日本語](../ja/CONTRIBUTING.md) · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · **Deutsch** · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Vielen Dank, dass du dabei hilfst, Fennara Godot AI zu verbessern.

<a id="good-contributions"></a>
## Gute Beiträge

- Korrekturen an der Dokumentation
- Reproduzierbare Fehlerbehebungen
- Korrekturen zur Plattformkompatibilität
- Verbesserungen an Build und Paketierung
- Kleine Verbesserungen an der Verständlichkeit der Einrichtung

<a id="design-discussion-required"></a>
## Designbesprechung erforderlich

Eröffne ein Issue oder eine Diskussion, bevor du mit Folgendem beginnst:

- neue MCP-Werkzeuge
- Änderungen an Werkzeugschemas
- Änderungen am Release-Workflow
- große Architekturänderungen
- Änderungen, die sich auf generierte Projektanweisungen auswirken

<a id="pull-requests"></a>
## Pull Requests

- Halte Pull Requests klein und fokussiert.
- Erkläre, was geändert wurde und warum.
- Erkläre, wie du die Änderung überprüft hast.
- Füge bei sichtbaren Änderungen an der Benutzeroberfläche oder am Rendering der Dokumentation Screenshots oder Aufzeichnungen hinzu.
- Nimm keine unabhängigen Formatierungsänderungen oder Aufräumarbeiten auf.
- Füge keine umfangreichen generierten Beschreibungen in Issues oder Pull Requests ein.

<a id="commit-and-pr-titles"></a>
## Commit- und PR-Titel

Verwende den Conventional-Commit-Stil:

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

Häufige Typen:

- `feat`: für Benutzer sichtbare Funktion
- `fix`: Fehlerbehebung
- `docs`: Dokumentation
- `ci`: GitHub Actions und Automatisierung
- `build`: Build oder Paketierung
- `refactor`: Umstrukturierung des Codes ohne Verhaltensänderung
- `test`: Tests
- `chore`: Wartung

<a id="project-boundaries"></a>
## Projektgrenzen

Fennara soll spielunabhängig bleiben. Vermeide APIs oder Anweisungen, die Annahmen über Steuerung, Ziele, Wirtschaft, Inventar, Kampf, Wegfindung, Quests oder den Ablauf der Benutzeroberfläche eines Spiels treffen.

Agenten sollen die tatsächlichen Szenen, Skripte, Ressourcen, Einstellungen, den Laufzeitstatus, Diagnosen und Screenshots eines Godot-Projekts untersuchen und anschließend generische Fennara-Werkzeuge für dieses Projekt zusammensetzen.

<a id="documentation-translations"></a>
## Dokumentationsübersetzungen

Englisch ist die kanonische Quelle. Korrigiere zuerst die englische Fassung und aktualisiere anschließend jede
betroffene Sprache. Der übersetzte Dokumentensatz und die Sprachmetadaten befinden sich in
`docs/i18n/languages.json`.

- Lies die vollständige englische Seite und verfasse die Übersetzung direkt. Verwende keine Dienste für maschinelle Massenübersetzungen und keine Skripte zur Erzeugung von Prosa.
- Behalte Codeblöcke, Inline-Code, Befehle, Pfade, Konfigurationsschlüssel, URLs und Produktnamen exakt bei.
- Bewahre die Quellenmarkierung und die expliziten englischen Anker-Aliasse, die von den Dokumentationsskripten verwaltet werden.
- Kennzeichne eine Übersetzung nicht als von Muttersprachlern geprüft, solange sie nicht von einer fließend sprechenden Person geprüft wurde.
- Übersetze Rechtstexte, interne Agenten-Prompts, generierte Projektanweisungen, Herstellerdateien oder Test-Fixtures nicht als eigenständige Quellen.

Führe nach einer Änderung an der kanonischen oder übersetzten Dokumentation Folgendes aus:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Diese Befehle pflegen Navigationsmetadaten und prüfen die Struktur. Sie
schreiben keine übersetzte Prosa.

Die normale Navigationssynchronisierung behält alle vorhandenen Quellenhashes
bei. Aktualisiere nach einer Änderung an einer englischen Quelle diese Seite
direkt in allen neun übersetzten Sprachen und bestätige anschließend bewusst
nur diese kanonische Quelle:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

Wiederhole `--accept-source <path>` für jede englische Seite, deren
Übersetzungen geprüft und aktualisiert wurden. Bestätige einen Quellenhash erst,
wenn alle neun Übersetzungen die neue Bedeutung enthalten.
