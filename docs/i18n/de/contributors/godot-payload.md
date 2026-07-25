<!-- fennara-i18n: locale=de source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Godot-Payload

<!-- fennara-doc-nav:start -->
[Dokumentation](../README.md)

🌐 **Sprachen**

[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · **Deutsch** · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

Dieses Verzeichnis ist der Quellbaum für den Godot-seitigen Addon-Payload, der in Benutzerprojekte kopiert und in Release-Archive paketiert wird.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` muss als normales Godot-Addon-Verzeichnis installierbar bleiben. Alles, was hier committet wird, soll ein Benutzerprojekt direkt unter `res://addons/fennara/` erhalten können.

<a id="what-belongs-here"></a>
## Was hierher gehört

- `addons/fennara/fennara.gdextension` und `.uid`-Dateien, die Godot lädt.
- `addons/fennara/bin/` mit Editor-GDExtension-Binärdateien aus Plattform-Builds.
- `addons/fennara/dist/` mit generierten Web-Chat-Assets für das native Chat-Webview.
- `addons/fennara/runtime/` mit synchronisierten Godot-seitigen Laufzeit-Hilfsskripten aus `runtime/`.
- `addons/fennara/VERSION`, die während der Paketierung mit `VERSION` im Repository übereinstimmt.

<a id="what-does-not-belong-here"></a>
## Was nicht hierher gehört

- Lokaler Godot-Benutzerstatus wie `.godot/`, `.import/`, Protokolle, temporäre Dateien oder Editor-Caches.
- Root-Paketausgaben von Workflows. Diese gehören in ignorierte Build-Ordner wie `dist/` oder `.package-preview/`.
- Gemeinsam genutzte lokale Laufzeit-Payloads wie die ausführbaren Dateien für Fennara-Daemon/MCP oder die Linux-CEF-Laufzeit. Diese werden von der CLI im Fennara-App-Datenverzeichnis des Benutzers installiert und nicht in jedes Godot-Projekt-Addon kopiert.

<a id="generated-files"></a>
## Generierte Dateien

Der Quellcode der Chat-Benutzeroberfläche befindet sich unter `ui/chat/`. Führe nach Änderungen Folgendes aus:

```powershell
node scripts\sync-chat-ui.mjs
```

Dadurch werden die gebauten Webview-Dateien mit `godot_demo/addons/fennara/dist/` synchronisiert. Dieses Verzeichnis wird absichtlich committet, weil Addon-Benutzer weder Node.js noch einen Frontend-Build-Schritt benötigen sollen.

Der Quellcode der Laufzeit-Helfer befindet sich unter `runtime/`. Führe nach Änderungen Folgendes aus:

```powershell
node scripts\sync-runtime.mjs
```

Dadurch werden die Godot-seitigen Laufzeit-Helfer mit `godot_demo/addons/fennara/runtime/` synchronisiert. Dieses Verzeichnis wird absichtlich committet, weil Addon-Benutzer diese Skripte zusammen mit dem Release-ZIP erhalten sollen.
