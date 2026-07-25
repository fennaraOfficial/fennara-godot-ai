<!-- fennara-i18n: locale=ru source=docs/slash-commands.md sha256=a6f8a02a401ca4ff41adf6f0df1b17ca69b8561b605a2420a8248857e4eb2cd3 -->
<a id="built-in-chat-slash-commands"></a>
# Slash-команды встроенного чата

<!-- fennara-doc-nav:start -->
[English](../../slash-commands.md) · [简体中文](../zh-CN/slash-commands.md) · [Español](../es/slash-commands.md) · [Português do Brasil](../pt-BR/slash-commands.md) · [日本語](../ja/slash-commands.md) · [한국어](../ko/slash-commands.md) · **Русский** · [Français](../fr/slash-commands.md) · [Deutsch](../de/slash-commands.md) · [Türkçe](../tr/slash-commands.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../slash-commands.md)
<!-- fennara-doc-nav:end -->

Slash-команды являются сокращениями на панели чата Fennara внутри Godot. Это команды интерфейса, а не инструменты MCP и не запросы, отправляемые модели.

Введите `/` в поле ввода, чтобы открыть палитру команд.

| Команда | Что открывает | Для чего используется |
| --- | --- | --- |
| `/provider` | Выбор провайдера | Подключение облачного провайдера, настройка URL локального провайдера или переключение провайдера. |
| `/model` | Выбор модели | Выбор модели текущего или подключенного провайдера. |

<a id="how-they-behave"></a>
## Поведение команд

- Используйте клавиши со стрелками для перемещения между предложениями команд.
- Нажмите Enter, чтобы выполнить выбранную команду.
- Нажмите Escape, чтобы закрыть палитру команд.
- Текст slash-команды удаляется из поля ввода перед отправкой сообщения чата.

<a id="common-flow"></a>
## Обычный рабочий процесс

Для встроенной панели чата:

```text
/provider
```

Подключите OpenAI, Anthropic, OpenRouter, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, локальный Ollama или LM Studio.

Затем:

```text
/model
```

Выберите модель, которую должна использовать панель.

Не используйте эти slash-команды для внешних MCP-приложений. Настройте приложение с помощью `fennara mcp-setup`, затем попросите приложение использовать инструменты Fennara MCP.
