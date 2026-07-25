<!-- fennara-i18n: locale=de source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# Skripte

<!-- fennara-doc-nav:start -->
[Dokumentation](../README.md)

🌐 **Sprachen**

[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · [Español](../../es/contributors/scripts.md) · [Português do Brasil](../../pt-BR/contributors/scripts.md) · [日本語](../../ja/contributors/scripts.md) · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · **Deutsch** · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

Dieses Verzeichnis enthält Repository-Automatisierung, die von lokaler Entwicklung, Package Preview und Release-Workflows gemeinsam verwendet wird.

Skripte sollen klein, deterministisch und sicher aus dem Repository-Root ausführbar sein, sofern ihr Hilfetext nichts anderes angibt. Sie dürfen außerhalb des Repositorys keinen benutzerspezifischen Status schreiben.

<a id="version-scripts"></a>
## Versionsskripte

- `set-version.mjs`: aktualisiert `VERSION` im Repository, `VERSION` im Addon, Metadaten des lokalen Rust-Workspace, Paketversionen in der Lockdatei und die Versionskonstante des C++-Plugins.
- `check-version.mjs`: überprüft, ob diese versionierten Dateien weiterhin synchron sind.

Führe `check-version.mjs` in CI und vor der Release-Paketierung aus. Verwende `set-version.mjs` nur, wenn du die Fennara-Version absichtlich änderst.

<a id="packaging-scripts"></a>
## Paketierungsskripte

- `package-preview.mjs`: synchronisiert committete Addon-Payloads und stellt anschließend plattformspezifische Preview-Archive zusammen, nachdem GDExtension und lokale Rust-Binärdateien bereits gebaut wurden.
- `package-addon-all.mjs`: fasst plattformspezifische Addon-Teile im endgültigen plattformübergreifenden Addon-Archiv zusammen.
- `release-policy.mjs`: definiert die kompatible Mindestversion der veröffentlichten CLI für jeden Release-Track.
- `write-release-manifest.mjs`: schreibt `fennara-release-manifest-v<version>.json` aus den Release-Assets und validiert jeden referenzierten SHA-256.

Beide Skripte verwenden `.package-preview/` als temporären Stagingbereich und schreiben ZIP-Ausgaben in den Ordner `dist/` im Repository-Root. Diese Ausgaben werden ignoriert und dürfen nicht committet werden.

Paketierungsskripte müssen den Addon-Payload klein halten. Insbesondere dürfen Linux-CEF-Laufzeitdateien wie `libcef.so` und `fennara_cef_helper` nicht in `fennara-addon-*` gebündelt werden. CEF wird einmal im gemeinsamen Fennara-App-Datenverzeichnis des Benutzers installiert.

<a id="staging-release-scripts"></a>
## Skripte für Staging-Releases

- `write-staging-candidate.mjs`: erstellt die exakte Vorabrelease-Identität für einen Pull Request und einen eingefrorenen Quell-Commit.
- `validate-staging-build.mjs`: prüft vor der Veröffentlichung Addon-Teile, Plattformarchive, das zusammengesetzte Addon, das Release-Manifest und Linux CEF.
- `smoke-public-release.mjs`: lädt jeden veröffentlichten Kandidaten über seine nicht authentifizierte Browser-URL herunter und prüft die vertrauenswürdigen Asset- und Manifest-Hashes, bevor der Kanal weitergeschaltet wird.
- `write-staging-pointer.mjs`: schreibt den kleinen PR-spezifischen Zeiger, nachdem das exakte Release-Manifest gehasht wurde.
- `check-staging-channel-advance.mjs`: lehnt rückwärts gerichtete oder widersprüchliche Kanalbewegungen ab.
- `validate-staging-publish-bundle.mjs`: validiert das endgültige Artefaktpaket erneut, ohne Kandidatencode auszuführen.
- `verify-published-assets.mjs`: vergleicht die erwarteten und heruntergeladenen Namen sowie SHA-256-Werte von GitHub-Release-Assets.

Diese Skripte unterstützen `.github/workflows/staging-release.yml`. Kandidaten-Build-Jobs werden ohne Release-Zugangsdaten ausgeführt. Nur der vertrauenswürdige abschließende Job kann veröffentlichen. Er schaltet die PR-spezifische Git-Referenz weiter, nachdem das exakte Release heruntergeladen und geprüft wurde.

<a id="linux-cef-scripts"></a>
## Linux-CEF-Skripte

- `prepare-linux-cef-sdk.mjs`: lädt das festgelegte offizielle Linux-x64-CEF-SDK herunter und entpackt es, das zum Build der Linux-CEF-Brücke verwendet wird.
- `prepare-linux-cef-runtime.mjs`: stellt das separate Linux-CEF-Laufzeit-ZIP bereit, validiert erforderliche Dateien, entfernt unter Linux Debug-Symbole aus bereitgestellten ELF-Binärdateien und kann für die Release-Paketierung das generierte Manifest `local/webview-runtimes/linux-cef.json` schreiben.
- `check-linux-cef-runtime-release.mjs`: prüft, ob Release-Assets das von dem aktivierten Manifest benannte CEF-Laufzeit-ZIP enthalten und ob sein SHA-256 übereinstimmt.
- `cef/linux/fennara_cef_helper.cpp`: Quellcode des kleinen CEF-Hilfsprozesses, der beim Build des Laufzeithelfers aus dem CEF-SDK verwendet wird.

Die CEF-Skripte arbeiten nur mit kopierten Stagingdateien. Sie dürfen den heruntergeladenen Quellbaum des CEF-SDK nicht verändern.

<a id="development-tests"></a>
## Entwicklungstests

- `test-run-scene-edit-script-inspect.mjs`: erstellt ein ignoriertes Godot-Smoke-Projekt unter `temp/` und prüft die Untersuchung importierter `PackedScene`-Ressourcen, Schutzmechanismen für schreibgeschützten Kontext, Fehler bei fehlender Quelle und Verhalten ohne Speichern mit einer gebauten Editor-GDExtension.

<a id="documentation-localization"></a>
## Dokumentationslokalisierung

- `sync-doc-navigation.mjs`: fügt Quellenhashes, stabile Anker und den kompakten Sprachwähler für dieselbe Seite hinzu, ohne Prosa zu übersetzen.
- `check-doc-i18n.mjs`: prüft vollständige Sprachabdeckung, Aktualität der Quelle, Navigation, Anker, Markdown-Struktur, geschützten Code, URLs und Links.
- `doc-i18n-lib.mjs`: verwaltet das gemeinsame Sprachmanifest, die Normalisierung der Quellen, das Rendering der Navigation und Strukturhilfen.

Ausführen:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Die Sprachen und der Dokumentensatz sind in `docs/i18n/languages.json` deklariert. Englisch
bleibt kanonisch. Übersetzte Prosa muss anhand der englischen Quelle verfasst werden und darf
nicht von diesen Skripten erzeugt werden.

Die normale Synchronisierung aktualisiert die Navigation und stabile Anker,
behält vorhandene Quellenhashes jedoch bei. Aktualisiere nach der direkten
Überarbeitung aller neun Übersetzungen einer geänderten englischen Seite
bewusst nur diese Quelle:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

Die Option kann für mehrere geprüfte Quellen wiederholt werden. Bestätige keine
Quelle, deren übersetzte Prosa noch nicht aktualisiert wurde. CI führt
`sync-doc-navigation.mjs --check` vor dem vollständigen Übersetzungsvalidator
aus.

<a id="ui-sync"></a>
## UI-Synchronisierung

- `sync-chat-ui.mjs`: kopiert `ui/chat/` nach `godot_demo/addons/fennara/dist/`.

`godot_demo/addons/fennara/dist/` wird absichtlich committet, weil veröffentlichte Addon-ZIPs das gebaute Chat-Webview enthalten müssen. Nimm Änderungen in `ui/chat/` vor, führe das Synchronisierungsskript aus und committe Quellcode und generierte Addon-Assets gemeinsam.

<a id="runtime-sync"></a>
## Laufzeit-Synchronisierung

- `sync-runtime.mjs`: kopiert `runtime/` nach `godot_demo/addons/fennara/runtime/`.

`godot_demo/addons/fennara/runtime/` wird absichtlich committet, weil veröffentlichte Addon-ZIPs die Godot-seitigen Laufzeit-Hilfsskripte enthalten müssen. Nimm Änderungen in `runtime/` vor, führe das Synchronisierungsskript aus und committe Quellcode und generierte Addon-Assets gemeinsam.

<a id="guidance-sync"></a>
## Synchronisierung der Anweisungen

- `sync-guidance.mjs`: kopiert die kompakten Richtlinien und bei Bedarf geladenen Wissensseiten aus `local/templates/` nach `godot_demo/addons/fennara/ai/`, entsprechend den Dateien, die `fennara install` und `fennara update` in Benutzerprojekte schreiben.

`godot_demo/addons/fennara/ai/` wird absichtlich committet, weil das Demo-Addon das Layout eines installierten Addons nachbildet. Nimm Änderungen in `local/templates/` vor, führe das Synchronisierungsskript aus und committe Quellcode und generierte Addon-Anweisungen gemeinsam.

<a id="boundaries"></a>
## Grenzen

- Skripte dürfen `.package-preview/` und Ausgaben im Root-Ordner `dist/` erstellen.
- Skripte dürfen committete generierte Payloads nur dann aktualisieren, wenn dies ihre ausdrückliche Aufgabe ist, etwa bei `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs` oder `set-version.mjs`.
- Skripte dürfen Godot-Editor-Caches, lokale Installationen in App-Daten, heruntergeladene Release-Artefakte oder VM-Testausgaben nicht in nachverfolgte Quellordner schreiben.
