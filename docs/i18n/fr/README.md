<!-- fennara-i18n: locale=fr source=docs/README.md sha256=2f8fb6a711c8bb56af570d1657f802c63cbdf2ced6b2c620339c588c9c9211cb -->
<a id="fennara-documentation"></a>
# Documentation de Fennara

<!-- fennara-doc-nav:start -->
[English](../../README.md) · [简体中文](../zh-CN/README.md) · [Español](../es/README.md) · [Português do Brasil](../pt-BR/README.md) · [日本語](../ja/README.md) · [한국어](../ko/README.md) · [Русский](../ru/README.md) · **Français** · [Deutsch](../de/README.md) · [Türkçe](../tr/README.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../README.md)
<!-- fennara-doc-nav:end -->

Commencez par la tâche que vous souhaitez accomplir. Chaque page présente d'abord
le chemin habituel et réserve les détails avancés à la suite de la page.

<a id="languages"></a>
## Langues

Utilisez le menu des langues ci-dessus pour rester sur la même page dans une autre langue. Consultez
[Langues et état des traductions](languages.md) pour connaître la couverture, l'état des relectures
et la politique relative à la source de référence.

<a id="start-here"></a>
## Commencer ici

| Je veux... | Lire... |
| --- | --- |
| Installer Fennara | [Installation](setup.md) |
| Connecter le chat intégré | [Fournisseurs de chat](providers.md) |
| Connecter Codex, Claude, Cursor ou une autre application MCP | [Configuration MCP](mcp-setup.md) |
| Mettre à jour ou récupérer Fennara | [Mettre à jour Fennara](setup.md#update-fennara) |
| Résoudre un problème d'installation | [Dépannage](setup.md#troubleshooting) |

<a id="use-fennara"></a>
## Utiliser Fennara

| Guide | Contenu |
| --- | --- |
| [Applications MCP et chat intégré](chat-vs-mcp.md) | Le compte de modèle utilisé par chaque voie |
| [Outils](tools.md) | Les outils qui comprennent Godot et le moment de les utiliser |
| [Exemples](examples.md) | Des prompts pour les processus Godot courants |
| [Commandes slash](slash-commands.md) | `/provider` et `/model` dans le dock de chat |
| [FAQ](faq.md) | Des réponses courtes aux questions fréquentes |
| [Démos](demos.md) | Des vidéos et des présentations de projets |
| [Télémétrie anonyme](telemetry.md) | Les données collectées, le comportement d'envoi et les options de désactivation |

<a id="reference-and-recovery"></a>
## Référence et récupération

| Référence | À utiliser lorsque... |
| --- | --- |
| [CLI Fennara](cli.md) | Vous avez besoin de commandes de terminal, de diagnostics ou d'automatisation |
| [Installation manuelle](manual-install.md) | Le programme d'installation normal ne peut pas être utilisé |
| [Référence de configuration MCP](mcp-setup.md) | Vous avez besoin d'une configuration propre à une application ou d'une configuration manuelle |
| [Référence des fournisseurs](providers.md) | Vous avez besoin de clés, d'identifiants de modèle ou de détails sur un serveur local |

<a id="for-contributors"></a>
## Pour les contributeurs

| Document | Objectif |
| --- | --- |
| [Contribuer](CONTRIBUTING.md) | Attentes relatives aux contributions et aux pull requests |
| [Architecture](architecture.md) | Limites du système et flux d'exécution |
| [Plan du dépôt](repo-map.md) | Emplacement du code et des fichiers générés |
| [Processus de publication](release.md) | Paquets, manifestes, validation et publication |
| [Vocabulaire du projet](CONTEXT.md) | Noms partagés dans le code et la documentation |
| [Sécurité](SECURITY.md) | Signalement des vulnérabilités |
| [Métadonnées GitHub](github-metadata.md) | Description et sujets du dépôt |
| [Charge utile Godot](contributors/godot-payload.md) | Limites des sources de l'addon empaqueté |
| [Addons Godot](contributors/godot-addons.md) | Structure et règles du répertoire des addons |
| [Outils locaux](contributors/local-tools.md) | CLI, daemon, serveur MCP et environnement d'exécution local |
| [Auxiliaires d'exécution](contributors/runtime-helpers.md) | Sources des auxiliaires d'exécution côté Godot |
| [Scripts du dépôt](contributors/scripts.md) | Automatisation de la compilation, de la synchronisation, de la validation et des paquets |
| [Interface de chat](contributors/chat-ui.md) | Sources et règles de conception du chat facultatif intégré à l'éditeur |

<a id="learn-from-examples"></a>
## Apprendre par les exemples

- [Fennara face à un MCP Godot traditionnel](fennara-vs-traditional-godot-mcp.md)
- [Analyse de la démo Open RPG](open-rpg-demo.md)
- [Exemples de prompts](examples.md)
