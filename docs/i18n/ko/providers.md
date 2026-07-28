<!-- fennara-i18n: locale=ko source=docs/providers.md sha256=93027562acc107ae99aff9afdcc200a001640ab9efc6fa7f2fedb84c14e92f2e -->
<a id="built-in-chat-providers"></a>
# 내장 채팅 제공업체

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · **한국어** · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../providers.md)
<!-- fennara-doc-nav:end -->

Godot 안의 Fennara 채팅 독에 모델 제공업체를 연결합니다.

> [!NOTE]
> 외부 MCP 앱은 자체 모델 설정을 사용합니다. Codex, Claude, Cursor 또는 다른 MCP 앱에서 Fennara를 사용하기 위해 여기에서 제공업체를 연결할 필요는 없습니다. [MCP 앱과 내장 채팅](chat-vs-mcp.md)을 참고하세요.

<a id="quick-setup"></a>
## 빠른 설정

1. Fennara 독에서 **Chat Settings > Chat**을 엽니다.
2. **Open providers**를 선택합니다.
3. 클라우드 제공업체를 선택하고 사용자 키를 입력하거나, 로컬 모델에는 Ollama 또는 LM Studio를 선택합니다.
4. 모델을 선택합니다.

작성기에서 `/provider`와 `/model`을 입력할 수도 있습니다.

<a id="provider-reference"></a>
## 제공업체 참고

