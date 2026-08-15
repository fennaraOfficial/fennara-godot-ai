<!-- fennara-i18n: locale=de source=docs/tools.md sha256=ed8da3ac5003a9dafd4a90594f6f1b1e03c5987f73328fd0458a9ee6f18f3d7d -->
<a id="tools"></a>
# Werkzeuge

<!-- fennara-doc-nav:start -->
[English](../../tools.md) · [简体中文](../zh-CN/tools.md) · [Español](../es/tools.md) · [Português do Brasil](../pt-BR/tools.md) · [日本語](../ja/tools.md) · [한국어](../ko/tools.md) · [Русский](../ru/tools.md) · [Français](../fr/tools.md) · **Deutsch** · [Türkçe](../tr/tools.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../tools.md)
<!-- fennara-doc-nav:end -->

Fennara stellt Coding-Agenten Godot-bezogene Untersuchung, Bearbeitung, Validierung,
Screenshots und Laufzeit-Feedback bereit. Es ergänzt normale Repository- und
Shell-Werkzeuge, statt sie zu ersetzen.

Diese Seite erklärt, was jedes Werkzeug leisten kann, was ein erfolgreicher Aufruf
bedeutet und welche wichtigen Einschränkungen oder Fehlerfälle bestehen. Die aktuellen
Werkzeugschemas bleiben die maßgebliche Quelle für exakte Argumente, Ergebnisfelder,
Grenzwerte und Agentenanweisungen. Installierte Projekte erhalten außerdem kompakte
Richtlinien und bei Bedarf bereitgestelltes Wissen unter `addons/fennara/ai/`.

<a id="tool-surfaces"></a>
## Werkzeugoberflächen

Externe MCP-Clients, darunter Codex, Claude Code, Cursor und Gemini, verbinden sich über
den lokalen Prozess `fennara-mcp`. Sie verwenden ihr eigenes Modellkonto sowie ihre
normalen Datei-, Such-, Diff- und Shell-Werkzeuge zusammen mit Fennara.

Der integrierte Fennara-Chat verwendet denselben Daemon und dieselbe Godot-Bridge. Er kann
dieselben Godot-Werkzeuge aufrufen und stellt zusätzlich projektbezogene Werkzeuge
`read_file` und `exec_command` bereit. Die Einrichtung von Anbieter und Modell gehört
zum integrierten Chat, nicht zum MCP-Server.

`fennara_status` steht externen MCP-Clients zur Verfügung. Der integrierte Chat erhält
Verbindungs- und aktiven Projektstatus bereits vom Daemon.

Jeder externe MCP-Prozess wählt beim Start genau einen Routingmodus. Ein Prozess
im Modus `bound` leitet anhand des kanonischen Projektstamms weiter. Ein Prozess
im Modus `legacy_unbound` verwendet das im Dock ausgewählte Kompatibilitätsziel
und eignet sich nicht für isolierte gleichzeitige Arbeit.

<a id="typical-workflow"></a>
## Typischer Arbeitsablauf

1. Bestätige bei Verwendung eines externen MCP-Clients das verbundene Projekt.
2. Untersuche die relevante Szene, Ressource, Klasse, den Importstatus oder die Projekteinstellung.
3. Nimm die kleinste nützliche Änderung vor.
4. Führe Diagnosen oder eine Szenenvalidierung aus.
5. Verwende Screenshots oder Laufzeitwerkzeuge, wenn visuelle oder verhaltensbezogene Nachweise wichtig sind.

Das Dateisystem des Editors kann vorübergehend mit Prüfung oder Import beschäftigt sein.
Asset-Werkzeuge sollten erst verwendet werden, nachdem es Bereitschaft meldet.

<a id="connection"></a>
## Verbindung

<a id="fennarastatus"></a>
### `fennara_status`

Meldet MCP-Server, Daemon, Routingmodus, ausgewählten Godot-Editor, verbundene
Editor-Sitzungen, Komponentenversionen, Rendering-Kontext, angekündigte Werkzeuge
und Bereitschaft des Editor-Dateisystems.

Funktionsweise:

- Gibt einen einzelnen Klartext-Statusblock zurück.
- Meldet bei einem gebundenen Prozess die Bindungsquelle (`cli`, `environment` oder
  `cwd`), den kanonischen Projektstamm und den Editorzustand (`connected`,
  `not_connected` oder `ambiguous`).
- Meldet das im Dock ausgewählte Legacy-MCP-Ziel getrennt von einer Projektbindung.
- Unterscheidet ein bereites Editor-Dateisystem von einem, das gerade prüft oder importiert.
- Meldet, ob Asset-bezogene Werkzeuge derzeit bereit sind.
- Zeigt Versionsunterschiede, damit nicht übereinstimmende Installationen diagnostiziert werden können.

