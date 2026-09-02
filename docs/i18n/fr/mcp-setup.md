<!-- fennara-i18n: locale=fr source=docs/mcp-setup.md sha256=86c9fe3fc7a69c2ade417dd01a0ccabb05ddaa91cf417fa8559c28d4b01811bd -->
<a id="mcp-setup"></a>
# Configuration MCP

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · [Русский](../ru/mcp-setup.md) · **Français** · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

Connectez une application IA externe aux outils Godot de Fennara. L'application
continue d'utiliser son propre compte de modèle, son abonnement ou sa configuration API.

> [!NOTE]
> Cette procédure ne configure pas le chat Fennara intégré. Consultez
> [Applications MCP et chat intégré](chat-vs-mcp.md) si vous ne savez pas de
> quelle voie vous avez besoin.

<a id="quick-setup"></a>
## Configuration rapide

1. Terminez **Set Up Fennara** dans le dock Godot.
2. Ouvrez **Chat Settings > MCP Apps**.
3. Trouvez votre application et appuyez sur **Set Up**.
4. Redémarrez l'application.

Fennara crée une sauvegarde avant de modifier la configuration MCP d'une
application. L'option combinée **Claude** configure Claude Code et Claude Desktop.
**Gemini & Antigravity** configure les deux cibles partagées.

<a id="terminal-alternative"></a>
### Autre méthode depuis le terminal

Exécutez d'abord `fennara install` dans le projet Godot, puis choisissez une cible :

| Application | Commande |
| --- | --- |
| Claude Code et Claude Desktop | `fennara mcp-setup --claude` |
| Claude Code uniquement | `fennara mcp-setup --claude-code` |
| Claude Desktop uniquement | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini et Antigravity | `fennara mcp-setup --gemini` ou `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

Exécutez `fennara mcp-setup --help` pour obtenir la liste des cibles prises en
charge par la CLI que vous avez installée.

La configuration écrit une entrée de lanceur globale et neutre vis-à-vis du
projet. Exécuter `fennara mcp-setup` dans un projet ne lie pas toutes les futures
connexions à ce projet.

<a id="bind-a-connection-to-one-project"></a>
## Lier une connexion à un seul projet

Pour utiliser plusieurs dépôts ou arbres de travail sur la même machine,
exécutez un processus et une connexion MCP par projet. Configurez ce processus
dans les réglages de projet ou d'espace de travail de l'hôte MCP avec soit :

```text
--project-path /absolute/path/to/godot-project
```

soit :

```text
FENNARA_PROJECT_PATH=/absolute/path/to/godot-project
```

L'environnement sélectionne sa liaison de projet une seule fois au démarrage,
dans cet ordre :

1. `--project-path`
2. `FENNARA_PROJECT_PATH`
3. le plus proche ancêtre du répertoire de démarrage contenant `project.godot`
4. le mode de compatibilité non lié lorsque la découverte ne trouve aucun projet

Un chemin explicite non valide empêche le démarrage du serveur MCP. Il ne se
replie jamais sur la cible du dock ni sur un autre éditeur. Une liaison valide
reste active si son éditeur est temporairement absent et récupère la connexion
lorsque cette racine de projet se reconnecte. Il n'existe aucune substitution
de projet par outil destinée au modèle.

Consultez [Plusieurs agents et arbres de travail](multi-agent-worktrees.md) pour
des exemples de configuration, les limites de prise en charge des hôtes, la
vérification de l'état, le comportement des éditeurs en double et la
sérialisation des tests de jeu.

<a id="manual-setup"></a>
## Configuration manuelle

Utilisez la configuration manuelle uniquement si votre application ne figure pas
dans la liste, si la commande de configuration ne trouve pas le fichier de
configuration de l'application ou si vous souhaitez volontairement modifier
la configuration MCP à la main.

Avant toute modification, créez une sauvegarde du fichier de configuration.
Ajoutez ensuite un serveur MCP stdio local nommé `fennara` qui pointe vers le
lanceur MCP stable de Fennara.

Chemins par défaut du lanceur :

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Utilisez le véritable chemin absolu de votre machine. Ne dirigez pas les
applications MCP vers `versions/<version>/fennara-mcp-runtime`. Le lanceur
stable dans `bin/` permet aux configurations des applications de continuer
à fonctionner après les mises à jour de Fennara.

<a id="json-mcpservers"></a>
### JSON `mcpServers`

De nombreuses applications MCP utilisent un objet `mcpServers` de premier niveau :

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

Certaines applications utilisent la même clé `mcpServers`, mais exigent uniquement
`command`. Si la configuration existante contient déjà d'autres serveurs, préservez
ces entrées et ajoutez uniquement le serveur `fennara`.

Pour une entrée locale au projet qui doit rester isolée, ajoutez sa liaison dans
`args` :

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/godot-project"],
      "env": {}
    }
  }
}
```

Les configurations de style Cline peuvent aussi comprendre un délai d'expiration
d'outil plus long, exprimé en secondes :

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### JSON `servers` de style VS Code

