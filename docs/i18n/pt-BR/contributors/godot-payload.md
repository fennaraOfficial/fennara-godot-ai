<!-- fennara-i18n: locale=pt-BR source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Payload do Godot

<!-- fennara-doc-nav:start -->
[Documentação](../README.md)

🌐 **Idiomas**

[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · **Português do Brasil** · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

Este diretório é a árvore de código-fonte do payload do addon voltado ao Godot,
que é copiado para os projetos dos usuários e empacotado nos arquivos de lançamento.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/` deve continuar instalável como um diretório normal
de addon do Godot. Tudo o que for commitado aqui deve poder ser recebido
diretamente por um projeto de usuário em `res://addons/fennara/`.

<a id="what-belongs-here"></a>
## O que pertence aqui

- `addons/fennara/fennara.gdextension` e arquivos `.uid` carregados pelo Godot.
- Binários GDExtension do editor em `addons/fennara/bin/`, produzidos pelas compilações de plataforma.
- Recursos gerados do chat web em `addons/fennara/dist/`, usados pela webview nativa do chat.
- Scripts auxiliares de runtime do lado do Godot em `addons/fennara/runtime/`, sincronizados de `runtime/`.
- `addons/fennara/VERSION`, correspondente ao `VERSION` do repositório durante o empacotamento.

<a id="what-does-not-belong-here"></a>
## O que não pertence aqui

- Estado local do usuário do Godot, como `.godot/`, `.import/`, logs, arquivos temporários ou caches do editor.
- Saídas de pacotes na raiz produzidas pelos fluxos de trabalho. Elas pertencem a pastas de compilação ignoradas, como `dist/` ou `.package-preview/`.
- Payloads de runtime local compartilhados, como executáveis do daemon ou MCP do Fennara e o runtime CEF do Linux. Eles são instalados no diretório de dados de aplicativo do usuário pela CLI, e não copiados para cada addon de projeto Godot.

<a id="generated-files"></a>
## Arquivos gerados

O código-fonte da interface de chat fica em `ui/chat/`. Depois de alterá-lo, execute:

```powershell
node scripts\sync-chat-ui.mjs
```

Isso sincroniza os arquivos compilados da webview em `godot_demo/addons/fennara/dist/`,
que é intencionalmente commitado porque usuários do addon não devem precisar do
Node.js nem de uma etapa de compilação do frontend.

O código-fonte dos auxiliares de runtime fica em `runtime/`. Depois de alterá-lo, execute:

```powershell
node scripts\sync-runtime.mjs
```

Isso sincroniza os auxiliares de runtime do lado do Godot em
`godot_demo/addons/fennara/runtime/`, que é intencionalmente commitado porque os
usuários do addon devem receber esses scripts com o zip de lançamento.
