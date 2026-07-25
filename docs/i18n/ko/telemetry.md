<!-- fennara-i18n: locale=ko source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# 익명 텔레메트리

<!-- fennara-doc-nav:start -->
[문서](README.md)

🌐 **언어**

[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · [Español](../es/telemetry.md) · [Português do Brasil](../pt-BR/telemetry.md) · [日本語](../ja/telemetry.md) · **한국어** · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · [Deutsch](../de/telemetry.md) · [Türkçe](../tr/telemetry.md)

> ℹ️ 영문 원본을 바탕으로 AI가 작성한 번역입니다. 원어민 검토를 환영합니다. [영문 원본](../../telemetry.md)
<!-- fennara-doc-nav:end -->

Fennara는 UTC 기준 하루에 최대 한 번 작은 익명 활동 이벤트를 전송합니다. 호환되는 Godot 에디터가 로컬 데몬에 연결된 뒤에만 이벤트를 전송합니다. 이 정보는 유지관리자가 활성 설치 수, 지원 플랫폼 사용량, 버전 채택을 측정하는 데 도움이 됩니다.

텔레메트리는 기본적으로 활성화되어 있습니다. 비활성화하려면 **Chat Settings > Chat > Anonymous telemetry**를 여세요. 헤드리스 및 자동화 환경에서는 다음 중 하나를 설정할 수 있습니다.

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

환경 변수는 저장된 UI 설정보다 우선합니다. 텔레메트리를 끄면 이후 이벤트가 중지되고 로컬 텔레메트리 식별자와 마지막 전송 상태가 삭제됩니다. 다시 켜면 다음에 Godot이 연결될 때 새로운 무작위 식별자가 생성됩니다.

<a id="event-contents"></a>
## 이벤트 내용

`fennara_active_installation` 이벤트에는 다음 항목만 포함됩니다.

| 필드 | 목적 |
| --- | --- |
| `schema_version` | 작은 텔레메트리 페이로드 계약의 버전 |
| `event` | 고정 이벤트 이름 |
| `installation_id` | 로컬에서 생성되며 하드웨어나 계정에서 파생되지 않는 무작위 UUID |
| `fennara_version` | 실행 중인 데몬 버전 |
| `godot_version` | `4.6.3` 같은 숫자형 Godot 버전 |
| `platform` | `windows`, `macos` 또는 `linux` |
| `architecture` | `x86_64` 또는 `aarch64` |

Fennara는 프로젝트 이름, 프로젝트 경로, 계정 정보, 프롬프트, 채팅 메시지, 제공업체 키, 모델 이름, 도구 이름, 도구 인수, 도구 결과, 로그, 스크린샷, 씬 내용, 파일 이름 또는 오류 텍스트를 전송하지 않습니다.

<a id="storage-and-transport"></a>
## 저장 및 전송

데몬은 무작위 식별자와 마지막으로 성공한 UTC 날짜를 공유 Fennara 앱 데이터 디렉터리 아래에 저장합니다.

```text
Fennara/
  telemetry/
    state.json
```

데몬은 HTTPS를 통해 `https://fennara.io/api/telemetry`로 이벤트를 전송합니다. 수신기는 정확한 필드 허용 목록을 검증하고 원시 설치 UUID를 서버 측 HMAC로 대체한 다음 PostHog로 전달합니다. 이 이벤트에서는 PostHog 사용자 프로필과 IP 지리 위치가 비활성화되어 있습니다.

Vercel 수신기는 HTTPS 요청을 처리하는 동안 일반 네트워크 메타데이터를 필연적으로 확인합니다. 해당 메타데이터는 PostHog 이벤트 페이로드에 복사되지 않습니다.

<a id="delivery-behavior"></a>
## 전송 동작

텔레메트리는 Godot 도구 호출 경로 밖에서 실행됩니다.

- 제한된 큐가 기다리지 않고 활동 신호를 받습니다.
- 백그라운드 작업자 하나가 HTTP 클라이언트 하나를 재사용합니다.
- 요청에는 짧은 제한 시간이 있습니다.
- 큐 가득 참, 파일 시스템 문제, 네트워크 실패 또는 서버 거부는 조용히 허용되며 Fennara 도구를 실패시키지 않습니다.
- 서버가 이벤트를 받아들인 뒤에만 UTC 날짜를 기록하므로, 실패한 전송은 이후 Godot 연결에서 다시 시도할 수 있습니다.
- 종료할 때 잠시 기다린 뒤 데몬을 지연시키는 대신 텔레메트리 작업자를 취소합니다.

하나의 설치는 저장된 무작위 UUID 하나입니다. 컴퓨터 두 대에서 Fennara를 사용하면 설치 두 개로 계산됩니다. Fennara 앱 데이터를 지우거나 텔레메트리를 비활성화한 뒤 다시 활성화하면 새 식별자가 생성됩니다.

월간 활성 설치 수는 해당 달에 `fennara_active_installation` 이벤트를 하나 이상 전송한 서로 다른 익명 설치 식별자의 수입니다.
