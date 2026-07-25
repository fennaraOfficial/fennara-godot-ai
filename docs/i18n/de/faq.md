<!-- fennara-i18n: locale=de source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# FAQ

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · [Español](../es/faq.md) · [Português do Brasil](../pt-BR/faq.md) · [日本語](../ja/faq.md) · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · **Deutsch** · [Türkçe](../tr/faq.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../faq.md)
<!-- fennara-doc-nav:end -->

Beginne bei Installation und Aktualisierungen mit [Einrichtung](setup.md). Diese Seite bietet
kurze Antworten und Links zur ausführlichen Referenz.

| Frage | Kurze Antwort |
| --- | --- |
| Benötige ich einen Anbieterschlüssel? | Nur für einen Cloud-Anbieter im integrierten Chat |
| Kann ich stattdessen eine externe MCP-App verwenden? | Ja, sie verwendet ihr eigenes Modellkonto |
| Lädt Fennara mein Projekt auf einen Fennara-Server hoch? | Nein |
| Können mehrere Godot-Editoren geöffnet sein? | Ja, wähle das externe MCP-Ziel im Dock aus |

<a id="is-fennara-only-a-code-generator"></a>
## Ist Fennara nur ein Codegenerator?

Nein. Fennara ist ein Godot-spezifischer Agenten-Arbeitsablauf. Es kann mit Projektdateien, Szenen, Diagnosen, Laufzeitfehlern, Screenshots und dem Kontext des Godot-Editors arbeiten.

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## Ist Fennara nur ein weiterer Godot-MCP-Befehlsserver?

Nein. MCP ist ein Weg, Fennara aus Apps wie Codex, Claude, Cursor, Gemini und Antigravity zu verwenden. Fennara besitzt außerdem ein optionales integriertes Chat-Dock. Die zentrale Produktthese ist die Godot-Feedbackschleife: Diagnosen, Validierung, Laufzeitfehler, Screenshots und strukturierte Werkzeugergebnisse, damit Agenten Fehler korrigieren können.

<a id="does-fennara-replace-godot-knowledge"></a>
## Ersetzt Fennara Godot-Kenntnisse?

Nein. Fennara versucht nicht, Godot optional zu machen. Es soll KI-Agenten gegenüber der tatsächlichen Godot-Engine verantwortlich machen.

<a id="how-should-i-install-fennara"></a>
## Wie soll ich Fennara installieren?

Füge unter Windows und Linux das Addon hinzu, öffne das Fennara-Dock und drücke **Set Up
Fennara**, oder installiere es über das Terminal. Installiere es unter macOS über die CLI, um
die Sicherheitsmeldung zu vermeiden, die auftreten kann, wenn ein über den Browser heruntergeladenes Addon-
ZIP manuell entpackt wird. Unter [Einrichtung](setup.md) findest du beide Wege.

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## Warum meldet macOS, dass `libfennara.macos.editor` nicht überprüft werden kann?

Das Release-Addon enthält eine native Bibliothek, die derzeit nicht
von Apple notarisiert ist. Wenn das Addon-ZIP über einen Browser heruntergeladen und
manuell entpackt wird, kann Finder Quarantänemetadaten auf diese Bibliothek übertragen,
wodurch die macOS-Meldung ausgelöst wird.

