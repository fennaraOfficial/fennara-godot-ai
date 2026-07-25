<!-- fennara-i18n: locale=ko source=docs/release.md sha256=60b8cc51e0fcde9b4e18eadc230aaf1d8cc4fad2fe70cbf5190ab9123bac0073 -->
<a id="release-process"></a>
# 릴리스 절차

<!-- fennara-doc-nav:start -->
[English](../../release.md) · [简体中文](../zh-CN/release.md) · [Español](../es/release.md) · [Português do Brasil](../pt-BR/release.md) · [日本語](../ja/release.md) · **한국어** · [Русский](../ru/release.md) · [Français](../fr/release.md) · [Deutsch](../de/release.md) · [Türkçe](../tr/release.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../release.md)
<!-- fennara-doc-nav:end -->

릴리스는 수동으로 진행합니다. 풀 리퀘스트 워크플로에서 게시하지 마세요.

> [!IMPORTANT]
> `main`에서 릴리스를 실행하고, `VERSION`과 워크플로 입력을 동일하게 유지하며,
> 릴리스에 더 높은 최소 CLI 버전이 필요한지 명시적으로 결정하세요.

<a id="release-at-a-glance"></a>
## 릴리스 한눈에 보기

| 단계 | 결과 |
| --- | --- |
| 버전 변경 준비 및 병합 | 저장소의 버전 출처가 일치함 |
| Package Preview 실행 | 게시하지 않고 릴리스 형태의 산출물을 빌드함 |
| 미리 보기 검사 | 아카이브, 매니페스트, 해시, Linux CEF 배치를 검증함 |
| `main`에서 Release 실행 | 태그와 GitHub Release를 게시함 |
| 설치 및 업데이트 스모크 테스트 | 공개 사용자 흐름을 검증함 |

<a id="versioning"></a>
## 버전 관리

`VERSION`이 정본입니다.

릴리스 도구는 SemVer 값을 허용합니다. 안정 릴리스에는 `X.Y.Z`를 사용합니다.
스테이징 후보에는 `1.2.3-pr.101.2`처럼 격리된 풀 리퀘스트 프리릴리스를
사용합니다. 여기서 `pr-101`은 스테이징 채널이고 `2`는 해당 채널의 후보
번호입니다.

저장소 버전을 올리려면 다음을 실행합니다.

```bash
node scripts/set-version.mjs X.Y.Z
```

이 스크립트는 다음을 업데이트합니다.

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- 플러그인 버전 상수
- `local/` 아래의 Rust 워크스페이스 패키지 버전
- `local/Cargo.lock`

애드온에는 `addons/fennara/release.json`도 포함됩니다. 안정 신원은 위의 일반
명령으로 자동 기록됩니다. 스테이징 빌드 워크스페이스는 명시적인 신원 입력을
사용합니다.

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

스테이징 버전, 채널, 소스 커밋, 정확한 릴리스 태그는 서로 일치해야 합니다.
이 신원이 없는 프리릴리스 애드온은 거부됩니다. `release.json` 도입 전에
만들어진 기존 안정 애드온은 계속 안정 트랙을 기본값으로 사용합니다.

버전 동기화를 검사합니다.

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. 릴리스 커밋 준비

1. 버전 스크립트를 실행합니다.
2. diff를 검토합니다.
3. 변경된 범위에 맞는 로컬 검사를 실행합니다.
4. 릴리스 준비 PR을 `main`에 병합합니다.

일반적인 검사:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

GDExtension 변경 사항이 있으면 가능할 때 애드온도 로컬에서 빌드합니다.

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Package Preview 실행

패키징이 변경되었거나 시험 실행이 필요할 때 게시 전에 사용하세요.

GitHub:

```text
Actions > Package Preview > Run workflow
```

워크플로는 Windows, Linux, macOS 패키지를 빌드하고 임시 산출물을
업로드합니다. 태그, GitHub Releases 또는 `latest`는 만들지 않습니다.

Package Preview는 병합 전에 릴리스 패키징을 실행해 볼 수 있을 만큼 Release의
비게시 부분을 밀접하게 재현합니다.

