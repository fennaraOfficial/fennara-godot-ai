<!-- fennara-i18n: locale=pt-BR source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Explicação da demonstração Open RPG

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · [Español](../es/open-rpg-demo.md) · **Português do Brasil** · [日本語](../ja/open-rpg-demo.md) · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · [Deutsch](../de/open-rpg-demo.md) · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

Vídeo:

https://www.youtube.com/watch?v=0Egu3S-9MM0

Esta demonstração testa o Fennara MCP no projeto de código aberto Godot 4 Open RPG da GDQuest.

O objetivo da demonstração não é mostrar uma IA criando um projeto vazio do zero. O ponto é que um agente de IA trabalhou dentro de uma base de código RPG existente no Godot, cometeu erros, recebeu feedback do Godot, corrigiu a implementação e continuou.

<a id="project"></a>
## Projeto

GDQuest Godot 4 Open RPG:

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## Tarefa

Adicionar um recurso de progressão no qual Baloo, o Bear player battler, desbloqueia uma nova habilidade de combate chamada Tactical Guard após vencer um encontro existente.

A habilidade precisava:

- mirar em um inimigo
- causar dano moderado
- aumentar a Defense de Baloo
- aparecer no menu de ações de combate de Baloo após o desbloqueio
- mostrar uma mensagem como `Baloo learned Tactical Guard!` após o desbloqueio

<a id="what-happened"></a>
## O que aconteceu

Um agente de programação com IA se conectou ao projeto Godot em execução por meio do Fennara MCP e inspecionou a arquitetura do projeto.

Ele usou ferramentas do Fennara para:

- inspeção da árvore de cena
- inspeção das propriedades dos nós
- diagnósticos de GDScript
- validação de cenas
- feedback de erros em execução
- inspeção do projeto e das cenas

A primeira implementação não funcionou perfeitamente. Essa foi a parte útil.

O Fennara retornou feedback do Godot, o agente corrigiu o script quebrado, ajustou a implementação e continuou até que o recurso funcionasse dentro do jogo.

<a id="why-this-matters"></a>
## Por que isso importa

Demonstrações em projetos vazios são fáceis. É nos projetos existentes que agentes de IA normalmente falham.

A tese do Fennara é que agentes de IA para Godot precisam de feedback do motor:

- O script foi analisado?
- A cena foi validada?
- O runtime emitiu um erro?
- O agente inspecionou a estrutura real do projeto?
- O agente consegue corrigir o erro em vez de fingir que a tarefa foi concluída?

O MCP tradicional oferece comandos à IA.

O Fennara oferece à IA feedback do Godot.
