<!-- fennara-i18n: locale=fr source=docs/repo-map.md sha256=48c47152f755d34f1f6526d58c15c3d16205d5389a00442e4f1409efa514e73c -->
<a id="repo-map"></a>
# Plan du dépôt

<!-- fennara-doc-nav:start -->
[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · [Русский](../ru/repo-map.md) · **Français** · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../repo-map.md)
<!-- fennara-doc-nav:end -->

Voici le plan rapide destiné aux contributeurs et aux agents de programmation qui travaillent dans ce dépôt.

<a id="find-the-right-area"></a>
## Trouver la bonne zone

| Modification | Emplacement principal |
| --- | --- |
| Installation utilisateur ou comportement de la CLI | `local/crates/fennara-cli/` |
| Protocole MCP externe ou schémas | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Découverte ou égalité des racines de projet | `local/crates/fennara-project-identity/` |
| Chat intégré ou comportement du daemon | `local/crates/fennara-daemon/` |
| Intégration à l'éditeur Godot | `fennara-cpp/` |
| Interface de chat | `ui/chat/` |
| Scripts auxiliaires d'exécution | `runtime/` |
| Création de paquets ou publication | `scripts/`, `.github/workflows/` |
| Documentation utilisateur | `README.md`, `docs/` |

<a id="top-level"></a>
## Niveau supérieur

| Chemin | Responsabilité |
| --- | --- |
| `.github/` | Modèle de pull request, modèles d'issue et processus GitHub Actions. |
| `docs/` | Documentation du projet, guides d'installation, notes d'architecture, exemples, démos et notes de publication. |
| `docs/i18n/` | Manifeste des langues et arborescences complètes de la documentation traduite. |
| `fennara-cpp/` | Sources C++ de la GDExtension Godot et point d'entrée de compilation SCons. |
| `godot_demo/addons/fennara/` | Charge utile de l'addon Godot installable et copiée dans les projets des utilisateurs. |
| `local/` | CLI Rust, serveur MCP, daemon, schémas et code de l'environnement d'exécution local. |
| `media/` | Images et médias publics employés par la documentation. |
| `runtime/` | Sources des scripts auxiliaires d'exécution Godot utilisés par `runtime_session` et `runtime_script`. |
| `scripts/` | Scripts auxiliaires de gestion des versions, de création des paquets et de publication. |
| `ui/chat/` | Sources de l'interface facultative de chat web intégrée à l'éditeur. |
| `local/templates/` | Instructions compactes du projet et pages de connaissances IA à la demande écrites dans les projets Godot par `fennara install` et actualisées par `fennara update`. |
| `local/webview-runtimes/` | Fichiers de manifeste et de configuration des environnements de webview externes installés dans les données d'application Fennara partagées, comme la charge utile CEF sous Linux. |
| `install.ps1` / `install.sh` | Scripts d'amorçage qui installent la CLI Fennara depuis les versions publiées sur GitHub. |
| `VERSION` | Source de référence de la version. |
| `README.md` | Présentation courte destinée aux humains et démarrage rapide. |
| `docs/README.md` | Index de documentation organisé par tâche. |
| `docs/setup.md` | Installation tournée vers l'addon pour les utilisateurs, prérequis du chat, connexion MCP, processus de mise à jour et dépannage. |
| `docs/multi-agent-worktrees.md` | Routage d'une connexion MCP par projet, limites des hôtes pris en charge, vérification et utilisation sérialisée de l'emplacement d'exécution. |
| `docs/cli.md` | Référence des commandes du terminal, comportement d'installation et de mise à jour appartenant à la CLI, récupération, diagnostics, disposition des données d'application et conseils d'automatisation. |
| `docs/telemetry.md` | Charge utile d'activité anonyme, état dans les données d'application, comportement d'envoi, définition de l'activité mensuelle et contrôles de désactivation. |
| `CONTRIBUTING.md` | Règles de contribution. |
| `SECURITY.md` | Politique de signalement des problèmes de sécurité. |
| `LICENSE.md` | Licence du projet. |

<a id="local-rust-packages"></a>
## Paquets Rust locaux

