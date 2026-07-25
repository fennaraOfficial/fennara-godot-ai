<!-- fennara-i18n: locale=de source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Godot-Addons

<!-- fennara-doc-nav:start -->
[Dokumentation](../README.md)

🌐 **Sprachen**

[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · **Deutsch** · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

Dieses Verzeichnis bildet die Struktur nach, die Godot innerhalb eines Projekts erwartet:

```text
res://addons/
  fennara/
```

Wenn der Repository-Payload unter `godot_demo/addons/` bleibt, können Paketierungs- und lokale Testskripte das Addon in ein Projekt kopieren, ohne Pfade umzuformen.

<a id="current-addon"></a>
## Aktuelles Addon

`fennara/` ist das installierbare Fennara-Godot-AI-Addon. Es enthält:

- `fennara.gdextension`, den Godot-Einstiegspunkt für die native Erweiterung.
- `bin/`, aus `fennara-cpp/` gebaute Editor-Binärdateien für die Plattformen.
- `dist/`, aus `ui/chat/` synchronisierte generierte Assets des nativen Chat-Webviews.
- `runtime/`, synchronisierte Godot-seitige Hilfsskripte aus dem Quellverzeichnis `runtime/` im Repository-Root.
- `debugger/`, Addon-Assets für den Debugger.
- `VERSION`, die Versionsmarkierung des paketierten Addons.

<a id="rules"></a>
## Regeln

- Halte Addon-relative Pfade stabil. Benutzerprojekte erhalten diesen Ordner als `res://addons/fennara/`.
- Lege hier keine Package-Preview-ZIPs, Release-ZIPs, heruntergeladenen CEF-Archive, Protokolle oder lokalen Testausgaben ab.
- Bearbeite generierte Webview-Dateien in `fennara/dist/` nicht manuell, außer du korrigierst absichtlich die generierte Ausgabe und synchronisierst anschließend auch die Quelländerung.
- Bearbeite synchronisierte Laufzeit-Hilfsdateien in `fennara/runtime/` nicht manuell, ohne auch `runtime/` zu aktualisieren und `node scripts/sync-runtime.mjs` auszuführen.
- Füge neue Addon-Payloads nur dann hier hinzu, wenn sie in Godot-Projekte kopiert werden sollen.
