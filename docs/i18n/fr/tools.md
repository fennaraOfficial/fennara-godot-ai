<!-- fennara-i18n: locale=fr source=docs/tools.md sha256=4cf72381fada4fec347f29da5995d9768b39235f71b437dd698088ac0acb3518 -->
<a id="tools"></a>
# Outils

<!-- fennara-doc-nav:start -->
[Documentation](README.md)

🌐 **Langues**

[English](../../tools.md) · [简体中文](../zh-CN/tools.md) · [Español](../es/tools.md) · [Português do Brasil](../pt-BR/tools.md) · [日本語](../ja/tools.md) · [한국어](../ko/tools.md) · [Русский](../ru/tools.md) · **Français** · [Deutsch](../de/tools.md) · [Türkçe](../tr/tools.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../tools.md)
<!-- fennara-doc-nav:end -->

Fennara fournit aux agents de programmation l'inspection, la modification, la
validation, les captures d'écran et les informations d'exécution qui comprennent
Godot. Il complète les outils ordinaires du dépôt et du shell au lieu de les remplacer.

Cette page explique les capacités de chaque outil, la signification d'un appel
réussi et les principales limites ou causes d'échec. Les schémas d'outils actifs
restent la source de référence pour les arguments exacts, les champs de résultat,
les limites et les instructions destinées aux agents. Les projets installés
reçoivent également des instructions compactes et des connaissances à la demande
sous `addons/fennara/ai/`.

<a id="tool-surfaces"></a>
## Surfaces d'outils

Les clients MCP externes, notamment Codex, Claude Code, Cursor et Gemini, se
connectent par le processus local `fennara-mcp`. Ils utilisent leur propre compte
de modèle et leurs outils habituels de fichiers, de recherche, de diff et de shell
en complément de Fennara.

Le chat Fennara intégré utilise le même daemon et le même pont Godot. Il peut
appeler les mêmes outils Godot et fournit également les outils `read_file` et
`exec_command` limités au projet. La configuration du fournisseur et du modèle
appartient au chat intégré, pas au serveur MCP.

`fennara_status` est accessible aux clients MCP externes. Le chat intégré reçoit
déjà l'état de la connexion et du projet actif depuis le daemon.

<a id="typical-workflow"></a>
## Processus courant

1. Confirmez le projet connecté lorsque vous utilisez un client MCP externe.
2. Inspectez la scène, la ressource, la classe, l'état d'importation ou le réglage du projet concerné.
3. Effectuez la plus petite modification utile.
4. Exécutez les diagnostics ou la validation de la scène.
5. Utilisez les captures d'écran ou les outils d'exécution lorsqu'une preuve visuelle ou comportementale est importante.

Le système de fichiers de l'éditeur peut être temporairement occupé à analyser
ou importer. Utilisez les outils de ressources après qu'il a signalé être prêt.

<a id="connection"></a>
## Connexion

<a id="fennarastatus"></a>
### `fennara_status`

Indique l'état du serveur MCP, du daemon, du projet Godot actif, des sessions
d'éditeur connectées, des versions des composants, du contexte de rendu, des
outils annoncés et de la disponibilité du système de fichiers de l'éditeur.

Comportement normal :

- Renvoie un seul bloc d'état en texte brut.
- Distingue un système de fichiers d'éditeur prêt d'un système en cours d'analyse ou d'importation.
- Indique si les outils tournés vers les ressources sont actuellement prêts.
- Affiche les différences de version afin de diagnostiquer les installations incompatibles.

Limites et échecs importants :

- Il indique la disponibilité au niveau du projet, pas celle d'un chemin de ressource précis.
- Un daemon déconnecté, l'absence de projet actif ou un plugin Godot déconnecté
  est signalé directement au lieu d'être considéré comme un projet prêt.
- La disponibilité peut changer brièvement pendant que Godot réimporte des fichiers.

<a id="inspection"></a>
## Inspection

<a id="getscenetree"></a>
### `get_scene_tree`