- 빌드가 필요 없는 채팅 UI와 런타임 도우미 소스를 애드온 페이로드에 동기화
- Linux CEF 런타임 zip 빌드
- 생성된 Linux CEF 런타임 매니페스트 기록
- 생성된 매니페스트를 플랫폼 패키지 빌드에 입력
- 모든 플랫폼용 애드온 아카이브 조립
- 로컬 및 애드온 패키지 이름을 매니페스트가 관리하는 릴리스 자산 이름으로 변경
- 생성된 매니페스트를 기준으로 Linux CEF 런타임 자산 검증
- `fennara-release-manifest-v<version>.json` 기록
- 릴리스 형태의 zip과 매니페스트가 들어 있는 하나의
  `fennara-package-preview-release-assets` 산출물 업로드

미리 보기 산출물은 게시 전에 zip 내용과 매니페스트 형태를 확인하는 데
유용합니다. 이는 Actions 산출물이며 공개 릴리스 자산이 아닙니다.

<a id="3-run-release"></a>
## 3. Release 실행

`main`에서 수동 릴리스 워크플로를 실행합니다.

```text
Actions > Release > Run workflow
```

입력:

```text
version: X.Y.Z
promote_latest: true
```

`version` 입력은 `VERSION`과 일치해야 합니다.

워크플로가 게시하는 항목:

- `v<version>`
- `promote_latest`가 true이면 `v<version>`을 GitHub Latest로 지정

릴리스 워크플로는 플랫폼 패키징 전에 Linux CEF 런타임을 준비합니다.
고정된 공식 CEF 139 Linux 최소 SDK를 다운로드하고, 별도의
`fennara-webview-cef-linux-x64-<cef-version>.zip`을 조립하고, 스테이징된
ELF 바이너리를 스트립하고, 활성화된
`local/webview-runtimes/linux-cef.json` 매니페스트를 생성하여 기록한 뒤,
그 매니페스트를 CLI 패키지에 입력합니다. 게시 작업은 릴리스 자산에 생성된
매니페스트가 지정한 정확한 CEF zip이 포함되어 있고 SHA-256이 일치하는지
검증합니다. 또한 `fennara-release-manifest-v<version>.json`을 기록하고,
참조된 모든 자산과 해시를 검증한 뒤, 릴리스와 함께 해당 매니페스트를
업로드합니다.

풀 리퀘스트 워크플로는 릴리스를 게시하지 않습니다. Package Preview
워크플로는 매니페스트와 Linux CEF 런타임 페이로드를 포함한 릴리스 형태의
테스트 산출물을 만들므로, 유지관리자는 병합 전에 패키징을 스모크 테스트할 수
있습니다. Package Preview는 사용자 대상 릴리스 채널이 아닙니다.

<a id="release-assets"></a>
## 릴리스 자산

각 릴리스에는 플랫폼별 CLI 및 로컬 런타임 패키지와 모든 플랫폼이 공유하는
애드온 패키지 하나가 포함되어야 합니다.

| 대상 | 자산 |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| 모든 플랫폼 | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

패키지 역할:

| 패턴 | 역할 |
| --- | --- |
| `fennara-cli-*` | 한 플랫폼용 `fennara` CLI만 포함한 설치 스크립트 페이로드 |
| `fennara-release-local-*` | 한 플랫폼용 MCP 및 데몬 실행기와 버전이 지정된 런타임 바이너리 |
| `fennara-release-addon-v*` | 릴리스 매니페스트를 통해 해석되는 버전 지정 모든 플랫폼용 애드온 |
| `fennara-addon-latest.zip` | 문서와 수동 다운로드용으로 안정적인 이름을 쓰는 모든 플랫폼 애드온 별칭 |
| `fennara-webview-cef-linux-x64-*` | Fennara 앱 데이터에 한 번 설치되는 Linux 전용 공유 CEF 런타임 |
| `fennara-release-manifest-v*` | 자산 이름, SHA-256 값, 설치 기본 요소, 공유 런타임을 포함한 설치 및 업데이트 계획 |

macOS 애드온 GDExtension은 현재 Apple 공증을 받지 않았습니다. 브라우저
다운로드와 수동 Finder 추출은 격리 메타데이터를 전파하고 macOS 검증 알림을
발생시킬 수 있습니다. 사용자용 설치 문서는 macOS에서 `fennara install`을
권장하고, 수동 ZIP의 제한을 설명하며, 영향을 받은 사용자에게 수동으로 복사한
애드온을 제거한 뒤 CLI를 통해 다시 설치하라고 안내해야 합니다. 릴리스 검증은
ZIP 생성만으로 macOS 서명 또는 공증이 완료된 것으로 간주하지 않습니다.

