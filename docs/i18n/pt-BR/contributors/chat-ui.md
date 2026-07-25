<!-- fennara-i18n: locale=pt-BR source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Interface de chat do Fennara

<!-- fennara-doc-nav:start -->
[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · [Español](../../es/contributors/chat-ui.md) · **Português do Brasil** · [日本語](../../ja/contributors/chat-ui.md) · [한국어](../../ko/contributors/chat-ui.md) · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · [Deutsch](../../de/contributors/chat-ui.md) · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

Esta pasta contém o código-fonte da interface de chat opcional dentro do editor.

A primeira versão não exige compilação de propósito: apenas HTML, CSS e JavaScript.
Isso mantém o repositório OSS fácil de inspecionar e evita adicionar uma cadeia
de ferramentas de frontend antes que o host da webview e a ponte de chat do
daemon estejam consolidados.

A cópia empacotada fica em `godot_demo/addons/fennara/dist/`.

Depois de editar esta pasta, execute:

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## Observações de design

- Corresponda às interfaces do editor Godot: controles compactos, contraste
  discreto, raios pequenos, estados de foco claros e nenhum tratamento de hero
  com estilo de marketing.
- Use apenas APIs locais de daemon e chat do Fennara. Não exija serviços hospedados.
- O suporte ao OpenRouter deve usar uma chave fornecida pelo usuário, armazenada localmente fora do projeto Godot.
- Mantenha a interface útil sem uma conexão de modelo: os estados de status, configurações, transcrição e compositor ainda devem estar visíveis.
