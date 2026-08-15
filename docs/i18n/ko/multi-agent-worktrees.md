<!-- fennara-i18n: locale=ko source=docs/multi-agent-worktrees.md sha256=037571da5f2479b968f6b74e8994d0272042d4a26616c50af5f62f3d2a9bbcf9 -->
<a id="multiple-agents-and-godot-worktrees"></a>
# 여러 에이전트와 Godot 워크트리

<!-- fennara-doc-nav:start -->
[English](../../multi-agent-worktrees.md) · [简体中文](../zh-CN/multi-agent-worktrees.md) · [Español](../es/multi-agent-worktrees.md) · [Português do Brasil](../pt-BR/multi-agent-worktrees.md) · [日本語](../ja/multi-agent-worktrees.md) · **한국어** · [Русский](../ru/multi-agent-worktrees.md) · [Français](../fr/multi-agent-worktrees.md) · [Deutsch](../de/multi-agent-worktrees.md) · [Türkçe](../tr/multi-agent-worktrees.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../multi-agent-worktrees.md)
<!-- fennara-doc-nav:end -->

한 컴퓨터에서 여러 코딩 에이전트가 서로 다른 저장소나 워크트리에서 작업하면서, 한 에이전트의 대상 선택이 다른 에이전트의 라우팅을 바꾸지 않게 합니다. 각 프로젝트는 전용 Fennara MCP 프로세스와 연결을 갖고, 모든 프로젝트는 동일한 사용자별 데몬을 공유합니다.

```text
agent A -> MCP process A -- Project Binding A --\
                                                  shared daemon -> Godot editor A
agent B -> MCP process B -- Project Binding B --/              -> Godot editor B
```

편집, 검사, 범위가 제한된 씬 검증, 독립형 스크린샷 호출은 동시에 실행할 수 있습니다. 데몬이 관리하는 대화형 게임 실행은 시스템 전역 Runtime Slot 하나를 통해 직렬화됩니다.

<a id="one-mcp-connection-per-project"></a>
## 프로젝트마다 MCP 연결 하나

MCP 프로세스는 시작할 때 안정적인 Project Root 하나를 선택합니다. 이 MCP Project Binding은 `project.godot`이 있는 디렉터리의 정규 파일 시스템 식별성이며, 프로젝트 이름이나 Godot 프로세스 ID가 아닙니다.

각 저장소나 워크트리마다 별도의 MCP 프로세스와 연결을 사용하세요. 모든 에이전트가 의도적으로 동일한 프로젝트에서 작업할 때만 연결 하나를 여러 에이전트가 공유해도 됩니다. Fennara 도구는 호출별 프로젝트 선택기를 노출하지 않으므로, 모델이 실수로 프로세스를 다른 프로젝트로 바꾸지 못합니다.

각 프로젝트에는 Fennara가 활성화된 연결 상태의 Godot 에디터도 필요합니다. 에디터가 닫혔다가 새 프로세스 ID로 재연결되면, 동일한 Project Root가 재연결될 때 기존 MCP 프로세스가 라우팅을 재개합니다.

<a id="how-a-process-chooses-its-project"></a>
## 프로세스가 프로젝트를 선택하는 방법

MCP 런타임은 시작 작업 디렉터리를 캡처하고 다음 순서로 바인딩을 한 번만 선택합니다.

1. `--project-path <path>` 또는 `--project-path=<path>`.
2. `FENNARA_PROJECT_PATH`.
3. `project.godot`이 있는 가장 가까운 시작 디렉터리 조상.
4. 자동 검색에서 Godot 프로젝트를 찾지 못했을 때의 레거시 미바인딩 호환 모드.

명령줄과 환경 경로는 명시적 선언입니다. 비어 있거나, 접근할 수 없거나, 없거나, 디렉터리가 아니거나, Godot 프로젝트가 아니거나, 지원되지 않는 경로는 MCP 서버가 시작되지 못하게 합니다. 다른 프로젝트로 절대 대체하지 않습니다. 상대 경로는 캡처된 시작 디렉터리에서 해석됩니다. MCP 호스트의 시작 디렉터리가 분명하지 않으면 절대 경로를 사용하세요.

