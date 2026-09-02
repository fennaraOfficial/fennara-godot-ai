<!-- fennara-i18n: locale=ru source=local/README.md sha256=29a4563cb548ac4612f1881d66af9e72f4de9b1c118920e0d14ba00d0279edec -->
<a id="fennara-local-tools"></a>
# Локальные инструменты Fennara

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · **Русский** · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Эта папка содержит локально-нативные компоненты Fennara.

<a id="daemon"></a>
## Демон

`crates/fennara-daemon` запускает локальный демон Fennara по адресу:

```text
http://127.0.0.1:41287
```

Конечные точки:

- `GET /health`: состояние работоспособности демона.
- `GET /status`: состояние демона и метаданные подключенного плагина Godot.
- `POST /status/bound`: привилегированное привязанное состояние. Сопоставляет канонический Project Root одного MCP-
  процесса с подключенными сеансами редактора Godot.
- `POST /tools/call`: пересылает вызов инструмента подключенному плагину Godot и ожидает результат.
- `WS /godot/ws`: локальный мост к плагину Godot. После подключения плагин отправляет сообщение `hello`.

Один демон используется всеми редакторами с Fennara и внешними MCP-процессами текущего пользователя.
Привязанные внешние запросы маршрутизируются по каноническому Project Root; внутренние запросы
встроенного чата остаются привязанными к Godot Editor Session, а устаревшие непривязанные MCP-запросы
используют выбранную на панели цель совместимости.

Демон также владеет одним общим для компьютера Runtime Slot. Владение Runtime Session и состояние возобновляемой
аренды связаны с Project Root, поэтому редактор может подключиться заново, не передавая управление.

Бинарный файл для разработки:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP-сервер

`crates/fennara-mcp` является локальным MCP-сервером. Он обменивается сообщениями JSON-RPC через stdio, поэтому клиенты MCP могут запускать его как локальный процесс.

Каждый MCP-процесс при запуске фиксирует одну необязательную Project Binding. Порядок выбора:
`--project-path`, затем `FENNARA_PROJECT_PATH`, затем ближайший предок каталога запуска с `project.godot`. Если
проект не найден, автоматически включается устаревший непривязанный режим совместимости; недопустимый явный
путь завершает запуск с ошибкой. Для межпроектной изоляции используйте по одному MCP-процессу и
подключению на проект.

`crates/fennara-project-identity` совместно используется средой MCP и демоном. Он отвечает за поиск,
проверку, канонизацию, преобразование без потерь для протокола и сравнение Project Root в активной
файловой системе.

`fennara-mcp` во время сборки встраивает выбранные схемы, предназначенные для MCP,
из `local/schemas/tools/` и пересылает вызовы этих инструментов локальному демону.
Во время выполнения внешний сервис схем ему не требуется. Встроенный чат выбирает
связанный, но отличающийся набор инструментов из того же каталога схем.

Команда `fennara install` также записывает сформированные инструкции проекта из `local/templates/` в проект Godot:

```text
AGENTS.md
addons/fennara/ai/
  guidelines.md
  index.md
  visual-observation.md
  runtime-observation.md
  operations.md
  clients/cursor.md
```

Сборка:

```powershell
cd local
cargo build
```

В Windows, если терминал еще не обновил PATH для Rust:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Бинарный файл для разработки:

```text
local/target/debug/fennara-mcp.exe
```

Текущие инструменты:

- `fennara_status`: проверяет, что MCP-сервер установлен и доступен, а затем, если демон работает, сообщает
  режим маршрутизации, источник и корень привязки, состояние выбранного редактора и готовность моста Godot.
- Инструменты проекта Godot, такие как `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` и `screenshot_scene`, пересылаются
  демону, который передает их подключенному плагину Godot.

Будущий путь установки пользователя в Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