| Chemin | Responsabilité |
| --- | --- |
| `local/crates/fennara-cli/` | Commande `fennara` : installation, mise à jour, mise à jour automatique de la CLI, diagnostic, diagnostics des opérations, vérification des prérequis de webview, prise en charge de C#, configuration des applications MCP et instructions de projet générées. |
| `local/crates/fennara-cli/src/operation.rs` | Coordinateur public des opérations d'installation et de mise à jour, phases et points d'entrée de transmission à la CLI. |
| `local/crates/fennara-cli/src/operation/` | Journal d'opérations spécialisé, stockage durable, masquage des diagnostics et modules de test. |
| `local/crates/fennara-cli/src/project_addon.rs` | Validation de la version de l'addon existant du projet et de la bibliothèque GDExtension de la plateforme actuelle. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Préparation des exportations CI sans addon, qui retire uniquement l'autoload d'exécution persistant de Fennara avant le démarrage de Godot. |
| `local/crates/fennara-cli/src/release_identity.rs` | Identité stable ou de prépublication de l'addon, sélecteurs de versions exactes, validation du canal de pull request et compatibilité avec les anciennes versions stables. |
| `local/crates/fennara-cli/src/release_channel.rs` | Validation du pointeur de prépublication propre à chaque canal et résolution vers une version numérotée exacte. |
| `local/crates/fennara-cli/src/release_manifest.rs` | Analyse du manifeste de version, validation des hachages des ressources, liaison de l'identité et sélection du paquet de plateforme. |
| `local/crates/fennara-cli/src/release_version.rs` | Analyse SemVer partagée de la CLI et ordre de priorité employés par les manifestes et la sélection des versions. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Adoption de la version exacte d'un addon complet existant sans remplacer les fichiers de l'addon du projet. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Vérification de l'état du daemon partagé, disponibilité de la version exacte et démarrage employés par l'installation et le diagnostic. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Tests au niveau des processus pour les échecs, les diagnostics durables, le masquage et les journaux d'opération à échec fermé. |
| `local/crates/fennara-cli/src/diagnostics.rs` | Accès destiné à l'utilisateur au dernier rapport d'opération nettoyé ou à un rapport nommé. |
| `local/crates/fennara-project-identity/` | Découverte, validation et canonicalisation partagées des racines de projet, conversion sans perte pour le protocole et comparaison de l'identité active dans le système de fichiers. |
| `local/crates/fennara-mcp/` | Serveur MCP stdio local, découverte de la liaison de projet limitée au processus, présentation de l'état et transmission des schémas d'outils. |
| `local/crates/fennara-daemon/` | Daemon local partagé utilisé pour le routage selon la racine de projet, la propriété et les baux de l'emplacement d'exécution, les sessions d'exécution et le travail du pont Godot. |
| `local/crates/fennara-daemon/src/runtime_daemon/godot_bridge.rs` | Sélection mutuellement exclusive d'une session d'éditeur, d'un projet lié ou de la cible MCP héritée, ainsi que corrélation des requêtes et réponses Godot. |
| `local/crates/fennara-daemon/src/runtime_daemon/runtime_slot.rs` | Admission atomique de l'emplacement d'exécution à l'échelle de la machine, identité du propriétaire et état du bail. |
| `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs` | Cycle de vie du processus de jeu géré, autorisation du propriétaire, journaux, nettoyage après expiration et reçus de session. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | Planificateur d'activité quotidienne anonyme, file bornée, envoi HTTP et intégration au cycle de vie du daemon. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | Validation de l'identité d'installation aléatoire, persistance atomique dans les données d'application, état du reçu quotidien et nettoyage après désactivation. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Modes d'approbation du chat intégré, classification du risque des outils, décisions d'autorisation et types de demandes d'approbation en attente. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Implémentation de `exec_command` appartenant au daemon pour le chat intégré : détection du shell, validation du cwd, lancement du processus, délai et arrêt de l'arbre, capture de la sortie, journalisation de l'artefact de résultat et mise en forme du résultat. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Planificateur de compactage du contexte du chat intégré : protection exacte de la fin, élagage de style OpenCode sous la pression des anciens résultats d'outils, sélection, stockage et relecture des segments de résumé, sérialisation du prompt de résumé, budgets de jetons et rendu des espaces réservés. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | PromptBuilder du chat intégré et contexte d'environnement d'exécution généré. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Enregistreur de traces du chat intégré uniquement local, lignes d'événement SQLite, conservation et auxiliaires de requête de débogage. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | Primitives d'exécution des fournisseurs du chat intégré, catalogue et résolution, hooks de vérification préalable du contexte, types normalisés de flux et d'erreur, et adaptateurs compatibles OpenAI ou Anthropic pour OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, points d'accès personnalisés, Ollama local et LM Studio. |
| `local/schemas/tools/` | Schémas JSON partagés des outils. Le serveur MCP externe et le chat intégré incorporent chacun leur propre sous-ensemble autorisé. |
| `local/webview-runtimes/linux-cef.json` | Manifeste temporaire ou généré de l'environnement CEF Linux, employé pour générer le manifeste de version, afficher le diagnostic et assurer la compatibilité historique. Il enregistre la disposition des données d'application partagées et les métadonnées de l'archive sans placer CEF dans le fichier ZIP de l'addon. |
| `local/Cargo.toml` | Configuration de l'espace de travail Rust. |
| `local/Cargo.lock` | Graphe verrouillé des dépendances Rust. |

