<!-- fennara-i18n: locale=de source=docs/repo-map.md sha256=a6f2ff719d03e067423cd68229e6941f701a52bfc4ed84361327716d5d9e9c0d -->
<a id="repo-map"></a>
# Repositorysübersicht

<!-- fennara-doc-nav:start -->
[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · **Deutsch** · [Türkçe](../tr/repo-map.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../repo-map.md)
<!-- fennara-doc-nav:end -->

Dies ist die Kurzübersicht für Mitwirkende und Coding-Agenten, die in diesem Repository arbeiten.

<a id="find-the-right-area"></a>
## Den richtigen Bereich finden

| Änderung | Primärer Ort |
| --- | --- |
| Einrichtung für Benutzer oder CLI-Verhalten | `local/crates/fennara-cli/` |
| Externes MCP-Protokoll oder Schemas | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Integrierter Chat oder Daemon-Verhalten | `local/crates/fennara-daemon/` |
| Integration in den Godot-Editor | `fennara-cpp/` |
| Chat-Benutzeroberfläche | `ui/chat/` |
| Laufzeit-Hilfsskripte | `runtime/` |
| Paketierung oder Releases | `scripts/`, `.github/workflows/` |
| Benutzerdokumentation | `README.md`, `docs/` |

<a id="top-level"></a>
## Oberste Ebene

| Pfad | Zuständigkeit |
| --- | --- |
| `.github/` | Pull-Request-Vorlage, Issue-Vorlagen und GitHub-Actions-Workflows. |
| `docs/` | Projektdokumentation, Einrichtungsanleitungen, Architekturnotizen, Beispiele, Demos und Release-Notizen. |
| `docs/i18n/` | Sprachmanifest und vollständige übersetzte Dokumentationsbäume. |
| `fennara-cpp/` | C++-Quellcode der Godot-GDExtension und Einstiegspunkt für den SCons-Build. |
| `godot_demo/addons/fennara/` | Installierbare Nutzlast des Godot-Addons, die in Benutzerprojekte kopiert wird. |
| `local/` | Rust-CLI, MCP-Server, Daemon, Schemas und lokaler Laufzeitcode. |
| `media/` | Bilder und öffentliche Medien, die von der Dokumentation verwendet werden. |
| `runtime/` | Quellcode der Godot-Laufzeit-Hilfsskripte, die von `runtime_session` und `runtime_script` verwendet werden. |
| `scripts/` | Hilfsskripte für Versionierung, Paketierung und Releases. |
| `ui/chat/` | Quellcode für die optionale Web-Chat-Benutzeroberfläche im Editor. |
| `local/templates/` | Kompakte Projektrichtlinien und bei Bedarf bereitgestellte KI-Wissensseiten, die von `fennara install` in Godot-Projekte geschrieben und von `fennara update` aktualisiert werden. |
| `local/webview-runtimes/` | Manifest- und Konfigurationsdateien für externe Webview-Laufzeiten, die in den gemeinsam genutzten Fennara-Anwendungsdaten installiert werden, beispielsweise die Linux-CEF-Nutzlast. |
| `install.ps1` / `install.sh` | Bootstrap-Skripte, die die Fennara-CLI aus GitHub-Releases installieren. |
| `VERSION` | Maßgebliche Quelle für die Version. |
| `README.md` | Kurze, an Menschen gerichtete Übersicht und Schnelleinstieg. |
| `docs/README.md` | Aufgabenorientierter Dokumentationsindex. |
| `docs/setup.md` | An Benutzer gerichtete, vom Addon ausgehende Einrichtung, Chat-Voraussetzungen, MCP-Verbindung, Aktualisierungsablauf und Fehlerbehebung. |
| `docs/cli.md` | Referenz für Terminalbefehle, CLI-eigenes Installations- und Aktualisierungsverhalten, Wiederherstellung, Diagnose, Layout der Anwendungsdaten und Hinweise zur Automatisierung. |
| `docs/telemetry.md` | Anonyme Aktivitätsnutzlast, Status der Anwendungsdaten, Übermittlungsverhalten, Definition der monatlich aktiven Installationen und Deaktivierungsoptionen. |
| `CONTRIBUTING.md` | Regeln für Beiträge. |
| `SECURITY.md` | Richtlinie zur Meldung von Sicherheitsproblemen. |
| `LICENSE.md` | Projektlizenz. |

<a id="local-rust-packages"></a>
## Lokale Rust-Pakete

| Pfad | Zuständigkeit |
| --- | --- |
| `local/crates/fennara-cli/` | Befehl `fennara`: Installation, Aktualisierung, Selbstaktualisierung der CLI, Doctor, Betriebsdiagnose, Prüfung der Webview-Voraussetzungen, C#-Unterstützung, Einrichtung von MCP-Anwendungen und generierte Projektrichtlinien. |
| `local/crates/fennara-cli/src/operation.rs` | Öffentlicher Koordinator für Installations- und Aktualisierungsvorgänge, Phasen und Einstiegspunkte für die CLI-Übergabe. |
| `local/crates/fennara-cli/src/operation/` | Fokussierte Module für Betriebsjournal, dauerhafte Speicherung, Schwärzung von Diagnosedaten und Tests. |
| `local/crates/fennara-cli/src/project_addon.rs` | Validierung der Version eines vorhandenen Projekt-Addons und der GDExtension-Bibliothek für die aktuelle Plattform. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Vorbereitung addonfreier CI-Exporte, die vor dem Start von Godot ausschließlich Fennaras dauerhaften Laufzeit-Autoload entfernt. |
| `local/crates/fennara-cli/src/release_identity.rs` | Stabile/Staging-Addon-Identität, exakte Release-Selektoren, Validierung von Pull-Request-Kanälen und Kompatibilität mit älteren stabilen Versionen. |
| `local/crates/fennara-cli/src/release_channel.rs` | Validierung von Staging-Zeigern pro Kanal und Auflösung zu einem exakt versionierten Release. |
| `local/crates/fennara-cli/src/release_manifest.rs` | Parsen des Release-Manifests, Validierung von Asset-Hashes, Bindung der Identität und Auswahl des Plattformpakets. |
| `local/crates/fennara-cli/src/release_version.rs` | Gemeinsames Parsen und gemeinsame Rangfolge der CLI-SemVer, die von Manifesten und der Release-Auswahl verwendet werden. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Übernahme eines vorhandenen vollständigen Addons mit exakter Version, ohne Dateien des Projekt-Addons zu ersetzen. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Gemeinsame Zustandsprüfung des Daemons, Bereitschaft für eine exakte Version und Start, die von Installation und Doctor verwendet werden. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Tests für Fehler auf Prozessebene, dauerhafte Diagnosen, Schwärzung und fehlersicher geschlossene Betriebsprotokolle. |
| `local/crates/fennara-cli/src/diagnostics.rs` | Benutzerzugriff auf den neuesten oder einen benannten bereinigten Betriebsbericht. |
| `local/crates/fennara-mcp/` | Lokaler stdio-MCP-Server und Weiterleitung von Werkzeugschemas. |
| `local/crates/fennara-daemon/` | Lokaler Daemon für Laufzeitsitzungen und Godot-Bridge-Aufgaben. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | Planer für anonyme täglich aktive Installationen, begrenzte Warteschlange, HTTP-Übermittlung und Integration in den Daemon-Lebenszyklus. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | Validierung einer zufälligen Installationsidentität, atomare Speicherung in den Anwendungsdaten, täglicher Bestätigungsstatus und Bereinigung bei Deaktivierung. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Genehmigungsmodi des integrierten Chats, Klassifizierung des Werkzeugrisikos, Berechtigungsentscheidungen und Typen ausstehender Genehmigungsanfragen. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Daemon-eigene Implementierung von `exec_command` für den integrierten Chat: Shell-Erkennung, cwd-Validierung, Prozessstart, Zeitüberschreitung/Beenden des Prozessbaums, Ausgabeerfassung, Protokollierung des Ergebnisartefakts und Ergebnisformatierung. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Planer für die Kontextkomprimierung des integrierten Chats: exakter Schutz des Endes, OpenCode-ähnliches druckabhängiges Bereinigen alter Werkzeugergebnisse, Auswahl/Speicherung/Wiedergabe von Zusammenfassungsabschnitten, Serialisierung des Zusammenfassungs-Prompts, Token-Budgets und Darstellung von Platzhaltern. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | PromptBuilder des integrierten Chats und generierter Kontext der Laufzeitumgebung. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Rein lokaler Trace-Recorder des integrierten Chats, SQLite-Ereigniszeilen, Aufbewahrung und Hilfsfunktionen für Debug-Abfragen. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | Laufzeitprimitive, Katalog/Auflösung, Hooks für Kontext-Vorprüfungen, normalisierte Stream-/Fehlertypen und OpenAI-kompatible oder Anthropic-kompatible Adapter des integrierten Chats für OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, benutzerdefinierte Endpunkte, Ollama/lokal und LM Studio. |
| `local/schemas/tools/` | Gemeinsame JSON-Schemas der Werkzeuge. Der externe MCP-Server und der integrierte Chat betten jeweils ihre eigenen erlaubten Teilmengen ein. |
| `local/webview-runtimes/linux-cef.json` | Platzhalter-/generiertes Manifest der Linux-CEF-Laufzeit, das für die Erzeugung des Release-Manifests, die Doctor-Ausgabe und den Legacy-Fallback verwendet wird. Es zeichnet das gemeinsame Layout der Anwendungsdaten und die Archivmetadaten auf, ohne CEF in der Addon-ZIP-Datei abzulegen. |
| `local/Cargo.toml` | Konfiguration des Rust-Workspace. |
| `local/Cargo.lock` | Gesperrter Rust-Abhängigkeitsgraph. |

<a id="gdextension-source"></a>
## GDExtension-Quellcode

| Pfad | Zuständigkeit |
| --- | --- |
| `fennara-cpp/SConstruct` | Einstiegspunkt für den GDExtension-Build. |
| `fennara-cpp/include/` | Öffentliche C++-Header. |
| `fennara-cpp/src/` | C++-Implementierung. |
| `fennara-cpp/src/setup/` | Nativer Status der Ersteinrichtung, CLI-Bootstrap über das Release-Manifest, Hash-Prüfung, CLI-Start und Leser des dauerhaften Betriebsfortschritts. |
| `fennara-cpp/src/release/version.cpp` | Native SemVer-Validierung und Rangfolge, die von der Release-/Aktualisierungserkennung verwendet werden. |
| `fennara-cpp/src/release/identity.cpp` | Validierung der paketierten stabilen/Staging-Identität und Kompatibilität mit älteren stabilen Versionen. |
| `fennara-cpp/src/release/discovery.cpp` | GitHub-Latest-Erkennung und isolierte Aktualisierungserkennung für Staging-Kanäle. |
| `fennara-cpp/src/update/` | Koordination von Aktualisierungen auf ein exaktes Ziel, Erkennung dauerhafter Bestätigungen, Übergabe zum Schließen/Installieren und Status der Wiederherstellungsoberfläche. |
| `fennara-cpp/src/ui/setup_panel.cpp` | Webview-unabhängiges Panel für die Ersteinrichtung mit Fortschritt, Wiederholung, Protokollen und Aktionen für bereinigte Berichte. |
| `fennara-cpp/vendor/cef/` | Offizieller Snapshot der CEF-139-Header, die von der Linux-OSR-Bridge verwendet werden. Laufzeit-Binärdateien verbleiben außerhalb des Addons. |
| `fennara-cpp/src/ui/webview_host*` | Nativer Host der Chat-Webview im Editor und Plattform-Backends. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Nur für Linux bestimmte Erkennung der gemeinsam genutzten CEF-Laufzeit, Validierung der Markierung und Grundlage des dynamischen `libcef.so`-Loaders. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Nur für Linux bestimmte CEF-Oberfläche für Off-Screen-Rendering, Weiterleitung von Godot-Eingaben, Laden der Bridge-ABI und Aktualisierung von Godot-Texturen für die interne Chat-Webview. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Kleine, nur für Linux bestimmte Bridge-Bibliothek, die aus dem angehefteten offiziellen CEF-139-Quellcode von `libcef_dll_wrapper` und Fennaras CEF-OSR-Adapter gebaut wird. Die Haupt-GDExtension lädt sie per dlopen, nachdem die externe Laufzeit `libcef.so` geladen wurde. |
| `fennara-cpp/src/tools/` | Godot-seitige Werkzeugimplementierungen. |
| `fennara-cpp/src/lsp/` | Skriptdiagnose und Hilfsfunktionen für Sprachserver. |
| `fennara-cpp/src/csharp/` | Reine Build-Unterstützung für C#-Projektauswahl, Vorbereitung im Hintergrund, isolierte Diagnose und Laufzeit-Vorprüfung. |
| `fennara-cpp/src/runtime/` | Native Laufzeitunterstützung, die von Werkzeugen verwendet wird, einschließlich Vorprüfung von Laufzeitszenen, Skriptdiagnose und Debugger-Snapshots. |
| `fennara-cpp/godot-cpp/` | Submodul mit Godot-C++-Bindings. |

<a id="addon-payload"></a>
## Addon-Nutzlast

| Pfad | Zuständigkeit |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Registrierungsdatei der Godot-GDExtension. |
| `godot_demo/addons/fennara/VERSION` | Version des Addon-Pakets. |
| `godot_demo/addons/fennara/release.json` | Paketierte stabile oder Staging-Identität, einschließlich exakter Version, Release-Tag, Kanal und Quell-Commit des Stagings. |
| `godot_demo/addons/fennara/bin/` | Gebaute Plattformbibliotheken. |
| `godot_demo/addons/fennara/dist/` | Paketierte Web-UI-Assets, die von der Chat-Webview im Editor verwendet werden. |
| `godot_demo/addons/fennara/runtime/` | Synchronisierte, paketierte Kopie von `runtime/`, die im Addon ausgeliefert wird. |
| `godot_demo/tests/first_run_setup_test.gd` | Headless-Test für den nativen Status der Ersteinrichtung und deterministische Fehler. |
| `godot_demo/tests/export_plugin_test.gd` | Nativer Headless-Regressionstest für den Exportausschluss und die Wiederherstellung des Autoloads. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | Headless-Regressionstest für den nativen Argumentvertrag von Screenshots. |
| `godot_demo/tests/image_sheet_test.gd` | Headless-Regressionstest für die gemeinsame Zusammensetzung von Screenshot-/Laufzeitbilderbögen. |
| `godot_demo/tests/runtime_image_context_test.gd` | Headless-Regressionstest für rohe Laufzeit-Frames, Bögen und beliebige Image-Ausgaben. |

<a id="runtime-helper-source"></a>
## Quellcode der Laufzeit-Hilfsfunktionen

| Pfad | Zuständigkeit |
| --- | --- |
| `runtime/game_capture_helper.gd` | Einstiegspunkt der Laufzeit-Hilfsfunktion, der von der GDExtension für Szenensitzungen und Laufzeitprüfungen geladen wird. |
| `runtime/image_label.gd` | Kompakte deterministische Beschriftungen, die nach der Erfassung auf zusammengesetzte Image-Zellen gestempelt werden. |
| `runtime/image_sheet.gd` | Gemeinsame, ausschließlich mit Image arbeitende Bogenzusammensetzung, die in Screenshot- und Laufzeitskript-Kontexten verwendet wird. |
| `runtime/screenshot_script_context.gd` | Öffentliche Fassade für Screenshot-Skripte, die dem nativen Erfassungskontext gemeinsame Image-Zusammensetzung hinzufügt. |
| `runtime/runtime_script_context.gd` | Öffentliche Oberfläche der für `runtime_script` bereitgestellten `ctx`-Hilfsfunktionen, einschließlich roher Frames, Image-Zusammensetzung/-Ausgabe, Warteoperationen, Eingaben, Snapshots, Bedingungen, Raycasts und Klicks. |
| `runtime/runtime_input_driver.gd` | Treiber für niedrigstufige Laufzeiteingaben für Tasten, Maustasten, absolute Mausbewegung, relative Mausbewegung, Zusatztasten und Eingabebereinigung. |
| `runtime/runtime_node_snapshot.gd` | Suche von Laufzeitknoten, Existenzprüfungen, gegen veraltete Referenzen sichere Snapshots, Eigenschaftslesevorgänge und Zusammenfassungen untergeordneter Knoten. |
| `runtime/runtime_physics_query.gd` | Hilfsfunktionen für exakte 2D-/3D-Raycast- und Scan-Abfragen zur Laufzeit mit kompakten Trefferbestätigungen. |
| `runtime/runtime_query_utils.gd` | Gemeinsame Hilfsfunktionen für Laufzeitabfragen zur Vektorkonvertierung, sicheren Auflösung von Knoten/Pfaden, Objektidentität und allgemeinen Zielübereinstimmung. |
| `runtime/runtime_capture_store.gd` | Writer für Laufzeit-Erfassungs-/Statusartefakte, der von Laufzeitsitzungen, Skripten und Umgebungsprüfungen verwendet wird. |
| `runtime/runtime_check_runner.gd` | Runner für Laufzeitprüfungen mit Spezifikationen zur nicht interaktiven Szenenausführung. |

<a id="scripts-and-workflows"></a>
## Skripte und Workflows

| Pfad | Zuständigkeit |
| --- | --- |
| `scripts/set-version.mjs` | Aktualisiert versionierte Dateien im gesamten Repository. |
| `scripts/check-version.mjs` | Prüft die Versionssynchronisierung. |
| `scripts/release-identity.mjs` | Gemeinsame Node-Validierung und Erzeugung für die SemVer-Release-Identität sowie Staging-Zeiger pro PR. |
| `scripts/release-policy.mjs` | Richtlinie für die mindestens kompatible veröffentlichte CLI in stabilen und Staging-Release-Manifesten. |
| `scripts/staging-candidate.mjs` | Erzeugung vertrauenswürdiger Staging-Kandidatenidentitäten und monotone Entscheidungen für Zeiger pro PR. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | Fokussierte Validierung von Staging-Addon, Archiv, Manifest, gemeinsamem Dateisystem und Veröffentlichungsbündel. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | Strenge Validierungseinstiegspunkte für nicht vertrauenswürdige Build-Ausgaben und das vertrauenswürdige Veröffentlichungsbündel. |
| `scripts/check-staging-channel-advance.mjs` | Wendet Monotonie- und Herkunftsprüfungen an, bevor ein Zeiger des Staging-Kanals vorrückt. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | Prüft die Bytes veröffentlichter Assets und das öffentliche Downloadverhalten vor der Heraufstufung des Zeigers. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | Erstellt ein ignoriertes temporäres Godot-Projekt und führt einen Smoke-Test der schreibgeschützten Untersuchung importierter `PackedScene`-Ressourcen mit der Editor-GDExtension aus. |
| `scripts/release-targets.mjs` | Definiert unterstützte Plattformziele für Releases und die Namen ihrer paketierten Assets. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | Schreibt die eingefrorene Kandidatenidentität und den kleinen Kanalzeiger. |
| `scripts/sync-chat-ui.mjs` | Kopiert den ohne Build verwendbaren Quellcode der Chat-UI in die Addon-Nutzlast. |
| `scripts/sync-runtime.mjs` | Kopiert den Quellcode der Laufzeit-Hilfsfunktionen aus dem Repository-Stamm in die Addon-Nutzlast. |
| `scripts/sync-doc-navigation.mjs` | Fügt Dokumentationsnavigation, Quell-Hashes und stabile Anker hinzu, ohne Fließtext zu übersetzen. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | Validiert Übersetzungsabdeckung, Aktualität, Markdown-Struktur, URLs und Links. |
| `scripts/package-preview.mjs` | Stellt nach den Plattform-Builds Vorschau-/Release-ZIP-Dateien für Addon, CLI und lokale Laufzeit zusammen. |
| `scripts/prepare-linux-cef-runtime.mjs` | Stellt die separate ZIP-Datei der Linux-x64-CEF-Laufzeit bereit, entfernt Symbole aus den bereitgestellten ELF-Binärdateien, validiert erforderliche Dateien und kann das generierte Release-Manifest schreiben. |
| `scripts/prepare-linux-cef-sdk.mjs` | Lädt das angeheftete offizielle minimale CEF-139-Linux-SDK herunter und entpackt es für CI-Builds, die den Quellcode des Wrappers `libcef_dll/` benötigen. |
| `scripts/check-linux-cef-runtime-release.mjs` | Validiert das Release-Asset der Linux-CEF-Laufzeit gegen das generierte Manifest `local/webview-runtimes/linux-cef.json`. |
| `scripts/write-release-manifest.mjs` | Schreibt und validiert `fennara-release-manifest-v<version>.json` aus Release-Assets, einschließlich Hashes für lokales Paket, Addon und gemeinsam genutzte Laufzeit. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Quellcode des minimalen Linux-CEF-Unterprozesshelfers, der in der separaten CEF-Laufzeit-ZIP-Datei paketiert wird. |
| `.github/workflows/version-check.yml` | Prüfung der Versionskonsistenz. |
| `.github/workflows/gdextension-build.yml` | Plattformübergreifende Prüfung des GDExtension-Builds sowie nativer Windows-Headless-Test des Status der Ersteinrichtung. |
| `.github/workflows/local-build.yml` | Build-Prüfung des lokalen Rust-Pakets. |
| `.github/workflows/package-preview.yml` | Manuelle Pakete für Vorschauartefakte, einschließlich eines ausschließlich für Tests bestimmten Linux-CEF-Laufzeitartefakts für Linux-Chat-Smoke-Tests. |
| `.github/workflows/release.yml` | Manuelle Veröffentlichung von GitHub-Releases, einschließlich Erzeugung des Linux-CEF-Laufzeitpakets, Erzeugung des Release-Manifests und abschließender Asset-Validierung. |
| `.github/workflows/staging-release.yml` | Manueller Staging-Build für einen exakten SHA, rein validierender Probelauf, Veröffentlichung eines exakten Vorab-Releases und Vorrücken des Zeigers pro PR. |

<a id="where-to-change-things"></a>
## Wo Änderungen vorzunehmen sind

| Aufgabe | Hier beginnen |
| --- | --- |
| Ein Godot-Werkzeug hinzufügen oder ändern | `fennara-cpp/src/tools/` und `local/schemas/tools/` |
| MCP-Schematext ändern | `local/schemas/tools/` |
| `fennara install` oder `fennara update` ändern | `local/crates/fennara-cli/src/`; natives Staging sowie abgekoppeltes Anwenden/Rollback liegen in der Zuständigkeit von `release_update.rs`, `update_stage.rs`, `update_stage/` und `update_apply/` |
| CLI-Befehle oder Terminalverhalten ändern | `local/crates/fennara-cli/src/` und `docs/cli.md` |
| Nativen Aktualisierungsfortschritt, Bestätigung des Herunterfahrens, Aktivierungs-Handshake oder Wiederherstellung ändern | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` und `ui/chat/` |
| Native Ersteinrichtung oder CLI-Bootstrap ändern | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp` und `fennara-cpp/src/ui/dock.cpp` |
| Addon-Ausschluss zur Exportzeit ändern | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp` und `godot_demo/tests/export_plugin_test.gd` |
| Betriebsprotokolle, Phasen, Fehlercodes oder Diagnoseberichte für Installation/Aktualisierung ändern | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/` und `local/crates/fennara-cli/src/diagnostics.rs` |
| Prüfung der Webview-Voraussetzungen ändern | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs` und `fennara-cpp/src/ui/webview_host*` |
| Generierte Projektrichtlinien ändern | `local/templates/` und `local/crates/fennara-cli/src/project_guidance.rs` |
| Generierte Richtlinien des Demo-Addons synchronisieren | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs` und `godot_demo/addons/fennara/ai/` |
| Einrichtung einer MCP-Anwendung ändern | `local/crates/fennara-cli/src/mcp_setup.rs` und `docs/mcp-setup.md` |
| Prozess-/Protokollverhalten von Laufzeitsitzungen ändern | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/` und `fennara-cpp/src/tool_results/` |
| ctx-Hilfsfunktionen, Eingaben, Snapshots, Warteoperationen, Raycasts, Erfassungen oder Bereinigung von `runtime_script` ändern | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json` und `docs/tools.md` |
| Chat-Benutzeroberfläche im Editor, Slash-Befehle oder Modell-/Anbieterauswahl ändern | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp` und `fennara-cpp/src/ui/webview_host*` |
| Anbieter des integrierten Chats ändern | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` und `ui/chat/` |
| Felder, Planung oder Datenschutzoptionen der anonymen Telemetrie ändern | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` und `docs/telemetry.md` |
| Mitgelieferte Chat-UI-Bibliotheken ändern | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/` und `THIRD_PARTY_NOTICES.md` |
| C#-Unterstützung ändern | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/` sowie die C#-Werkzeugschemas und -Richtlinien |
| Release-Pakete, Mindest-CLI-Richtlinie oder CLI-Selbstaktualisierung ändern | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs` und `.github/workflows/release.yml` |
| Version erhöhen | `node scripts/set-version.mjs <version>` |
| Einrichtung/Dokumentation für Chat im Vergleich zu MCP, Anbieter oder Slash-Befehle aktualisieren | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md` und `llms.txt` |
| Dokumentationsübersetzungen aktualisieren | Kanonische englische Seite, `docs/i18n/languages.json`, die entsprechenden Sprachseiten, `scripts/sync-doc-navigation.mjs` und `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## Hinweise

- Halte diese Datei aktuell, wenn du größere Quellbereiche hinzufügst oder verschiebst.
- Die Release-Schritte gehören in [release.md](release.md).
- Die Einrichtungsschritte gehören in [setup.md](setup.md).
- Das Verhalten von Terminalbefehlen gehört in [cli.md](cli.md).