Charge une scène au moyen de Godot et renvoie sa hiérarchie de nœuds, les classes
des nœuds, les scripts attachés et les sous-scènes instanciées. Les chemins
renvoyés peuvent être utilisés par les autres outils de scène.

Comportement normal :

- Lit les scènes créées sans les réécrire.
- Rend la structure des nœuds et des instances visible avant une modification.
- Garde le résultat centré sur la hiérarchie au lieu de développer chaque ressource.

Limites et échecs importants :

- Il ne s'agit pas d'un rapport complet sur les ressources 3D, les maillages, les matériaux, les squelettes ou les animations.
- Une scène que Godot ne peut pas charger renvoie un échec au lieu d'un arbre supposé.
- Les détails volumineux des ressources doivent être obtenus par l'inspection ciblée d'une propriété ou d'un script.

<a id="getnodeproperties"></a>
### `get_node_properties`

Affiche les propriétés qui diffèrent des valeurs par défaut de la classe pour
les nœuds sélectionnés et développe des résumés utiles des ressources intégrées.

Comportement normal :

- Prend en charge jusqu'à cinq nœuds cibles dans un appel.
- Lit les propriétés GDScript exportées et les métadonnées disponibles des scripts C#.
- Résume les ressources comme les animations, les thèmes, les données de tuiles,
  les bibliothèques de maillages, les images de sprites et les graphes d'animation
  au lieu de déverser des valeurs opaques.

Limites et échecs importants :

- Il cible des nœuds. Ce n'est pas un inventaire des ressources de toute la scène.
- Les ressources sources importées peuvent exposer moins d'informations que les
  nœuds `.tscn` créés. Utilisez `run_asset_import_script` lorsque la ressource
  importée générée doit être inspectée directement.
- Les chemins de nœud non valides sont signalés au lieu d'être ignorés silencieusement.

<a id="getclassinfo"></a>
### `get_class_info`

Renvoie la véritable surface d'API d'une classe Godot, notamment l'héritage, les
propriétés, les méthodes, les signaux, les énumérations, les constantes et la
documentation disponible.

Comportement normal :

- Les informations ClassDB à l'exécution proviennent de l'éditeur Godot connecté.
- Les classes intégrées utilisent la documentation XML officielle de Godot qui
  correspond à la version majeure et mineure connectée, avec un repli explicite sur `master`.
- Les classes GDExtension et celles des addons natifs renvoient les informations
  disponibles sur la classe et les propriétés à l'exécution sans prétendre
  posséder une documentation Godot officielle.

Limites et échecs importants :

- La recherche de documentation peut être incomplète lorsque le fichier XML de
  la classe amont correspondante est indisponible ou qu'une réponse ne peut pas
  être reçue en totalité.
- Le comportement propre à l'exécution peut encore nécessiter un petit script de sonde côté éditeur.
- Un nom de classe inexistant est signalé comme absent.

<a id="editing"></a>
## Modification

<a id="writeorupdatefile"></a>
### `write_or_update_file`

Crée, réécrit ou effectue un remplacement exact dans un fichier texte du projet.

Comportement normal :

- `write` crée ou remplace un fichier à partir de son contenu complet.
- `update` remplace un bloc de texte exact et unique.
- Les modifications de GDScript et de shaders renvoient automatiquement les diagnostics de Godot.
- Les modifications de shaders tentent aussi de sérialiser de nouveau les scènes
  et les ressources qui les référencent au moyen de Godot afin que les données
  de matériau intégrées ne restent pas obsolètes.
- Les écritures C# peuvent former une modification portant sur plusieurs fichiers
  avant la demande d'une compilation de diagnostic du projet.

Limites et échecs importants :

- Un texte de mise à jour ambigu ou absent entraîne un échec au lieu de modifier une correspondance arbitraire.
- Les chemins protégés de Fennara, de Git, du cache Godot, du manifeste du plugin
  et des réglages du projet ne peuvent pas être modifiés par cet outil.
