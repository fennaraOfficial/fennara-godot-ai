<!-- fennara-i18n: locale=ko source=docs/fennara-vs-traditional-godot-mcp.md sha256=e45a741b1db7c20e40b4a311c198af216172dfa024ca9c123db4f9336c9a6e7f -->
<a id="fennara-vs-traditional-godot-mcp"></a>
# Fennara와 기존 Godot MCP 비교

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../fennara-vs-traditional-godot-mcp.md) · [简体中文](../zh-CN/fennara-vs-traditional-godot-mcp.md) · [Español](../es/fennara-vs-traditional-godot-mcp.md) · [Português do Brasil](../pt-BR/fennara-vs-traditional-godot-mcp.md) · [日本語](../ja/fennara-vs-traditional-godot-mcp.md) · **한국어** · [Русский](../ru/fennara-vs-traditional-godot-mcp.md) · [Français](../fr/fennara-vs-traditional-godot-mcp.md) · [Deutsch](../de/fennara-vs-traditional-godot-mcp.md) · [Türkçe](../tr/fennara-vs-traditional-godot-mcp.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../fennara-vs-traditional-godot-mcp.md)
<!-- fennara-doc-nav:end -->

| 기존 명령 브리지 | Fennara 피드백 루프 |
| --- | --- |
| 에디터 작업 노출 | Godot 인식 검사, 작업 및 검사 노출 |
| 명령 성공이 흐름의 끝이 될 수 있음 | 진단, 검증, 런타임 로그, 스크린샷이 다음 단계를 결정 |
| 직접적이고 이미 알려진 편집에 적합 | 에이전트가 검사, 변경, 검증, 복구해야 할 때 적합 |

대부분의 Godot MCP 서버는 AI 클라이언트에 에디터 명령을 제공합니다.

예:

- 노드 생성
- 속성 설정
- 씬 열기
- 씬 저장
- 로그 읽기
- 스크린샷 촬영
- 프로젝트 실행
- 시그널 연결
- 입력 맵 편집
- 머티리얼 관리
- 테스트 실행

이는 유용합니다. Godot을 API 표면으로 바꿉니다.

하지만 실제 AI 게임 개발에서 어려운 점은 AI가 `set_property`를 호출할 수 있는지가 아닙니다.

어려운 점은 AI가 프로젝트가 망가졌는지 알아낼 수 있는가입니다.

<a id="traditional-mcp-pattern"></a>
## 기존 MCP 패턴

```text
AI calls editor command.
Editor returns result.
AI guesses next step.
```

이 방식은 작고 직접적인 편집에 잘 맞습니다.

예:

```text
Rename Camera3D to MainCamera.
```

하지만 에이전트가 아키텍처를 살펴보고, 스크립트, 리소스, 씬을 편집하고, 실패를 확인하고, 복구해야 하는 더 큰 프로젝트 작업에는 약합니다.

<a id="fennara-pattern"></a>
## Fennara 패턴

```text
AI changes project.
Godot feedback comes back.
AI patches and reruns until it works.
```

Fennara는 피드백에 집중합니다.

- GDScript 진단
- 씬 검증
- 런타임 오류
- 씬 트리 검사
- 노드 속성
- 클래스 및 API 검사
- 스크린샷
- 생성되는 프로젝트 지침
- 수정 후 재실행 워크플로

<a id="the-difference"></a>
## 차이점

기존 Godot MCP가 묻는 질문:

```text
What editor commands should we expose?
```

Fennara가 묻는 질문:

```text
What feedback does the model need to successfully build inside Godot?
```

명령은 기본 조건입니다.

피드백이 경쟁력입니다.
