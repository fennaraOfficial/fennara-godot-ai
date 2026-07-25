<!-- fennara-i18n: locale=ko source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# 내장 채팅 슬래시 명령

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · **한국어** · [Русский](../ru/slash-commands.md) · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

슬래시 명령은 Godot 안의 Fennara 채팅 독에서 사용하는 바로 가기입니다. UI 명령이며 MCP 도구도, 모델에 보내는 프롬프트도 아닙니다.

작성기에 `/`를 입력하면 명령 팔레트가 열립니다.

| 명령 | 여는 항목 | 용도 |
| --- | --- | --- |
| `/provider` | 제공업체 선택기 | 클라우드 제공업체 연결, 로컬 제공업체 URL 구성 또는 제공업체 전환. |
| `/model` | 모델 선택기 | 현재 또는 연결된 제공업체의 모델 선택. |

<a id="how-they-behave"></a>
## 동작 방식

- 화살표 키로 명령 제안 사이를 이동합니다.
- Enter를 눌러 선택한 명령을 실행합니다.
- Escape를 눌러 명령 팔레트를 닫습니다.
- 채팅 메시지를 보내기 전에 작성기에서 슬래시 명령 텍스트가 제거됩니다.

<a id="common-flow"></a>
## 일반적인 흐름

내장 채팅 독에서:

```text
/provider
```

OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, 로컬 Ollama 또는 LM Studio를 연결합니다.

그다음:

```text
/model
```

독에서 사용할 모델을 선택합니다.

외부 MCP 앱에서는 이 슬래시 명령을 사용하지 마세요. `fennara mcp-setup`으로 앱을 구성한 다음 앱에 Fennara MCP 도구를 사용해 달라고 요청하세요.