`fennara-release-local-*` 접두사는 오래된 CLI가 매니페스트 관리 패키지 경로를
조용히 우회하지 못하게 합니다.

<a id="release-manifest"></a>
## 릴리스 매니페스트

0.3.0부터 `fennara install`과 `fennara update`는 릴리스에 매니페스트가
게시되어 있으면 이를 우선 사용합니다. 매니페스트는 다음을 기록합니다.

- `schema_version`
- `version`
- `minimum_cli_version`
- 지원되는 설치 기본 요소
- SHA-256 해시가 있는 플랫폼별 CLI 및 로컬 런타임 자산
- SHA-256이 있는 공유 애드온 자산
- 플랫폼별 공유 런타임 자산, 현재는 Linux CEF

`scripts/release-policy.mjs`가 `minimum_cli_version`의 정본입니다.
매니페스트 기록기는 릴리스 신원을 검증한 뒤 정책을 선택하므로 Stable,
Package Preview, Staging이 서로 다른 값을 선택할 수 없습니다. 일반적인
패키지 배치 또는 자산 이름 변경은 외부 CLI를 바꾸지 말고 매니페스트 데이터로
처리해야 합니다. 릴리스에 더 새로운 업데이터 인계, 매니페스트 스키마, 설치
기본 요소, 자체 업데이트 동작 또는 이전에 게시된 CLI가 안전하게 수행할 수
없는 다른 CLI 기능이 필요하면 정책을 높이세요.

CLI가 너무 오래된 경우 `fennara update`는 매니페스트의 플랫폼별
`assets.cli` 항목을 사용해 설치된 CLI를 먼저 업데이트한 다음
`--no-self-update`로 패키지 업데이트를 재개해야 합니다. 해당 릴리스 또는
설치 위치에서 자체 업데이트를 사용할 수 없으면 패키지 설치 전에 실패하고
`install.sh` 또는 `install.ps1`을 다시 실행하라는 명확한 지침을 출력해야
합니다.

매니페스트 스키마 1에 추가된 선택적 릴리스 신원에는 최소 CLI 버전 상향이
필요하지 않습니다. 이전 스키마 1 클라이언트는 알 수 없는 필드를 무시하고,
스테이징 인식 클라이언트는 신원이 있을 때 이를 검증합니다. 채널 인식 활성화
또는 업데이터 인계에 의존하는 향후 릴리스는 게시 전에 최소 CLI를 다시
검토해야 합니다.

<a id="staging-identity-and-discovery-contract"></a>
## 스테이징 신원 및 검색 계약

스테이징 채널은 풀 리퀘스트별로 격리됩니다.

| 값 | PR 101 예시 |
| --- | --- |
| 채널 | `pr-101` |
| 후보 버전 | `1.2.3-pr.101.2` |
| 정확한 릴리스 | `v1.2.3-pr.101.2` |
| 채널 ref | `fennara-staging/pr-101` |
| 포인터 파일 | `fennara-staging-channel-pr-101.json` |

채널별 Git ref에는 정확한 버전 릴리스를 가리키는 작은 포인터 파일만
들어 있습니다. 릴리스 바이너리는 이동하는 채널 ref 아래에 두지 않습니다.
CLI는 내부 버전 요청 `channel:pr-101`으로 이 포인터를 해석한 뒤 정확한
버전만 계속 사용합니다.

따라서 PR 101과 PR 125는 서로 다른 릴리스 태그와 포인터 자산을 사용합니다.
한 채널을 업데이트해도 다른 채널의 테스터가 리디렉션되지 않습니다. 한 채널을
게시해도 안정 GitHub Latest 지정이나 다른 풀 리퀘스트의 채널은 절대
변경되지 않습니다.

<a id="staging-candidate-workflow"></a>
## 스테이징 후보 워크플로

수동 **Staging Release** 워크플로는 열려 있는 풀 리퀘스트의 현재 head에서
후보를 빌드합니다. `main`에서 실행하고 다음을 제공합니다.

| 입력 | 의미 |
| --- | --- |
| `pull_request` | 빌드할 열려 있는 풀 리퀘스트 |
| `base_version` | `X.Y.Z` 형식의 계획된 안정 버전 |
| `candidate` | 이 풀 리퀘스트에서 증가하는 후보 번호 |
| `source_commit` | 여전히 풀 리퀘스트 head여야 하는 선택적 전체 SHA |
| `publish` | 산출물 전용 검증에서는 끄고, 후보 게시에는 켬 |

