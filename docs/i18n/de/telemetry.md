<!-- fennara-i18n: locale=de source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# Anonyme Telemetrie

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · [Español](../es/telemetry.md) · [Português do Brasil](../pt-BR/telemetry.md) · [日本語](../ja/telemetry.md) · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · **Deutsch** · [Türkçe](../tr/telemetry.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara sendet höchstens einmal pro UTC-Tag ein kleines anonymes Aktivitätsereignis. Das
Ereignis wird erst gesendet, nachdem sich ein kompatibler Godot-Editor mit dem lokalen Daemon verbindet.
Es hilft den Verantwortlichen dabei, aktive Installationen, die Nutzung unterstützter Plattformen und
die Verbreitung von Versionen zu messen.

Telemetrie ist standardmäßig aktiviert. Öffne **Chat Settings > Chat > Anonymous
telemetry**, um sie zu deaktivieren. Headless- und automatisierte Umgebungen können eine der beiden Variablen setzen:

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

Eine Umgebungsvariable hat Vorrang vor der gespeicherten UI-Einstellung. Wenn
die Telemetrie deaktiviert wird, werden künftige Ereignisse gestoppt und die lokale Telemetrieidentität sowie
der Status des letzten Sendevorgangs gelöscht. Wenn sie wieder aktiviert wird, entsteht eine neue zufällige Identität,
sobald Godot die nächste Verbindung herstellt.

<a id="event-contents"></a>
## Ereignisinhalt

Das Ereignis `fennara_active_installation` enthält ausschließlich:

| Feld | Zweck |
| --- | --- |
| `schema_version` | Version des kleinen Telemetrie-Payload-Vertrags |
| `event` | Fester Ereignisname |
| `installation_id` | Lokal erzeugte zufällige UUID, die nicht aus Hardware oder Konten abgeleitet wird |
| `fennara_version` | Version des laufenden Daemons |
| `godot_version` | Numerische Godot-Version, etwa `4.6.3` |
| `platform` | `windows`, `macos` oder `linux` |
| `architecture` | `x86_64` oder `aarch64` |

Fennara sendet keine Projektnamen, Projektpfade, Kontoinformationen, Prompts,
Chatnachrichten, Anbieterschlüssel, Modellnamen, Werkzeugnamen, Werkzeugargumente, Werkzeug-
ergebnisse, Protokolle, Screenshots, Szeneninhalte, Dateinamen oder Fehlertexte.

<a id="storage-and-transport"></a>
## Speicherung und Übertragung

Der Daemon speichert seine zufällige Identität und den letzten erfolgreichen UTC-Tag im
gemeinsamen Fennara-App-Datenverzeichnis:

```text
Fennara/
  telemetry/
    state.json
```

Der Daemon sendet das Ereignis über HTTPS an
`https://fennara.io/api/telemetry`. Der Empfänger validiert eine exakte Feld-
Positivliste und ersetzt die rohe Installations-UUID durch einen serverseitigen HMAC, bevor
das Ereignis an PostHog weitergeleitet wird. PostHog-Personenprofile und IP-Geolokalisierung sind
für dieses Ereignis deaktiviert.

Der Vercel-Empfänger sieht während der Verarbeitung der HTTPS-Anfrage zwangsläufig normale
Netzwerkmetadaten. Diese Metadaten werden nicht in den PostHog-Ereignis-Payload kopiert.

<a id="delivery-behavior"></a>
## Übertragungsverhalten

Die Telemetrie wird außerhalb der Pfade für Godot-Werkzeugaufrufe ausgeführt:

- Eine begrenzte Warteschlange nimmt Aktivitätssignale ohne Wartezeit an.
- Ein einzelner Hintergrund-Worker verwendet einen einzigen HTTP-Client wieder.
- Anfragen haben ein kurzes Zeitlimit.
- Eine volle Warteschlange, ein Dateisystemproblem, ein Netzwerkfehler oder eine Serverablehnung wird
  stillschweigend toleriert und lässt niemals ein Fennara-Werkzeug fehlschlagen.
- Der UTC-Tag wird erst aufgezeichnet, nachdem der Server ein Ereignis akzeptiert hat, sodass eine spätere
  Godot-Verbindung eine fehlgeschlagene Übertragung erneut versuchen kann.
- Beim Herunterfahren wird kurz gewartet und anschließend der Telemetrie-Worker abgebrochen, anstatt
  den Daemon zu verzögern.

Eine Installation entspricht einer dauerhaft gespeicherten zufälligen UUID. Die Nutzung von Fennara auf zwei Computern
zählt als zwei Installationen. Das Löschen der Fennara-App-Daten oder das Deaktivieren und spätere
erneute Aktivieren der Telemetrie erzeugt eine neue Identität.

Monatlich aktive Installationen werden als unterschiedliche anonyme Installations-
identitäten gezählt, die im Kalendermonat mindestens ein
`fennara_active_installation`-Ereignis gesendet haben.
