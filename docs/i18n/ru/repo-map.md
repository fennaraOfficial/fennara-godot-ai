<!-- fennara-i18n: locale=ru source=docs/repo-map.md sha256=a6f2ff719d03e067423cd68229e6941f701a52bfc4ed84361327716d5d9e9c0d -->
<a id="repo-map"></a>
# Карта репозитория

<!-- fennara-doc-nav:start -->
[Документация](README.md)

🌐 **Языки**

[English](../../repo-map.md) · [简体中文](../zh-CN/repo-map.md) · [Español](../es/repo-map.md) · [Português do Brasil](../pt-BR/repo-map.md) · [日本語](../ja/repo-map.md) · [한국어](../ko/repo-map.md) · **Русский** · [Français](../fr/repo-map.md) · [Deutsch](../de/repo-map.md) · [Türkçe](../tr/repo-map.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../repo-map.md)
<!-- fennara-doc-nav:end -->

Это краткая карта для участников и агентов программирования, работающих в этом
репозитории.

<a id="find-the-right-area"></a>
## Поиск нужной области

| Изменение | Основное расположение |
| --- | --- |
| Настройка пользователя или поведение CLI | `local/crates/fennara-cli/` |
| Внешний протокол MCP или схемы | `local/crates/fennara-mcp/`, `local/schemas/tools/` |
| Встроенный чат или поведение фоновой службы | `local/crates/fennara-daemon/` |
| Интеграция с редактором Godot | `fennara-cpp/` |
| Интерфейс чата | `ui/chat/` |
| Вспомогательные сценарии среды выполнения | `runtime/` |
| Упаковка или выпуски | `scripts/`, `.github/workflows/` |
| Пользовательская документация | `README.md`, `docs/` |

<a id="top-level"></a>
## Верхний уровень

| Путь | Отвечает за |
| --- | --- |
| `.github/` | Шаблон pull request, шаблоны задач и процессы GitHub Actions. |
| `docs/` | Документация проекта, руководства по настройке, заметки об архитектуре, примеры, демонстрации и заметки о выпусках. |
| `docs/i18n/` | Манифест локалей и полные деревья переведенной документации. |
| `fennara-cpp/` | Исходный код C++ GDExtension для Godot и точка входа сборки SCons. |
| `godot_demo/addons/fennara/` | Данные устанавливаемого дополнения Godot, копируемые в проекты пользователей. |
| `local/` | CLI на Rust, сервер MCP, фоновая служба, схемы и код локальной среды выполнения. |
| `media/` | Изображения и общедоступные материалы документации. |
| `runtime/` | Исходные вспомогательные сценарии среды выполнения Godot для `runtime_session` и `runtime_script`. |
| `scripts/` | Вспомогательные сценарии управления версиями, упаковки и выпуска. |
| `ui/chat/` | Исходный код необязательного интерфейса веб-чата в редакторе. |
| `local/templates/` | Краткие инструкции проекта и страницы знаний ИИ по запросу, записываемые в проекты Godot командой `fennara install` и обновляемые командой `fennara update`. |
| `local/webview-runtimes/` | Файлы манифестов и конфигурации внешних веб-сред выполнения, устанавливаемых в общие данные приложения Fennara, например данные CEF для Linux. |
| `install.ps1` / `install.sh` | Загрузочные сценарии, устанавливающие CLI Fennara из выпусков GitHub. |
| `VERSION` | Источник истины для версии. |
| `README.md` | Краткий обзор для пользователей и быстрый старт. |
| `docs/README.md` | Указатель документации по задачам. |
| `docs/setup.md` | Пользовательская настройка с дополнения, предварительные условия чата, подключение MCP, процесс обновления и устранение неполадок. |
| `docs/cli.md` | Справочник команд терминала, принадлежащее CLI поведение установки и обновления, восстановление, диагностика, структура данных приложения и рекомендации по автоматизации. |
| `docs/telemetry.md` | Данные анонимной активности, состояние в данных приложения, поведение доставки, определение месячной активности и средства отказа. |
| `CONTRIBUTING.md` | Правила участия. |
| `SECURITY.md` | Политика сообщения об уязвимостях. |
| `LICENSE.md` | Лицензия проекта. |

<a id="local-rust-packages"></a>
## Локальные пакеты Rust

