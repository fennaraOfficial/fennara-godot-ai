# Languages And Translation Status

<!-- fennara-doc-nav:start -->
[Documentation](README.md)

🌐 **Languages**

**English** · [简体中文](i18n/zh-CN/languages.md) · [Español](i18n/es/languages.md) · [Português do Brasil](i18n/pt-BR/languages.md) · [日本語](i18n/ja/languages.md) · [한국어](i18n/ko/languages.md) · [Русский](i18n/ru/languages.md) · [Français](i18n/fr/languages.md) · [Deutsch](i18n/de/languages.md) · [Türkçe](i18n/tr/languages.md)
<!-- fennara-doc-nav:end -->

English is the canonical documentation source. Fennara also provides complete
AI-authored translations in nine languages. Every translated page links to its
current English source and invites review from native speakers.

| Language | Documentation | Coverage | Review status |
| --- | --- | --- | --- |
| English | [English documentation](README.md) | 30/30 | Canonical |
| 简体中文 | [简体中文文档](i18n/zh-CN/README.md) | 30/30 | Native review requested |
| Español | [Documentación en español](i18n/es/README.md) | 30/30 | Native review requested |
| Português do Brasil | [Documentação em português](i18n/pt-BR/README.md) | 30/30 | Native review requested |
| 日本語 | [日本語ドキュメント](i18n/ja/README.md) | 30/30 | Native review requested |
| 한국어 | [한국어 문서](i18n/ko/README.md) | 30/30 | Native review requested |
| Русский | [Документация на русском](i18n/ru/README.md) | 30/30 | Native review requested |
| Français | [Documentation en français](i18n/fr/README.md) | 30/30 | Native review requested |
| Deutsch | [Deutsche Dokumentation](i18n/de/README.md) | 30/30 | Native review requested |
| Türkçe | [Türkçe belgeler](i18n/tr/README.md) | 30/30 | Native review requested |

## What Is Translated

The translated set contains the main README, every page directly under
`docs/`, `CONTRIBUTING.md`, `CONTEXT.md`, `SECURITY.md`, and the six
contributor-facing subsystem READMEs.

Legal text, third-party notices, issue templates, internal agent instructions,
generated project guidance, test fixtures, and vendor documentation remain in
their authoritative form. Generated or behavior-bearing files are not
independent translation sources.

## Freshness And Validation

Each translated page records its canonical source path and source hash.
Navigation is generated from one locale manifest, and stable English anchor
aliases keep deep links working when headings are translated.

Run:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

These tools do not translate prose. They only maintain navigation metadata and
check coverage, freshness, Markdown structure, commands, links, anchors, code
blocks, tables, and URLs. Native-speaker corrections are welcome through normal
pull requests.

Normal sync preserves existing source hashes, so an English prose change leaves
its translations stale until they are directly updated. After reviewing all
nine translations for one changed English page, acknowledge only that source:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI runs navigation sync in check mode before structural validation, which also
verifies that every stable English anchor remains attached to the corresponding
translated heading.
