<!-- fennara-i18n: locale=ru source=docs/mcp-setup.md sha256=42086801de2de7b36545c45d5af394cca77a858878ed242ca2014555e79b76df -->
<a id="mcp-setup"></a>
# Настройка MCP

<!-- fennara-doc-nav:start -->
[English](../../mcp-setup.md) · [简体中文](../zh-CN/mcp-setup.md) · [Español](../es/mcp-setup.md) · [Português do Brasil](../pt-BR/mcp-setup.md) · [日本語](../ja/mcp-setup.md) · [한국어](../ko/mcp-setup.md) · **Русский** · [Français](../fr/mcp-setup.md) · [Deutsch](../de/mcp-setup.md) · [Türkçe](../tr/mcp-setup.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../mcp-setup.md)
<!-- fennara-doc-nav:end -->

Подключите внешнее приложение ИИ к инструментам Godot в Fennara. Приложение
продолжит использовать собственную учетную запись модели, подписку или
настройку API.

> [!NOTE]
> Встроенный чат Fennara при этом не настраивается. Если вы не уверены, какой
> вариант вам нужен, см. [Приложения MCP и встроенный чат](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Быстрая настройка

1. Завершите **Set Up Fennara** в панели Godot.
2. Откройте **Chat Settings > MCP Apps**.
3. Найдите свое приложение и нажмите **Set Up**.
4. Перезапустите приложение.

Перед изменением конфигурации MCP приложения Fennara создает резервную копию.
Объединенный вариант **Claude** настраивает Claude Code и Claude Desktop.
Вариант **Gemini & Antigravity** настраивает обе общие цели.

<a id="terminal-alternative"></a>
### Вариант через терминал

Сначала запустите `fennara install` внутри проекта Godot, затем выберите цель:

| Приложение | Команда |
| --- | --- |
| Claude Code и Claude Desktop | `fennara mcp-setup --claude` |
| Только Claude Code | `fennara mcp-setup --claude-code` |
| Только Claude Desktop | `fennara mcp-setup --claude-desktop` |
| Codex | `fennara mcp-setup --codex` |
| Cursor | `fennara mcp-setup --cursor` |
| Gemini и Antigravity | `fennara mcp-setup --gemini` или `fennara mcp-setup --antigravity` |
| Cline | `fennara mcp-setup --cline` |
| VS Code | `fennara mcp-setup --vscode` |
| OpenCode | `fennara mcp-setup --opencode` |
| Windsurf | `fennara mcp-setup --windsurf` |
| Kiro | `fennara mcp-setup --kiro` |

Выполните `fennara mcp-setup --help`, чтобы увидеть список целей,
поддерживаемых установленной версией CLI.

<a id="manual-setup"></a>
## Ручная настройка

Используйте ручную настройку, только если вашего приложения нет в списке,
команда настройки не может найти его файл конфигурации или вы намеренно хотите
отредактировать конфигурацию MCP самостоятельно.

Перед редактированием создайте резервную копию файла конфигурации. Затем добавьте
локальный сервер MCP stdio с именем `fennara`, который указывает на стабильную
программу запуска Fennara MCP.

Стандартные пути к программе запуска:

```text
Windows: %LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
macOS:   ~/Library/Application Support/Fennara/bin/fennara-mcp
Linux:   ~/.local/share/fennara/bin/fennara-mcp
```

Используйте настоящий абсолютный путь на своем компьютере. Не направляйте
приложения MCP на `versions/<version>/fennara-mcp-runtime`. Стабильная программа
запуска из `bin/` сохраняет работоспособность конфигураций приложений после
обновлений Fennara.

<a id="json-mcpservers"></a>
### JSON `mcpServers`

Многие приложения MCP используют объект верхнего уровня `mcpServers`:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

Некоторые приложения используют тот же ключ `mcpServers`, но требуют только
`command`. Если в существующей конфигурации уже есть другие серверы, сохраните
эти записи и добавьте только сервер `fennara`.

Конфигурации в стиле Cline также могут включать более длительное время ожидания
инструмента в секундах:

```json
{
  "mcpServers": {
    "fennara": {
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {},
      "timeout": 300
    }
  }
}
```

<a id="vs-code-style-json-servers"></a>
### JSON `servers` в стиле VS Code

Некоторые клиенты, включая пользовательскую или проектную конфигурацию MCP в
VS Code, используют объект верхнего уровня `servers` и требуют
`type: "stdio"`:

```json
{
  "servers": {
    "fennara": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe",
      "args": [],
      "env": {}
    }
  }
}
```

<a id="opencode-style-json-mcp"></a>
### JSON `mcp` в стиле OpenCode

Конфигурация JSON в стиле OpenCode использует объект верхнего уровня `mcp`.
Время ожидания в нем указывается в миллисекундах:

```json
{
  "mcp": {
    "fennara": {
      "type": "local",
      "command": ["C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"],
      "enabled": true,
      "timeout": 300000
    }
  }
}
```

<a id="codex-style-toml"></a>
### TOML в стиле Codex

Codex использует TOML:

```toml
[mcp_servers.fennara]
command = "C:\\Users\\you\\AppData\\Local\\Fennara\\bin\\fennara-mcp.exe"
startup_timeout_sec = 30
tool_timeout_sec = 300
```

Не вставляйте JSON в файл TOML или TOML в файл JSON. Используйте формат,
который уже применяет приложение.

<a id="common-config-locations"></a>
## Распространенные расположения конфигурации

Ниже приведены распространенные расположения, используемые помощником настройки
Fennara и текущими клиентами MCP. Приложения могут менять пути к своим
конфигурациям, а некоторые поддерживают одновременно глобальные и локальные
для проекта конфигурации. Если в приложении есть команда наподобие
**Open MCP Config**, используйте ее вместо догадок.

```text
Codex:          ~/.codex/config.toml
Cursor:         ~/.cursor/mcp.json
Cline:          ~/.cline/data/settings/cline_mcp_settings.json
VS Code:        user mcp.json or <project>/.vscode/mcp.json
Claude Code:    ~/.claude.json
Claude Desktop: macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
                Windows: %APPDATA%\Claude\claude_desktop_config.json
Gemini CLI:     ~/.gemini/settings.json
Antigravity:    ~/.gemini/config/mcp_config.json or ~/.gemini/antigravity/mcp_config.json
OpenCode:       ~/.config/opencode/opencode.json
Windsurf:       ~/.codeium/windsurf/mcp_config.json
Kiro:           ~/.kiro/settings/mcp.json
```

<a id="timeout-guidance"></a>
## Рекомендации по времени ожидания

Некоторые инструменты Fennara могут работать дольше небольшого стандартного
времени ожидания MCP, поскольку они могут запрашивать у Godot проверку сцен,
изучение состояния среды выполнения, создание снимков экрана или запуск
диагностики.

Если клиент поддерживает эту возможность, задайте более длительное время
ожидания отдельного инструмента:

```text
30 seconds for server startup
300 seconds for tool calls
300000 milliseconds for clients whose timeout field is in milliseconds
```

Если клиент не поддерживает время ожидания для отдельного сервера, используйте
документированную глобальную настройку времени ожидания MCP этого клиента.

<a id="verify-the-connection"></a>
## Проверка подключения

Откройте проект Godot, затем попросите свое приложение MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Если открыто несколько проектов Godot, выберите проект, который должен получать
вызовы внешних инструментов MCP, с помощью элемента **MCP target** в панели
Fennara.

<a id="troubleshooting"></a>
## Устранение неполадок

Если Fennara не появляется в приложении MCP:

- убедитесь, что путь к программе запуска абсолютный и файл существует;
- убедитесь, что синтаксис конфигурации является допустимым JSON, JSON5 или TOML
  в соответствии с требованиями приложения;
- убедитесь, что сервер назван `fennara`;
- убедитесь, что приложение читает отредактированный вами файл конфигурации;
- полностью закройте и снова откройте приложение MCP;
- убедитесь, что в проекте Godot установлено дополнение Fennara;
- убедитесь, что нужный проект Godot выбран в качестве цели MCP.

<a id="unsupported-mcp-apps"></a>
## Неподдерживаемые приложения MCP

Если вашего приложения MCP нет в списке, сначала найдите в официальной
документации приложения расположение и формат конфигурации MCP. Затем попросите
LLM предложить минимальное безопасное изменение:

```text
I have a local stdio MCP server executable at:
<paste the full path to fennara-mcp here>

I want to add it to <app name>.
The app's MCP config file is:
<paste config path here>

The config format is <JSON/TOML/YAML/etc>.

Please show the smallest safe edit to add a server named "fennara".
Preserve all existing config. If the app needs "mcpServers", "servers", "mcp",
or another top-level key, use the key required by that app's official docs.
```

Проверьте результат перед сохранением, затем перезапустите приложение MCP.
