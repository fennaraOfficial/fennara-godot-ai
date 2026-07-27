<!-- fennara-i18n: locale=ko source=docs/repo-map.md sha256=dd8616d3a3f73e8f05b95898cd34041186e47818eefe9f41f1f0a951f1c27fdb -->
<a id="repo-map"></a>
# 저장소 지도

<!-- fennara-doc-nav:start -->
[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · **한국어** · [Русский](../ru/repo-map.md) · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../repo-map.md)
<!-- fennara-doc-nav:end -->

이 문서는 이 저장소에서 작업하는 기여자와 코딩 에이전트를 위한 빠른 지도입니다.

<a id="find-the-right-area"></a>
## 올바른 영역 찾기

| 변경 사항 | 기본 위치 |
| --- | --- |
| 사용자 설정 또는 CLI 동작 | `local/crates/fennara-cli/` |
| 외부 MCP 프로토콜 또는 스키마 | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| 내장 채팅 또는 데몬 동작 | `local/crates/fennara-daemon/` |
| Godot 편집기 통합 | `fennara-cpp/` |
| 채팅 UI | `ui/chat/` |
| 런타임 도우미 스크립트 | `runtime/` |
| 패키징 또는 릴리스 | `scripts/`, `.github/workflows/` |
| 사용자 문서 | `README.md`, `docs/` |

<a id="top-level"></a>
## 최상위 영역

| 경로 | 담당 범위 |
| --- | --- |
| `.github/` | 풀 리퀘스트 템플릿, 이슈 템플릿, GitHub Actions 워크플로. |
| `docs/` | 프로젝트 문서, 설정 가이드, 아키텍처 참고 자료, 예제, 데모, 릴리스 노트. |
| `docs/i18n/` | 로케일 매니페스트와 전체 번역 문서 트리. |
| `fennara-cpp/` | C++ Godot GDExtension 소스와 SCons 빌드 진입점. |
| `godot_demo/addons/fennara/` | 사용자 프로젝트에 복사되는 설치 가능한 Godot 애드온 페이로드. |
| `local/` | Rust CLI, MCP 서버, 데몬, 스키마, 로컬 런타임 코드. |
| `media/` | 문서에서 사용하는 이미지와 공개 미디어. |
| `runtime/` | `runtime_session`과 `runtime_script`에서 사용하는 Godot 런타임 도우미 스크립트 소스. |
| `scripts/` | 버전 관리, 패키징, 릴리스 도우미 스크립트. |
| `ui/chat/` | 선택 사항인 편집기 내 웹 채팅 UI의 소스. |
| `local/templates/` | `fennara install`이 Godot 프로젝트에 쓰고 `fennara update`가 새로 고치는 간결한 프로젝트 지침과 요청 시 제공되는 AI 지식 페이지. |
| `local/webview-runtimes/` | Linux CEF 페이로드처럼 공유 Fennara 앱 데이터에 설치되는 외부 웹뷰 런타임의 매니페스트 및 구성 파일. |
| `install.ps1` / `install.sh` | GitHub 릴리스에서 Fennara CLI를 설치하는 부트스트랩 스크립트. |
| `VERSION` | 버전의 정본. |
| `README.md` | 짧은 사용자용 개요와 빠른 시작. |
| `docs/README.md` | 작업 중심 문서 색인. |
| `docs/setup.md` | 애드온 우선 사용자 설정, 채팅 필수 조건, MCP 연결, 업데이트 흐름, 문제 해결. |
| `docs/cli.md` | 터미널 명령 참조, CLI가 담당하는 설치 및 업데이트 동작, 복구, 진단, 앱 데이터 배치, 자동화 지침. |
| `docs/telemetry.md` | 익명 활동 페이로드, 앱 데이터 상태, 전송 동작, 월간 활성 사용자 정의, 옵트아웃 제어. |
| `CONTRIBUTING.md` | 기여 규칙. |
| `SECURITY.md` | 보안 문제 보고 정책. |
| `LICENSE.md` | 프로젝트 라이선스. |

