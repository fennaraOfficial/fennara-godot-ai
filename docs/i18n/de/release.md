<!-- fennara-i18n: locale=de source=docs/release.md sha256=60b8cc51e0fcde9b4e18eadc230aaf1d8cc4fad2fe70cbf5190ab9123bac0073 -->
<a id="release-process"></a>
# Release-Prozess

<!-- fennara-doc-nav:start -->
[English](../../release.md) · [简体中文](../zh-CN/release.md) · [Español](../es/release.md) · [Português do Brasil](../pt-BR/release.md) · [日本語](../ja/release.md) · [한국어](../ko/release.md) · [Русский](../ru/release.md) · [Français](../fr/release.md) · **Deutsch** · [Türkçe](../tr/release.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../release.md)
<!-- fennara-doc-nav:end -->

Releases erfolgen manuell. Veröffentliche nicht aus Pull-Request-Workflows.

> [!IMPORTANT]
> Führe Releases von `main` aus, halte `VERSION` und die Workflow-Eingabe identisch und
> entscheide ausdrücklich, ob das Release eine höhere CLI-Mindestversion erfordert.

<a id="release-at-a-glance"></a>
## Release auf einen Blick

| Schritt | Ergebnis |
| --- | --- |
| Versionsänderung vorbereiten und zusammenführen | Die Versionsquellen des Repositorys stimmen überein |
| Package Preview ausführen | Release-förmige Artefakte werden ohne Veröffentlichung gebaut |
| Vorschau untersuchen | Archive, Manifest, Hashes und Linux-CEF-Layout sind geprüft |
| Release von `main` ausführen | Tag und GitHub Release werden veröffentlicht |
| Installation und Aktualisierung einem Smoke-Test unterziehen | Der öffentliche Benutzerablauf ist geprüft |

<a id="versioning"></a>
## Versionierung

`VERSION` ist die maßgebliche Quelle.

Die Release-Werkzeuge akzeptieren SemVer-Werte. Stabile Releases verwenden `X.Y.Z`.
Staging-Kandidaten verwenden ein isoliertes Pull-Request-Vorab-Release wie
`1.2.3-pr.101.2`, wobei `pr-101` der Staging-Kanal und `2` die Kandidatennummer
dieses Kanals ist.

So erhöhst du die Repository-Version:

```bash
node scripts/set-version.mjs X.Y.Z
```

Das Skript aktualisiert:

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- Versionskonstanten des Plugins
- die Paketversion des Rust-Workspace unter `local/`
- `local/Cargo.lock`

Das Addon enthält außerdem `addons/fennara/release.json`. Die stabile Identität wird
vom normalen obigen Befehl automatisch geschrieben. Ein Staging-Build-Workspace verwendet
die ausdrücklichen Identitätseingaben:

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

Staging-Version, Kanal, Quell-Commit und exakter Release-Tag müssen
übereinstimmen. Ein Vorab-Release-Addon ohne diese Identität wird abgelehnt. Vorhandene
stabile Addons aus der Zeit vor `release.json` verwenden weiterhin standardmäßig den
stabilen Track.

Prüfe die Versionssynchronisierung:

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. Release-Commit vorbereiten

1. Führe das Versionsskript aus.
2. Prüfe den Diff.
3. Führe lokale Prüfungen aus, die zur geänderten Oberfläche passen.
4. Führe den PR zur Release-Vorbereitung in `main` zusammen.

Übliche Prüfungen:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

Baue bei Änderungen an der GDExtension das Addon nach Möglichkeit außerdem lokal:

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Package Preview ausführen

Verwende dies vor der Veröffentlichung, wenn die Paketierung geändert wurde oder du einen
Probelauf durchführen möchtest.

GitHub:

```text
Actions > Package Preview > Run workflow
```

Der Workflow baut Windows-, Linux- und macOS-Pakete und lädt temporäre Artefakte hoch.
Er erstellt weder Tags noch GitHub Releases oder `latest`.

