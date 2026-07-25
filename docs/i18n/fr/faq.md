<!-- fennara-i18n: locale=fr source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# FAQ

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · [Español](../es/faq.md) · [Português do Brasil](../pt-BR/faq.md) · [日本語](../ja/faq.md) · [한국어](../ko/faq.md) · [Русский](../ru/faq.md) · **Français** · [Deutsch](../de/faq.md) · [Türkçe](../tr/faq.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../faq.md)
<!-- fennara-doc-nav:end -->

Commencez par [Installation](setup.md) pour l'installation et les mises à jour. Utilisez cette page pour
obtenir des réponses courtes et accéder aux références détaillées.

| Question | Réponse courte |
| --- | --- |
| Ai-je besoin d'une clé de fournisseur ? | Uniquement pour un fournisseur cloud dans le chat intégré |
| Puis-je utiliser une application MCP externe à la place ? | Oui, elle utilise son propre compte de modèle |
| Fennara envoie-t-il mon projet à un serveur Fennara ? | Non |
| Plusieurs éditeurs Godot peuvent-ils être ouverts ? | Oui, choisissez la cible MCP externe dans le dock |

<a id="is-fennara-only-a-code-generator"></a>
## Fennara est-il seulement un générateur de code ?

Non. Fennara est un processus pour agents qui comprend Godot. Il peut travailler avec les fichiers du projet, les scènes, les diagnostics, les erreurs d'exécution, les captures d'écran et le contexte de l'éditeur Godot.

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## Fennara est-il simplement un autre serveur de commandes MCP pour Godot ?

Non. MCP est l'un des moyens d'utiliser Fennara depuis des applications comme Codex, Claude, Cursor, Gemini et Antigravity. Fennara possède également un dock de chat intégré facultatif. La thèse centrale du produit est la boucle de rétroaction Godot : diagnostics, validation, erreurs d'exécution, captures d'écran et résultats d'outils structurés qui permettent aux agents de corriger leurs erreurs.

<a id="does-fennara-replace-godot-knowledge"></a>
## Fennara remplace-t-il la connaissance de Godot ?

Non. Fennara ne cherche pas à rendre Godot facultatif. Il est conçu pour obliger les agents IA à tenir compte du véritable moteur Godot.

<a id="how-should-i-install-fennara"></a>
## Comment installer Fennara ?

Sous Windows et Linux, ajoutez l'addon, ouvrez le dock Fennara et appuyez sur
**Set Up Fennara**, ou effectuez l'installation depuis le terminal. Sous macOS,
installez-le au moyen de la CLI pour éviter la notification de sécurité qui peut
apparaître lorsqu'un fichier ZIP d'addon téléchargé dans un navigateur est extrait
manuellement. Consultez [Installation](setup.md) pour les deux méthodes.

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## Pourquoi macOS indique-t-il qu'il ne peut pas vérifier `libfennara.macos.editor` ?

L'addon de la version publiée contient une bibliothèque native qui n'est actuellement
pas notariée par Apple. Lorsque le fichier ZIP de l'addon est téléchargé dans un
navigateur et extrait manuellement, Finder peut propager les métadonnées de quarantaine
à cette bibliothèque, ce qui provoque la notification de macOS.