Wichtige Einschränkungen und Fehler:

- Es meldet Bereitschaft auf Projektebene, nicht die Bereitschaft eines bestimmten Asset-Pfads.
- Ein gebundener Stamm ohne passenden Editor meldet den wiederholbaren Fehler
  `bound_project_not_connected` und fällt niemals auf das Dock-Ziel zurück.
- Doppelte Editoren für denselben gebundenen Stamm melden
  `ambiguous_project_binding`; es wird kein Editor ausgewählt.
- Ein getrennter Daemon, ein fehlendes Kompatibilitätsziel oder ein getrenntes
  Godot-Plugin wird direkt gemeldet, statt als bereites Projekt behandelt zu werden.
- Eine fehlgeschlagene gebundene Zuordnung zeigt niemals die
  Dateisystembereitschaft oder einen scheinbaren Erfolg eines fremden Editors an.
- Der Legacy-unbound-Modus meldet eine Warnung zur gleichzeitigen Nutzung.
- Während Godot Dateien erneut importiert, kann sich die Bereitschaft kurzzeitig ändern.

<a id="inspection"></a>
## Untersuchung

<a id="getscenetree"></a>
### `get_scene_tree`

Lädt eine Szene über Godot und gibt ihre Knotenhierarchie, Knotenklassen, angefügten
Skripte und instanziierten Unterszenen zurück. Die zurückgegebenen Pfade können von
anderen Szenenwerkzeugen verwendet werden.

Funktionsweise:

- Liest erstellte Szenen, ohne sie neu zu schreiben.
- Macht Knoten- und Instanzstruktur vor einer Bearbeitung sichtbar.
- Konzentriert das Ergebnis auf die Hierarchie, statt jede Ressource zu erweitern.

Wichtige Einschränkungen und Fehler:

- Es ist kein vollständiger Bericht über 3D-Assets, Meshes, Materialien, Skelette oder Animationen.
- Eine Szene, die Godot nicht laden kann, gibt einen Fehler statt eines vermuteten Baums zurück.
- Umfangreiche Ressourcendetails gehören in eine gezielte Eigenschaften- oder Skriptuntersuchung.

<a id="getnodeproperties"></a>
### `get_node_properties`

Zeigt Eigenschaften, die bei ausgewählten Knoten von den Klassenvorgaben abweichen, und
erweitert nützliche Zusammenfassungen eingebetteter Ressourcen.

Funktionsweise:

- Unterstützt bis zu fünf Knotenziele in einem Aufruf.
- Liest exportierte GDScript-Eigenschaften und verfügbare C#-Skriptmetadaten.
- Fasst Ressourcen wie Animationen, Themes, Tile-Daten, Mesh-Bibliotheken,
  Sprite-Frames und Animationsgraphen zusammen, statt undurchsichtige Werte auszugeben.

Wichtige Einschränkungen und Fehler:

- Es ist auf Knoten ausgerichtet, kein vollständiges Ressourceninventar der Szene.
- Importierte Quell-Assets können weniger Informationen offenlegen als erstellte
  `.tscn`-Knoten. Verwende `run_asset_import_script`, wenn die generierte importierte
  Ressource direkt untersucht werden muss.
- Ungültige Knotenpfade werden gemeldet, statt unbemerkt ignoriert zu werden.

<a id="getclassinfo"></a>
### `get_class_info`

Gibt die tatsächliche API-Oberfläche einer Godot-Klasse zurück, einschließlich Vererbung,
Eigenschaften, Methoden, Signale, Enums, Konstanten und verfügbarer Dokumentation.

Funktionsweise:

- Laufzeitinformationen von ClassDB stammen aus dem verbundenen Godot-Editor.
- Integrierte Klassen verwenden offizielle Godot-XML-Dokumentation, die zur verbundenen
  Haupt- und Nebenversion passt, mit einem ausdrücklichen Fallback auf `master`.
- GDExtension- und native Addon-Klassen geben ihre verfügbaren Laufzeitklassen- und
  Eigenschaftsinformationen zurück, ohne vorzugeben, offizielle Godot-Dokumentation zu besitzen.

Wichtige Einschränkungen und Fehler:

- Die Dokumentationssuche kann unvollständig sein, wenn die passende vorgelagerte
  Klassen-XML nicht verfügbar ist oder eine Antwort nicht vollständig empfangen werden kann.
- Reines Laufzeitverhalten kann weiterhin eine kleine Skriptsonde auf Editor-Seite erfordern.
- Ein nicht vorhandener Klassenname wird als fehlend gemeldet.

<a id="editing"></a>
## Bearbeitung

