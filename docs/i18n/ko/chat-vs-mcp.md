<!-- fennara-i18n: locale=ko source=docs/chat-vs-mcp.md sha256=b6f27b2c7e905515aba56b75bf6736644a9c36c885f4cab61555c82cd6c47fda -->
<a id="mcp-apps-or-built-in-chat"></a>
# MCP 앱과 내장 채팅 중 무엇을 사용할까요?

<!-- fennara-doc-nav:start -->
[English](../../chat-vs-mcp.md) · [简体中文](../zh-CN/chat-vs-mcp.md) · [Español](../es/chat-vs-mcp.md) · [Português do Brasil](../pt-BR/chat-vs-mcp.md) · [日本語](../ja/chat-vs-mcp.md) · **한국어** · [Русский](../ru/chat-vs-mcp.md) · [Français](../fr/chat-vs-mcp.md) · [Deutsch](../de/chat-vs-mcp.md) · [Türkçe](../tr/chat-vs-mcp.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../chat-vs-mcp.md)
<!-- fennara-doc-nav:end -->

Fennara는 두 가지를 모두 지원합니다. 대화할 위치를 선택하세요.

| | 외부 MCP 앱 | Fennara 내장 채팅 |
| --- | --- | --- |
| 채팅 위치 | Codex, Claude, Cursor, Gemini 또는 다른 MCP 앱 | Fennara 독 또는 시스템 브라우저 |
| 모델 계정 | 외부 앱의 계정 또는 구독 | Fennara Chat Settings에 연결된 제공업체 |
| Fennara가 추가하는 기능 | Godot 인식 MCP 도구 | 채팅 UI, 같은 핵심 Godot 도구, 채팅 전용 파일 및 셸 도구 |
| 설정 | **Chat Settings > MCP Apps** | **Chat Settings > Chat > Open providers** |

> [!TIP]
> 두 경로를 모두 사용할 수 있습니다. 모델 설정은 서로 분리되어 있습니다.

<a id="external-mcp-apps"></a>
## 외부 MCP 앱

MCP 앱을 연결하면 해당 앱이 로컬 Fennara MCP 서버를 시작하고 Godot 도구를 호출할 수 있습니다. 앱의 구독이나 로그인을 내장 채팅과 공유하지는 않습니다.

**Chat Settings > MCP Apps**에서 앱을 설정하거나 CLI를 사용하세요.

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Fennara 채팅 제공업체 키는 필요하지 않습니다. 설정 후 외부 앱을 다시 시작하세요. 모든 대상과 수동 구성은 [MCP 설정](mcp-setup.md)을 참고하세요.

<a id="built-in-chat"></a>
## 내장 채팅

내장 채팅에는 Fennara Chat Settings에 연결된 제공업체가 필요합니다. 클라우드 제공업체에는 사용자의 키를 사용하고, 로컬 Ollama 또는 LM Studio 서버를 연결할 수도 있습니다.

같은 채팅이 Godot 독 또는 시스템 브라우저에 표시될 수 있습니다. 이 표시 선택은 제공업체, 모델, 기록 또는 프로젝트를 바꾸지 않습니다.

코드를 첨부하려면 Godot 스크립트 에디터에서 코드를 선택하고 컨텍스트 메뉴를 연 다음 **Add to Chat**을 선택하세요. 제공업체 및 모델 설정은 [내장 채팅 제공업체](providers.md)를 참고하세요.

<a id="project-routing"></a>
## 프로젝트 라우팅

두 경로 모두 Godot 피드백을 위해 로컬 Fennara 데몬을 사용합니다.

- 외부 MCP 프로세스는 시작할 때 정규 Godot Project Root에 한 번 바인딩할 수
  있습니다. 호출은 독의 **MCP target**을 읽거나 변경하지 않고 일치하는
  에디터로 라우팅됩니다.
- 바인딩되지 않은 외부 MCP 프로세스는 호환 동작을 유지합니다. 먼저 독에서
  선택한 유효한 MCP Target을 사용하고, 유효한 독 대상이 없을 때만 연결된
  유일한 에디터로 대체합니다. 대상 없이 여러 에디터가 연결되어 있으면
  모호성 오류가 발생합니다.
- 내장 채팅은 해당 채팅을 연 Godot 에디터에 계속 연결됩니다.

다른 저장소나 워크트리에서 독립적으로 작업하는 에이전트를 위해 프로젝트마다
하나의 MCP 프로세스와 연결을 사용하세요. 설정과 Runtime Slot 동작은
[여러 에이전트와 워크트리](multi-agent-worktrees.md)를 참고하세요.

외부 MCP 연결을 확인하려면 다음과 같이 요청하세요.

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

동시 작업을 시작하기 전에 상태가 라우팅 모드 `bound`와 예상한 정규 Project Root를
보고하는지 확인하세요. 레거시 미바인딩 모드에는 동시 작업 경고가 포함됩니다.
