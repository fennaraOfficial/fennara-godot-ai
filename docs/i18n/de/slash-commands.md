<!-- fennara-i18n: locale=de source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# Slash-Befehle des integrierten Chats

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · **Deutsch** · [Türkçe](../tr/slash-commands.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

Slash-Befehle sind Kurzbefehle im Fennara-Chat-Dock innerhalb von Godot. Es handelt sich um UI-Befehle, nicht um MCP-Werkzeuge und nicht um Prompts, die an das Modell gesendet werden.

Gib `/` im Eingabefeld ein, um die Befehlspalette zu öffnen.

| Befehl | Öffnet | Verwendung |
| --- | --- | --- |
| `/provider` | Anbieterauswahl | Einen Cloud-Anbieter verbinden, die URL eines lokalen Anbieters konfigurieren oder den Anbieter wechseln. |
| `/model` | Modellauswahl | Ein Modell des aktuellen oder eines verbundenen Anbieters auswählen. |

<a id="how-they-behave"></a>
## Verhalten

- Verwende die Pfeiltasten, um durch Befehlsvorschläge zu navigieren.
- Drücke Enter, um den ausgewählten Befehl auszuführen.
- Drücke Escape, um die Befehlspalette zu schließen.
- Der Text des Slash-Befehls wird aus dem Eingabefeld entfernt, bevor die Chatnachricht gesendet wird.

<a id="common-flow"></a>
## Häufiger Ablauf

Für das integrierte Chat-Dock:

```text
/provider
```

Verbinde OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, lokales Ollama oder LM Studio.

Dann:

```text
/model
```

Wähle das Modell aus, das das Dock verwenden soll.

Verwende diese Slash-Befehle nicht für externe MCP-Apps. Konfiguriere die App mit `fennara mcp-setup` und bitte die App anschließend, Fennara-MCP-Werkzeuge zu verwenden.
