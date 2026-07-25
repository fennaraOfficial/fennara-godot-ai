<!-- fennara-i18n: locale=ko source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · **한국어** · [Русский](README.ru.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/ko/demos.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

[Somni Game Studios](https://somnigamestudios.com/)를 비롯한 Godot 개발자와 팀이 Fennara를 사용하고 있습니다.

Fennara는 AI 어시스턴트를 Godot에 실시간으로 연결합니다. Codex, Claude, Cursor, Gemini, Antigravity 같은 MCP 지원 앱에서 사용할 수도 있고, 선택 사항인 에디터 내 채팅 독에서 사용할 수도 있습니다.

에이전트는 프로젝트 파일만 보고 추측하는 대신 에디터 안에서 씬을 살펴보고, 스크립트를 확인하고, 스크린샷을 캡처하고, 런타임 오류를 읽고, 변경 사항을 검증할 수 있습니다.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="다른 Godot MCP와 Fennara 비교" width="100%" />
      </a>
    </td>
    <td>
      <strong>주요 데모 보기</strong><br />
      다른 Godot MCP와 Fennara 비교.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">동영상 재생</a><br />
      <a href="docs/i18n/ko/demos.md">모든 데모 동영상 보기</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## 주요 기능

- MCP를 통해 외부 AI 앱에 Godot 인식 도구를 제공합니다
- Godot 에디터 안에 선택 사항인 로컬 채팅 독을 추가합니다
- 씬 트리, 진단, 스크린샷, 런타임 로그, 검증 결과 등 실제 Godot 피드백을 반환합니다
- 파일 시스템만 보는 것이 아니라 열려 있는 에디터를 기준으로 에이전트의 작업을 확인할 수 있게 합니다

외부 MCP 앱과 내장 채팅은 서로 별도의 모델 설정을 사용합니다. [MCP 앱과 내장 채팅](docs/i18n/ko/chat-vs-mcp.md), [내장 채팅 제공업체](docs/i18n/ko/providers.md)를 참고하세요.

<a id="requirements"></a>
## 요구 사항

- Godot 4.5 이상.
- 지원되는 데스크톱 OS: Windows x86_64, Linux x86_64 또는 macOS arm64.
- Claude, Codex, Cursor, Gemini, Antigravity 또는 다른 외부 AI 앱에서 Fennara를 사용하려는 경우에만 MCP 지원 코딩 앱이 필요합니다.
- 내장 Fennara 채팅 독을 사용하려는 경우에만 채팅 제공업체가 필요합니다. 클라우드 제공업체 키 또는 Ollama / LM Studio 같은 로컬 제공업체를 사용할 수 있습니다.

전체 설치 안내는 [설정](docs/i18n/ko/setup.md)을 참고하세요.

<a id="what-setup-adds"></a>
## 설정으로 추가되는 항목

- `res://addons/fennara/`에 유지되는 Fennara 애드온
- Fennara 앱 데이터에 설치되는 작은 `fennara` CLI
- AI 코딩 앱이 사용하는 로컬 MCP 서버
- MCP 및 채팅 요청을 열려 있는 Godot 에디터로 전달하는 로컬 데몬
- AI 에이전트를 위해 생성되는 프로젝트 지침

내장 채팅 독은 플랫폼 웹뷰를 사용합니다. Windows에서는 Microsoft Edge WebView2, macOS에서는 WKWebView/WebKit, Linux에서는 Fennara가 관리하는 공유 CEF 런타임을 사용합니다. 선택 사항인 채팅 독을 시작할 수 없어도 MCP 도구는 계속 작동합니다.

<a id="install"></a>
## 설치

Windows와 Linux에서는 애드온 설치와 CLI 설치 중 하나를 선택하세요. macOS에서 애드온 ZIP을 직접 내려받아 압축 해제할 때 나타날 수 있는 macOS 보안 알림을 피하려면 아래의 CLI 설치를 사용하세요.

<a id="add-the-addon-to-your-project"></a>
### 프로젝트에 애드온 추가

- [최신 릴리스](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)를 열고 `fennara-addon-latest.zip`을 내려받은 다음, 그 안의 `addons/fennara/` 폴더를 프로젝트에 압축 해제합니다.

프로젝트를 열고 Fennara 독을 선택한 다음 **Set Up Fennara**를 누릅니다.

Fennara는 에디터 의존성이며 게임 런타임 의존성이 아닙니다. 내보내는 동안 에디터 플러그인은 내보낼 프로젝트에서 런타임 autoload를 제거하고 `res://addons/fennara/` 및 `res://.fennara/` 아래의 파일을 제외합니다. 내보내기가 끝나면 에디터 프로젝트를 원래 상태로 복원합니다. CI 체크아웃에서 `.gitignore`로 애드온을 제외한다면 Godot을 시작하기 전에 `fennara prepare-export --project path/to/project`를 실행하거나 해당 체크아웃에 애드온을 설치하세요. Godot은 내보내기 플러그인이 실행되기 전에 autoload 경로를 검증하므로 이 준비 작업을 먼저 수행해야 합니다.

> **macOS:** 릴리스 애드온에는 현재 Apple 공증을 받지 않은 네이티브 라이브러리가 포함되어 있습니다. 브라우저에서 애드온 ZIP을 내려받아 직접 압축 해제하면 macOS에서 `libfennara.macos.editor`에 악성 코드가 없는지 확인할 수 없다고 알릴 수 있습니다. 이 알림을 피하려면 아래의 CLI 설치를 사용하세요. 이미 알림이 표시된다면 Godot을 닫고 직접 복사한 `addons/fennara/` 폴더를 제거한 다음 CLI로 Fennara를 설치하세요.

<a id="install-with-the-cli-recommended-on-macos"></a>
### CLI로 설치 (macOS 권장)

CLI는 같은 Fennara 애드온을 설치합니다. 위에서 설명한 알림의 원인이 되는 브라우저 및 Finder 격리 경로를 피하므로 macOS에서 권장되는 설치 방법입니다.

Windows에서 CLI를 설치합니다.

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS와 Linux에서는 다음을 실행합니다.

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

그다음 Godot 프로젝트에서 Fennara를 실행합니다.

```bash
cd path/to/your-godot-project
fennara install
```

문제 해결은 [설정](docs/i18n/ko/setup.md)을 참고하고, 전체 명령어는 [Fennara CLI](docs/i18n/ko/cli.md)를 참고하세요.

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## 제공업체 설정 또는 MCP 앱 연결

<a id="built-in-chat"></a>
### 내장 채팅

**Chat Settings > Chat**을 열고 **Open providers**를 선택한 다음 제공업체를 연결합니다. Fennara는 클라우드 제공업체에 사용자가 소유한 키를 사용합니다(BYOK). 로컬 Ollama 또는 LM Studio 서버도 사용할 수 있습니다. [지원되는 제공업체 목록](docs/i18n/ko/providers.md)을 참고하세요.

<a id="mcp-apps"></a>
### MCP 앱

**Chat Settings > MCP Apps**를 열고 앱을 찾은 다음 **Set Up**을 누릅니다.

터미널에서도 앱을 연결할 수 있습니다.

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Chat Settings에 MCP 앱이 없다면 [MCP 설정](docs/i18n/ko/mcp-setup.md)에서 전체 앱 목록과 수동 구성 방법을 확인하세요.

<a id="update"></a>
## 업데이트

Fennara 독에 **Update**가 표시되면 누르고 안내를 따릅니다.

> **Fennara v0.3.8 이하에서 업그레이드:** `fennara update`를 실행하기 전에 위의 플랫폼 설치 명령으로 CLI를 한 번 다시 설치하세요. 해당 CLI 버전은 사용이 중단된 릴리스 태그를 조회하므로 현재 릴리스를 찾을 수 없습니다. CLI를 다시 설치하면 이후 업데이트가 GitHub의 Latest Release 엔드포인트를 사용하며, 기존 프로젝트 애드온이나 설정은 제거하지 않습니다.

> **Fennara v0.3.11에서 업그레이드하는 macOS 사용자:** 업데이트하기 전에 위의 macOS 설치 명령으로 CLI를 한 번 다시 설치하세요. v0.3.11 CLI는 자체 업데이트 단계에 도달하기 전에 기존 macOS 프레임워크 번들을 거부합니다. 다시 설치하면 CLI만 교체되며 프로젝트 애드온과 설정은 제거되지 않습니다.

터미널에서 업데이트하려면 Godot을 닫고 다음을 실행합니다.

```bash
cd path/to/your-godot-project
fennara update
```

복구와 진단 방법은 [Fennara 업데이트](docs/i18n/ko/setup.md#fennara-업데이트)를 참고하세요.

<a id="tools"></a>
## 도구

Fennara는 소수의 Godot 인식 도구를 제공합니다.

- 프로젝트 파일을 작성하거나 업데이트하고 진단 결과 반환
- 일회성 씬 편집 스크립트 실행
- 씬 트리, 노드, 리소스, Godot 클래스 검사
- 씬 검증
- 스크린샷 캡처
- 런타임 세션 시작 및 런타임 로그 읽기
- 실행 중인 씬을 대상으로 작은 런타임 스크립트 실행

목표는 에이전트의 일반 파일 도구를 대체하는 것이 아닙니다. Fennara는 부족했던 Godot 피드백 루프를 제공합니다.

<a id="privacy"></a>
## 개인정보 보호

Fennara는 Godot이 연결된 뒤 UTC 기준 하루에 최대 한 번 익명 활성 설치 이벤트를 전송합니다. 이벤트에는 무작위 설치 UUID, Fennara 및 Godot 버전, 운영체제, CPU 아키텍처가 포함됩니다. 프로젝트 데이터, 경로, 프롬프트, 도구 활동, 로그, 스크린샷 또는 계정 정보는 포함되지 않습니다.

**Chat Settings > Chat > Anonymous telemetry**, `FENNARA_DISABLE_TELEMETRY=true` 또는 `DO_NOT_TRACK=1`로 텔레메트리를 비활성화할 수 있습니다. 전체 페이로드, 저장, 전송, 비활성화 계약은 [익명 텔레메트리](docs/i18n/ko/telemetry.md)를 참고하세요.

<a id="demos"></a>
## 데모

Fennara 실습 영상을 시청하세요.

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

더 많은 동영상:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

Fennara 채널의 더 많은 영상은 [데모](docs/i18n/ko/demos.md)를 참고하세요.

<a id="star-history"></a>
## Star 기록
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="Star History Chart" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## 문서

| 시작할 문서 | 필요한 내용 |
| --- | --- |
| [문서 홈](docs/i18n/ko/README.md) | 모든 가이드 및 참고 페이지 |
| [설정](docs/i18n/ko/setup.md) | 설치, 업데이트 및 문제 해결 |
| [채팅 제공업체](docs/i18n/ko/providers.md) | 내장 채팅 모델 및 키 |
| [MCP 설정](docs/i18n/ko/mcp-setup.md) | Codex, Claude, Cursor 및 기타 MCP 앱 |
| [도구](docs/i18n/ko/tools.md) | 에이전트가 사용할 수 있는 Godot 피드백 |
| [익명 텔레메트리](docs/i18n/ko/telemetry.md) | 수집 데이터, 전송 동작 및 비활성화 제어 |
| [기여](docs/i18n/ko/CONTRIBUTING.md) | 개발 및 풀 리퀘스트 지침 |

<a id="community"></a>
## 커뮤니티

질문, 설정 도움말, 초기 피드백은 Discord에서 환영합니다.

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## 라이선스

[LICENSE.md](LICENSE.md)를 참고하세요.
