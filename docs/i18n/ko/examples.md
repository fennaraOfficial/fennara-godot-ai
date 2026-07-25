<!-- fennara-i18n: locale=ko source=docs/examples.md sha256=86616717ed75b07c196cfe98fbab584e1ae25cb0967c03e8f514e4b1ab1f3140 -->
<a id="examples"></a>
# 예제

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../examples.md) · [简体中文](../zh-CN/examples.md) · [Español](../es/examples.md) · [Português do Brasil](../pt-BR/examples.md) · [日本語](../ja/examples.md) · **한국어** · [Русский](../ru/examples.md) · [Français](../fr/examples.md) · [Deutsch](../de/examples.md) · [Türkçe](../tr/examples.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../examples.md)
<!-- fennara-doc-nav:end -->

프롬프트를 복사하고 프로젝트 세부 정보를 바꾼 다음 MCP 앱 또는 Fennara 내장 채팅에서 보내세요.

| 목표 | 예제 |
| --- | --- |
| 연결된 에디터 확인 | [연결 확인](#연결-확인) |
| 기존 프로젝트 이해 | [편집 전 검사](#편집하기-전에-프로젝트-검사) |
| 집중된 변경 수행 | [아키텍처를 고려한 변경](#작고-아키텍처를-고려한-변경) |
| 실행 중인 프로젝트 진단 | [런타임 오류](#런타임-오류-디버그) |
| 렌더링 결과 검사 | [시각적 피드백](#시각적-피드백) |

<a id="check-connection"></a>
## 연결 확인

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

<a id="inspect-a-project-before-editing"></a>
## 편집하기 전에 프로젝트 검사

```text
Use Fennara MCP to inspect this Godot project. Look at the scene tree, relevant files, diagnostics, and project structure before suggesting changes.
```

<a id="make-a-small-architecture-aware-change"></a>
## 작고 아키텍처를 고려한 변경

```text
Work inside this existing Godot project like a careful contributor. Inspect how the relevant system is organized, make the smallest useful change, and explain what files/resources changed and how I can test it.
```

<a id="debug-a-runtime-error"></a>
## 런타임 오류 디버그

```text
Use Fennara MCP to inspect the latest Godot runtime errors, find the likely source, patch the issue, and explain the fix.
```

<a id="visual-feedback"></a>
## 시각적 피드백

```text
Use Fennara MCP to capture a screenshot of the current scene, inspect the UI layout, and suggest or make a small fix if something is visibly wrong.
```

<a id="built-in-chat-provider-setup"></a>
## 내장 채팅 제공업체 설정

Godot 안의 Fennara 독에서 다음을 실행합니다.

```text
/provider
```

클라우드 또는 로컬 제공업체를 연결합니다.

그다음:

```text
/model
```

독에서 사용할 모델을 선택합니다.

<a id="existing-project-demo-prompt"></a>
## 기존 프로젝트 데모 프롬프트

Open RPG 데모에서 사용한 프롬프트의 예입니다.

```text
I want you to work inside this existing Godot RPG project like a careful project contributor. Before making changes, understand how the relevant systems are organized. Reuse the existing architecture and naming style wherever possible. Add the requested feature in the smallest clean way, then tell me what changed and how to try it in-game.
```