- Il n'est pas destiné à la modification brute des fichiers `.tscn`, `.tres` ou `.res`.
- La validation C# ne s'exécute pas après chaque écriture. Utilisez une analyse
  de diagnostic du projet une fois toutes les modifications C# liées terminées.
- Les propriétaires des références de shader qui ne peuvent pas être sérialisés
  de nouveau en toute sécurité sont signalés comme ignorés ou accompagnés d'un avertissement.

<a id="runsceneeditscript"></a>
### `run_scene_edit_script`

Exécute un worker GDScript unique à l'heure de l'éditeur sur une scène créée ou
un graphe de ressources Godot. C'est le moyen structuré d'inspecter ou de modifier
les scènes au moyen du modèle d'objets et du sérialiseur de Godot.

Comportement normal :

- Le mode inspection charge un graphe de scène détaché en lecture seule et ne l'enregistre jamais.
- Le mode modification peut ajouter, supprimer, renommer ou reparentaliser des
  nœuds, affecter des ressources, modifier des propriétés, créer des scènes et
  enregistrer au moyen de la sérialisation Godot.
- Les scènes existantes sont enregistrées uniquement lorsque le worker marque le contexte comme modifié.
- Les nouveaux nœuds et les instances PackedScene utilisent des auxiliaires de
  propriété explicites afin que Godot sérialise la structure voulue.
- Les diagnostics des scripts sont exécutés avant l'exécution, et les scènes enregistrées sont ensuite validées.
- Les racines des scènes héritées sont préservées lorsque Godot peut sérialiser en toute sécurité les substitutions demandées.
- Chaque appel renvoie le chemin réel du worker temporaire afin qu'un worker en
  échec puisse être corrigé sans être recréé à partir de rien.

Limites et échecs importants :

- Le graphe chargé n'est pas équivalent à l'action Run Scene. Les API de jeu
  dépendantes de SceneTree, les temporisateurs, le traitement des images et les
  transformations globales peuvent se comporter différemment ou échouer sur des nœuds détachés.
- Le mode inspection bloque les auxiliaires de modification du contexte Fennara,
  mais tout GDScript arbitraire doit quand même éviter les effets de bord directs
  sur le système de fichiers, l'éditeur, le système d'exploitation et l'enregistrement des ressources.
- Les fichiers sources importés comme `.glb` et `.gltf` ne sont pas enregistrés
  par cet outil. Les réglages d'importation appartiennent à `run_asset_import_script`.
- Une propriété incorrecte des éléments internes d'une PackedScene est refusée,
  car elle peut aplatir ou dupliquer le contenu de l'instance.
- Si l'enregistrement risque d'aplatir une racine héritée, Fennara restaure le fichier original et signale un échec.
- Les diagnostics ou les erreurs d'exécution arrêtent la modification. Un résultat
  en échec ne crée ni ne met à jour la scène cible, mais le script de worker
  temporaire peut rester disponible pour une nouvelle tentative.

<a id="runassetimportscript"></a>
### `run_asset_import_script`

Exécute un worker GDScript borné à l'heure de l'éditeur sur une ressource source
importée et sa configuration d'importation Godot. Il prend en charge les modèles,
les textures, l'audio, les polices et les autres formats qui possèdent déjà un
fichier annexe `.import` correspondant.

Comportement normal en mode inspection :

- Indique l'importateur, la classe de la ressource générée, la validité de
  l'importation, les options actuelles typées, les fichiers générés et les dépendances amont.
- Charge la ressource générée sans réutiliser les entrées de cache imbriquées obsolètes.
- Peut instancier temporairement une PackedScene importée dans le SceneTree actif
  de l'éditeur pour une inspection bornée, puis la supprime sans l'enregistrer.
- Fournit des résumés bornés des sous-ressources générées.
- Ne conserve jamais les modifications des options d'importation en mode inspection.

Comportement normal en mode modification :

- Met en attente les options d'importation existantes prises en charge tout en préservant leurs types Variant natifs de Godot.
- Laisse l'éditeur actif effectuer la réimportation au moyen de `EditorFileSystem`.
- Signale une réussite uniquement après la vérification des réglages d'importation
  canoniques, des résultats générés, de l'état du système de fichiers de l'éditeur
  et d'un nouveau chargement approfondi de la ressource.