<a id="local-rust-packages"></a>
## 로컬 Rust 패키지

| 경로 | 담당 범위 |
| --- | --- |
| `local/crates/fennara-cli/` | `fennara` 명령: 설치, 업데이트, CLI 자체 업데이트, doctor, 작업 진단, 웹뷰 필수 조건 검사, C# 지원, MCP 앱 설정, 생성된 프로젝트 지침. |
| `local/crates/fennara-cli/src/operation.rs` | 공개 설치 및 업데이트 작업 조정자, 단계, CLI 인계 진입점. |
| `local/crates/fennara-cli/src/operation/` | 집중화된 작업 저널, 영구 저장소, 진단 정보 교정, 테스트 모듈. |
| `local/crates/fennara-cli/src/project_addon.rs` | 기존 프로젝트 애드온 버전과 현재 플랫폼 GDExtension 라이브러리 검증. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Godot을 시작하기 전에 Fennara의 영구 런타임 autoload만 제거하는 애드온 없는 CI 내보내기 준비. |
| `local/crates/fennara-cli/src/release_identity.rs` | 안정 및 스테이징 애드온 신원, 정확한 릴리스 선택자, 풀 리퀘스트 채널 검증, 기존 안정 버전 호환성. |
| `local/crates/fennara-cli/src/release_channel.rs` | 채널별 스테이징 포인터 검증과 정확한 버전 릴리스로의 해석. |
| `local/crates/fennara-cli/src/release_manifest.rs` | 릴리스 매니페스트 구문 분석, 자산 해시 검증, 신원 바인딩, 플랫폼 패키지 선택. |
| `local/crates/fennara-cli/src/release_version.rs` | 매니페스트와 릴리스 선택에서 사용하는 공유 CLI SemVer 구문 분석 및 우선순위. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | 프로젝트 애드온 파일을 교체하지 않고 기존의 완전한 애드온을 정확한 버전으로 채택. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | 설치와 doctor에서 사용하는 공유 데몬 상태 검사, 정확한 버전 준비 상태, 시작. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | 프로세스 수준 실패, 영구 진단, 정보 교정, 실패 시 폐쇄되는 작업 로그 테스트. |
| `local/crates/fennara-cli/src/diagnostics.rs` | 최신 또는 지정된 정제 작업 보고서에 대한 사용자용 접근. |
| `local/crates/fennara-mcp/` | 로컬 stdio MCP 서버와 도구 스키마 전달. |
| `local/crates/fennara-daemon/` | 런타임 세션과 Godot 브리지 작업에 사용하는 로컬 데몬. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | 익명 일간 활성 사용자 스케줄러, 제한된 큐, HTTP 전송, 데몬 수명 주기 통합. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | 무작위 설치 신원 검증, 원자적 앱 데이터 영구 저장, 일간 수신 상태, 옵트아웃 정리. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | 내장 채팅 승인 모드, 도구 위험 분류, 권한 결정, 대기 중인 승인 요청 유형. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | 데몬이 담당하는 내장 채팅 `exec_command` 구현: 셸 감지, cwd 검증, 프로세스 생성, 시간 제한 및 프로세스 트리 종료, 출력 캡처, 결과 산출물 기록, 결과 형식 지정. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | 내장 채팅 컨텍스트 압축 계획자: 정확한 꼬리 보호, OpenCode 방식의 오래된 도구 결과 압력 가지치기, 요약 청크 선택 및 저장과 재생, 요약 프롬프트 직렬화, 토큰 예산, 자리표시자 렌더링. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | 내장 채팅 PromptBuilder와 생성된 런타임 환경 컨텍스트. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | 로컬 전용 내장 채팅 추적 기록기, SQLite 이벤트 행, 보존 정책, 디버그 조회 도우미. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | 내장 채팅 제공자 런타임 기본 요소, 카탈로그 및 해석, 컨텍스트 사전 점검 훅, 정규화된 스트림 및 오류 유형, 그리고 OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, 사용자 지정 엔드포인트, Ollama/local, LM Studio용 OpenAI 호환 또는 Anthropic 호환 어댑터. |
| `local/schemas/tools/` | 공유 도구 JSON 스키마. 외부 MCP 서버와 내장 채팅은 각각 허용된 하위 집합을 내장합니다. |
| `local/webview-runtimes/linux-cef.json` | 릴리스 매니페스트 생성, doctor 출력, 기존 대체 경로에 사용하는 Linux CEF 런타임 자리표시자 및 생성 매니페스트. CEF를 애드온 zip 안에 넣지 않고 공유 앱 데이터 배치와 아카이브 메타데이터를 기록합니다. |
| `local/Cargo.toml` | Rust 워크스페이스 구성. |
| `local/Cargo.lock` | 잠긴 Rust 의존성 그래프. |

