<!-- fennara-i18n: locale=fr source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# Auxiliaires d'exécution

<!-- fennara-doc-nav:start -->
[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · **Français** · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

Ce dossier contient les sources des scripts auxiliaires côté Godot employés par
`runtime_session` et `runtime_script`.

La copie de l'addon empaqueté se trouve dans :

```text
godot_demo/addons/fennara/runtime/
```

Après avoir modifié les fichiers de ce dossier, exécutez :

```bash
node scripts/sync-runtime.mjs
```

Les scripts d'exécution continuent de charger ces auxiliaires depuis
`res://addons/fennara/runtime/` dans un projet Godot où l'addon est installé.
Gardez les auxiliaires primitifs et indépendants du projet : les entrées,
l'attente, les instantanés des nœuds, les captures, les requêtes physiques et
la prise en charge du cycle de vie des scènes sont appropriés. Les hypothèses
propres aux déplacements, aux combats, aux quêtes, à l'inventaire ou au déroulement
de l'interface d'un jeu ne le sont pas.

`image_sheet.gd` est également utilisé par la façade des scripts de capture
d'écran. Gardez sa composition déterministe et indépendante de l'état de la
scène, de l'animation ou du jeu.