| Путь | Отвечает за |
| --- | --- |
| `local/crates/fennara-cli/` | Команда `fennara`: установка, обновление, самостоятельное обновление CLI, doctor, диагностика операций, проверка предварительных условий веб-представления, поддержка C#, настройка приложений MCP и сгенерированные инструкции проекта. |
| `local/crates/fennara-cli/src/operation.rs` | Общедоступный координатор операций установки и обновления, этапы и точки входа передачи управления CLI. |
| `local/crates/fennara-cli/src/operation/` | Узкие модули журнала операций, надежного хранения, очистки диагностики и тестов. |
| `local/crates/fennara-cli/src/project_addon.rs` | Проверка версии существующего дополнения проекта и библиотеки GDExtension для текущей платформы. |
| `local/crates/fennara-cli/src/prepare_export.rs` | Подготовка экспорта в CI без аддона, которая до запуска Godot удаляет только постоянную автозагрузку среды выполнения Fennara. |
| `local/crates/fennara-cli/src/release_identity.rs` | Стабильный и тестовый идентификатор дополнения, точные селекторы выпуска, проверка каналов pull request и совместимость с прежними стабильными версиями. |
| `local/crates/fennara-cli/src/release_channel.rs` | Проверка указателя тестового канала и разрешение в точный версионный выпуск для каждого канала. |
| `local/crates/fennara-cli/src/release_manifest.rs` | Разбор манифеста выпуска, проверка хешей ресурсов, привязка идентификатора и выбор пакета платформы. |
| `local/crates/fennara-cli/src/release_version.rs` | Общий разбор SemVer в CLI и порядок версий для манифестов и выбора выпуска. |
| `local/crates/fennara-cli/src/existing_addon_install.rs` | Принятие точной версии существующего полного дополнения без замены файлов дополнения проекта. |
| `local/crates/fennara-cli/src/daemon_setup.rs` | Проверка состояния общей фоновой службы, готовности точной версии и запуск, используемые установкой и doctor. |
| `local/crates/fennara-cli/tests/operation_failures.rs` | Тесты сбоев на уровне процесса, долговечной диагностики, очистки данных и закрытого поведения при отказе журнала операций. |
| `local/crates/fennara-cli/src/diagnostics.rs` | Пользовательский доступ к последнему или указанному очищенному отчету об операции. |
| `local/crates/fennara-mcp/` | Локальный сервер MCP stdio и перенаправление схем инструментов. |
| `local/crates/fennara-daemon/` | Локальная фоновая служба для сеансов среды выполнения и работы моста Godot. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs` | Планировщик анонимной ежедневной активности, ограниченная очередь, доставка HTTP и интеграция с жизненным циклом фоновой службы. |
| `local/crates/fennara-daemon/src/runtime_daemon/telemetry/state.rs` | Проверка случайного идентификатора установки, атомарное сохранение в данных приложения, состояние ежедневной квитанции и очистка при отказе. |
| `local/crates/fennara-daemon/src/runtime_daemon/permissions.rs` | Режимы подтверждения встроенного чата, классификация риска инструментов, решения о разрешениях и типы ожидающих запросов подтверждения. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/exec_command.rs` | Принадлежащая фоновой службе реализация `exec_command` встроенного чата: определение оболочки, проверка cwd, запуск процесса, время ожидания и завершение дерева, сбор вывода, журналирование артефакта результата и форматирование результата. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/context_compaction/` | Планировщик сжатия контекста встроенного чата: защита точного хвоста, удаление старых результатов инструментов под давлением в стиле OpenCode, выбор, хранение и воспроизведение фрагментов сводки, сериализация запроса сводки, бюджеты токенов и отображение заполнителей. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/prompt.rs` | PromptBuilder встроенного чата и сгенерированный контекст окружения среды выполнения. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/trace.rs` | Локальный регистратор трассировки встроенного чата, строки событий SQLite, хранение и вспомогательные запросы отладки. |
| `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/` | Примитивы среды поставщиков встроенного чата, каталог и разрешение, обработчики предварительной проверки контекста, нормализованные типы потока и ошибок, а также совместимые с OpenAI или Anthropic адаптеры для OpenAI, Anthropic, OpenRouter, NVIDIA, Ollama Cloud, DeepSeek, Z.AI, Moonshot AI, Kimi For Coding, MiniMax, пользовательских конечных точек, Ollama/local и LM Studio. |
| `local/schemas/tools/` | Общие схемы JSON инструментов. Внешний сервер MCP и встроенный чат встраивают собственные разрешенные подмножества. |
| `local/webview-runtimes/linux-cef.json` | Заполнитель или сгенерированный манифест среды выполнения CEF для Linux, используемый при создании манифеста выпуска, выводе doctor и прежнем резервном поведении. Он описывает структуру общих данных приложения и метаданные архива, не помещая CEF в ZIP-архив дополнения. |
| `local/Cargo.toml` | Конфигурация рабочего пространства Rust. |
| `local/Cargo.lock` | Зафиксированный граф зависимостей Rust. |

<a id="gdextension-source"></a>
## Исходный код GDExtension

| Путь | Отвечает за |
| --- | --- |
| `fennara-cpp/SConstruct` | Точка входа сборки GDExtension. |
| `fennara-cpp/include/` | Общедоступные заголовочные файлы C++. |
| `fennara-cpp/src/` | Реализация C++. |
| `fennara-cpp/src/setup/` | Состояние нативной первоначальной настройки, загрузка CLI по манифесту выпуска, проверка хеша, запуск CLI и чтение хода долговечной операции. |
| `fennara-cpp/src/release/version.cpp` | Нативная проверка SemVer и порядок версий для обнаружения выпуска и обновления. |
| `fennara-cpp/src/release/identity.cpp` | Проверка упакованного стабильного или тестового идентификатора и совместимость с прежними стабильными версиями. |
| `fennara-cpp/src/release/discovery.cpp` | Обнаружение последнего выпуска GitHub и изолированного обновления тестового канала. |
| `fennara-cpp/src/update/` | Координация обновления для точной цели, обнаружение долговечной квитанции, передача управления для закрытия и установки и состояние интерфейса восстановления. |
| `fennara-cpp/src/ui/setup_panel.cpp` | Независимая от веб-представления панель первоначальной настройки с ходом выполнения, повторной попыткой, журналами и действиями с очищенным отчетом. |
| `fennara-cpp/vendor/cef/` | Снимок официальных заголовочных файлов CEF 139 для моста OSR в Linux. Двоичные файлы среды выполнения остаются вне дополнения. |
| `fennara-cpp/src/ui/webview_host*` | Нативный узел веб-представления чата в редакторе и серверные части платформ. |
| `fennara-cpp/src/ui/linux_cef_runtime.*` | Только для Linux: обнаружение общей среды выполнения CEF, проверка маркера и основа динамического загрузчика `libcef.so`. |
| `fennara-cpp/src/ui/linux_cef_osr.*` / `linux_cef_input.*` / `linux_cef_bridge_loader.*` / `linux_cef_bridge_api.hpp` | Только для Linux: поверхность внеэкранного рендеринга CEF, передача ввода Godot, загрузка ABI моста и обновление текстуры Godot для внутреннего веб-представления чата. |
| `fennara-cpp/src/ui/linux_cef_bridge/` | Небольшая библиотека моста только для Linux, собранная из закрепленного официального исходного кода `libcef_dll_wrapper` CEF 139 и адаптера CEF OSR Fennara. Основная GDExtension выполняет ее dlopen после загрузки внешней среды `libcef.so`. |
| `fennara-cpp/src/tools/` | Реализации инструментов на стороне Godot. |
| `fennara-cpp/src/lsp/` | Диагностика сценариев и вспомогательные средства языкового сервера. |
| `fennara-cpp/src/csharp/` | Выбор проекта C# только для сборки, фоновая подготовка, изолированная диагностика и предварительная проверка среды выполнения. |
| `fennara-cpp/src/runtime/` | Нативная поддержка среды выполнения для инструментов, включая предварительную проверку сцены, диагностику сценариев и снимки отладчика. |
| `fennara-cpp/godot-cpp/` | Подмодуль привязок C++ Godot. |

<a id="addon-payload"></a>
## Данные дополнения

| Путь | Отвечает за |
| --- | --- |
| `godot_demo/addons/fennara/fennara.gdextension` | Файл регистрации GDExtension в Godot. |
| `godot_demo/addons/fennara/VERSION` | Версия пакета дополнения. |
| `godot_demo/addons/fennara/release.json` | Упакованный стабильный или тестовый идентификатор, включая точную версию, тег выпуска, канал и исходный коммит тестовой сборки. |
| `godot_demo/addons/fennara/bin/` | Собранные библиотеки платформ. |
| `godot_demo/addons/fennara/dist/` | Упакованные ресурсы веб-интерфейса для веб-представления чата в редакторе. |
| `godot_demo/addons/fennara/runtime/` | Синхронизированная упакованная копия `runtime/`, поставляемая внутри дополнения. |
| `godot_demo/tests/first_run_setup_test.gd` | Тест нативного состояния первоначальной настройки и детерминированного сбоя в автономном режиме. |
| `godot_demo/tests/export_plugin_test.gd` | Регрессионный тест нативного исключения при экспорте и восстановления автозагрузки в автономном режиме. |
| `godot_demo/tests/screenshot_scene_contract_test.gd` | Регрессионный тест контракта аргументов нативного снимка экрана в автономном режиме. |
| `godot_demo/tests/image_sheet_test.gd` | Регрессионный тест общей композиции листа снимков экрана и среды выполнения в автономном режиме. |
| `godot_demo/tests/runtime_image_context_test.gd` | Регрессионный тест необработанного кадра среды выполнения, листа и произвольного вывода Image в автономном режиме. |

<a id="runtime-helper-source"></a>
## Исходный код вспомогательных средств среды выполнения

| Путь | Отвечает за |
| --- | --- |
| `runtime/game_capture_helper.gd` | Точка входа вспомогательного средства среды выполнения, загружаемая GDExtension для сеансов сцен и проверок среды. |
| `runtime/image_label.gd` | Краткие детерминированные подписи, наносимые на составленные ячейки Image после захвата. |
| `runtime/image_sheet.gd` | Общая композиция листа только из Image для контекстов снимка экрана и сценария среды выполнения. |
| `runtime/screenshot_script_context.gd` | Общедоступный фасад сценария снимка экрана, добавляющий общую композицию Image в нативный контекст захвата. |
| `runtime/runtime_script_context.gd` | Общедоступная поверхность вспомогательного объекта `ctx` для `runtime_script`, включая необработанные кадры, композицию и вывод Image, ожидания, ввод, снимки, условия, трассировку лучей и щелчки. |
| `runtime/runtime_input_driver.gd` | Низкоуровневый драйвер событий ввода среды для клавиш, кнопок мыши, абсолютного и относительного движения мыши, модификаторов и очистки ввода. |
| `runtime/runtime_node_snapshot.gd` | Поиск узлов среды выполнения, проверка существования, безопасные при устаревших ссылках снимки, чтение свойств и сводки дочерних узлов. |
| `runtime/runtime_physics_query.gd` | Точные помощники трассировки лучей и сканирования 2D/3D с краткими квитанциями попаданий. |
| `runtime/runtime_query_utils.gd` | Общие утилиты запросов среды для преобразования векторов, безопасного разрешения узлов и путей, идентификации объектов и общего сопоставления целей. |
| `runtime/runtime_capture_store.gd` | Средство записи артефактов захвата и состояния среды, используемое сеансами, сценариями и проверками окружения. |
| `runtime/runtime_check_runner.gd` | Средство выполнения проверок среды для спецификаций неинтерактивного выполнения сцен. |

<a id="scripts-and-workflows"></a>
## Сценарии и процессы

| Путь | Отвечает за |
| --- | --- |
| `scripts/set-version.mjs` | Обновляет версионные файлы по всему репозиторию. |
| `scripts/check-version.mjs` | Проверяет синхронизацию версий. |
| `scripts/release-identity.mjs` | Общая проверка и генерация Node для идентификатора выпуска SemVer и указателей тестовой сборки каждого PR. |
| `scripts/release-policy.mjs` | Политика минимальной совместимой опубликованной версии CLI для стабильных и тестовых манифестов выпуска. |
| `scripts/staging-candidate.mjs` | Доверенное создание идентификатора тестового кандидата и решения о монотонном указателе каждого PR. |
| `scripts/staging-*-validation.mjs` / `scripts/staging-validation-files.mjs` | Узкая проверка тестового дополнения, архива, манифеста, общей файловой системы и пакета публикации. |
| `scripts/validate-staging-build.mjs` / `scripts/validate-staging-publish-bundle.mjs` | Строгие точки входа проверки для недоверенных результатов сборки и доверенного пакета публикации. |
| `scripts/check-staging-channel-advance.mjs` | Применяет проверки монотонности и происхождения до продвижения указателя тестового канала. |
| `scripts/verify-published-assets.mjs` / `scripts/smoke-public-release.mjs` | Проверяют байты опубликованных ресурсов и поведение общедоступной загрузки до продвижения указателя. |
| `scripts/test-run-scene-edit-script-inspect.mjs` | Создает игнорируемый временный проект Godot и дымовым тестом проверяет чтение импортированного `PackedScene` через GDExtension редактора. |
| `scripts/release-targets.mjs` | Определяет поддерживаемые целевые платформы выпуска и имена упакованных ресурсов. |
| `scripts/write-staging-candidate.mjs` / `scripts/write-staging-pointer.mjs` | Записывают зафиксированный идентификатор кандидата и его небольшой указатель канала. |
| `scripts/sync-chat-ui.mjs` | Копирует исходный код не требующего сборки интерфейса чата в данные дополнения. |
| `scripts/sync-runtime.mjs` | Копирует исходный код вспомогательных средств из корня репозитория в данные дополнения. |
| `scripts/sync-doc-navigation.mjs` | Добавляет навигацию документации, хеши источников и стабильные якоря без перевода текста. |
| `scripts/check-doc-i18n.mjs` / `scripts/doc-i18n-lib.mjs` | Проверяют полноту и актуальность переводов, структуру Markdown, URL и ссылки. |
| `scripts/package-preview.mjs` | Формирует предварительные или выпускные ZIP-архивы дополнения, CLI и локальной среды после сборок платформ. |
| `scripts/prepare-linux-cef-runtime.mjs` | Подготавливает отдельный ZIP-архив среды CEF для Linux x64, удаляет отладочные символы из подготовленных ELF, проверяет обязательные файлы и может записать сгенерированный манифест выпуска. |
| `scripts/prepare-linux-cef-sdk.mjs` | Скачивает и извлекает закрепленный официальный минимальный SDK CEF 139 для Linux для сборок CI, которым нужен исходный код оболочки `libcef_dll/`. |
| `scripts/check-linux-cef-runtime-release.mjs` | Проверяет ресурс среды выполнения CEF для Linux по сгенерированному манифесту `local/webview-runtimes/linux-cef.json`. |
| `scripts/write-release-manifest.mjs` | Записывает и проверяет `fennara-release-manifest-v<version>.json` по ресурсам выпуска, включая хеши локального пакета, дополнения и общей среды выполнения. |
| `scripts/cef/linux/fennara_cef_helper.cpp` | Исходный код минимального подпроцесса CEF для Linux, упакованного в отдельный ZIP-архив среды CEF. |
| `.github/workflows/version-check.yml` | Проверка согласованности версии. |
| `.github/workflows/gdextension-build.yml` | Проверка сборки GDExtension для разных платформ и нативный тест состояния первоначальной настройки Windows в автономном режиме. |
| `.github/workflows/local-build.yml` | Проверка сборки локального пакета Rust. |
| `.github/workflows/package-preview.yml` | Ручные предварительные артефакты пакета, включая тестовый артефакт среды CEF для Linux для дымовой проверки чата в Linux. |
| `.github/workflows/release.yml` | Ручная публикация выпуска GitHub, включая создание среды CEF для Linux, генерацию манифеста выпуска и финальную проверку ресурсов. |
| `.github/workflows/staging-release.yml` | Ручная тестовая сборка точного SHA, пробный запуск только с проверкой, публикация точного предварительного выпуска и продвижение указателя каждого PR. |

<a id="where-to-change-things"></a>
## Где вносить изменения

| Задача | Начните здесь |
| --- | --- |
| Добавить или изменить инструмент Godot | `fennara-cpp/src/tools/` и `local/schemas/tools/` |
| Изменить текст схемы MCP | `local/schemas/tools/` |
| Изменить `fennara install` или `fennara update` | `local/crates/fennara-cli/src/`; нативная подготовка и отделенное применение или откат принадлежат `release_update.rs`, `update_stage.rs`, `update_stage/` и `update_apply/` |
| Изменить команды CLI или поведение терминала | `local/crates/fennara-cli/src/` и `docs/cli.md` |
| Изменить ход нативного обновления, подтверждение завершения, рукопожатие активации или восстановление | `fennara-cpp/src/update/`, `fennara-cpp/src/ui/update_panel.cpp`, `fennara-cpp/src/ui/dock.cpp`, `local/crates/fennara-daemon/src/runtime_daemon/chat/mod.rs` и `ui/chat/` |
| Изменить нативную первоначальную настройку или загрузку CLI | `fennara-cpp/src/setup/`, `fennara-cpp/src/ui/setup_panel.cpp` и `fennara-cpp/src/ui/dock.cpp` |
| Изменить исключение аддона во время экспорта | `fennara-cpp/src/ui/export_plugin.cpp`, `fennara-cpp/include/fennara/ui/export_plugin.hpp` и `godot_demo/tests/export_plugin_test.gd` |
| Изменить журналы операций установки и обновления, этапы, коды ошибок или диагностические отчеты | `local/crates/fennara-cli/src/operation.rs`, `local/crates/fennara-cli/src/operation/` и `local/crates/fennara-cli/src/diagnostics.rs` |
| Изменить проверки предварительных условий веб-представления | `local/crates/fennara-cli/src/webview_prereq.rs`, `local/crates/fennara-cli/src/webview_runtime.rs` и `fennara-cpp/src/ui/webview_host*` |
| Изменить сгенерированные инструкции проекта | `local/templates/` и `local/crates/fennara-cli/src/project_guidance.rs` |
| Синхронизировать сгенерированные инструкции демонстрационного дополнения | `local/templates/fennara-guidelines.md`, `local/templates/fennara-ai/`, `scripts/sync-guidance.mjs` и `godot_demo/addons/fennara/ai/` |
| Изменить настройку приложения MCP | `local/crates/fennara-cli/src/mcp_setup.rs` и `docs/mcp-setup.md` |
| Изменить поведение процесса или журнала сеанса среды выполнения | `local/crates/fennara-daemon/src/runtime_daemon/runtime_sessions.rs`, `local/crates/fennara-daemon/src/runtime_daemon/runtime_log.rs`, `fennara-cpp/src/tools/runtime_session/` и `fennara-cpp/src/tool_results/` |
| Изменить помощники ctx для `runtime_script`, ввод, снимки, ожидания, трассировку лучей, захваты или очистку | `runtime/`, `scripts/sync-runtime.mjs`, `godot_demo/addons/fennara/runtime/`, `local/schemas/tools/runtime_script.json` и `docs/tools.md` |
| Изменить интерфейс чата в редакторе, команды с косой чертой или выбор модели и поставщика | `ui/chat/`, `godot_demo/addons/fennara/dist/`, `fennara-cpp/src/ui/dock.cpp` и `fennara-cpp/src/ui/webview_host*` |
| Изменить поставщиков встроенного чата | `local/crates/fennara-daemon/src/runtime_daemon/chat/providers/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/models.rs`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs` и `ui/chat/` |
| Изменить поля анонимной телеметрии, планирование или средства конфиденциальности | `local/crates/fennara-daemon/src/runtime_daemon/telemetry.rs`, `local/crates/fennara-daemon/src/runtime_daemon/telemetry/`, `local/crates/fennara-daemon/src/runtime_daemon/chat/settings.rs`, `ui/chat/` и `docs/telemetry.md` |
| Изменить сторонние библиотеки интерфейса чата | `ui/chat/vendor/`, `godot_demo/addons/fennara/dist/vendor/` и `THIRD_PARTY_NOTICES.md` |
| Изменить поддержку C# | `fennara-cpp/src/csharp/`, `fennara-cpp/include/fennara/csharp/`, схемы инструментов C# и инструкции |
| Изменить пакеты выпуска, политику минимальной версии CLI или самостоятельное обновление CLI | `local/crates/fennara-cli/src/release_manifest.rs`, `local/crates/fennara-cli/src/release_client.rs`, `local/crates/fennara-cli/src/release_package.rs`, `local/crates/fennara-cli/src/self_update.rs`, `scripts/package-preview.mjs`, `scripts/release-policy.mjs`, `scripts/write-release-manifest.mjs` и `.github/workflows/release.yml` |
| Повысить версию | `node scripts/set-version.mjs <version>` |
| Обновить настройку или документацию по чату и MCP, поставщикам или командам с косой чертой | `README.md`, `docs/mcp-setup.md`, `docs/chat-vs-mcp.md`, `docs/providers.md`, `docs/slash-commands.md`, `docs/setup.md`, `docs/faq.md`, `docs/manual-install.md`, `docs/tools.md`, `docs/examples.md` и `llms.txt` |
| Обновить переводы документации | Каноническая английская страница, `docs/i18n/languages.json`, соответствующие страницы локалей, `scripts/sync-doc-navigation.mjs` и `scripts/check-doc-i18n.mjs` |

<a id="notes"></a>
## Примечания

- Поддерживайте этот файл в актуальном состоянии при добавлении или перемещении
  крупных областей исходного кода.
- Храните этапы выпуска в [release.md](release.md).
- Храните этапы настройки в [setup.md](setup.md).
- Храните описание поведения команд терминала в [cli.md](cli.md).
