<!-- fennara-i18n: locale=fr source=docs/setup.md sha256=ab1b11ff7dd3472ab14185e920004b6504fa14eb1c29e7c7b1d7a322780af1dd -->
<a id="setup"></a>
# Installation

<!-- fennara-doc-nav:start -->
[Documentation](README.md)

🌐 **Langues**

[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · [Español](../es/setup.md) · [Português do Brasil](../pt-BR/setup.md) · [日本語](../ja/setup.md) · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · **Français** · [Deutsch](../de/setup.md) · [Türkçe](../tr/setup.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../setup.md)
<!-- fennara-doc-nav:end -->

Installez Fennara, choisissez où vous souhaitez discuter et connectez votre projet Godot.

> [!TIP]
> La plupart des utilisateurs doivent seulement ajouter l'addon, ouvrir le dock
> Fennara et appuyer sur **Set Up Fennara**. Sous macOS, utilisez l'installation
> par la CLI ci-dessous pour éviter la notification de sécurité qui peut suivre
> l'extraction manuelle d'un fichier ZIP d'addon.

<a id="before-you-start"></a>
## Avant de commencer

| Prérequis | Quand en avez-vous besoin ? |
| --- | --- |
| Godot 4.5 ou version ultérieure | Toujours |
| Windows x86_64, Linux x86_64 ou macOS arm64 | Toujours |
| Une application IA compatible MCP | Uniquement pour une utilisation MCP externe |
| Une clé API cloud, Ollama ou LM Studio | Uniquement pour le chat intégré |
| Le SDK .NET disponible sous la commande `dotnet` | Uniquement pour les diagnostics C# et la vérification préalable de l'exécution |

<a id="install-from-godot"></a>
## Installer depuis Godot

> [!IMPORTANT]
> Sous macOS, l'addon de la version publiée contient une bibliothèque native
> qui n'est actuellement pas notariée par Apple. Télécharger le fichier ZIP de
> l'addon dans un navigateur et l'extraire manuellement peut conduire macOS à
> signaler qu'il ne peut pas vérifier que `libfennara.macos.editor` est exempt
> de logiciels malveillants. Utilisez [Installer depuis le terminal](#install-from-the-terminal-recommended-on-macos)
> pour éviter cette notification.

1. Téléchargez `fennara-addon-latest.zip` depuis la
   [dernière version publiée](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)
   et copiez `addons/fennara/` dans votre projet.
2. Ouvrez le projet et sélectionnez le dock Fennara.
3. Appuyez sur **Set Up Fennara**.

Fennara installe les composants locaux correspondants et connecte le projet
ouvert. Si un ancien daemon partagé est inactif, l'installation l'arrête avant
d'activer la version correspondante. Un changement de version exige qu'aucun
projet ne soit connecté. Le projet en cours d'installation reste normalement
déconnecté tant que les versions diffèrent. Si l'installation signale un projet
connecté, fermez tous les autres éditeurs compatibles Fennara et réessayez. Si
une connexion obsolète subsiste pour le projet actuel, fermez et rouvrez cet
éditeur, puis réessayez.
Si l'installation échoue, le dock propose **Retry**, **Copy Report** et **Open Logs**.
Les rapports copiés sont nettoyés et ne contiennent ni clés API, ni contenu de
chat, ni fichiers du projet.

> [!NOTE]
> L'addon reste dans votre projet. La CLI, le daemon, le serveur MCP, les journaux
> et l'environnement d'exécution partagé du navigateur résident dans les données
> d'application de Fennara, hors du projet.

<a id="install-from-the-terminal-recommended-on-macos"></a>
## Installer depuis le terminal (recommandé sous macOS)

La CLI installe le même addon et constitue la méthode d'installation recommandée
sous macOS. Elle évite le chemin de quarantaine du navigateur et de Finder qui
provoque la notification de bibliothèque native décrite ci-dessus.

Installez la CLI sous Windows :

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Ou sous macOS et Linux :

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Exécutez ensuite Fennara dans le projet :

```bash
cd path/to/your-godot-project
fennara install
```

Si vous avez déjà extrait manuellement l'addon sous macOS et que la notification
s'affiche, fermez Godot et supprimez le dossier `addons/fennara/` copié manuellement
avant d'exécuter `fennara install`. Cette étape est importante, car la CLI préserve
un addon complet existant au lieu de le remplacer.

Si le projet contient déjà un addon Fennara complet, la CLI le conserve et
installe les composants locaux correspondants. Sinon, elle installe également
l'addon de la version actuelle. Consultez la [référence d'installation de la CLI](cli.md#install-a-project)
pour le verrouillage de version et l'automatisation.

<a id="choose-how-you-use-fennara"></a>
## Choisir comment utiliser Fennara

| Voie | Compte du modèle | Configuration |
| --- | --- | --- |
| Chat intégré | Un fournisseur connecté dans Fennara Chat Settings | [Connecter un fournisseur](#connect-the-built-in-chat) |
| Application MCP externe | Le compte ou l'abonnement au modèle de l'application | [Connecter une application MCP](#connect-an-mcp-app) |
| Les deux | Chaque voie conserve ses propres réglages de modèle | Effectuez les deux sections |

<a id="connect-the-built-in-chat"></a>
### Connecter le chat intégré

1. Ouvrez **Chat Settings > Chat**.
2. Sélectionnez **Open providers**.
3. Connectez un fournisseur cloud avec votre propre clé, ou connectez un serveur
   Ollama ou LM Studio local.
4. Choisissez un modèle.

Consultez [Fournisseurs du chat intégré](providers.md) pour les fournisseurs pris
en charge, les clés, les URL des serveurs locaux et les identifiants de modèle.
Utilisez `/provider` et `/model` pour effectuer les mêmes actions depuis la zone
de rédaction.

Le chat intégré utilise la webview de la plateforme :

| Plateforme | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | WKWebView/WebKit du système |
| Linux | Environnement d'exécution CEF partagé géré par Fennara |

`fennara install`, `fennara update` et `fennara doctor` vérifient ces
prérequis. Les outils MCP continuent de fonctionner si le chat intégré facultatif
ne peut pas démarrer.

Pour utiliser le navigateur système à la place, activez **Open chat in my system
browser next time** dans Chat Settings et redémarrez Godot. Cette option change
uniquement l'emplacement du chat intégré. Elle conserve le même fournisseur, le
même historique et la même connexion au projet.

Pour joindre du code au prochain message du chat intégré, sélectionnez le code
dans l'éditeur de scripts de Godot, ouvrez le menu contextuel et choisissez
**Add to Chat**.

<a id="connect-an-mcp-app"></a>
### Connecter une application MCP

Ouvrez **Chat Settings > MCP Apps**, trouvez votre application et appuyez sur
**Set Up**. Redémarrez l'application afin qu'elle puisse charger Fennara.

Vous pouvez également connecter une application depuis le terminal :

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Si votre application ne figure pas dans la liste, consultez [Configuration MCP](mcp-setup.md)
pour connaître toutes les cibles prises en charge et les formats de configuration manuelle.

Les applications MCP externes utilisent leurs propres comptes de modèle. Le chat
intégré utilise le fournisseur sélectionné dans Fennara Chat Settings. Consultez
[Applications MCP et chat intégré](chat-vs-mcp.md) pour comprendre cette distinction.

<a id="verify-the-connection"></a>
## Vérifier la connexion

Ouvrez le projet Godot, puis demandez à votre application MCP :

```text
Utilise Fennara MCP pour exécuter fennara_status et indique-moi quel projet Godot est connecté.
```

Si elle signale le mauvais projet, sélectionnez la bonne cible MCP dans le dock Fennara.

<a id="update-fennara"></a>
## Mettre à jour Fennara

Lorsque le dock affiche **Update**, appuyez dessus et suivez les instructions.
Fennara télécharge et vérifie la mise à jour avant de demander la fermeture de
Godot. Il rouvre le même projet après l'installation et conserve la version
fonctionnelle précédente jusqu'à la validation de la mise à jour.

Pour effectuer la mise à jour depuis le terminal, fermez Godot et exécutez :

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Si vous effectuez la mise à niveau depuis Fennara v0.3.8 ou une version
> antérieure, réinstallez une fois la CLI avec la commande d'installation de
> votre plateforme ci-dessus avant d'exécuter `fennara update`. Ces CLI
> interrogent une étiquette de version retirée et ne peuvent pas découvrir les
> versions actuelles. La réinstallation de la CLI fait basculer les futures
> mises à jour vers le point d'accès Latest Release de GitHub sans supprimer
> l'addon ni les réglages de votre projet.

> [!IMPORTANT]
> Sous macOS, réinstallez une fois la CLI avant d'effectuer la mise à niveau depuis
> Fennara v0.3.11. Cette CLI rejette le bundle de framework existant avant
> d'atteindre sa propre mise à jour. La réinstallation remplace uniquement la CLI
> et préserve l'addon ainsi que les réglages du projet.

Si la validation échoue, utilisez **Restore Previous Version**, **Open Logs** ou
**Copy Report** dans le dock. Consultez la [référence de mise à jour de la CLI](cli.md#update-a-project)
pour les versions précises, la préparation et la récupération des mises à jour interrompues.

<a id="troubleshooting"></a>
## Dépannage

<a id="an-install-or-update-failed"></a>
### Une installation ou une mise à jour a échoué

Copiez le rapport nettoyé depuis le dock, ou affichez le dernier rapport dans un terminal :

```bash
fennara diagnostics
```

Consultez [Diagnostics de la CLI](cli.md#inspect-health-and-failures) pour les
identifiants d'opération, la sortie JSON, les champs enregistrés et les garanties
de masquage.

<a id="fennara-is-not-found"></a>
### La commande `fennara` est introuvable

Ouvrez un nouveau terminal et exécutez :

```bash
fennara doctor
```

Si la commande reste indisponible, ajoutez le répertoire `bin` de Fennara à PATH.
La [page d'installation de la CLI](cli.md#install-the-cli) indique les chemins
de chaque plateforme.

<a id="windows-binaries-fail-before-starting"></a>
### Les binaires Windows échouent avant le démarrage

Si un binaire Fennara signale l'absence d'une DLL `VCRUNTIME` ou `MSVCP`, le
code de sortie `-1073741515` ou `0xc0000135`, installez Microsoft Visual C++
Redistributable 2015-2022 x64 :

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

Cette installation est requise uniquement sur les machines Windows auxquelles
ces DLL d'exécution Microsoft font défaut.

<a id="a-release-requires-a-newer-cli"></a>
### Une version publiée exige une CLI plus récente

Si la mise à jour automatique de la CLI ne peut pas installer la version requise,
réexécutez le script d'installation indiqué dans [Installer la CLI](cli.md#install-the-cli),
puis réessayez la commande.

<a id="the-addon-is-not-visible-in-godot"></a>
### L'addon n'est pas visible dans Godot

Vérifiez que ce fichier existe, puis rouvrez le projet :

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` affiche le mauvais projet

Ouvrez le projet voulu et sélectionnez-le à l'aide du contrôle de cible MCP du dock Fennara.

<a id="c-diagnostics-are-missing"></a>
### Les diagnostics C# sont absents

Vérifiez que le projet contient un seul fichier `.csproj`, `.sln` ou `.slnx`
clairement identifiable, puis exécutez :

```bash
dotnet --version
```

Pour les dispositions des environnements d'exécution du navigateur, la récupération
manuelle et les détails d'implémentation, consultez [Architecture](architecture.md),
[Installation manuelle](manual-install.md) et la [FAQ](faq.md).
