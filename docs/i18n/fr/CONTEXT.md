<!-- fennara-i18n: locale=fr source=CONTEXT.md sha256=ee0d279d8a4916d5cf894616b1c72658669a36bf0ec958efef5a09ee196c704e -->
<a id="fennara-context"></a>
# Contexte de Fennara

<!-- fennara-doc-nav:start -->
[Documentation](README.md)

🌐 **Langues**

[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · [Español](../es/CONTEXT.md) · [Português do Brasil](../pt-BR/CONTEXT.md) · [日本語](../ja/CONTEXT.md) · [한국어](../ko/CONTEXT.md) · [Русский](../ru/CONTEXT.md) · **Français** · [Deutsch](../de/CONTEXT.md) · [Türkçe](../tr/CONTEXT.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

Ce fichier définit les termes courants employés dans la documentation de Fennara, les issues, les notes de publication et les instructions destinées aux agents.

<a id="product-terms"></a>
## Termes du produit

**Fennara**

L'environnement pour agents qui comprend Godot dans ce dépôt. Fennara connecte les outils d'IA à de véritables informations de Godot, comme les diagnostics, la validation des scènes, les erreurs d'exécution, les captures d'écran et les instructions de projet.

**Addon Godot**

Le plugin installable copié dans le projet Godot d'un utilisateur sous `res://addons/fennara/`. Il possède l'interface du dock, les outils d'inspection tournés vers Godot, la bibliothèque GDExtension native, les ressources empaquetées de l'interface de chat, les scripts auxiliaires d'exécution et la version locale de l'addon du projet.

**CLI Fennara**

La commande `fennara` installée sur la machine de l'utilisateur. Elle gère l'installation, la mise à jour, la mise à jour automatique de la CLI, les vérifications du diagnostic, la configuration des applications MCP, les avertissements sur les prérequis de la webview, les vérifications de la configuration C# et les instructions de projet générées.

**Paquet local**

Le fichier ZIP de la version publiée qui contient les exécutables Fennara locaux, comme le serveur MCP, le daemon, les binaires d'exécution et les binaires de lancement pour une plateforme et une architecture données.

**Instructions de projet**

Les fichiers d'instructions générés et placés dans un projet Godot, notamment `AGENTS.md` et les références dirigées sous `addons/fennara/ai/`, afin que les agents de programmation IA sachent quand et comment utiliser Fennara.

<a id="mcp-terms"></a>
## Termes MCP

**Serveur MCP Fennara**

Le serveur MCP stdio local lancé par une application de programmation IA comme Claude Code, Cursor, Cline, Gemini CLI ou un autre client MCP. Il expose les outils Fennara à cette application externe.

**Application MCP**

Une application IA externe configurée par `fennara mcp-setup`. La configuration des applications MCP détermine quelle application externe peut appeler les outils Fennara. Elle ne sélectionne pas le modèle utilisé par le chat intégré de Fennara.

**Cible MCP**

Le projet Godot actuellement sélectionné pour recevoir les appels MCP de Fennara.

**Schéma d'outil**

La description d'un outil MCP Fennara destinée au modèle, notamment ses arguments, ses limites et ses notes de déroulement.

**Enveloppe de résultat d'outil**

Le résultat concis destiné au modèle et renvoyé après un appel d'outil. Les résultats de Fennara doivent expliquer l'état, les constats importants et le prochain contexte utile sans déverser de données brutes inutiles.

<a id="built-in-chat-terms"></a>
## Termes du chat intégré

**Chat intégré**

La propre interface de chat de Fennara dans l'addon Godot ou le navigateur système. Elle est distincte des applications MCP externes. Un utilisateur peut configurer Claude Code pour MCP tout en choisissant un autre fournisseur ou modèle pour le chat intégré.

**Surface de chat**

Le mode d'affichage du chat intégré. Le mode incorporé utilise la webview native du dock Godot. Le mode navigateur sert la même interface depuis le daemon local et l'ouvre dans le navigateur système.

**Fournisseur de chat**

Un backend capable de produire les réponses du chat intégré, comme OpenAI, Anthropic,
OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax,
Ollama local ou LM Studio.

**Référence de modèle**

L'identifiant de modèle qualifié par son fournisseur et sélectionné dans le chat intégré. Les commandes slash comme `/provider` et `/model` aident les utilisateurs à connecter des fournisseurs et à choisir des références de modèle.

**Connexion au fournisseur**

Les réglages locaux et l'état d'authentification gérés par le daemon pour un fournisseur de chat, notamment les clés API ou les URL de base locales. Les secrets des fournisseurs doivent rester dans le stockage local géré par le daemon, pas dans le projet Godot.

**Trace de génération**

Les métadonnées conservées pour une génération du chat intégré, qui relient les messages de l'assistant, les appels d'outil, le choix du fournisseur et du modèle, l'utilisation et les journaux de coût à la génération qui les a produits.

<a id="runtime-and-webview-terms"></a>
## Termes de l'exécution et de la webview

**Daemon Fennara**

Le service local qui connecte les appels MCP et les requêtes du chat intégré à l'addon Godot, stocke l'état d'exécution local et sert les routes de chat hébergées par le daemon, comme `/chat/`.

**Session d'exécution**

Une session d'exécution Godot gérée par le daemon et utilisée pour l'inspection à l'exécution, les journaux, la validation, les captures d'écran et les futurs processus portant sur une scène active.

**Instantané Godot**

Un instantané réversible de l'état du projet pris avant un tour assisté par Fennara susceptible de modifier des fichiers. La création de l'instantané doit se terminer avant l'enregistrement du tour utilisateur, afin qu'un échec de la création ne laisse pas de prompts orphelins.

**Environnement d'exécution de la webview**

La prise en charge par la plateforme nécessaire pour afficher le chat intégré dans Godot ou à proximité. Windows utilise WebView2, macOS utilise WebKit/WKWebView et Linux utilise un environnement d'exécution CEF partagé installé dans les données d'application de Fennara.

**Environnement d'exécution CEF Linux partagé**

La charge utile d'exécution CEF externe utilisée par la webview de chat Linux. Elle est installée une seule fois dans le répertoire des données d'application de Fennara et ne doit pas être intégrée à chaque fichier ZIP de l'addon Godot.

<a id="release-terms"></a>
## Termes des versions publiées

**Manifeste de version**

La ressource JSON nommée `fennara-release-manifest-v<version>.json`. Elle associe les ressources publiées aux plateformes, enregistre les hachages SHA-256, déclare les ressources d'exécution partagées et définit `minimum_cli_version`.

**Version minimale de la CLI**

La plus ancienne version de la CLI `fennara` autorisée à consommer un manifeste
de version. Si une version publiée exige une logique d'installation ou de mise
à jour plus récente, mettez à jour sa piste dans `scripts/release-policy.mjs`.
Le générateur de manifeste applique cette politique après avoir validé l'identité
de la version. Les processus automatisés ne choisissent pas la valeur.

**Dernière version publiée**

Le pointeur Latest Release de GitHub vers une version numérotée précise. Les programmes
d'installation et les mises à jour par défaut résolvent ce pointeur au moyen de l'API
de GitHub. Fennara n'utilise ni étiquette ni version publiée littérale `latest`.
La modification des fichiers sources après la publication ne modifie pas les ressources
publiées. Les ressources de manifeste déjà publiées doivent être remplacées explicitement.
