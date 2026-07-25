<!-- fennara-i18n: locale=de source=CONTEXT.md sha256=ee0d279d8a4916d5cf894616b1c72658669a36bf0ec958efef5a09ee196c704e -->
<a id="fennara-context"></a>
# Fennara-Kontext

<!-- fennara-doc-nav:start -->
[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · [Español](../es/CONTEXT.md) · [Português do Brasil](../pt-BR/CONTEXT.md) · [日本語](../ja/CONTEXT.md) · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · **Deutsch** · [Türkçe](../tr/CONTEXT.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

Diese Datei definiert gebräuchliche Begriffe, die in der Fennara-Dokumentation, in Issues, Release Notes und Anweisungen für Agenten verwendet werden.

<a id="product-terms"></a>
## Produktbegriffe

**Fennara**

Die Godot-spezifische Agentenumgebung in diesem Repository. Fennara verbindet KI-Werkzeuge mit echtem Godot-Feedback wie Diagnosen, Szenenvalidierung, Laufzeitfehlern, Screenshots und Projektanweisungen.

**Godot-Addon**

Das installierbare Plugin, das unter `res://addons/fennara/` in das Godot-Projekt eines Benutzers kopiert wird. Es umfasst die Dock-Benutzeroberfläche, Godot-seitige Inspektionswerkzeuge, die native GDExtension-Bibliothek, paketierte Chat-UI-Assets, Laufzeit-Hilfsskripte und die projektlokale Addon-Version.

**Fennara CLI**

Der auf dem Rechner des Benutzers installierte Befehl `fennara`. Er übernimmt Installation, Aktualisierung, Selbstaktualisierung der CLI, Doctor-Prüfungen, die Einrichtung von MCP-Apps, Warnungen zu Webview-Voraussetzungen, Prüfungen der C#-Einrichtung und generierte Projektanweisungen.

**Lokales Paket**

Das Release-ZIP mit lokalen ausführbaren Fennara-Dateien wie MCP-Server, Daemon, Laufzeitbinärdateien und Launcher-Binärdateien für eine Plattform und Architektur.

**Projektanweisungen**

Generierte Anweisungsdateien in einem Godot-Projekt, darunter `AGENTS.md` und die weitergeleiteten Referenzen unter `addons/fennara/ai/`, damit KI-Coding-Agenten wissen, wann und wie sie Fennara verwenden sollen.

<a id="mcp-terms"></a>
## MCP-Begriffe

**Fennara-MCP-Server**

Der lokale stdio-MCP-Server, der von einer KI-Coding-App wie Claude Code, Cursor, Cline, Gemini CLI oder einem anderen MCP-Client gestartet wird. Er stellt dieser externen App Fennara-Werkzeuge bereit.

**MCP-App**

Eine externe KI-App, die durch `fennara mcp-setup` konfiguriert wird. Die Einrichtung der MCP-App legt fest, welche externe App Fennara-Werkzeuge aufrufen kann. Sie wählt nicht das Modell aus, das im integrierten Chat von Fennara verwendet wird.

**MCP-Ziel**

Das aktuell ausgewählte Godot-Projekt, das Fennara-MCP-Aufrufe empfangen soll.

**Werkzeugschema**

Die für das Modell bestimmte Beschreibung eines Fennara-MCP-Werkzeugs, einschließlich Argumenten, Grenzen und Hinweisen zum Arbeitsablauf.

**Werkzeugergebnis-Umschlag**

Das knappe, für das Modell bestimmte Ergebnis, das nach einem Werkzeugaufruf zurückgegeben wird. Fennara-Ergebnisse sollen den Status, wichtige Erkenntnisse und den nächsten nützlichen Kontext erklären, ohne unnötige Rohdaten auszugeben.

<a id="built-in-chat-terms"></a>
## Begriffe zum integrierten Chat

**Integrierter Chat**

Fennaras eigene Chat-Oberfläche innerhalb des Godot-Addons oder im Systembrowser. Sie ist von externen MCP-Apps getrennt. Ein Benutzer kann Claude Code für MCP konfigurieren und dennoch einen anderen Anbieter oder ein anderes Modell für den integrierten Chat wählen.

**Chat-Oberfläche**

Der Anzeigemodus des integrierten Chats. Der eingebettete Modus verwendet das native Godot-Dock-Webview. Der Browsermodus stellt dieselbe Benutzeroberfläche über den lokalen Daemon bereit und öffnet sie im Systembrowser.

**Chat-Anbieter**

Ein Backend, das Antworten für den integrierten Chat erzeugen kann, etwa OpenAI, Anthropic,
OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax,
lokales Ollama oder LM Studio.

**Modellreferenz**

Die anbieterqualifizierte Modellkennung, die im integrierten Chat ausgewählt ist. Slash-Befehle wie `/provider` und `/model` helfen Benutzern dabei, Anbieter zu verbinden und Modellreferenzen auszuwählen.

**Anbieterverbindung**

Vom Daemon verwaltete lokale Einstellungen und Authentifizierungsdaten eines Chat-Anbieters, einschließlich API-Schlüsseln oder lokaler Basis-URLs. Anbietergeheimnisse sollen im vom lokalen Daemon verwalteten Speicher bleiben und nicht im Godot-Projekt liegen.

**Generierungs-Trace**

Gespeicherte Metadaten für eine Generierung im integrierten Chat, die Assistentennachrichten, Werkzeugaufrufe, die Auswahl von Anbieter und Modell sowie Nutzungs- und Kostenprotokolle mit der Generierung verknüpfen, die sie hervorgebracht hat.

<a id="runtime-and-webview-terms"></a>
## Begriffe zu Laufzeit und Webview

**Fennara-Daemon**

Der lokale Dienst, der MCP-Aufrufe und Anfragen des integrierten Chats mit dem Godot-Addon verbindet, lokalen Laufzeitstatus speichert und vom Daemon bereitgestellte Chat-Routen wie `/chat/` ausliefert.

**Laufzeitsitzung**

Eine vom Daemon verwaltete Godot-Laufzeitsitzung, die für Laufzeitinspektion, Protokolle, Validierung, Screenshots und künftige Arbeitsabläufe mit laufenden Szenen verwendet wird.

**Godot-Snapshot**

Ein reversibler Snapshot des Projektzustands, der vor einem von Fennara unterstützten Turn erstellt wird, der Dateien verändern könnte. Die Snapshot-Einrichtung soll abgeschlossen sein, bevor der Benutzer-Turn dauerhaft gespeichert wird, damit eine fehlgeschlagene Einrichtung keine verwaisten Prompts hinterlässt.

**Webview-Laufzeit**

Die Plattformunterstützung, die zur Anzeige des integrierten Chats in oder in der Nähe von Godot benötigt wird. Windows verwendet WebView2, macOS verwendet WebKit/WKWebView und Linux verwendet eine gemeinsame CEF-Laufzeit, die unter den Fennara-App-Daten installiert ist.

**Gemeinsame Linux-CEF-Laufzeit**

Das externe Linux-CEF-Laufzeitpaket, das vom Linux-Chat-Webview verwendet wird. Es wird einmal im App-Datenverzeichnis von Fennara installiert und darf nicht in jedes Godot-Addon-ZIP gebündelt werden.

<a id="release-terms"></a>
## Release-Begriffe

**Release-Manifest**

Das JSON-Asset mit dem Namen `fennara-release-manifest-v<version>.json`. Es ordnet Release-Assets Plattformen zu, zeichnet SHA-256-Hashes auf, deklariert gemeinsame Laufzeit-Assets und legt `minimum_cli_version` fest.

**Mindestversion der CLI**

Die niedrigste Version der `fennara`-CLI, die ein Release-Manifest verwenden darf. Wenn ein
Release neuere Installations- oder Aktualisierungslogik benötigt, aktualisiere dessen Track in
`scripts/release-policy.mjs`. Der Manifest-Writer wendet diese Richtlinie an, nachdem er
die Release-Identität validiert hat. Workflows wählen diesen Wert nicht aus.

**Neuestes Release**

GitHubs Zeiger „Latest Release“ auf ein exaktes versioniertes Release. Installer und
standardmäßige Aktualisierungen lösen diesen Zeiger über die GitHub-API auf. Fennara verwendet
weder ein wörtliches Tag noch ein wörtliches Release namens `latest`. Das Aktualisieren von Quelldateien nach der Veröffentlichung
ändert die Release-Assets nicht. Bereits veröffentlichte Manifest-Assets müssen
ausdrücklich ersetzt werden.
