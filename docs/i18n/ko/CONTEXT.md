<!-- fennara-i18n: locale=ko source=CONTEXT.md sha256=7d76acbada75ade69b43dc52fcd543f90d678c04b3e9b50fc11601b8b1853fd4 -->
<a id="fennara-context"></a>
# Fennara 용어

<!-- fennara-doc-nav:start -->
[English](../../../CONTEXT.md) · [简体中文](../zh-CN/CONTEXT.md) · [Español](../es/CONTEXT.md) · [Português do Brasil](../pt-BR/CONTEXT.md) · [日本語](../ja/CONTEXT.md) · **한국어** · [Русский](../ru/CONTEXT.md) · [Français](../fr/CONTEXT.md) · [Deutsch](../de/CONTEXT.md) · [Türkçe](../tr/CONTEXT.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../CONTEXT.md)
<!-- fennara-doc-nav:end -->

이 파일은 Fennara 문서, 이슈, 릴리스 노트 및 에이전트 대상 지침에서 사용하는 공통 용어를 정의합니다.

<a id="product-terms"></a>
## 제품 용어

**Fennara**

이 저장소에 있는 Godot 인식 에이전트 환경입니다. Fennara는 AI 도구를 진단, 씬 검증, 런타임 오류, 스크린샷, 프로젝트 지침 같은 실제 Godot 피드백에 연결합니다.

**Godot Addon**

사용자의 Godot 프로젝트 `res://addons/fennara/`에 복사되는 설치 가능 플러그인입니다. 독 UI, Godot 대상 검사 도구, 네이티브 GDExtension 라이브러리, 패키징된 채팅 UI 자산, 런타임 헬퍼 스크립트, 프로젝트 로컬 애드온 버전을 담당합니다.

**Fennara CLI**

사용자 컴퓨터에 설치되는 `fennara` 명령입니다. 설치, 업데이트, CLI 자체 업데이트, doctor 검사, MCP 앱 설정, 웹뷰 필수 구성 요소 경고, C# 설정 검사, 생성되는 프로젝트 지침을 처리합니다.

**Local Package**

한 플랫폼 및 아키텍처용 MCP 서버, 데몬, 런타임 바이너리, 런처 바이너리 같은 로컬 Fennara 실행 파일이 담긴 릴리스 ZIP입니다.

**Project Guidance**

AI 코딩 에이전트가 언제 어떻게 Fennara를 사용할지 알 수 있도록 Godot 프로젝트에 배치되는 생성 지침 파일입니다. `AGENTS.md`와 `addons/fennara/ai/` 아래의 라우팅된 참고 문서를 포함합니다.

<a id="mcp-terms"></a>
## MCP 용어

**Fennara MCP Server**

Claude Code, Cursor, Cline, Gemini CLI 또는 다른 MCP 클라이언트 같은 AI 코딩 앱이 시작하는 로컬 stdio MCP 서버입니다. 외부 앱에 Fennara 도구를 제공합니다.

**MCP App**

`fennara mcp-setup`으로 구성하는 외부 AI 앱입니다. MCP 앱 설정은 어떤 외부 앱이 Fennara 도구를 호출할 수 있는지 제어하며, Fennara 내장 채팅이 사용할 모델을 선택하지는 않습니다.

**MCP Target**

프로젝트 바인딩이 없는 외부 MCP 연결에서 사용하는, 독에서
선택하는 데몬 전역 호환 대상입니다. 바인딩된 MCP 연결은 이 대상을
읽거나 변경하지 않습니다.

**MCP Project Binding**

Fennara MCP 프로세스가 시작할 때 한 번 선택하는 안정적인 Project Root입니다.
데몬 전역 MCP Target을 사용하지 않고 해당 프로세스의 호출을 일치하는
Godot Editor Session으로 라우팅합니다.

**Project Root**

하나의 Godot 프로젝트에 속한 `project.godot`이 있는 정규 파일 시스템
디렉터리입니다. Fennara는 저장소와 워크트리를 구분할 때 프로젝트 이름이 아닌
파일 시스템 식별성을 사용합니다.

**Godot Editor Session**

현재 연결된 하나의 Fennara 애드온과 Godot 에디터 인스턴스입니다. 프로젝트
경로와 Godot 프로세스 ID를 갖으며, 연결이 끊겼다가 새로 연결되어도 MCP
프로세스의 Project Binding은 변하지 않을 수 있습니다.

**Tool Schema**

인수, 제한, 워크플로 참고 사항을 포함한 Fennara MCP 도구의 모델 대상 설명입니다.

**Tool Result Envelope**

도구 호출 뒤 반환되는 간결한 모델 대상 결과입니다. Fennara 결과는 불필요한 원시 데이터를 쏟아내지 않고 상태, 중요한 발견, 다음에 유용한 맥락을 설명해야 합니다.

<a id="built-in-chat-terms"></a>
## 내장 채팅 용어

**Built-In Chat**

