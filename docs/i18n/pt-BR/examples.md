<!-- fennara-i18n: locale=pt-BR source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# Exemplos

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · [Español](../es/examples.md) · **Português do Brasil** · [日本語](../ja/examples.md) · [한국어](../ko/examples.md) · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · [Deutsch](../de/examples.md) · [Türkçe](../tr/examples.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../examples.md)
<!-- fennara-doc-nav:end -->

Copie um prompt, substitua os detalhes do projeto e envie-o a partir de um
aplicativo MCP ou do chat integrado do Fennara.

| Objetivo | Exemplo |
| --- | --- |
| Confirmar o editor conectado | [Verificar a conexão](#verificar-a-conexão) |
| Entender um projeto existente | [Inspecionar antes de editar](#inspecionar-um-projeto-antes-de-editar) |
| Fazer uma alteração focada | [Alteração ciente da arquitetura](#fazer-uma-pequena-alteração-ciente-da-arquitetura) |
| Diagnosticar um projeto em execução | [Erro de execução](#depurar-um-erro-de-execução) |
| Inspecionar a saída renderizada | [Feedback visual](#feedback-visual) |

<a id="check-connection"></a>
## Verificar a conexão

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

<a id="inspect-a-project-before-editing"></a>
## Inspecionar um projeto antes de editar

```text
Use Fennara MCP to inspect this Godot project. Look at the scene tree, relevant files, diagnostics, and project structure before suggesting changes.
```

<a id="make-a-small-architecture-aware-change"></a>
## Fazer uma pequena alteração ciente da arquitetura

```text
Work inside this existing Godot project like a careful contributor. Inspect how the relevant system is organized, make the smallest useful change, and explain what files/resources changed and how I can test it.
```

<a id="debug-a-runtime-error"></a>
## Depurar um erro de execução

```text
Use Fennara MCP to inspect the latest Godot runtime errors, find the likely source, patch the issue, and explain the fix.
```

<a id="visual-feedback"></a>
## Feedback visual

```text
Use Fennara MCP to capture a screenshot of the current scene, inspect the UI layout, and suggest or make a small fix if something is visibly wrong.
```

<a id="built-in-chat-provider-setup"></a>
## Configuração do provedor do chat integrado

No dock do Fennara dentro do Godot:

```text
/provider
```

Conecte um provedor na nuvem ou um provedor local.

Depois:

```text
/model
```

Escolha o modelo que o dock deve usar.

<a id="existing-project-demo-prompt"></a>
## Prompt de demonstração para um projeto existente

Este é o tipo de prompt usado na demonstração Open RPG:

```text
I want you to work inside this existing Godot RPG project like a careful project contributor. Before making changes, understand how the relevant systems are organized. Reuse the existing architecture and naming style wherever possible. Add the requested feature in the smallest clean way, then tell me what changed and how to try it in-game.
```
