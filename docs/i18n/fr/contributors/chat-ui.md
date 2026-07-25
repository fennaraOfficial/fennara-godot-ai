<!-- fennara-i18n: locale=fr source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Interface de chat Fennara

<!-- fennara-doc-nav:start -->
[Documentation](../README.md)

🌐 **Langues**

[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · [Español](../../es/contributors/chat-ui.md) · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · [日本語](../../ja/contributors/chat-ui.md) · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · **Français** · [Deutsch](../../de/contributors/chat-ui.md) · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

Ce dossier contient les sources de l'interface de chat facultative intégrée à l'éditeur.

La première version ne nécessite volontairement aucune compilation : elle utilise du HTML, du CSS et du JavaScript simples.
Le dépôt OSS reste ainsi facile à inspecter et évite l'ajout d'une chaîne d'outils frontend
avant la stabilisation de l'hôte de la webview et du pont de chat du daemon.

La copie empaquetée se trouve dans `godot_demo/addons/fennara/dist/`.

Après avoir modifié ce dossier, exécutez :

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## Notes de conception

- Respectez les surfaces de l'éditeur Godot : contrôles compacts, contraste discret,
  petits rayons, états de focus clairs et aucun traitement de bannière promotionnelle.
- Utilisez uniquement les API locales du daemon et du chat Fennara. N'exigez aucun service hébergé.
- La prise en charge d'OpenRouter doit utiliser une clé fournie par l'utilisateur et conservée localement hors du projet Godot.
- Gardez l'interface utile sans connexion à un modèle : l'état, les réglages, la transcription et les états de la zone de rédaction doivent rester visibles.