- Tente de restaurer et de réimporter la configuration précédente lorsque la
  vérification échoue, puis indique si cette récupération a réussi.

Limites et échecs importants :

- Le fichier source doit déjà être importé et posséder un fichier annexe `.import` valide.
- La première version modifie uniquement les options classées comme modifications
  sûres du cache généré pour les importateurs intégrés de textures et de scènes pris en charge.
- L'identité de l'importateur, les scripts d'importation, `_subresources`, les
  chemins d'extraction externes et les options dont l'effet est inconnu restent en lecture seule.
- Les options inconnues, les options non prises en charge et les valeurs dont le
  type Variant est incorrect entraînent un échec au lieu d'être converties.
- La modification directe du fichier `.import` est détectée, annulée et signalée
  comme un échec. Fennara possède la persistance du fichier annexe.
- Les scènes importées configurées avec un script racine ne sont pas instanciées
  temporairement par l'auxiliaire d'inspection.
- Les dépendances décrivent les fichiers nécessaires à l'importation de la ressource
  sélectionnée. Elles n'identifient pas les consommateurs en aval du projet, comme
  les scènes qui utilisent un modèle, les matériaux qui utilisent une texture,
  les scripts qui lisent un son ou les thèmes qui utilisent une police.
- Les diagnostics de script, les erreurs d'exécution ou de réimportation, les
  fichiers générés manquants, un état de système de fichiers invalide ou les
  échecs de rechargement empêchent tout résultat réussi.
- Les grands tableaux et les éléments internes des ressources sont bornés ou
  résumés pour protéger la sortie de l'outil. Un résultat borné ne garantit pas
  que chaque sommet, clé ou dépendance a été affiché dans la sortie.

<a id="projectsettings"></a>
### `project_settings`

Lit et modifie les réglages structurés du projet Godot, les autoloads, les
métadonnées de l'application, les réglages de rendu et d'affichage et les actions d'entrée.

Comportement normal :

- Utilise des opérations structurées qui comprennent Godot au lieu d'un remplacement de texte brut dans `project.godot`.
- Répertorie les actions d'entrée avec leurs zones mortes, le nombre d'événements et des résumés lisibles des événements.
- Prend en charge les événements d'entrée structurés lors de l'ajout ou de la mise à jour des contrôles.

Limites et échecs importants :

- Les opérations inconnues et les valeurs de réglage non valides sont signalées.
- Cet outil ne remplace pas la modification des scènes ou des scripts.
- Les modifications doivent quand même être validées lorsqu'elles touchent le démarrage, le rendu, les entrées ou le comportement de l'addon.

<a id="checks"></a>
## Vérifications

<a id="scriptdiagnostics"></a>
### `script_diagnostics`

Exécute des diagnostics qui comprennent Godot pour les scripts et les shaders.

Comportement normal :

- Les appels ciblés de GDScript et de shaders prennent en charge jusqu'à cinq fichiers.
- Les diagnostics GDScript proviennent du serveur de langage de Godot.
- Les diagnostics de shaders proviennent de l'analyseur de shaders de Godot.
- Les vérifications GDScript ciblées chargent également les scènes pertinentes
  en mémoire afin que les erreurs provoquées par l'attachement à une scène
  puissent être associées au script et à la scène.
- Les analyses du projet vérifient GDScript et les shaders, puis effectuent une
  compilation C# incrémentale et isolée lorsqu'un projet C# est présent.
- Les assemblages C# de diagnostic restent séparés des assemblages d'exécution normaux de l'éditeur.

Limites et échecs importants :

- Les diagnostics de fichiers C# ciblés ne sont pas pris en charge. C# utilise une analyse du projet.
- Les analyses de tout le projet n'instancient pas chaque scène et peuvent manquer
  les problèmes qui apparaissent uniquement lorsqu'un script est chargé par une scène donnée.