Certains clients, dont la configuration MCP utilisateur ou projet de VS Code,
utilisent un objet `servers` de premier niveau et exigent `type: "stdio"` :

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### JSON `mcp` de style OpenCode

La configuration JSON de style OpenCode utilise un objet `mcp` de premier niveau.
Son délai d'expiration est exprimé en millisecondes :

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### TOML de style Codex

Codex utilise TOML :

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Ne collez pas de JSON dans un fichier TOML ni de TOML dans un fichier JSON.
Respectez le format déjà utilisé par l'application.

Pour lier une entrée de style Codex, ajoutez l'argument sans modifier son
lanceur stable :

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/godot-project"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

<a id="common-config-locations"></a>
## Emplacements courants des configurations

Voici les emplacements courants employés par l'auxiliaire de configuration de
Fennara et par les clients MCP actuels. Les applications peuvent modifier leurs
chemins de configuration, et certaines prennent en charge des configurations
globales et locales au projet. Si une application possède une commande comme
**Open MCP Config**, utilisez-la au lieu de deviner.

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

Les espaces de travail à dossier unique de VS Code peuvent fournir le projet
comme répertoire de démarrage du processus enfant MCP. Claude Code, Gemini CLI,
Antigravity, Cline, Cursor, OpenCode, Kiro et Codex peuvent utiliser une
configuration de projet ou d'espace de travail. Utilisez une liaison explicite
ou un répertoire de démarrage de projet documenté lorsque l'isolation doit être
garantie.

Claude Desktop et les versions héritées de Windsurf/Cascade utilisent une
configuration globale pour ce processus. Leur configuration par défaut reste
non liée. Les utilisateurs avancés peuvent créer des entrées globales aux noms
distincts avec différents chemins de projet explicites, mais ces applications
ne fournissent pas d'isolation automatique locale au projet.

<a id="timeout-guidance"></a>
## Conseils relatifs aux délais d'expiration

Certains outils Fennara peuvent dépasser un petit délai d'expiration MCP par
défaut, car ils peuvent demander à Godot de valider des scènes, d'inspecter
l'état d'exécution, de capturer des images ou d'exécuter des diagnostics.

Utilisez un délai par outil plus long lorsque le client le permet :

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

Si un client ne prend pas en charge les délais propres à chaque serveur, utilisez
le réglage de délai d'expiration MCP global documenté par ce client.

<a id="verify-the-connection"></a>
## Vérifier la connexion

Ouvrez le projet Godot, puis demandez à votre application MCP :

```text
Utilise Fennara MCP pour exécuter fennara_status et indique-moi quel projet Godot est connecté.
```

Pour un travail isolé, vérifiez que l'état indique le mode de routage `bound`, la
source attendue de la liaison, la racine de projet canonique, l'état d'éditeur
lié `connected` et la disponibilité du système de fichiers de cet éditeur.

Si l'état indique `legacy_unbound`, la connexion n'a trouvé aucune racine de
projet automatique. Elle utilise la voie de compatibilité de la **MCP target**
du dock et avertit que ce mode n'est pas sûr pour un travail concurrent isolé.

<a id="troubleshooting"></a>
## Dépannage

Si Fennara n'apparaît pas dans l'application MCP :

- vérifiez que le chemin du lanceur est absolu et qu'il existe
- vérifiez que la syntaxe de la configuration est un JSON, JSON5 ou TOML valide selon les exigences de l'application
- vérifiez que le serveur est nommé `fennara`
- vérifiez que l'application lit le fichier de configuration que vous avez modifié
- quittez complètement l'application MCP, puis rouvrez-la
- vérifiez que l'addon Fennara est installé dans le projet Godot
- pour une connexion liée, vérifiez que son chemin explicite ou son répertoire
  de démarrage correspond à la racine de projet Godot voulue
- si l'état indique `bound_project_not_connected`, ouvrez ce projet dans Godot
  et attendez que l'addon se connecte
- si l'état indique `ambiguous_project_binding`, fermez l'éditeur en double ou
  ouvrez-le depuis un arbre de travail distinct
- pour une connexion non liée héritée, vérifiez que le projet voulu est
  sélectionné comme cible MCP dans le dock

<a id="unsupported-mcp-apps"></a>
## Applications MCP non prises en charge

Si votre application MCP ne figure pas dans la liste, trouvez d'abord l'emplacement
et le format officiels de sa configuration MCP. Demandez ensuite à un LLM de
proposer la plus petite modification sûre :

```text
Je dispose d'un exécutable de serveur MCP stdio local à l'emplacement :
<paste the full path to fennara-mcp here>

Je veux l'ajouter à <app name>.
Le fichier de configuration MCP de l'application est :
<paste config path here>

Le format de la configuration est <JSON/TOML/YAML/etc>.

Montrez-moi la plus petite modification sûre pour ajouter un serveur nommé "fennara".
Préservez toute la configuration existante. Si l'application exige "mcpServers",
"servers", "mcp" ou une autre clé de premier niveau, utilisez la clé imposée par
la documentation officielle de cette application.
```

Relisez le résultat avant de l'enregistrer, puis redémarrez l'application MCP.
