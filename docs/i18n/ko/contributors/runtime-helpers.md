<!-- fennara-i18n: locale=ko source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# 런타임 헬퍼

<!-- fennara-doc-nav:start -->
[문서](../README.md)

🌐 **언어**

[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · **한국어** · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

이 폴더는 `runtime_session`과 `runtime_script`에서 사용하는 Godot 측 런타임 헬퍼 스크립트의 소스입니다.

패키징된 애드온 사본은 다음 위치에 있습니다.

```text
godot_demo/addons/fennara/runtime/
```

이곳의 파일을 편집한 뒤 다음을 실행하세요.

```bash
node scripts/sync-runtime.mjs
```

설치된 Godot 프로젝트 안에서 런타임 스크립트는 계속 `res://addons/fennara/runtime/`에서 이 헬퍼를 불러옵니다. 헬퍼를 기본 기능 중심이며 프로젝트에 종속되지 않게 유지하세요. 입력, 대기, 노드 스냅샷, 캡처, 물리 쿼리, 씬 생명주기 지원은 적합합니다. 게임별 이동, 전투, 퀘스트, 인벤토리 또는 UI 흐름 가정은 적합하지 않습니다.

`image_sheet.gd`는 스크린샷 스크립트 퍼사드에서도 사용됩니다. 합성 결과를 결정론적으로 유지하고 씬, 애니메이션 또는 게임플레이 상태에 종속되지 않게 하세요.
