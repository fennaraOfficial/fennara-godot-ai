<!-- fennara-i18n: locale=de source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# Beispiele

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · [Español](../es/examples.md) · [Português do Brasil](../pt-BR/examples.md) · [日本語](../ja/examples.md) · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · **Deutsch** · [Türkçe](../tr/examples.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../examples.md)
<!-- fennara-doc-nav:end -->

Kopiere einen Prompt, ersetze die Projektdetails und sende ihn aus einer MCP-App oder dem
integrierten Fennara-Chat.

| Ziel | Beispiel |
| --- | --- |
| Den verbundenen Editor bestätigen | [Verbindung prüfen](#check-connection) |
| Ein vorhandenes Projekt verstehen | [Vor der Bearbeitung untersuchen](#inspect-a-project-before-editing) |
| Eine fokussierte Änderung vornehmen | [Architekturbewusste Änderung](#make-a-small-architecture-aware-change) |
| Ein laufendes Projekt diagnostizieren | [Laufzeitfehler](#debug-a-runtime-error) |
| Gerenderte Ausgabe untersuchen | [Visuelles Feedback](#visual-feedback) |

<a id="check-connection"></a>
## Verbindung prüfen

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

<a id="inspect-a-project-before-editing"></a>
## Ein Projekt vor der Bearbeitung untersuchen

```text
Use Fennara MCP to inspect this Godot project. Look at the scene tree, relevant files, diagnostics, and project structure before suggesting changes.
```

<a id="make-a-small-architecture-aware-change"></a>
## Eine kleine architekturbewusste Änderung vornehmen

```text
Work inside this existing Godot project like a careful contributor. Inspect how the relevant system is organized, make the smallest useful change, and explain what files/resources changed and how I can test it.
```

<a id="debug-a-runtime-error"></a>
## Einen Laufzeitfehler debuggen

```text
Use Fennara MCP to inspect the latest Godot runtime errors, find the likely source, patch the issue, and explain the fix.
```

<a id="visual-feedback"></a>
## Visuelles Feedback

```text
Use Fennara MCP to capture a screenshot of the current scene, inspect the UI layout, and suggest or make a small fix if something is visibly wrong.
```

<a id="built-in-chat-provider-setup"></a>
## Einrichtung eines Anbieters für den integrierten Chat

Im Fennara-Dock innerhalb von Godot:

```text
/provider
```

Verbinde einen Cloud-Anbieter oder lokalen Anbieter.

Dann:

```text
/model
```

Wähle das Modell aus, das das Dock verwenden soll.

<a id="existing-project-demo-prompt"></a>
## Demo-Prompt für ein vorhandenes Projekt

Dies ist die Art von Prompt, die für die Open-RPG-Demo verwendet wurde:

```text
I want you to work inside this existing Godot RPG project like a careful project contributor. Before making changes, understand how the relevant systems are organized. Reuse the existing architecture and naming style wherever possible. Add the requested feature in the smallest clean way, then tell me what changed and how to try it in-game.
```
