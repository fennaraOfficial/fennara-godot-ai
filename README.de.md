<!-- fennara-i18n: locale=de source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[Dokumentation](docs/i18n/de/README.md)

🌐 **Sprachen**

[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Русский](README.ru.md) · [Français](README.fr.md) · **Deutsch** · [Türkçe](README.tr.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/de/demos.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Fennara wird von Godot-Entwicklern und Teams verwendet, darunter [Somni Game Studios](https://somnigamestudios.com/).

Fennara gibt KI-Assistenten eine direkte Verbindung zu Godot. Verwende es aus MCP-fähigen Apps wie Codex, Claude, Cursor, Gemini und Antigravity oder über das optionale Chat-Dock im Editor.

Agenten können Szenen untersuchen, Skripte prüfen, Screenshots aufnehmen, Laufzeitfehler lesen und Änderungen innerhalb des Editors validieren, anstatt ausschließlich anhand der Projektdateien zu raten.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Vergleich von Fennara mit anderen Godot-MCPs" width="100%" />
      </a>
    </td>
    <td>
      <strong>Ausgewählte Demo ansehen</strong><br />
      Vergleich von Fennara mit anderen Godot-MCPs.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Dieses Video abspielen</a><br />
      <a href="docs/i18n/de/demos.md">Alle Demovideos durchsuchen</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## Funktionsweise

- stellt externen KI-Apps über MCP Godot-spezifische Werkzeuge bereit
- fügt im Godot-Editor ein optionales lokales Chat-Dock hinzu
- gibt echtes Godot-Feedback zurück: Szenenbäume, Diagnosen, Screenshots, Laufzeitprotokolle und Validierungsergebnisse
- bindet den Agenten an den geöffneten Editor, anstatt ihn nur mit dem Dateisystem arbeiten zu lassen

Externe MCP-Apps und der integrierte Chat verwenden getrennte Modelleinstellungen. Siehe [MCP-Apps und integrierter Chat](docs/i18n/de/chat-vs-mcp.md) und [Anbieter für den integrierten Chat](docs/i18n/de/providers.md).

<a id="requirements"></a>
## Voraussetzungen

- Godot 4.5 oder neuer.
- Ein unterstütztes Desktop-Betriebssystem: Windows x86_64, Linux x86_64 oder macOS arm64.
- Eine MCP-fähige Coding-App nur dann, wenn du Fennara aus Claude, Codex, Cursor, Gemini, Antigravity oder einer anderen externen KI-App verwenden möchtest.
- Einen Chat-Anbieter nur dann, wenn du das integrierte Fennara-Chat-Dock verwenden möchtest. Das kann ein Schlüssel für einen Cloud-Anbieter oder ein lokaler Anbieter wie Ollama / LM Studio sein.

Die vollständige Installationsanleitung findest du unter [Einrichtung](docs/i18n/de/setup.md).

<a id="what-setup-adds"></a>
## Was die Einrichtung hinzufügt

- das Fennara-Addon unter `res://addons/fennara/`
- eine kleine `fennara`-CLI, die in den Fennara-App-Daten installiert wird
- einen lokalen MCP-Server für KI-Coding-Apps
- einen lokalen Daemon, der MCP- und Chatanfragen an den geöffneten Godot-Editor weiterleitet
- generierte Projektanweisungen für KI-Agenten

Das integrierte Chat-Dock verwendet das Webview der Plattform: Microsoft Edge WebView2 unter Windows, WKWebView/WebKit unter macOS und eine von Fennara verwaltete gemeinsame CEF-Laufzeit unter Linux. MCP-Werkzeuge funktionieren weiterhin, wenn das optionale Chat-Dock nicht gestartet werden kann.

<a id="install"></a>
## Installation

Wähle unter Windows und Linux entweder die Addon- oder die CLI-Installation. Verwende unter macOS die
nachfolgende CLI-Installation, wenn du die macOS-Sicherheitsmeldung vermeiden möchtest, die
nach dem manuellen Herunterladen und Entpacken des Addon-ZIPs erscheinen kann.

<a id="add-the-addon-to-your-project"></a>
### Addon zu deinem Projekt hinzufügen

- Öffne das [neueste Release](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest), lade `fennara-addon-latest.zip` herunter und entpacke dessen Ordner `addons/fennara/` in dein Projekt.

Öffne das Projekt, wähle das Fennara-Dock aus und drücke **Set Up Fennara**.

Fennara ist eine Editor-Abhängigkeit, keine Laufzeitabhängigkeit des Spiels.
Während des Exports entfernt das Editor-Plugin seinen Laufzeit-Autoload aus dem
exportierten Projekt und überspringt `res://addons/fennara/` sowie
`res://.fennara/`. Nach Abschluss des Exports wird das Editor-Projekt
wiederhergestellt. Wenn ein CI-Checkout das Addon über `.gitignore` ausschließt,
führe vor dem Start von Godot
`fennara prepare-export --project path/to/project` aus oder installiere das Addon
in diesem Checkout. Godot validiert Autoload-Pfade, bevor Export-Plugins
ausgeführt werden können. Deshalb muss diese Vorbereitung zuerst erfolgen.

> **macOS:** Das Release-Addon enthält eine native Bibliothek, die derzeit nicht
> von Apple notarisiert ist. Wenn du das Addon-ZIP über einen Browser herunterlädst und
> manuell entpackst, kann macOS melden, dass nicht überprüft werden könne, ob
> `libfennara.macos.editor` frei von Schadsoftware ist. Verwende die nachfolgende
> CLI-Installation, um diese Meldung zu vermeiden. Wenn du die Meldung bereits siehst, schließe Godot,
> entferne den manuell kopierten Ordner `addons/fennara/` und installiere Fennara anschließend
> mit der CLI.

<a id="install-with-the-cli-recommended-on-macos"></a>
### Mit der CLI installieren (unter macOS empfohlen)

Die CLI installiert dasselbe Fennara-Addon. Unter macOS ist dies die empfohlene
Installationsmethode, weil sie den Quarantänepfad über Browser und Finder vermeidet,
der die oben beschriebene Meldung verursacht.

Installiere die CLI unter Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Oder unter macOS und Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Führe Fennara anschließend aus deinem Godot-Projekt aus:

```bash
cd path/to/your-godot-project
fennara install
```

Unter [Einrichtung](docs/i18n/de/setup.md) findest du Hilfe bei Problemen und unter [Fennara CLI](docs/i18n/de/cli.md)
die vollständige Befehlsreferenz.

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## Anbieter einrichten oder MCP-App verbinden

<a id="built-in-chat"></a>
### Integrierter Chat

Öffne **Chat Settings > Chat**, wähle **Open providers** und verbinde einen Anbieter.
Fennara verwendet für Cloud-Anbieter deinen eigenen Schlüssel (BYOK). Du kannst auch einen lokalen
Ollama- oder LM-Studio-Server verwenden. Siehe die [Liste unterstützter Anbieter](docs/i18n/de/providers.md).

<a id="mcp-apps"></a>
### MCP-Apps

Öffne **Chat Settings > MCP Apps**, suche deine App und drücke **Set Up**.

Du kannst eine App auch aus dem Terminal verbinden:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Wenn deine MCP-App nicht in den Chat Settings aufgeführt ist, findest du unter [MCP-Einrichtung](docs/i18n/de/mcp-setup.md)
die vollständige App-Liste und Anleitungen zur manuellen Konfiguration.

<a id="update"></a>
## Aktualisierung

Wenn das Fennara-Dock **Update** anzeigt, drücke darauf und befolge die Anweisungen.

> **Aktualisierung von Fennara v0.3.8 oder älter:** Installiere die CLI einmal mit dem
> obigen Installationsbefehl für deine Plattform neu, bevor du `fennara update` ausführst. Diese CLI-
> Versionen lösen ein nicht mehr verwendetes Release-Tag auf und können aktuelle Releases nicht finden.
> Durch die Neuinstallation der CLI verwenden künftige Aktualisierungen den Endpunkt „Latest Release“
> von GitHub. Dein bestehendes Projekt-Addon und deine Einstellungen werden nicht entfernt.

> **macOS-Benutzer, die von Fennara v0.3.11 aktualisieren:** Installiere die CLI einmal mit dem
> obigen macOS-Installationsbefehl neu, bevor du aktualisierst. Die CLI von v0.3.11 lehnt das
> vorhandene macOS-Framework-Bundle ab, bevor sie sich selbst aktualisieren kann. Die Neuinstallation ersetzt
> nur die CLI. Dein Projekt-Addon und deine Einstellungen werden nicht entfernt.

Um über das Terminal zu aktualisieren, schließe Godot und führe Folgendes aus:

```bash
cd path/to/your-godot-project
fennara update
```

Unter [Fennara aktualisieren](docs/i18n/de/setup.md#update-fennara) findest du Hinweise zur Wiederherstellung und Diagnose.

<a id="tools"></a>
## Werkzeuge

Fennara stellt eine kleine Auswahl Godot-spezifischer Werkzeuge bereit:

- Projektdateien schreiben oder aktualisieren und Diagnosen zurückgeben
- einmalige Skripte zur Szenenbearbeitung ausführen
- Szenenbäume, Knoten, Ressourcen und Godot-Klassen untersuchen
- Szenen validieren
- Screenshots aufnehmen
- Laufzeitsitzungen starten und Laufzeitprotokolle lesen
- kleine Laufzeitskripte auf eine laufende Szene anwenden

Das Ziel besteht nicht darin, die normalen Dateiwerkzeuge eines Agenten zu ersetzen. Fennara stellt die fehlende Godot-Feedbackschleife bereit.

<a id="privacy"></a>
## Datenschutz

Fennara sendet höchstens einmal pro UTC-Tag ein anonymes Ereignis zu aktiven Installationen,
nachdem Godot eine Verbindung hergestellt hat. Es enthält eine zufällige Installations-UUID, die Versionen von Fennara und Godot,
das Betriebssystem und die CPU-Architektur. Es enthält keine Projektdaten,
Pfade, Prompts, Werkzeugaktivitäten, Protokolle, Screenshots oder Kontoinformationen.

Telemetrie kann unter **Chat Settings > Chat > Anonymous telemetry**,
mit `FENNARA_DISABLE_TELEMETRY=true` oder mit `DO_NOT_TRACK=1` deaktiviert werden. Unter [Anonyme
Telemetrie](docs/i18n/de/telemetry.md) findest du den vollständigen Vertrag zu Payload, Speicherung, Übertragung und
Deaktivierung.

<a id="demos"></a>
## Demos

Sieh dir einen praktischen Fennara-Durchlauf an:

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

Weitere Videos:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

Unter [Demos](docs/i18n/de/demos.md) findest du weitere Videos vom Fennara-Kanal.

<a id="star-history"></a>
## Star-Verlauf
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## Dokumentation

| Beginne mit... | Wenn du Folgendes benötigst... |
| --- | --- |
| [Dokumentationsübersicht](docs/i18n/de/README.md) | Alle Leitfäden und Referenzseiten |
| [Einrichtung](docs/i18n/de/setup.md) | Installation, Aktualisierungen und Fehlerbehebung |
| [Chat-Anbieter](docs/i18n/de/providers.md) | Modelle und Schlüssel für den integrierten Chat |
| [MCP-Einrichtung](docs/i18n/de/mcp-setup.md) | Codex, Claude, Cursor und andere MCP-Apps |
| [Werkzeuge](docs/i18n/de/tools.md) | Das Godot-Feedback, das Agenten zur Verfügung steht |
| [Anonyme Telemetrie](docs/i18n/de/telemetry.md) | Erfasste Daten, Übertragungsverhalten und Möglichkeiten zum Deaktivieren |
| [Mitwirken](docs/i18n/de/CONTRIBUTING.md) | Anweisungen für Entwicklung und Pull Requests |

<a id="community"></a>
## Community

Fragen, Hilfe bei der Einrichtung und frühes Feedback sind auf Discord willkommen:

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## Lizenz

Siehe [LICENSE.md](LICENSE.md).
