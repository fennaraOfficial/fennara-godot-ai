<!-- fennara-i18n: locale=ko source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# MCP 설정

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · **한국어** · [Русский](../ru/mcp-setup.md) · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

외부 AI 앱을 Fennara의 Godot 도구에 연결합니다. 앱은 자체 모델 계정, 구독 또는 API 설정을 계속 사용합니다.

> [!NOTE]
> 이 과정은 Fennara 내장 채팅을 구성하지 않습니다. 어떤 경로가 필요한지 확실하지 않다면 [MCP 앱과 내장 채팅](chat-vs-mcp.md)을 참고하세요.

<a id="quick-setup"></a>
## 빠른 설정

1. Godot 독에서 **Set Up Fennara**를 완료합니다.
2. **Chat Settings > MCP Apps**를 엽니다.
3. 앱을 찾아 **Set Up**을 누릅니다.
4. 앱을 다시 시작합니다.

Fennara는 앱의 MCP 구성을 변경하기 전에 백업을 만듭니다. 결합된 **Claude** 옵션은 Claude Code와 Claude Desktop을 구성합니다. **Gemini & Antigravity**는 공유되는 두 대상을 모두 구성합니다.

<a id="terminal-alternative"></a>
### 터미널 대안

먼저 Godot 프로젝트 안에서 `fennara install`을 실행한 다음 대상을 선택하세요.

| 앱 | 명령 |
| --- | --- |
| Claude Code 및 Claude Desktop | `fennara mcp-setup --claude` |
| Claude Code만 | `fennara mcp-setup --claude-code` |
| Claude Desktop만 | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini 및 Antigravity | `fennara mcp-setup --gemini` 또는 `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

설치된 CLI가 지원하는 대상 목록은 `fennara mcp-setup --help`를 실행해 확인하세요.

<a id="manual-setup"></a>
## 수동 설정

앱이 목록에 없거나, 설정 명령이 앱의 구성 파일을 찾지 못하거나, 의도적으로 MCP 구성을 직접 편집하려는 경우에만 수동 설정을 사용하세요.

편집 전에 구성 파일을 백업하세요. 그런 다음 안정적인 Fennara MCP 런처를 가리키는 `fennara`라는 로컬 stdio MCP 서버를 추가합니다.

기본 런처 경로:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

사용자 컴퓨터의 실제 절대 경로를 사용하세요. MCP 앱이 `versions/<version>/fennara-mcp-runtime`을 가리키게 하지 마세요. `bin/`의 안정적인 런처가 Fennara 업데이트 뒤에도 앱 구성을 계속 작동하게 합니다.

<a id="json-mcpservers"></a>
### JSON `mcpServers`

많은 MCP 앱은 최상위 `mcpServers` 객체를 사용합니다.

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

일부 앱은 같은 `mcpServers` 키를 사용하지만 `command`만 필요합니다. 기존 구성에 다른 서버가 있다면 해당 항목을 보존하고 `fennara` 서버만 추가하세요.

Cline 형식의 구성에는 초 단위의 더 긴 도구 제한 시간을 포함할 수도 있습니다.

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### VS Code 형식 JSON `servers`

VS Code 사용자 또는 프로젝트 MCP 구성 등 일부 클라이언트는 최상위 `servers` 객체를 사용하고 `type: "stdio"`를 요구합니다.

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### OpenCode 형식 JSON `mcp`

OpenCode 형식 JSON 구성은 최상위 `mcp` 객체를 사용합니다. 제한 시간은 밀리초 단위입니다.

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### Codex 형식 TOML

Codex는 TOML을 사용합니다.

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

JSON 파일에 TOML을 붙여 넣거나 TOML 파일에 JSON을 붙여 넣지 마세요. 앱에서 이미 사용하는 형식을 따르세요.

<a id="common-config-locations"></a>
## 일반적인 구성 위치

다음은 Fennara 설정 헬퍼와 현재 MCP 클라이언트가 사용하는 일반적인 위치입니다. 앱은 구성 경로를 바꿀 수 있으며 일부는 전역 구성과 프로젝트 로컬 구성을 모두 지원합니다. 앱에 **Open MCP Config** 같은 명령이 있으면 추측하지 말고 해당 명령을 사용하세요.

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

<a id="timeout-guidance"></a>
## 제한 시간 안내

일부 Fennara 도구는 Godot에 씬 검증, 런타임 상태 검사, 스크린샷 캡처 또는 진단 실행을 요청하므로 짧은 기본 MCP 제한 시간보다 오래 걸릴 수 있습니다.

클라이언트가 지원한다면 도구별 제한 시간을 더 길게 설정하세요.

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

클라이언트가 서버별 제한 시간을 지원하지 않으면 해당 클라이언트 문서의 전역 MCP 제한 시간 설정을 사용하세요.

<a id="verify-the-connection"></a>
## 연결 확인

Godot 프로젝트를 연 다음 MCP 앱에 다음과 같이 요청하세요.

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Godot 프로젝트가 여러 개 열려 있다면 Fennara 독의 **MCP target** 컨트롤에서 외부 MCP 도구 호출을 받을 프로젝트를 선택하세요.

<a id="troubleshooting"></a>
## 문제 해결

MCP 앱에 Fennara가 표시되지 않으면 다음을 확인하세요.

- 런처 경로가 절대 경로이며 실제로 존재함
- 앱이 요구하는 JSON, JSON5 또는 TOML 문법이 올바름
- 서버 이름이 `fennara`임
- 앱이 편집한 구성 파일을 실제로 읽고 있음
- MCP 앱을 완전히 종료하고 다시 열었음
- Godot 프로젝트에 Fennara 애드온이 설치되어 있음
- 원하는 Godot 프로젝트가 MCP 대상으로 선택되어 있음

<a id="unsupported-mcp-apps"></a>
## 지원되지 않는 MCP 앱

MCP 앱이 목록에 없다면 먼저 해당 앱의 공식 MCP 구성 위치와 형식을 찾으세요. 그런 다음 LLM에 가장 작은 안전한 변경을 요청하세요.

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

저장하기 전에 결과를 검토한 뒤 MCP 앱을 다시 시작하세요.
