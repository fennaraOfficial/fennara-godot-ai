<!-- fennara-i18n: locale=fr source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# Contribuer

<!-- fennara-doc-nav:start -->
[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · [Español](../es/CONTRIBUTING.md) · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · [日本語](../ja/CONTRIBUTING.md) · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · **Français** · [Deutsch](../de/CONTRIBUTING.md) · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Merci de contribuer à l'amélioration de Fennara Godot AI.

<a id="good-contributions"></a>
## Contributions utiles

- Corrections de documentation
- Corrections de bogues reproductibles
- Corrections de compatibilité avec les plateformes
- Améliorations de la compilation et de la création des paquets
- Petites améliorations de la clarté de l'installation

<a id="design-discussion-required"></a>
## Discussion de conception requise

Ouvrez une issue ou une discussion avant de commencer :

- de nouveaux outils MCP
- des modifications de schéma d'outil
- des modifications du processus de publication
- des modifications importantes de l'architecture
- des modifications qui touchent les instructions de projet générées

<a id="pull-requests"></a>
## Pull requests

- Gardez les pull requests petites et ciblées.
- Expliquez ce qui a changé et pourquoi.
- Expliquez comment vous avez vérifié la modification.
- Ajoutez des captures d'écran ou des enregistrements pour les modifications visibles de l'interface ou du rendu de la documentation.
- N'incluez pas de mise en forme ou de nettoyage sans rapport avec la modification.
- Ne collez pas de longues descriptions générées dans les issues ou les pull requests.

<a id="commit-and-pr-titles"></a>
## Titres des commits et des PR

Utilisez le style Conventional Commits :

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

Types courants :

- `feat` : fonctionnalité visible par l'utilisateur
- `fix` : correction de bogue
- `docs` : documentation
- `ci` : GitHub Actions et automatisation
- `build` : compilation ou création de paquets
- `refactor` : restructuration du code sans modification du comportement
- `test` : tests
- `chore` : maintenance

<a id="project-boundaries"></a>
## Limites du projet

Fennara doit rester indépendant de tout jeu. Évitez les API ou les instructions qui supposent les contrôles, les objectifs, l'économie, l'inventaire, les combats, la recherche de chemin, les quêtes ou le déroulement de l'interface d'un jeu.

Les agents doivent inspecter les scènes, les scripts, les ressources, les réglages, l'état d'exécution, les diagnostics et les captures d'écran réels d'un projet Godot, puis composer des outils Fennara génériques adaptés à ce projet.

<a id="documentation-translations"></a>
## Traductions de la documentation

L'anglais est la source canonique. Corrigez d'abord l'anglais, puis mettez à
jour chaque langue concernée. L'ensemble traduit et les métadonnées des langues
se trouvent dans `docs/i18n/languages.json`.

- Lisez la page anglaise complète et rédigez directement la traduction. N'utilisez ni service de traduction automatique en masse ni script de génération de prose.
- Conservez exactement les blocs de code, le code en ligne, les commandes, les chemins, les clés de configuration, les URL et les noms de produits.
- Préservez le marqueur de source et les alias d'ancre anglaise explicites gérés par les scripts de documentation.
- N'indiquez pas qu'une traduction a été relue par un locuteur natif tant qu'une personne compétente ne l'a pas vérifiée.
- Ne traduisez pas les textes juridiques, les prompts internes des agents, les instructions de projet générées, les fichiers de fournisseurs ou les données de test comme sources indépendantes.

Après toute modification de la documentation canonique ou traduite, exécutez :

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Ces commandes gèrent les métadonnées de navigation et valident la structure.
Elles ne rédigent pas la prose traduite.

La synchronisation ordinaire de la navigation préserve tous les hachages de
source existants. Après la modification d'une source anglaise, mettez
directement cette page à jour dans les neuf langues traduites, puis validez
délibérément cette seule source canonique :

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

Répétez `--accept-source <path>` pour chaque page anglaise dont les traductions
ont été relues et mises à jour. Ne validez jamais un hachage de source avant que
les neuf traductions contiennent le nouveau sens.
