<!-- fennara-i18n: locale=ru source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
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
- `POST /tools/call`: пересылает вызов инструмента подключенному плагину Godot и ожидает результат.
- `WS /godot/ws`: локальный мост к плагину Godot. После подключения плагин отправляет сообщение `hello`.

Бинарный файл для разработки:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP-сервер

`crates/fennara-mcp` является локальным MCP-сервером. Он обменивается сообщениями JSON-RPC через stdio, поэтому клиенты MCP могут запускать его как локальный процесс.

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

- `fennara_status`: проверяет, что MCP-сервер установлен и доступен, а затем сообщает состояние демона и моста Godot, если демон работает.
- Инструменты проекта Godot, такие как `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` и `screenshot_scene`, пересылаются
  демону, который передает их подключенному плагину Godot.

Будущий путь установки пользователя в Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