Package Preview spiegelt die nicht veröffentlichenden Bestandteile von Release genau genug
wider, um die Release-Paketierung vor dem Zusammenführen zu erproben:

- synchronisiert die ohne Build verwendbare Chat-UI und den Quellcode der Laufzeit-Hilfsfunktionen mit der Addon-Nutzlast
- baut die ZIP-Datei der Linux-CEF-Laufzeit
- schreibt das generierte Manifest der Linux-CEF-Laufzeit
- führt dieses generierte Manifest den Plattform-Paket-Builds zu
- stellt das plattformübergreifende Addon-Archiv zusammen
- benennt lokale/Add-on-Pakete in die vom Manifest verwalteten Namen der Release-Assets um
- validiert das Asset der Linux-CEF-Laufzeit anhand des generierten Manifests
- schreibt `fennara-release-manifest-v<version>.json`
- lädt ein Artefakt `fennara-package-preview-release-assets` hoch, das die
  release-förmigen ZIP-Dateien und das Manifest enthält

Vorschauartefakte sind nützlich, um ZIP-Inhalte und die Form des Manifests vor der
Veröffentlichung zu prüfen. Sie sind Actions-Artefakte, keine öffentlichen Release-Assets.

<a id="3-run-release"></a>
## 3. Release ausführen

Führe den manuellen Release-Workflow von `main` aus:

```text
Actions > Release > Run workflow
```

Eingaben:

```text
version: X.Y.Z
promote_latest: true
```

Die Eingabe `version` muss mit `VERSION` übereinstimmen.

Der Workflow veröffentlicht:

- `v<version>`
- kennzeichnet `v<version>` als GitHub Latest, wenn `promote_latest` wahr ist

Der Release-Workflow bereitet die Linux-CEF-Laufzeit vor der Plattform-Paketierung vor.
Er lädt das angeheftete offizielle minimale Linux-SDK von CEF 139 herunter, stellt die
separate Datei `fennara-webview-cef-linux-x64-<cef-version>.zip` zusammen, entfernt
Symbole aus bereitgestellten ELF-Binärdateien, schreibt ein generiertes aktiviertes
Manifest `local/webview-runtimes/linux-cef.json` und führt dieses Manifest den CLI-Paketen
zu. Der Veröffentlichungsjob prüft anschließend, ob die Release-Assets genau die CEF-ZIP-Datei
enthalten, die vom generierten Manifest benannt wird, und ob ihr SHA-256 übereinstimmt.
Er schreibt außerdem `fennara-release-manifest-v<version>.json`, validiert jedes
referenzierte Asset sowie jeden Hash und lädt dieses Manifest mit dem Release hoch.

Pull-Request-Workflows veröffentlichen keine Releases. Der Package-Preview-Workflow erstellt
release-förmige Testartefakte, einschließlich Manifest und Linux-CEF-Laufzeitnutzlast, damit
Maintainer die Paketierung vor dem Zusammenführen mit einem Smoke-Test prüfen können.
Package Preview ist nicht der benutzerseitige Release-Kanal.

<a id="release-assets"></a>
## Release-Assets

Jedes Release sollte plattformspezifische CLI-/lokale Laufzeitpakete sowie ein gemeinsames
plattformübergreifendes Addon-Paket enthalten.

| Ziel | Assets |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| Alle Plattformen | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

Paketaufgaben:

| Muster | Aufgabe |
| --- | --- |
| `fennara-cli-*` | Nutzlast des Installationsskripts, die ausschließlich die `fennara`-CLI für eine Plattform enthält |
| `fennara-release-local-*` | MCP- und Daemon-Launcher sowie versionierte Laufzeit-Binärdateien für eine Plattform |
| `fennara-release-addon-v*` | Versioniertes plattformübergreifendes Addon, das über das Release-Manifest aufgelöst wird |
| `fennara-addon-latest.zip` | Plattformübergreifender Addon-Alias mit stabilem Namen für Dokumentation und manuelle Downloads |
| `fennara-webview-cef-linux-x64-*` | Nur für Linux bestimmte gemeinsame CEF-Laufzeit, die einmal in den Fennara-Anwendungsdaten installiert wird |
| `fennara-release-manifest-v*` | Installations- und Aktualisierungsplan mit Asset-Namen, SHA-256-Werten, Installationsprimitiven und gemeinsamen Laufzeiten |

