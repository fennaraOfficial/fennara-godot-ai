<!-- fennara-i18n: locale=ko source=docs/faq.md sha256=dc4d4d61e292532de7c87813b66925ae4ead2b2fbc0417b2366d8b53b42f7c4f -->
<a id="faq"></a>
# 자주 묻는 질문

<!-- fennara-doc-nav:start -->
[English](../../faq.md) · [简体中文](../zh-CN/faq.md) · [Español](../es/faq.md) · [Português do Brasil](../pt-BR/faq.md) · [日本語](../ja/faq.md) · **한국어** · [Русский](../ru/faq.md) · [Français](../fr/faq.md) · [Deutsch](../de/faq.md) · [Türkçe](../tr/faq.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../faq.md)
<!-- fennara-doc-nav:end -->

설치 및 업데이트는 [설정](setup.md)부터 시작하세요. 짧은 답변과 상세 참고 문서 링크는 이 페이지를 사용하세요.

| 질문 | 짧은 답변 |
| --- | --- |
| 제공업체 키가 필요한가요? | 내장 채팅에서 클라우드 제공업체를 사용할 때만 필요합니다 |
| 외부 MCP 앱을 대신 사용할 수 있나요? | 예, 외부 앱은 자체 모델 계정을 사용합니다 |
| Fennara가 프로젝트를 Fennara 서버에 업로드하나요? | 아니요 |
| Godot 에디터 여러 개를 열 수 있나요? | 예, 독에서 외부 MCP 대상을 선택하세요 |

<a id="is-fennara-only-a-code-generator"></a>
## Fennara는 코드 생성기일 뿐인가요?

아닙니다. Fennara는 Godot을 인식하는 에이전트 워크플로입니다. 프로젝트 파일, 씬, 진단, 런타임 오류, 스크린샷, Godot 에디터 맥락을 활용할 수 있습니다.

<a id="is-fennara-just-another-godot-mcp-command-server"></a>
## Fennara는 또 다른 Godot MCP 명령 서버인가요?

아닙니다. MCP는 Codex, Claude, Cursor, Gemini, Antigravity 같은 앱에서 Fennara를 사용하는 한 가지 방법입니다. Fennara에는 선택 사항인 내장 채팅 독도 있습니다. 제품의 핵심 주장은 Godot 피드백 루프입니다. 진단, 검증, 런타임 오류, 스크린샷, 구조화된 도구 결과를 통해 에이전트가 실수를 수정할 수 있게 합니다.

<a id="does-fennara-replace-godot-knowledge"></a>
## Fennara가 Godot 지식을 대체하나요?

아닙니다. Fennara는 Godot을 불필요하게 만들려는 것이 아닙니다. AI 에이전트가 실제 Godot 엔진에 근거해 작업하도록 설계되었습니다.

<a id="how-should-i-install-fennara"></a>
## Fennara를 어떻게 설치해야 하나요?

Windows와 Linux에서는 애드온을 추가하고 Fennara 독을 연 다음 **Set Up Fennara**를 누르거나 터미널에서 설치하세요. macOS에서는 브라우저로 내려받은 애드온 ZIP을 직접 압축 해제할 때 생길 수 있는 보안 알림을 피하도록 CLI로 설치하세요. 두 경로 모두 [설정](setup.md)에 설명되어 있습니다.

<a id="why-does-macos-say-it-cannot-verify-libfennaramacoseditor"></a>
## macOS가 `libfennara.macos.editor`를 확인할 수 없다고 하는 이유는 무엇인가요?

릴리스 애드온에는 현재 Apple 공증을 받지 않은 네이티브 라이브러리가 포함되어 있습니다. 브라우저로 애드온 ZIP을 내려받아 직접 압축 해제하면 Finder가 해당 라이브러리에 격리 메타데이터를 전파하여 macOS 알림을 일으킬 수 있습니다.