Godot 애드온 또는 시스템 브라우저에 표시되는 Fennara 자체 채팅 화면입니다. 외부 MCP 앱과 별개입니다. 사용자는 MCP용으로 Claude Code를 구성하면서도 내장 채팅에는 다른 제공업체나 모델을 선택할 수 있습니다.

**Chat Surface**

내장 채팅의 표시 모드입니다. 임베디드 모드는 네이티브 Godot 독 웹뷰를 사용합니다. 브라우저 모드는 로컬 데몬에서 같은 UI를 제공하고 시스템 브라우저에서 엽니다.

**Chat Provider**

OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, 로컬 Ollama 또는 LM Studio처럼 내장 채팅 응답을 생성할 수 있는 백엔드입니다.

**Model Ref**

내장 채팅에서 선택된 제공업체 한정 모델 식별자입니다. `/provider`, `/model` 같은 슬래시 명령으로 제공업체를 연결하고 모델 참조를 선택할 수 있습니다.

**Provider Connection**

API 키나 로컬 기본 URL을 포함하여 데몬이 관리하는 채팅 제공업체의 로컬 설정과 인증 상태입니다. 제공업체 비밀은 Godot 프로젝트가 아니라 데몬이 관리하는 로컬 저장소에 있어야 합니다.

**Generation Trace**

어시스턴트 메시지, 도구 호출, 제공업체 및 모델 선택, 사용량, 비용 로그를 이를 생성한 세대에 연결하는 내장 채팅 생성의 저장 메타데이터입니다.

<a id="runtime-and-webview-terms"></a>
## 런타임 및 웹뷰 용어

**Fennara Daemon**

MCP 호출과 내장 채팅 요청을 Godot 애드온에 연결하고, 로컬 런타임 상태를 저장하며, `/chat/` 같은 데몬 호스팅 채팅 경로를 제공하는 로컬 서비스입니다.

**Runtime Session**

실행 중인 씬 검사, 로그, 런타임 캡처에 사용하는 데몬 관리형 대화형 Godot 게임
프로세스입니다. 해당 프로젝트의 에디터가 재연결되어도 소유자인 정규 Project Root가
제어권을 유지합니다. 범위가 제한된 씬 검증과 독립형 스크린샷 호출은 별도 경로를 사용하며
Runtime Slot을 차지하지 않습니다.

**Runtime Slot**

연결된 모든 프로젝트를 통틀어 데몬이 관리하는 Runtime Session을 한 번에 하나만
시작하거나 실행하게 하는 시스템 전역 허가 상태입니다.

**Runtime Lease**

소유 Project Root에 주어지는, Runtime Slot을 점유할 수 있는 갱신 가능하고 시간이
제한된 권리입니다. 소유자 활동은 비활성 마감을 갱신하지만 절대 마감은 항상
적용됩니다.

**Godot Snapshot**

파일을 수정할 수 있는 Fennara 지원 턴 전에 만드는 되돌릴 수 있는 프로젝트 상태 스냅샷입니다. 설정 실패로 고립된 프롬프트가 남지 않도록 사용자 턴을 저장하기 전에 스냅샷 설정이 완료되어야 합니다.

**Webview Runtime**

Godot 안이나 주변에 내장 채팅을 표시하는 데 필요한 플랫폼 지원입니다. Windows는 WebView2, macOS는 WebKit/WKWebView, Linux는 Fennara 앱 데이터에 설치되는 공유 CEF 런타임을 사용합니다.

**Shared Linux CEF Runtime**

Linux 채팅 웹뷰가 사용하는 외부 Linux CEF 런타임 페이로드입니다. 사용자 Fennara 앱 데이터 아래에 한 번 설치되며 모든 Godot 애드온 ZIP에 포함되어서는 안 됩니다.

<a id="release-terms"></a>
## 릴리스 용어

**Release Manifest**

`fennara-release-manifest-v<version>.json`이라는 JSON 자산입니다. 릴리스 자산을 플랫폼에 매핑하고 SHA-256 해시를 기록하며 공유 런타임 자산과 `minimum_cli_version`을 선언합니다.

**Minimum CLI Version**

릴리스 매니페스트를 사용할 수 있는 가장 낮은 `fennara` CLI 버전입니다. 릴리스에 더 새로운 설치 또는 업데이트 로직이 필요하면 `scripts/release-policy.mjs`에서 해당 트랙을 업데이트하세요. 매니페스트 작성기는 릴리스 식별자를 검증한 뒤 정책을 적용하며, 워크플로가 값을 선택하지 않습니다.

**Latest Release**

정확한 버전 릴리스를 가리키는 GitHub의 Latest Release 포인터입니다. 설치 프로그램과 기본 업데이트는 GitHub API를 통해 이 포인터를 해석합니다. Fennara는 리터럴 `latest` 태그나 릴리스를 사용하지 않습니다. 게시 후 소스 파일을 업데이트해도 릴리스 자산은 바뀌지 않으며, 이미 게시된 매니페스트 자산은 명시적으로 교체해야 합니다.
