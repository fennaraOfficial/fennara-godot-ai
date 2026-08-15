<!-- fennara-i18n: locale=fr source=local/README.md sha256=b63443a8ff7310d53044b20261e7f206a949bad07299656235ba39fe69f92d9d -->
<a id="fennara-local-tools"></a>
# Outils locaux de Fennara

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · **Français** · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Ce dossier contient les composants natifs locaux de Fennara.

<a id="daemon"></a>
## Daemon

`crates/fennara-daemon` exécute le daemon Fennara local sur :

```text
http://127.0.0.1:41287
```

Points d'accès :

- `GET /health` : état du daemon.
- `GET /status` : état du daemon et métadonnées du plugin Godot connecté.
- état lié privilégié : résout la racine de projet canonique d'un processus MCP
  par rapport aux sessions d'éditeur Godot connectées.
- `POST /tools/call` : transmet un appel d'outil au plugin Godot connecté et attend son résultat.
- `WS /godot/ws` : pont du plugin Godot local. Le plugin envoie un message `hello` après la connexion.

Un seul daemon est partagé par tous les éditeurs utilisant Fennara et tous les
processus MCP externes de l'utilisateur actuel. Les requêtes externes liées sont
dirigées selon leur racine de projet canonique ; les requêtes internes du chat
intégré restent liées à leur session d'éditeur Godot, et les requêtes MCP non
liées héritées utilisent la cible de compatibilité sélectionnée dans le dock.

Le daemon possède également un seul emplacement d'exécution à l'échelle de la
machine. La propriété de la session d'exécution et l'état de son bail
renouvelable sont associés à une racine de projet, afin qu'un éditeur puisse se
reconnecter sans transférer le contrôle.

Binaire de développement :

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## Serveur MCP

`crates/fennara-mcp` est le serveur MCP local. Il communique en JSON-RPC sur stdio afin que les clients MCP puissent le lancer comme processus local.

Chaque processus MCP fige une liaison de projet facultative au démarrage. La
sélection utilise `--project-path`, puis `FENNARA_PROJECT_PATH`, puis le plus
proche ancêtre du répertoire de démarrage contenant `project.godot`. Si aucun
projet n'est trouvé, le processus entre automatiquement dans le mode de
compatibilité non lié ; un chemin explicite non valide fait échouer le
démarrage. Utilisez un processus et une connexion MCP par projet pour isoler
les projets entre eux.

`crates/fennara-project-identity` est partagé par l'environnement MCP et le
daemon. Il possède la découverte, la validation et la canonicalisation des
racines de projet, leur conversion sans perte pour le protocole et la
comparaison de leur identité active dans le système de fichiers.

`fennara-mcp` intègre à la compilation les schémas destinés à MCP qu'il a
sélectionnés depuis `local/schemas/tools/` et transmet les appels de ces outils
au daemon local. Il n'a pas besoin d'un service de schémas externe à l'exécution.
Le chat intégré sélectionne dans le même répertoire de schémas un ensemble
d'outils lié, mais différent.

`fennara install` écrit également dans le projet Godot les instructions de projet générées depuis `local/templates/` :

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

Compilation :

```powershell
cd local
cargo build
```

Sous Windows, si un terminal n'a pas encore actualisé le PATH de Rust :

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Binaire de développement :

```text
local/target/debug/fennara-mcp.exe
```

Outils actuels :

- `fennara_status` : vérifie que le serveur MCP est installé et accessible,
  puis indique le mode de routage, la source et la racine de la liaison, l'état
  de l'éditeur sélectionné et la disponibilité du pont Godot lorsque le daemon
  fonctionne.
- Les outils de projet Godot comme `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` et `screenshot_scene` sont transmis
  au daemon, qui les transmet au plugin Godot connecté.

Futur chemin utilisateur installé sous Windows :

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