Fennara는 호스트 전용 워크스페이스 변수를 암묵적으로 사용하지 않습니다. MCP 호스트는 자신의 워크스페이스 값을 `--project-path` 또는 `FENNARA_PROJECT_PATH`에 매핑할 수 있습니다.

<a id="configure-a-project-bound-connection"></a>
## 프로젝트에 바인딩된 연결 구성

`fennara mcp-setup`은 전역적이고 프로젝트와 무관한 상태를 유지합니다. 프로젝트 안에서 실행해도 앞으로 시작할 모든 MCP 프로세스가 해당 프로젝트에 바인딩되지는 않습니다. 안정적인 런처 경로를 유지한 뒤 MCP 호스트의 프로젝트 또는 워크스페이스 구성을 사용해 바인딩을 추가하세요.

JSON 형식 구성:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": ["--project-path", "/absolute/path/to/worktree-a"],
      "env": {}
    }
  }
}
```

환경 변수 사용:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "/absolute/path/to/fennara-mcp",
      "args": [],
      "env": {
        "FENNARA_PROJECT_PATH": "/absolute/path/to/worktree-a"
      }
    }
  }
}
```

Codex 형식 TOML:

```toml
[mcp_servers.fennara]
command = "/absolute/path/to/fennara-mcp"
args = ["--project-path", "/absolute/path/to/worktree-a"]
startup_timeout_sec = 30
tool_timeout_sec = 300
```

다음 에이전트는 자신의 프로젝트/워크스페이스 구성에서 `/absolute/path/to/worktree-b`를 사용하도록 구성하세요. 호스트가 각 프로젝트 디렉터리에서 별도의 MCP 프로세스를 안정적으로 시작한다면, 조상 검색으로 명시적 경로 없이도 동일한 바인딩을 얻을 수 있습니다.

<a id="mcp-host-boundaries"></a>
## MCP 호스트 경계

프로젝트 로컬 구성과 시작 디렉터리 동작은 호스트마다 다릅니다.

- VS Code 단일 폴더 워크스페이스는 호스트에 문서화된 자식 작업 디렉터리에 의존할 수 있지만, 명시적 프로젝트 바인딩이 가장 분명한 구성입니다.
- Claude Code, Gemini CLI, Antigravity, Cline, Cursor, OpenCode, Kiro, Codex는 프로젝트/워크스페이스 구성을 사용할 수 있습니다. 격리를 보장해야 한다면 명시적 바인딩이나 문서화된 프로젝트 시작 디렉터리를 사용하세요.
- Claude Desktop과 레거시 Windsurf/Cascade 구성은 전역입니다. 기본 Fennara 항목은 레거시 미바인딩 상태로 남으며 자동 프로젝트 로컬 격리를 제공할 수 없습니다. 고급 사용자는 서로 다른 명시적 경로를 가진 별도의 전역 항목을 이름을 달리해 만들 수 있지만 올바른 항목을 선택해야 합니다.

프로젝트 로컬 구성 자동 생성과 새 Windsurf/Devin Local 지원은 이 워크플로의 범위 밖입니다.

<a id="start-and-verify-the-editors"></a>
## 에디터 시작 및 확인

각 워크트리에는 Fennara가 활성화된 Godot 에디터가 필요합니다. 헤드리스 에디터는 Fennara 데몬을 공유하면서 서로 다른 Godot LSP 포트를 사용할 수 있습니다.

```bash
godot --editor --headless --path /absolute/path/to/worktree-a --lsp-port 6006
godot --editor --headless --path /absolute/path/to/worktree-b --lsp-port 6007
```

LSP 포트는 Godot에 속합니다. Fennara는 기존 루프백 주소에서 공유 데몬 하나를 계속 사용합니다.

동시 작업 전에 모든 에이전트에서 `fennara_status`를 실행하고 다음이 보고되는지 확인하세요.

- 라우팅 모드 `bound`
- 예상한 바인딩 소스와 정규 Project Root
- 바인딩된 에디터 상태 `connected`
- 해당 에디터의 파일 시스템 준비 상태