Die GDExtension des macOS-Addons ist derzeit nicht von Apple notarisiert. Downloads über
den Browser und manuelles Entpacken im Finder können Quarantänemetadaten weitergeben und
die macOS-Verifizierungsmitteilung auslösen. Die an Benutzer gerichtete
Installationsdokumentation muss unter macOS `fennara install` empfehlen, die Einschränkung
der manuellen ZIP-Datei erklären und betroffene Benutzer anweisen, das manuell kopierte
Addon vor der Neuinstallation über die CLI zu entfernen. Die Release-Validierung behandelt
allein die Erstellung einer ZIP-Datei nicht als Signierung oder Notarisierung für macOS.

Das Präfix `fennara-release-local-*` verhindert, dass ältere CLIs den vom Manifest
verwalteten Paketpfad unbemerkt umgehen.

<a id="release-manifest"></a>
## Release-Manifest

Ab 0.3.0 bevorzugen `fennara install` und `fennara update` das Release-Manifest, wenn
das Release eines veröffentlicht. Das Manifest zeichnet Folgendes auf:

- `schema_version`
- `version`
- `minimum_cli_version`
- unterstützte Installationsprimitive
- plattformspezifische CLI- und lokale Laufzeit-Assets mit SHA-256-Hashes
- das gemeinsame Addon-Asset mit SHA-256
- plattformspezifische gemeinsame Laufzeit-Assets, derzeit Linux CEF

`scripts/release-policy.mjs` ist die maßgebliche Quelle für
`minimum_cli_version`. Der Manifest-Writer wählt die Richtlinie nach der Validierung der
Release-Identität aus, sodass Stable, Package Preview und Staging keine unabhängigen Werte
wählen können. Normale Änderungen des Paketlayouts oder der Asset-Namen sollten durch
Manifestdaten behandelt werden, nicht durch eine Änderung der äußeren CLI. Erhöhe die
Richtlinie, wenn ein Release eine neuere Updater-Übergabe, ein neues Manifestschema,
Installationsprimitiv, Selbstaktualisierungsverhalten oder eine andere CLI-Fähigkeit
benötigt, die eine ältere veröffentlichte CLI nicht sicher ausführen kann.

Wenn die CLI zu alt ist, sollte `fennara update` zuerst den plattformspezifischen Eintrag
`assets.cli` des Manifests verwenden, um die installierte CLI zu aktualisieren, und dann
die Paketaktualisierung mit `--no-self-update` fortsetzen. Wenn die Selbstaktualisierung
für dieses Release oder diesen Installationsort nicht verfügbar ist, sollte der Vorgang
vor der Paketinstallation fehlschlagen und eine klare Anweisung ausgeben, `install.sh`
oder `install.ps1` erneut auszuführen.

Die optionale Release-Identität, die zu Manifestschema 1 hinzugefügt wurde, erfordert keine
Erhöhung der CLI-Mindestversion. Ältere Schema-1-Clients ignorieren unbekannte Felder,
während Staging-fähige Clients die Identität validieren, wenn sie vorhanden ist. Bei einem
zukünftigen Release, das von kanalbezogener Aktivierung oder Updater-Übergabe abhängt, muss
die CLI-Mindestversion vor der Veröffentlichung neu betrachtet werden.

<a id="staging-identity-and-discovery-contract"></a>
## Vertrag für Staging-Identität und -Erkennung

Staging-Kanäle sind pro Pull Request isoliert:

