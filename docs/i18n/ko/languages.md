<!-- fennara-i18n: locale=ko source=docs/languages.md sha256=476e4afb5a17f76474a25864d08280c73ba0e4fdd1ebcfbbfd814d010dfac932 -->
<a id="languages-and-translation-status"></a>
# 언어 및 번역 상태

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · [Español](../es/languages.md) · [Português do Brasil](../pt-BR/languages.md) · [日本語](../ja/languages.md) · **한국어** · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · [Deutsch](../de/languages.md) · [Türkçe](../tr/languages.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../languages.md)
<!-- fennara-doc-nav:end -->

영어가 문서의 정본입니다. Fennara는 AI가 직접 작성한 완전한 번역을 아홉 개 언어로도 제공합니다. 모든 번역 페이지는 현재 영문 원본으로 연결되며 원어민 검토를 요청합니다.

| 언어 | 문서 | 범위 | 검토 상태 |
| --- | --- | --- | --- |
| English | [English documentation](../../README.md) | 31/31 | 정본 |
| 简体中文 | [简体中文文档](../zh-CN/README.md) | 31/31 | 원어민 검토 요청 |
| Español | [Documentación en español](../es/README.md) | 31/31 | 원어민 검토 요청 |
| Português do Brasil | [Documentação em português](../pt-BR/README.md) | 31/31 | 원어민 검토 요청 |
| 日本語 | [日本語ドキュメント](../ja/README.md) | 31/31 | 원어민 검토 요청 |
| 한국어 | [한국어 문서](README.md) | 31/31 | 원어민 검토 요청 |
| Русский | [Документация на русском](../ru/README.md) | 31/31 | 원어민 검토 요청 |
| Français | [Documentation en français](../fr/README.md) | 31/31 | 원어민 검토 요청 |
| Deutsch | [Deutsche Dokumentation](../de/README.md) | 31/31 | 원어민 검토 요청 |
| Türkçe | [Türkçe belgeler](../tr/README.md) | 31/31 | 원어민 검토 요청 |

<a id="what-is-translated"></a>
## 번역 대상

번역 대상에는 기본 README, `docs/` 바로 아래의 모든 페이지, `CONTRIBUTING.md`, `CONTEXT.md`, `SECURITY.md`, 기여자 대상 하위 시스템 README 여섯 개가 포함됩니다.

법률 문서, 서드 파티 고지, 이슈 템플릿, 내부 에이전트 지침, 생성되는 프로젝트 지침, 테스트 픽스처, 벤더 문서는 권위 있는 원본 상태로 유지됩니다. 생성 파일 또는 동작을 포함한 파일은 독립적인 번역 원본이 아닙니다.

<a id="freshness-and-validation"></a>
## 최신 상태 및 검증

각 번역 페이지는 정본 소스 경로와 소스 해시를 기록합니다. 탐색은 하나의 로케일 매니페스트에서 생성되며, 안정적인 영어 앵커 별칭은 제목이 번역되어도 깊은 링크가 계속 작동하게 합니다.

다음을 실행하세요.

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

이 도구는 산문을 번역하지 않습니다. 탐색 메타데이터를 관리하고 범위, 최신성, Markdown 구조, 명령, 링크, 앵커, 코드 블록, 표, URL만 검사합니다. 원어민의 수정은 일반 풀 리퀘스트로 환영합니다.

일반 동기화는 기존 소스 해시를 보존하므로 영문 산문이 바뀌면 번역은 직접 업데이트할 때까지 오래된 상태로 남습니다. 변경된 영문 페이지 하나의 번역 아홉 개를 모두 검토한 뒤 해당 소스만 승인하세요.

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI는 구조 검증 전에 검사 모드로 탐색 동기화를 실행합니다. 이 과정에서는 안정적인 각 영어 앵커가 대응하는 번역 제목에 계속 연결되어 있는지도 검증합니다.
