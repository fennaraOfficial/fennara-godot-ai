<!-- fennara-i18n: locale=fr source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Charge utile Godot

<!-- fennara-doc-nav:start -->
[Documentation](../README.md)

🌐 **Langues**

[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · **Français** · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

Ce répertoire est l'arborescence source de la charge utile de l'addon tournée vers Godot, copiée dans les projets des utilisateurs et intégrée aux archives des versions publiées.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` doit rester installable comme un répertoire d'addon Godot normal. Tout élément commité ici doit pouvoir être reçu directement par un projet utilisateur sous `res://addons/fennara/`.

<a id="what-belongs-here"></a>
## Ce qui appartient à ce répertoire

- `addons/fennara/fennara.gdextension` et les fichiers `.uid` chargés par Godot.
- Les binaires GDExtension de l'éditeur dans `addons/fennara/bin/`, produits par les compilations de chaque plateforme.
- Les ressources générées du chat web dans `addons/fennara/dist/`, utilisées par la webview native du chat.
- Les scripts auxiliaires d'exécution côté Godot dans `addons/fennara/runtime/`, synchronisés depuis `runtime/`.
- `addons/fennara/VERSION`, qui correspond au fichier `VERSION` du dépôt lors de la création des paquets.

<a id="what-does-not-belong-here"></a>
## Ce qui n'appartient pas à ce répertoire

- L'état utilisateur local de Godot comme `.godot/`, `.import/`, les journaux, les fichiers temporaires ou les caches de l'éditeur.
- Les résultats de paquets à la racine issus des processus automatisés. Ils appartiennent à des répertoires de compilation ignorés comme `dist/` ou `.package-preview/`.
- Les charges utiles d'exécution locales partagées comme les exécutables du daemon et du serveur MCP de Fennara ou l'environnement CEF de Linux. La CLI les installe dans le répertoire de données d'application Fennara de l'utilisateur. Elles ne sont pas copiées dans l'addon de chaque projet Godot.

<a id="generated-files"></a>
## Fichiers générés

Les sources de l'interface de chat résident sous `ui/chat/`. Après les avoir modifiées, exécutez :

```powershell
node scripts\sync-chat-ui.mjs
```

Cette commande synchronise les fichiers de la webview compilée dans `godot_demo/addons/fennara/dist/`. Ils sont volontairement commités, car les utilisateurs de l'addon ne doivent pas avoir besoin de Node.js ni d'une étape de compilation frontend.

Les sources des auxiliaires d'exécution résident sous `runtime/`. Après les avoir modifiées, exécutez :

```powershell
node scripts\sync-runtime.mjs
```

Cette commande synchronise les auxiliaires d'exécution côté Godot dans `godot_demo/addons/fennara/runtime/`. Ils sont volontairement commités, car les utilisateurs de l'addon doivent recevoir ces scripts dans le fichier ZIP de la version publiée.