| Wert | Beispiel für PR 101 |
| --- | --- |
| Kanal | `pr-101` |
| Kandidatenversion | `1.2.3-pr.101.2` |
| Exaktes Release | `v1.2.3-pr.101.2` |
| Kanalreferenz | `fennara-staging/pr-101` |
| Zeigerdatei | `fennara-staging-channel-pr-101.json` |

Die Git-Referenz pro Kanal enthält ausschließlich eine kleine Zeigerdatei auf ein exakt
versioniertes Release. Release-Binärdateien befinden sich niemals unter der beweglichen
Kanalreferenz. Die CLI kann diesen Zeiger mit der internen Versionsanfrage
`channel:pr-101` auflösen und verwendet anschließend ausschließlich die exakte Version.

PR 101 und PR 125 verwenden daher unterschiedliche Release-Tags und Zeiger-Assets. Eine
Aktualisierung eines Kanals kann Tester des anderen Kanals nicht umleiten. Die
Veröffentlichung eines Kanals ändert niemals die stabile GitHub-Latest-Kennzeichnung oder
den Kanal eines anderen Pull Requests.

<a id="staging-candidate-workflow"></a>
## Workflow für Staging-Kandidaten

Der manuelle Workflow **Staging Release** baut einen Kandidaten aus dem aktuellen Kopf
eines offenen Pull Requests. Führe ihn von `main` aus und gib Folgendes an:

| Eingabe | Bedeutung |
| --- | --- |
| `pull_request` | Offener Pull Request, der gebaut werden soll |
| `base_version` | Geplante stabile Version im Format `X.Y.Z` |
| `candidate` | Steigende Kandidatennummer für diesen Pull Request |
| `source_commit` | Optionaler vollständiger SHA, der weiterhin der Kopf des Pull Requests sein muss |
| `publish` | Aus für reine Artefaktvalidierung, ein zum Veröffentlichen des Kandidaten |

Der Workflow fixiert den Kopf-SHA des Pull Requests vor jedem Plattform-Build. Die
Windows-, Linux- und macOS-Jobs checken genau diesen Commit mit schreibgeschützten
Berechtigungen, ohne gespeicherte Git-Zugangsdaten, ohne Release-Zugangsdaten und ohne die
Möglichkeit aus, gemeinsam genutzte Abhängigkeits-Caches zu speichern. Sie dürfen kompatible
SCons-/godot-cpp- und Cargo-Caches wiederherstellen, die von vertrauenswürdigen Workflows
des Standardzweigs geschrieben wurden. Staging verwendet die Cache-Aktion ausschließlich
zum Wiederherstellen, sodass Kandidatencode vertrauenswürdige Build-Ausgaben verwenden,
aber Caches für spätere Durchläufe weder ersetzen noch vergiften kann.
Kandidatencode kann Build-Artefakte erzeugen, aber kein GitHub Release veröffentlichen.

Vertrauenswürdige Repository-Skripte validieren anschließend Kandidatenidentität, exaktes
Archivinventar, Addon-Inhalte, Plattform-Paketlayout, Release-Manifest und jeden
SHA-256-Wert. Die Veröffentlichung bleibt deaktiviert, solange `publish` nicht ausdrücklich
ausgewählt wurde.

Wenn die Veröffentlichung aktiviert ist, führt der vertrauenswürdige abschließende Job
Folgendes aus:

1. Er validiert die Kandidatenartefakte erneut als Daten.
2. Er erstellt einen Entwurf, lädt jedes Asset hoch und veröffentlicht ihn als exaktes
   Vorab-Release `v<exact-version>`, ohne GitHub Latest zu ändern.
3. Er lädt die veröffentlichten Assets herunter und vergleicht ihre Namen und Hashes.
4. Er lehnt eine rückwärts gerichtete oder widersprüchliche Kanaländerung ab.
5. Er aktualisiert zuletzt die kleine Zeigerreferenz `fennara-staging/pr-<number>` über
   einen bedingten Schreibvorgang der GitHub Contents API.
