<!-- fennara-i18n: locale=fr source=docs/languages.md sha256=476e4afb5a17f76474a25864d08280c73ba0e4fdd1ebcfbbfd814d010dfac932 -->
<a id="languages-and-translation-status"></a>
# Langues et état des traductions

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · [Español](../es/languages.md) · [Português do Brasil](../pt-BR/languages.md) · [日本語](../ja/languages.md) · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · **Français** · [Deutsch](../de/languages.md) · [Türkçe](../tr/languages.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../languages.md)
<!-- fennara-doc-nav:end -->

L'anglais est la source canonique de la documentation. Fennara propose également
des traductions complètes rédigées par une IA dans neuf langues. Chaque page
traduite renvoie à sa source anglaise actuelle et invite les locuteurs natifs
à la relire.

| Langue | Documentation | Couverture | État de la relecture |
| --- | --- | --- | --- |
| English | [Documentation en anglais](../../README.md) | 31/31 | Canonique |
| 简体中文 | [Documentation en chinois simplifié](../zh-CN/README.md) | 31/31 | Relecture native demandée |
| Español | [Documentation en espagnol](../es/README.md) | 31/31 | Relecture native demandée |
| Português do Brasil | [Documentation en portugais](../pt-BR/README.md) | 31/31 | Relecture native demandée |
| 日本語 | [Documentation en japonais](../ja/README.md) | 31/31 | Relecture native demandée |
| 한국어 | [Documentation en coréen](../ko/README.md) | 31/31 | Relecture native demandée |
| Русский | [Documentation en russe](../ru/README.md) | 31/31 | Relecture native demandée |
| Français | [Documentation en français](README.md) | 31/31 | Relecture native demandée |
| Deutsch | [Documentation en allemand](../de/README.md) | 31/31 | Relecture native demandée |
| Türkçe | [Documentation en turc](../tr/README.md) | 31/31 | Relecture native demandée |

<a id="what-is-translated"></a>
## Ce qui est traduit

L'ensemble traduit contient le README principal, chaque page directement située
sous `docs/`, `CONTRIBUTING.md`, `CONTEXT.md`, `SECURITY.md` et les six README
des sous-systèmes destinés aux contributeurs.

Les textes juridiques, les avis de tiers, les modèles d'issue, les instructions
internes des agents, les instructions de projet générées, les données de test et
la documentation des fournisseurs restent sous leur forme de référence. Les
fichiers générés ou qui déterminent un comportement ne constituent pas des
sources de traduction indépendantes.

<a id="freshness-and-validation"></a>
## Actualité et validation

Chaque page traduite enregistre le chemin de sa source canonique et le hachage
de celle-ci. La navigation est générée à partir d'un manifeste de langues unique
et des alias d'ancre anglaise stables préservent le fonctionnement des liens
profonds lorsque les titres sont traduits.

Exécutez :

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Ces outils ne traduisent pas la prose. Ils gèrent uniquement les métadonnées de
navigation et vérifient la couverture, l'actualité, la structure Markdown, les
commandes, les liens, les ancres, les blocs de code, les tableaux et les URL.
Les corrections de locuteurs natifs sont les bienvenues par le processus habituel
de pull request.

Une synchronisation normale préserve les hachages de source existants. Une
modification de la prose anglaise laisse donc ses traductions périmées jusqu'à
ce qu'elles soient directement mises à jour. Après avoir relu les neuf
traductions d'une page anglaise modifiée, validez uniquement cette source :

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

La CI exécute la synchronisation de la navigation en mode de vérification avant
la validation structurelle. Elle vérifie également que chaque ancre anglaise
stable reste attachée au titre traduit correspondant.
