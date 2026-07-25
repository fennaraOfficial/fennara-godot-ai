<!-- fennara-i18n: locale=ru source=README.md sha256=de25e1e576166d72e20af0bb21179e8cb1f0cc5e13764f7ae945647d006c2052 -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · [Português do Brasil](README.pt-BR.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · **Русский** · [Français](README.fr.md) · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Демонстрации](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/ru/demos.md)
[![Лицензия](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Fennara используют разработчики и команды, работающие с Godot, в том числе [Somni Game Studios](https://somnigamestudios.com/).

Fennara предоставляет ИИ-ассистентам прямое подключение к Godot. Используйте его из приложений с поддержкой MCP, таких как Codex, Claude, Cursor, Gemini и Antigravity, или через необязательную панель чата в редакторе.

Агенты могут исследовать сцены, проверять скрипты, делать снимки экрана, читать ошибки выполнения и проверять изменения внутри редактора, вместо того чтобы строить догадки только по файлам проекта.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Сравнение Fennara с другими MCP для Godot" width="100%" />
      </a>
    </td>
    <td>
      <strong>Посмотрите основную демонстрацию</strong><br />
      Сравнение Fennara с другими MCP для Godot.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Воспроизвести видео</a><br />
      <a href="docs/i18n/ru/demos.md">Посмотреть все демонстрационные видео</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## Что делает Fennara

- предоставляет внешним ИИ-приложениям инструменты, учитывающие особенности Godot, через MCP
- добавляет необязательную локальную панель чата в редактор Godot
- возвращает реальные данные обратной связи от Godot: деревья сцен, диагностику, снимки экрана, журналы выполнения и результаты проверок
- позволяет агенту сверяться с открытым редактором, а не полагаться только на файловую систему

Внешние MCP-приложения и встроенный чат используют отдельные настройки моделей. См. [MCP-приложения и встроенный чат](docs/i18n/ru/chat-vs-mcp.md) и [Провайдеры встроенного чата](docs/i18n/ru/providers.md).

<a id="requirements"></a>
## Требования

- Godot 4.5 или новее.
- Поддерживаемая настольная ОС: Windows x86_64, Linux x86_64 или macOS arm64.
- Приложение для программирования с поддержкой MCP требуется только в том случае, если вы хотите использовать Fennara из Claude, Codex, Cursor, Gemini, Antigravity или другого внешнего ИИ-приложения.
- Провайдер чата требуется только в том случае, если вы хотите использовать встроенную панель чата Fennara. Это может быть ключ облачного провайдера или локальный провайдер, например Ollama / LM Studio.

Полное пошаговое руководство по установке приведено в разделе [Настройка](docs/i18n/ru/setup.md).

<a id="what-setup-adds"></a>
## Что добавляет настройка

- аддон Fennara, размещенный в `res://addons/fennara/`
- небольшую CLI-команду `fennara`, установленную в каталоге данных приложения Fennara
- локальный MCP-сервер, который используют приложения для программирования с ИИ
- локальный демон, связывающий запросы MCP и чата с открытым редактором Godot
- сформированные инструкции проекта для ИИ-агентов

Встроенная панель чата использует системный webview: Microsoft Edge WebView2 в Windows, WKWebView/WebKit в macOS и общий рантайм CEF под управлением Fennara в Linux. Инструменты MCP продолжают работать, даже если необязательную панель чата запустить не удается.

<a id="install"></a>
## Установка

В Windows и Linux выберите установку через аддон или CLI. В macOS используйте
установку через CLI, описанную ниже, если хотите избежать уведомления системы
безопасности macOS, которое может появиться после загрузки и распаковки ZIP-файла
аддона вручную.

<a id="add-the-addon-to-your-project"></a>
### Добавление аддона в проект

- Откройте страницу [Последний выпуск](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest), загрузите `fennara-addon-latest.zip` и извлеките находящуюся в нем папку `addons/fennara/` в свой проект.

Откройте проект, выберите панель Fennara и нажмите **Set Up Fennara**.

Fennara является зависимостью редактора, а не среды выполнения игры. Во время
экспорта плагин редактора удаляет автозагрузку среды выполнения из
экспортируемого проекта и исключает файлы в `res://addons/fennara/` и
`res://.fennara/`. После завершения экспорта проект редактора восстанавливается.
Если при получении проекта в CI аддон исключается через `.gitignore`, перед
запуском Godot выполните `fennara prepare-export --project path/to/project` или
установите аддон в эту рабочую копию. Godot проверяет пути автозагрузки до
запуска плагинов экспорта, поэтому подготовку необходимо выполнить заранее.

> **macOS:** Аддон из выпуска содержит нативную библиотеку, которая в настоящее
> время не нотарифицирована Apple. Если загрузить ZIP-файл аддона через браузер
> и распаковать его вручную, macOS может сообщить, что не может проверить
> `libfennara.macos.editor` на отсутствие вредоносного ПО. Чтобы избежать этого
> уведомления, используйте установку через CLI, описанную ниже. Если уведомление
> уже появилось, закройте Godot, удалите скопированную вручную папку
> `addons/fennara/`, затем установите Fennara с помощью CLI.

<a id="install-with-the-cli-recommended-on-macos"></a>
### Установка с помощью CLI (рекомендуется в macOS)

CLI устанавливает тот же аддон Fennara. Этот способ рекомендуется для macOS,
поскольку он позволяет избежать прохождения через карантин браузера и Finder,
которое приводит к описанному выше уведомлению.

Установите CLI в Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Или в macOS и Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Затем запустите Fennara из своего проекта Godot:

```bash
cd path/to/your-godot-project
fennara install
```

Рекомендации по устранению неполадок приведены в разделе [Настройка](docs/i18n/ru/setup.md), а полный справочник команд приведен в разделе [CLI Fennara](docs/i18n/ru/cli.md).

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## Настройка провайдера или подключение MCP-приложения

<a id="built-in-chat"></a>
### Встроенный чат

Откройте **Chat Settings > Chat**, выберите **Open providers** и подключите провайдера.
Fennara использует ваш собственный ключ для облачных провайдеров (BYOK). Также
можно использовать локальный сервер Ollama или LM Studio. См. [список поддерживаемых провайдеров](docs/i18n/ru/providers.md).

<a id="mcp-apps"></a>
### MCP-приложения

Откройте **Chat Settings > MCP Apps**, найдите свое приложение и нажмите **Set Up**.

Приложение также можно подключить из терминала:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Если вашего MCP-приложения нет в Chat Settings, обратитесь к разделу [Настройка MCP](docs/i18n/ru/mcp-setup.md), где приведены полный список приложений и инструкции по ручной настройке.

<a id="update"></a>
## Обновление

Когда на панели Fennara появится кнопка **Update**, нажмите ее и следуйте подсказкам.

> **При обновлении с Fennara v0.3.8 или более ранней версии:** Один раз переустановите
> CLI с помощью приведенной выше команды для своей платформы, прежде чем запускать
> `fennara update`. Эти версии CLI обращаются к более не используемому тегу выпуска
> и не могут обнаружить актуальные выпуски. Переустановка CLI переключает последующие
> обновления на конечную точку GitHub Latest Release и не удаляет существующий аддон
> проекта или его настройки.

> **Для пользователей macOS, обновляющихся с Fennara v0.3.11:** Один раз
> переустановите CLI с помощью приведенной выше команды для macOS перед обновлением.
> CLI версии v0.3.11 отклоняет существующий пакет фреймворка macOS до того, как
> успевает обновить сам себя. Переустановка заменяет только CLI, она не удаляет
> аддон проекта или его настройки.

Чтобы выполнить обновление из терминала, закройте Godot и запустите:

```bash
cd path/to/your-godot-project
fennara update
```

Инструкции по восстановлению и диагностике приведены в разделе [Обновление Fennara](docs/i18n/ru/setup.md#update-fennara).

<a id="tools"></a>
## Инструменты

Fennara предоставляет небольшой набор инструментов, учитывающих особенности Godot:

- запись или обновление файлов проекта с возвратом диагностики
- выполнение одноразовых скриптов редактирования сцен
- исследование деревьев сцен, узлов, ресурсов и классов Godot
- проверка сцен
- создание снимков экрана
- запуск сеансов выполнения и чтение журналов выполнения
- выполнение небольших скриптов времени выполнения в работающей сцене

Цель не состоит в том, чтобы заменить обычные файловые инструменты агента. Fennara предоставляет недостающий цикл обратной связи с Godot.

<a id="privacy"></a>
## Конфиденциальность

Fennara отправляет одно анонимное событие об активной установке не чаще одного
раза в сутки по UTC после подключения Godot. Оно содержит случайный UUID
установки, версии Fennara и Godot, операционную систему и архитектуру процессора.
Оно не содержит данные проекта, пути, запросы, действия инструментов, журналы,
снимки экрана или сведения об учетной записи.

Телеметрию можно отключить в **Chat Settings > Chat > Anonymous telemetry**,
с помощью `FENNARA_DISABLE_TELEMETRY=true` или `DO_NOT_TRACK=1`. Полное описание
содержимого события, хранения, передачи и условий отказа приведено в разделе
[Анонимная телеметрия](docs/i18n/ru/telemetry.md).

<a id="demos"></a>
## Демонстрации

Посмотрите практическую демонстрацию Fennara:

[![Этот плагин Godot навсегда меняет разработку игр с ИИ](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

Другие видео:

- [Я дал Codex изображение игры от ИИ, и он создал ее в Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP создает игру в стиле Katamari на Godot](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [Этот плагин Godot навсегда меняет разработку игр с ИИ](https://www.youtube.com/watch?v=wKln8248y2M)

Больше видео с канала Fennara приведено в разделе [Демонстрации](docs/i18n/ru/demos.md).

<a id="star-history"></a>
## История звезд
<a href="https://www.star-history.com/?repos=fennaraOfficial%2Ffennara-godot-ai&type=date&legend=top-left">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&theme=dark&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
   <img alt="График истории звезд" src="https://api.star-history.com/chart?repos=fennaraOfficial/fennara-godot-ai&type=date&legend=top-left&sealed_token=fezQNXcS0bAmXpZnoyG8FLlAkcnajD5wnBrugJG7WDJRaoSAqXHjV010Bm1XJN9cWChDHTsk1MaWr3jWkh8KF-Hqp1fxnJfmPlvjUc8vtS_kao5tXHGBGQyL5IHhgzDdaoMqjRdH5B8pdo2Z-Pm511AXJxdwOYbXFCqcKNkpgS6WgxVUNjOTrKc5_ZkO" />
 </picture>
</a>

<a id="documentation"></a>
## Документация

| Начните с... | Если вам нужно... |
| --- | --- |
| [Главная страница документации](docs/i18n/ru/README.md) | Все руководства и справочные страницы |
| [Настройка](docs/i18n/ru/setup.md) | Установка, обновления и устранение неполадок |
| [Провайдеры чата](docs/i18n/ru/providers.md) | Модели и ключи встроенного чата |
| [Настройка MCP](docs/i18n/ru/mcp-setup.md) | Codex, Claude, Cursor и другие MCP-приложения |
| [Инструменты](docs/i18n/ru/tools.md) | Обратная связь Godot, доступная агентам |
| [Анонимная телеметрия](docs/i18n/ru/telemetry.md) | Собираемые данные, порядок доставки и способы отказа |
| [Участие в разработке](docs/i18n/ru/CONTRIBUTING.md) | Рекомендации по разработке и pull request |

<a id="community"></a>
## Сообщество

Вопросы, помощь с настройкой и ранние отзывы приветствуются в Discord:

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## Лицензия

См. [LICENSE.md](LICENSE.md).