<a id="writeorupdatefile"></a>
### `write_or_update_file`

Erstellt oder überschreibt eine Projekttextdatei oder führt darin einen exakten Austausch aus.

Funktionsweise:

- `write` erstellt oder ersetzt eine Datei anhand ihres vollständigen Inhalts.
- `update` ersetzt einen einzigen eindeutigen exakten Textblock.
- Änderungen an GDScript und Shadern geben automatisch Godot-Diagnosen zurück.
- Shader-Änderungen versuchen außerdem, referenzierende Szenen und Ressourcen über Godot
  erneut zu serialisieren, damit eingebettete Materialdaten nicht veraltet bleiben.
- C#-Schreibvorgänge dürfen eine Bearbeitung über mehrere Dateien bilden, bevor ein
  Diagnose-Build des Projekts angefordert wird.

Wichtige Einschränkungen und Fehler:

- Mehrdeutiger oder fehlender Aktualisierungstext führt zu einem Fehler, statt einen beliebigen Treffer zu ändern.
- Geschützte Pfade von Fennara, Git, Godot-Cache, Plugin-Manifest und Projekteinstellungen können mit diesem Werkzeug nicht bearbeitet werden.
- Es ist nicht für direkte Eingriffe in `.tscn`, `.tres` oder `.res` vorgesehen.
- C#-Validierung wird nicht nach jedem einzelnen Schreibvorgang ausgeführt. Verwende nach
  Abschluss der zusammengehörigen C#-Änderungen eine Diagnoseprüfung des Projekts.
- Besitzer referenzierender Shader, die nicht sicher erneut serialisiert werden können, werden als übersprungen oder mit Warnung gemeldet.

<a id="runsceneeditscript"></a>
### `run_scene_edit_script`

Führt einen GDScript-Worker zur Editorzeit gegen eine erstellte Szene oder einen
Godot-Ressourcengraphen aus. Dies ist der strukturierte Weg, Szenen über Godots Objektmodell
und Serializer zu untersuchen oder zu bearbeiten.

Funktionsweise:

- Der Untersuchungsmodus lädt einen abgekoppelten schreibgeschützten Szenengraphen und speichert ihn niemals.
- Der Bearbeitungsmodus kann Knoten hinzufügen, entfernen, umbenennen oder neu einordnen, Ressourcen zuweisen, Eigenschaften ändern, Szenen erstellen und über Godots Serialisierung speichern.
- Bestehende Szenen werden nur gespeichert, wenn der Worker den Kontext als geändert markiert.
- Neue Knoten und PackedScene-Instanzen verwenden ausdrückliche Eigentums-Hilfsfunktionen, damit Godot die beabsichtigte Struktur serialisiert.
- Skriptdiagnosen werden vor der Ausführung durchgeführt, und gespeicherte Szenen erhalten eine nachgelagerte Validierung.
- Vererbte Szenenwurzeln bleiben erhalten, wenn Godot die angeforderten Überschreibungen sicher serialisieren kann.
- Jeder Aufruf gibt den tatsächlich verwendeten temporären Worker-Pfad zurück, sodass ein fehlgeschlagener Worker korrigiert werden kann, ohne ihn von Grund auf neu zu erstellen.

Wichtige Einschränkungen und Fehler:

- Der geladene Graph entspricht nicht dem Drücken von Run Scene. Von SceneTree abhängige
  Gameplay-APIs, Timer, Frame-Verarbeitung und globale Transformationen können sich bei
  abgekoppelten Knoten anders verhalten oder fehlschlagen.
- Der Untersuchungsmodus blockiert Änderungen über Fennara-Kontexthilfen, aber beliebiges
  GDScript muss weiterhin direkte Nebenwirkungen auf Dateisystem, Editor, Betriebssystem
  und Ressourcenspeicherung vermeiden.
- Importierte Quelldateien wie `.glb` und `.gltf` werden von diesem Werkzeug nicht
  gespeichert. Importeinstellungen gehören zu `run_asset_import_script`.
- Falsche Eigentumszuweisung für PackedScene-Interna wird abgelehnt, da sie Instanzinhalte abflachen oder duplizieren kann.
- Falls ein Speichervorgang eine vererbte Wurzel abflachen würde, stellt Fennara die ursprüngliche Datei wieder her und meldet einen Fehler.
- Diagnose- oder Laufzeitfehler beenden die Bearbeitung. Ein fehlgeschlagenes Ergebnis
  erstellt oder aktualisiert die Zielszene nicht, obwohl das temporäre Worker-Skript für
  einen erneuten Versuch bestehen bleiben kann.

<a id="runassetimportscript"></a>
### `run_asset_import_script`

