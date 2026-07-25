<!-- fennara-i18n: locale=ko source=docs/setup.md sha256=ab1b11ff7dd3472ab14185e920004b6504fa14eb1c29e7c7b1d7a322780af1dd -->
<a id="setup"></a>
# 설정

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · [Español](../es/setup.md) · [Português do Brasil](../pt-BR/setup.md) · [日本語](../ja/setup.md) · **한국어** · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · [Deutsch](../de/setup.md) · [Türkçe](../tr/setup.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../setup.md)
<!-- fennara-doc-nav:end -->

Fennara를 설치하고 채팅할 위치를 선택한 뒤 Godot 프로젝트를 연결합니다.

> [!TIP]
> 대부분의 사용자는 애드온을 추가하고 Fennara 독을 연 다음 **Set Up Fennara**만 누르면 됩니다. macOS에서는 직접 내려받은 애드온 ZIP 뒤에 발생할 수 있는 보안 알림을 피하려면 아래의 CLI 설치를 사용하세요.

<a id="before-you-start"></a>
## 시작하기 전에

| 요구 사항 | 필요한 경우 |
| --- | --- |
| Godot 4.5 이상 | 항상 |
| Windows x86_64, Linux x86_64 또는 macOS arm64 | 항상 |
| MCP 지원 AI 앱 | 외부 MCP 사용 시에만 |
| 클라우드 API 키, Ollama 또는 LM Studio | 내장 채팅 사용 시에만 |
| `dotnet`으로 사용할 수 있는 .NET SDK | C# 진단 및 런타임 사전 검사 시에만 |

<a id="install-from-godot"></a>
## Godot에서 설치