자동 검색에서 프로젝트를 찾지 못하면 상태는 `legacy_unbound`와 동시 작업 경고를 보고합니다. 이 호환 모드에서는 독에서 선택한 MCP Target을 먼저 사용하고, 그다음으로 연결된 유일한 에디터를 사용합니다. 격리된 동시 작업에 미바인딩 연결을 사용하지 마세요.

<a id="missing-and-duplicate-editors"></a>
## 누락된 에디터와 중복 에디터

유효한 Project Binding은 에디터가 없어도 유지됩니다. 도구 호출은 해당 Project Root가 재연결될 때까지 재시도 가능한 `bound_project_not_connected`를 반환하며, 독 대상으로 절대 대체하지 않습니다.

두 에디터가 동일한 Project Root로 해석되면 `ambiguous_project_binding`이 발생합니다. 중복 에디터를 닫거나 별도의 워크트리를 제공하세요. Fennara는 프로세스 ID, 연결 순서, 프로젝트 이름, 독 대상을 기준으로 선택하지 않습니다.

동일한 프로젝트를 가리키는 심볼릭 별칭은 동일한 실행 중인 파일 시스템 식별성으로 해석됩니다. MCP 시작 후 심볼릭의 대상을 바꿔도 바인딩은 변하지 않습니다. 다시 바인딩하려면 해당 MCP 프로세스를 다시 시작하세요.

<a id="serialized-runtime-sessions"></a>
## 직렬화된 런타임 세션

모든 프로젝트는 데몬이 관리하는 게임 실행을 위해 시스템 전역 Runtime Slot 하나를 공유합니다. 다른 프로젝트가 세션을 시작 중이거나 실행 중이면 `runtime_session.start`는 `availability: "busy"`, `slot_acquired: false`, 권장 `retry_after_ms`를 포함한 성공적인 `busy` 도메인 결과를 반환합니다. 소유자, 세션 ID, 프로세스 ID, 씬, 로그, 대기열 위치, 예상 시간은 노출하지 않습니다.

FIFO 대기열은 없습니다. 권장 재시도 지연에 가까운 주기로 지터를 적용해 폴링하고, 각 `runtime_session.start`를 마지막 원자적 확보로 취급하세요. 사전 검사 후 다른 에이전트가 경쟁에서 이길 수 있으므로 빈 상태는 참고용일 뿐입니다.

소유 Project Root만 해당 Runtime Session을 검사하고, 갱신하고, 스크립트를 실행하거나 중지할 수 있습니다. 소유자 상태는 120초의 비활성 마감을 갱신합니다. 제한된 소유자 런타임 작업은 활성 상태인 동안 비활성 만료를 일시 중지하고, 종료 상태의 스크립트 결과를 반환한 후에만 마감을 갱신합니다. 제한 시간 초과, 설정 오류 또는 취소는 갱신하지 않습니다. 실행이 진행되는 동안 에이전트는 약 30초마다 지터를 적용해 소유자 상태를 폴링해야 합니다.

기본 절대 Runtime Lease는 900초입니다. `max_run_seconds`는 최대 86,400초의 양의 정수를 받습니다. 예를 들어 예상 시간이 한 시간인 회귀 테스트는 안전 여유를 위해 4,500초를 요청할 수 있습니다. 절대 마감은 중지되지 않습니다. 자연 종료, 명시적 중지, 시작 실패, 비활성, 절대 만료 시 게임을 중지하거나 회수하고 Runtime Slot을 해제합니다.

<a id="safe-multi-agent-checklist"></a>
## 안전한 멀티 에이전트 체크리스트

1. 각 프로젝트마다 별도의 저장소나 워크트리를 만듭니다.
2. Fennara를 설치하고 Project Root마다 Godot 에디터를 하나씩 엽니다.
3. 프로젝트마다 프로젝트에 바인딩된 MCP 프로세스를 하나씩 구성합니다.
4. 모든 에이전트에서 `fennara_status`를 실행하고 정규 루트를 확인합니다.
5. 편집, 검사, 범위가 제한된 씬 검증, 독립형 스크린샷을 동시에 진행합니다.
6. 플레이테스트의 오류가 아닌 `busy` 결과를 폴링하고 재시도하며, 실행 중에는 소유자 상태로 성공한 세션을 유지합니다.
