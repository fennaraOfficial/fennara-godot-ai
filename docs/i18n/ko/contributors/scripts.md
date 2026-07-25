<!-- fennara-i18n: locale=ko source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# 스크립트

<!-- fennara-doc-nav:start -->
[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · [Español](../../es/contributors/scripts.md) · [Português do Brasil](../../pt-BR/contributors/scripts.md) · [日本語](../../ja/contributors/scripts.md) · **한국어** · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · [Deutsch](../../de/contributors/scripts.md) · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

이 디렉터리에는 로컬 개발, 패키지 미리보기, 릴리스 워크플로가 공유하는 저장소 자동화가 있습니다.

도움말에 다르게 명시되어 있지 않다면 스크립트는 작고 결정론적이며 저장소 루트에서 안전하게 실행할 수 있어야 합니다. 저장소 밖에 사용자별 상태를 쓰면 안 됩니다.

<a id="version-scripts"></a>
## 버전 스크립트

- `set-version.mjs`: 저장소 `VERSION`, 애드온 `VERSION`, 로컬 Rust 워크스페이스 메타데이터, 잠금 파일 패키지 버전, C++ 플러그인 버전 상수를 업데이트합니다.
- `check-version.mjs`: 해당 버전 파일이 계속 동기화되어 있는지 확인합니다.

CI 및 릴리스 패키징 전에 `check-version.mjs`를 실행하세요. Fennara 버전을 의도적으로 변경할 때만 `set-version.mjs`를 사용하세요.

<a id="packaging-scripts"></a>
## 패키징 스크립트

- `package-preview.mjs`: 커밋된 애드온 페이로드를 동기화한 뒤 GDExtension과 로컬 Rust 바이너리가 이미 빌드된 상태에서 플랫폼별 미리보기 아카이브를 조립합니다.
- `package-addon-all.mjs`: 플랫폼별 애드온 부분을 최종 전 플랫폼 애드온 아카이브로 결합합니다.
- `release-policy.mjs`: 각 릴리스 트랙에서 호환되는 최소 게시 CLI를 정의합니다.
- `write-release-manifest.mjs`: 릴리스 자산에서 `fennara-release-manifest-v<version>.json`을 작성하고 참조된 모든 SHA-256을 검증합니다.

두 스크립트는 `.package-preview/`를 임시 스테이징에 사용하고 ZIP 출력을 저장소 루트 `dist/` 폴더 아래에 작성합니다. 이러한 출력은 무시되며 커밋하면 안 됩니다.

패키징 스크립트는 애드온 페이로드를 작게 유지해야 합니다. 특히 `libcef.so`, `fennara_cef_helper` 같은 Linux CEF 런타임 파일을 `fennara-addon-*` 안에 묶으면 안 됩니다. CEF는 사용자 공유 Fennara 앱 데이터 디렉터리에 한 번 설치됩니다.

<a id="staging-release-scripts"></a>
## 스테이징 릴리스 스크립트

- `write-staging-candidate.mjs`: 하나의 풀 리퀘스트와 고정된 소스 커밋에 대한 정확한 프리릴리스 식별자를 만듭니다.
- `validate-staging-build.mjs`: 게시 전에 애드온 부분, 플랫폼 아카이브, 조립된 애드온, 릴리스 매니페스트, Linux CEF를 검사합니다.
- `smoke-public-release.mjs`: 인증 없는 브라우저 URL로 게시된 모든 후보를 내려받고 신뢰된 자산 및 매니페스트 해시를 확인한 뒤 채널을 전진시킵니다.
- `write-staging-pointer.mjs`: 정확한 릴리스 매니페스트를 해시한 뒤 작은 PR별 포인터를 작성합니다.
- `check-staging-channel-advance.mjs`: 채널의 역방향 또는 충돌 이동을 거부합니다.
- `validate-staging-publish-bundle.mjs`: 후보 코드를 실행하지 않고 최종 아티팩트 번들을 다시 검증합니다.
- `verify-published-assets.mjs`: 예상 GitHub Release 자산과 내려받은 자산의 이름 및 SHA-256을 비교합니다.

이 스크립트는 `.github/workflows/staging-release.yml`을 지원합니다. 후보 빌드 작업은 릴리스 자격 증명 없이 실행됩니다. 신뢰된 최종 작업만 게시할 수 있으며, 정확한 릴리스를 내려받아 검증한 뒤 PR별 Git ref를 전진시킵니다.

<a id="linux-cef-scripts"></a>
## Linux CEF 스크립트

- `prepare-linux-cef-sdk.mjs`: Linux CEF 브리지를 빌드하는 데 사용하는 고정된 공식 Linux x64 CEF SDK를 내려받고 압축 해제합니다.
- `prepare-linux-cef-runtime.mjs`: 별도 Linux CEF 런타임 ZIP을 스테이징하고 필수 파일을 검증하며 Linux에서 스테이징된 ELF 바이너리를 strip하고, 릴리스 패키징용 생성 `local/webview-runtimes/linux-cef.json` 매니페스트를 작성할 수 있습니다.
- `check-linux-cef-runtime-release.mjs`: 릴리스 자산에 활성화된 매니페스트가 지정한 CEF 런타임 ZIP이 있고 SHA-256이 일치하는지 검증합니다.
- `cef/linux/fennara_cef_helper.cpp`: CEF SDK에서 런타임 헬퍼를 빌드할 때 사용하는 작은 CEF 헬퍼 프로세스 소스입니다.

CEF 스크립트는 복사된 스테이징 파일에서만 작동합니다. 내려받은 또는 원본 CEF SDK 트리를 변경하면 안 됩니다.

<a id="development-tests"></a>
## 개발 테스트

- `test-run-scene-edit-script-inspect.mjs`: `temp/` 아래에 무시되는 Godot 스모크 프로젝트를 만들고, 빌드된 에디터 GDExtension을 대상으로 가져온 `PackedScene` 검사, 읽기 전용 컨텍스트 가드, 소스 누락 실패, 저장하지 않는 동작을 검증합니다.

<a id="documentation-localization"></a>
## 문서 현지화

- `sync-doc-navigation.mjs`: 산문을 번역하지 않고 소스 해시, 안정적인 앵커, 작은 동일 페이지 언어 선택기를 추가합니다.
- `check-doc-i18n.mjs`: 완전한 로케일 범위, 소스 최신성, 탐색, 앵커, Markdown 구조, 보호된 코드, URL, 링크를 검증합니다.
- `doc-i18n-lib.mjs`: 공유 로케일 매니페스트, 소스 정규화, 탐색 렌더링, 구조 헬퍼를 담당합니다.

다음을 실행하세요.

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

로케일과 문서 집합은 `docs/i18n/languages.json`에 선언됩니다. 영어가 정본입니다. 번역 산문은 이 스크립트가 생성하는 것이 아니라 영문 원본을 읽고 작성해야 합니다.

일반 동기화는 탐색과 안정적인 앵커를 업데이트하지만 기존 소스 해시는 보존합니다. 변경된 영문 페이지의 번역 아홉 개를 모두 직접 업데이트한 뒤 해당 소스만 의도적으로 갱신하세요.

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

검토한 소스가 여러 개라면 이 옵션을 반복해서 지정할 수 있습니다. 번역 산문을 업데이트하지 않은 소스를 승인하지 마세요. CI는 전체 번역 검증기보다 먼저 `sync-doc-navigation.mjs --check`를 실행합니다.

<a id="ui-sync"></a>
## UI 동기화

- `sync-chat-ui.mjs`: `ui/chat/`을 `godot_demo/addons/fennara/dist/`로 복사합니다.

릴리스 애드온 ZIP에 빌드된 채팅 웹뷰가 들어 있어야 하므로 `godot_demo/addons/fennara/dist/`는 의도적으로 커밋됩니다. `ui/chat/`에서 변경하고 동기화 스크립트를 실행한 다음 소스와 생성 애드온 자산을 함께 커밋하세요.

<a id="runtime-sync"></a>
## 런타임 동기화

- `sync-runtime.mjs`: `runtime/`을 `godot_demo/addons/fennara/runtime/`으로 복사합니다.

릴리스 애드온 ZIP에 Godot 측 런타임 헬퍼 스크립트가 들어 있어야 하므로 `godot_demo/addons/fennara/runtime/`은 의도적으로 커밋됩니다. `runtime/`에서 변경하고 동기화 스크립트를 실행한 다음 소스와 생성 애드온 자산을 함께 커밋하세요.

<a id="guidance-sync"></a>
## 지침 동기화

- `sync-guidance.mjs`: `local/templates/`의 작은 지침과 필요할 때 불러오는 지식 페이지를 `godot_demo/addons/fennara/ai/`로 복사하여 `fennara install` 및 `fennara update`가 사용자 프로젝트에 작성하는 파일과 일치시킵니다.

데모 애드온이 설치된 애드온 구조를 반영하므로 `godot_demo/addons/fennara/ai/`는 의도적으로 커밋됩니다. `local/templates/`에서 변경하고 동기화 스크립트를 실행한 다음 소스와 생성 애드온 지침을 함께 커밋하세요.

<a id="boundaries"></a>
## 경계

- 스크립트는 `.package-preview/`와 루트 `dist/` 출력을 만들 수 있습니다.
- `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs`, `set-version.mjs`처럼 명시적으로 맡은 작업인 경우에만 커밋된 생성 페이로드를 업데이트할 수 있습니다.
- 스크립트는 Godot 에디터 캐시, 로컬 앱 데이터 설치, 내려받은 릴리스 아티팩트 또는 VM 테스트 출력을 추적되는 소스 폴더에 쓰면 안 됩니다.
