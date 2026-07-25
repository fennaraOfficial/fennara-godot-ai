<!-- fennara-i18n: locale=fr source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Addons Godot

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · **Français** · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

Ce répertoire reproduit la structure attendue par Godot dans un projet :

```text
res://addons/
  fennara/
```

Conserver la charge utile du dépôt sous `godot_demo/addons/` permet aux scripts de création de paquets et de tests locaux de copier l'addon dans un projet sans modifier la structure des chemins.

<a id="current-addon"></a>
## Addon actuel

`fennara/` est l'addon installable Fennara Godot AI. Il contient :

- `fennara.gdextension`, le point d'entrée Godot de l'extension native.
- `bin/`, les binaires d'éditeur des plateformes compilés depuis `fennara-cpp/`.
- `dist/`, les ressources générées de la webview du chat natif, synchronisées depuis `ui/chat/`.
- `runtime/`, les scripts auxiliaires côté Godot synchronisés depuis les sources `runtime/` à la racine du dépôt.
- `debugger/`, les ressources de l'addon destinées au débogueur.
- `VERSION`, le marqueur de version de l'addon empaqueté.

<a id="rules"></a>
## Règles

- Gardez les chemins relatifs à l'addon stables. Les projets des utilisateurs reçoivent ce dossier sous `res://addons/fennara/`.
- Ne placez ici ni fichiers ZIP de prévisualisation ou de version publiée, ni archives CEF téléchargées, ni journaux, ni résultats de tests locaux.
- Ne modifiez pas manuellement les fichiers générés de la webview dans `fennara/dist/`, sauf si vous corrigez volontairement le résultat généré puis synchronisez également la modification des sources.
- Ne modifiez pas manuellement les fichiers auxiliaires synchronisés dans `fennara/runtime/` sans mettre également à jour `runtime/` et exécuter `node scripts/sync-runtime.mjs`.
- Ajoutez de nouvelles charges utiles d'addon ici uniquement si elles doivent être copiées dans les projets Godot.
