<!-- fennara-i18n: locale=de source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# Laufzeit-Helfer

<!-- fennara-doc-nav:start -->
[Dokumentation](../README.md)

🌐 **Sprachen**

[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · **Deutsch** · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

Dieser Ordner enthält den Quellcode für die Godot-seitigen Laufzeit-Hilfsskripte, die von
`runtime_session` und `runtime_script` verwendet werden.

Die paketierte Addon-Kopie befindet sich unter:

```text
godot_demo/addons/fennara/runtime/
```

Führe nach der Bearbeitung von Dateien in diesem Ordner Folgendes aus:

```bash
node scripts/sync-runtime.mjs
```

Laufzeitskripte laden diese Helfer in einem installierten Godot-Projekt weiterhin aus
`res://addons/fennara/runtime/`. Halte die Helfer primitiv und projektunabhängig:
Eingabe, Warten, Knoten-Snapshots, Aufnahmen, Physikabfragen und Unterstützung des
Szenenlebenszyklus sind gute Einsatzbereiche. Spielabhängige Annahmen zu Bewegung, Kampf,
Quests, Inventar oder Abläufen der Benutzeroberfläche sind es nicht.

`image_sheet.gd` wird außerdem von der Skriptfassade für Screenshots verwendet. Halte seine
Komposition deterministisch und unabhängig vom Zustand von Szenen, Animationen oder Gameplay.
