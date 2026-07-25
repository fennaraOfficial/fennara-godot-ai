<!-- fennara-i18n: locale=ko source=CONTRIBUTING.md sha256=392729b4a281a8359dfe2f0790554a73c58dc998861e826067549ab62eb1761c -->
<a id="contributing"></a>
# 기여하기

<!-- fennara-doc-nav:start -->
[English](../../../CONTRIBUTING.md) · [简体中文](../zh-CN/CONTRIBUTING.md) · [Español](../es/CONTRIBUTING.md) · [Português do Brasil](../pt-BR/CONTRIBUTING.md) · [日本語](../ja/CONTRIBUTING.md) · **한국어** · [Русский](../ru/CONTRIBUTING.md) · [Français](../fr/CONTRIBUTING.md) · [Deutsch](../de/CONTRIBUTING.md) · [Türkçe](../tr/CONTRIBUTING.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../CONTRIBUTING.md)
<!-- fennara-doc-nav:end -->

Fennara Godot AI 개선에 도움을 주셔서 감사합니다.

<a id="good-contributions"></a>
## 좋은 기여

- 문서 수정
- 재현 가능한 버그 수정
- 플랫폼 호환성 수정
- 빌드 및 패키징 개선
- 설정 안내를 더 명확하게 만드는 작은 개선

<a id="design-discussion-required"></a>
## 설계 논의가 필요한 변경

다음 작업을 시작하기 전에는 이슈나 토론을 열어 주세요.

- 새로운 MCP 도구
- 도구 스키마 변경
- 릴리스 워크플로 변경
- 대규모 아키텍처 변경
- 생성되는 프로젝트 지침에 영향을 주는 변경

<a id="pull-requests"></a>
## 풀 리퀘스트

- 풀 리퀘스트를 작고 집중된 범위로 유지하세요.
- 무엇을 왜 변경했는지 설명하세요.
- 변경 사항을 어떻게 검증했는지 설명하세요.
- 화면에 보이는 UI 또는 문서 렌더링 변경에는 스크린샷이나 녹화를 포함하세요.
- 관련 없는 서식 변경이나 정리를 포함하지 마세요.
- 이슈나 풀 리퀘스트에 대량으로 생성된 설명을 붙여 넣지 마세요.

<a id="commit-and-pr-titles"></a>
## 커밋 및 PR 제목

Conventional Commit 형식을 사용하세요.

```text
fix(daemon): handle missing daemon status
docs(setup): clarify setup steps
ci(actions): add public pull request checks
```

일반적인 유형:

- `feat`: 사용자 대상 기능
- `fix`: 버그 수정
- `docs`: 문서
- `ci`: GitHub Actions 및 자동화
- `build`: 빌드 또는 패키징
- `refactor`: 동작을 보존하는 코드 구조 변경
- `test`: 테스트
- `chore`: 유지보수

<a id="project-boundaries"></a>
## 프로젝트 경계

Fennara는 게임에 종속되지 않아야 합니다. 특정 게임의 조작, 목표, 경제, 인벤토리, 전투, 경로 탐색, 퀘스트 또는 UI 흐름을 가정하는 API나 지침을 피하세요.

에이전트는 Godot 프로젝트의 실제 씬, 스크립트, 리소스, 설정, 런타임 상태, 진단 및 스크린샷을 살펴본 다음 해당 프로젝트에 맞게 범용 Fennara 도구를 조합해야 합니다.

<a id="documentation-translations"></a>
## 문서 번역

영어가 정본입니다. 먼저 영어를 수정한 다음 영향을 받는 모든 로케일을 업데이트하세요. 번역 대상과 로케일 메타데이터는 `docs/i18n/languages.json`에 있습니다.

- 영어 페이지 전체를 읽고 번역을 직접 작성하세요. 일괄 기계 번역 서비스나 산문 생성 스크립트를 사용하지 마세요.
- 코드 블록, 인라인 코드, 명령, 경로, 구성 키, URL 및 제품 이름을 정확히 유지하세요.
- 문서 스크립트가 관리하는 소스 마커와 명시적인 영어 앵커 별칭을 보존하세요.
- 유창한 검토자가 확인하지 않은 번역을 원어민 검토 완료로 표시하지 마세요.
- 법률 문서, 내부 에이전트 프롬프트, 생성되는 프로젝트 지침, 벤더 파일 또는 테스트 픽스처를 독립적인 번역 원본으로 번역하지 마세요.

정본 또는 번역 문서를 변경한 뒤 다음을 실행하세요.

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

이 명령은 탐색 메타데이터를 관리하고 구조를 검증합니다. 번역문을 작성하지는 않습니다.

일반 탐색 동기화는 기존의 모든 소스 해시를 보존합니다. 영문 원본을 변경했다면 해당 페이지를 번역된 로케일 아홉 개 모두에서 직접 업데이트한 다음, 그 정본 소스만 의도적으로 승인하세요.

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
node scripts/check-doc-i18n.mjs
```

번역을 검토하고 업데이트한 각 영문 페이지에 대해 `--accept-source <path>`를 반복하세요. 아홉 개 번역 모두에 새로운 의미가 반영되기 전에는 소스 해시를 승인하지 마세요.