| 제공업체 | 연결 방법 | 모델 ID 형식 | 참고 |
| --- | --- | --- | --- |
| OpenAI | [OpenAI API keys](https://platform.openai.com/api-keys)에서 키를 만듭니다. Fennara 키/환경 변수: `OPENAI_API_KEY`. | `openai/<model>` | OpenAI 공식 API를 사용합니다. |
| Anthropic | [Claude Console API keys](https://console.anthropic.com/settings/keys)에서 키를 만듭니다. Fennara 키/환경 변수: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Anthropic 공식 Messages API를 사용합니다. |
| OpenRouter | [OpenRouter Keys](https://openrouter.ai/settings/keys)에서 키를 만듭니다. Fennara 키/환경 변수: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | OpenRouter API를 사용합니다. |
| Ollama Cloud | [Ollama API keys](https://ollama.com/settings/keys)에서 키를 만듭니다. Fennara 키/환경 변수: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | 로컬 Ollama 서버가 아니라 Ollama 호스팅 API를 사용합니다. |
| DeepSeek | [DeepSeek API keys](https://platform.deepseek.com/api_keys)에서 키를 만듭니다. Fennara 키/환경 변수: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | DeepSeek의 OpenAI 호환 API를 사용합니다. |
| Z.AI | [Z.AI API keys](https://z.ai/manage-apikey/apikey-list)에서 키를 만듭니다. Fennara 키/환경 변수: `ZHIPU_API_KEY`. | `zai/<model>` | Z.AI의 OpenAI 호환 API를 사용합니다. |
| Moonshot AI | [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys)에서 키를 만듭니다. Fennara 키/환경 변수: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Moonshot의 OpenAI 호환 API를 사용합니다. |
| Moonshot AI (China) | [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys)에서 키를 만듭니다. Fennara 키/환경 변수: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Moonshot China의 OpenAI 호환 API를 사용합니다. |
| Kimi For Coding | [Kimi Code Console](https://www.kimi.com/code/console)에서 키를 만듭니다. Fennara 키/환경 변수: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Kimi의 Anthropic 호환 Messages API를 사용합니다. Kimi Code 접근 권한이 필요합니다. |
| MiniMax | [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview)의 **API Keys > Create new secret key**에서 종량제 키를 만듭니다. Fennara 키/환경 변수: `MINIMAX_API_KEY`. | `minimax/<model>` | `minimax.io`에서 MiniMax의 Anthropic 호환 Messages API를 사용합니다. |
| MiniMax Token Plan | [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview)의 **Billing > Token Plan**에 있는 Subscription Key를 사용합니다. Fennara 키/환경 변수: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Token Plan Subscription Key는 종량제 API 키와 별개입니다. |
| MiniMax (China) | [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) API 키 페이지에서 종량제 키를 만듭니다. Fennara 키/환경 변수: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | `minimaxi.com`에서 MiniMax China의 Anthropic 호환 Messages API를 사용합니다. |
| MiniMax Token Plan (China) | [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview) Token Plan 페이지의 Subscription Key를 사용합니다. Fennara 키/환경 변수: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | China Token Plan Subscription Key는 종량제 API 키와 별개입니다. |
| NVIDIA | [build.nvidia.com](https://build.nvidia.com/)에서 키를 만듭니다. Fennara 키/환경 변수: `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | NVIDIA의 OpenAI 호환 호스팅 NIM API를 사용합니다. |
| Ollama | 로컬 Ollama 서버를 실행합니다. 클라우드 API 키는 필요하지 않습니다. | `ollama/<local-model>` | 기본값은 `http://127.0.0.1:11434`입니다. |
| LM Studio | LM Studio의 로컬 서버를 시작합니다. 기본적으로 키가 필요하지 않습니다. | `lmstudio/<local-model>` | 기본값은 `http://127.0.0.1:1234/v1`입니다. LM Studio 서버에 인증이 필요하면 데몬 환경에서 `LMSTUDIO_API_KEY`를 설정하세요. |

클라우드 제공업체에는 사용자 API 키 또는 구독 키가 필요합니다. 로컬 제공업체에는 사용 가능한 모델이 있는 실행 중인 로컬 서버가 필요합니다.

OpenRouter 선택은 항상 명시적인 `openrouter/<provider>/<model>` 형식을 사용합니다. 이전에 저장된 `<provider>/<model>` OpenRouter 선택은 설정을 불러올 때 한 번 마이그레이션되지만, 새 라우팅에는 해당 레거시 형식을 사용하지 않습니다.

Fennara는 독의 제공업체 선택기에서 입력한 키를 저장할 수 있습니다. Chat Settings의 **Open providers** 버튼은 같은 선택기를 엽니다. 환경 변수를 선호하는 경우 위의 키/환경 변수 이름을 Fennara가 인식합니다. 저장된 키는 Godot 프로젝트 밖의 데몬 로컬 앱 데이터에 있습니다.

<a id="custom-openai-compatible-providers"></a>
## 사용자 지정 OpenAI 호환 제공업체

제공업체 선택기 아래쪽의 **Custom**을 선택하여 로컬 라우터나 내부 API 게이트웨이 같은 OpenAI 호환 엔드포인트를 추가합니다. 다음을 입력하세요.

- 고유한 소문자 제공업체 ID
- Fennara에 표시할 이름
- API 버전에서 끝나는 기본 URL, 예: `http://localhost:20128/v1`
- 선택 사항인 API 키
- 하나 이상의 모델 ID, 표시 이름, 컨텍스트 길이, 최대 출력 토큰 제한
- 선택 사항인 요청 헤더

모델 ID는 엔드포인트가 기대하는 값과 일치해야 합니다. Fennara는 모델 선택기에 `<provider-id>/<model-id>`로 표시하지만 제공업체에는 `<model-id>`만 전송합니다. 엔드포인트는 OpenAI 호환 `/chat/completions` 요청 및 스트리밍 응답 형식을 구현해야 합니다.

API 키와 사용자 지정 헤더 값은 Fennara의 보호된 데몬 인증 저장소를 사용합니다. 제공업체 정의는 Godot 프로젝트 밖의 데몬 관리 로컬 앱 데이터에 남습니다. 모델 제한이 정확하면 요청이 모델 컨텍스트 창을 넘기기 전에 Fennara가 대화 기록을 압축하고 생성되는 요약을 모델 출력 제한 안에 유지할 수 있습니다. 이 필드가 생기기 전에 저장된 기존 사용자 지정 모델은 호환성 기본값인 컨텍스트 64,000토큰과 출력 4,096토큰으로 불러옵니다.

저장하면 사용자 지정 제공업체가 모델 수와 함께 제공업체 선택기에 나타납니다. 해당 제공업체를 선택하면 폼을 다시 열어 모델을 추가하거나 이름을 바꿀 수 있습니다. API 키를 비워 두면 저장된 키가 유지되고, 새로 입력한 헤더는 이름을 기준으로 저장된 헤더와 병합됩니다.

<a id="where-settings-live"></a>
## 설정 저장 위치

Fennara는 내장 채팅 설정을 Godot 프로젝트 밖에서 데몬을 통해 로컬로 저장합니다.

- 제공업체 API 키
- 사용자 지정 제공업체 헤더 값
- 사용자 지정 OpenAI 호환 제공업체 정의
- 로컬 제공업체 기본 URL
- 선택한 모델
- 추론 강도
- 제공업체 응답 제한 시간
- Godot 임베디드 또는 시스템 브라우저로 열기 중 하나인 채팅 표시 모드
- 채팅 기록

이 설정은 `res://addons/fennara/`에 기록되지 않으며 Claude, Codex, Cursor, Gemini 또는 다른 외부 MCP 앱과 공유되지 않습니다.

<a id="provider-response-timeout"></a>
## 제공업체 응답 제한 시간

**Provider response timeout** 설정은 내장 채팅이 각 모델 요청의 완료를 기다리는 시간을 제어합니다. 기본값은 120초이며 30초에서 3600초 사이로 설정할 수 있습니다. 값을 늘리면 느린 로컬 모델이나 도구를 많이 사용하는 긴 턴이 완료되는 데 도움이 될 수 있습니다. 데몬은 선택한 제한 시간을 제공업체 요청에 적용하고 제한에 도달하면 요청을 취소합니다.

<a id="chat-display-setting"></a>
## 채팅 표시 설정

Chat Settings 대화상자에는 **Open chat in my system browser next time**이 있습니다.

이 설정이 꺼져 있으면 Fennara는 Godot 독 안에 내장 채팅을 렌더링하려고 합니다. 켜져 있으면 독에 **Open chat** 버튼이 표시되고 `127.0.0.1`의 로컬 데몬을 통해 같은 내장 채팅을 엽니다. Godot 에디터의 GPU 및 메모리 사용량을 줄일 수 있으며 네이티브 웹뷰를 시작할 수 없을 때의 대체 경로이기도 합니다.

이 설정의 변경은 다음에 Godot을 시작할 때 적용됩니다. 내장 채팅 UI의 표시 위치만 바뀌며 선택한 제공업체, 모델, API 키, 채팅 기록, MCP 앱 설정 또는 Claude/Codex/Cursor가 외부에서 사용하는 모델은 바뀌지 않습니다.

<a id="picker-shortcuts"></a>
## 선택기 바로 가기

Chat Settings, 독 컨트롤, `/provider`는 같은 제공업체 선택기를 엽니다. `/model` 또는 독 모델 컨트롤로 모델 선택기를 여세요.

명령 팔레트 동작은 [내장 채팅 슬래시 명령](slash-commands.md)을 참고하세요.

<a id="local-providers"></a>
## 로컬 제공업체

Ollama:

```bash
ollama serve
ollama pull llama3.1:8b
```

그다음 다음을 선택합니다.

```text
ollama/llama3.1:8b
```

이전 `local/<model>` 선택도 Ollama 호환 별칭으로 계속 허용됩니다. 새 설정에는 명시적인 `ollama/<model>` 형식을 권장합니다.

Fennara는 Ollama의 호출당 최댓값을 OpenAI 호환 `max_tokens` 필드로
전송하며, Ollama는 이를 기본 `num_predict` 옵션에 매핑합니다.

LM Studio에서는 LM Studio에서 로컬 서버를 시작하고 다음 형식의 모델 ID를 선택하세요.

```text
lmstudio/<loaded-model-id>
```

Ollama와 LM Studio 제공자 설정 양식은 호출당 최대 출력 설정을 공유하며
기본값은 8,192토큰입니다. Fennara는 이 값을 `max_tokens`로 전송하고 채팅
기록을 압축할 시점을 결정할 때 해당 공간을 예약합니다. 로컬 서버가 로드된
컨텍스트 길이를 보고하면 입력 공간을 유지할 수 있도록 유효 출력 한도를
컨텍스트의 절반으로 제한합니다.

<a id="model-catalog"></a>
## 모델 카탈로그

데몬은 클라우드 제공업체용 로컬 모델 카탈로그를 유지하고 로컬 서버에는 현재 사용 가능한 모델을 요청합니다. Godot이 열린 동안 카탈로그나 로컬 서버가 바뀌면 모델 선택기를 새로 고치거나 제공업체 및 모델 선택기를 다시 여세요.

Fennara는 요청을 보내기 전에 기본 모델 기능을 검사합니다.

- 텍스트 출력 필수
- Fennara 도구 사용에는 도구 호출 필수
- 이미지 첨부 파일을 이미지 컨텍스트로 보내기 전에 이미지 입력 필수

Fennara 채팅에서 Ollama 이미지 입력은 아직 활성화되지 않았습니다.