6. Er lädt den aktiven Zeiger herunter und prüft dessen exakten Inhalt.

Durchläufe für einen Pull Request werden serialisiert. Verschiedene Pull Requests verwenden
getrennte Nebenläufigkeitsgruppen, Release-Tags und Zeigerreferenzen. Ein erneuter Versuch
desselben Kandidaten prüft das bestehende exakte Release, statt Dateien hineinzumischen.
Der Workflow erstellt niemals Stable GitHub Latest, lädt niemals etwas dorthin hoch und
stuft es niemals dorthin herauf.

Eine stabile Veröffentlichung verwendet keinen wörtlichen Tag oder kein wörtliches Release
`latest`. Der Release-Workflow erstellt das exakte Release `v<version>` als Entwurf, prüft
die hochgeladenen Assets Byte für Byte, veröffentlicht es als veränderliches Release und
kennzeichnet genau dieses Release als GitHub Latest, wenn `promote_latest` wahr ist.
Installationsprogramme und die stabile CLI-Erkennung lösen den API-Endpunkt für GitHubs
Latest Release auf.

Stabile und Staging-Releases sind veränderlich, solange die Unveränderlichkeit von
Repository-Releases deaktiviert ist. Beide Workflows prüfen Release-Metadaten und
heruntergeladene Asset-Bytes, bevor sie die Veröffentlichung abschließen oder einen
Staging-Kanal vorrücken. Die Asset-Veröffentlichung verwendet das Job-bezogene
`GITHUB_TOKEN` mit Schreibzugriff auf Inhalte.

Die Release-Richtlinie erfordert derzeit CLI `0.4.1` für stabile Manifeste und CLI
`0.3.8` für Staging-Manifeste. Die stabile Erkennung löst den stillgelegten Tag `latest`
nicht mehr auf. Stable `0.4.1` erfordert die korrigierte Aktualisierungsvalidierung, die
Vorprüfung für Versionswechsel, die Behandlung des Windows-Betriebsjournals und die
Reparatur der Linux-CEF-Laufzeitmarkierung. Ein
Staging-Kandidat wie `0.4.1-pr.123.1` wird unter SemVer niedriger als Stable `0.4.1`
eingestuft, deshalb muss sein Mindestwert unter der Kandidatenversion bleiben, damit die
Ersteinrichtung die Kandidaten-CLI installieren kann. Ändere keinen der beiden Mindestwerte
ausschließlich aufgrund der Kompatibilität mit dem Manifestschema.

Die gemeinsame Addon-ZIP-Datei enthält jede gebaute GDExtension-Binärdatei, auf die
`godot_demo/addons/fennara/fennara.gdextension` verweist. Godot lädt die passende Bibliothek
für das Betriebssystem des Benutzers und ignoriert die anderen.

Nutzlasten der Linux-CEF-Webview-Laufzeit sind vom Addon-Archiv getrennt. Die
Release-Paketierung erzeugt das aktivierte Laufzeitmanifest und bettet diese Daten in
`fennara-release-manifest-v<version>.json` ein. Die CLI installiert die passende
CEF-Nutzlast einmal im Fennara-Anwendungsdatenverzeichnis des Benutzers:

```text
webview/cef/linux-x64/<cef-version>/
```

Lege `libcef.so`, CEF-Hilfsprogramme, CEF-Ressourcen oder Sprachpakete nicht in
`fennara-addon-*` ab. Package Preview baut ein separates CEF-Artefakt für Tests und
schreibt dieselbe Art von generiertem Laufzeitmanifest, die von Release verwendet wird.
Die Veröffentlichung von Releases bleibt jedoch die einzige benutzerseitige Quelle für
Release-Assets.

Linux-GDExtension-Builds benötigen außerdem den offiziellen Quellcode des CEF-SDK-Wrappers,
aber nicht die CEF-Laufzeitdateien im Addon. CI führt Folgendes aus:

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

