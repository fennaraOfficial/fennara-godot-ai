<!-- fennara-i18n: locale=fr source=docs/multi-agent-worktrees.md sha256=037571da5f2479b968f6b74e8994d0272042d4a26616c50af5f62f3d2a9bbcf9 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# Plusieurs agents et arbres de travail Godot

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · [Español](../es/multi-agent-worktrees.md) · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · [日本語](../ja/multi-agent-worktrees.md) · [한국어](../ko/multi-agent-worktrees.md) · [Русский](../ru/multi-agent-worktrees.md) · **Français** · [Deutsch](../de/multi-agent-worktrees.md) · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

Exécutez plusieurs agents de programmation dans des dépôts ou des arbres de
travail distincts sur une même machine sans que le choix de cible d'un agent ne
redirige un autre agent. Chaque projet reçoit son propre processus et sa propre
connexion MCP Fennara ; tous les projets partagent le même daemon par utilisateur.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

Les appels de modification, d'inspection, de validation et de capture d'écran
peuvent s'exécuter simultanément. Les exécutions de jeu gérées par le daemon sont
sérialisées au moyen d'un seul emplacement d'exécution à l'échelle de la machine.

<a id="one-mcp-connection-per-project"></a>
## Une connexion MCP par projet

Un processus MCP sélectionne une racine de projet stable à son démarrage. Cette
liaison de projet MCP est l'identité canonique, dans le système de fichiers, du
répertoire contenant `project.godot` ; il ne s'agit ni d'un nom de projet ni
d'un identifiant de processus Godot.

Utilisez un processus et une connexion MCP distincts pour chaque dépôt ou arbre
de travail. Une connexion peut servir plusieurs agents uniquement lorsque tous
travaillent volontairement sur le même projet. Les outils Fennara n'exposent
aucun sélecteur de projet par appel ; le modèle ne peut donc pas faire basculer
accidentellement un processus vers un autre projet.

Chaque projet exige également un éditeur Godot connecté dans lequel Fennara est
activé. Si un éditeur se ferme puis se reconnecte avec un nouvel identifiant de
processus, le processus MCP existant reprend son routage lorsque la même racine
de projet se reconnecte.

<a id="how-a-process-chooses-its-project"></a>
## Comment un processus choisit son projet

L'environnement MCP capture son répertoire de travail de démarrage et choisit
sa liaison une seule fois, dans cet ordre :

