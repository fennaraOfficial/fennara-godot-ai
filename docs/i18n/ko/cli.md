<!-- fennara-i18n: locale=ko source=docs/cli.md sha256=561efbe26f75accb949050e7af9d06ae607040b86d061c04c4dcf8732ba61135 -->
<a id="fennara-cli"></a>
# Fennara CLI

<!-- fennara-doc-nav:start -->
[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · [Español](../es/cli.md) · [Português do Brasil](../pt-BR/cli.md) · [日本語](../ja/cli.md) · **한국어** · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · [Deutsch](../de/cli.md) · [Türkçe](../tr/cli.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../cli.md)
<!-- fennara-doc-nav:end -->

터미널을 선호하거나, 진단 또는 복구가 필요하거나, 정확한 버전으로 자동 설치하려는 경우 CLI를 사용하세요.

> [!TIP]
> macOS에서는 CLI가 권장 설치 방법입니다. 브라우저로 내려받은 애드온 ZIP을 직접 압축 해제하여 네이티브 라이브러리가 Finder 격리를 상속할 때 발생할 수 있는 macOS 보안 알림을 피합니다.

<a id="common-flow"></a>
## 일반적인 흐름

```bash
cd path/to/your-godot-project
fennara install
```

로컬 설치를 검사하거나 복구해야 할 때 `fennara doctor`를 사용하세요.

일반적인 Godot 흐름은 [설정](setup.md)을 사용하고, 이 페이지는 터미널 명령 참고로 유지하세요.

<a id="install-the-cli"></a>
## CLI 설치

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS 및 Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

직접 압축 해제한 macOS 애드온이 이미 `libfennara.macos.editor` 알림을 일으킨다면 Godot을 닫고 직접 복사한 `addons/fennara/` 폴더를 제거한 뒤 `fennara install`을 실행하세요. 그렇지 않으면 CLI가 기존의 완전한 애드온을 보존합니다.

`fennara`를 바로 사용할 수 없다면 새 터미널을 열고 설치를 확인하세요.

```bash
fennara --version
fennara doctor
```

CLI는 사용자별로 설치됩니다. 프로젝트 애드온은 각 Godot 프로젝트 안에 있고, 공유 런처, 버전별 런타임, 작업 기록, 로그, Linux CEF는 Fennara 앱 데이터에 있습니다.

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## 명령 요약

| 명령 | 목적 |
| --- | --- |
| `fennara install` | 프로젝트 애드온과 일치하는 로컬 구성 요소 설치 또는 기존 애드온 채택 |
| `fennara update` | 프로젝트와 로컬 구성 요소 업데이트 |
| `fennara doctor` | 로컬 설치 검사 또는 복구 |
| `fennara diagnostics` | 민감한 정보가 제거된 작업 보고서 표시 |
| `fennara mcp-setup` | 외부 MCP 앱 연결 |
| `fennara prepare-export` | 애드온 없는 CI 내보내기 전에 Fennara의 autoload 제거 |
| `fennara recover` | 중단된 네이티브 업데이트 복원 |
| `fennara self-update` | 설치된 CLI만 업데이트 |

설치된 명령 요약은 `fennara --help`를 실행하세요. 지원되는 MCP 앱 대상은 `fennara mcp-setup --help`를 사용하세요.

<a id="install-a-project"></a>
## 프로젝트 설치

`project.godot`이 있는 폴더 안에서 실행합니다.

```bash
fennara install
```

프로젝트를 명시할 수도 있습니다.

```bash
fennara install --project path/to/project
```

`--version`이 없으면 CLI는 현재 릴리스 매니페스트를 선택합니다. 재현성이 중요할 때는 정확한 릴리스를 사용하세요.

```bash
fennara install --project path/to/project --version <version>
```

설치에는 두 가지 안전한 경로가 있습니다.

- 완전한 애드온이 없으면 CLI가 선택한 릴리스를 내려받아 검증하고, `addons/fennara`와 일치하는 로컬 구성 요소를 설치하며, Fennara 프로젝트 지침을 작성합니다.
- 완전한 애드온이 이미 있으면 CLI가 해당 `VERSION`을 읽고 현재 플랫폼 라이브러리를 검증한 뒤 정확히 같은 버전의 CLI 관리 구성 요소를 설치합니다. 프로젝트 애드온은 바꾸지 않습니다. 명시적인 `--version`은 기존 애드온과 일치해야 합니다.

<a id="prepare-an-addon-free-ci-export"></a>
## 애드온 없는 CI 내보내기 준비

CI 체크아웃에서 `addons/fennara/`를 제외한다면 Godot을 시작하기 전에 Fennara의 영구 런타임 autoload를 제거하세요.

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

이 명령은 `project.godot`의 `_fennara_game_capture` 항목만 편집합니다. 다른 autoload와 설정은 보존하며 다시 실행해도 안전합니다. 프로젝트를 시작할 때 Godot은 에디터 또는 내보내기 플러그인이 실행되기 전에 autoload 경로를 검증하므로 이 단계는 Godot보다 먼저 실행해야 합니다. 대신 Godot을 시작하기 전에 CI에 Fennara 애드온을 설치할 수도 있습니다.

<a id="update-a-project"></a>
## 프로젝트 업데이트

일반적인 터미널 업데이트에서는 해당 프로젝트의 Godot을 닫고 다음을 실행합니다.

```bash
fennara update --project path/to/project
```

`--version`이 없으면 CLI가 설치된 애드온 식별자를 읽습니다. 안정 애드온은 GitHub의 Latest Release를 해석하고 스테이징 애드온은 자체 `pr-<number>` 채널만 해석합니다. 선택자는 CLI 자체 교체를 거쳐서도 즉시 정확한 버전 하나로 고정됩니다. 그다음 CLI는 릴리스 자산을 검증하고, 애드온과 버전별 로컬 구성 요소를 갱신하고, 프로젝트 지침을 업데이트하고, 플랫폼 웹뷰 필수 구성 요소를 검사합니다. 정확한 릴리스를 명시하려면 `--version <version>`을 사용하세요.

`--no-self-update`는 제어된 자동화 또는 CLI가 이미 교체된 뒤 이어서 실행할 때 사용합니다. 릴리스의 최소 CLI 요구 사항을 우회하는 데 사용하지 마세요.

> [!IMPORTANT]
> Fennara v0.3.8 이하에서 업그레이드한다면 `fennara update`를 실행하기 전에 [설정](setup.md#터미널에서-설치-macos-권장)의 플랫폼 설치 명령으로 CLI를 한 번 다시 설치하세요. 해당 CLI는 사용이 중단된 릴리스 태그를 조회하므로 현재 릴리스를 찾을 수 없습니다. CLI 재설치는 프로젝트 애드온이나 설정을 제거하지 않습니다.

> [!IMPORTANT]
> macOS에서 Fennara v0.3.11로부터 업그레이드한다면 CLI를 한 번 다시 설치하세요. 해당 CLI는 자체 업데이트에 도달하기 전에 기존 프레임워크 번들을 거부합니다. 재설치는 CLI만 교체하고 프로젝트 애드온과 설정을 보존합니다.

<a id="prepare-while-godot-is-open"></a>
### Godot이 열린 동안 준비

에디터 내 업데이트 버튼은 다음 스테이징 형식을 사용합니다.

```bash
fennara update --prepare --project path/to/project
```

준비 과정은 애드온을 내려받고 검증하여 내구성 있게 스테이징합니다. Godot을 닫거나, 사용 중인 애드온을 교체하거나, 활성 런타임 매니페스트를 전환하거나, 데몬을 다시 시작하지 않습니다. Godot 독은 작업 영수증을 살펴보고 분리된 닫기, 교체, 다시 열기, 검증 단계를 시작하기 전에 사용자에게 묻습니다. 독은 이미 발견한 정확한 버전을 전달하므로 포인터가 이동해도 진행 중인 업데이트가 바뀌지 않습니다.

Fennara는 활성 공유 런타임 버전 하나만 지원합니다. 다른 Fennara 사용 Godot 에디터가 공유 데몬에 계속 연결되어 있으면 활성화가 차단됩니다. 다른 에디터를 닫고 다시 시도하세요. 이전 로컬 버전과 런타임 포인터는 네트워크 없이도 복구할 수 있도록 유지됩니다.

`--prepare`는 Godot 통합용 저수준 기본 기능입니다. 터미널 사용자는 일반적으로 Godot을 닫고 `fennara update`를 사용합니다.

<a id="recover-an-interrupted-update"></a>
## 중단된 업데이트 복구

업데이트된 애드온이 복구 패널을 표시할 정도로 불러와지지 않으면 Godot을 닫고 다음을 실행하세요.

```bash
fennara recover --project path/to/project
```

CLI는 복구 가능한 상태의 작업만 복원합니다. 이전 애드온, 공유 런처, 활성 런타임 매니페스트를 복원한 뒤 기록된 Godot 실행 파일을 다시 열려고 시도합니다. 지원 담당자가 작업 ID를 제공한 경우 특정 트랜잭션을 선택하세요.

```bash
fennara recover --project path/to/project --operation <operation-id>
```

완료됨, 준비만 됨, 이미 롤백됨 상태의 작업은 거부됩니다.

<a id="inspect-health-and-failures"></a>
## 상태와 실패 검사

`doctor`는 감지된 플랫폼, 앱 데이터 구조, 활성 버전, 런처, 런타임, 데몬 상태, 웹뷰 필수 구성 요소를 보고합니다.

```bash
fennara doctor
```

실행 중인 데몬 또는 MCP 런타임이 `current.json`보다 오래되었다고 보고하면 Godot 또는 해당 MCP 앱을 다시 시작하여 선택한 런타임을 시작하게 하세요.

누락된 기본 앱 데이터 디렉터리를 다시 만들려면 `--repair`를 사용하세요. Linux에서는 오래된 CEF 프로세스 프로필을 정리하고 완전한 관리 런타임이 이미 설치되어 있으면 현재 런타임 마커도 복구합니다.

```bash
fennara doctor --repair
```

설치, 업데이트, 복구, 자체 업데이트 작업은 내구성 있는 상태와 이벤트를 기록합니다. 가장 최근의 민감한 정보가 제거된 보고서를 표시하려면:

```bash
fennara diagnostics
```

이전 작업 또는 기계 판독 가능 출력을 보려면:

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

보고서에는 안정적인 오류 코드, 단계, 구성 요소 버전, 선택한 자산 이름, 해시 검증 결과가 들어 있습니다. 프로젝트, 홈, Fennara 앱 데이터 경로, 자격 증명, bearer 토큰, URL 쿼리는 제거됩니다. 채팅 메시지, 제공업체 키 또는 프로젝트 파일 내용은 포함되지 않습니다.

<a id="configure-an-external-mcp-app"></a>
## 외부 MCP 앱 구성

Godot 채팅 독은 이 명령을 **Chat Settings > MCP Apps** 아래에 제공합니다. **Set Up** 버튼은 로컬 데몬에 설치된 CLI를 호출하도록 요청하므로 독과 터미널 워크플로가 같은 구성 및 백업 구현을 사용합니다.

지원 대상을 선택하려면 `fennara mcp-setup --help`를 실행하세요. 구성을 바꾼 뒤 MCP 앱을 다시 시작하세요. 이 명령은 외부 앱을 Fennara MCP 서버에 연결하며 내장 Godot 채팅 독에서 사용하는 모델 제공업체를 선택하지 않습니다. 대상 목록, 구성 위치, 수동 구성 예제는 [MCP 설정](mcp-setup.md)에 있습니다.

<a id="update-only-the-cli"></a>
## CLI만 업데이트

일반 프로젝트 업데이트는 CLI 자체 업데이트를 자동으로 처리합니다. 설치된 CLI만 업데이트하려면:

```bash
fennara self-update
fennara self-update --version <version>
```

`--version`이 없으면 자체 업데이트가 활성 설치 트랙을 보존합니다. 안정 트랙은 GitHub의 Latest Release를 사용하고 스테이징 트랙은 기록된 PR 채널만 사용합니다.

스테이징은 자동으로 안정 트랙으로 넘어가지 않습니다. 스테이징을 의도적으로 떠나려면 Godot을 닫고 `fennara update --version <stable-version> --project <path>`를 실행하세요. 공유 활성 버전이 바뀌기 전에 정확한 안정 릴리스를 검증합니다.

지원 담당자가 요청하거나 프로젝트 업데이트에서 설치된 CLI가 너무 오래되어 안전하게 계속할 수 없다고 보고할 때 사용하세요.

<a id="automation-guidance"></a>
## 자동화 지침

- 현재 디렉터리에 의존하지 말고 `--project`를 전달하세요.
- 빌드를 재현할 수 있어야 한다면 `--version`을 고정하세요.
- 실패 시 출력된 작업 ID와 로그 경로를 보존하세요.
- 구조화된 보고에는 `fennara diagnostics --operation <id> --json`을 사용하세요.
- `current.json`, 버전 디렉터리, 업데이트 영수증 또는 스테이징된 애드온 폴더를 직접 편집하지 마세요.
- 프로젝트가 Godot에서 열려 있는 동안 일반 애드온 교체 업데이트를 실행하지 마세요. 에디터 내 업데이트 흐름을 사용하거나 먼저 Godot을 닫으세요.