<a id="gdextension-source"></a>
## Sources de la GDExtension

| Chemin | Responsabilité |
| --- | --- |
| `fennara-cpp/SConstruct` | Point d'entrée de compilation de la GDExtension. |
| `fennara-cpp/include/` | En-têtes C++ publics. |
| `fennara-cpp/src/` | Implémentation C++. |
| `fennara-cpp/src/setup/` | État natif de la première installation, amorçage de la CLI depuis le manifeste de version, vérification des hachages, lancement de la CLI et lecteur durable de la progression de l'opération. |
| `fennara-cpp/src/release/version.cpp` | Validation et priorité SemVer natives employées par la découverte des versions et des mises à jour. |
| `fennara-cpp/src/release/identity.cpp` | Validation de l'identité stable ou de prépublication empaquetée et compatibilité avec les anciennes versions stables. |
| `fennara-cpp/src/release/discovery.cpp` | Découverte de GitHub Latest et des mises à jour de canaux de prépublication isolés. |
| `fennara-cpp/src/update/` | Coordination de la mise à jour vers une cible exacte, découverte des reçus durables, transmission de la fermeture et de l'installation, et état de l'interface de récupération. |
| `fennara-cpp/src/ui/setup_panel.cpp` | Panneau natif de première installation indépendant de la webview, avec progression, nouvelle tentative, journaux et actions sur le rapport nettoyé. |
| `fennara-cpp/vendor/cef/` | Copie des en-têtes officiels CEF 139 utilisée par le pont OSR Linux. Les binaires d'exécution restent hors de l'addon. |
| `fennara-cpp/src/ui/webview_host*` | Hôte natif de la webview de chat intégrée à l'éditeur et backends des plateformes. |
| `fennara-cpp/src/ui/native_webview_occlusion.*` | Détection partagée entre Windows et macOS qui masque temporairement la superposition de webview native lorsqu'une fenêtre contextuelle Godot ou une interface d'éditeur de premier plan la recouvre. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Découverte Linux de l'environnement CEF partagé, validation du marqueur et base du chargeur dynamique de `libcef.so`. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Surface de rendu CEF hors écran propre à Linux, transmission des entrées Godot, chargement de l'ABI du pont et mises à jour de la texture Godot pour la webview de chat interne. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Petite bibliothèque de pont réservée à Linux et compilée depuis les sources officielles épinglées de `libcef_dll_wrapper` pour CEF 139 et l'adaptateur CEF OSR de Fennara. La GDExtension principale la charge par dlopen après le chargement de l'environnement externe `libcef.so`. |
| `fennara-cpp/src/tools/` | Implémentations des outils tournés vers Godot. |
| `fennara-cpp/src/lsp/` | Diagnostics des scripts et auxiliaires du serveur de langage. |
| `fennara-cpp/src/csharp/` | Sélection de projet C# réservée à la compilation, préparation en arrière-plan, diagnostics isolés et vérification préalable de l'exécution. |
| `fennara-cpp/src/runtime/` | Prise en charge native de l'exécution employée par les outils, notamment la vérification préalable des scènes, les diagnostics de scripts et les instantanés du débogueur. |
| `fennara-cpp/godot-cpp/` | Sous-module des liaisons C++ de Godot. |

