<!-- fennara-i18n: locale=de source=docs/providers.md sha256=d5f056754b227e0b3fe57ed00c86e9d16b9dd39cef2250d43e4417912ae5e07c -->
<a id="built-in-chat-providers"></a>
# Anbieter für den integrierten Chat

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · **Deutsch** · [Türkçe](../tr/providers.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../providers.md)
<!-- fennara-doc-nav:end -->

Verbinde einen Modellanbieter mit dem Fennara-Chat-Dock in Godot.

> [!NOTE]
> Externe MCP-Apps verwenden ihre eigene Modelleinrichtung. Du musst hier keinen
> Anbieter verbinden, um Fennara aus Codex, Claude, Cursor oder einer anderen MCP-App zu verwenden.
> Siehe [MCP-Apps und integrierter Chat](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Schnelleinrichtung

1. Öffne **Chat Settings > Chat** im Fennara-Dock.
2. Wähle **Open providers**.
3. Wähle einen Cloud-Anbieter und gib deinen eigenen Schlüssel ein oder wähle Ollama oder LM
   Studio für ein lokales Modell.
4. Wähle ein Modell aus.

Du kannst außerdem `/provider` und `/model` im Eingabefeld eingeben.

<a id="provider-reference"></a>
## Anbieterreferenz

| Anbieter | Verbindung | Form der Modell-ID | Hinweise |
| --- | --- | --- | --- |
| OpenAI | Erstelle einen Schlüssel unter [OpenAI API keys](https://platform.openai.com/api-keys). Fennara-Schlüssel/Umgebung: `OPENAI_API_KEY`. | `openai/<model>` | Verwendet die offizielle API von OpenAI. |
| Anthropic | Erstelle einen Schlüssel unter [Claude Console API keys](https://console.anthropic.com/settings/keys). Fennara-Schlüssel/Umgebung: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Verwendet die offizielle Messages API von Anthropic. |
| OpenRouter | Erstelle einen Schlüssel unter [OpenRouter Keys](https://openrouter.ai/settings/keys). Fennara-Schlüssel/Umgebung: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | Verwendet die API von OpenRouter. |
| Ollama Cloud | Erstelle einen Schlüssel unter [Ollama API keys](https://ollama.com/settings/keys). Fennara-Schlüssel/Umgebung: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Verwendet die gehostete API von Ollama, nicht den lokalen Ollama-Server. |
| DeepSeek | Erstelle einen Schlüssel unter [DeepSeek API keys](https://platform.deepseek.com/api_keys). Fennara-Schlüssel/Umgebung: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | Verwendet die OpenAI-kompatible API von DeepSeek. |
| Z.AI | Erstelle einen Schlüssel unter [Z.AI API keys](https://z.ai/manage-apikey/apikey-list). Fennara-Schlüssel/Umgebung: `ZHIPU_API_KEY`. | `zai/<model>` | Verwendet die OpenAI-kompatible API von Z.AI. |
| Moonshot AI | Erstelle einen Schlüssel unter [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys). Fennara-Schlüssel/Umgebung: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Verwendet die OpenAI-kompatible API von Moonshot. |
| Moonshot AI (China) | Erstelle einen Schlüssel unter [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys). Fennara-Schlüssel/Umgebung: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Verwendet die OpenAI-kompatible API von Moonshot China. |
| Kimi For Coding | Erstelle einen Schlüssel in der [Kimi Code Console](https://www.kimi.com/code/console). Fennara-Schlüssel/Umgebung: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Verwendet die Anthropic-kompatible Messages API von Kimi. Erfordert Zugriff auf Kimi Code. |
| MiniMax | Erstelle unter [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) über **API Keys > Create new secret key** einen verbrauchsabhängig abgerechneten Schlüssel. Fennara-Schlüssel/Umgebung: `MINIMAX_API_KEY`. | `minimax/<model>` | Verwendet die Anthropic-kompatible Messages API von MiniMax unter `minimax.io`. |
| MiniMax Token Plan | Verwende den Subscription Key unter [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) **Billing > Token Plan**. Fennara-Schlüssel/Umgebung: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Subscription Keys des Token Plan sind von verbrauchsabhängig abgerechneten API-Schlüsseln getrennt. |
| MiniMax (China) | Erstelle auf der Seite für API-Schlüssel von [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) einen verbrauchsabhängig abgerechneten Schlüssel. Fennara-Schlüssel/Umgebung: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | Verwendet die Anthropic-kompatible Messages API von MiniMax China unter `minimaxi.com`. |
| MiniMax Token Plan (China) | Verwende den Subscription Key von der Token-Plan-Seite von [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Fennara-Schlüssel/Umgebung: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | Chinesische Subscription Keys des Token Plan sind von verbrauchsabhängig abgerechneten API-Schlüsseln getrennt. |
| NVIDIA | Erstelle einen Schlüssel unter [build.nvidia.com](https://build.nvidia.com/). Fennara-Schlüssel/Umgebung: `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | Verwendet die gehostete OpenAI-kompatible NIM API von NVIDIA. |
| Ollama | Führe einen lokalen Ollama-Server aus. Es ist kein Cloud-API-Schlüssel erforderlich. | `ollama/<local-model>` | Standardmäßig `http://127.0.0.1:11434`. |
| LM Studio | Starte den lokalen Server von LM Studio. Standardmäßig ist kein Schlüssel erforderlich. | `lmstudio/<local-model>` | Standardmäßig `http://127.0.0.1:1234/v1`. Falls dein LM-Studio-Server Authentifizierung erfordert, setze `LMSTUDIO_API_KEY` in der Daemon-Umgebung. |

Cloud-Anbieter benötigen deinen eigenen API- oder Abonnementschlüssel. Bei lokalen Anbietern muss
der lokale Server mit einem verfügbaren Modell ausgeführt werden.

OpenRouter-Auswahlen verwenden immer die ausdrückliche Form `openrouter/<provider>/<model>`.
Ältere gespeicherte OpenRouter-Auswahlen in der Form `<provider>/<model>` werden einmalig
beim Laden der Einstellungen migriert. Diese veraltete Form wird jedoch nicht für neue Weiterleitungen verwendet.

Fennara kann Schlüssel aus der Anbieterauswahl im Dock speichern. Chat Settings enthält die Schaltfläche **Open providers**, die dieselbe Auswahl öffnet. Die oben genannten Schlüssel- und Umgebungsnamen versteht Fennara auch, wenn du Umgebungsvariablen bevorzugst. Gespeicherte Schlüssel befinden sich in den lokalen App-Daten des Daemons außerhalb des Godot-Projekts.

<a id="custom-openai-compatible-providers"></a>
## Benutzerdefinierte OpenAI-kompatible Anbieter

Wähle unten in der Anbieterauswahl **Custom**, um einen OpenAI-kompatiblen
Endpunkt wie einen lokalen Router oder ein internes API-Gateway hinzuzufügen. Gib Folgendes ein:

- eine eindeutige kleingeschriebene Anbieter-ID
- den in Fennara angezeigten Namen
- eine Basis-URL, die bei der API-Version endet, zum Beispiel `http://localhost:20128/v1`
- einen optionalen API-Schlüssel
- eine oder mehrere Modell-IDs, Anzeigenamen, Kontextlängen und Grenzwerte für die maximale Zahl an Ausgabe-Token
- optionale Anfrage-Header

Modell-IDs müssen den Erwartungen des Endpunkts entsprechen. Fennara zeigt sie in der
Modellauswahl als `<provider-id>/<model-id>` an, sendet jedoch nur `<model-id>` an
den Anbieter. Der Endpunkt muss die OpenAI-kompatible Anfrageform
`/chat/completions` und die entsprechende Streaming-Antwort implementieren.

API-Schlüssel und Werte benutzerdefinierter Header verwenden Fennaras geschützten Authentifizierungsspeicher des Daemons.
Anbieterdefinitionen bleiben in vom Daemon verwalteten lokalen App-Daten außerhalb des Godot-
Projekts. Mit genauen Modellgrenzen kann Fennara
den Gesprächsverlauf komprimieren, bevor eine Anfrage das Kontextfenster des Modells
überschreitet, und generierte Zusammenfassungen innerhalb der Ausgabegrenze des Modells halten. Vor
Einführung dieser Felder gespeicherte benutzerdefinierte Modelle werden mit kompatiblen
Standardwerten von 64.000 Kontext-Token und 4.096 Ausgabe-Token geladen.

Nach dem Speichern erscheint der benutzerdefinierte Anbieter mit seiner Modellanzahl in der Anbieterauswahl.
Wähle diesen Anbieter aus, um das Formular erneut zu öffnen und Modelle hinzuzufügen oder umzubenennen. Wenn
der API-Schlüssel leer bleibt, wird der gespeicherte Schlüssel beibehalten. Neu eingegebene Header werden
anhand ihres Namens mit den gespeicherten Headern zusammengeführt.

<a id="where-settings-live"></a>
## Speicherort der Einstellungen

Fennara speichert die Einstellungen des integrierten Chats lokal über den Daemon außerhalb des Godot-Projekts:

- API-Schlüssel der Anbieter
- Werte benutzerdefinierter Anbieter-Header
- Definitionen benutzerdefinierter OpenAI-kompatibler Anbieter
- Basis-URLs lokaler Anbieter
- separate maximale Ausgabetoken-Werte für Ollama und LM Studio
- ausgewähltes Modell
- Reasoning-Aufwand
- Zeitlimit für Anbieterantworten
- Chat-Anzeigemodus, entweder in Godot eingebettet oder im Systembrowser geöffnet
- Chatverlauf

Diese Einstellungen werden nicht in `res://addons/fennara/` geschrieben und nicht mit Claude, Codex, Cursor, Gemini oder anderen externen MCP-Apps geteilt.

<a id="provider-response-timeout"></a>
## Zeitlimit für Anbieterantworten

Die Einstellung **Provider response timeout** legt fest, wie lange der integrierte Chat jede Modellanfrage ausführen darf. Der Standardwert beträgt 120 Sekunden. Zulässig sind Werte von 30 bis 3600 Sekunden. Ein höherer Wert kann langsameren lokalen Modellen oder langen, werkzeugintensiven Durchläufen helfen, die Ausführung abzuschließen. Der Daemon wendet das ausgewählte Zeitlimit auf die Anbieteranfrage an und bricht die Anfrage ab, wenn das Limit erreicht ist.

<a id="chat-display-setting"></a>
## Einstellung der Chat-Anzeige

Der Dialog Chat Settings enthält **Open chat in my system browser next time**.

Wenn die Option deaktiviert ist, versucht Fennara, den integrierten Chat innerhalb des Godot-Docks zu rendern. Ist sie aktiviert, zeigt das Dock die Schaltfläche **Open chat** an und startet denselben integrierten Chat über den lokalen Daemon unter `127.0.0.1`. Dies kann die GPU- und Speichernutzung des Godot-Editors reduzieren und dient zugleich als Ausweichweg, falls das native Webview nicht gestartet werden kann.

Die Änderung dieser Einstellung wird beim nächsten Start von Godot wirksam. Sie ändert lediglich, wo die Benutzeroberfläche des integrierten Chats angezeigt wird. Sie ändert weder ausgewählten Anbieter, Modell, API-Schlüssel, Chatverlauf und MCP-App-Einrichtung noch das Modell, das Claude, Codex oder Cursor extern verwenden.

<a id="picker-shortcuts"></a>
## Kurzbefehle der Auswahl

Chat Settings, die Dock-Steuerelemente und `/provider` öffnen dieselbe Anbieterauswahl.
Verwende `/model` oder die Modellsteuerung des Docks, um die Modellauswahl zu öffnen.

Unter [Slash-Befehle des integrierten Chats](slash-commands.md) findest du das Verhalten der Befehlspalette.

<a id="local-providers"></a>
## Lokale Anbieter

Für Ollama:

```bash
ollama serve
ollama pull llama3.1:8b
```

Wähle anschließend:

```text
ollama/llama3.1:8b
```

Ältere Auswahlen in der Form `local/<model>` werden weiterhin als Ollama-Kompatibilitäts-
aliase akzeptiert. Verwende für neue Einstellungen bevorzugt die ausdrückliche Form `ollama/<model>`.

Fennara sendet Ollamas Maximum pro Aufruf im OpenAI-kompatiblen Feld
`max_tokens`, das Ollama seiner nativen Option `num_predict` zuordnet.

Starte für LM Studio den lokalen Server in LM Studio und wähle eine Modell-ID in folgender Form:

```text
lmstudio/<loaded-model-id>
```

Die Einrichtungsformulare für Ollama und LM Studio verwenden dieselbe Standard-
und Kontextbegrenzungsrichtlinie für separate anbieterspezifische maximale
Ausgabeeinstellungen pro Aufruf. Jede Einstellung beträgt standardmäßig 8.192
Tokens. Wenn ein lokaler Server die geladene Kontextlänge meldet, begrenzt
Fennara die Einstellung dieses Anbieters auf die Hälfte des Kontexts, damit
Platz für die Eingabe bleibt. Fennara sendet dieses wirksame Limit als
`max_tokens` und reserviert denselben Wert bei der Entscheidung, wann der
Chatverlauf komprimiert wird.

<a id="model-catalog"></a>
## Modellkatalog

Der Daemon verwaltet einen lokalen Modellkatalog für Cloud-Anbieter und fragt lokale Server nach ihren derzeit verfügbaren Modellen. Wenn sich ein Katalog oder ein lokaler Server ändert, während Godot geöffnet ist, aktualisiere die Modellauswahl oder öffne die Anbieter- bzw. Modellauswahl erneut.

Fennara prüft grundlegende Modellfähigkeiten, bevor eine Anfrage gesendet wird:

- Textausgabe ist erforderlich
- Werkzeugaufrufe sind für die Verwendung von Fennara-Werkzeugen erforderlich
- Bildeingabe ist erforderlich, bevor Bildanhänge als Bildkontext gesendet werden

Die Bildeingabe von Ollama ist im Fennara-Chat noch nicht aktiviert.
