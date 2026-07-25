<!-- fennara-i18n: locale=fr source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
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

Si plusieurs projets Godot sont ouverts, utilisez le contrôle **MCP target** du
dock Fennara pour sélectionner le projet qui reçoit les appels d'outils MCP externes.

<a id="troubleshooting"></a>
## Dépannage

Si Fennara n'apparaît pas dans l'application MCP :

- vérifiez que le chemin du lanceur est absolu et qu'il existe
- vérifiez que la syntaxe de la configuration est un JSON, JSON5 ou TOML valide selon les exigences de l'application
- vérifiez que le serveur est nommé `fennara`
- vérifiez que l'application lit le fichier de configuration que vous avez modifié
- quittez complètement l'application MCP, puis rouvrez-la
- vérifiez que l'addon Fennara est installé dans le projet Godot
- vérifiez que le projet Godot voulu est sélectionné comme cible MCP

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