워크플로는 플랫폼 빌드 전에 풀 리퀘스트 head SHA를 고정합니다. Windows,
Linux, macOS 작업은 읽기 전용 권한, 지속되는 Git 자격 증명 없음, 릴리스
자격 증명 없음, 공유 의존성 캐시를 저장할 능력 없음의 조건으로 정확한 해당
커밋을 체크아웃합니다. 신뢰할 수 있는 기본 브랜치 워크플로가 기록한 호환
SCons/godot-cpp 및 Cargo 캐시는 복원할 수 있습니다. 스테이징은 복원 전용
캐시 액션을 사용하므로 후보 코드는 신뢰할 수 있는 빌드 출력을 사용할 수
있지만 이후 실행의 캐시를 교체하거나 오염시킬 수 없습니다. 후보 코드는 빌드
산출물을 만들 수 있지만 GitHub Release를 게시할 수 없습니다.

그런 다음 신뢰할 수 있는 저장소 스크립트가 후보 신원, 정확한 아카이브
인벤토리, 애드온 내용, 플랫폼 패키지 배치, 릴리스 매니페스트, 모든 SHA-256
값을 검증합니다. `publish`를 명시적으로 선택하지 않으면 게시는 비활성
상태로 유지됩니다.

게시가 활성화되면 신뢰할 수 있는 최종 작업이 다음을 수행합니다.

1. 후보 산출물을 데이터로 다시 검증합니다.
2. 초안을 만들고 모든 자산을 업로드한 뒤, GitHub Latest를 변경하지 않고
   정확한 `v<exact-version>` 프리릴리스로 게시합니다.
3. 게시된 자산을 다운로드하고 이름과 해시를 비교합니다.
4. 역행하거나 충돌하는 채널 변경을 거부합니다.
5. 조건부 GitHub Contents API 쓰기를 통해 작은
   `fennara-staging/pr-<number>` 포인터 ref를 마지막에 업데이트합니다.
6. 활성 포인터를 다운로드하고 정확한 내용을 검증합니다.

한 풀 리퀘스트의 실행은 직렬화됩니다. 서로 다른 풀 리퀘스트는 별도의 동시성
그룹, 릴리스 태그, 포인터 ref를 사용합니다. 같은 후보를 다시 시도하면 파일을
섞지 않고 기존의 정확한 릴리스를 검증합니다. 워크플로는 안정 GitHub Latest를
만들거나, 여기에 업로드하거나, 이를 승격하지 않습니다.

안정 게시에는 문자 그대로의 `latest` 태그 또는 릴리스를 사용하지 않습니다.
Release 워크플로는 정확한 `v<version>` 릴리스를 초안으로 만들고, 업로드된
자산을 바이트 단위로 검증하고, 변경 가능한 릴리스로 게시한 뒤,
`promote_latest`가 true이면 해당 정확한 릴리스를 GitHub Latest로
지정합니다. 설치 프로그램과 안정 CLI 검색은 GitHub의 Latest Release API
엔드포인트를 해석합니다.

저장소 릴리스 불변성이 비활성화된 동안 안정 및 스테이징 릴리스는 변경
가능합니다. 두 워크플로 모두 게시 완료 또는 스테이징 채널 전진 전에 릴리스
메타데이터와 다운로드한 자산 바이트를 검증합니다. 자산 게시에는 콘텐츠 쓰기
권한이 있는 작업 범위 `GITHUB_TOKEN`을 사용합니다.

현재 릴리스 정책은 안정 매니페스트에 CLI `0.4.1`, 스테이징 매니페스트에
CLI `0.3.8`을 요구합니다. 안정 검색은 폐기된 `latest` 태그를 더 이상
해석하지 않습니다. 안정 `0.4.1`에는 수정된 업데이트 검증, 버전 전환 사전
점검, Windows 작업 저널 처리 및 Linux CEF 런타임 마커 복구가 필요합니다. `0.4.1-pr.123.1` 같은
스테이징 후보는 SemVer에서 안정 `0.4.1`보다 낮게 비교되므로, 최초 실행
설정이 후보 CLI를 설치할 수 있도록 최소 버전이 후보 버전보다 낮아야 합니다.
매니페스트 스키마 호환성만을 근거로 어느 최소 버전도 변경하지 마세요.

