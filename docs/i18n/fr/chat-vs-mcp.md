<!-- fennara-i18n: locale=fr source=docs/chat-vs-mcp.md sha256=b6f27b2c7e905515aba56b75bf6736644a9c36c885f4cab61555c82cd6c47fda -->
<a id="mcp-apps-or-built-in-chat"></a>
# Applications MCP ou chat intégré ?

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · [Español](../es/chat-vs-mcp.md) · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · [日本語](../ja/chat-vs-mcp.md) · [한국어](../ko/chat-vs-mcp.md) · [Русский](../ru/chat-vs-mcp.md) · **Français** · [Deutsch](../de/chat-vs-mcp.md) · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara prend en charge les deux. Choisissez l'endroit où vous souhaitez mener la conversation.

| | Application MCP externe | Chat Fennara intégré |
| --- | --- | --- |
| Lieu de la conversation | Codex, Claude, Cursor, Gemini ou une autre application MCP | Le dock Fennara ou votre navigateur système |
| Compte du modèle | Le compte ou l'abonnement de l'application externe | Un fournisseur connecté dans Fennara Chat Settings |
| Apport de Fennara | Des outils MCP qui comprennent Godot | Une interface de chat, les mêmes outils Godot principaux et des outils de fichiers et de shell réservés au chat |
| Configuration | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> Vous pouvez utiliser les deux voies. Leurs réglages de modèle restent distincts.

<a id="external-mcp-apps"></a>
## Applications MCP externes

La connexion d'une application MCP permet à cette application de démarrer le serveur
MCP local de Fennara et d'appeler les outils Godot. Elle ne partage ni l'abonnement ni
la connexion de l'application avec le chat intégré.

Configurez une application depuis **Chat Settings > MCP Apps**, ou utilisez la CLI :

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Aucune clé de fournisseur de chat Fennara n'est requise. Redémarrez l'application
externe après la configuration. Consultez [Configuration MCP](mcp-setup.md) pour
chaque cible et pour la configuration manuelle.

<a id="built-in-chat"></a>
## Chat intégré

Le chat intégré exige un fournisseur connecté dans Fennara Chat Settings. Utilisez
votre propre clé pour un fournisseur cloud, ou connectez un serveur Ollama ou LM Studio local.

Le même chat peut apparaître dans le dock Godot ou dans votre navigateur système. Ce choix
d'affichage ne modifie ni son fournisseur, ni son modèle, ni son historique, ni son projet.

Pour joindre du code, sélectionnez-le dans l'éditeur de scripts de Godot, ouvrez le menu
contextuel et choisissez **Add to Chat**. Consultez [Fournisseurs du chat intégré](providers.md)
pour configurer le fournisseur et le modèle.

<a id="project-routing"></a>
## Routage des projets

Les deux voies utilisent le daemon Fennara local pour obtenir les informations de Godot.

- Un processus MCP externe peut se lier une seule fois au démarrage à une racine
  de projet Godot canonique. Ses appels sont dirigés vers l'éditeur correspondant
  sans lire ni modifier la **MCP target** du dock.
- Un processus MCP externe non lié conserve le comportement de compatibilité :
  il utilise la cible MCP sélectionnée dans le dock, ou l'unique éditeur connecté
  lorsqu'aucune cible valide n'est sélectionnée.
- Le chat intégré reste lié à l'éditeur Godot qui a ouvert le chat.

Utilisez un processus et une connexion MCP par projet pour isoler les agents qui
travaillent dans des dépôts ou arbres de travail distincts. Consultez [Plusieurs
agents et arbres de travail](multi-agent-worktrees.md) pour la configuration et
le comportement de l'emplacement d'exécution.

Pour vérifier une connexion MCP externe, demandez :

```text
Utilise Fennara MCP pour exécuter fennara_status et indique-moi quel projet Godot est connecté.
```

Avant tout travail concurrent, vérifiez que l'état indique le mode de routage
`bound` et la racine de projet canonique attendue. Le mode non lié hérité
comprend un avertissement relatif au travail concurrent.
