<!-- fennara-i18n: locale=fr source=docs/providers.md sha256=47172d638ee7954d827f50c7e8803e4a5bb2552ceb9ff44a780d49b9bf96fac6 -->
<a id="built-in-chat-providers"></a>
# Fournisseurs du chat intégré

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · **Français** · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../providers.md)
<!-- fennara-doc-nav:end -->

Connectez un fournisseur de modèle au dock de chat Fennara dans Godot.

> [!NOTE]
> Les applications MCP externes utilisent leur propre configuration de modèle.
> Vous n'avez pas besoin de connecter un fournisseur ici pour utiliser Fennara
> depuis Codex, Claude, Cursor ou une autre application MCP. Consultez
> [Applications MCP et chat intégré](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Configuration rapide

1. Ouvrez **Chat Settings > Chat** dans le dock Fennara.
2. Sélectionnez **Open providers**.
3. Choisissez un fournisseur cloud et saisissez votre propre clé, ou choisissez
   Ollama ou LM Studio pour un modèle local.
4. Sélectionnez un modèle.

Vous pouvez également saisir `/provider` et `/model` dans la zone de rédaction.

<a id="provider-reference"></a>
## Référence des fournisseurs

| Fournisseur | Méthode de connexion | Forme de l'identifiant du modèle | Remarques |
| --- | --- | --- | --- |
| OpenAI | Créez une clé dans [OpenAI API keys](https://platform.openai.com/api-keys). Clé/variable Fennara : `OPENAI_API_KEY`. | `openai/<model>` | Utilise l'API officielle d'OpenAI. |
| Anthropic | Créez une clé dans [Claude Console API keys](https://console.anthropic.com/settings/keys). Clé/variable Fennara : `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Utilise l'API Messages officielle d'Anthropic. |
| OpenRouter | Créez une clé dans [OpenRouter Keys](https://openrouter.ai/settings/keys). Clé/variable Fennara : `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | Utilise l'API d'OpenRouter. |
| Ollama Cloud | Créez une clé dans [Ollama API keys](https://ollama.com/settings/keys). Clé/variable Fennara : `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Utilise l'API hébergée d'Ollama, pas le serveur Ollama local. |
| DeepSeek | Créez une clé dans [DeepSeek API keys](https://platform.deepseek.com/api_keys). Clé/variable Fennara : `DEEPSEEK_API_KEY`. | `deepseek/<model>` | Utilise l'API compatible OpenAI de DeepSeek. |
| Z.AI | Créez une clé dans [Z.AI API keys](https://z.ai/manage-apikey/apikey-list). Clé/variable Fennara : `ZHIPU_API_KEY`. | `zai/<model>` | Utilise l'API compatible OpenAI de Z.AI. |
| Moonshot AI | Créez une clé dans [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys). Clé/variable Fennara : `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Utilise l'API compatible OpenAI de Moonshot. |
| Moonshot AI (Chine) | Créez une clé dans [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys). Clé/variable Fennara : `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Utilise l'API compatible OpenAI de Moonshot China. |
| Kimi For Coding | Créez une clé dans [Kimi Code Console](https://www.kimi.com/code/console). Clé/variable Fennara : `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Utilise l'API Messages compatible Anthropic de Kimi. Nécessite l'accès à Kimi Code. |
| MiniMax | Créez une clé avec paiement à l'usage sur [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview), **API Keys > Create new secret key**. Clé/variable Fennara : `MINIMAX_API_KEY`. | `minimax/<model>` | Utilise l'API Messages compatible Anthropic de MiniMax sur `minimax.io`. |
| MiniMax Token Plan | Utilisez la Subscription Key de [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview), **Billing > Token Plan**. Clé/variable Fennara : `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Les Subscription Keys du Token Plan sont distinctes des clés API avec paiement à l'usage. |
| MiniMax (Chine) | Créez une clé avec paiement à l'usage depuis la page des clés API de [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Clé/variable Fennara : `MINIMAX_API_KEY`. | `minimax-cn/<model>` | Utilise l'API Messages compatible Anthropic de MiniMax China sur `minimaxi.com`. |
| MiniMax Token Plan (Chine) | Utilisez la Subscription Key de la page Token Plan de [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Clé/variable Fennara : `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | Les Subscription Keys du Token Plan chinois sont distinctes des clés API avec paiement à l'usage. |
| NVIDIA | Créez une clé sur [build.nvidia.com](https://build.nvidia.com/). Clé/variable Fennara : `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | Utilise l'API NIM hébergée de NVIDIA compatible OpenAI. |
| Ollama | Exécutez un serveur Ollama local. Aucune clé API cloud n'est requise. | `ollama/<local-model>` | Utilise `http://127.0.0.1:11434` par défaut. |
| LM Studio | Démarrez le serveur local de LM Studio. Aucune clé n'est requise par défaut. | `lmstudio/<local-model>` | Utilise `http://127.0.0.1:1234/v1` par défaut. Si votre serveur LM Studio exige une authentification, définissez `LMSTUDIO_API_KEY` dans l'environnement du daemon. |

Les fournisseurs cloud ont besoin de votre propre clé API ou clé d'abonnement.
Les fournisseurs locaux ont besoin d'un serveur local en fonctionnement avec
un modèle disponible.

Les sélections OpenRouter utilisent toujours la forme explicite
`openrouter/<provider>/<model>`. Les anciennes sélections OpenRouter enregistrées
sous la forme `<provider>/<model>` sont migrées une fois au chargement des réglages,
mais cette forme historique n'est pas utilisée pour les nouveaux routages.

Fennara peut conserver les clés saisies dans le sélecteur de fournisseurs du dock. Chat Settings comprend un bouton **Open providers** qui permet d'ouvrir le même sélecteur. Les noms de clé et de variable ci-dessus sont ceux que Fennara reconnaît également si vous préférez les variables d'environnement. Les clés conservées résident dans les données d'application locales du daemon, hors du projet Godot.

<a id="custom-openai-compatible-providers"></a>
## Fournisseurs personnalisés compatibles OpenAI

Choisissez **Custom** au bas du sélecteur de fournisseurs pour ajouter un point
d'accès compatible OpenAI, comme un routeur local ou une passerelle API interne.
Saisissez :

- un identifiant de fournisseur unique en minuscules
- le nom affiché dans Fennara
- une URL de base qui se termine au niveau de la version de l'API, par exemple `http://localhost:20128/v1`
- une clé API facultative
- un ou plusieurs identifiants de modèle, noms affichés, longueurs de contexte et limites maximales de jetons de sortie
- des en-têtes de requête facultatifs

Les identifiants de modèle doivent correspondre à ceux attendus par le point
d'accès. Fennara les expose sous la forme `<provider-id>/<model-id>` dans le
sélecteur de modèle, mais n'envoie que `<model-id>` au fournisseur. Le point
d'accès doit implémenter la forme de requête et de réponse en flux compatible
OpenAI de `/chat/completions`.

Les clés API et les valeurs d'en-tête personnalisées utilisent le stockage
d'authentification protégé du daemon Fennara. Les définitions de fournisseurs
restent dans les données d'application locales gérées par le daemon, hors du
projet Godot. Des limites de modèle précises permettent à Fennara de compacter
l'historique de la conversation avant qu'une requête dépasse la fenêtre de
contexte du modèle et de maintenir les résumés produits dans la limite de sortie
du modèle. Les modèles personnalisés existants enregistrés avant l'ajout de ces
champs sont chargés avec les valeurs de compatibilité par défaut de 64 000 jetons
de contexte et 4 096 jetons de sortie.

Après l'enregistrement, le fournisseur personnalisé apparaît dans le sélecteur
avec son nombre de modèles. Sélectionnez ce fournisseur pour rouvrir le formulaire
et ajouter ou renommer des modèles. Laisser la clé API vide conserve la clé
enregistrée, et tout nouvel en-tête saisi est fusionné avec les en-têtes enregistrés
selon son nom.

<a id="where-settings-live"></a>
## Emplacement des réglages

Fennara conserve localement les réglages du chat intégré par l'intermédiaire du daemon, hors du projet Godot :

- les clés API des fournisseurs
- les valeurs d'en-tête des fournisseurs personnalisés
- les définitions de fournisseurs personnalisés compatibles OpenAI
- les URL de base des fournisseurs locaux
- le modèle sélectionné
- l'effort de raisonnement
- le délai de réponse du fournisseur
- le mode d'affichage du chat, intégré à Godot ou ouvert dans le navigateur système
- l'historique du chat

Ces réglages ne sont pas écrits dans `res://addons/fennara/` et ne sont pas partagés avec Claude, Codex, Cursor, Gemini ou les autres applications MCP externes.

<a id="provider-response-timeout"></a>
## Délai de réponse du fournisseur

Le réglage **Provider response timeout** contrôle la durée maximale de chaque requête de modèle dans le chat intégré. Sa valeur par défaut est de 120 secondes et il accepte des valeurs comprises entre 30 et 3600 secondes. Une valeur plus élevée peut permettre aux modèles locaux plus lents ou aux longs tours utilisant de nombreux outils de se terminer. Le daemon applique le délai sélectionné à la requête du fournisseur et annule celle-ci si la limite est atteinte.

<a id="chat-display-setting"></a>
## Réglage d'affichage du chat

La boîte de dialogue Chat Settings comprend **Open chat in my system browser next time**.

Lorsque ce réglage est désactivé, Fennara tente d'afficher le chat intégré dans le dock Godot. Lorsqu'il est activé, le dock affiche un bouton **Open chat** et lance le même chat intégré par l'intermédiaire du daemon local sur `127.0.0.1`. Cela peut réduire l'utilisation du processeur graphique et de la mémoire par l'éditeur Godot et sert également de solution de secours si la webview native ne peut pas démarrer.

La modification de ce réglage prend effet au prochain démarrage de Godot. Elle change uniquement l'endroit où l'interface du chat intégré est affichée. Elle ne modifie ni le fournisseur sélectionné, ni le modèle, ni les clés API, ni l'historique du chat, ni la configuration des applications MCP, ni le modèle utilisé en externe par Claude, Codex ou Cursor.

<a id="picker-shortcuts"></a>
## Raccourcis des sélecteurs

Chat Settings, les contrôles du dock et `/provider` ouvrent le même sélecteur
de fournisseurs. Utilisez `/model` ou le contrôle de modèle du dock pour ouvrir
le sélecteur de modèle.

Consultez [Commandes slash du chat intégré](slash-commands.md) pour connaître le comportement de la palette de commandes.

<a id="local-providers"></a>
## Fournisseurs locaux

Pour Ollama :

```bash
ollama serve
ollama pull llama3.1:8b
```

Choisissez ensuite :

```text
ollama/llama3.1:8b
```

Les anciennes sélections `local/<model>` sont encore acceptées comme alias de
compatibilité Ollama. Préférez la forme explicite `ollama/<model>` pour les
nouveaux réglages.

Fennara envoie le maximum par appel d’Ollama dans le champ compatible OpenAI
`max_tokens`, qu’Ollama associe à son option native `num_predict`.

Pour LM Studio, démarrez le serveur local depuis LM Studio et choisissez un identifiant de modèle de la forme :

```text
lmstudio/<loaded-model-id>
```

Les formulaires de configuration d’Ollama et de LM Studio partagent une limite
maximale configurée de sortie par appel, fixée par défaut à 8 192 tokens.
Lorsqu’un serveur local indique la longueur du contexte chargé, Fennara limite
ce réglage à la moitié du contexte afin de conserver de la place pour l’entrée.
Fennara envoie cette limite effective sous la forme `max_tokens` et réserve la
même valeur lorsqu’il détermine quand compacter l’historique du chat.

<a id="model-catalog"></a>
## Catalogue de modèles

Le daemon conserve un catalogue local de modèles pour les fournisseurs cloud et demande aux serveurs locaux la liste des modèles actuellement disponibles. Si un catalogue ou un serveur local change alors que Godot est ouvert, actualisez le sélecteur de modèle ou rouvrez le sélecteur de fournisseur ou de modèle.

Fennara vérifie les capacités élémentaires du modèle avant d'envoyer une requête :

- la sortie de texte est obligatoire
- l'appel d'outils est obligatoire pour utiliser les outils Fennara
- l'entrée d'images est obligatoire avant que les pièces jointes d'images soient envoyées comme contexte visuel

L'entrée d'images d'Ollama n'est pas encore activée dans le chat Fennara.