공유 애드온 zip에는 `godot_demo/addons/fennara/fennara.gdextension`에서
참조하는 빌드된 모든 GDExtension 바이너리가 포함됩니다. Godot은 사용자
OS에 맞는 라이브러리를 로드하고 나머지는 무시합니다.

Linux CEF 웹뷰 런타임 페이로드는 애드온 아카이브와 분리됩니다. 릴리스
패키징은 활성화된 런타임 매니페스트를 생성하고 해당 데이터를
`fennara-release-manifest-v<version>.json`에 내장합니다. CLI는 일치하는
CEF 페이로드를 사용자의 Fennara 앱 데이터 디렉터리 아래에 한 번 설치합니다.

```text
webview/cef/linux-x64/<cef-version>/
```

`libcef.so`, CEF 도우미 실행 파일, CEF 리소스 또는 로케일 팩을
`fennara-addon-*` 안에 넣지 마세요. Package Preview는 테스트용으로 별도의
CEF 산출물을 빌드하고 Release가 사용하는 것과 같은 종류의 생성된 런타임
매니페스트를 기록하지만, 릴리스 게시는 사용자 대상 릴리스 자산의 유일한
출처로 유지됩니다.

Linux GDExtension 빌드에도 공식 CEF SDK 래퍼 소스가 필요하지만, 애드온에
CEF 런타임 파일은 필요하지 않습니다. CI는 다음을 실행합니다.

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

그리고 추출된 디렉터리를 `FENNARA_CEF_ROOT`로 SCons에 전달합니다. SCons는
`FENNARA_CEF_ROOT/libcef_dll/`을 사용하여 고정된 CEF 139 C++ 래퍼를
대상으로 작은 `libfennara_linux_cef_bridge.so` 애드온 라이브러리를
빌드합니다. 생성된 래퍼 소스가 런타임 CEF ABI와 일치해야 하므로 SDK
다운로드는 버전과 해시를 검사합니다. 브리지는 애드온과 함께 패키지되며,
`libcef.so`, 리소스, 로케일 팩, `fennara_cef_helper`는 별도의 공유 CEF
런타임에 남습니다.

애드온 아카이브 안에서 CEF 런타임 파일이 발견되면 패키지 스크립트가
실패합니다. 런타임 자산 이름은 다음과 같아야 합니다.

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

zip을 추출하면 다음 필수 파일이 루트에 있어야 합니다.

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

선택한 CEF 배포판에 `chrome-sandbox`, `libEGL.so`, `libGLESv2.so`,
`libvk_swiftshader.so`, `libvulkan.so.1`, `vk_swiftshader_icd.json`,
`snapshot_blob.bin`, 추가 `locales/*.pak` 같은 선택적 CEF 런타임 파일이
있으면 포함해야 합니다.

유지관리자가 선택한 CEF 바이너리 트리에서 런타임 zip을 수동으로 조립하려면:

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

Linux에서는 이 스크립트가 `fennara-cpp/vendor/cef/`의 공식 CEF 헤더를
대상으로 `scripts/cef/linux/fennara_cef_helper.cpp`에서
`fennara_cef_helper`를 빌드합니다. 다른 OS에서는 먼저 Linux에서 해당
도우미를 빌드하고 `--helper /path/to/fennara_cef_helper`를 전달하세요.
zip을 쓰기 전에 선택된 파일을 검사하려면 `--dry-run`을 사용하세요.

스크립트가 SHA-256을 출력하면 `local/webview-runtimes/linux-cef.json`을
업데이트합니다.

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

일반 릴리스에서는 워크플로가 `--write-manifest`를 사용하여 Linux CEF 런타임
매니페스트를 자동으로 기록한 다음, `scripts/write-release-manifest.mjs`가
런타임 필드를 `fennara-release-manifest-v<version>.json`으로 복사합니다.
수동 런타임 자산 경로 또는 기존 대체 동작을 의도적으로 디버깅하는 경우가
아니면 체크인된 자리표시자 매니페스트를 직접 활성화하지 마세요. 생성된
매니페스트 데이터가 누락된 자산이나 SHA-256이 일치하지 않는 자산을
가리키면 Release 워크플로와 Linux의 `fennara install` /
`fennara update`가 명확하게 실패합니다.