Führt einen begrenzten GDScript-Worker zur Editorzeit gegen ein importiertes Quell-Asset
und dessen Godot-Importkonfiguration aus. Es unterstützt Modelle, Texturen, Audio, Schriften
und andere Formate, die bereits eine passende `.import`-Sidecar-Datei besitzen.

Funktionsweise im Untersuchungsmodus:

- Meldet Importer, Klasse der generierten Ressource, Gültigkeit des Imports, typisierte aktuelle Optionen, generierte Dateien und vorgelagerte Abhängigkeiten.
- Lädt die generierte Ressource, ohne veraltete verschachtelte Cache-Einträge wiederzuverwenden.
- Kann eine importierte PackedScene vorübergehend innerhalb des aktiven Editor-SceneTree für eine begrenzte Untersuchung instanziieren und entfernt sie anschließend, ohne sie zu speichern.
- Stellt begrenzte Zusammenfassungen für generierte Unterressourcen bereit.
- Behält Änderungen an Importoptionen im Untersuchungsmodus niemals dauerhaft bei.

Funktionsweise im Bearbeitungsmodus:

- Stellt unterstützte vorhandene Importoptionen bereit und behält dabei ihre nativen Godot-Variant-Typen bei.
- Lässt den aktiven Editor den erneuten Import über `EditorFileSystem` ausführen.
- Meldet Erfolg erst, nachdem die kanonischen Importeinstellungen, generierten Ausgaben,
  der Status des Editor-Dateisystems und ein frisches tiefes Laden der Ressource die Prüfung bestanden haben.
- Versucht bei fehlgeschlagener Prüfung, die vorherige Konfiguration wiederherzustellen und
  erneut zu importieren, und meldet, ob diese Wiederherstellung erfolgreich war.

Wichtige Einschränkungen und Fehler:

- Die Quelldatei muss bereits importiert sein und eine gültige `.import`-Sidecar-Datei besitzen.
- Bearbeitungen der ersten Version sind auf Optionen beschränkt, die bei unterstützten
  integrierten Textur- und Szenenimportern als sichere Änderungen des generierten Caches eingestuft sind.
- Importer-Identität, Importskripte, `_subresources`, externe Extraktionspfade und Optionen mit unbekannten Auswirkungen bleiben schreibgeschützt.
- Unbekannte Optionen, nicht unterstützte Optionen und Werte mit falschem Variant-Typ führen zu einem Fehler, statt konvertiert zu werden.
- Direkte Änderungen der `.import`-Datei werden erkannt, rückgängig gemacht und als Fehler gemeldet. Fennara besitzt die Sidecar-Persistenz.
- Importierte Szenen, die mit einem Wurzelskript konfiguriert sind, werden von der Untersuchungshilfe nicht vorübergehend instanziiert.
- Abhängigkeiten beschreiben Dateien, die zum Import des ausgewählten Assets erforderlich
  sind. Sie nennen keine nachgelagerten Projektverbraucher wie Szenen, die ein Modell
  verwenden, Materialien, die eine Textur verwenden, Skripte, die Audio abspielen, oder Themes, die eine Schrift verwenden.
- Skriptdiagnosen, Laufzeitfehler, Fehler beim erneuten Import, fehlende generierte Dateien,
  ungültiger Dateisystemstatus oder Fehler beim Neuladen verhindern ein erfolgreiches Ergebnis.
- Große Arrays und Ressourceninterna werden begrenzt oder zusammengefasst, um die
  Werkzeugausgabe zu schützen. Ein begrenztes Ergebnis verspricht nicht, dass jeder Vertex,
  Key oder jede Abhängigkeit direkt ausgegeben wurde.

<a id="projectsettings"></a>
### `project_settings`

Liest und ändert strukturierte Godot-Projekteinstellungen, Autoloads,
Anwendungsmetadaten, Rendering- und Anzeigeeinstellungen sowie Eingabeaktionen.

Funktionsweise:

- Verwendet Godot-bezogene strukturierte Operationen statt direkter Textersetzung in `project.godot`.
- Listet Eingabeaktionen mit Deadzones, Ereignisanzahlen und lesbaren Ereigniszusammenfassungen auf.
- Unterstützt beim Hinzufügen oder Aktualisieren von Steuerungen strukturierte Eingabeereignisse.

Wichtige Einschränkungen und Fehler:

- Unbekannte Operationen oder ungültige Einstellungswerte werden gemeldet.
- Dieses Werkzeug ersetzt nicht die Bearbeitung von Szenen oder Skripten.
- Änderungen sollten weiterhin validiert werden, wenn sie Start, Rendering, Eingabe oder Addon-Verhalten beeinflussen.