und übergibt das extrahierte Verzeichnis als `FENNARA_CEF_ROOT` an SCons. SCons verwendet
`FENNARA_CEF_ROOT/libcef_dll/`, um die kleine Addon-Bibliothek
`libfennara_linux_cef_bridge.so` gegen den angehefteten C++-Wrapper von CEF 139 zu bauen.
Der SDK-Download wird anhand von Version und Hash geprüft, weil der generierte Wrapper-Quellcode
mit der ABI der CEF-Laufzeit übereinstimmen muss. Die Bridge wird mit dem Addon paketiert.
`libcef.so`, Ressourcen, Sprachpakete und `fennara_cef_helper` verbleiben in der separaten
gemeinsamen CEF-Laufzeit.

Paketierungsskripte schlagen fehl, wenn CEF-Laufzeitdateien im Addon-Archiv gefunden werden.
Der Name des Laufzeit-Assets muss wie folgt lauten:

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

Beim Extrahieren der ZIP-Datei müssen sich die erforderlichen Dateien in deren Stamm befinden:

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

Optionale CEF-Laufzeitdateien wie `chrome-sandbox`, `libEGL.so`,
`libGLESv2.so`, `libvk_swiftshader.so`, `libvulkan.so.1`,
`vk_swiftshader_icd.json`, `snapshot_blob.bin` und zusätzliche `locales/*.pak`
sollten enthalten sein, wenn sie in der ausgewählten CEF-Distribution vorhanden sind.

So stellst du die Laufzeit-ZIP-Datei manuell aus einem von einem Maintainer ausgewählten
CEF-Binärbaum zusammen:

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

Unter Linux baut das Skript `fennara_cef_helper` aus
`scripts/cef/linux/fennara_cef_helper.cpp` gegen die offiziellen CEF-Header in
`fennara-cpp/vendor/cef/`. Baue diese Hilfsfunktion auf einem anderen Betriebssystem
zuerst unter Linux und übergib `--helper /path/to/fennara_cef_helper`. Verwende
`--dry-run`, um die ausgewählten Dateien zu untersuchen, bevor die ZIP-Datei geschrieben
wird.

Aktualisiere `local/webview-runtimes/linux-cef.json`, nachdem das Skript den SHA-256
ausgegeben hat:

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Für normale Releases schreibt der Workflow das Manifest der Linux-CEF-Laufzeit automatisch
mit `--write-manifest`. Anschließend kopiert `scripts/write-release-manifest.mjs` die
Laufzeitfelder nach `fennara-release-manifest-v<version>.json`. Aktiviere das eingecheckte
Platzhaltermanifest nicht von Hand, sofern du nicht bewusst einen manuellen Pfad für das
Laufzeit-Asset oder Legacy-Fallback-Verhalten debuggst. Wenn generierte Manifestdaten auf
ein fehlendes Asset verweisen oder dessen SHA-256 nicht übereinstimmt, schlagen der
Release-Workflow und unter Linux `fennara install` / `fennara update` mit einer klaren
Meldung fehl.

Die CLI muss Aktualisierungen der Linux-CEF-Laufzeit atomar veröffentlichen: Extrahiere und
validiere in einem Staging-Verzeichnis, schreibe die Laufzeitmarkierung erst, nachdem die
erforderlichen Dateien vorhanden sind, veröffentliche dann das Versionsverzeichnis und
aktualisiere `current.json` durch Umbenennen einer temporären Datei. Die installierte
Markierung `fennara-cef-runtime.json` muss den Vertrag des nativen Loaders mit
`"runtime": "cef"` kennzeichnen. Installation und Aktualisierung reparieren eine passende
Legacy-Markierung, die ausschließlich `"kind": "cef"` enthält, ohne die CEF-Nutzlast erneut
herunterzuladen. Laufende Editoren verwenden weiterhin die bereits geladene Laufzeit.