CLI는 Linux CEF 런타임 업데이트를 원자적으로 게시해야 합니다. 스테이징
디렉터리에 추출하고 검증하고, 필수 파일이 있는 경우에만 런타임 마커를 기록한
다음, 버전 디렉터리를 게시하고 임시 파일 이름 변경으로 `current.json`을
업데이트합니다. 설치된 `fennara-cef-runtime.json` 마커는
`"runtime": "cef"`로 네이티브 로더 계약을 식별해야 합니다. 설치와
업데이트는 CEF 페이로드를 다시 다운로드하지 않고 `"kind": "cef"`만
포함하는 일치하는 기존 마커를 복구합니다. 실행 중인 편집기는 이미 로드한
런타임을 계속 사용합니다.

CLI는 `local/templates/`에서 생성된 프로젝트 지침 템플릿을 내장합니다.
릴리스 패키징이 CLI를 빌드할 때 해당 템플릿은 나머지 CLI 코드와 함께
바이너리에 컴파일됩니다.

<a id="what-latest-means"></a>
## `latest`의 의미

GitHub의 Latest Release 포인터는 일반 설치 및 업데이트 흐름에서 사용하는
버전 지정 릴리스를 선택합니다. Fennara는 문자 그대로의 `latest` 태그를
만들거나 이동하지 않습니다.

- `install.ps1`과 `install.sh`는 기본적으로 최신 CLI 자산을 가져옵니다.
- `fennara update`는 기본적으로 GitHub의 Latest Release 엔드포인트를 통해 릴리스 매니페스트를 가져오고, 필요할 때 설치된 CLI를 자체 업데이트한 다음, 매니페스트에서 로컬, 애드온, 공유 런타임 자산을 해석합니다.
- 편집기 내 업데이트는 종료 전에 검증된 자산을 스테이징하고, 교체 전에 스테이징된 전체 애드온의 다이제스트를 다시 검사하며, 활성화 검증이 성공할 때까지 이전 애드온, 실행기, 런타임 매니페스트를 보관하고, 롤백 데이터를 삭제하기 전에 다시 열린 GDExtension 핸드셰이크를 요구합니다.
- `fennara install`은 기본적으로 GitHub의 Latest Release 엔드포인트를 통해 릴리스 매니페스트를 가져온 다음, 여기에서 로컬, 애드온, 공유 런타임 자산을 해석합니다.
- Godot 플러그인 업데이트 검사는 GitHub의 최신 릴리스와 비교합니다.

기본 사용자 설치가 되어서는 안 되는 버전을 게시할 때만
`promote_latest: false`를 사용하세요.

설치 프로그램과 릴리스 다운로드는 릴리스 메타데이터, 자산 다운로드, 추출,
설치, 검증 단계를 출력해야 합니다. 네트워크 가져오기에는 제한된 시간 초과를
사용하여 GitHub 또는 CDN 정지가 멈춘 것처럼 보이지 않고 진단과 함께
실패하도록 해야 합니다. Windows에서 `install.ps1`은 성공을 출력하기 전에
CLI 검증 종료 코드를 검사해야 합니다. 종료 코드 `-1073741515`
(`0xC0000135`)는 CLI 실행 파일은 기록되었지만 필수 DLL이 없어 Windows가
시작하지 못했다는 뜻입니다. 사용자에게 Microsoft Visual C++ Redistributable
2015-2022 x64를 설치한 다음 `fennara --version`, `fennara doctor`,
`fennara install`을 다시 실행하라고 안내하세요. 다운로드 URL:
`https://aka.ms/vs/17/release/vc_redist.x64.exe`.

<a id="smoke-test-after-release"></a>
## 릴리스 후 스모크 테스트

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

Godot 프로젝트:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

프로젝트에 다음이 생겼는지 확인합니다.

```text
AGENTS.md
addons/fennara/ai/
```

Godot에서 프로젝트를 연 다음 MCP 앱에 다음과 같이 요청합니다.

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

업데이트 테스트:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## 규칙

- Release 워크플로는 `main`에서만 실행합니다.
- 릴리스 버전 입력은 `VERSION`과 일치해야 합니다.
- 풀 리퀘스트 워크플로는 테스트 산출물을 빌드하고 업로드할 수 있지만 릴리스를 게시해서는 안 됩니다.
- 일반 사용자를 대상으로 하는 의도된 릴리스를 GitHub Latest로 유지하세요.
- 유지관리자가 손상된 릴리스를 교체하기로 의도적으로 결정하지 않는 한 게시된 릴리스 태그를 다시 쓰지 마세요.