<a id="checks"></a>
## Prüfungen

<a id="scriptdiagnostics"></a>
### `script_diagnostics`

Führt Godot-bezogene Diagnosen für Skripte und Shader aus.

Funktionsweise:

- Gezielte GDScript- und Shader-Aufrufe unterstützen bis zu fünf Dateien.
- GDScript-Diagnosen stammen von Godots Sprachserver.
- Shader-Diagnosen stammen von Godots Shader-Parser.
- Gezielte GDScript-Prüfungen laden außerdem relevante Szenen im Speicher, damit durch
  Szenenanhänge verursachte Fehler dem Skript und der Szene zugeordnet werden können.
- Projektprüfungen kontrollieren GDScript und Shader und führen danach einen isolierten
  inkrementellen C#-Build aus, wenn ein C#-Projekt vorhanden ist.
- Diagnose-C#-Assemblies werden von den normalen Laufzeit-Assemblies des Editors getrennt gehalten.

Wichtige Einschränkungen und Fehler:

- Gezielte Diagnosen einzelner C#-Dateien werden nicht unterstützt. C# verwendet eine Projektprüfung.
- Projektweite Prüfungen überspringen die Instanziierung einzelner Szenen und können Probleme
  übersehen, die nur auftreten, wenn ein Skript über eine bestimmte Szene geladen wird.
- Fehler von Sprachserver, Parser oder Build werden als Diagnosefehler zurückgegeben und nicht als fehlerfreie Ergebnisse behandelt.
- Diagnosen belegen, dass der geprüfte Code im getesteten Kontext geparst oder kompiliert werden kann. Sie belegen nicht die Korrektheit des Gameplays.

<a id="validatescene"></a>
### `validate_scene`

Prüft eine oder mehrere Szenen auf strukturelle Probleme und führt, wo unterstützt, einen
kurzen Headless-Startdurchlauf aus.

Funktionsweise:

- Akzeptiert bis zu zehn Szenenpfade.
- Strukturprüfungen decken fehlende Skripte und Ressourcen, ungültige Knotenpfade,
  doppelte Namen gleichgeordneter Knoten, zyklische Szenenabhängigkeiten und relevante
  exportierte Referenzen ab.
- Optionale oder zur Laufzeit zugewiesene exportierte Referenzen werden als Hinweise und nicht als bedingungslose Fehler gemeldet.
- Erstellte Szenen mit fehlerfreien Strukturergebnissen erhalten einen dreisekündigen Headless-Startdurchlauf, dessen Protokolle und Artefakte aufbewahrt werden.
- Diese begrenzten Validierungs-Worker belegen den interaktiven Laufzeit-Slot nicht;
  die Validierung kann fortfahren, während ein anderes Projekt eine Laufzeitsitzung besitzt.
- Wiederholte Befunde werden gruppiert, damit große Szenen das Ergebnis nicht überfluten.

Wichtige Einschränkungen und Fehler:

- Importierte Quellszenen erhalten ausschließlich strukturelle Validierung, da sie nicht direkt als erstellte Projektszenen gestartet werden können.
- Fennara beendet den Prozess absichtlich nach dem Validierungsfenster. Dieser Stop-Code allein wird nicht als Szenenfehler behandelt.
- Ein kurzer Startdurchlauf kann nicht alle Gameplay-Pfade, visuellen Aspekte, Leistung, Animationsqualität oder Benutzerinteraktionen validieren.
- Strukturfehler verhindern für die betreffende Szene den Laufzeitdurchlauf.

<a id="visual-and-runtime-feedback"></a>
## Visuelles und Laufzeit-Feedback

<a id="screenshotscene"></a>
### `screenshot_scene`

Erfasst visuelle Nachweise aus erstellten Szenen und unterstützten importierten 3D-Assets.

Funktionsweise:

- Jede Szene wird in einem isolierten SubViewport instanziiert. Die Screenshot-Erfassung öffnet oder ändert die erstellte Szene nicht.
- Automatische 3D-Bildausschnittwahl kann neutrale Vorschaubeleuchtung hinzufügen, wenn das Asset weder Umgebung noch Licht besitzt.
- `scene_path` ist die einzige erforderliche Eingabe. Wenn sowohl `code` als auch
  `script_path` weggelassen werden, erfasst Fennara die abgekoppelte Wurzel mit automatischer Bildausschnittwahl.
- GDScript kann mit gewöhnlichem Godot-Code einen Knoten oder ein Array von Knoten
  auswählen, Subjekte frei gruppieren, Szenenteile ein- oder ausblenden, die abgekoppelte
  Szene vorübergehend ändern und Erfassungen mit `ctx.capture(...)` anfordern. Diese
  vorübergehenden Änderungen werden gerendert, aber niemals in der erstellten Szene gespeichert.
