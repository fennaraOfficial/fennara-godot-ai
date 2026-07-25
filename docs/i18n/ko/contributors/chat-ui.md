<!-- fennara-i18n: locale=ko source=ui/chat/README.md sha256=7667d9eea6d28d72c97e48574ab99262c8250b1feccbdabbec7a242eb3ba7091 -->
<a id="fennara-chat-ui"></a>
# Fennara 채팅 UI

<!-- fennara-doc-nav:start -->
[문서](../README.md)

🌐 **언어**

[English](../../../../ui/chat/README.md) · [简体中文](../../zh-CN/contributors/chat-ui.md) · [Español](../../es/contributors/chat-ui.md) · [Português do Brasil](../../pt-BR/contributors/chat-ui.md) · [日本語](../../ja/contributors/chat-ui.md) · **한국어** · [Русский](../../ru/contributors/chat-ui.md) · [Français](../../fr/contributors/chat-ui.md) · [Deutsch](../../de/contributors/chat-ui.md) · [Türkçe](../../tr/contributors/chat-ui.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../../../ui/chat/README.md)
<!-- fennara-doc-nav:end -->

이 폴더에는 선택 사항인 에디터 내 채팅 화면의 소스가 있습니다.

첫 버전은 의도적으로 빌드 단계가 없습니다. 일반 HTML, CSS, JavaScript를 사용합니다. 웹뷰 호스트와 데몬 채팅 브리지가 안정되기 전에 프런트엔드 도구 체인을 추가하지 않아 OSS 저장소를 쉽게 살펴볼 수 있게 합니다.

패키징된 사본은 `godot_demo/addons/fennara/dist/`에 있습니다.

이 폴더를 편집한 뒤 다음을 실행하세요.

```bash
node scripts/sync-chat-ui.mjs
```

<a id="design-notes"></a>
## 설계 참고

- Godot 에디터 화면에 맞추세요. 조밀한 컨트롤, 차분한 대비, 작은 모서리 반경, 명확한 포커스 상태를 사용하고 마케팅식 히어로 표현은 피하세요.
- 로컬 Fennara 데몬 및 채팅 API만 사용하고 호스팅 서비스가 필요하지 않게 하세요.
- OpenRouter 지원은 Godot 프로젝트 밖의 로컬 저장소에 보관되는 사용자 제공 키를 사용해야 합니다.
- 모델이 연결되지 않아도 UI를 유용하게 유지하세요. 상태, 설정, 대화 내용, 작성기 상태가 계속 보여야 합니다.
