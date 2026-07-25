<!-- fennara-i18n: locale=fr source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Русский](README.ru.md) · **Français** · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Démos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/fr/demos.md)
[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Utilisé par des développeurs et des équipes Godot, notamment [Somni Game Studios](https://somnigamestudios.com/).

Fennara offre aux assistants IA une connexion en direct à Godot. Utilisez-le depuis des applications compatibles MCP comme Codex, Claude, Cursor, Gemini et Antigravity, ou depuis le dock de chat facultatif intégré à l'éditeur.

Les agents peuvent inspecter les scènes, vérifier les scripts, capturer des images, lire les erreurs d'exécution et valider les modifications dans l'éditeur au lieu de se fier uniquement aux fichiers du projet.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Comparaison de Fennara avec d'autres MCP pour Godot" width="100%" />
      </a>
    </td>
    <td>
      <strong>Regarder la démo à la une</strong><br />
      Comparaison de Fennara avec d'autres MCP pour Godot.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Lire cette vidéo</a><br />
      <a href="docs/i18n/fr/demos.md">Parcourir toutes les vidéos de démonstration</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## Fonctionnalités

- expose des outils qui comprennent Godot aux applications IA externes via MCP
- ajoute un dock de chat local facultatif dans l'éditeur Godot
- renvoie de véritables informations de Godot : arbres de scène, diagnostics, captures d'écran, journaux d'exécution et résultats de validation
- oblige l'agent à tenir compte de l'éditeur ouvert au lieu de se limiter au système de fichiers

Les applications MCP externes et le chat intégré utilisent des réglages de modèle distincts. Consultez [Applications MCP et chat intégré](docs/i18n/fr/chat-vs-mcp.md) et [Fournisseurs du chat intégré](docs/i18n/fr/providers.md).

<a id="requirements"></a>
## Prérequis

- Godot 4.5 ou version ultérieure.
- Un système d'exploitation de bureau pris en charge : Windows x86_64, Linux x86_64 ou macOS arm64.
- Une application de programmation compatible MCP uniquement si vous souhaitez utiliser Fennara depuis Claude, Codex, Cursor, Gemini, Antigravity ou une autre application IA externe.
- Un fournisseur de chat uniquement si vous souhaitez utiliser le dock de chat intégré de Fennara. Il peut s'agir d'une clé de fournisseur cloud ou d'un fournisseur local comme Ollama ou LM Studio.

Pour la procédure d'installation complète, consultez [Installation](docs/i18n/fr/setup.md).

<a id="what-setup-adds"></a>
## Ce que l'installation ajoute

- l'addon Fennara conservé dans `res://addons/fennara/`
- une petite CLI `fennara` installée dans les données d'application de Fennara
- un serveur MCP local utilisé par les applications de programmation IA
- un daemon local qui transmet les requêtes MCP et de chat à l'éditeur Godot ouvert
- des instructions de projet générées pour les agents IA

Le dock de chat intégré utilise la webview de la plateforme : Microsoft Edge WebView2 sous Windows, WKWebView/WebKit sous macOS et un environnement d'exécution CEF partagé géré par Fennara sous Linux. Les outils MCP continuent de fonctionner si le dock de chat facultatif ne peut pas démarrer.

<a id="install"></a>
## Installation

Sous Windows et Linux, choisissez l'installation par l'addon ou par la CLI. Sous macOS, utilisez
l'installation par la CLI ci-dessous si vous voulez éviter la notification de sécurité de macOS qui peut
apparaître après le téléchargement et l'extraction manuels du fichier ZIP de l'addon.

<a id="add-the-addon-to-your-project"></a>
### Ajouter l'addon à votre projet

- Ouvrez la [dernière version publiée](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest), téléchargez `fennara-addon-latest.zip`, puis extrayez son dossier `addons/fennara/` dans votre projet.

Ouvrez le projet, sélectionnez le dock Fennara et appuyez sur **Set Up Fennara**.

Fennara est une dépendance de l'éditeur, pas une dépendance du jeu à l'exécution.
Pendant l'exportation, le plugin de l'éditeur retire son autoload d'exécution du
projet exporté et ignore `res://addons/fennara/` ainsi que `res://.fennara/`.
Le projet de l'éditeur est restauré une fois l'exportation terminée. Si une copie
de travail CI exclut l'addon au moyen de `.gitignore`, exécutez
`fennara prepare-export --project path/to/project` avant de démarrer Godot, ou
installez l'addon dans cette copie de travail. Godot valide les chemins des
autoloads avant que les plugins d'exportation puissent s'exécuter. Cette
préparation doit donc avoir lieu en premier.

> **macOS :** l'addon de la version publiée contient une bibliothèque native qui n'est
> actuellement pas notariée par Apple. Si vous téléchargez le fichier ZIP de l'addon
> dans un navigateur et l'extrayez manuellement, macOS peut signaler qu'il ne peut pas
> vérifier que `libfennara.macos.editor` est exempt de logiciels malveillants. Pour
> éviter cette notification, utilisez l'installation par la CLI ci-dessous. Si la
> notification est déjà affichée, fermez Godot, supprimez le dossier
> `addons/fennara/` copié manuellement, puis installez Fennara avec la CLI.

<a id="install-with-the-cli-recommended-on-macos"></a>
### Installer avec la CLI (recommandé sous macOS)

La CLI installe le même addon Fennara. Il s'agit de la méthode d'installation
recommandée sous macOS, car elle évite le chemin de quarantaine du navigateur et
du Finder qui provoque la notification décrite ci-dessus.

Installez la CLI sous Windows :

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Ou sous macOS et Linux :

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Exécutez ensuite Fennara depuis votre projet Godot :

```bash
cd path/to/your-godot-project
fennara install
```

Consultez [Installation](docs/i18n/fr/setup.md) pour le dépannage et [CLI Fennara](docs/i18n/fr/cli.md)
pour la référence complète des commandes.

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## Configurer un fournisseur ou connecter une application MCP

<a id="built-in-chat"></a>
### Chat intégré

Ouvrez **Chat Settings > Chat**, sélectionnez **Open providers**, puis connectez un fournisseur.
Fennara utilise votre propre clé pour les fournisseurs cloud (BYOK). Vous pouvez aussi utiliser
un serveur Ollama ou LM Studio local. Consultez la [liste des fournisseurs pris en charge](docs/i18n/fr/providers.md).

<a id="mcp-apps"></a>
### Applications MCP

Ouvrez **Chat Settings > MCP Apps**, trouvez votre application et appuyez sur **Set Up**.

Vous pouvez également connecter une application depuis le terminal :

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Si votre application MCP ne figure pas dans Chat Settings, consultez [Configuration MCP](docs/i18n/fr/mcp-setup.md)
pour obtenir la liste complète des applications et les instructions de configuration manuelle.

<a id="update"></a>
## Mise à jour

Lorsque le dock Fennara affiche **Update**, appuyez dessus et suivez les instructions.

> **Mise à niveau depuis Fennara v0.3.8 ou une version antérieure :** réinstallez
> une fois la CLI avec la commande d'installation de votre plateforme ci-dessus
> avant d'exécuter `fennara update`. Ces versions de la CLI résolvent une étiquette
> de version publiée retirée et ne peuvent pas découvrir les versions actuelles.
> La réinstallation de la CLI fait basculer les futures mises à jour vers le point
> d'accès Latest Release de GitHub et ne supprime ni l'addon ni les réglages existants
> de votre projet.

> **Utilisateurs de macOS mettant à niveau depuis Fennara v0.3.11 :** réinstallez
> une fois la CLI avec la commande d'installation macOS ci-dessus avant la mise à
> jour. La CLI v0.3.11 rejette le bundle de framework macOS existant avant de pouvoir
> se mettre à jour elle-même. La réinstallation remplace uniquement la CLI. Elle ne
> supprime ni l'addon ni les réglages de votre projet.

Pour effectuer la mise à jour depuis le terminal, fermez Godot et exécutez :

```bash
cd path/to/your-godot-project
fennara update
```

Consultez [Mettre à jour Fennara](docs/i18n/fr/setup.md#update-fennara) pour la récupération et les diagnostics.

<a id="tools"></a>
## Outils

Fennara expose un petit ensemble d'outils qui comprennent Godot :

- écrire ou mettre à jour les fichiers du projet et renvoyer les diagnostics
- exécuter des scripts ponctuels de modification de scène
- inspecter les arbres de scène, les nœuds, les ressources et les classes Godot
- valider les scènes
- capturer des images
- démarrer des sessions d'exécution et lire leurs journaux
- exécuter de petits scripts d'exécution sur une scène active

L'objectif n'est pas de remplacer les outils de fichiers habituels d'un agent. Fennara fournit la boucle de rétroaction Godot qui lui manque.

<a id="privacy"></a>
## Confidentialité

Fennara envoie au maximum un événement anonyme d'installation active par jour UTC
après la connexion de Godot. Il contient un UUID d'installation aléatoire, les versions
de Fennara et de Godot, le système d'exploitation et l'architecture du processeur. Il ne
contient ni données de projet, ni chemins, ni prompts, ni activité d'outil, ni journaux,
ni captures d'écran, ni informations de compte.

La télémétrie peut être désactivée dans **Chat Settings > Chat > Anonymous telemetry**,
avec `FENNARA_DISABLE_TELEMETRY=true` ou avec `DO_NOT_TRACK=1`. Consultez [Télémétrie
anonyme](docs/i18n/fr/telemetry.md) pour connaître la charge utile complète, le stockage,
le transport et les modalités de désactivation.

<a id="demos"></a>
## Démos

Regardez une présentation pratique de Fennara :

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

Autres vidéos :

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

Consultez [Démos](docs/i18n/fr/demos.md) pour d'autres vidéos de la chaîne Fennara.

<a id="star-history"></a>
## Historique des étoiles
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Graphique de l'historique des étoiles" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## Documentation

| Commencez par... | Lorsque vous avez besoin de... |
| --- | --- |
| [Accueil de la documentation](docs/i18n/fr/README.md) | Tous les guides et toutes les pages de référence |
| [Installation](docs/i18n/fr/setup.md) | Installation, mises à jour et dépannage |
| [Fournisseurs de chat](docs/i18n/fr/providers.md) | Modèles et clés du chat intégré |
| [Configuration MCP](docs/i18n/fr/mcp-setup.md) | Codex, Claude, Cursor et les autres applications MCP |
| [Outils](docs/i18n/fr/tools.md) | Les informations Godot accessibles aux agents |
| [Télémétrie anonyme](docs/i18n/fr/telemetry.md) | Données collectées, comportement d'envoi et options de désactivation |
| [Contribuer](docs/i18n/fr/CONTRIBUTING.md) | Conseils de développement et de pull request |

<a id="community"></a>
## Communauté

Les questions, l'aide à l'installation et les premiers retours sont les bienvenus sur Discord :

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## Licence

Consultez [LICENSE.md](LICENSE.md).
