<!-- fennara-i18n: locale=pt-BR source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# Como contribuir

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · [Español](../es/CONTRIBUTING.md) · **Português do Brasil** · [日本語](../ja/CONTRIBUTING.md) · [한국어](../ko/CONTRIBUTING.md) · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · [Deutsch](../de/CONTRIBUTING.md) · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Obrigado por ajudar a melhorar o Fennara Godot AI.

<a id="good-contributions"></a>
## Boas contribuições

- Correções na documentação
- Correções de bugs reproduzíveis
- Correções de compatibilidade entre plataformas
- Melhorias de compilação e empacotamento
- Pequenas melhorias na clareza da configuração

<a id="design-discussion-required"></a>
## Discussão de design obrigatória

Abra uma issue ou discussão antes de começar:

- novas ferramentas MCP
- alterações nos esquemas das ferramentas
- alterações no fluxo de lançamento
- grandes alterações de arquitetura
- alterações que afetem as orientações de projeto geradas

<a id="pull-requests"></a>
## Pull requests

- Mantenha os pull requests pequenos e focados.
- Explique o que mudou e por quê.
- Explique como você verificou a alteração.
- Inclua capturas de tela ou gravações para alterações visíveis na interface ou na renderização da documentação.
- Não inclua formatação ou limpeza sem relação com a alteração.
- Não cole descrições extensas geradas em issues ou pull requests.

<a id="commit-and-pr-titles"></a>
## Títulos de commits e PRs

Use o estilo Conventional Commits:

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

Tipos comuns:

- `feat`: recurso voltado ao usuário
- `fix`: correção de bug
- `docs`: documentação
- `ci`: GitHub Actions e automação
- `build`: compilação ou empacotamento
- `refactor`: reestruturação de código que preserva o comportamento
- `test`: testes
- `chore`: manutenção

<a id="project-boundaries"></a>
## Limites do projeto

O Fennara deve continuar independente de qualquer jogo específico. Evite APIs ou orientações que pressuponham controles, objetivos, economia, inventário, combate, navegação, missões ou fluxo de interface de um jogo.

Os agentes devem inspecionar as cenas, scripts, recursos, configurações, estado de execução, diagnósticos e capturas de tela reais de um projeto Godot, e então combinar ferramentas genéricas do Fennara para esse projeto.

<a id="documentation-translations"></a>
## Traduções da documentação

O inglês é a fonte canônica. Corrija primeiro o texto em inglês e depois atualize
todas as localidades afetadas. O conjunto traduzido e os metadados de localidade
ficam em `docs/i18n/languages.json`.

- Leia a página completa em inglês e escreva a tradução diretamente. Não use serviços de tradução automática em massa nem scripts de geração de prosa.
- Mantenha blocos de código, código embutido, comandos, caminhos, chaves de configuração, URLs e nomes de produtos exatamente iguais.
- Preserve o marcador de origem e os aliases explícitos de âncoras em inglês mantidos pelos scripts da documentação.
- Não marque uma tradução como revisada por falante nativo sem que um revisor fluente a tenha verificado.
- Não traduza textos jurídicos, prompts internos de agentes, orientações de projeto geradas, arquivos de fornecedores ou fixtures de teste como fontes independentes.

Depois de alterar a documentação canônica ou traduzida, execute:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Esses comandos mantêm os metadados de navegação e validam a estrutura. Eles não
escrevem a prosa traduzida.

A sincronização normal da navegação preserva todos os hashes de origem
existentes. Depois de alterar uma fonte em inglês, atualize diretamente essa
página em todas as nove localidades traduzidas e então aceite
deliberadamente somente essa fonte canônica:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

Repita `--accept-source <path>` para cada página em inglês cujas traduções foram
revisadas e atualizadas. Nunca aceite o hash de uma fonte antes que todas as
nove traduções contenham o novo significado.
