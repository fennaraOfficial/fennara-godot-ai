<!-- fennara-i18n: locale=de source=docs/setup.md sha256=ab1b11ff7dd3472ab14185e920004b6504fa14eb1c29e7c7b1d7a322780af1dd -->
<a id="setup"></a>
# Einrichtung

<!-- fennara-doc-nav:start -->
[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · [Español](../es/setup.md) · [Português do Brasil](../pt-BR/setup.md) · [日本語](../ja/setup.md) · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · **Deutsch** · [Türkçe](../tr/setup.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../setup.md)
<!-- fennara-doc-nav:end -->

Installiere Fennara, wähle aus, wo du chatten möchtest, und verbinde dein Godot-Projekt.

> [!TIP]
> Die meisten Benutzer müssen nur das Addon hinzufügen, das Fennara-Dock öffnen und **Set Up Fennara** drücken. Verwende unter macOS die nachfolgende CLI-Installation, um die Sicherheitsmeldung zu vermeiden, die nach einem manuell heruntergeladenen Addon-ZIP auftreten kann.

<a id="before-you-start"></a>
## Bevor du beginnst

| Voraussetzung | Wann du sie benötigst |
| --- | --- |
| Godot 4.5 oder neuer | Immer |
| Windows x86_64, Linux x86_64 oder macOS arm64 | Immer |
| Eine MCP-fähige KI-App | Nur für die externe MCP-Nutzung |
| Ein Cloud-API-Schlüssel, Ollama oder LM Studio | Nur für den integrierten Chat |
| Das als `dotnet` verfügbare .NET SDK | Nur für C#-Diagnose und die Laufzeit-Vorabprüfung |

<a id="install-from-godot"></a>
## Aus Godot installieren

> [!IMPORTANT]
> Unter macOS enthält das Release-Addon eine native Bibliothek, die derzeit nicht von Apple notarisiert ist. Wenn du das Addon-ZIP über einen Browser herunterlädst und manuell entpackst, kann macOS melden, es könne nicht überprüfen, ob `libfennara.macos.editor` frei von Schadsoftware ist. Verwende [Aus dem Terminal installieren](#install-from-the-terminal-recommended-on-macos), um diese Meldung zu vermeiden.

1. Lade `fennara-addon-latest.zip` aus der [neuesten Veröffentlichung](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest) herunter und kopiere `addons/fennara/` in dein Projekt.
2. Öffne das Projekt und wähle das Fennara-Dock aus.
3. Drücke **Set Up Fennara**.

Fennara installiert die passenden lokalen Komponenten und verbindet das geöffnete Projekt. Falls ein älterer gemeinsam verwendeter Daemon inaktiv ist, beendet die Einrichtung ihn, bevor sie die passende Version aktiviert. Ein Versionswechsel erfordert, dass keine Projekte verbunden sind. Das Projekt, das gerade eingerichtet wird, bleibt normalerweise getrennt, solange die Versionen nicht übereinstimmen. Falls die Einrichtung ein verbundenes Projekt meldet, schließe jeden anderen Editor mit aktiviertem Fennara und versuche es erneut. Falls für das aktuelle Projekt eine veraltete Verbindung bestehen bleibt, schließe diesen Editor, öffne ihn erneut und versuche es anschließend noch einmal.
Wenn die Einrichtung fehlschlägt, stellt das Dock **Retry**, **Copy Report** und **Open Logs** bereit. Kopierte Berichte sind bereinigt und enthalten weder API-Schlüssel und Chat-Inhalte noch Projektdateien.

> [!NOTE]
> Das Addon bleibt in deinem Projekt. Die CLI, der Daemon, der MCP-Server, Protokolle und die gemeinsam verwendete Browserlaufzeit befinden sich außerhalb des Projekts in den Fennara-Anwendungsdaten.

<a id="install-from-the-terminal-recommended-on-macos"></a>
## Aus dem Terminal installieren (unter macOS empfohlen)

Die CLI installiert dasselbe Addon und ist unter macOS die empfohlene Installationsmethode. Sie vermeidet den Quarantänepfad über Browser und Finder, der die oben beschriebene Meldung zur nativen Bibliothek verursacht.

Installiere die CLI unter Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Oder unter macOS und Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Führe anschließend Fennara im Projekt aus:

```bash
cd path/to/your-godot-project
fennara install
```

Falls du das Addon unter macOS bereits manuell entpackt hast und die Meldung siehst, schließe Godot und entferne den manuell kopierten Ordner `addons/fennara/`, bevor du `fennara install` ausführst. Dies ist wichtig, da die CLI ein vorhandenes vollständiges Addon beibehält, statt es zu ersetzen.

Wenn das Projekt bereits ein vollständiges Fennara-Addon enthält, behält die CLI es bei und installiert die dazu passenden lokalen Komponenten. Andernfalls installiert sie außerdem das Addon der aktuellen Veröffentlichung. In der [CLI-Installationsreferenz](cli.md#install-a-project) findest du Informationen zum Festlegen einer Version und zur Automatisierung.

<a id="choose-how-you-use-fennara"></a>
## Wähle aus, wie du Fennara verwendest

| Weg | Modellkonto | Einrichtung |
| --- | --- | --- |
| Integrierter Chat | Ein in den Fennara Chat Settings verbundener Anbieter | [Einen Anbieter verbinden](#connect-the-built-in-chat) |
| Externe MCP-App | Das eigene Modellkonto oder Abonnement der App | [Eine MCP-App verbinden](#connect-an-mcp-app) |
| Beides | Jeder Weg behält seine eigenen Modelleinstellungen | Beide Abschnitte abschließen |

<a id="connect-the-built-in-chat"></a>
### Den integrierten Chat verbinden

1. Öffne **Chat Settings > Chat**.
2. Wähle **Open providers** aus.
3. Verbinde einen Cloud-Anbieter mit deinem eigenen Schlüssel oder verbinde einen lokalen Ollama- oder LM-Studio-Server.
4. Wähle ein Modell aus.

Unter [Anbieter des integrierten Chats](providers.md) findest du unterstützte Anbieter, Schlüssel, lokale Server-URLs und Modell-IDs. Verwende `/provider` und `/model`, um dieselben Aktionen im Eingabefeld auszuführen.

Der eingebettete Chat verwendet die Webview der jeweiligen Plattform:

| Plattform | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | System WKWebView/WebKit |
| Linux | Von Fennara verwaltete gemeinsam verwendete CEF-Laufzeit |

`fennara install`, `fennara update` und `fennara doctor` prüfen diese Voraussetzungen. MCP-Werkzeuge funktionieren weiter, wenn der optionale eingebettete Chat nicht gestartet werden kann.

Um stattdessen den Systembrowser zu verwenden, aktiviere **Open chat in my system browser next time** in den Chat Settings und starte Godot neu. Dadurch ändert sich nur, wo der integrierte Chat erscheint. Derselbe Anbieter, Verlauf und dieselbe Projektverbindung bleiben erhalten.

Um Code an die nächste Nachricht im integrierten Chat anzuhängen, markiere Code im Skripteditor von Godot, öffne das Kontextmenü und wähle **Add to Chat** aus.

<a id="connect-an-mcp-app"></a>
### Eine MCP-App verbinden

Öffne **Chat Settings > MCP Apps**, suche deine App und drücke **Set Up**. Starte die App neu, damit sie Fennara laden kann.

Du kannst eine App auch aus dem Terminal verbinden:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Falls deine App nicht aufgeführt ist, findest du unter [MCP-Einrichtung](mcp-setup.md) alle unterstützten Ziele und Formate für die manuelle Konfiguration.

Externe MCP-Apps verwenden ihre eigenen Modellkonten. Der integrierte Chat verwendet den in den Fennara Chat Settings ausgewählten Anbieter. Unter [MCP-Apps und integrierter Chat](chat-vs-mcp.md) wird der Unterschied erläutert.

<a id="verify-the-connection"></a>
## Die Verbindung überprüfen

Öffne das Godot-Projekt und bitte dann deine MCP-App:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Wenn das falsche Projekt gemeldet wird, wähle im Fennara-Dock das richtige MCP-Ziel aus.

<a id="update-fennara"></a>
## Fennara aktualisieren

Wenn das Dock **Update** anzeigt, drücke darauf und folge den Anweisungen. Fennara lädt die Aktualisierung herunter und überprüft sie, bevor es darum bittet, Godot zu schließen. Nach der Installation öffnet es dasselbe Projekt erneut und behält die vorherige funktionierende Version, bis die Aktualisierung erfolgreich überprüft wurde.

Um über das Terminal zu aktualisieren, schließe Godot und führe Folgendes aus:

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Wenn du von Fennara v0.3.8 oder älter aktualisierst, installiere die CLI einmal mit dem Plattform-Installationsbefehl oben neu, bevor du `fennara update` ausführst. Diese CLI-Versionen fragen ein eingestelltes Release-Tag ab und können aktuelle Veröffentlichungen nicht finden. Durch die Neuinstallation der CLI verwenden zukünftige Aktualisierungen GitHubs Endpunkt Latest Release, ohne dein Projekt-Addon oder deine Einstellungen zu entfernen.

> [!IMPORTANT]
> Installiere unter macOS die CLI einmal neu, bevor du von Fennara v0.3.11 aktualisierst. Diese CLI lehnt das vorhandene Framework-Bundle ab, bevor sie die Selbstaktualisierung erreicht. Die Neuinstallation ersetzt nur die CLI und behält das Projekt-Addon sowie die Einstellungen bei.

Wenn die Überprüfung fehlschlägt, verwende **Restore Previous Version**, **Open Logs** oder **Copy Report** im Dock. In der [CLI-Aktualisierungsreferenz](cli.md#update-a-project) findest du Informationen zu exakten Versionen, zur Vorbereitung und zur Wiederherstellung unterbrochener Aktualisierungen.

<a id="troubleshooting"></a>
## Fehlerbehebung

<a id="an-install-or-update-failed"></a>
### Eine Installation oder Aktualisierung ist fehlgeschlagen

Kopiere den bereinigten Bericht aus dem Dock oder zeige den neuesten Bericht in einem Terminal an:

```bash
fennara diagnostics
```

Unter [CLI-Diagnose](cli.md#inspect-health-and-failures) findest du Informationen zu Vorgangs-IDs, zur JSON-Ausgabe, zu aufgezeichneten Feldern und zu den Bereinigungsgarantien.

<a id="fennara-is-not-found"></a>
### `fennara` wurde nicht gefunden

Öffne ein neues Terminal und führe Folgendes aus:

```bash
fennara doctor
```

Falls der Befehl weiterhin nicht verfügbar ist, füge das Fennara-Verzeichnis `bin` zu PATH hinzu. Auf der [CLI-Installationsseite](cli.md#install-the-cli) sind die Plattformpfade aufgeführt.

<a id="windows-binaries-fail-before-starting"></a>
### Windows-Binärdateien schlagen vor dem Start fehl

Wenn eine Fennara-Binärdatei eine fehlende `VCRUNTIME`- oder `MSVCP`-DLL, den Exitcode `-1073741515` oder `0xc0000135` meldet, installiere Microsoft Visual C++ Redistributable 2015-2022 x64:

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

Dies ist nur auf Windows-Computern erforderlich, auf denen diese Microsoft-Laufzeit-DLLs fehlen.

<a id="a-release-requires-a-newer-cli"></a>
### Eine Veröffentlichung erfordert eine neuere CLI

Falls die Selbstaktualisierung der CLI die erforderliche Version nicht installieren kann, führe das Installationsskript aus [Die CLI installieren](cli.md#install-the-cli) erneut aus und wiederhole anschließend den Befehl.

<a id="the-addon-is-not-visible-in-godot"></a>
### Das Addon ist in Godot nicht sichtbar

Prüfe, ob diese Datei vorhanden ist, und öffne das Projekt anschließend erneut:

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` zeigt das falsche Projekt

Öffne das vorgesehene Projekt und wähle es mit dem Steuerelement für das MCP-Ziel im Fennara-Dock aus.

<a id="c-diagnostics-are-missing"></a>
### C#-Diagnosen fehlen

Prüfe, ob das Projekt genau eine eindeutige `.csproj`-, `.sln`- oder `.slnx`-Datei enthält, und führe anschließend Folgendes aus:

```bash
dotnet --version
```

Informationen zu Browserlaufzeit-Anordnungen, manueller Wiederherstellung und Implementierungsdetails findest du unter [Architektur](architecture.md), [Manuelle Installation](manual-install.md) und in den [häufig gestellten Fragen](faq.md).