<a id="gdextension-source"></a>
## GDExtension 소스

| 경로 | 담당 범위 |
| --- | --- |
| `fennara-cpp/SConstruct` | GDExtension 빌드 진입점. |
| `fennara-cpp/include/` | 공개 C++ 헤더. |
| `fennara-cpp/src/` | C++ 구현. |
| `fennara-cpp/src/setup/` | 네이티브 최초 실행 설정 상태, 릴리스 매니페스트 CLI 부트스트랩, 해시 검증, CLI 실행, 영구 작업 진행 상황 판독기. |
| `fennara-cpp/src/release/version.cpp` | 릴리스 및 업데이트 검색에서 사용하는 네이티브 SemVer 검증과 우선순위. |
| `fennara-cpp/src/release/identity.cpp` | 패키지된 안정 및 스테이징 신원 검증과 기존 안정 버전 호환성. |
| `fennara-cpp/src/release/discovery.cpp` | GitHub Latest 및 격리된 스테이징 채널 업데이트 검색. |
| `fennara-cpp/src/update/` | 정확한 대상 업데이트 조정, 영구 수신 확인 검색, 닫기 및 설치 인계, 복구 UI 상태. |
| `fennara-cpp/src/ui/setup_panel.cpp` | 진행 상황, 재시도, 로그, 정제된 보고서 작업을 제공하는 웹뷰 비의존 최초 실행 설정 패널. |
| `fennara-cpp/vendor/cef/` | Linux OSR 브리지에서 사용하는 공식 CEF 139 헤더 스냅샷. 런타임 바이너리는 애드온 외부에 둡니다. |
| `fennara-cpp/src/ui/webview_host*` | 네이티브 편집기 내 채팅 웹뷰 호스트와 플랫폼 백엔드. |
| `fennara-cpp/src/ui/native_webview_occlusion.*` | 겹치는 Godot 팝업 또는 최상위 편집기 UI가 표시되는 동안 네이티브 웹뷰 오버레이를 일시적으로 숨기는 Windows 및 macOS 공용 감지 로직. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Linux 전용 공유 CEF 런타임 검색, 마커 검증, 동적 `libcef.so` 로더 기반. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Linux 전용 CEF 오프스크린 렌더링 표면, Godot 입력 전달, 브리지 ABI 로딩, Godot 텍스처 업데이트. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | 고정된 공식 CEF 139 `libcef_dll_wrapper` 소스와 Fennara의 CEF OSR 어댑터로 빌드되는 작은 Linux 전용 브리지 라이브러리. 주 GDExtension은 외부 `libcef.so` 런타임이 로드된 뒤 이 라이브러리를 dlopen합니다. |
| `fennara-cpp/src/tools/` | Godot 대상 도구 구현. |
| `fennara-cpp/src/lsp/` | 스크립트 진단 및 언어 서버 도우미. |
| `fennara-cpp/src/csharp/` | 빌드 전용 C# 프로젝트 선택, 백그라운드 준비, 격리된 진단, 런타임 사전 점검. |
| `fennara-cpp/src/runtime/` | 런타임 장면 사전 점검, 스크립트 진단, 디버거 스냅샷을 포함하여 도구에서 사용하는 네이티브 런타임 지원. |
| `fennara-cpp/godot-cpp/` | Godot C++ 바인딩 서브모듈. |

