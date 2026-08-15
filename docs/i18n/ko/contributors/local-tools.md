<!-- fennara-i18n: locale=ko source=local/README.md sha256=b63443a8ff7310d53044b20261e7f206a949bad07299656235ba39fe69f92d9d -->
<a id="fennara-local-tools"></a>
# Fennara 로컬 도구

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · **한국어** · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

이 폴더에는 로컬 네이티브 Fennara 구성 요소가 있습니다.

<a id="daemon"></a>
## 데몬

`crates/fennara-daemon`은 다음 주소에서 로컬 Fennara 데몬을 실행합니다.

```text
http://127.0.0.1:41287
```

엔드포인트:

- `GET /health`: 데몬 상태.
- `GET /status`: 데몬 상태 및 연결된 Godot 플러그인 메타데이터.
- 권한이 필요한 바인딩 상태: 하나의 MCP 프로세스의 정규 Project Root를 연결된 Godot 에디터 세션과 매칭.
- `POST /tools/call`: 연결된 Godot 플러그인으로 도구 호출을 전달하고 결과를 기다림.
- `WS /godot/ws`: 로컬 Godot 플러그인 브리지. 플러그인은 연결 뒤 `hello` 메시지를 보냄.

현재 사용자의 모든 Fennara 활성 에디터와 외부 MCP 프로세스가 데몬 하나를 공유합니다.
바인딩된 외부 요청은 정규 Project Root를 기준으로 라우팅됩니다. 내부 내장 채팅 요청은 해당
Godot Editor Session에 계속 바인딩되며, 레거시 미바인딩 MCP 요청은 독에서 선택한 호환
대상을 사용합니다.

데몬은 시스템 전역 Runtime Slot 하나도 소유합니다. 에디터가 재연결되어도 제어권이 옮겨가지
않도록 Runtime Session 소유권과 갱신 가능한 리스 상태는 Project Root와 연결됩니다.

개발 바이너리:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP 서버

`crates/fennara-mcp`는 로컬 MCP 서버입니다. MCP 클라이언트가 로컬 프로세스로 시작할 수 있도록 stdio에서 JSON-RPC를 사용합니다.

각 MCP 프로세스는 시작할 때 선택적 Project Binding 하나를 고정합니다. 선택 순서는
`--project-path`, `FENNARA_PROJECT_PATH`, 시작 디렉터리에서 가장 가까운 `project.godot` 조상
순입니다. 프로젝트를 찾지 못하면 자동으로 레거시 미바인딩 호환 모드로 진입하고, 잘못된
명시적 경로는 시작을 실패합니다. 프로젝트 간 격리를 위해 프로젝트마다 MCP 프로세스와 연결을
하나씩 사용하세요.

`crates/fennara-project-identity`는 MCP 런타임과 데몬이 공유합니다. Project Root 검색, 검증,
정규화, 손실 없는 프로토콜 변환, 실행 중인 파일 시스템 동일성 판단을 소유합니다.

`fennara-mcp`는 빌드 시 `local/schemas/tools/`에서 선택한 MCP 대상 스키마를 내장하고 해당 도구 호출을 로컬 데몬으로 전달합니다. 런타임에 외부 스키마 서비스가 필요하지 않습니다. 내장 채팅은 같은 스키마 디렉터리에서 관련되지만 서로 다른 도구 집합을 선택합니다.

`fennara install`은 `local/templates/`에서 생성한 프로젝트 지침도 Godot 프로젝트에 작성합니다.

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

빌드:

```powershell
cd local
cargo build
```

Windows에서 터미널의 Rust PATH가 아직 새로 고쳐지지 않았다면:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

개발 바이너리:

```text
local/target/debug/fennara-mcp.exe
```

현재 도구:

- `fennara_status`: MCP 서버가 설치되어 있고 접근 가능한지 확인한 뒤, 데몬이 실행 중이면 라우팅 모드,
  바인딩 소스와 루트, 선택된 에디터 상태, Godot 브리지 준비 상태를 보고합니다.
- `write_or_update_file`, `run_scene_edit_script`, `get_scene_tree`, `script_diagnostics`, `screenshot_scene` 같은 Godot 프로젝트 도구는 데몬으로 전달되며, 데몬은 연결된 Godot 플러그인으로 다시 전달합니다.

향후 Windows 설치 사용자 경로:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
