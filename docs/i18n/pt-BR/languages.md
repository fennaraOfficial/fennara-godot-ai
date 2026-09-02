<!-- fennara-i18n: locale=pt-BR source=docs/languages.md sha256=476e4afb5a17f76474a25864d08280c73ba0e4fdd1ebcfbbfd814d010dfac932 -->
<a id="languages-and-translation-status"></a>
# Idiomas e status das traduções

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · [Español](../es/languages.md) · **Português do Brasil** · [日本語](../ja/languages.md) · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · [Deutsch](../de/languages.md) · [Türkçe](../tr/languages.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../languages.md)
<!-- fennara-doc-nav:end -->

O inglês é a fonte canônica da documentação. O Fennara também oferece traduções
completas, escritas por IA, em nove idiomas. Cada página traduzida contém um link
para sua fonte atual em inglês e convida falantes nativos a revisá-la.

| Idioma | Documentação | Cobertura | Status da revisão |
| --- | --- | --- | --- |
| English | [Documentação em inglês](../../README.md) | 31/31 | Canônica |
| 简体中文 | [简体中文文档](../zh-CN/README.md) | 31/31 | Revisão nativa solicitada |
| Español | [Documentación en español](../es/README.md) | 31/31 | Revisão nativa solicitada |
| Português do Brasil | [Documentação em português](README.md) | 31/31 | Revisão nativa solicitada |
| 日本語 | [日本語ドキュメント](../ja/README.md) | 31/31 | Revisão nativa solicitada |
| 한국어 | [한국어 문서](../ko/README.md) | 31/31 | Revisão nativa solicitada |
| Русский | [Документация на русском](../ru/README.md) | 31/31 | Revisão nativa solicitada |
| Français | [Documentation en français](../fr/README.md) | 31/31 | Revisão nativa solicitada |
| Deutsch | [Deutsche Dokumentation](../de/README.md) | 31/31 | Revisão nativa solicitada |
| Türkçe | [Türkçe belgeler](../tr/README.md) | 31/31 | Revisão nativa solicitada |

<a id="what-is-translated"></a>
## O que é traduzido

O conjunto traduzido contém o README principal, todas as páginas diretamente em
`docs/`, `CONTRIBUTING.md`, `CONTEXT.md`, `SECURITY.md` e os seis
READMEs de subsistemas voltados aos colaboradores.

Textos jurídicos, avisos de terceiros, modelos de issues, instruções internas
para agentes, orientações de projeto geradas, fixtures de teste e documentação
de fornecedores permanecem em sua forma oficial. Arquivos gerados ou que
definem comportamento não são fontes de tradução independentes.

<a id="freshness-and-validation"></a>
## Atualização e validação

Cada página traduzida registra o caminho e o hash de sua fonte canônica.
A navegação é gerada a partir de um único manifesto de localidades, e aliases
estáveis de âncoras em inglês mantêm links profundos funcionando quando os
títulos são traduzidos.

Execute:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Essas ferramentas não traduzem a prosa. Elas apenas mantêm os metadados de
navegação e verificam cobertura, atualização, estrutura Markdown, comandos,
links, âncoras, blocos de código, tabelas e URLs. Correções por falantes nativos
são bem-vindas por meio de pull requests normais.

A sincronização normal preserva os hashes de origem existentes, portanto uma
alteração na prosa em inglês deixa suas traduções desatualizadas até que sejam
atualizadas diretamente. Depois de revisar todas as nove traduções de uma
página em inglês alterada, aceite somente essa fonte:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

A CI executa a sincronização da navegação no modo de verificação antes da
validação estrutural, que também confirma que cada âncora estável em inglês
continua associada ao título traduzido correspondente.