1. `--project-path <path>` ou `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. Le plus proche ancêtre du répertoire de démarrage contenant `project.godot`.
4. Le mode de compatibilité non lié lorsque la découverte automatique ne trouve
   aucun projet Godot.

Les chemins indiqués sur la ligne de commande ou dans l'environnement sont des
assertions explicites. Un chemin vide, inaccessible, absent, qui n'est pas un
répertoire, qui n'est pas un projet Godot ou qui n'est pas pris en charge
empêche le démarrage du serveur MCP ; celui-ci ne se replie jamais sur un autre
projet. Les chemins relatifs sont résolus depuis le répertoire de démarrage
capturé. Préférez un chemin absolu lorsque le répertoire de lancement de l'hôte
MCP n'est pas clair.

Fennara ne lit pas implicitement les variables d'espace de travail propres à un
hôte. Un hôte MCP peut transmettre sa propre valeur d'espace de travail à
`--project-path` ou à `FENNARA_PROJECT_PATH`.

<a id="configure-a-project-bound-connection"></a>
## Configurer une connexion liée à un projet

`fennara mcp-setup` reste global et neutre vis-à-vis du projet. L'exécuter dans
un projet ne lie pas tous les futurs processus MCP à ce projet. Conservez le
chemin stable de son lanceur, puis utilisez la configuration de projet ou
d'espace de travail de l'hôte MCP pour ajouter une liaison.

Pour une configuration de style JSON :

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

Ou utilisez l'environnement :

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Pour un fichier TOML de style Codex :

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Configurez l'agent suivant dans sa propre configuration de projet ou d'espace
de travail avec `/absolute/path/to/worktree-b`. Si un hôte démarre de manière
fiable un processus MCP distinct depuis chaque répertoire de projet, la
découverte des ancêtres peut fournir la même liaison sans chemin explicite.

<a id="mcp-host-boundaries"></a>
## Limites des hôtes MCP

Le comportement de la configuration locale au projet et du répertoire de
démarrage varie selon l'hôte :

- Les espaces de travail à dossier unique de VS Code peuvent s'appuyer sur le
  répertoire de travail enfant documenté par l'hôte, mais une liaison de projet
  explicite reste la configuration la plus claire.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro et Codex
  peuvent utiliser une configuration de projet ou d'espace de travail. Utilisez
  une liaison explicite ou un répertoire de démarrage de projet documenté
  lorsque l'isolation doit être garantie.
- La configuration de Claude Desktop et des versions héritées de
  Windsurf/Cascade est globale. Leur entrée Fennara par défaut reste non liée et
  ne peut pas assurer une isolation automatique locale au projet. Les
  utilisateurs avancés peuvent créer des entrées globales aux noms distincts
  avec des chemins explicites différents, mais doivent choisir la bonne entrée.

La génération automatique de configuration locale au projet et la nouvelle
prise en charge de Windsurf/Devin Local sortent du cadre de ce processus.

<a id="start-and-verify-the-editors"></a>
## Démarrer et vérifier les éditeurs

Chaque arbre de travail exige son propre éditeur Godot dans lequel Fennara est
activé. Les éditeurs sans interface peuvent utiliser des ports LSP Godot
distincts tout en partageant le daemon de Fennara :

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

Les ports LSP appartiennent à Godot. Fennara continue d'utiliser un daemon
partagé unique à son adresse de bouclage habituelle.

Exécutez `fennara_status` depuis chaque agent avant tout travail concurrent.
Vérifiez qu'il indique :

- le mode de routage `bound`
- la source attendue de la liaison et la racine de projet canonique
- l'état d'éditeur lié `connected`
- la disponibilité du système de fichiers de cet éditeur

Si la découverte automatique n'a trouvé aucun projet, l'état indique
`legacy_unbound` et un avertissement relatif au travail concurrent. Dans ce
mode de compatibilité, la cible MCP sélectionnée dans le dock est utilisée en
premier, puis l'unique éditeur connecté. N'utilisez pas de connexion non liée
pour un travail concurrent isolé.

<a id="missing-and-duplicate-editors"></a>
## Éditeurs absents ou en double

Une liaison de projet valide reste active lorsque son éditeur est absent. Les
appels d'outil renvoient l'erreur `bound_project_not_connected`, qui autorise
une nouvelle tentative, jusqu'à la reconnexion de cette racine de projet ; ils
ne se replient jamais sur la cible du dock.

Deux éditeurs qui se résolvent vers la même racine de projet produisent
`ambiguous_project_binding`. Fermez l'éditeur en double ou donnez-lui un arbre
de travail distinct. Fennara ne choisit ni selon l'identifiant de processus, ni
selon l'ordre de connexion, le nom du projet ou la cible du dock.

Les alias de liens symboliques vers le même projet se résolvent vers la même
identité active du système de fichiers. Rediriger un lien symbolique après le
démarrage de MCP ne modifie pas une liaison ; redémarrez ce processus MCP pour
établir une nouvelle liaison.

<a id="serialized-runtime-sessions"></a>
## Sessions d'exécution sérialisées

Tous les projets partagent un seul emplacement d'exécution à l'échelle de la
machine pour les exécutions de jeu gérées par le daemon. Lorsqu'un autre projet
démarre ou exécute une session, `runtime_session.start` renvoie un résultat de
domaine `busy` réussi avec `availability: "busy"`, `slot_acquired: false` et une
valeur `retry_after_ms` suggérée. Il ne révèle ni le propriétaire, ni
l'identifiant de la session, l'identifiant du processus, la scène, les journaux,
la position dans une file ou la durée prévue.

Il n'existe aucune file FIFO. Interrogez avec une gigue proche du délai de
nouvelle tentative suggéré et considérez chaque appel de `runtime_session.start`
comme la revendication atomique définitive. Un état libre n'est qu'indicatif,
car un autre agent peut remporter la course après la vérification préalable.

Seule la racine de projet propriétaire peut inspecter, renouveler, scripter ou
arrêter sa session d'exécution. L'état demandé par le propriétaire renouvelle un
délai d'inactivité de 120 secondes. Une opération d'exécution bornée du
propriétaire suspend l'expiration pour inactivité tant qu'elle est active et ne
renouvelle le délai qu'après avoir renvoyé un résultat terminal du script ; un
dépassement de délai, une erreur de préparation ou une annulation ne le
renouvelle pas. Les agents doivent interroger l'état du propriétaire environ
toutes les 30 secondes, avec une gigue, pendant l'exécution.

Le bail d'exécution absolu par défaut dure 900 secondes. `max_run_seconds`
accepte un entier positif allant jusqu'à 86 400 secondes ; par exemple, une
régression qui devrait durer une heure peut demander 4 500 secondes pour
conserver une marge de sécurité. L'échéance absolue n'est jamais suspendue. Une
fin naturelle, un arrêt explicite, un échec du démarrage, une période
d'inactivité ou l'expiration absolue arrête ou récolte le jeu et libère
l'emplacement d'exécution.

<a id="safe-multi-agent-checklist"></a>
## Liste de contrôle sûre pour plusieurs agents

1. Créez un dépôt ou un arbre de travail distinct pour chaque projet.
2. Installez Fennara et ouvrez un éditeur Godot pour chaque racine de projet.
3. Configurez un processus MCP lié au projet pour chaque projet.
4. Exécutez `fennara_status` depuis chaque agent et vérifiez sa racine canonique.
5. Laissez la modification, l'inspection, la validation bornée des scènes et les
   captures d'écran autonomes s'effectuer simultanément.
6. Interrogez et retentez les résultats `busy` qui ne sont pas des erreurs pour
   les tests de jeu ; maintenez la session gagnante active en interrogeant son
   état propriétaire pendant son exécution.