- Les échecs du serveur de langage, de l'analyseur ou de la compilation sont renvoyés
  comme des échecs de diagnostic, et non considérés comme des résultats propres.
- Les diagnostics prouvent que le code vérifié peut être analysé ou compilé dans
  le contexte testé. Ils ne prouvent pas que le comportement du jeu est correct.

<a id="validatescene"></a>
### `validate_scene`

Recherche les problèmes structurels dans une ou plusieurs scènes et, lorsque
c'est possible, exécute un bref passage de démarrage sans interface.

Comportement normal :

- Accepte jusqu'à dix chemins de scène.
- Les vérifications structurelles couvrent les scripts et ressources manquants,
  les chemins de nœud non valides, les noms de frères dupliqués, les dépendances
  cycliques entre scènes et les références exportées pertinentes.
- Les références exportées facultatives ou affectées à l'exécution sont signalées
  comme des remarques plutôt que comme des échecs inconditionnels.
- Les scènes créées dont les résultats structurels sont propres reçoivent un
  passage de démarrage sans interface de trois secondes, dont les journaux et
  artefacts sont conservés.
- Les constats répétés sont regroupés afin que les grandes scènes ne saturent pas le résultat.

Limites et échecs importants :

- Les scènes sources importées reçoivent uniquement une validation structurelle,
  car elles ne peuvent pas être lancées directement comme scènes de projet créées.
- Fennara arrête volontairement le processus après la fenêtre de validation. Ce
  code d'arrêt ne constitue pas à lui seul un échec de la scène.
- Un bref passage de démarrage ne peut pas valider tous les chemins du jeu, les
  visuels, les performances, la qualité des animations ou les interactions utilisateur.
- Les erreurs structurelles empêchent le passage d'exécution de cette scène.

<a id="visual-and-runtime-feedback"></a>
## Retour visuel et d'exécution

<a id="screenshotscene"></a>
### `screenshot_scene`

Capture des preuves visuelles provenant des scènes créées et des ressources 3D importées prises en charge.

Comportement normal :

- Chaque scène est instanciée dans un SubViewport isolé. La capture d'écran
  n'ouvre ni ne modifie la scène créée.
- Le cadrage 3D automatique peut ajouter un éclairage d'aperçu neutre lorsque
  la ressource ne possède ni environnement ni lumières.
- `scene_path` est la seule entrée obligatoire. Lorsque `code` et `script_path`
  sont tous deux omis, Fennara capture la racine détachée avec un cadrage automatique.
- Le GDScript peut sélectionner un nœud ou un tableau de nœuds avec du code Godot
  ordinaire, regrouper librement les sujets, afficher ou masquer des parties de
  la scène, modifier temporairement la scène détachée et demander des captures
  avec `ctx.capture(...)`. Ces modifications temporaires sont rendues, mais
  jamais enregistrées dans la scène créée.
- `await ctx.capture(...)` rend l'état de la scène à cet instant précis et renvoie
  une `Image` Godot ordinaire. Le worker peut inspecter, comparer, redimensionner,
  ignorer ou combiner les images capturées avant de publier les résultats choisis
  avec `ctx.output(image, description)`.
- Pour un maximum de huit sujets sélectionnés, lorsqu'une capture 3D scriptée
  omet `view` et `camera`, Fennara examine 17 points de vue déterministes et
  choisit celui qui favorise la visibilité des nœuds sélectionnés, une taille
  lisible, un dégagement des bords et un faible chevauchement. Utilisez une vue
  ou une caméra explicite lorsque la direction utile est déjà connue. Utilisez
  plusieurs captures lorsque des sujets éloignés deviendraient trop petits dans une seule image.
- Un worker de capture d'écran reçoit uniquement `ctx.root`, `await ctx.capture(...)`,
  `ctx.sheet(...)`, `ctx.output(...)`, `ctx.log(...)` et `ctx.error(...)`.
  `ctx.sheet(...)` compose les Images dans l'ordre donné par l'appelant en pages
  déterministes, avec des libellés facultatifs, sans choisir les états ni les
  publier. Il peut transmettre une Camera2D ou Camera3D temporaire sous `ctx.root`
  dans les options de capture lorsqu'un cadrage créé exact est nécessaire.
