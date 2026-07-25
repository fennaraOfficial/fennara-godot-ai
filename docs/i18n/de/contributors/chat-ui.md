<!-- fennara-i18n: locale=de source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Fennara-Chat-Benutzeroberfläche

<!-- fennara-doc-nav:start -->
[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · [Español](../../es/contributors/chat-ui.md) · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · [日本語](../../ja/contributors/chat-ui.md) · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · **Deutsch** · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

Dieser Ordner enthält den Quellcode für die optionale Chat-Oberfläche im Editor.

Die erste Version verzichtet absichtlich auf einen Build: einfaches HTML, CSS und JavaScript.
Dadurch bleibt das OSS-Repository leicht nachvollziehbar und es wird keine Frontend-Toolchain
hinzugefügt, bevor der Webview-Host und die Daemon-Chat-Brücke feststehen.

Die paketierte Kopie befindet sich in `godot_demo/addons/fennara/dist/`.

Führe nach Änderungen an diesem Ordner Folgendes aus:

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## Designhinweise

- Orientiere dich an Godot-Editor-Oberflächen: kompakte Steuerelemente, zurückhaltender Kontrast, kleine Radien,
  klare Fokuszustände und keine Hero-Darstellung im Marketingstil.
- Verwende ausschließlich lokale Fennara-Daemon- und Chat-APIs. Setze keine gehosteten Dienste voraus.
- Für OpenRouter soll ein vom Benutzer bereitgestellter Schlüssel verwendet werden, der lokal außerhalb des
  Godot-Projekts gespeichert wird.
- Halte die Benutzeroberfläche auch ohne Modellverbindung nützlich: Status, Einstellungen, Verlauf
  und Zustände des Eingabefelds sollen weiterhin sichtbar sein.
