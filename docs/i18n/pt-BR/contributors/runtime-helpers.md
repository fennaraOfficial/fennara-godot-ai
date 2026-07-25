<!-- fennara-i18n: locale=pt-BR source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# Auxiliares de runtime

<!-- fennara-doc-nav:start -->
[Documentação](../README.md)

🌐 **Idiomas**

[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · **Português do Brasil** · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

Esta pasta contém o código-fonte dos scripts auxiliares do lado do Godot usados
por `runtime_session` e `runtime_script`.

A cópia no addon empacotado fica em:

```text
godot_demo/addons/fennara/runtime/
```

Depois de editar os arquivos aqui, execute:

```bash
node scripts/sync-runtime.mjs
```

Os scripts de runtime continuam carregando esses auxiliares de
`res://addons/fennara/runtime/` dentro de um projeto Godot instalado. Mantenha
os auxiliares primitivos e independentes do projeto: entrada, espera, snapshots
de nós, capturas, consultas de física e suporte ao ciclo de vida de cenas são
adequados. Suposições específicas sobre movimento, combate, missões, inventário
ou fluxo de interface de um jogo não são.

`image_sheet.gd` também é usado pela fachada de scripts de captura de tela.
Mantenha sua composição determinística e independente do estado da cena, da
animação ou da jogabilidade.