- Les chemins de caméra, les chemins de cible, les rectangles de vue et les
  paramètres de cadrage de premier niveau ne sont pas acceptés. La sélection et
  le cadrage résident entièrement dans le script du worker.
- Chaque image publiée est enregistrée et répertoriée. Les clients MCP qui prennent
  en charge les images et les modèles du chat intégré reçoivent les six premiers
  résultats publiés comme contextes d'image distincts, dans l'ordre des appels.
  Les résultats ultérieurs restent disponibles par leur chemin enregistré, et
  le reçu indique explicitement le nombre d'images omises.
- Les captures clairsemées sont renvoyées avec leurs mesures de cadrage et un
  état partiel au lieu de masquer l'image.

Limites et échecs importants :

- Le cadrage automatique ne peut pas toujours déduire le point de vue artistiquement
  utile pour un grand intérieur, une pièce, un niveau ou une ressource squelettée inhabituelle.
- Une image renvoyée peut être valide alors que la validation du contenu signale
  un cadrage clairsemé ou incertain.
- Les modèles uniquement textuels reçoivent le reçu et les chemins enregistrés,
  mais ne peuvent pas voir directement les pixels des images jointes.
- Les échecs de chargement, de rendu, de propriété de la capture ou d'enregistrement du fichier sont signalés.
- Les anciens arguments inconnus de capture d'écran sont refusés avec une erreur de migration.
- Les erreurs d'analyse du script, les erreurs d'exécution, l'absence d'appel de
  capture, les nœuds hors de la racine détachée et les caméras temporaires non
  valides sont signalés sans effectuer de capture.

<a id="runtimesession"></a>
### `runtime_session`

Démarre, vérifie ou arrête une scène Godot fenêtrée gérée par le daemon.

Comportement normal :

- Des barrières de démarrage sont exécutées avant le lancement du processus de la scène.
- Un démarrage réussi renvoie un identifiant de session, l'état du processus,
  les chemins des journaux, les constats de démarrage et les informations de capture disponibles.
- L'état renvoie les nouvelles sorties d'exécution sans supprimer le journal complet de la session.
- L'arrêt renvoie les informations finales du processus et du journal.
- Les projets C# reçoivent une véritable compilation d'exécution dans la sortie
  Debug normale de Godot avant le lancement, afin que le processus utilise les assemblages actuels.
- Le journal d'exécution constitue la source de référence pour la sortie de Godot,
  les erreurs d'exécution, les marqueurs auxiliaires, les captures et les événements d'arrêt.

Limites et échecs importants :

- Une seule session d'exécution gérée par le daemon est active globalement à la fois.
- L'échec des barrières de démarrage empêche l'ouverture de la scène.
- Une compilation d'exécution C# peut provoquer le rechargement normal des assemblages dans l'éditeur ouvert.
- Les marqueurs de disponibilité du démarrage peuvent arriver après la réponse
  initiale et apparaître lors d'un appel d'état ultérieur.
- Les sessions gérées sont des processus Godot distincts, pas la scène exécutée manuellement dans l'éditeur.

<a id="runtimescript"></a>
### `runtime_script`

Exécute une sonde GDScript bornée ou un pilote d'entrée dans une session
d'exécution gérée active.

Comportement normal :

- Peut inspecter les nœuds actifs, journaliser les constats, attendre un état,
  envoyer des entrées mappées ou de bas niveau, effectuer des raycasts, interagir
  avec une interface élémentaire et capturer des images.
- Peut collecter des Images de viewport non enregistrées avec `ctx.frame()`,
  composer les mêmes planches contrôlées par l'appelant que les workers de
  capture d'écran avec `ctx.sheet()` et publier directement des Images dérivées
  avec `ctx.output()` sans les afficher dans le jeu.