- `await ctx.capture(...)` rendert den Szenenstatus genau zu diesem Zeitpunkt und gibt ein
  gewöhnliches Godot-`Image` zurück. Der Worker kann erfasste Bilder untersuchen, vergleichen,
  skalieren, verwerfen oder kombinieren, bevor ausgewählte Ergebnisse mit
  `ctx.output(image, description)` veröffentlicht werden.
- Wenn eine skriptgesteuerte 3D-Erfassung für bis zu acht ausgewählte Subjekte `view`
  und `camera` weglässt, prüft Fennara 17 deterministische Blickpunkte und wählt einen
  aus, der Sichtbarkeit der ausgewählten Knoten, lesbare Größe, Abstand zum Rand und geringe
  Überlappung begünstigt. Verwende eine ausdrückliche Ansicht oder Kamera, wenn die nützliche
  Richtung bereits bekannt ist, und mehrere Erfassungen, wenn weit voneinander entfernte
  Subjekte in einem einzelnen Frame zu klein würden.
- Ein Screenshot-Worker erhält ausschließlich `ctx.root`, `await ctx.capture(...)`,
  `ctx.sheet(...)`, `ctx.output(...)`, `ctx.log(...)` und `ctx.error(...)`.
  `ctx.sheet(...)` setzt vom Aufrufer geordnete Images zu deterministischen, optional
  beschrifteten Seiten zusammen, ohne Zustände auszuwählen oder sie zu veröffentlichen. Es
  kann in den Erfassungsoptionen eine temporäre Camera2D oder Camera3D unter `ctx.root`
  übergeben, wenn ein exakt erstellter Bildausschnitt benötigt wird.
- Kamerapfade, Zielpfade, Ansichtsrechtecke und Bildausschnittparameter oberster Ebene werden
  nicht akzeptiert. Auswahl und Bildausschnittwahl liegen vollständig im Worker-Skript.
- Jedes veröffentlichte Bild wird gespeichert und aufgelistet. Bildfähige MCP-Clients und
  Modelle des integrierten Chats erhalten die ersten sechs veröffentlichten Ausgaben als
  getrennten Bildkontext in Aufrufreihenfolge. Spätere Ausgaben bleiben unter ihrem
  gespeicherten Pfad verfügbar, wobei die Bestätigung eine ausdrückliche Anzahl
  ausgelassener Bilder enthält.
- Spärliche Erfassungen werden mit Bildausschnittmetriken und Teilstatus zurückgegeben, statt das Bild zu verbergen.

Wichtige Einschränkungen und Fehler:

- Die automatische Bildausschnittwahl kann bei einem großen Innenraum, Zimmer, Level oder ungewöhnlichen geriggten Asset nicht immer den künstlerisch nützlichen Blickpunkt ableiten.
- Ein zurückgegebenes Bild kann gültig sein, während die Inhaltsvalidierung meldet, dass der Bildausschnitt spärlich oder unsicher ist.
- Reine Textmodelle erhalten die Bestätigung und gespeicherten Pfade, können angefügte Bildpixel jedoch nicht direkt sehen.
- Fehler beim Laden, Rendern, bei Erfassungseigentum oder Dateispeicherung werden gemeldet.
- Unbekannte veraltete Screenshot-Argumente werden mit einem Migrationsfehler abgelehnt.
- Fehler beim Parsen des Skripts, Laufzeitfehler, fehlende Erfassungsaufrufe, Knoten außerhalb
  der abgekoppelten Wurzel und ungültige temporäre Kameras werden gemeldet, ohne etwas zu erfassen.

<a id="runtimesession"></a>
### `runtime_session`

Startet, prüft oder beendet eine vom Daemon verwaltete Godot-Szene in einem Fenster.

Funktionsweise:

- Start-Gates werden ausgeführt, bevor ein Szenenprozess gestartet wird.
- Ein erfolgreicher Start gibt Sitzungskennung, Prozessstatus, Protokollpfade, Startbefunde und verfügbare Erfassungsinformationen zurück.
- Ein belegter rechnerweiter Laufzeit-Slot gibt das erfolgreiche Domänenergebnis
  `busy` mit `availability: "busy"`, `slot_acquired: false` und einem
  vorgeschlagenen `retry_after_ms` zurück. Der Eigentümer oder seine Sitzung wird
  nicht offengelegt.
