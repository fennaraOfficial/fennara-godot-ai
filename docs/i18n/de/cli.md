<!-- fennara-i18n: locale=de source=docs/cli.md sha256=561efbe26f75accb949050e7af9d06ae607040b86d061c04c4dcf8732ba61135 -->
<a id="fennara-cli"></a>
# Fennara-CLI

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · [Español](../es/cli.md) · [Português do Brasil](../pt-BR/cli.md) · [日本語](../ja/cli.md) · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · **Deutsch** · [Türkçe](../tr/cli.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../cli.md)
<!-- fennara-doc-nav:end -->

Verwende die CLI, wenn du das Terminal bevorzugst, Diagnose oder Wiederherstellung benötigst oder eine automatisierte Installation mit einer exakten Version durchführen möchtest.

> [!TIP]
> Die CLI ist die empfohlene Installationsmethode unter macOS. Sie vermeidet die macOS-Sicherheitsmeldung, die auftreten kann, wenn ein über den Browser heruntergeladenes Addon-ZIP manuell entpackt wird und die native Bibliothek die Finder-Quarantäne übernimmt.

<a id="common-flow"></a>
## Üblicher Ablauf

```bash
cd path/to/your-godot-project
fennara install
```

Verwende `fennara doctor`, wenn du die lokale Installation überprüfen oder reparieren musst.

Unter [Einrichtung](setup.md) findest du den normalen Ablauf in Godot. Diese Seite dient als Referenz für Terminalbefehle.

<a id="install-the-cli"></a>
## Die CLI installieren

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS und Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Falls ein manuell entpacktes macOS-Addon bereits eine Meldung für `libfennara.macos.editor` auslöst, schließe Godot und entferne den manuell kopierten Ordner `addons/fennara/`, bevor du `fennara install` ausführst. Andernfalls behält die CLI ein vorhandenes vollständiges Addon bei.

Öffne ein neues Terminal, falls `fennara` nicht sofort verfügbar ist, und überprüfe anschließend die Installation:

```bash
fennara --version
fennara doctor
```

Die CLI wird pro Benutzer installiert. Projekt-Addons verbleiben in ihren Godot-Projekten; gemeinsam verwendete Starter, versionierte Laufzeitumgebungen, Vorgangsdatensätze, Protokolle und Linux CEF werden in den Fennara-Anwendungsdaten gespeichert:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## Befehlsübersicht

| Befehl | Zweck |
| --- | --- |
| `fennara install` | Ein Projekt-Addon und die dazu passenden lokalen Komponenten installieren oder übernehmen |
| `fennara update` | Ein Projekt und seine lokalen Komponenten aktualisieren |
| `fennara doctor` | Die lokale Installation überprüfen oder reparieren |
| `fennara diagnostics` | Einen bereinigten Vorgangsbericht anzeigen |
| `fennara mcp-setup` | Eine externe MCP-App verbinden |
| `fennara prepare-export` | Den Fennara-Autoload vor einem addonfreien CI-Export entfernen |
| `fennara recover` | Eine unterbrochene native Aktualisierung wiederherstellen |
| `fennara self-update` | Nur die installierte CLI aktualisieren |

Führe `fennara --help` aus, um die Zusammenfassung der installierten Befehle anzuzeigen. Verwende `fennara mcp-setup --help` für die unterstützten MCP-App-Ziele.

<a id="install-a-project"></a>
## Ein Projekt installieren

Führe den Befehl in einem Ordner aus, der `project.godot` enthält:

```bash
fennara install
```

Oder gib das Projekt ausdrücklich an:

```bash
fennara install --project path/to/project
```

Ohne `--version` wählt die CLI das aktuelle Release-Manifest aus. Verwende eine exakte Veröffentlichung, wenn Reproduzierbarkeit wichtig ist:

```bash
fennara install --project path/to/project --version <version>
```

Die Installation hat zwei sichere Abläufe:

- Wenn kein vollständiges Addon vorhanden ist, lädt die CLI die ausgewählte Veröffentlichung herunter und überprüft sie, installiert `addons/fennara`, installiert die passenden lokalen Komponenten und schreibt die Fennara-Projektanweisungen.
- Wenn bereits ein vollständiges Addon vorhanden ist, liest die CLI dessen `VERSION`, überprüft die Bibliothek für die aktuelle Plattform und installiert die von der CLI verwalteten Komponenten exakt dieser Version. Das Projekt-Addon bleibt unverändert. Eine ausdrücklich angegebene `--version` muss mit dem vorhandenen Addon übereinstimmen.

<a id="prepare-an-addon-free-ci-export"></a>
## Einen addonfreien CI-Export vorbereiten

Wenn `addons/fennara/` aus einem CI-Checkout ausgeschlossen ist, entferne
Fennaras dauerhaften Laufzeit-Autoload, bevor Godot gestartet wird:

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

