<!-- fennara-i18n: locale=ko source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# 수동 설치

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · **한국어** · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Godot 설정 흐름이나 `fennara install`을 사용하지 않고 Fennara를 직접 조립해야 하는 경우에만 이 페이지를 사용하세요.

> [!TIP]
> Windows와 Linux에서 대부분의 사용자는 프로젝트에 `addons/fennara`를 추가하고 Fennara 독을 연 다음 **Set Up Fennara**를 누르면 됩니다. macOS에서는 CLI를 사용하세요. [설정](setup.md)을 참고하세요.

> [!IMPORTANT]
> macOS에서는 애드온 ZIP 수동 설치를 권장하지 않습니다. 애드온에는 현재 Apple 공증을 받지 않은 네이티브 라이브러리가 들어 있으며, 브라우저 다운로드와 Finder 압축 해제로 인해 macOS에서 `libfennara.macos.editor`에 악성 코드가 없는지 확인할 수 없다고 알릴 수 있습니다. 이 알림을 피하려면 [CLI 설치](setup.md#터미널에서-설치-macos-권장)를 사용하세요. 이미 알림이 표시된다면 Godot을 닫고 직접 복사한 `addons/fennara/` 폴더를 제거한 뒤 `fennara install`을 실행하세요.

수동 설치는 CLI, 프로젝트 애드온, 공유 로컬 런타임 패키지, 선택 사항인 MCP 앱 구성의 네 부분으로 이루어집니다.

<a id="1-download-release-files"></a>
## 1. 릴리스 파일 내려받기

최신 GitHub 릴리스를 엽니다.

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

릴리스 매니페스트, 해당 플랫폼 파일, 공유 애드온 ZIP을 내려받으세요.

| 목적 | 자산 |
| --- | --- |
| 릴리스 계획 및 SHA-256 값 | `fennara-release-manifest-v<version>.json` |
| Windows x86_64 CLI | `fennara-cli-windows-x86_64-v<version>.zip` |
| Windows x86_64 로컬 런타임 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 CLI | `fennara-cli-linux-x86_64-v<version>.zip` |
| Linux x86_64 로컬 런타임 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Linux x86_64 임베디드 웹뷰 | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 CLI | `fennara-cli-macos-arm64-v<version>.zip` |
| macOS arm64 로컬 런타임 | `fennara-release-local-macos-arm64-v<version>.zip` |
| 버전별 전 플랫폼 애드온 | `fennara-release-addon-v<version>.zip` |

릴리스에는 문서 및 수동 다운로드용으로 다음 안정적인 이름의 애드온 별칭도 있습니다.

```text
fennara-addon-latest.zip
```

매니페스트에는 로컬 런타임, 애드온, 공유 런타임 자산의 예상 SHA-256이 기록됩니다. 수동 다운로드를 검사할 때 정본으로 사용하세요.

<a id="2-install-the-cli"></a>
## 2. CLI 설치

`fennara-cli` ZIP을 압축 해제합니다.

그 `bin` 디렉터리를 PATH에 추가하거나 `fennara` 바이너리를 기존 PATH 폴더 중 하나에 복사합니다.

확인:

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Godot 애드온 설치

`fennara-addon` ZIP을 압축 해제합니다.

다음을 복사합니다.

```text
addons/fennara
```

Godot 프로젝트에 복사하여 프로젝트에 다음 파일이 있게 합니다.

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. 로컬 런타임 패키지 설치

일반적으로 CLI가 이 작업을 관리합니다. `fennara install`을 사용하지 않을 때만 런타임을 수동으로 설정해야 합니다.

기본 Fennara 데이터 폴더:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

예상 구조:

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

Windows에서 바이너리는 `.exe`를 사용합니다.

`current.json`은 런처 바이너리가 활성 런타임 버전을 가리키게 합니다. 일반 `fennara install` 및 `fennara update` 명령은 이 파일을 자동으로 만듭니다.

Linux 임베디드 채팅은 공유 `webview/cef/linux-x64/<cef-version>/` 런타임 위치를 사용합니다. 일반 `fennara install` 및 `fennara update`는 릴리스 매니페스트와 자산에서 릴리스 관리 CEF 런타임을 자동으로 설치합니다. 모든 것을 직접 설치한다면 `fennara-webview-cef-linux-x64-<cef-version>.zip`을 해당 공유 런타임 위치에 압축 해제하고 일치하는 `webview/cef/linux-x64/current.json` 마커를 작성하세요. 페이로드를 Godot 프로젝트 애드온 밖에 두세요. `addons/fennara`에 `libcef.so` 또는 다른 CEF 런타임 파일이 있으면 안 됩니다.

이 CEF 페이로드는 Linux 임베디드 채팅 전용입니다. Chat Settings에서 **Open chat in my system browser next time**을 선택하면 임베디드 Godot 웹뷰 대신 로컬 데몬을 통해 시스템 브라우저에서 같은 내장 채팅을 표시할 수 있습니다.

최종 Linux CEF 구조:

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json`은 다음과 같아야 합니다.

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json`은 CEF 자산과 일치하는 릴리스 매니페스트여야 합니다. 예:

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

쓰기 가능한 브라우저 상태를 CEF 버전 디렉터리 안에 넣지 마세요. 일반 사용에서는 에디터별 프로필과 로그를 Fennara 앱 데이터의 캐시 및 로그 루트 아래에 작성하고 런타임 페이로드는 공유 읽기 전용으로 유지합니다.

<a id="5-configure-your-mcp-app"></a>
## 5. MCP 앱 구성

로컬 런타임 패키지를 설치한 뒤 MCP 앱을 구성하세요.

```bash
fennara mcp-setup --claude
```

다른 대상:

```bash
fennara mcp-setup --help
```

설정 뒤 MCP 앱을 다시 시작하세요.

앱이 목록에 없거나 설치 과정에서 MCP 구성을 직접 편집한다면 안정적인 런처 경로와 JSON/TOML 예제를 [MCP 설정](mcp-setup.md)에서 확인하세요.

이 과정은 외부 MCP 앱만 Fennara의 Godot 도구에 연결합니다. 내장 Fennara 채팅 독의 모델 제공업체를 구성하지는 않습니다. 내장 채팅을 원한다면 Godot 안에서 독을 구성하거나 [MCP 앱과 내장 채팅](chat-vs-mcp.md)을 참고하세요.

<a id="6-verify"></a>
## 6. 확인

Godot 프로젝트를 연 다음 MCP 앱에 다음과 같이 요청하세요.

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

경로가 올바르면 수동 설치가 작동하는 것입니다.

<a id="recommended-shortcut"></a>
## 권장 단축 경로

CLI를 직접 설치했더라도 애드온과 로컬 런타임 패키지 설치는 CLI에 맡길 수 있습니다.

```bash
cd path/to/your-godot-project
fennara install
```

CLI는 AI 코딩 에이전트를 위한 프로젝트 지침도 작성합니다.

```text
AGENTS.md
addons/fennara/ai/
```

AI 디렉터리에는 항상 읽는 작은 지침, 색인, 관련 있을 때만 불러오는 전문 페이지가 있습니다. 직접 복사한 애드온 ZIP에 이 패키징된 디렉터리가 포함될 수는 있지만 프로젝트 루트 `AGENTS.md`를 만들거나 갱신하지는 않습니다. Fennara가 완전한 프로젝트 지침을 관리하고 갱신하도록 하려면 `fennara install` 및 `fennara update`를 사용하세요.