- Der Status gibt neue Laufzeitausgabe zurück, ohne das vollständige Sitzungsprotokoll zu verwerfen.
- Stop gibt abschließende Prozess- und Protokollinformationen zurück.
- C#-Projekte erhalten vor dem Start einen echten Laufzeit-Build in Godots normale Debug-Ausgabe, damit der Prozess aktuelle Assemblies verwendet.
- Das Laufzeitprotokoll ist die maßgebliche Quelle für Godot-Ausgabe, Laufzeitfehler, Hilfsmarkierungen, Erfassungen und Stop-Ereignisse.
- `start` akzeptiert optionale `user_args`. Fennara übergibt jede Zeichenfolge unverändert nach Godots `--`-Trennzeichen, sodass das laufende Projekt sie mit `OS.get_cmdline_user_args()` lesen kann.

Wichtige Einschränkungen und Fehler:

- Global darf nur eine vom Daemon verwaltete Laufzeitsitzung gestartet werden oder
  laufen. Frage ein `busy`-Ergebnis mit Jitter ab und behandle jede neue
  Startanfrage als den endgültigen atomaren Anspruch; ein vorheriger freier Status
  ist nur ein Hinweis.
- Eine Laufzeitsitzung gehört zu ihrem kanonischen Projektstamm. Nur dieser
  Eigentümer darf detaillierten Status abrufen, die Lease erneuern, Skripte
  ausführen oder die Sitzung beenden. Andere Projekte sehen nur einen anonymen
  Belegungsstatus.
- Statusabfragen des Eigentümers erneuern eine Inaktivitätsfrist von 120
  Sekunden. Eine begrenzte Laufzeitoperation des Eigentümers setzt den
  Inaktivitätsablauf während ihrer Ausführung aus und erneuert die Frist erst,
  nachdem sie ein abschließendes Skriptergebnis zurückgegeben hat; bei
  Zeitüberschreitung, Einrichtungsfehler oder Abbruch wird sie nicht erneuert.
  Frage den Eigentümerstatus während einer Ausführung etwa alle 30 Sekunden mit
  Jitter ab.
- `max_run_seconds` ist eine positive Ganzzahl mit einem Standardwert von 900
  Sekunden und einem Höchstwert von 86.400 Sekunden. Für eine einstündige
  Regression können 4.500 Sekunden als Puffer angefordert werden. Die absolute
  Frist wird niemals angehalten.
- Natürliches Prozessende, ausdrücklicher Stopp, Startfehler, Inaktivitätsablauf
  oder absoluter Ablauf geben den Laufzeit-Slot frei.
- Fehlgeschlagene Start-Gates verhindern das Öffnen der Szene.
- Ein C#-Laufzeit-Build kann das normale Neuladen der Assembly im geöffneten Editor auslösen.
- Sowohl `FENNARA_RUNTIME_SESSION_READY` als auch
  `FENNARA_RUNTIME_ORIENTATION_NOTE` müssen vor Ablauf der fünfsekündigen
  Startfrist erscheinen. Fehlt eine der Markierungen, beendet der Daemon den
  Prozess und wartet auf dessen Ende, bevor er `startup_timeout` zurückgibt;
  ein anfänglicher Erfolg wird nicht gemeldet.
- Verwaltete Sitzungen sind getrennte Godot-Prozesse, nicht die manuell im Editor laufende Szene.

<a id="runtimescript"></a>
### `runtime_script`

Führt eine begrenzte GDScript-Sonde oder einen Eingabetreiber innerhalb einer aktiven
verwalteten Laufzeitsitzung aus.

Funktionsweise:

- Kann aktive Knoten untersuchen, Befunde protokollieren, auf einen Zustand warten,
  zugeordnete oder niedrigstufige Eingaben senden, Raycasts ausführen, mit einfacher UI
  interagieren und Frames erfassen.
- Kann ungespeicherte Viewport-Images mit `ctx.frame()` sammeln, dieselben
  aufrufergesteuerten Bögen wie Screenshot-Worker mit `ctx.sheet()` zusammensetzen und
  abgeleitete Images direkt mit `ctx.output()` veröffentlichen, ohne sie im Spiel anzuzeigen.
- Ein Skript kann enden, während die verwaltete Szene für eine weitere Sonde geöffnet bleibt.
- Ergebnisse enthalten Diagnosen, Laufzeitbefunde, Erfassungspfade, Protokollpfade und, sofern verfügbar, den Sitzungsstatus.

Wichtige Einschränkungen und Fehler:

- Es erfordert eine gültige aktive `runtime_session`-Kennung.
- Nur der Projektstamm, dem die Sitzung gehört, darf eine Sonde ausführen. Ein
  anderer Eigentümer erhält `runtime_session_not_owned_or_found`; dadurch wird
  nicht offengelegt, ob die Kennung eines anderen Projekts existiert.