<a id="addon-payload"></a>
## Charge utile de l'addon

| Chemin | Responsabilité |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Fichier d'enregistrement de la GDExtension Godot. |
| `godot_demo/addons/fennara/VERSION` | Version du paquet de l'addon. |
| `godot_demo/addons/fennara/release.json` | Identité stable ou de prépublication empaquetée, notamment la version précise, l'étiquette de publication, le canal et le commit source de prépublication. |
| `godot_demo/addons/fennara/bin/` | Bibliothèques compilées des plateformes. |
| `godot_demo/addons/fennara/dist/` | Ressources empaquetées de l'interface web employées par la webview de chat intégrée à l'éditeur. |
| `godot_demo/addons/fennara/runtime/` | Copie empaquetée et synchronisée de `runtime/` livrée dans l'addon. |
| `godot_demo/tests/first_run_setup_test.gd` | Test déterministe et sans interface de l'état natif de première installation et des échecs. |
| `godot_demo/tests/export_plugin_test.gd` | Test de régression natif sans interface de l'exclusion à l'exportation et de la restauration de l'autoload. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | Test de régression sans interface du contrat des arguments natifs de capture d'écran. |
| `godot_demo/tests/image_sheet_test.gd` | Test de régression sans interface de la composition partagée des planches de captures et d'exécution. |
| `godot_demo/tests/runtime_image_context_test.gd` | Test de régression sans interface des images brutes, planches et sorties Image arbitraires à l'exécution. |

<a id="runtime-helper-source"></a>
## Sources des auxiliaires d'exécution

| Chemin | Responsabilité |
| --- | --- |
| `runtime/game_capture_helper.gd` | Point d'entrée auxiliaire à l'exécution chargé par la GDExtension pour les sessions de scène et les vérifications d'exécution. |
| `runtime/image_label.gd` | Libellés compacts et déterministes apposés sur les cellules Image composées après la capture. |
| `runtime/image_sheet.gd` | Composition de planches Image pure partagée par les contextes des captures d'écran et des scripts d'exécution. |
| `runtime/screenshot_script_context.gd` | Façade publique des scripts de capture d'écran qui ajoute la composition Image partagée au contexte de capture natif. |
| `runtime/runtime_script_context.gd` | Surface publique de l'auxiliaire `ctx` exposée à `runtime_script`, comprenant les images brutes, la composition et la sortie Image, les attentes, les entrées, les instantanés, les conditions, les raycasts et les clics. |
| `runtime/runtime_input_driver.gd` | Pilote de bas niveau des événements d'entrée à l'exécution pour les touches, les boutons de souris, les mouvements absolus et relatifs de la souris, les modificateurs et le nettoyage des entrées. |
| `runtime/runtime_node_snapshot.gd` | Recherche de nœuds à l'exécution, vérification de leur existence, instantanés sûrs face aux références obsolètes, lecture des propriétés et résumés des enfants. |
| `runtime/runtime_physics_query.gd` | Auxiliaires d'exécution de raycast et d'analyse exacts en 2D et 3D, avec reçus de résultat compacts. |
| `runtime/runtime_query_utils.gd` | Utilitaires partagés des requêtes d'exécution pour la conversion des vecteurs, la résolution sûre des nœuds et chemins, l'identité des objets et la correspondance générique des cibles. |
| `runtime/runtime_capture_store.gd` | Enregistreur d'artefacts de capture et d'état à l'exécution, employé par les sessions, les scripts et les vérifications d'environnement. |
| `runtime/runtime_check_runner.gd` | Exécuteur de vérifications à l'exécution pour les spécifications d'exécution de scène non interactives. |

<a id="scripts-and-workflows"></a>
## Scripts et processus automatisés