Der Befehl bearbeitet ausschließlich den Eintrag `_fennara_game_capture` in
`project.godot`. Andere Autoloads und Einstellungen bleiben erhalten, und der
Befehl kann gefahrlos erneut ausgeführt werden. Dieser Schritt muss vor Godot
erfolgen, da beim Projektstart Autoload-Pfade validiert werden, bevor Editor-
oder Export-Plugins ausgeführt werden können. Alternativ kann die CI das
Fennara-Addon installieren, bevor Godot gestartet wird.

<a id="update-a-project"></a>
## Ein Projekt aktualisieren

Schließe für eine normale Aktualisierung über das Terminal Godot für dieses Projekt und führe Folgendes aus:

```bash
fennara update --project path/to/project
```

Ohne `--version` liest die CLI die Identität des installierten Addons. Stabile Addons verwenden GitHubs Veröffentlichung Latest, während Staging-Addons ausschließlich ihren Kanal `pr-<number>` verwenden. Die Auswahl wird sofort auf eine exakte Version festgeschrieben, auch während eines Selbstaustauschs der CLI. Anschließend überprüft die CLI die Release-Assets, erneuert das Addon und die versionierten lokalen Komponenten, aktualisiert die Projektanweisungen und prüft die Webview-Voraussetzung der Plattform. Verwende `--version <version>`, um ausdrücklich eine exakte Veröffentlichung auszuwählen.

`--no-self-update` ist für kontrollierte Automatisierung oder die Fortsetzung vorgesehen, nachdem die CLI bereits ersetzt wurde. Verwende es nicht, um die Mindestanforderung einer Veröffentlichung an die CLI zu umgehen.

