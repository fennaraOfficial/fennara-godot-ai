<!-- fennara-i18n: locale=de source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# Manuelle Installation

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · **Deutsch** · [Türkçe](../tr/manual-install.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Verwende diese Seite nur, wenn du Fennara ohne den Einrichtungsablauf in Godot oder ohne `fennara install` zusammenstellen musst.

> [!TIP]
> Unter Windows und Linux sollten die meisten Benutzer `addons/fennara` zum Projekt hinzufügen, das Fennara-Dock öffnen und **Set Up Fennara** drücken. Verwende unter macOS die CLI. Siehe [Einrichtung](setup.md).

> [!IMPORTANT]
> Die manuelle Installation des Addon-ZIPs wird unter macOS nicht empfohlen. Das Addon enthält eine native Bibliothek, die derzeit nicht von Apple notarisiert ist. Ein Download im Browser mit anschließendem Entpacken im Finder kann dazu führen, dass macOS meldet, es könne nicht überprüfen, ob `libfennara.macos.editor` frei von Schadsoftware ist. Verwende die [CLI-Installation](setup.md#install-from-the-terminal-recommended-on-macos), um diese Meldung zu vermeiden. Falls die Meldung bereits erscheint, schließe Godot, entferne den manuell kopierten Ordner `addons/fennara/` und führe `fennara install` aus.

Die manuelle Installation besteht aus vier Teilen: der CLI, dem Projekt-Addon, dem gemeinsam verwendeten lokalen Laufzeitpaket und der optionalen Konfiguration einer MCP-App.

<a id="1-download-release-files"></a>
## 1. Release-Dateien herunterladen

Öffne die neueste GitHub-Veröffentlichung:

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

Lade das Release-Manifest, die Dateien für deine Plattform und das gemeinsam verwendete Addon-ZIP herunter.

| Zweck | Asset |
| --- | --- |
| Release-Plan und SHA-256-Werte | `fennara-release-manifest-v<version>.json` |
| Windows x86_64 CLI | `fennara-cli-windows-x86_64-v<version>.zip` |
| Lokale Laufzeit für Windows x86_64 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 CLI | `fennara-cli-linux-x86_64-v<version>.zip` |
| Lokale Laufzeit für Linux x86_64 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Eingebettete Webview für Linux x86_64 | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 CLI | `fennara-cli-macos-arm64-v<version>.zip` |
| Lokale Laufzeit für macOS arm64 | `fennara-release-local-macos-arm64-v<version>.zip` |
| Versioniertes Addon für alle Plattformen | `fennara-release-addon-v<version>.zip` |

Die Veröffentlichung enthält außerdem diesen Addon-Alias mit stabilem Namen für Dokumentation und manuelle Downloads:

```text
fennara-addon-latest.zip
```

Das Manifest zeichnet die erwarteten SHA-256-Werte für die lokale Laufzeit, das Addon und die gemeinsam verwendeten Laufzeit-Assets auf. Verwende es beim Überprüfen manueller Downloads als maßgebliche Quelle.

<a id="2-install-the-cli"></a>
## 2. Die CLI installieren

Entpacke das ZIP `fennara-cli`.

Füge dessen Verzeichnis `bin` zu PATH hinzu oder kopiere die Binärdatei `fennara` in einen deiner vorhandenen PATH-Ordner.

Überprüfe sie:

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Das Godot-Addon installieren

Entpacke das ZIP `fennara-addon`.

Kopiere:

```text
addons/fennara
```

in dein Godot-Projekt, sodass das Projekt Folgendes enthält:

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. Das lokale Laufzeitpaket installieren

Normalerweise übernimmt die CLI dies für dich. Eine manuelle Einrichtung der Laufzeit ist nur erforderlich, wenn du `fennara install` nicht verwenden möchtest.

Standardordner für Fennara-Daten:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

Die erwartete Anordnung lautet:

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

Unter Windows verwenden die Binärdateien `.exe`.

`current.json` verweist die Starter-Binärdateien auf die aktive Laufzeitversion. Die normalen Befehle `fennara install` und `fennara update` erstellen diese Datei automatisch.

Der eingebettete Linux-Chat verwendet den gemeinsam verwendeten Laufzeitort `webview/cef/linux-x64/<cef-version>/`. Bei normalen Ausführungen von `fennara install` und `fennara update` wird die von der Veröffentlichung verwaltete CEF-Laufzeit anhand des Release-Manifests und Assets automatisch installiert. Wenn du alles von Hand installierst, entpacke `fennara-webview-cef-linux-x64-<cef-version>.zip` in diesen gemeinsam verwendeten Laufzeitort und schreibe die passende Markierungsdatei `webview/cef/linux-x64/current.json`. Bewahre diese Nutzlast außerhalb des Godot-Projekt-Addons auf; `addons/fennara` darf weder `libcef.so` noch andere CEF-Laufzeitdateien enthalten.

Diese CEF-Nutzlast dient ausschließlich dem eingebetteten Linux-Chat. Benutzer können in den Chat Settings die Option **Open chat in my system browser next time** auswählen, um denselben integrierten Chat über den lokalen Daemon im Systembrowser statt in der eingebetteten Godot-Webview anzuzeigen.

Die endgültige Linux-CEF-Anordnung sollte wie folgt aussehen:

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json` muss Folgendes enthalten:

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` muss das passende Release-Manifest für das CEF-Asset sein, zum Beispiel:

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Lege keine beschreibbaren Browserdaten im CEF-Versionsverzeichnis ab. Bei normaler Verwendung werden Profile und Protokolle pro Editor unter den Cache- und Protokollstammordnern der Fennara-Anwendungsdaten geschrieben, während die Laufzeitnutzlast gemeinsam verwendet wird und schreibgeschützt bleibt.

<a id="5-configure-your-mcp-app"></a>
## 5. Deine MCP-App konfigurieren

Konfiguriere deine MCP-App, nachdem das lokale Laufzeitpaket installiert wurde:

```bash
fennara mcp-setup --claude
```

Weitere Ziele:

```bash
fennara mcp-setup --help
```

Starte die MCP-App nach der Einrichtung neu.

Wenn deine App nicht aufgeführt ist oder du im Rahmen dieser Installation die MCP-Konfiguration manuell bearbeitest, findest du unter [MCP-Einrichtung](mcp-setup.md) den stabilen Starterpfad sowie Beispiele für JSON und TOML.

Dies verbindet nur die externe MCP-App mit den Godot-Werkzeugen von Fennara. Es konfiguriert nicht den Modellanbieter des integrierten Fennara-Chat-Docks. Konfiguriere das Dock in Godot, wenn du den integrierten Chat verwenden möchtest, oder lies [MCP-Apps und integrierter Chat](chat-vs-mcp.md).

<a id="6-verify"></a>
## 6. Überprüfen

Öffne das Godot-Projekt und bitte dann deine MCP-App:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Wenn der Pfad stimmt, funktioniert die manuelle Installation.

<a id="recommended-shortcut"></a>
## Empfohlene Abkürzung

Selbst wenn du die CLI manuell installierst, kannst du sie das Addon und das lokale Laufzeitpaket installieren lassen:

```bash
cd path/to/your-godot-project
fennara install
```

Die CLI schreibt außerdem Projektanweisungen für KI-Coding-Agenten:

```text
AGENTS.md
addons/fennara/ai/
```

Das KI-Verzeichnis enthält kompakte, stets gelesene Richtlinien, einen Index und spezialisierte Seiten, die nur bei Bedarf geladen werden. Ein manuell kopiertes Addon-ZIP kann dieses gepackte Verzeichnis enthalten, erstellt oder aktualisiert jedoch nicht die Datei `AGENTS.md` im Projektstamm. Verwende `fennara install` und `fennara update`, wenn Fennara die vollständigen Projektanweisungen verwalten und aktualisieren soll.
