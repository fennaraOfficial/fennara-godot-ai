<!-- fennara-i18n: locale=ko source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Godot 애드온

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · **한국어** · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

이 디렉터리는 Godot이 프로젝트 안에서 기대하는 구조를 반영합니다.

```text
res://addons/
  fennara/
```

저장소 페이로드를 `godot_demo/addons/` 아래에 두면 패키징 및 로컬 테스트 스크립트가 경로 구조를 바꾸지 않고 애드온을 프로젝트에 복사할 수 있습니다.

<a id="current-addon"></a>
## 현재 애드온

`fennara/`는 설치 가능한 Fennara Godot AI 애드온입니다. 다음 항목을 포함합니다.

- 네이티브 확장의 Godot 진입점인 `fennara.gdextension`.
- `fennara-cpp/`에서 빌드한 플랫폼 에디터 바이너리가 있는 `bin/`.
- `ui/chat/`에서 동기화된 생성형 네이티브 채팅 웹뷰 자산이 있는 `dist/`.
- 저장소 루트 `runtime/` 소스에서 동기화된 Godot 측 헬퍼 스크립트가 있는 `runtime/`.
- 디버거 대상 애드온 자산이 있는 `debugger/`.
- 패키징된 애드온 버전 마커인 `VERSION`.

<a id="rules"></a>
## 규칙

- 애드온 기준 상대 경로를 안정적으로 유지하세요. 사용자 프로젝트는 이 폴더를 `res://addons/fennara/`로 받습니다.
- 패키지 미리보기 ZIP, 릴리스 ZIP, 내려받은 CEF 아카이브, 로그 또는 로컬 테스트 출력을 여기에 넣지 마세요.
- 생성된 웹뷰 파일을 `fennara/dist/`에서 직접 편집하지 마세요. 의도적으로 생성 출력을 패치한다면 소스 변경도 함께 동기화해야 합니다.
- `runtime/`도 업데이트하고 `node scripts/sync-runtime.mjs`를 실행하지 않은 채 `fennara/runtime/`의 동기화된 런타임 헬퍼 파일을 직접 편집하지 마세요.
- Godot 프로젝트로 복사할 항목인 경우에만 새 애드온 페이로드를 여기에 추가하세요.
