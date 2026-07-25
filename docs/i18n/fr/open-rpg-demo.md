<!-- fennara-i18n: locale=fr source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Analyse de la démo Open RPG

<!-- fennara-doc-nav:start -->
[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · [Español](../es/open-rpg-demo.md) · [Português do Brasil](../pt-BR/open-rpg-demo.md) · [日本語](../ja/open-rpg-demo.md) · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · **Français** · [Deutsch](../de/open-rpg-demo.md) · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

Vidéo :

https://www.youtube.com/watch?v=0Egu3S-9MM0

Cette démo teste Fennara MCP sur Open RPG, le projet Godot 4 open source de GDQuest.

L'intérêt de la démo n'est pas qu'une IA ait créé un projet vide à partir de rien. Il tient au fait qu'un agent IA a travaillé dans un code source existant de RPG Godot, a commis des erreurs, a reçu des informations de Godot, a corrigé l'implémentation et a poursuivi son travail.

<a id="project"></a>
## Projet

Godot 4 Open RPG de GDQuest :

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## Tâche

Ajouter une fonctionnalité de progression dans laquelle Baloo, le combattant joueur Ours, débloque une nouvelle capacité de combat nommée Tactical Guard après avoir remporté un affrontement existant.

La capacité devait :

- cibler un ennemi
- infliger des dégâts modérés
- augmenter la défense de Baloo
- apparaître dans le menu des actions de combat de Baloo après le déblocage
- afficher un message comme `Baloo learned Tactical Guard!` après le déblocage

<a id="what-happened"></a>
## Déroulement

Un agent de programmation IA connecté au projet Godot actif au moyen de Fennara MCP a inspecté l'architecture du projet.

Il a utilisé les outils Fennara pour :

- inspecter l'arbre de scène
- inspecter les propriétés des nœuds
- obtenir les diagnostics GDScript
- valider les scènes
- obtenir les erreurs d'exécution
- inspecter le projet et les scènes

La première implémentation n'a pas parfaitement fonctionné. C'est précisément ce qui était utile.

Fennara a renvoyé les informations de Godot, l'agent a corrigé le script défectueux, ajusté l'implémentation et poursuivi le travail jusqu'à ce que la fonctionnalité fonctionne dans le jeu.

<a id="why-this-matters"></a>
## Pourquoi est-ce important ?

Les démos vides sont faciles. Les agents IA échouent généralement dans les projets existants.

La thèse de Fennara est que les agents IA Godot ont besoin des informations du moteur :

- Le script a-t-il été analysé correctement ?
- La scène a-t-elle été validée ?
- L'exécution a-t-elle émis une erreur ?
- L'agent a-t-il inspecté la véritable structure du projet ?
- L'agent peut-il corriger son erreur au lieu de prétendre que la tâche est terminée ?

Un MCP traditionnel donne des commandes à une IA.

Fennara donne à l'IA des informations provenant de Godot.