> [!IMPORTANT]
> Wenn du von Fennara v0.3.8 oder älter aktualisierst, installiere die CLI einmal mit dem Plattform-Installationsbefehl unter [Einrichtung](setup.md#install-from-the-terminal-recommended-on-macos) neu, bevor du `fennara update` ausführst. Diese CLI-Versionen fragen ein eingestelltes Release-Tag ab und können aktuelle Veröffentlichungen nicht finden. Die Neuinstallation der CLI entfernt weder dein Projekt-Addon noch deine Einstellungen.

> [!IMPORTANT]
> Installiere unter macOS die CLI einmal neu, bevor du von Fennara v0.3.11 aktualisierst. Diese CLI lehnt das vorhandene Framework-Bundle ab, bevor sie die Selbstaktualisierung erreicht. Die Neuinstallation ersetzt nur die CLI und behält das Projekt-Addon sowie die Einstellungen bei.

<a id="prepare-while-godot-is-open"></a>
### Vorbereiten, während Godot geöffnet ist

Die Schaltfläche zum Aktualisieren im Editor verwendet die Staging-Form:

```bash
fennara update --prepare --project path/to/project
```

Die Vorbereitung lädt das Addon herunter, überprüft es und stellt es dauerhaft bereit. Sie schließt Godot nicht, ersetzt das aktive Addon nicht, wechselt nicht das aktive Laufzeitmanifest und startet den Daemon nicht neu. Das Godot-Dock beobachtet den Vorgangsbeleg und fragt den Benutzer um Erlaubnis, bevor es den abgekoppelten Ablauf zum Schließen, Ersetzen, erneuten Öffnen und Überprüfen startet. Das Dock übergibt die exakte Version, die es bereits ermittelt hat, sodass eine Änderung des Verweises eine laufende Aktualisierung nicht verändern kann.

Fennara unterstützt jeweils eine aktive gemeinsam verwendete Laufzeitversion. Die Aktivierung wird blockiert, wenn ein anderer Godot-Editor mit aktiviertem Fennara weiterhin mit dem gemeinsamen Daemon verbunden ist. Schließe den anderen Editor und versuche es erneut. Die vorherige lokale Version und der vorherige Laufzeitverweis bleiben zur Wiederherstellung ohne Netzwerkzugriff verfügbar.

`--prepare` ist ein grundlegender Baustein für die Godot-Integration. Benutzer des Terminals führen normalerweise `fennara update` aus, wenn Godot bereits geschlossen ist.

<a id="recover-an-interrupted-update"></a>
## Eine unterbrochene Aktualisierung wiederherstellen

Wenn das aktualisierte Addon nicht weit genug geladen werden kann, um den Wiederherstellungsbereich anzuzeigen, schließe Godot und führe Folgendes aus:

```bash
fennara recover --project path/to/project
```

Die CLI stellt nur Vorgänge wieder her, die sich in einem wiederherstellbaren Zustand befinden. Sie stellt das vorherige Addon, die gemeinsam verwendeten Starter und das aktive Laufzeitmanifest wieder her und versucht anschließend, die aufgezeichnete Godot-Programmdatei erneut zu öffnen. Wähle eine bestimmte Transaktion aus, wenn der Support dir deren Vorgangs-ID nennt:

```bash
fennara recover --project path/to/project --operation <operation-id>
```

Abgeschlossene, lediglich vorbereitete und bereits zurückgesetzte Vorgänge werden abgelehnt.

<a id="inspect-health-and-failures"></a>
## Zustand und Fehler überprüfen

`doctor` meldet die erkannte Plattform, die Anordnung der Anwendungsdaten, die aktive Version, Starter, Laufzeitumgebungen, den Zustand des Daemons und die Webview-Voraussetzung:

```bash
fennara doctor
```

Wenn ein laufender Daemon oder eine laufende MCP-Laufzeit gemeldet wird, die älter als `current.json` ist, starte Godot oder die betroffene MCP-App neu, damit die ausgewählte Laufzeit gestartet wird.

Verwende `--repair`, um fehlende grundlegende Anwendungsdatenverzeichnisse neu zu erstellen. Unter Linux bereinigt dies außerdem veraltete CEF-Prozessprofile und repariert die Markierung der aktuellen Laufzeit, wenn bereits eine vollständige verwaltete Laufzeit installiert ist:

```bash
fennara doctor --repair
```

Installations-, Aktualisierungs-, Wiederherstellungs- und Selbstaktualisierungsvorgänge schreiben dauerhafte Zustände und Ereignisse. Zeige den neuesten bereinigten Bericht wie folgt an:

```bash
fennara diagnostics
```

Für einen älteren Vorgang oder eine maschinenlesbare Ausgabe:

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

Berichte enthalten stabile Fehlercodes, Phasen, Komponentenversionen, ausgewählte Asset-Namen und Ergebnisse der Hash-Überprüfung. Sie entfernen Projekt-, Benutzerverzeichnis- und Fennara-Anwendungsdatenpfade, Zugangsdaten, Bearer-Token und URL-Abfragen. Sie enthalten weder Chatnachrichten und Anbieterschlüssel noch Inhalte von Projektdateien.

<a id="configure-an-external-mcp-app"></a>
## Eine externe MCP-App konfigurieren

Das Godot-Chat-Dock stellt diese Befehle unter **Chat Settings > MCP Apps** bereit. Seine Schaltfläche Set Up weist den lokalen Daemon an, die installierte CLI aufzurufen. Dadurch verwenden die Abläufe im Dock und im Terminal dieselbe Implementierung für Konfiguration und Sicherung.

Führe `fennara mcp-setup --help` aus, um ein unterstütztes Ziel auszuwählen. Starte die MCP-App neu, nachdem du ihre Konfiguration geändert hast. Dieser Befehl verbindet eine externe App mit dem Fennara-MCP-Server; er wählt nicht den Modellanbieter aus, den das integrierte Godot-Chat-Dock verwendet. [MCP-Einrichtung](mcp-setup.md) enthält die Zielliste, Konfigurationsorte und Beispiele für die manuelle Konfiguration.

<a id="update-only-the-cli"></a>
## Nur die CLI aktualisieren

Normale Projektaktualisierungen führen die Selbstaktualisierung der CLI automatisch durch. So aktualisierst du nur die installierte CLI:

```bash
fennara self-update
fennara self-update --version <version>
```

Ohne `--version` behält die Selbstaktualisierung den aktiven Installationszweig bei: Stable verwendet GitHubs Veröffentlichung Latest und Staging verwendet ausschließlich den aufgezeichneten PR-Kanal.

Staging wechselt niemals automatisch zu Stable. Um Staging bewusst zu verlassen, schließe Godot und führe `fennara update --version <stable-version> --project <path>` aus. Diese exakte stabile Veröffentlichung wird überprüft, bevor sich die gemeinsam verwendete aktive Version ändert.

Verwende dies, wenn der Support darum bittet oder wenn eine Projektaktualisierung meldet, dass die installierte CLI zu alt ist, um sicher fortzufahren.

<a id="automation-guidance"></a>
## Hinweise zur Automatisierung

- Übergib `--project`, statt dich auf das aktuelle Verzeichnis zu verlassen.
- Lege `--version` fest, wenn ein Build reproduzierbar sein muss.
- Bewahre bei einem Fehler die ausgegebene Vorgangs-ID und den Protokollpfad auf.
- Verwende `fennara diagnostics --operation <id> --json` für strukturierte Berichte.
- Bearbeite `current.json`, Versionsverzeichnisse, Aktualisierungsbelege oder bereitgestellte Addon-Ordner nicht von Hand.
- Führe keine normale Aktualisierung aus, die das Addon ersetzt, während das betreffende Projekt in Godot geöffnet ist. Verwende den Aktualisierungsablauf im Editor oder schließe Godot zuerst.