- Un script peut se terminer tandis que la scène gérée reste ouverte pour une autre sonde.
- Les résultats comprennent les diagnostics, les constats d'exécution, les chemins
  des captures et des journaux et, lorsqu'il est disponible, l'état de la session.

Limites et échecs importants :

- Il exige un identifiant `runtime_session` actif valide.
- Les scripts d'exécution ne sont pas des scripts `@tool` de l'éditeur et ne
  peuvent pas servir de workers de modification des scènes.
- Les diagnostics non valides, les délais dépassés, les erreurs d'exécution, les
  sessions fermées ou les nœuds indisponibles sont signalés.
- Les sondes doivent rester bornées. Elles ne remplacent pas un framework
  permanent d'automatisation du jeu.

<a id="scrapeeditor"></a>
### `scrape_editor`

Lit un instantané compact du débogueur après l'exécution manuelle d'une scène par l'utilisateur dans l'éditeur Godot.

Comportement normal :

- Regroupe les problèmes répétés et limite les détails bruyants.
- Aide à inspecter la sortie lancée par l'éditeur qui n'appartient pas à une session d'exécution gérée.

Limites et échecs importants :

- Il est volontairement plus limité que la lecture de chaque élément d'interface ou ligne de journal de l'éditeur.
- Il ne doit pas être utilisé pour les scènes lancées par `runtime_session`. Le journal d'exécution géré est plus complet.
- Aucun état utile du débogueur ne peut être disponible lorsque rien n'a été exécuté manuellement.

<a id="built-in-chat-tools-and-controls"></a>
## Outils et contrôles du chat intégré

<a id="readfile"></a>
### `read_file`

Lit les fichiers texte limités au projet et les images prises en charge au moyen
de la gestion des chemins de Godot. Il est utile lorsque la normalisation
`res://` ou la gestion des images est importante. La navigation générale dans
les sources appartient toujours aux outils ordinaires du dépôt.

<a id="execcommand"></a>
### `exec_command`

Exécute une commande non interactive avec la racine du projet actif comme répertoire de travail par défaut.

Comportement normal :

- Capture la sortie standard et les erreurs avec des limites de temps et de volume.
- Refuse les répertoires de travail situés hors de la racine du projet actif.
- Conserve un reçu brut côté daemon afin que les longues sorties ne restent pas dans la conversation du modèle.

Limites et échecs importants :

- Il s'agit d'une limitation à la racine du projet et d'une gestion des approbations, pas d'un bac à sable du système d'exploitation.
- Il ne fournit ni terminal interactif, ni PTY, ni session en arrière-plan, ni
  flux d'entrée standard, ni configuration arbitraire de l'environnement.
- Les codes de sortie non nuls, les délais dépassés et la troncature de la sortie sont signalés.

<a id="chat-controls"></a>
### Contrôles du chat

Le chat intégré prend en charge des modes d'approbation pour les appels d'outil
qui modifient le projet ou l'exécution. L'inspection en lecture seule peut
s'exécuter immédiatement, tandis que la modification ou l'exécution peut exiger
une approbation explicite. L'accès complet supprime ces demandes, mais ne
contourne pas les contrôles de sécurité stricts.

Le code sélectionné dans l'éditeur de scripts de Godot peut être joint avec
**Add to Chat**. La zone de rédaction affiche la pièce jointe avant l'envoi.
`/provider` ouvre la configuration des fournisseurs et `/model` ouvre la
sélection du modèle. Ce sont des commandes de chat, pas des outils MCP.

<a id="what-fennara-does-not-replace"></a>
## Ce que Fennara ne remplace pas

Utilisez les outils de développement ordinaires pour :

- la recherche et la navigation générales dans le dépôt
- la lecture des fichiers texte ordinaires
- les diffs et le contrôle de version
- les modifications qui ne nécessitent pas d'informations de Godot
- le travail général dans le shell

Utilisez Fennara lorsque la réponse dépend de la compréhension, de l'importation,
de la sérialisation, du rendu ou de la validation par Godot, ou de l'exécution du projet.
