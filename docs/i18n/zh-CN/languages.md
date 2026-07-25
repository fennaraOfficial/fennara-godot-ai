<!-- fennara-i18n: locale=zh-CN source=docs/languages.md sha256=29ca1071b436e0ff29fa5d18d9e2b09cbe64749513ea7f4e1e6471569fcb6456 -->
<a id="languages-and-translation-status"></a>
# 语言与翻译状态

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · **简体中文** · [Español](../es/languages.md) · [Português do Brasil](../pt-BR/languages.md) · [日本語](../ja/languages.md) · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · [Deutsch](../de/languages.md) · [Türkçe](../tr/languages.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../languages.md)
<!-- fennara-doc-nav:end -->

英文是规范文档源。Fennara 还提供由 AI 编写的九种语言完整翻译。每个翻译页面都会链接到当前英文源，并邀请母语使用者审阅。

| 语言 | 文档 | 覆盖率 | 审阅状态 |
| --- | --- | --- | --- |
| English | [English documentation](../../README.md) | 29/29 | 规范源 |
| 简体中文 | [简体中文文档](README.md) | 29/29 | 征求母语审阅 |
| Español | [Documentación en español](../es/README.md) | 29/29 | 征求母语审阅 |
| Português do Brasil | [Documentação em português](../pt-BR/README.md) | 29/29 | 征求母语审阅 |
| 日本語 | [日本語ドキュメント](../ja/README.md) | 29/29 | 征求母语审阅 |
| 한국어 | [한국어 문서](../ko/README.md) | 29/29 | 征求母语审阅 |
| Русский | [Документация на русском](../ru/README.md) | 29/29 | 征求母语审阅 |
| Français | [Documentation en français](../fr/README.md) | 29/29 | 征求母语审阅 |
| Deutsch | [Deutsche Dokumentation](../de/README.md) | 29/29 | 征求母语审阅 |
| Türkçe | [Türkçe belgeler](../tr/README.md) | 29/29 | 征求母语审阅 |

<a id="what-is-translated"></a>
## 翻译范围

翻译集合包含主 README、`docs/` 直属的每个页面、`CONTRIBUTING.md`、`CONTEXT.md`、`SECURITY.md`，以及六个面向贡献者的子系统 README。

法律文本、第三方声明、issue 模板、内部智能体指令、生成的项目指南、测试夹具和供应商文档保留其权威原文。生成文件或承载行为的文件不是独立翻译源。

<a id="freshness-and-validation"></a>
## 新鲜度与验证

每个翻译页面都记录规范源路径及其源哈希。导航由单一 locale 清单生成，稳定的英文锚点别名可在标题翻译后继续支持深层链接。

运行：

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

这些工具不会翻译正文。它们只维护导航元数据，并检查覆盖率、新鲜度、Markdown 结构、命令、链接、锚点、代码块、表格和 URL。欢迎母语使用者通过常规拉取请求进行修正。

常规同步会保留现有源哈希，因此英文正文发生更改后，其翻译会一直保持
过期状态，直到被直接更新。审阅某个已更改英文页面的全部九种翻译后，
只确认该规范源：

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI 会先以检查模式运行导航同步，再执行结构验证。结构验证还会确认
每个稳定的英文锚点仍附加在对应的翻译标题上。