| Chemin | Responsabilité |
| --- | --- |
| `scripts/set-version.mjs` | Met à jour les fichiers versionnés dans l'ensemble du dépôt. |
| `scripts/check-version.mjs` | Vérifie la synchronisation des versions. |
| `scripts/release-identity.mjs` | Validation et génération Node partagées de l'identité SemVer publiée et des pointeurs de prépublication propres aux PR. |
| `scripts/release-policy.mjs` | Politique de la CLI publiée minimale compatible pour les manifestes stables et de prépublication. |
| `scripts/staging-candidate.mjs` | Génération fiable de l'identité du candidat de prépublication et décisions monotones du pointeur propre à la PR. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | Validation spécialisée de l'addon, de l'archive, du manifeste, du système de fichiers partagé et de l'ensemble de publication de préproduction. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | Points d'entrée de validation stricte pour les résultats de compilation non fiables et l'ensemble fiable destiné à la publication. |
| `scripts/check-staging-channel-advance.mjs` | Applique les contrôles de monotonie et de provenance avant l'avancement d'un pointeur de canal de prépublication. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | Vérifie les octets des ressources publiées et le comportement public du téléchargement avant la promotion du pointeur. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | Compile un projet Godot temporaire ignoré et teste rapidement l'inspection en lecture seule d'une `PackedScene` importée avec la GDExtension de l'éditeur. |
| `scripts/release-targets.mjs` | Définit les cibles de publication prises en charge et les noms de leurs ressources empaquetées. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | Écrit l'identité figée du candidat et son petit pointeur de canal. |
| `scripts/sync-chat-ui.mjs` | Copie les sources sans compilation de l'interface de chat dans la charge utile de l'addon. |
| `scripts/sync-runtime.mjs` | Copie les sources des auxiliaires d'exécution de la racine du dépôt dans la charge utile de l'addon. |
| `scripts/sync-doc-navigation.mjs` | Ajoute la navigation de la documentation, les hachages de source et les ancres stables sans traduire la prose. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | Valide la couverture des traductions, leur actualité, la structure Markdown, les URL et les liens. |
| `scripts/package-preview.mjs` | Assemble les fichiers ZIP de prévisualisation ou de publication de l'addon, de la CLI et de l'environnement local après les compilations des plateformes. |
| `scripts/prepare-linux-cef-runtime.mjs` | Prépare le fichier ZIP distinct de l'environnement CEF Linux x64, retire les symboles des binaires ELF préparés, valide les fichiers requis et peut écrire le manifeste de version généré. |
| `scripts/prepare-linux-cef-sdk.mjs` | Télécharge et extrait le SDK minimal officiel CEF 139 Linux épinglé pour les compilations CI qui nécessitent les sources du wrapper `libcef_dll/`. |
| `scripts/check-linux-cef-runtime-release.mjs` | Valide la ressource publiée de l'environnement CEF Linux selon le manifeste généré `local/webview-runtimes/linux-cef.json`. |
| `scripts/write-release-manifest.mjs` | Écrit et valide `fennara-release-manifest-v<version>.json` à partir des ressources publiées, notamment les hachages du paquet local, de l'addon et des environnements partagés. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Sources du petit sous-processus auxiliaire CEF Linux empaqueté dans le fichier ZIP d'exécution CEF distinct. |
| `.github/workflows/version-check.yml` | Vérification de cohérence des versions. |
| `.github/workflows/gdextension-build.yml` | Vérification de compilation multiplateforme de la GDExtension et test natif sans interface de l'état de première installation sous Windows. |
| `.github/workflows/local-build.yml` | Vérification de compilation du paquet Rust local. |
| `.github/workflows/package-preview.yml` | Artefacts manuels de prévisualisation des paquets, notamment un environnement CEF Linux réservé aux tests rapides du chat sous Linux. |
| `.github/workflows/release.yml` | Publication manuelle des versions GitHub, notamment création de l'environnement CEF Linux généré, génération du manifeste de version et validation finale des ressources. |
| `.github/workflows/staging-release.yml` | Compilation de prépublication manuelle pour un SHA précis, essai à blanc de validation, publication exacte de la préversion et avancement du pointeur propre à la PR. |

<a id="where-to-change-things"></a>
## Où effectuer les modifications

