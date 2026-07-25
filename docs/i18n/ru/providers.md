<!-- fennara-i18n: locale=ru source=docs/providers.md sha256=17c492522ed54ff1ffb4942e27572fa7e15709cec2853ac38be5b5f0dc91db0d -->
<a id="built-in-chat-providers"></a>
# Провайдеры встроенного чата

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · [Português do Brasil](../pt-BR/providers.md) · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · **Русский** · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../providers.md)
<!-- fennara-doc-nav:end -->

Подключите провайдера моделей к панели чата Fennara внутри Godot.

> [!NOTE]
> Внешние MCP-приложения используют собственную настройку модели. Чтобы использовать
> Fennara из Codex, Claude, Cursor или другого MCP-приложения, подключать здесь
> провайдера не требуется. См. [MCP-приложения и встроенный чат](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Быстрая настройка

1. Откройте **Chat Settings > Chat** на панели Fennara.
2. Выберите **Open providers**.
3. Выберите облачного провайдера и введите собственный ключ или выберите Ollama
   либо LM Studio для локальной модели.
4. Выберите модель.

Также можно ввести `/provider` и `/model` в поле ввода.

<a id="provider-reference"></a>
## Справочник провайдеров

| Провайдер | Как подключить | Формат идентификатора модели | Примечания |
| --- | --- | --- | --- |
| OpenAI | Создайте ключ в [OpenAI API keys](https://platform.openai.com/api-keys). Ключ или переменная среды Fennara: `OPENAI_API_KEY`. | `openai/<model>` | Использует официальный API OpenAI. |
| Anthropic | Создайте ключ в [Claude Console API keys](https://console.anthropic.com/settings/keys). Ключ или переменная среды Fennara: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Использует официальный Messages API Anthropic. |
| OpenRouter | Создайте ключ в [OpenRouter Keys](https://openrouter.ai/settings/keys). Ключ или переменная среды Fennara: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | Использует API OpenRouter. |
| Ollama Cloud | Создайте ключ в [Ollama API keys](https://ollama.com/settings/keys). Ключ или переменная среды Fennara: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Использует размещенный API Ollama, а не локальный сервер Ollama. |
| DeepSeek | Создайте ключ в [DeepSeek API keys](https://platform.deepseek.com/api_keys). Ключ или переменная среды Fennara: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | Использует совместимый с OpenAI API DeepSeek. |
| Z.AI | Создайте ключ в [Z.AI API keys](https://z.ai/manage-apikey/apikey-list). Ключ или переменная среды Fennara: `ZHIPU_API_KEY`. | `zai/<model>` | Использует совместимый с OpenAI API Z.AI. |
| Moonshot AI | Создайте ключ в [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys). Ключ или переменная среды Fennara: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Использует совместимый с OpenAI API Moonshot. |
| Moonshot AI (China) | Создайте ключ в [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys). Ключ или переменная среды Fennara: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Использует совместимый с OpenAI API Moonshot China. |
| Kimi For Coding | Создайте ключ в [Kimi Code Console](https://www.kimi.com/code/console). Ключ или переменная среды Fennara: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Использует совместимый с Anthropic Messages API Kimi. Требуется доступ к Kimi Code. |
| MiniMax | Создайте ключ с оплатой по мере использования на [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview) в разделе **API Keys > Create new secret key**. Ключ или переменная среды Fennara: `MINIMAX_API_KEY`. | `minimax/<model>` | Использует совместимый с Anthropic Messages API MiniMax по адресу `minimax.io`. |
| MiniMax Token Plan | Используйте Subscription Key из раздела **Billing > Token Plan** на [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview). Ключ или переменная среды Fennara: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | Subscription Key для Token Plan отделен от ключей API с оплатой по мере использования. |
| MiniMax (China) | Создайте ключ с оплатой по мере использования на странице ключей API [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Ключ или переменная среды Fennara: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | Использует совместимый с Anthropic Messages API MiniMax China по адресу `minimaxi.com`. |
| MiniMax Token Plan (China) | Используйте Subscription Key со страницы Token Plan [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Ключ или переменная среды Fennara: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | Китайские Subscription Key для Token Plan отделены от ключей API с оплатой по мере использования. |
| NVIDIA | Создайте ключ на [build.nvidia.com](https://build.nvidia.com/). Ключ или переменная среды Fennara: `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | Использует размещенный NIM API NVIDIA, совместимый с OpenAI. |
| Ollama | Запустите локальный сервер Ollama. Облачный ключ API не требуется. | `ollama/<local-model>` | По умолчанию используется `http://127.0.0.1:11434`. |
| LM Studio | Запустите локальный сервер LM Studio. По умолчанию ключ не требуется. | `lmstudio/<local-model>` | По умолчанию используется `http://127.0.0.1:1234/v1`. Если сервер LM Studio требует аутентификацию, задайте `LMSTUDIO_API_KEY` в среде демона. |

Облачным провайдерам требуется ваш собственный ключ API или Subscription Key.
Локальным провайдерам нужен работающий локальный сервер с доступной моделью.

Выбранные варианты OpenRouter всегда используют явный формат `openrouter/<provider>/<model>`.
Старые сохраненные варианты OpenRouter в формате `<provider>/<model>` переносятся
один раз при загрузке настроек, но этот устаревший формат не используется для
новой маршрутизации.

Fennara может хранить ключи, введенные в окне выбора провайдера на панели. В Chat Settings есть кнопка **Open providers**, открывающая то же окно. Приведенные выше имена ключей и переменных среды являются теми же именами, которые понимает Fennara, если вы предпочитаете переменные среды. Сохраненные ключи находятся в локальных данных приложения демона вне проекта Godot.

<a id="custom-openai-compatible-providers"></a>
## Пользовательские провайдеры, совместимые с OpenAI

Выберите **Custom** в нижней части окна выбора провайдера, чтобы добавить совместимую
с OpenAI конечную точку, например локальный маршрутизатор или внутренний шлюз API.
Введите:

- уникальный идентификатор провайдера в нижнем регистре
- отображаемое в Fennara имя
- базовый URL-адрес, заканчивающийся версией API, например `http://localhost:20128/v1`
- необязательный ключ API
- один или несколько идентификаторов и отображаемых имен моделей, размеров контекста и максимальных ограничений выходных токенов
- необязательные заголовки запросов

Идентификаторы моделей должны совпадать с ожидаемыми конечной точкой. Fennara
показывает их в окне выбора моделей как `<provider-id>/<model-id>`, при этом
провайдеру отправляется только `<model-id>`. Конечная точка должна реализовывать
формат запроса `/chat/completions` и потокового ответа, совместимый с OpenAI.

Ключи API и значения пользовательских заголовков используют защищенное хранилище
аутентификации демона Fennara. Определения провайдеров остаются в локальных данных
приложения под управлением демона вне проекта Godot. Точные ограничения моделей
позволяют Fennara сжимать историю беседы до того, как запрос превысит контекстное
окно модели, и удерживать сформированные сводки в пределах ограничения выходных
данных модели. Существующие пользовательские модели, сохраненные до появления
этих полей, загружаются с совместимыми значениями по умолчанию: 64 000 токенов
контекста и 4 096 выходных токенов.

После сохранения пользовательский провайдер появляется в окне выбора провайдеров
с количеством моделей. Выберите этого провайдера, чтобы снова открыть форму,
добавить или переименовать модели. Если оставить ключ API пустым, сохраненный
ключ останется без изменений, а новые введенные заголовки объединятся с
сохраненными по имени.

<a id="where-settings-live"></a>
## Где хранятся настройки

Fennara хранит настройки встроенного чата локально через демон, вне проекта Godot:

- ключи API провайдеров
- значения пользовательских заголовков провайдеров
- определения пользовательских провайдеров, совместимых с OpenAI
- базовые URL-адреса локальных провайдеров
- выбранная модель
- глубина рассуждений
- режим отображения чата: встроенный в Godot или открытый в системном браузере
- история чата

Эти настройки не записываются в `res://addons/fennara/` и не передаются Claude, Codex, Cursor, Gemini или другим внешним MCP-приложениям.

<a id="chat-display-setting"></a>
## Настройка отображения чата

Диалог Chat Settings содержит параметр **Open chat in my system browser next time**.

Когда он выключен, Fennara пытается отобразить встроенный чат внутри панели Godot. Когда он включен, на панели отображается кнопка **Open chat**, которая запускает тот же встроенный чат через локальный демон по адресу `127.0.0.1`. Это может снизить использование GPU и памяти редактора Godot, а также служит резервным способом, если нативный webview не удается запустить.

Изменение этого параметра вступает в силу при следующем запуске Godot. Оно меняет только место отображения интерфейса встроенного чата. Оно не меняет выбранного провайдера, модель, ключи API, историю чата, настройку MCP-приложений или модели, используемые Claude, Codex и Cursor во внешних приложениях.

<a id="picker-shortcuts"></a>
## Сокращения окна выбора

Chat Settings, элементы управления панели и `/provider` открывают одно и то же окно выбора провайдера.
Используйте `/model` или элемент управления моделью на панели, чтобы открыть окно выбора модели.

Поведение палитры команд описано в разделе [Slash-команды встроенного чата](slash-commands.md).

<a id="local-providers"></a>
## Локальные провайдеры

Для Ollama:

```bash
ollama serve
ollama pull llama3.1:8b
```

Затем выберите:

```text
ollama/llama3.1:8b
```

Старые варианты `local/<model>` по-прежнему принимаются как псевдонимы
совместимости Ollama. Для новых настроек используйте явный формат
`ollama/<model>`.

Для LM Studio запустите локальный сервер из LM Studio и выберите идентификатор модели следующего формата:

```text
lmstudio/<loaded-model-id>
```

<a id="model-catalog"></a>
## Каталог моделей

Демон хранит локальный каталог моделей для облачных провайдеров и запрашивает у локальных серверов список доступных в данный момент моделей. Если каталог или локальный сервер изменился, пока Godot открыт, обновите окно выбора модели или повторно откройте окно выбора провайдера или модели.

Fennara проверяет основные возможности модели перед отправкой запроса:

- требуется текстовый вывод
- для использования инструментов Fennara требуется вызов инструментов
- перед отправкой прикрепленных изображений в качестве контекста требуется поддержка изображений на входе

Ввод изображений Ollama пока не включен в чате Fennara.
