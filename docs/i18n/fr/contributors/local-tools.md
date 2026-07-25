<!-- fennara-i18n: locale=fr source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
<a id="fennara-local-tools"></a>
# Outils locaux de Fennara

<!-- fennara-doc-nav:start -->
[Documentation](../README.md)

🌐 **Langues**

[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · **Français** · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ Traduction rédigée par une IA à partir de la source anglaise. La relecture par des locuteurs natifs est la bienvenue. [Source anglaise](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Ce dossier contient les composants natifs locaux de Fennara.

<a id="daemon"></a>
## Daemon

`crates/fennara-daemon` exécute le daemon Fennara local sur :

```text
http://127.0.0.1:41287
```

Points d'accès :

- `GET /health` : état du daemon.
- `GET /status` : état du daemon et métadonnées du plugin Godot connecté.
- `POST /tools/call` : transmet un appel d'outil au plugin Godot connecté et attend son résultat.
- `WS /godot/ws` : pont du plugin Godot local. Le plugin envoie un message `hello` après la connexion.

Binaire de développement :

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## Serveur MCP

`crates/fennara-mcp` est le serveur MCP local. Il communique en JSON-RPC sur stdio afin que les clients MCP puissent le lancer comme processus local.

`fennara-mcp` intègre à la compilation les schémas destinés à MCP qu'il a
sélectionnés depuis `local/schemas/tools/` et transmet les appels de ces outils
au daemon local. Il n'a pas besoin d'un service de schémas externe à l'exécution.
Le chat intégré sélectionne dans le même répertoire de schémas un ensemble
d'outils lié, mais différent.

`fennara install` écrit également dans le projet Godot les instructions de projet générées depuis `local/templates/` :

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

Compilation :

```powershell
cd local
cargo build
```

Sous Windows, si un terminal n'a pas encore actualisé le PATH de Rust :

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Binaire de développement :

```text
local/target/debug/fennara-mcp.exe
```

Outils actuels :

- `fennara_status` : vérifie que le serveur MCP est installé et accessible, puis indique l'état du daemon et du pont Godot lorsque le daemon fonctionne.
- Les outils de projet Godot comme `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` et `screenshot_scene` sont transmis
  au daemon, qui les transmet au plugin Godot connecté.

Futur chemin utilisateur installé sous Windows :

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
