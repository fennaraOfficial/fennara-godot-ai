<!-- fennara-i18n: locale=ru source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# Ручная установка

<!-- fennara-doc-nav:start -->
[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · [Português do Brasil](../pt-BR/manual-install.md) · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · **Русский** · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Используйте эту страницу, только если вам нужно установить Fennara без процесса
настройки в Godot или команды `fennara install`.

> [!TIP]
> В Windows и Linux большинству пользователей достаточно добавить `addons/fennara`
> в проект, открыть панель Fennara и нажать **Set Up Fennara**. В macOS используйте CLI.
> См. [Настройка](setup.md).

> [!IMPORTANT]
> Ручная установка ZIP-архива дополнения не рекомендуется в macOS. Дополнение
> содержит нативную библиотеку, которая сейчас не нотарифицирована Apple, поэтому
> после загрузки через браузер и распаковки в Finder macOS может сообщить, что не
> удается проверить отсутствие вредоносного ПО в `libfennara.macos.editor`. Используйте
> [установку через CLI](setup.md#install-from-the-terminal-recommended-on-macos),
> чтобы избежать этого уведомления. Если уведомление уже появилось, закройте Godot,
> удалите вручную скопированную папку `addons/fennara/` и запустите `fennara install`.

Ручная установка состоит из четырех частей: CLI, дополнения проекта, общего
локального пакета среды выполнения и необязательной настройки приложения MCP.

<a id="1-download-release-files"></a>
## 1. Скачайте файлы выпуска

Откройте последний выпуск на GitHub:

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

Скачайте манифест выпуска, файлы для своей платформы и общий ZIP-архив дополнения.

| Назначение | Ресурс |
| --- | --- |
| План выпуска и значения SHA-256 | `fennara-release-manifest-v<version>.json` |
| CLI для Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip` |
| Локальная среда выполнения для Windows x86_64 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| CLI для Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip` |
| Локальная среда выполнения для Linux x86_64 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Встроенное веб-представление для Linux x86_64 | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| CLI для macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip` |
| Локальная среда выполнения для macOS arm64 | `fennara-release-local-macos-arm64-v<version>.zip` |
| Версионное дополнение для всех платформ | `fennara-release-addon-v<version>.zip` |

Выпуск также содержит этот псевдоним дополнения со стабильным именем для
документации и ручного скачивания:

```text
fennara-addon-latest.zip
```

В манифесте указаны ожидаемые значения SHA-256 для локальной среды выполнения,
дополнения и общих ресурсов среды выполнения. Используйте его как источник истины
при проверке файлов, скачанных вручную.

<a id="2-install-the-cli"></a>
## 2. Установите CLI

Распакуйте ZIP-архив `fennara-cli`.

Добавьте его каталог `bin` в PATH или скопируйте исполняемый файл `fennara` в один из уже существующих каталогов PATH.

Проверьте установку:

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Установите дополнение Godot

Распакуйте ZIP-архив `fennara-addon`.

Скопируйте:

```text
addons/fennara
```

в свой проект Godot, чтобы в проекте появился файл:

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. Установите локальный пакет среды выполнения

Обычно этим управляет CLI. Ручная настройка среды выполнения нужна только в том случае, если вы не используете `fennara install`.

Стандартные каталоги данных Fennara:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

Ожидаемая структура:

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

В Windows исполняемые файлы имеют расширение `.exe`.

Файл `current.json` указывает программам запуска на активную версию среды выполнения. Обычные команды `fennara install` и `fennara update` создают этот файл автоматически.

Встроенный чат в Linux использует общий каталог среды выполнения
`webview/cef/linux-x64/<cef-version>/`. При обычном выполнении `fennara install` /
`fennara update` управляемая выпуском среда выполнения CEF устанавливается
автоматически на основании манифеста и ресурса выпуска.
Если вы устанавливаете все вручную, распакуйте
`fennara-webview-cef-linux-x64-<cef-version>.zip` в этот общий каталог среды
выполнения и запишите соответствующий маркер `webview/cef/linux-x64/current.json`.
Храните эти данные вне дополнения проекта Godot. В `addons/fennara` не должно
быть `libcef.so` или других файлов среды выполнения CEF.

Эти данные CEF нужны только для встроенного чата в Linux. В настройках чата
пользователь может выбрать **Open chat in my system browser next time**, чтобы
отображать тот же встроенный чат через локальную фоновую службу в системном
браузере, а не во встроенном веб-представлении Godot.

Итоговая структура CEF в Linux должна выглядеть так:

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

Файл `webview/cef/linux-x64/current.json` должен содержать:

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

Файл `webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` должен быть
соответствующим манифестом выпуска для ресурса CEF, например:

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Не размещайте изменяемое состояние браузера внутри каталога версии CEF. При
обычной работе профили и журналы отдельных редакторов записываются в корневые
каталоги кеша и журналов в данных приложения Fennara, а данные среды выполнения
остаются общими и доступными только для чтения.

<a id="5-configure-your-mcp-app"></a>
## 5. Настройте свое приложение MCP

После установки локального пакета среды выполнения настройте приложение MCP:

```bash
fennara mcp-setup --claude
```

Другие целевые приложения:

```bash
fennara mcp-setup --help
```

После настройки перезапустите приложение MCP.

Если вашего приложения нет в списке или вы вручную редактируете конфигурацию
MCP в рамках этой установки, см. [Настройка MCP](mcp-setup.md), где приведены
стабильный путь к программе запуска и примеры JSON/TOML.

Это только подключает внешнее приложение MCP к инструментам Godot в Fennara.
Выбор поставщика модели для встроенной панели чата Fennara при этом не
настраивается. Если нужен встроенный чат, настройте панель в Godot или см.
[Приложения MCP и встроенный чат](chat-vs-mcp.md).

<a id="6-verify"></a>
## 6. Проверьте установку

Откройте проект Godot, затем попросите свое приложение MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Если путь указан верно, ручная установка работает.

<a id="recommended-shortcut"></a>
## Рекомендуемый короткий путь

Даже если вы устанавливаете CLI вручную, можно поручить ему установку дополнения
и локального пакета среды выполнения:

```bash
cd path/to/your-godot-project
fennara install
```

CLI также записывает в проект инструкции для агентов ИИ, работающих с кодом:

```text
AGENTS.md
addons/fennara/ai/
```

Каталог ИИ содержит краткие постоянно читаемые инструкции, указатель и
специализированные страницы, которые загружаются только при необходимости.
Скопированный вручную ZIP-архив дополнения может включать этот упакованный
каталог, но он не создает и не обновляет `AGENTS.md` в корне проекта.
Используйте `fennara install` и `fennara update`, если Fennara должна управлять
полным комплектом инструкций проекта и поддерживать его в актуальном состоянии.