- Eine begrenzte Sonde des Eigentümers setzt den Inaktivitätsablauf während
  ihrer Ausführung aus. Ein zurückgegebenes abschließendes Skriptergebnis
  erneuert die Inaktivitätsfrist; Zeitüberschreitung, Einrichtungsfehler oder
  Abbruch tun dies nicht. Keine Sonde verlängert die absolute Laufzeit-Lease.
- Laufzeitskripte sind keine Editor-`@tool`-Skripte und können nicht als Worker für Szenenbearbeitung verwendet werden.
- Ungültige Diagnosen, Zeitüberschreitungen, Laufzeitfehler, geschlossene Sitzungen oder nicht verfügbare Knoten werden gemeldet.
- Sonden müssen begrenzt bleiben. Sie sind kein Ersatz für ein dauerhaftes Framework zur Gameplay-Automatisierung.

<a id="scrapeeditor"></a>
### `scrape_editor`

Liest einen kompakten Debugger-Snapshot, nachdem der Benutzer eine Szene manuell über den
Godot-Editor ausgeführt hat.

Funktionsweise:

- Gruppiert wiederholte Debugger-Probleme und begrenzt übermäßig umfangreiche Details.
- Hilft bei der Untersuchung der vom Editor ausgeführten Ausgabe, die nicht im Besitz einer verwalteten Laufzeitsitzung ist.

Wichtige Einschränkungen und Fehler:

- Es ist absichtlich enger gefasst als das Lesen jedes UI-Elements oder jeder Protokollzeile des Editors.
- Es sollte nicht für Szenen verwendet werden, die über `runtime_session` gestartet wurden. Das verwaltete Laufzeitprotokoll ist vollständiger.
- Wenn nichts manuell ausgeführt wurde, ist möglicherweise kein nützlicher Debugger-Status verfügbar.

<a id="built-in-chat-tools-and-controls"></a>
## Werkzeuge und Steuerungen des integrierten Chats

<a id="readfile"></a>
### `read_file`

Liest projektbezogene Textdateien und unterstützte Bilder mithilfe von Godots Pfadbehandlung.
Es ist nützlich, wenn die Normalisierung von `res://` oder Bildbehandlung wichtig ist. Die
umfassende Navigation im Quellcode gehört weiterhin zu normalen Repository-Werkzeugen.

<a id="execcommand"></a>
### `exec_command`

Führt einen einzelnen nicht interaktiven Befehl aus, wobei der Stamm des aktiven Projekts
das standardmäßige Arbeitsverzeichnis ist.

Funktionsweise:

- Erfasst Standardausgabe und Fehler mit Zeit- und Ausgabegrenzen.
- Lehnt Arbeitsverzeichnisse außerhalb des aktiven Projektstamms ab.
- Speichert eine rohe Bestätigung auf Daemon-Seite, sodass umfangreiche Ausgabe nicht im Modellgespräch verbleiben muss.

Wichtige Einschränkungen und Fehler:

- Es handelt sich um eine Einschränkung auf den Projektstamm und um Genehmigungsbehandlung, nicht um eine Betriebssystem-Sandbox.
- Es stellt kein interaktives Terminal, PTY, keine Hintergrundsitzung, keinen Standardeingabestrom und keine beliebige Umgebungskonfiguration bereit.
- Exitcodes ungleich null, Zeitüberschreitungen und gekürzte Ausgabe werden gemeldet.

<a id="chat-controls"></a>
### Chat-Steuerungen

Der integrierte Chat unterstützt Genehmigungsmodi für projektverändernde Werkzeugaufrufe
und Laufzeit-Werkzeugaufrufe. Schreibgeschützte Untersuchungen können sofort ausgeführt
werden, während Änderungen oder Ausführungen eine ausdrückliche Genehmigung erfordern
können. Vollzugriff entfernt diese Abfragen, umgeht aber keine strengen Sicherheitsprüfungen.

Ausgewählter Code aus Godots Skripteditor kann mit **Add to Chat** angefügt werden. Der
Composer zeigt den Anhang vor dem Senden an. `/provider` öffnet die Anbietereinrichtung und
`/model` öffnet die Modellauswahl. Dies sind Chat-Befehle, keine MCP-Werkzeuge.

<a id="what-fennara-does-not-replace"></a>
## Was Fennara nicht ersetzt

Verwende normale Entwicklungswerkzeuge für:

- umfassende Suche und Navigation im Repository
- gewöhnliches Lesen von Textdateien
- Diffs und Versionskontrolle
- Bearbeitungen, die kein Godot-Feedback benötigen
- allgemeine Shell-Arbeit

Verwende Fennara, wenn die Antwort davon abhängt, dass Godot das Projekt versteht,
importiert, serialisiert, rendert, validiert oder ausführt.