<a id="addon-payload"></a>
## 애드온 페이로드

| 경로 | 담당 범위 |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Godot GDExtension 등록 파일. |
| `godot_demo/addons/fennara/VERSION` | 애드온 패키지 버전. |
| `godot_demo/addons/fennara/release.json` | 정확한 버전, 릴리스 태그, 채널, 스테이징 소스 커밋을 포함한 패키지된 안정 또는 스테이징 신원. |
| `godot_demo/addons/fennara/bin/` | 빌드된 플랫폼 라이브러리. |
| `godot_demo/addons/fennara/dist/` | 편집기 내 채팅 웹뷰에서 사용하는 패키지된 웹 UI 자산. |
| `godot_demo/addons/fennara/runtime/` | 애드온 안에 제공되는 `runtime/`의 동기화된 패키지 사본. |
| `godot_demo/tests/first_run_setup_test.gd` | 헤드리스 네이티브 최초 실행 설정 상태 및 결정적 실패 테스트. |
| `godot_demo/tests/export_plugin_test.gd` | 헤드리스 네이티브 내보내기 제외 및 autoload 복원 회귀 테스트. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | 헤드리스 네이티브 스크린샷 인자 계약 회귀 테스트. |
| `godot_demo/tests/image_sheet_test.gd` | 헤드리스 공유 스크린샷 및 런타임 시트 구성 회귀 테스트. |
| `godot_demo/tests/runtime_image_context_test.gd` | 헤드리스 런타임 원시 프레임, 시트, 임의 Image 출력 회귀 테스트. |

<a id="runtime-helper-source"></a>
## 런타임 도우미 소스

| 경로 | 담당 범위 |
| --- | --- |
| `runtime/game_capture_helper.gd` | 장면 세션과 런타임 검사에서 GDExtension이 로드하는 런타임 도우미 진입점. |
| `runtime/image_label.gd` | 캡처 후 구성된 Image 셀에 찍는 간결하고 결정적인 레이블. |
| `runtime/image_sheet.gd` | 스크린샷과 런타임 스크립트 컨텍스트에서 사용하는 공유 순수 Image 시트 구성. |
| `runtime/screenshot_script_context.gd` | 네이티브 캡처 컨텍스트에 공유 Image 구성을 추가하는 공개 스크린샷 스크립트 퍼사드. |
| `runtime/runtime_script_context.gd` | 원시 프레임, Image 구성 및 출력, 대기, 입력, 스냅샷, 조건, 레이캐스트, 클릭을 포함하여 `runtime_script`에 공개되는 `ctx` 도우미 표면. |
| `runtime/runtime_input_driver.gd` | 키, 마우스 버튼, 절대 마우스 이동, 상대 마우스 이동, 수정 키, 입력 정리를 위한 저수준 런타임 입력 이벤트 드라이버. |
| `runtime/runtime_node_snapshot.gd` | 런타임 노드 조회, 존재 확인, 오래된 참조에 안전한 스냅샷, 속성 읽기, 자식 요약. |
| `runtime/runtime_physics_query.gd` | 간결한 적중 수신 확인을 제공하는 런타임 2D 및 3D 정확한 레이캐스트와 스캔 도우미. |
| `runtime/runtime_query_utils.gd` | 벡터 강제 변환, 안전한 노드 및 경로 해석, 객체 신원, 일반 대상 일치를 위한 공유 런타임 조회 유틸리티. |
| `runtime/runtime_capture_store.gd` | 런타임 세션, 스크립트, 환경 검사에서 사용하는 런타임 캡처 및 상태 산출물 기록기. |
| `runtime/runtime_check_runner.gd` | 비대화형 장면 실행 명세용 런타임 검사 실행기. |

<a id="scripts-and-workflows"></a>
## 스크립트와 워크플로

