<!-- fennara-i18n: locale=fr source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# Exemples

<!-- fennara-doc-nav:start -->
[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · [Español](../es/examples.md) · [Português do Brasil](../pt-BR/examples.md) · [日本語](../ja/examples.md) · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · **Français** · [Deutsch](../de/examples.md) · [Türkçe](../tr/examples.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../examples.md)
<!-- fennara-doc-nav:end -->

Copiez un prompt, remplacez les détails de son projet, puis envoyez-le depuis une
application MCP ou le chat Fennara intégré.

| Objectif | Exemple |
| --- | --- |
| Confirmer l'éditeur connecté | [Vérifier la connexion](#check-connection) |
| Comprendre un projet existant | [Inspecter avant de modifier](#inspect-a-project-before-editing) |
| Effectuer une modification ciblée | [Modification respectant l'architecture](#make-a-small-architecture-aware-change) |
| Diagnostiquer un projet en cours d'exécution | [Erreur d'exécution](#debug-a-runtime-error) |
| Inspecter le résultat rendu | [Retour visuel](#visual-feedback) |

<a id="check-connection"></a>
## Vérifier la connexion

```text
Utilise Fennara MCP pour exécuter fennara_status et indique-moi quel projet Godot est connecté.
```

<a id="inspect-a-project-before-editing"></a>
## Inspecter un projet avant de le modifier

```text
Utilise Fennara MCP pour inspecter ce projet Godot. Examine l'arbre de scène, les fichiers pertinents, les diagnostics et la structure du projet avant de proposer des modifications.
```

<a id="make-a-small-architecture-aware-change"></a>
## Effectuer une petite modification respectant l'architecture

```text
Travaille dans ce projet Godot existant comme un contributeur soigneux. Inspecte l'organisation du système concerné, effectue la plus petite modification utile, puis explique quels fichiers et quelles ressources ont changé et comment je peux la tester.
```

<a id="debug-a-runtime-error"></a>
## Déboguer une erreur d'exécution

```text
Utilise Fennara MCP pour inspecter les dernières erreurs d'exécution de Godot, trouver leur source probable, corriger le problème et expliquer la correction.
```

<a id="visual-feedback"></a>
## Retour visuel

```text
Utilise Fennara MCP pour capturer une image de la scène actuelle, inspecter la disposition de l'interface et suggérer ou effectuer une petite correction si quelque chose est visiblement incorrect.
```

<a id="built-in-chat-provider-setup"></a>
## Configuration du fournisseur du chat intégré

Dans le dock Fennara au sein de Godot :

```text
/provider
```

Connectez un fournisseur cloud ou un fournisseur local.

Puis :

```text
/model
```

Choisissez le modèle que le dock doit utiliser.

<a id="existing-project-demo-prompt"></a>
## Prompt de démonstration pour un projet existant

Voici le type de prompt utilisé pour la démo Open RPG :

```text
Je veux que tu travailles dans ce projet RPG Godot existant comme un contributeur soigneux du projet. Avant d'effectuer des modifications, comprends l'organisation des systèmes concernés. Réutilise autant que possible l'architecture et le style de nommage existants. Ajoute la fonctionnalité demandée de la façon la plus petite et la plus propre possible, puis indique-moi ce qui a changé et comment l'essayer dans le jeu.
```
