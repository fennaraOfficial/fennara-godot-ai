<!-- fennara-i18n: locale=ru source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# Вспомогательные средства рантайма

<!-- fennara-doc-nav:start -->
[Документация](../README.md)

🌐 **Языки**

[English](../../../../runtime/README.md) · [简体中文](../../zh-CN/contributors/runtime-helpers.md) · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · **Русский** · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

Эта папка содержит исходный код вспомогательных скриптов рантайма на стороне
Godot, используемых инструментами `runtime_session` и `runtime_script`.

Упакованная копия аддона находится по адресу:

```text
godot_demo/addons/fennara/runtime/
```

После изменения файлов здесь выполните:

```bash
node scripts/sync-runtime.mjs
```

Скрипты времени выполнения по-прежнему загружают эти вспомогательные средства
из `res://addons/fennara/runtime/` внутри установленного проекта Godot.
Сохраняйте вспомогательные средства примитивными и независимыми от проекта.
Подходящими задачами являются ввод, ожидание, снимки состояния узлов, захват
изображений, физические запросы и поддержка жизненного цикла сцен. Предположения
о перемещении, боях, заданиях, инвентаре или последовательности интерфейса
конкретной игры не подходят.

`image_sheet.gd` также используется фасадом скрипта снимков экрана. Его
композиция должна быть детерминированной и независимой от состояния сцены,
анимации или игрового процесса.