| 경로 | 담당 범위 |
| --- | --- |
| `scripts/set-version.mjs` | 저장소 전체의 버전 지정 파일을 업데이트합니다. |
| `scripts/check-version.mjs` | 버전 동기화를 검사합니다. |
| `scripts/release-identity.mjs` | SemVer 릴리스 신원과 PR별 스테이징 포인터를 위한 공유 Node 검증 및 생성. |
| `scripts/release-policy.mjs` | 안정 및 스테이징 릴리스 매니페스트에 호환되는 최소 게시 CLI 정책. |
| `scripts/staging-candidate.mjs` | 신뢰할 수 있는 스테이징 후보 신원 생성과 단조 증가 PR별 포인터 결정. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | 집중화된 스테이징 애드온, 아카이브, 매니페스트, 공유 파일 시스템, 게시 번들 검증. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | 신뢰할 수 없는 빌드 출력과 신뢰할 수 있는 게시 번들을 위한 엄격한 검증 진입점. |
| `scripts/check-staging-channel-advance.mjs` | 스테이징 채널 포인터가 전진하기 전에 단조성과 출처 검사를 적용합니다. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | 포인터 승격 전에 게시된 자산 바이트와 공개 다운로드 동작을 검증합니다. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | 무시되는 임시 Godot 프로젝트를 빌드하고 편집기 GDExtension을 대상으로 읽기 전용으로 가져온 `PackedScene` 검사를 스모크 테스트합니다. |
| `scripts/release-targets.mjs` | 지원되는 플랫폼 릴리스 대상과 패키지된 자산 이름을 정의합니다. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | 고정된 후보 신원과 작은 채널 포인터를 기록합니다. |
| `scripts/sync-chat-ui.mjs` | 빌드가 필요 없는 채팅 UI 소스를 애드온 페이로드로 복사합니다. |
| `scripts/sync-runtime.mjs` | 저장소 루트의 런타임 도우미 소스를 애드온 페이로드로 복사합니다. |
| `scripts/sync-doc-navigation.mjs` | 문서 산문을 번역하지 않고 문서 탐색, 소스 해시, 안정적인 앵커를 추가합니다. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | 번역 범위, 최신성, Markdown 구조, URL, 링크를 검증합니다. |
| `scripts/package-preview.mjs` | 플랫폼 빌드 후 애드온, CLI, 로컬 런타임 미리 보기 및 릴리스 zip을 조립합니다. |
| `scripts/prepare-linux-cef-runtime.mjs` | 별도의 Linux x64 CEF 런타임 zip을 스테이징하고, 스테이징된 ELF 바이너리를 스트립하고, 필수 파일을 검증하며, 생성된 릴리스 매니페스트를 기록할 수 있습니다. |
| `scripts/prepare-linux-cef-sdk.mjs` | `libcef_dll/` 래퍼 소스가 필요한 CI 빌드를 위해 고정된 공식 CEF 139 Linux 최소 SDK를 다운로드하고 추출합니다. |
| `scripts/check-linux-cef-runtime-release.mjs` | 생성된 `local/webview-runtimes/linux-cef.json` 매니페스트를 기준으로 Linux CEF 런타임 릴리스 자산을 검증합니다. |
| `scripts/write-release-manifest.mjs` | 로컬 패키지, 애드온, 공유 런타임 해시를 포함하여 릴리스 자산에서 `fennara-release-manifest-v<version>.json`을 기록하고 검증합니다. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | 별도의 CEF 런타임 zip 안에 패키지되는 최소 Linux CEF 하위 프로세스 도우미 소스. |
| `.github/workflows/version-check.yml` | 버전 일관성 검사. |
| `.github/workflows/gdextension-build.yml` | 교차 플랫폼 GDExtension 빌드 검사와 Windows 헤드리스 네이티브 최초 실행 설정 상태 테스트. |
| `.github/workflows/local-build.yml` | Rust 로컬 패키지 빌드 검사. |
| `.github/workflows/package-preview.yml` | Linux 채팅 스모크 테스트를 위한 테스트 전용 Linux CEF 런타임 산출물을 포함한 수동 패키지 미리 보기 산출물. |
| `.github/workflows/release.yml` | 생성된 Linux CEF 런타임 패키징, 릴리스 매니페스트 생성, 최종 자산 검증을 포함한 수동 GitHub 릴리스 게시. |
| `.github/workflows/staging-release.yml` | 수동 정확한 SHA 스테이징 빌드, 검증 전용 시험 실행, 정확한 프리릴리스 게시, PR별 포인터 전진. |

