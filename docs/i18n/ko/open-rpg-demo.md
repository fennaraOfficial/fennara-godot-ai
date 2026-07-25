<!-- fennara-i18n: locale=ko source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Open RPG 데모 분석

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · [Español](../es/open-rpg-demo.md) · [Português do Brasil](../pt-BR/open-rpg-demo.md) · [日本語](../ja/open-rpg-demo.md) · **한국어** · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · [Deutsch](../de/open-rpg-demo.md) · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

동영상:

https://www.youtube.com/watch?v=0Egu3S-9MM0

이 데모는 GDQuest의 오픈 소스 Godot 4 Open RPG 프로젝트에서 Fennara MCP를 테스트합니다.

데모의 핵심은 AI가 빈 프로젝트를 처음부터 만들었다는 것이 아닙니다. AI 에이전트가 기존 Godot RPG 코드베이스 안에서 작업하고, 실수하고, Godot의 피드백을 받고, 구현을 수정한 뒤 계속 진행했다는 점입니다.

<a id="project"></a>
## 프로젝트

GDQuest Godot 4 Open RPG:

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## 작업

곰 플레이어 전투원 Baloo가 기존 전투에서 승리한 뒤 Tactical Guard라는 새 전투 능력을 잠금 해제하는 진행 기능을 추가합니다.

능력 요구 사항:

- 적 하나를 대상으로 함
- 적당한 피해를 줌
- Baloo의 Defense를 높임
- 잠금 해제 뒤 Baloo의 전투 행동 메뉴에 표시됨
- 잠금 해제 뒤 `Baloo learned Tactical Guard!` 같은 메시지를 표시함

<a id="what-happened"></a>
## 진행 과정

AI 코딩 에이전트가 Fennara MCP를 통해 실행 중인 Godot 프로젝트에 연결하고 프로젝트 아키텍처를 살펴봤습니다.

다음 Fennara 도구를 사용했습니다.

- 씬 트리 검사
- 노드 속성 검사
- GDScript 진단
- 씬 검증
- 런타임 오류 피드백
- 프로젝트 및 씬 검사

첫 구현은 완벽하게 작동하지 않았습니다. 바로 그 점이 유용했습니다.

Fennara가 Godot의 피드백을 반환했고, 에이전트는 망가진 스크립트를 수정하고 구현을 조정한 뒤 기능이 게임 안에서 작동할 때까지 계속 진행했습니다.

<a id="why-this-matters"></a>
## 중요한 이유

빈 데모는 쉽습니다. AI 에이전트는 대개 기존 프로젝트에서 실패합니다.

Fennara의 주장은 Godot AI 에이전트에 엔진 피드백이 필요하다는 것입니다.

- 스크립트가 파싱되었는가?
- 씬이 검증되었는가?
- 런타임이 오류를 출력했는가?
- 에이전트가 실제 프로젝트 구조를 살펴봤는가?
- 에이전트가 작업을 끝냈다고 가장하는 대신 실수를 수정할 수 있는가?

기존 MCP는 AI에 명령을 제공합니다.

Fennara는 AI에 Godot의 피드백을 제공합니다.