Verwende die [CLI-Installation](setup.md#install-from-the-terminal-recommended-on-macos), um sie zu vermeiden.
Wenn die Meldung bereits angezeigt wird, schließe Godot, entferne den manuell kopierten
Ordner `addons/fennara/`, installiere die CLI und führe im
Projektverzeichnis `fennara install` aus. Die CLI installiert dasselbe Addon ohne diesen
Quarantänepfad über Browser und Finder.

<a id="do-i-need-a-chat-provider-api-key"></a>
## Benötige ich einen API-Schlüssel für einen Chat-Anbieter?

Nur wenn du im integrierten Fennara-Chat-Dock einen Cloud-Anbieter verwenden möchtest. Externe MCP-Clients verwenden ihre eigene Modell- und App-Konfiguration und können Fennara-MCP-Werkzeuge verwenden, ohne einen Anbieterschlüssel in den Fennara-Chat einzutragen.

Der integrierte Chat kann außerdem lokales Ollama oder LM Studio ohne Cloud-API-
Schlüssel verwenden. Siehe [Anbieter für den integrierten Chat](providers.md).

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## Warum fragt das Dock nach einem Anbieter, obwohl ich bereits `mcp-setup --claude` ausgeführt habe?

`fennara mcp-setup --claude` verbindet Claude mit den Godot-MCP-Werkzeugen von Fennara. Es verbindet den integrierten Fennara-Chat nicht mit Claude und teilt dein Claude-Abonnement nicht mit dem Fennara-Chat.

Verwende Claude Code oder Claude Desktop für den externen MCP-Ablauf. Konfiguriere nur dann einen separaten Anbieter, wenn du innerhalb des Fennara-Docks von Godot chatten möchtest. Siehe [MCP-Apps und integrierter Chat](chat-vs-mcp.md).

<a id="what-are-provider-and-model"></a>
## Was sind `/provider` und `/model`?

Das sind Slash-Befehle im integrierten Fennara-Chat-Dock. `/provider` öffnet die Anbieterauswahl. `/model` öffnet die Modellauswahl. Es handelt sich um UI-Kurzbefehle, nicht um externe MCP-Werkzeuge und nicht um Text, der an das Modell gesendet wird. Siehe [Slash-Befehle des integrierten Chats](slash-commands.md).

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## Sendet Fennara mein Godot-Projekt an einen Fennara-Server?

Nein. Im normalen OSS-Ablauf werden MCP-Client, Daemon und Godot-Addon lokal ausgeführt.
Der integrierte Chat sendet Modellanfragen nur an den von dir konfigurierten Anbieter,
etwa OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax oder einen lokalen Ollama-/LM-Studio-Server.

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## Welches Projekt erhält MCP-Werkzeugaufrufe, wenn mehrere Godot-Editoren geöffnet sind?

Der Daemon leitet externe MCP-Aufrufe an das aktive MCP-Ziel weiter. Verwende die
MCP-Zielsteuerung des Fennara-Docks in Godot, um das Projekt auszuwählen. Sitzungen des integrierten Chats
bleiben an den Godot-Editor gebunden, der diesen Chat geöffnet hat.

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## Warum installiert Linux eine separate CEF-Laufzeit?

Der unter Linux eingebettete Chat verwendet CEF-Off-Screen-Rendering. Der CEF-Payload ist groß, daher
installiert Fennara ihn einmal im Fennara-App-Datenverzeichnis des Benutzers, anstatt
ihn in jedes Godot-Projekt-Addon zu kopieren.

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## Soll das Addon `libcef.so` enthalten?

Nein. `libcef.so`, CEF-Ressourcen, Sprachpakete und der CEF-Helfer gehören in die
gemeinsame Linux-CEF-Laufzeit. Das Addon soll nur die Godot-Addon-Dateien,
GDExtension-Binärdateien, Chat-UI-Dateien und kleine gebündelte Hilfsbinärdateien wie
ripgrep enthalten.

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## Was ist, wenn das Webview des integrierten Chats nicht gestartet werden kann?

Fennara-MCP-Werkzeuge funktionieren weiterhin. Nur das optionale Chat-Dock im Editor benötigt das
Webview der Plattform. Installiere unter Windows die Microsoft Edge WebView2 Runtime, wenn
`fennara doctor` meldet, dass sie fehlt. Unter macOS stammt WKWebView aus dem systemeigenen
WebKit.framework. Führe unter Linux `fennara update` aus, damit die vom Release verwaltete CEF-
Laufzeit installiert oder repariert werden kann.

Du kannst außerdem in den Chat Settings die Option **Open chat in my system browser next
time** verwenden. Dabei bleiben derselbe integrierte Fennara-Chat und dieselben Anbietereinstellungen erhalten,
die Benutzeroberfläche wird jedoch über den lokalen Daemon im Systembrowser statt im
eingebetteten Godot-Webview geöffnet. Starte Godot nach der Änderung der Einstellung neu.

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## Verwendet der Chat beim Öffnen im Browser Claude oder meine MCP-App?

Nein. Die Browseranzeige ist nur eine Entscheidung für Benutzeroberfläche und Laufzeit des integrierten Fennara-Chats.
Er verwendet weiterhin den in den Fennara-Chat-Einstellungen ausgewählten Anbieter. `fennara
mcp-setup --claude` und ähnliche Befehle konfigurieren externe MCP-Apps. Sie
konfigurieren nicht das Modell des integrierten Chats.

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## Schreibt `fennara update` die Konfiguration von MCP-Apps neu?

Nein. `fennara update` aktualisiert bei Bedarf die installierte CLI, das Projekt-
Addon, das lokale Laufzeitpaket, die generierten Projektanweisungen und die von der Plattform verwalteten
Laufzeit-Assets. Führe `fennara mcp-setup` nur dann erneut aus, wenn du die
Konfiguration einer MCP-App einrichtest oder reparierst.

<a id="where-does-chat-history-live"></a>
## Wo wird der Chatverlauf gespeichert?

Der Chatverlauf wird lokal vom Daemon gespeichert und ist auf das aktuelle Godot-
Projekt beschränkt. Anbieterschlüssel und URLs lokaler Anbieter werden ebenfalls lokal vom
Daemon außerhalb des Godot-Projekts gespeichert.

<a id="what-should-agents-use-fennara-tools-for"></a>
## Wofür sollen Agenten Fennara-Werkzeuge verwenden?

Verwende Fennara für Godot-spezifisches Feedback: Szenenbäume, geänderte Knoten- und Ressourcen-
eigenschaften, Diagnosen, Validierung, Laufzeitsitzungen, Screenshots und den Zustand des Editor-
Debuggers. MCP-Clients sollen weiterhin ihre eigenen normalen Werkzeuge zum Lesen und
Durchsuchen von Dateien verwenden, sofern kein Fennara-spezifisches Werkzeug benötigt wird.
