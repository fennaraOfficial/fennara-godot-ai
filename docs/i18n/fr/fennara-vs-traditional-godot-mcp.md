<!-- fennara-i18n: locale=fr source=docs/fennara-vs-traditional-godot-mcp.md sha256=e45a741b1db7c20e40b4a311c198af216172dfa024ca9c123db4f9336c9a6e7f -->
<a id="fennara-vs-traditional-godot-mcp"></a>
# Fennara face à un MCP Godot traditionnel

<!-- fennara-doc-nav:start -->
[English](../../fennara-vs-traditional-godot-mcp.md) · [简体中文](../zh-CN/fennara-vs-traditional-godot-mcp.md) · [Español](../es/fennara-vs-traditional-godot-mcp.md) · [Português do Brasil](../pt-BR/fennara-vs-traditional-godot-mcp.md) · [日本語](../ja/fennara-vs-traditional-godot-mcp.md) · [한국어](../ko/fennara-vs-traditional-godot-mcp.md) · [Русский](../ru/fennara-vs-traditional-godot-mcp.md) · **Français** · [Deutsch](../de/fennara-vs-traditional-godot-mcp.md) · [Türkçe](../tr/fennara-vs-traditional-godot-mcp.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../fennara-vs-traditional-godot-mcp.md)
<!-- fennara-doc-nav:end -->

| Pont de commandes traditionnel | Boucle de rétroaction de Fennara |
| --- | --- |
| Expose les actions de l'éditeur | Expose l'inspection, les actions et les vérifications qui comprennent Godot |
| Une commande réussie peut marquer la fin du processus | Les diagnostics, la validation, les journaux d'exécution et les captures d'écran guident l'étape suivante |
| Idéal pour les modifications directes et connues | Idéal lorsqu'un agent doit inspecter, modifier, vérifier et récupérer |

La plupart des serveurs MCP pour Godot exposent les commandes de l'éditeur aux clients IA.

Exemples :

- créer un nœud
- définir une propriété
- ouvrir une scène
- enregistrer une scène
- lire les journaux
- capturer une image
- exécuter le projet
- connecter un signal
- modifier l'Input Map
- gérer les matériaux
- exécuter les tests

C'est utile. Godot devient ainsi une surface d'API.

Mais dans le véritable développement de jeux par IA, la difficulté n'est pas de savoir si une IA peut appeler `set_property`.

La difficulté est de savoir si l'IA peut détecter que le projet est défectueux.

<a id="traditional-mcp-pattern"></a>
## Modèle MCP traditionnel

```text
L'IA appelle une commande de l'éditeur.
L'éditeur renvoie le résultat.
L'IA devine l'étape suivante.
```

Ce modèle fonctionne bien pour les petites modifications directes.

Exemple :

```text
Renomme Camera3D en MainCamera.
```

Il est toutefois moins efficace pour les tâches de projet plus importantes dans lesquelles l'agent doit inspecter l'architecture, modifier les scripts, les ressources et les scènes, observer les échecs et récupérer.

<a id="fennara-pattern"></a>
## Modèle de Fennara

```text
L'IA modifie le projet.
Les informations de Godot sont renvoyées.
L'IA corrige et réexécute jusqu'à ce que tout fonctionne.
```

Fennara met l'accent sur le retour d'informations :

- diagnostics GDScript
- validation des scènes
- erreurs d'exécution
- inspection de l'arbre de scène
- propriétés des nœuds
- inspection des classes et de l'API
- captures d'écran
- instructions de projet générées
- processus de correction et de réexécution

<a id="the-difference"></a>
## La différence

Un MCP Godot traditionnel demande :

```text
Quelles commandes de l'éditeur devons-nous exposer ?
```

Fennara demande :

```text
De quelles informations le modèle a-t-il besoin pour réussir à construire dans Godot ?
```

Les commandes sont un prérequis.

Le retour d'informations est l'avantage décisif.
