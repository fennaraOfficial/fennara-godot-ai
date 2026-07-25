<!-- fennara-i18n: locale=ru source=godot_demo/README.md sha256=07f441ca3fe31dececc487571c165f3613da42dc04d1cc5f81be7fe40243f2f6 -->
<a id="godot-payload"></a>
# Полезная нагрузка Godot

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/README.md) · [简体中文](../../zh-CN/contributors/godot-payload.md) · [Español](../../es/contributors/godot-payload.md) · [Português do Brasil](../../pt-BR/contributors/godot-payload.md) · [日本語](../../ja/contributors/godot-payload.md) · [한국어](../../ko/contributors/godot-payload.md) · **Русский** · [Français](../../fr/contributors/godot-payload.md) · [Deutsch](../../de/contributors/godot-payload.md) · [Türkçe](../../tr/contributors/godot-payload.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../../../godot_demo/README.md)
<!-- fennara-doc-nav:end -->

Этот каталог является деревом исходного кода предназначенной для Godot полезной нагрузки аддона, которая копируется в проекты пользователей и упаковывается в архивы выпусков.

```text
godot_demo/
  addons/
    fennara/
```

Каталог `godot_demo/addons/fennara/` должен оставаться пригодным для установки в качестве обычного каталога аддона Godot. Все добавленное сюда должно быть пригодно для непосредственного размещения в пользовательском проекте по пути `res://addons/fennara/`.

<a id="what-belongs-here"></a>
## Что должно находиться здесь

- `addons/fennara/fennara.gdextension` и файлы `.uid`, загружаемые Godot.
- Бинарные файлы GDExtension для редактора в `addons/fennara/bin/`, создаваемые сборками для платформ.
- Сформированные ресурсы веб-чата в `addons/fennara/dist/`, используемые нативным webview чата.
- Синхронизированные вспомогательные скрипты рантайма на стороне Godot в `addons/fennara/runtime/`, исходники которых находятся в `runtime/`.
- Файл `addons/fennara/VERSION`, соответствующий файлу `VERSION` репозитория во время упаковки.

<a id="what-does-not-belong-here"></a>
## Что не должно находиться здесь

- Локальное пользовательское состояние Godot, такое как `.godot/`, `.import/`, журналы, временные файлы или кеши редактора.
- Корневые результаты упаковки из рабочих процессов. Они должны находиться в игнорируемых каталогах сборки, таких как `dist/` или `.package-preview/`.
- Общие локальные компоненты рантайма, такие как исполняемые файлы демона Fennara и MCP или рантайм CEF для Linux. CLI устанавливает их в каталог данных приложения Fennara пользователя, они не копируются в аддон каждого проекта Godot.

<a id="generated-files"></a>
## Сформированные файлы

Исходный код интерфейса чата находится в `ui/chat/`. После его изменения выполните:

```powershell
node scripts\sync-chat-ui.mjs
```

Эта команда синхронизирует собранные файлы webview с `godot_demo/addons/fennara/dist/`. Этот каталог намеренно хранится в репозитории, поскольку пользователям аддона не должны требоваться Node.js или этап сборки фронтенда.

Исходный код вспомогательных средств рантайма находится в `runtime/`. После его изменения выполните:

```powershell
node scripts\sync-runtime.mjs
```

Эта команда синхронизирует вспомогательные средства рантайма на стороне Godot с `godot_demo/addons/fennara/runtime/`. Этот каталог намеренно хранится в репозитории, поскольку пользователи аддона должны получать эти скрипты в ZIP-файле выпуска.