Die CLI bettet die generierten Vorlagen für Projektrichtlinien aus `local/templates/` ein.
Wenn die Release-Paketierung die CLI baut, werden diese Vorlagen zusammen mit dem übrigen
CLI-Code in die Binärdatei kompiliert.

<a id="what-latest-means"></a>
## Bedeutung von `latest`

Der Zeiger GitHub Latest Release wählt das versionierte Release aus, das von normalen
Installations- und Aktualisierungsabläufen verwendet wird. Fennara erstellt oder verschiebt
keinen wörtlichen Tag `latest`.

- `install.ps1` und `install.sh` rufen standardmäßig das neueste CLI-Asset ab.
- `fennara update` ruft standardmäßig das Release-Manifest über den Endpunkt GitHub Latest Release ab, aktualisiert die installierte CLI bei Bedarf selbst und löst danach lokale/Addon-/gemeinsame Laufzeit-Assets daraus auf.
- Aktualisierungen im Editor stellen geprüfte Assets vor dem Herunterfahren bereit, prüfen vor dem Ersetzen den vollständigen Digest des bereitgestellten Addons erneut, bewahren das vorherige Addon, die Launcher und das Laufzeitmanifest bis zum erfolgreichen Abschluss der Aktivierungsvalidierung auf und erfordern den Handshake der neu geöffneten GDExtension, bevor Rollback-Daten gelöscht werden.
- `fennara install` ruft standardmäßig das Release-Manifest über den Endpunkt GitHub Latest Release ab und löst anschließend lokale/Addon-/gemeinsame Laufzeit-Assets daraus auf.
- Die Aktualisierungsprüfung des Godot-Plugins vergleicht mit dem neuesten Release von GitHub.

Verwende `promote_latest: false` ausschließlich bei der Veröffentlichung einer Version, die
nicht zur standardmäßigen Benutzerinstallation werden soll.

Installationsprogramme und Release-Downloads sollten Release-Metadaten sowie Schritte für
Download, Extraktion, Installation und Prüfung von Assets ausgeben. Netzwerkabrufe sollten
begrenzte Zeitüberschreitungen verwenden, damit Stillstände von GitHub/CDN mit einer
Diagnose fehlschlagen, statt eingefroren zu wirken. Unter Windows muss `install.ps1` den
Exitcode der CLI-Prüfung kontrollieren, bevor Erfolg ausgegeben wird. Exitcode `-1073741515`
(`0xC0000135`) bedeutet, dass die ausführbare CLI-Datei geschrieben wurde, Windows sie
jedoch nicht starten konnte, weil eine erforderliche DLL fehlt. Weise den Benutzer an,
Microsoft Visual C++ Redistributable 2015-2022 x64 zu installieren und anschließend
`fennara --version`, `fennara doctor` und `fennara install` erneut auszuführen.
Download-URL: `https://aka.ms/vs/17/release/vc_redist.x64.exe`.

<a id="smoke-test-after-release"></a>
## Smoke-Test nach dem Release

Unter Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

In einem Godot-Projekt:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

Prüfe, ob das Projekt Folgendes erhalten hat:

```text
AGENTS.md
addons/fennara/ai/
```

Öffne das Projekt in Godot und bitte anschließend die MCP-Anwendung:

```text
Verwende Fennara MCP, um fennara_status auszuführen, und teile mir mit, welches Godot-Projekt verbunden ist.
```

Aktualisierungstest:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## Regeln

- Der Release-Workflow wird ausschließlich von `main` ausgeführt.
- Die Versionseingabe des Releases muss mit `VERSION` übereinstimmen.
- Pull-Request-Workflows dürfen Testartefakte bauen und hochladen, aber keine Releases veröffentlichen.
- Halte das vorgesehene Release für normale Benutzer als GitHub Latest gekennzeichnet.
- Schreibe veröffentlichte Release-Tags nicht neu, sofern die Maintainer nicht bewusst entscheiden, ein fehlerhaftes Release zu ersetzen.
