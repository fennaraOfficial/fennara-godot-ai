<!-- fennara-i18n: locale=fr source=docs/cli.md sha256=561efbe26f75accb949050e7af9d06ae607040b86d061c04c4dcf8732ba61135 -->
<a id="fennara-cli"></a>
# CLI Fennara

<!-- fennara-doc-nav:start -->
[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · [Español](../es/cli.md) · [Português do Brasil](../pt-BR/cli.md) · [日本語](../ja/cli.md) · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · **Français** · [Deutsch](../de/cli.md) · [Türkçe](../tr/cli.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../cli.md)
<!-- fennara-doc-nav:end -->

Utilisez la CLI si vous préférez le terminal, si vous avez besoin de diagnostics
ou d'une récupération, ou si vous souhaitez automatiser l'installation d'une
version précise.

> [!TIP]
> La CLI est la méthode d'installation recommandée sous macOS. Elle évite la
> notification de sécurité de macOS qui peut apparaître lorsqu'un fichier ZIP
> d'addon téléchargé dans un navigateur est extrait manuellement et que sa
> bibliothèque native hérite de la quarantaine de Finder.

<a id="common-flow"></a>
## Processus courant

```bash
cd path/to/your-godot-project
fennara install
```

Utilisez `fennara doctor` lorsque vous devez inspecter ou réparer l'installation locale.

Utilisez [Installation](setup.md) pour le parcours normal dans Godot. Conservez cette
page comme référence des commandes de terminal.

<a id="install-the-cli"></a>
## Installer la CLI

Windows :

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS et Linux :

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Si un addon macOS extrait manuellement provoque déjà une notification pour
`libfennara.macos.editor`, fermez Godot et supprimez le dossier `addons/fennara/`
copié manuellement avant d'exécuter `fennara install`. Dans les autres cas, la CLI
préserve un addon complet existant.

Ouvrez un nouveau terminal si `fennara` n'est pas immédiatement disponible, puis
vérifiez l'installation :

```bash
fennara --version
fennara doctor
```

La CLI est installée pour chaque utilisateur. Les addons des projets restent dans
leurs projets Godot. Les lanceurs partagés, les environnements d'exécution versionnés,
les enregistrements d'opérations, les journaux et CEF sous Linux restent dans les
données d'application de Fennara :

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## Résumé des commandes

| Commande | Objectif |
| --- | --- |
| `fennara install` | Installer ou adopter l'addon d'un projet et ses composants locaux correspondants |
| `fennara update` | Mettre à jour un projet et ses composants locaux |
| `fennara doctor` | Inspecter ou réparer l'installation locale |
| `fennara diagnostics` | Afficher un rapport d'opération nettoyé |
| `fennara mcp-setup` | Connecter une application MCP externe |
| `fennara prepare-export` | Retirer l'autoload de Fennara avant une exportation CI sans addon |
| `fennara recover` | Restaurer une mise à jour native interrompue |
| `fennara self-update` | Mettre à jour uniquement la CLI installée |

Exécutez `fennara --help` pour obtenir le résumé des commandes installées. Utilisez
`fennara mcp-setup --help` pour connaître les cibles d'applications MCP prises en charge.

<a id="install-a-project"></a>
## Installer un projet

Exécutez la commande dans un dossier contenant `project.godot` :

```bash
fennara install
```

Ou indiquez explicitement le projet :

```bash
fennara install --project path/to/project
```

Sans `--version`, la CLI sélectionne le manifeste de la version actuelle. Utilisez
une version publiée précise lorsque la reproductibilité est importante :

```bash
fennara install --project path/to/project --version <version>
```

L'installation possède deux parcours sûrs :

- Si aucun addon complet n'existe, la CLI télécharge et vérifie la version
  sélectionnée, installe `addons/fennara`, installe les composants locaux
  correspondants et écrit les instructions de projet Fennara.
- Si un addon complet existe déjà, la CLI lit son fichier `VERSION`, valide la
  bibliothèque de la plateforme actuelle et installe les composants gérés par
  la CLI de cette version précise. Elle laisse l'addon du projet inchangé. Une
  option `--version` explicite doit correspondre à l'addon existant.

<a id="prepare-an-addon-free-ci-export"></a>
## Préparer une exportation CI sans addon

Si `addons/fennara/` est exclu d'une copie de travail CI, retirez l'autoload
d'exécution persistant de Fennara avant le démarrage de Godot :

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

La commande ne modifie que l'entrée `_fennara_game_capture` dans
`project.godot`. Elle conserve les autres autoloads et paramètres, et peut être
réexécutée sans risque. Cette étape doit avoir lieu avant le démarrage de Godot,
car le lancement du projet valide les chemins des autoloads avant que les
plugins de l'éditeur ou d'exportation puissent s'exécuter. À la place, le
système CI peut installer l'addon Fennara avant de démarrer Godot.

<a id="update-a-project"></a>
## Mettre à jour un projet

Pour une mise à jour normale depuis le terminal, fermez Godot pour ce projet et exécutez :

```bash
fennara update --project path/to/project
```

Sans `--version`, la CLI lit l'identité de l'addon installé. Les addons stables
résolvent la dernière version de GitHub, tandis que les addons de prépublication
résolvent uniquement leur canal `pr-<number>`. Le sélecteur est immédiatement
figé sur une version précise, y compris pendant le remplacement automatique de
la CLI. La CLI vérifie ensuite les ressources publiées, actualise l'addon et les
composants locaux versionnés, met à jour les instructions du projet et vérifie
le prérequis de webview de la plateforme. Utilisez `--version <version>` pour
sélectionner explicitement une version publiée précise.

`--no-self-update` est destiné à une automatisation contrôlée ou à la poursuite
d'une opération après le remplacement de la CLI. Ne l'utilisez pas pour contourner
l'exigence de version minimale de la CLI d'une version publiée.

> [!IMPORTANT]
> Si vous effectuez la mise à niveau depuis Fennara v0.3.8 ou une version
> antérieure, réinstallez une fois la CLI avec la commande d'installation de
> votre plateforme indiquée dans [Installation](setup.md#install-from-the-terminal-recommended-on-macos)
> avant d'exécuter `fennara update`. Ces CLI interrogent une étiquette de version
> retirée et ne peuvent pas découvrir les versions actuelles. La réinstallation
> de la CLI ne supprime ni l'addon ni les réglages de votre projet.

> [!IMPORTANT]
> Sous macOS, réinstallez une fois la CLI avant d'effectuer la mise à niveau depuis
> Fennara v0.3.11. Cette CLI rejette le bundle de framework existant avant d'atteindre
> sa propre mise à jour. La réinstallation remplace uniquement la CLI et préserve
> l'addon ainsi que les réglages du projet.

<a id="prepare-while-godot-is-open"></a>
### Préparer pendant que Godot est ouvert

Le bouton de mise à jour intégré à l'éditeur utilise la forme de préinstallation :

```bash
fennara update --prepare --project path/to/project
```

La préparation télécharge, vérifie et met durablement l'addon en attente. Elle ne
ferme pas Godot, ne remplace pas l'addon actif, ne bascule pas le manifeste
d'exécution actif et ne redémarre pas le daemon. Le dock Godot observe le reçu de
l'opération et demande l'autorisation de l'utilisateur avant de lancer l'étape
détachée de fermeture, de remplacement, de réouverture et de validation. Le dock
transmet la version précise qu'il a déjà découverte. Le déplacement du pointeur
ne peut donc pas modifier une mise à jour en cours.

Fennara prend en charge une seule version active de l'environnement d'exécution
partagé à la fois. L'activation est bloquée si un autre éditeur Godot compatible
Fennara reste connecté au daemon partagé. Fermez l'autre éditeur, puis réessayez.
La version locale précédente et le pointeur d'exécution restent disponibles pour
une récupération sans accès au réseau.

`--prepare` est une primitive de bas niveau destinée à l'intégration Godot. Les
utilisateurs du terminal emploient normalement `fennara update` après avoir fermé Godot.

<a id="recover-an-interrupted-update"></a>
## Récupérer une mise à jour interrompue

Si l'addon mis à jour ne parvient pas à se charger suffisamment pour afficher le
panneau de récupération, fermez Godot et exécutez :

```bash
fennara recover --project path/to/project
```

La CLI restaure uniquement les opérations dans un état récupérable. Elle restaure
l'addon précédent, les lanceurs partagés et le manifeste d'exécution actif, puis
tente de rouvrir l'exécutable Godot enregistré. Sélectionnez une transaction
précise lorsque l'assistance vous communique son identifiant d'opération :

```bash
fennara recover --project path/to/project --operation <operation-id>
```

Les opérations terminées, simplement préparées ou déjà annulées sont refusées.

<a id="inspect-health-and-failures"></a>
## Inspecter l'état et les échecs

`doctor` indique la plateforme détectée, la disposition des données d'application,
la version active, les lanceurs, les environnements d'exécution, l'état du daemon
et le prérequis de la webview :

```bash
fennara doctor
```

S'il signale un daemon ou un environnement d'exécution MCP plus ancien que
`current.json`, redémarrez Godot ou l'application MCP concernée afin qu'il lance
l'environnement d'exécution sélectionné.

Utilisez `--repair` pour recréer les répertoires de base manquants des données
d'application. Sous Linux, cette option nettoie également les profils de processus
CEF obsolètes et répare le marqueur d'environnement actuel lorsqu'un environnement
géré complet est déjà installé :

```bash
fennara doctor --repair
```

Les opérations d'installation, de mise à jour, de récupération et de mise à jour
automatique écrivent un état et des événements durables. Affichez le rapport
nettoyé le plus récent avec :

```bash
fennara diagnostics
```

Pour une opération plus ancienne ou une sortie exploitable par une machine :

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

Les rapports comprennent des codes d'erreur stables, les phases, les versions
des composants, les noms des ressources sélectionnées et les résultats de
vérification des hachages. Ils masquent les chemins du projet, du dossier personnel
et des données d'application de Fennara, les identifiants, les jetons bearer et
les paramètres des URL. Ils ne contiennent ni messages de chat, ni clés de
fournisseur, ni contenu des fichiers du projet.

<a id="configure-an-external-mcp-app"></a>
## Configurer une application MCP externe

Le dock de chat Godot expose ces commandes sous **Chat Settings > MCP Apps**.
Son bouton Set Up demande au daemon local d'appeler la CLI installée. Les processus
du dock et du terminal utilisent donc la même implémentation de configuration et
de sauvegarde.

Exécutez `fennara mcp-setup --help` pour choisir une cible prise en charge.
Redémarrez l'application MCP après avoir modifié sa configuration. Cette commande
connecte une application externe au serveur MCP Fennara. Elle ne sélectionne pas
le fournisseur de modèle utilisé par le dock de chat Godot intégré.
[Configuration MCP](mcp-setup.md) définit la liste des cibles, les emplacements
des configurations et les exemples de configuration manuelle.

<a id="update-only-the-cli"></a>
## Mettre à jour uniquement la CLI

Les mises à jour normales des projets gèrent automatiquement la mise à jour de
la CLI elle-même. Pour mettre à jour uniquement la CLI installée :

```bash
fennara self-update
fennara self-update --version <version>
```

Sans `--version`, la mise à jour automatique préserve la piste de l'installation
active : la piste stable utilise la dernière version de GitHub et la piste de
prépublication utilise uniquement le canal de PR enregistré.

Une prépublication ne bascule jamais automatiquement vers la piste stable. Pour
quitter volontairement la prépublication, fermez Godot et exécutez `fennara update
--version <stable-version> --project <path>`. Cette version stable précise est
validée avant toute modification de la version partagée active.

Utilisez cette commande lorsque l'assistance le demande ou lorsqu'une mise à jour
de projet indique que la CLI installée est trop ancienne pour continuer en toute sécurité.

<a id="automation-guidance"></a>
## Conseils d'automatisation

- Transmettez `--project` au lieu de dépendre du répertoire actuel.
- Fixez `--version` lorsqu'une compilation doit être reproductible.
- Conservez l'identifiant d'opération et le chemin du journal affichés en cas d'échec.
- Utilisez `fennara diagnostics --operation <id> --json` pour les rapports structurés.
- Ne modifiez pas manuellement `current.json`, les répertoires de versions, les reçus
  de mise à jour ou les dossiers d'addons mis en attente.
- N'exécutez pas une mise à jour normale qui remplace l'addon lorsque ce projet est
  ouvert dans Godot. Utilisez le processus de mise à jour intégré à l'éditeur ou
  fermez d'abord Godot.