Pour l'éviter, utilisez [l'installation par la CLI](setup.md#install-from-the-terminal-recommended-on-macos).
Si la notification apparaît déjà, fermez Godot, supprimez le dossier `addons/fennara/`
copié manuellement, installez la CLI et exécutez `fennara install` depuis le répertoire
du projet. La CLI installe le même addon sans emprunter ce chemin de quarantaine du
navigateur et de Finder.

<a id="do-i-need-a-chat-provider-api-key"></a>
## Ai-je besoin d'une clé API de fournisseur de chat ?

Uniquement si vous souhaitez utiliser un fournisseur cloud dans le dock de chat intégré de Fennara. Les clients MCP externes utilisent leur propre configuration de modèle et d'application. Ils peuvent employer les outils Fennara MCP sans fournir de clé de fournisseur au chat Fennara.

Le chat intégré peut également utiliser Ollama ou LM Studio localement sans clé API
cloud. Consultez [Fournisseurs du chat intégré](providers.md).

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## Pourquoi le dock demande-t-il un fournisseur si j'ai déjà exécuté `mcp-setup --claude` ?

`fennara mcp-setup --claude` connecte Claude aux outils MCP Godot de Fennara. Il ne connecte pas le dock Fennara intégré à Claude et ne partage pas votre abonnement Claude avec le chat Fennara.

Utilisez Claude Code ou Claude Desktop pour le processus MCP externe. Configurez un fournisseur distinct uniquement si vous souhaitez discuter dans le dock Fennara de Godot. Consultez [Applications MCP et chat intégré](chat-vs-mcp.md).

<a id="what-are-provider-and-model"></a>
## Que sont `/provider` et `/model` ?

Ce sont des commandes slash du dock de chat intégré de Fennara. `/provider` ouvre le sélecteur de fournisseur. `/model` ouvre le sélecteur de modèle. Il s'agit de raccourcis d'interface, pas d'outils MCP externes ni de texte envoyé au modèle. Consultez [Commandes slash du chat intégré](slash-commands.md).

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## Fennara envoie-t-il mon projet Godot à un serveur Fennara ?

Non. Dans le parcours OSS normal, le client MCP, le daemon et l'addon Godot
s'exécutent localement. Le chat intégré envoie les requêtes au modèle uniquement
au fournisseur que vous configurez, comme OpenAI, Anthropic, OpenRouter, Ollama
Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax ou un serveur Ollama
ou LM Studio local.

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## Quel projet reçoit les appels d'outil MCP si plusieurs éditeurs Godot sont ouverts ?

Le daemon dirige les appels MCP externes vers la cible MCP active. Utilisez le
contrôle de cible MCP du dock Fennara dans Godot pour choisir le projet. Les sessions
du chat intégré restent liées à l'éditeur Godot qui a ouvert ce chat.

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## Pourquoi Linux installe-t-il un environnement d'exécution CEF distinct ?

Le chat intégré sous Linux utilise le rendu hors écran de CEF. La charge utile de
CEF est volumineuse. Fennara l'installe donc une seule fois dans le répertoire des
données d'application de Fennara de l'utilisateur au lieu de la copier dans l'addon
de chaque projet Godot.

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## L'addon est-il censé contenir `libcef.so` ?

Non. `libcef.so`, les ressources CEF, les paquets de langues et l'auxiliaire CEF
appartiennent à l'environnement d'exécution CEF Linux partagé. L'addon ne doit
contenir que les fichiers de l'addon Godot, les binaires GDExtension, les fichiers
de l'interface de chat et de petits binaires auxiliaires intégrés comme ripgrep.

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## Que se passe-t-il si la webview du chat intégré ne peut pas démarrer ?

Les outils Fennara MCP continuent de fonctionner. Seul le dock de chat facultatif
intégré à l'éditeur a besoin de la webview de la plateforme. Sous Windows, installez
Microsoft Edge WebView2 Runtime si `fennara doctor` signale son absence. Sous macOS,
WKWebView provient du composant système WebKit.framework. Sous Linux, exécutez
`fennara update` afin que l'environnement d'exécution CEF géré par la version publiée
puisse être installé ou réparé.

Vous pouvez aussi utiliser l'option **Open chat in my system browser next time**
de Chat Settings. Vous conservez ainsi le même chat Fennara intégré et les mêmes
réglages de fournisseur, mais l'interface s'ouvre par l'intermédiaire du daemon local
dans votre navigateur système au lieu de la webview Godot intégrée. Redémarrez Godot
après avoir modifié ce réglage.

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## L'ouverture du chat dans mon navigateur utilise-t-elle Claude ou mon application MCP ?

Non. L'affichage dans le navigateur est uniquement un choix d'interface et
d'environnement d'exécution pour le chat Fennara intégré. Il continue d'utiliser
le fournisseur sélectionné dans les réglages du chat Fennara. `fennara
mcp-setup --claude` et les commandes similaires configurent les applications MCP
externes. Elles ne configurent pas le modèle du chat intégré.

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## `fennara update` réécrit-il la configuration des applications MCP ?

Non. `fennara update` actualise, lorsque c'est nécessaire, la CLI installée,
l'addon du projet, le paquet d'exécution local, les instructions de projet générées
et les ressources d'exécution gérées par la plateforme. Réexécutez `fennara
mcp-setup` uniquement pour configurer ou réparer la configuration d'une application MCP.

<a id="where-does-chat-history-live"></a>
## Où l'historique du chat est-il conservé ?

L'historique du chat est conservé localement par le daemon et limité au projet
Godot actuel. Les clés des fournisseurs et les URL des fournisseurs locaux sont
également conservées localement par le daemon, hors du projet Godot.

<a id="what-should-agents-use-fennara-tools-for"></a>
## Pour quelles tâches les agents doivent-ils utiliser les outils Fennara ?

Utilisez Fennara pour les informations qui nécessitent la connaissance de Godot :
arbres de scène, propriétés modifiées des nœuds et des ressources, diagnostics,
validation, sessions d'exécution, captures d'écran et état du débogueur de l'éditeur.
Les clients MCP doivent continuer d'utiliser leurs propres outils ordinaires de
lecture et de recherche de fichiers, sauf si un outil propre à Fennara est nécessaire.
