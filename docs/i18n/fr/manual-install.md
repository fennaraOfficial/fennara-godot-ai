<!-- fennara-i18n: locale=fr source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# Installation manuelle

<!-- fennara-doc-nav:start -->
[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · **Français** · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Utilisez cette page uniquement si vous devez assembler Fennara sans le processus
d'installation de Godot ni `fennara install`.

> [!TIP]
> Sous Windows et Linux, la plupart des utilisateurs doivent ajouter
> `addons/fennara` au projet, ouvrir le dock Fennara et appuyer sur
> **Set Up Fennara**. Sous macOS, utilisez la CLI. Consultez [Installation](setup.md).

> [!IMPORTANT]
> L'installation manuelle du fichier ZIP de l'addon n'est pas recommandée sous
> macOS. L'addon contient une bibliothèque native qui n'est actuellement pas
> notariée par Apple. Le téléchargement dans un navigateur suivi de l'extraction
> dans Finder peut amener macOS à signaler qu'il ne peut pas vérifier que
> `libfennara.macos.editor` est exempt de logiciels malveillants. Utilisez
> [l'installation par la CLI](setup.md#install-from-the-terminal-recommended-on-macos)
> pour éviter cette notification. Si elle apparaît déjà, fermez Godot, supprimez
> le dossier `addons/fennara/` copié manuellement et exécutez `fennara install`.

L'installation manuelle comporte quatre parties : la CLI, l'addon du projet,
le paquet d'exécution local partagé et la configuration facultative de l'application MCP.

<a id="1-download-release-files"></a>
## 1. Télécharger les fichiers de la version publiée

Ouvrez la dernière version publiée sur GitHub :

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

Téléchargez le manifeste de version, les fichiers de votre plateforme et le
fichier ZIP partagé de l'addon.

| Objectif | Ressource |
| --- | --- |
| Plan de publication et valeurs SHA-256 | `fennara-release-manifest-v<version>.json` |
| CLI Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip` |
| Environnement d'exécution local Windows x86_64 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| CLI Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip` |
| Environnement d'exécution local Linux x86_64 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Webview intégrée Linux x86_64 | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| CLI macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip` |
| Environnement d'exécution local macOS arm64 | `fennara-release-local-macos-arm64-v<version>.zip` |
| Addon versionné pour toutes les plateformes | `fennara-release-addon-v<version>.zip` |

La version publiée comprend également cet alias d'addon au nom stable pour la
documentation et les téléchargements manuels :

```text
fennara-addon-latest.zip
```

Le manifeste enregistre les valeurs SHA-256 attendues pour l'environnement
d'exécution local, l'addon et les ressources d'exécution partagées. Utilisez-le
comme source de référence pour vérifier les téléchargements manuels.

<a id="2-install-the-cli"></a>
## 2. Installer la CLI

Extrayez le fichier ZIP `fennara-cli`.

Ajoutez son répertoire `bin` à PATH, ou copiez le binaire `fennara` dans l'un
de vos dossiers PATH existants.

Vérifiez-le :

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Installer l'addon Godot

Extrayez le fichier ZIP `fennara-addon`.

Copiez :

```text
addons/fennara
```

dans votre projet Godot afin que le projet contienne :

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. Installer le paquet d'exécution local

La CLI gère normalement cette étape pour vous. La configuration manuelle de
l'environnement d'exécution est nécessaire uniquement si vous évitez
`fennara install`.

Répertoires de données Fennara par défaut :

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

La disposition attendue est :

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

Sous Windows, les binaires utilisent l'extension `.exe`.

`current.json` dirige les binaires des lanceurs vers la version active de
l'environnement d'exécution. Les commandes normales `fennara install` et
`fennara update` créent automatiquement ce fichier.

Le chat intégré sous Linux utilise l'emplacement partagé
`webview/cef/linux-x64/<cef-version>/` de l'environnement d'exécution. Les
exécutions normales de `fennara install` et `fennara update` installent
automatiquement l'environnement d'exécution CEF géré par la version à partir
du manifeste et de la ressource publiés. Si vous installez tout à la main,
extrayez `fennara-webview-cef-linux-x64-<cef-version>.zip` dans cet emplacement
d'exécution partagé et écrivez le marqueur `webview/cef/linux-x64/current.json`
correspondant. Conservez cette charge utile hors de l'addon du projet Godot.
`addons/fennara` ne doit contenir ni `libcef.so` ni aucun autre fichier d'exécution CEF.

Cette charge utile CEF sert uniquement au chat intégré sous Linux. Les utilisateurs
peuvent choisir **Open chat in my system browser next time** dans Chat Settings
pour afficher le même chat intégré par l'intermédiaire du daemon local dans leur
navigateur système, au lieu de la webview Godot intégrée.

La disposition CEF finale sous Linux doit être la suivante :

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json` doit contenir :

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` doit être le
manifeste de version correspondant à la ressource CEF, par exemple :

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Ne placez pas l'état modifiable du navigateur dans le répertoire versionné de
CEF. L'utilisation normale écrit les profils et journaux propres à chaque
éditeur sous les racines de cache et de journaux des données d'application de
Fennara. La charge utile de l'environnement d'exécution reste partagée et en
lecture seule.

<a id="5-configure-your-mcp-app"></a>
## 5. Configurer votre application MCP

Après l'installation du paquet d'exécution local, configurez votre application MCP :

```bash
fennara mcp-setup --claude
```

Autres cibles :

```bash
fennara mcp-setup --help
```

Redémarrez l'application MCP après la configuration.

Si votre application ne figure pas dans la liste, ou si vous modifiez manuellement
la configuration MCP pendant cette installation, consultez [Configuration MCP](mcp-setup.md)
pour le chemin stable du lanceur et les exemples JSON et TOML.

Cette étape connecte uniquement l'application MCP externe aux outils Godot de
Fennara. Elle ne configure pas le fournisseur de modèle du dock de chat Fennara
intégré. Configurez le dock dans Godot si vous souhaitez utiliser le chat intégré,
ou consultez [Applications MCP et chat intégré](chat-vs-mcp.md).

<a id="6-verify"></a>
## 6. Vérifier

Ouvrez le projet Godot, puis demandez à votre application MCP :

```text
Utilise Fennara MCP pour exécuter fennara_status et indique-moi quel projet Godot est connecté.
```

Si le chemin est correct, l'installation manuelle fonctionne.

<a id="recommended-shortcut"></a>
## Raccourci recommandé

Même si vous installez la CLI manuellement, vous pouvez lui confier l'installation
de l'addon et du paquet d'exécution local :

```bash
cd path/to/your-godot-project
fennara install
```

La CLI écrit également les instructions de projet destinées aux agents de programmation IA :

```text
AGENTS.md
addons/fennara/ai/
```

Le répertoire IA contient des instructions compactes toujours lues, un index et
des pages spécialisées chargées uniquement lorsqu'elles sont pertinentes. Un
fichier ZIP d'addon copié manuellement peut contenir ce répertoire empaqueté,
mais il ne crée ni n'actualise le fichier `AGENTS.md` à la racine du projet.
Utilisez `fennara install` et `fennara update` lorsque Fennara doit gérer et
actualiser l'ensemble des instructions de projet.
