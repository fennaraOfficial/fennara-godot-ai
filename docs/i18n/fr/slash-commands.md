<!-- fennara-i18n: locale=fr source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# Commandes slash du chat intégré

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · [Русский](../ru/slash-commands.md) · **Français** · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

Les commandes slash sont des raccourcis du dock de chat Fennara dans Godot. Ce sont des commandes d'interface, pas des outils MCP ni des prompts envoyés au modèle.

Tapez `/` dans la zone de saisie pour ouvrir la palette de commandes.

| Commande | Ouvre | Utilité |
| --- | --- | --- |
| `/provider` | Sélecteur de fournisseur | Connecter un fournisseur cloud, configurer l'URL d'un fournisseur local ou changer de fournisseur. |
| `/model` | Sélecteur de modèle | Choisir un modèle du fournisseur actuel ou connecté. |

<a id="how-they-behave"></a>
## Comportement

- Utilisez les touches fléchées pour parcourir les suggestions de commandes.
- Appuyez sur Entrée pour exécuter la commande sélectionnée.
- Appuyez sur Échap pour fermer la palette de commandes.
- Le texte de la commande slash est retiré de la zone de saisie avant l'envoi du message de chat.

<a id="common-flow"></a>
## Processus courant

Pour le dock de chat intégré :

```text
/provider
```

Connectez OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, Ollama local ou LM Studio.

Puis :

```text
/model
```

Choisissez le modèle que le dock doit utiliser.

Pour les applications MCP externes, n'utilisez pas ces commandes slash. Configurez l'application avec `fennara mcp-setup`, puis demandez-lui d'utiliser les outils Fennara MCP.