| Tâche | Commencez ici |
| --- | --- |
| Ajouter ou modifier un outil Godot | `fennara-cpp/src/tools/` et `local/schemas/tools/` |
| Modifier le texte d'un schéma MCP | `local/schemas/tools/` |
| Modifier `fennara install` ou `fennara update` | `local/crates/fennara-cli/src/`. La préparation native et l'application ou l'annulation détachées appartiennent à `release_update.rs`, `update_stage.rs`, `update_stage/` et `update_apply/` |
| Modifier les commandes de la CLI ou le comportement du terminal | `local/crates/fennara-cli/src/` et `docs/cli.md` |
| Modifier la progression de mise à jour native, la confirmation d'arrêt, la confirmation d'activation ou la récupération | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` et `ui/chat/` |
| Modifier la première installation native ou l'amorçage de la CLI | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp` et `fennara-cpp/src/ui/dock.cpp` |
| Modifier l'exclusion de l'addon lors de l'exportation | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp` et `godot_demo/tests/export_plugin_test.gd` |
| Modifier les journaux, phases, codes d'erreur ou rapports de diagnostic des opérations d'installation et de mise à jour | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/` et `local/crates/fennara-cli/src/diagnostics.rs` |
| Modifier les vérifications des prérequis de webview | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs` et `fennara-cpp/src/ui/webview_host*` |
| Modifier les instructions de projet générées | `local/templates/` et `local/crates/fennara-cli/src/project_guidance.rs` |
| Synchroniser les instructions générées de l'addon de démonstration | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs` et `godot_demo/addons/fennara/ai/` |
| Modifier la configuration des applications MCP | `local/crates/fennara-cli/src/mcp_setup.rs` et `docs/mcp-setup.md` |
| Modifier la découverte des liaisons de projet MCP ou l'égalité des racines de projet | `local/crates/fennara-project-identity/`, `local/crates/fennara-mcp/src/runtime/`, `local/crates/fennara-daemon/src/runtime_daemon/godot_bridge.rs` et `docs/multi-agent-worktrees.md` |
| Modifier le routage des cibles du daemon ou l'état lié | `local/crates/fennara-daemon/src/runtime_daemon/godot_bridge.rs`, `local/crates/fennara-mcp/src/runtime/` et `docs/multi-agent-worktrees.md` |
| Modifier le comportement des processus et journaux des sessions d'exécution | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/` et `fennara-cpp/src/tool_results/` |
| Modifier la propriété, l'admission ou les baux de l'emplacement d'exécution | `local/crates/fennara-daemon/src/runtime_daemon/runtime_slot.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `fennara-cpp/src/tools/runtime_session/`, `fennara-cpp/src/tools/runtime_script.cpp` et `local/schemas/tools/runtime_session.json` |
| Modifier les auxiliaires ctx, les entrées, instantanés, attentes, raycasts, captures ou le nettoyage de `runtime_script` | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json` et `docs/tools.md` |
| Modifier l'interface de chat intégrée à l'éditeur, les commandes slash ou le sélecteur de modèle ou de fournisseur | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp` et `fennara-cpp/src/ui/webview_host*` |
| Modifier les fournisseurs du chat intégré | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` et `ui/chat/` |
| Modifier les champs, la planification ou les contrôles de confidentialité de la télémétrie anonyme | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` et `docs/telemetry.md` |
| Modifier les bibliothèques fournies avec l'interface de chat | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/` et `THIRD_PARTY_NOTICES.md` |
| Modifier la prise en charge de C# | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/` et les schémas et instructions des outils C# |
| Modifier les paquets publiés, la politique de version minimale de la CLI ou la mise à jour automatique de la CLI | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs` et `.github/workflows/release.yml` |
| Incrémenter la version | `node scripts/set-version.mjs <version>` |
| Mettre à jour l'installation ou la documentation du chat face à MCP, des fournisseurs ou des commandes slash | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md` et `llms.txt` |
| Mettre à jour les traductions de la documentation | Page anglaise canonique, `docs/i18n/languages.json`, pages correspondantes des langues, `scripts/sync-doc-navigation.mjs` et `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## Remarques

- Maintenez ce fichier à jour lorsque vous ajoutez ou déplacez des zones importantes des sources.
- Conservez les étapes de publication dans [release.md](release.md).
- Conservez les étapes d'installation dans [setup.md](setup.md).
- Conservez le comportement des commandes du terminal dans [cli.md](cli.md).
