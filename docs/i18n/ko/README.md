<!-- fennara-i18n: locale=ko source=docs/README.md sha256=ab01a3fd8024bccc8ad878eb0ec4cb15defa770ed3feccd0eef4c56270c7e763 -->
<a id="fennara-documentation"></a>
# Fennara 문서

<!-- fennara-doc-nav:start -->
[English](../../README.md) · [简体中文](../zh-CN/README.md) · [Español](../es/README.md) · [Português do Brasil](../pt-BR/README.md) · [日本語](../ja/README.md) · **한국어** · [Русский](../ru/README.md) · [Français](../fr/README.md) · [Deutsch](../de/README.md) · [Türkçe](../tr/README.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../README.md)
<!-- fennara-doc-nav:end -->

완료하려는 작업부터 시작하세요. 각 페이지는 일반적인 경로를 먼저 설명하고 고급 세부 내용은 아래쪽에 배치합니다.

<a id="languages"></a>
## 언어

각 페이지의 언어 메뉴를 사용하면 같은 페이지를 다른 언어로 볼 수 있습니다. 지원 범위, 검토 상태, 정본 정책은 [언어 및 번역 상태](languages.md)를 참고하세요.

<a id="start-here"></a>
## 시작하기

| 원하는 작업 | 읽을 문서 |
| --- | --- |
| Fennara 설치 | [설정](setup.md) |
| 내장 채팅 연결 | [채팅 제공업체](providers.md) |
| Codex, Claude, Cursor 또는 다른 MCP 앱 연결 | [MCP 설정](mcp-setup.md) |
| Fennara 업데이트 또는 복구 | [Fennara 업데이트](setup.md#fennara-업데이트) |
| 설정 문제 해결 | [문제 해결](setup.md#문제-해결) |

<a id="use-fennara"></a>
## Fennara 사용

| 가이드 | 다루는 내용 |
| --- | --- |
| [MCP 앱과 내장 채팅](chat-vs-mcp.md) | 각 경로에서 사용하는 모델 계정 |
| [여러 에이전트와 워크트리](multi-agent-worktrees.md) | 하나의 데몬을 공유하면서 각 MCP 연결을 자신의 Godot 프로젝트에 바인딩 |
| [도구](tools.md) | Godot 인식 도구와 사용 시점 |
| [예제](examples.md) | 일반적인 Godot 워크플로용 프롬프트 |
| [슬래시 명령](slash-commands.md) | 채팅 독의 `/provider`와 `/model` |
| [FAQ](faq.md) | 자주 묻는 질문에 대한 짧은 답변 |
| [데모](demos.md) | 동영상과 프로젝트 둘러보기 |
| [익명 텔레메트리](telemetry.md) | 수집 데이터, 전송 동작 및 비활성화 제어 |

<a id="reference-and-recovery"></a>
## 참고 및 복구

| 참고 문서 | 필요한 경우 |
| --- | --- |
| [Fennara CLI](cli.md) | 터미널 명령, 진단 또는 자동화가 필요할 때 |
| [수동 설치](manual-install.md) | 일반 설치 프로그램을 사용할 수 없을 때 |
| [MCP 설정 참고](mcp-setup.md) | 앱별 설정 또는 수동 구성이 필요할 때 |
| [제공업체 참고](providers.md) | 키, 모델 ID 또는 로컬 서버 세부 정보가 필요할 때 |

<a id="for-contributors"></a>
## 기여자용

| 문서 | 목적 |
| --- | --- |
| [기여하기](CONTRIBUTING.md) | 기여 및 풀 리퀘스트 기대 사항 |
| [아키텍처](architecture.md) | 시스템 경계 및 런타임 흐름 |
| [저장소 지도](repo-map.md) | 코드와 생성 파일의 위치 |
| [릴리스 절차](release.md) | 패키징, 매니페스트, 검증 및 게시 |
| [프로젝트 용어](CONTEXT.md) | 코드와 문서에서 사용하는 공통 명칭 |
| [보안](SECURITY.md) | 취약점 신고 |
| [GitHub 메타데이터](github-metadata.md) | 저장소 설명 및 주제 |
| [Godot 페이로드](contributors/godot-payload.md) | 패키징된 애드온 소스 경계 |
| [Godot 애드온](contributors/godot-addons.md) | 애드온 디렉터리 구조 및 규칙 |
| [로컬 도구](contributors/local-tools.md) | CLI, 데몬, MCP 서버 및 로컬 런타임 |
| [런타임 헬퍼](contributors/runtime-helpers.md) | Godot 측 런타임 헬퍼 소스 |
| [저장소 스크립트](contributors/scripts.md) | 빌드, 동기화, 검증 및 패키징 자동화 |
| [채팅 UI](contributors/chat-ui.md) | 선택 사항인 에디터 내 채팅 소스 및 설계 규칙 |

<a id="learn-from-examples"></a>
## 예제로 배우기

- [Fennara와 기존 Godot MCP 비교](fennara-vs-traditional-godot-mcp.md)
- [Open RPG 데모 분석](open-rpg-demo.md)
- [프롬프트 예제](examples.md)
