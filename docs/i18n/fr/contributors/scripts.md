<!-- fennara-i18n: locale=fr source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# Scripts

<!-- fennara-doc-nav:start -->
[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · [Español](../../es/contributors/scripts.md) · [Português do Brasil](../../pt-BR/contributors/scripts.md) · [日本語](../../ja/contributors/scripts.md) · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · **Français** · [Deutsch](../../de/contributors/scripts.md) · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

Ce répertoire contient l'automatisation du dépôt partagée par le développement local, la prévisualisation des paquets et les processus de publication.

Les scripts doivent être petits, déterministes et sûrs à exécuter depuis la racine du dépôt, sauf indication contraire dans leur texte d'aide. Ils ne doivent pas écrire d'état propre à l'utilisateur hors du dépôt.

<a id="version-scripts"></a>
## Scripts de version

- `set-version.mjs` : met à jour le fichier `VERSION` du dépôt, le fichier `VERSION` de l'addon, les métadonnées de l'espace de travail Rust local, les versions des paquets dans le lockfile et la constante de version du plugin C++.
- `check-version.mjs` : vérifie que ces fichiers versionnés restent synchronisés.

Exécutez `check-version.mjs` dans la CI et avant de créer les paquets d'une version. Utilisez `set-version.mjs` uniquement lorsque vous modifiez volontairement la version de Fennara.

<a id="packaging-scripts"></a>
## Scripts de création de paquets

- `package-preview.mjs` : synchronise les charges utiles commités de l'addon, puis assemble les archives de prévisualisation de chaque plateforme une fois que la GDExtension et les binaires Rust locaux ont été compilés.
- `package-addon-all.mjs` : réunit les parties de l'addon propres aux plateformes dans l'archive finale de l'addon pour toutes les plateformes.
- `release-policy.mjs` : définit la CLI publiée minimale compatible pour chaque piste de version.
- `write-release-manifest.mjs` : écrit `fennara-release-manifest-v<version>.json` à partir des ressources publiées et valide chaque SHA-256 référencé.

Les deux scripts utilisent `.package-preview/` comme zone temporaire et écrivent les fichiers ZIP de sortie dans le dossier `dist/` à la racine du dépôt. Ces résultats sont ignorés et ne doivent pas être commités.

Les scripts de création de paquets doivent garder la charge utile de l'addon petite. Les fichiers d'exécution CEF sous Linux, comme `libcef.so` et `fennara_cef_helper`, ne doivent notamment pas être intégrés à `fennara-addon-*`. CEF est installé une seule fois dans le répertoire partagé des données d'application Fennara de l'utilisateur.

<a id="staging-release-scripts"></a>
## Scripts des versions de prépublication

- `write-staging-candidate.mjs` : crée l'identité exacte de prépublication pour une pull request et un commit source figé.
- `validate-staging-build.mjs` : vérifie les parties de l'addon, les archives de plateformes, l'addon assemblé, le manifeste de version et CEF sous Linux avant la publication.
- `smoke-public-release.mjs` : télécharge chaque candidat publié par son URL de navigateur non authentifiée et vérifie les hachages fiables des ressources et du manifeste avant l'avancement du canal.
- `write-staging-pointer.mjs` : écrit le petit pointeur propre à une PR après avoir calculé le hachage du manifeste de version précis.
- `check-staging-channel-advance.mjs` : refuse tout déplacement en arrière ou contradictoire du canal.
- `validate-staging-publish-bundle.mjs` : revalide l'ensemble final de ressources sans exécuter le code candidat.
- `verify-published-assets.mjs` : compare les noms et valeurs SHA-256 des ressources GitHub Release attendues et téléchargées.

Ces scripts prennent en charge `.github/workflows/staging-release.yml`. Les jobs de compilation des candidats s'exécutent sans identifiants de publication. Seul le job final fiable peut publier. Il avance la référence Git propre au canal après le téléchargement et la vérification de la version exacte.

<a id="linux-cef-scripts"></a>
## Scripts CEF pour Linux

- `prepare-linux-cef-sdk.mjs` : télécharge et extrait le SDK officiel CEF Linux x64 épinglé, employé pour compiler le pont CEF de Linux.
- `prepare-linux-cef-runtime.mjs` : met en attente le fichier ZIP d'exécution CEF Linux distinct, valide les fichiers requis, retire les symboles des binaires ELF en attente sous Linux et peut écrire le manifeste généré `local/webview-runtimes/linux-cef.json` pour les paquets publiés.
- `check-linux-cef-runtime-release.mjs` : vérifie que les ressources publiées contiennent le fichier ZIP d'exécution CEF nommé par le manifeste activé et que son SHA-256 correspond.
- `cef/linux/fennara_cef_helper.cpp` : sources du petit processus auxiliaire CEF utilisé lors de la compilation de l'auxiliaire d'exécution depuis le SDK CEF.

Les scripts CEF agissent uniquement sur les fichiers copiés dans la zone de préparation. Ils ne doivent pas modifier l'arborescence du SDK CEF téléchargé ou source.

<a id="development-tests"></a>
## Tests de développement

- `test-run-scene-edit-script-inspect.mjs` : crée un projet de test Godot ignoré sous `temp/` et vérifie l'inspection de `PackedScene` importées, les protections du contexte en lecture seule, l'échec en cas de source manquante et l'absence d'enregistrement avec une GDExtension d'éditeur compilée.

<a id="documentation-localization"></a>
## Localisation de la documentation

- `sync-doc-navigation.mjs` : ajoute les hachages de source, les ancres stables et le sélecteur compact de langue pour une même page, sans traduire la prose.
- `check-doc-i18n.mjs` : valide la couverture complète des langues, l'actualité des sources, la navigation, les ancres, la structure Markdown, le code protégé, les URL et les liens.
- `doc-i18n-lib.mjs` : possède le manifeste partagé des langues, la normalisation des sources, le rendu de la navigation et les auxiliaires structurels.

Exécutez :

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

L'ensemble des langues et des documents est déclaré dans `docs/i18n/languages.json`.
L'anglais reste canonique. La prose traduite doit être rédigée à partir de la
source anglaise, et non générée par ces scripts.

La synchronisation normale met à jour la navigation et les ancres stables, mais
préserve les hachages de source existants. Après avoir directement mis à jour
les neuf traductions d'une page anglaise modifiée, actualisez délibérément cette
seule source :

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

L'option peut être répétée pour plusieurs sources relues. Ne validez pas une
source dont la prose traduite n'a pas été mise à jour. La CI exécute
`sync-doc-navigation.mjs --check` avant le validateur complet des traductions.

<a id="ui-sync"></a>
## Synchronisation de l'interface

- `sync-chat-ui.mjs` : copie `ui/chat/` dans `godot_demo/addons/fennara/dist/`.

`godot_demo/addons/fennara/dist/` est volontairement commité, car les fichiers ZIP publiés de l'addon doivent contenir la webview de chat compilée. Effectuez les modifications dans `ui/chat/`, exécutez le script de synchronisation, puis commitez ensemble les sources et les ressources générées de l'addon.

<a id="runtime-sync"></a>
## Synchronisation de l'environnement d'exécution

- `sync-runtime.mjs` : copie `runtime/` dans `godot_demo/addons/fennara/runtime/`.

`godot_demo/addons/fennara/runtime/` est volontairement commité, car les fichiers ZIP publiés de l'addon doivent contenir les scripts auxiliaires d'exécution côté Godot. Effectuez les modifications dans `runtime/`, exécutez le script de synchronisation, puis commitez ensemble les sources et les ressources générées de l'addon.

<a id="guidance-sync"></a>
## Synchronisation des instructions

- `sync-guidance.mjs` : copie les instructions compactes et les pages de connaissances à la demande depuis `local/templates/` vers `godot_demo/addons/fennara/ai/`, conformément aux fichiers que `fennara install` et `fennara update` écrivent dans les projets des utilisateurs.

`godot_demo/addons/fennara/ai/` est volontairement commité, car l'addon de démonstration reproduit la disposition d'un addon installé. Effectuez les modifications dans `local/templates/`, exécutez le script de synchronisation, puis commitez ensemble les sources et les instructions générées de l'addon.

<a id="boundaries"></a>
## Limites

- Les scripts peuvent créer les résultats `.package-preview/` et `dist/` à la racine.
- Les scripts peuvent mettre à jour les charges utiles générées et commitées uniquement lorsque c'est explicitement leur fonction, comme `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs` ou `set-version.mjs`.
- Les scripts ne doivent pas écrire le cache de l'éditeur Godot, les installations locales dans les données d'application, les ressources publiées téléchargées ou les résultats de tests en machine virtuelle dans les répertoires de sources suivis.
