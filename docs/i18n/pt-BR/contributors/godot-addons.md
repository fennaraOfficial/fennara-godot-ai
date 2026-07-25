<!-- fennara-i18n: locale=pt-BR source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Addons do Godot

<!-- fennara-doc-nav:start -->
[Documentação](../README.md)

🌐 **Idiomas**

[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · **Português do Brasil** · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

Este diretório espelha a estrutura esperada pelo Godot dentro de um projeto:

```text
res://addons/
  fennara/
```

Manter o payload do repositório em `godot_demo/addons/` permite que scripts de
empacotamento e testes locais copiem o addon para um projeto sem reorganizar caminhos.

<a id="current-addon"></a>
## Addon atual

`fennara/` é o addon instalável Fennara Godot AI. Ele contém:

- `fennara.gdextension`, o ponto de entrada do Godot para a extensão nativa.
- `bin/`, binários de editor das plataformas compilados de `fennara-cpp/`.
- `dist/`, recursos gerados da webview nativa do chat, sincronizados de `ui/chat/`.
- `runtime/`, scripts auxiliares do lado do Godot sincronizados do código-fonte `runtime/` na raiz do repositório.
- `debugger/`, recursos do addon voltados ao depurador.
- `VERSION`, o marcador de versão do addon empacotado.

<a id="rules"></a>
## Regras

- Mantenha estáveis os caminhos relativos ao addon. Projetos de usuários recebem esta pasta como `res://addons/fennara/`.
- Não coloque aqui zips de package preview ou lançamento, arquivos CEF baixados, logs nem saídas de testes locais.
- Não edite manualmente arquivos gerados da webview em `fennara/dist/`, a menos que esteja corrigindo intencionalmente a saída gerada e também sincronize a alteração no código-fonte.
- Não edite manualmente arquivos auxiliares de runtime sincronizados em `fennara/runtime/` sem também atualizar `runtime/` e executar `node scripts/sync-runtime.mjs`.
- Adicione novos payloads de addon aqui somente se eles forem destinados a ser copiados para projetos Godot.