이를 피하려면 [CLI 설치](setup.md#터미널에서-설치-macos-권장)를 사용하세요. 이미 알림이 표시된다면 Godot을 닫고 직접 복사한 `addons/fennara/` 폴더를 제거한 뒤 CLI를 설치하고 프로젝트 디렉터리에서 `fennara install`을 실행하세요. CLI는 브라우저와 Finder 격리 경로를 거치지 않고 같은 애드온을 설치합니다.

<a id="do-i-need-a-chat-provider-api-key"></a>
## 채팅 제공업체 API 키가 필요한가요?

Fennara 내장 채팅 독에서 클라우드 제공업체를 사용하려는 경우에만 필요합니다. 외부 MCP 클라이언트는 자체 모델 및 앱 구성을 사용하며 Fennara 채팅에 제공업체 키를 넣지 않고도 Fennara MCP 도구를 사용할 수 있습니다.

내장 채팅은 클라우드 API 키 없이 로컬 Ollama 또는 LM Studio도 사용할 수 있습니다. [내장 채팅 제공업체](providers.md)를 참고하세요.

<a id="why-does-the-dock-ask-for-a-provider-if-i-already-ran-mcp-setup---claude"></a>
## 이미 `mcp-setup --claude`를 실행했는데 독에서 제공업체를 요구하는 이유는 무엇인가요?

`fennara mcp-setup --claude`는 Claude를 Fennara의 Godot MCP 도구에 연결합니다. Fennara 내장 독을 Claude에 연결하거나 Claude 구독을 Fennara 채팅과 공유하지는 않습니다.

외부 MCP 흐름에는 Claude Code 또는 Claude Desktop을 사용하세요. Godot 안의 Fennara 독에서 채팅하려는 경우에만 별도의 제공업체를 구성하세요. [MCP 앱과 내장 채팅](chat-vs-mcp.md)을 참고하세요.

<a id="what-are-provider-and-model"></a>
## `/provider`와 `/model`은 무엇인가요?

Fennara 내장 채팅 독의 슬래시 명령입니다. `/provider`는 제공업체 선택기를 열고 `/model`은 모델 선택기를 엽니다. UI 바로 가기이며 외부 MCP 도구도, 모델에 보내는 텍스트도 아닙니다. [내장 채팅 슬래시 명령](slash-commands.md)을 참고하세요.

<a id="does-fennara-send-my-godot-project-to-a-fennara-server"></a>
## Fennara가 Godot 프로젝트를 Fennara 서버에 보내나요?

아닙니다. 일반 OSS 경로에서 MCP 클라이언트, 데몬, Godot 애드온은 로컬에서 실행됩니다. 내장 채팅은 OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax 또는 로컬 Ollama/LM Studio 서버처럼 사용자가 구성한 제공업체에만 모델 요청을 보냅니다.

<a id="which-project-receives-mcp-tool-calls-if-multiple-godot-editors-are-open"></a>
## Godot 에디터를 여러 개 열면 어느 프로젝트가 MCP 도구 호출을 받나요?

데몬은 외부 MCP 호출을 활성 MCP 대상으로 라우팅합니다. Godot의 Fennara 독에서 MCP target 컨트롤을 사용해 프로젝트를 선택하세요. 내장 채팅 세션은 해당 채팅을 연 Godot 에디터에 연결된 상태를 유지합니다.

<a id="why-does-linux-install-a-separate-cef-runtime"></a>
## Linux가 별도의 CEF 런타임을 설치하는 이유는 무엇인가요?

Linux 임베디드 채팅은 CEF 오프스크린 렌더링을 사용합니다. CEF 페이로드가 크므로 Fennara는 모든 Godot 프로젝트 애드온에 복사하는 대신 사용자 Fennara 앱 데이터 아래에 한 번 설치합니다.

<a id="is-the-addon-supposed-to-contain-libcefso"></a>
## 애드온에 `libcef.so`가 들어 있어야 하나요?

아닙니다. `libcef.so`, CEF 리소스, 로케일 팩, CEF 헬퍼는 공유 Linux CEF 런타임에 있어야 합니다. 애드온에는 Godot 애드온 파일, GDExtension 바이너리, 채팅 UI 파일, ripgrep 같은 작은 번들 헬퍼 바이너리만 포함되어야 합니다.

<a id="what-if-the-built-in-chat-webview-cannot-start"></a>
## 내장 채팅 웹뷰를 시작할 수 없으면 어떻게 하나요?

Fennara MCP 도구는 계속 작동합니다. 선택 사항인 에디터 내 채팅 독만 플랫폼 웹뷰를 필요로 합니다. Windows에서는 `fennara doctor`가 누락되었다고 보고하면 Microsoft Edge WebView2 Runtime을 설치하세요. macOS에서 WKWebView는 시스템 WebKit.framework에서 제공됩니다. Linux에서는 `fennara update`를 실행해 릴리스 관리 CEF 런타임을 설치하거나 복구하세요.

Chat Settings에서 **Open chat in my system browser next time**을 사용할 수도 있습니다. 같은 Fennara 내장 채팅과 제공업체 설정을 유지하면서 임베디드 Godot 웹뷰 대신 로컬 데몬을 통해 시스템 브라우저에서 UI를 엽니다. 설정을 바꾼 뒤 Godot을 다시 시작하세요.

<a id="does-opening-chat-in-my-browser-use-claude-or-my-mcp-app"></a>
## 브라우저에서 채팅을 열면 Claude 또는 MCP 앱을 사용하나요?

아닙니다. 브라우저 표시는 Fennara 내장 채팅의 UI 및 런타임 선택일 뿐입니다. 여전히 Fennara 채팅 설정에서 선택한 제공업체를 사용합니다. `fennara mcp-setup --claude` 같은 명령은 외부 MCP 앱을 구성하며 내장 채팅 모델은 구성하지 않습니다.

<a id="does-fennara-update-rewrite-mcp-app-config"></a>
## `fennara update`가 MCP 앱 구성을 다시 작성하나요?

아닙니다. `fennara update`는 필요한 경우 설치된 CLI, 프로젝트 애드온, 로컬 런타임 패키지, 생성되는 프로젝트 지침, 플랫폼 관리 런타임 자산을 갱신합니다. MCP 앱 구성을 추가하거나 복구할 때만 `fennara mcp-setup`을 다시 실행하세요.

<a id="where-does-chat-history-live"></a>
## 채팅 기록은 어디에 있나요?

채팅 기록은 데몬이 로컬에 저장하며 현재 Godot 프로젝트 범위에 속합니다. 제공업체 키와 로컬 제공업체 URL도 Godot 프로젝트 밖에서 데몬이 로컬로 저장합니다.

<a id="what-should-agents-use-fennara-tools-for"></a>
## 에이전트는 Fennara 도구를 어디에 사용해야 하나요?

씬 트리, 변경된 노드 및 리소스 속성, 진단, 검증, 런타임 세션, 스크린샷, 에디터 디버거 상태 같은 Godot 인식 피드백에 Fennara를 사용하세요. Fennara 전용 도구가 필요하지 않은 경우 MCP 클라이언트는 자체 일반 파일 읽기 및 검색 도구를 계속 사용해야 합니다.