> [!IMPORTANT]
> macOS용 릴리스 애드온에는 현재 Apple 공증을 받지 않은 네이티브 라이브러리가 포함되어 있습니다. 브라우저로 애드온 ZIP을 내려받아 직접 압축 해제하면 macOS에서 `libfennara.macos.editor`에 악성 코드가 없는지 확인할 수 없다고 알릴 수 있습니다. 이 알림을 피하려면 [터미널에서 설치](#터미널에서-설치-macos-권장)를 사용하세요.

1. [최신 릴리스](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)에서 `fennara-addon-latest.zip`을 내려받고 `addons/fennara/`를 프로젝트에 복사합니다.
2. 프로젝트를 열고 Fennara 독을 선택합니다.
3. **Set Up Fennara**를 누릅니다.

Fennara는 일치하는 로컬 구성 요소를 설치하고 열린 프로젝트를 연결합니다. 이전 공유 데몬이 유휴 상태라면 설정 과정에서 일치하는 버전을 활성화하기 전에 이를 중지합니다. 버전을 전환하려면 연결된 프로젝트가 하나도 없어야 합니다. 설정 중인 프로젝트는 버전이 다른 동안 일반적으로 연결되지 않은 상태를 유지합니다. 설정이 연결된 프로젝트를 보고하면 Fennara가 활성화된 다른 모든 에디터를 닫고 다시 시도하세요. 현재 프로젝트의 오래된 연결이 남아 있으면 이 에디터를 닫았다가 다시 연 뒤 재시도하세요.
설정에 실패하면 독에 **Retry**, **Copy Report**, **Open Logs**가 표시됩니다. 복사된 보고서는 민감한 정보가 제거되어 있으며 API 키, 채팅 내용 또는 프로젝트 파일을 포함하지 않습니다.

> [!NOTE]
> 애드온은 프로젝트 안에 남습니다. CLI, 데몬, MCP 서버, 로그, 공유 브라우저 런타임은 프로젝트 밖의 Fennara 앱 데이터에 있습니다.

<a id="install-from-the-terminal-recommended-on-macos"></a>
## 터미널에서 설치 (macOS 권장)

CLI는 같은 애드온을 설치하며 macOS에서 권장되는 설치 방법입니다. 위에서 설명한 네이티브 라이브러리 알림을 일으키는 브라우저 및 Finder 격리 경로를 피합니다.

Windows에서 CLI를 설치합니다.

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS와 Linux에서는:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

그다음 프로젝트 안에서 Fennara를 실행합니다.

```bash
cd path/to/your-godot-project
fennara install
```

macOS에서 이미 애드온을 직접 압축 해제하여 알림이 표시된다면 Godot을 닫고 직접 복사한 `addons/fennara/` 폴더를 제거한 뒤 `fennara install`을 실행하세요. CLI는 이미 존재하는 완전한 애드온을 교체하지 않고 보존하기 때문에 이 단계가 중요합니다.

프로젝트에 완전한 Fennara 애드온이 이미 있으면 CLI는 이를 유지하고 일치하는 로컬 구성 요소를 설치합니다. 없으면 현재 릴리스 애드온도 설치합니다. 버전 고정과 자동화는 [CLI 설치 참고](cli.md#프로젝트-설치)를 참고하세요.

<a id="choose-how-you-use-fennara"></a>
## Fennara 사용 방식 선택

| 경로 | 모델 계정 | 설정 |
| --- | --- | --- |
| 내장 채팅 | Fennara Chat Settings에 연결된 제공업체 | [제공업체 연결](#내장-채팅-연결) |
| 외부 MCP 앱 | 앱 자체 모델 계정 또는 구독 | [MCP 앱 연결](#mcp-앱-연결) |
| 둘 다 | 각 경로가 자체 모델 설정 유지 | 두 섹션 모두 완료 |

<a id="connect-the-built-in-chat"></a>
### 내장 채팅 연결

1. **Chat Settings > Chat**을 엽니다.
2. **Open providers**를 선택합니다.
3. 사용자 키로 클라우드 제공업체를 연결하거나 로컬 Ollama 또는 LM Studio 서버를 연결합니다.
4. 모델을 선택합니다.

지원되는 제공업체, 키, 로컬 서버 URL, 모델 ID는 [내장 채팅 제공업체](providers.md)를 참고하세요. 작성기에서는 `/provider`와 `/model`을 사용해 같은 작업을 할 수 있습니다.

임베디드 채팅은 플랫폼 웹뷰를 사용합니다.

| 플랫폼 | 웹뷰 |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | 시스템 WKWebView/WebKit |
| Linux | Fennara 관리 공유 CEF 런타임 |

`fennara install`, `fennara update`, `fennara doctor`는 이러한 필수 구성 요소를 검사합니다. 선택 사항인 임베디드 채팅을 시작할 수 없어도 MCP 도구는 계속 작동합니다.

시스템 브라우저를 사용하려면 Chat Settings에서 **Open chat in my system browser next time**을 활성화하고 Godot을 다시 시작하세요. 내장 채팅이 표시되는 위치만 바뀌며 제공업체, 기록, 프로젝트 연결은 유지됩니다.

다음 내장 채팅 메시지에 코드를 첨부하려면 Godot 스크립트 에디터에서 코드를 선택하고 컨텍스트 메뉴를 연 다음 **Add to Chat**을 선택하세요.

<a id="connect-an-mcp-app"></a>
### MCP 앱 연결

**Chat Settings > MCP Apps**를 열고 앱을 찾은 다음 **Set Up**을 누릅니다. Fennara를 불러올 수 있도록 앱을 다시 시작하세요.

터미널에서도 앱을 연결할 수 있습니다.

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

앱이 목록에 없다면 모든 지원 대상과 수동 구성 형식은 [MCP 설정](mcp-setup.md)을 참고하세요.

외부 MCP 앱은 자체 모델 계정을 사용합니다. 내장 채팅은 Fennara Chat Settings에서 선택한 제공업체를 사용합니다. 차이점은 [MCP 앱과 내장 채팅](chat-vs-mcp.md)을 참고하세요.

<a id="verify-the-connection"></a>
## 연결 확인

Godot 프로젝트를 연 다음 MCP 앱에 다음과 같이 요청하세요.

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

잘못된 프로젝트를 보고하면 Fennara 독에서 올바른 MCP target을 선택하세요.

<a id="update-fennara"></a>
## Fennara 업데이트

독에 **Update**가 표시되면 누르고 안내를 따릅니다. Fennara는 Godot을 닫도록 요청하기 전에 업데이트를 내려받아 검증합니다. 설치 후 같은 프로젝트를 다시 열고 업데이트 검증이 끝날 때까지 이전의 정상 작동 버전을 보존합니다.

터미널에서 업데이트하려면 Godot을 닫고 다음을 실행하세요.

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Fennara v0.3.8 이하에서 업그레이드한다면 `fennara update`를 실행하기 전에 위의 플랫폼 설치 명령으로 CLI를 한 번 다시 설치하세요. 해당 CLI는 사용이 중단된 릴리스 태그를 조회하므로 현재 릴리스를 찾을 수 없습니다. CLI 재설치는 프로젝트 애드온이나 설정을 제거하지 않으면서 이후 업데이트를 GitHub의 Latest Release 엔드포인트로 전환합니다.

> [!IMPORTANT]
> macOS에서 Fennara v0.3.11로부터 업그레이드한다면 CLI를 한 번 다시 설치하세요. 해당 CLI는 자체 업데이트에 도달하기 전에 기존 프레임워크 번들을 거부합니다. 재설치는 CLI만 교체하고 프로젝트 애드온과 설정을 보존합니다.

검증에 실패하면 독에서 **Restore Previous Version**, **Open Logs** 또는 **Copy Report**를 사용하세요. 정확한 버전, 준비, 중단된 업데이트 복구는 [CLI 업데이트 참고](cli.md#프로젝트-업데이트)를 참고하세요.

<a id="troubleshooting"></a>
## 문제 해결

<a id="an-install-or-update-failed"></a>
### 설치 또는 업데이트 실패

독에서 민감한 정보가 제거된 보고서를 복사하거나 터미널에서 최신 보고서를 표시하세요.

```bash
fennara diagnostics
```

작업 ID, JSON 출력, 기록 필드, 제거 보장은 [CLI 진단](cli.md#상태와-실패-검사)을 참고하세요.

<a id="fennara-is-not-found"></a>
### `fennara`를 찾을 수 없음

새 터미널을 열고 다음을 실행하세요.

```bash
fennara doctor
```

명령을 계속 사용할 수 없다면 Fennara `bin` 디렉터리를 PATH에 추가하세요. [CLI 설치 페이지](cli.md#cli-설치)에 플랫폼 경로가 나와 있습니다.

<a id="windows-binaries-fail-before-starting"></a>
### Windows 바이너리가 시작 전에 실패함

Fennara 바이너리가 누락된 `VCRUNTIME` 또는 `MSVCP` DLL, 종료 코드 `-1073741515` 또는 `0xc0000135`를 보고하면 Microsoft Visual C++ Redistributable 2015-2022 x64를 설치하세요.

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

이 항목은 해당 Microsoft 런타임 DLL이 없는 Windows 컴퓨터에서만 필요합니다.

<a id="a-release-requires-a-newer-cli"></a>
### 릴리스에 더 새로운 CLI가 필요함

CLI 자체 업데이트로 필요한 버전을 설치할 수 없다면 [CLI 설치](cli.md#cli-설치)의 설치 스크립트를 다시 실행한 뒤 명령을 재시도하세요.

<a id="the-addon-is-not-visible-in-godot"></a>
### Godot에 애드온이 표시되지 않음

다음 파일이 있는지 확인한 뒤 프로젝트를 다시 여세요.

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status`가 잘못된 프로젝트를 표시함

원하는 프로젝트를 열고 Fennara 독의 MCP target 컨트롤로 선택하세요.

<a id="c-diagnostics-are-missing"></a>
### C# 진단이 없음

프로젝트에 명확한 `.csproj`, `.sln` 또는 `.slnx`가 하나 있는지 확인한 뒤 다음을 실행하세요.

```bash
dotnet --version
```

브라우저 런타임 구조, 수동 복구, 구현 세부 정보는 [아키텍처](architecture.md), [수동 설치](manual-install.md), [FAQ](faq.md)를 참고하세요.