<a id="where-to-change-things"></a>
## 변경 위치 찾기

| 작업 | 시작 위치 |
| --- | --- |
| Godot 도구 추가 또는 변경 | `fennara-cpp/src/tools/` 및 `local/schemas/tools/` |
| MCP 스키마 텍스트 변경 | `local/schemas/tools/` |
| `fennara install` 또는 `fennara update` 변경 | `local/crates/fennara-cli/src/`; 네이티브 스테이징과 분리된 적용 및 롤백은 `release_update.rs`, `update_stage.rs`, `update_stage/`, `update_apply/`가 담당 |
| CLI 명령 또는 터미널 동작 변경 | `local/crates/fennara-cli/src/` 및 `docs/cli.md` |
| 네이티브 업데이트 진행 상황, 종료 확인, 활성화 핸드셰이크 또는 복구 변경 | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs`, `ui/chat/` |
| 네이티브 최초 실행 설정 또는 CLI 부트스트랩 변경 | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp`, `fennara-cpp/src/ui/dock.cpp` |
| 내보내기 중 애드온 제외 변경 | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp`, `godot_demo/tests/export_plugin_test.gd` |
| 설치 및 업데이트 작업 로그, 단계, 오류 코드 또는 진단 보고서 변경 | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/`, `local/crates/fennara-cli/src/diagnostics.rs` |
| 웹뷰 필수 조건 검사 변경 | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs`, `fennara-cpp/src/ui/webview_host*` |
| 생성된 프로젝트 지침 변경 | `local/templates/` 및 `local/crates/fennara-cli/src/project_guidance.rs` |
| 생성된 데모 애드온 지침 동기화 | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs`, `godot_demo/addons/fennara/ai/` |
| MCP 앱 설정 변경 | `local/crates/fennara-cli/src/mcp_setup.rs` 및 `docs/mcp-setup.md` |
| 런타임 세션 프로세스 및 로그 동작 변경 | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/`, `fennara-cpp/src/tool_results/` |
| `runtime_script` ctx 도우미, 입력, 스냅샷, 대기, 레이캐스트, 캡처 또는 정리 변경 | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json`, `docs/tools.md` |
| 편집기 내 채팅 UI, 슬래시 명령 또는 모델 및 제공자 선택기 변경 | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp`, `fennara-cpp/src/ui/webview_host*` |
| 내장 채팅 제공자 변경 | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` |
| 익명 텔레메트리 필드, 일정 또는 개인 정보 제어 변경 | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/`, `docs/telemetry.md` |
| 벤더 채팅 UI 라이브러리 변경 | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/`, `THIRD_PARTY_NOTICES.md` |
| C# 지원 변경 | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/`, C# 도구 스키마 및 지침 |
| 릴리스 패키지, 최소 CLI 정책 또는 CLI 자체 업데이트 변경 | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs`, `.github/workflows/release.yml` |
| 버전 올리기 | `node scripts/set-version.mjs <version>` |
| 채팅과 MCP, 제공자 또는 슬래시 명령의 설정 및 문서 업데이트 | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md`, `llms.txt` |
| 문서 번역 업데이트 | 정본 영어 페이지, `docs/i18n/languages.json`, 해당 로케일 페이지, `scripts/sync-doc-navigation.mjs`, `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## 참고

- 주요 소스 영역을 추가하거나 이동할 때 이 파일을 최신 상태로 유지하세요.
- 릴리스 단계는 [release.md](release.md)에 유지하세요.
- 설정 단계는 [setup.md](setup.md)에 유지하세요.
- 터미널 명령 동작은 [cli.md](cli.md)에 유지하세요.
