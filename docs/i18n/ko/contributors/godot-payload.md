<!-- fennara-i18n: locale=ko source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Godot 페이로드

<!-- fennara-doc-nav:start -->
[문서](../README.md)

🌐 **언어**

[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · **한국어** · [Русский](../../ru/contributors/godot-payload.md) · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

이 디렉터리는 사용자 프로젝트에 복사되고 릴리스 아카이브에 패키징되는 Godot 대상 애드온 페이로드의 소스 트리입니다.

```text
godot_demo/
  addons/
    fennara/
```

`godot_demo/addons/fennara/`는 일반 Godot 애드온 디렉터리로 설치 가능한 상태를 유지해야 합니다. 여기에 커밋되는 모든 항목은 사용자 프로젝트의 `res://addons/fennara/` 아래에 직접 들어갈 수 있어야 합니다.

<a id="what-belongs-here"></a>
## 이곳에 속하는 항목

- Godot이 불러오는 `addons/fennara/fennara.gdextension` 및 `.uid` 파일.
- 플랫폼 빌드가 생성하는 `addons/fennara/bin/` 에디터 GDExtension 바이너리.
- 네이티브 채팅 웹뷰에서 사용하는 생성된 `addons/fennara/dist/` 웹 채팅 자산.
- `runtime/`에서 동기화된 `addons/fennara/runtime/` Godot 측 런타임 헬퍼 스크립트.
- 패키징할 때 저장소 `VERSION`과 일치하는 `addons/fennara/VERSION`.

<a id="what-does-not-belong-here"></a>
## 이곳에 속하지 않는 항목

- `.godot/`, `.import/`, 로그, 임시 파일, 에디터 캐시 같은 로컬 Godot 사용자 상태.
- 워크플로의 루트 패키지 출력. 이러한 출력은 무시되는 `dist/` 또는 `.package-preview/` 같은 빌드 폴더에 있어야 합니다.
- Fennara 데몬 및 MCP 실행 파일이나 Linux CEF 런타임 같은 공유 로컬 런타임 페이로드. 이는 모든 Godot 프로젝트 애드온에 복사되지 않고 CLI가 사용자 Fennara 앱 데이터 디렉터리에 설치합니다.

<a id="generated-files"></a>
## 생성 파일

채팅 UI 소스는 `ui/chat/`에 있습니다. 변경한 뒤 다음을 실행하세요.

```powershell
node scripts\sync-chat-ui.mjs
```

이 명령은 빌드된 웹뷰 파일을 `godot_demo/addons/fennara/dist/`로 동기화합니다. 애드온 사용자가 Node.js나 프런트엔드 빌드 단계를 필요로 하지 않아야 하므로 이 디렉터리는 의도적으로 커밋됩니다.

런타임 헬퍼 소스는 `runtime/`에 있습니다. 변경한 뒤 다음을 실행하세요.

```powershell
node scripts\sync-runtime.mjs
```

이 명령은 Godot 측 런타임 헬퍼를 `godot_demo/addons/fennara/runtime/`으로 동기화합니다. 애드온 사용자가 릴리스 ZIP과 함께 이 스크립트를 받아야 하므로 이 디렉터리는 의도적으로 커밋됩니다.
