<!-- fennara-i18n: locale=ru source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Аддоны Godot

<!-- fennara-doc-nav:start -->
[Документация](../README.md)

🌐 **Языки**

[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · [Español](../../es/contributors/godot-addons.md) · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · **Русский** · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ Перевод написан ИИ на основе английского оригинала. Приветствуется проверка носителями языка. [Источник на английском](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

Этот каталог повторяет структуру, которую Godot ожидает внутри проекта:

```text
res://addons/
  fennara/
```

Размещение полезной нагрузки репозитория в `godot_demo/addons/` позволяет скриптам упаковки и локального тестирования копировать аддон в проект без изменения структуры путей.

<a id="current-addon"></a>
## Текущий аддон

`fennara/` является устанавливаемым аддоном Fennara Godot AI. Он содержит:

- `fennara.gdextension`, точку входа Godot для нативного расширения.
- `bin/`, бинарные файлы редактора для платформ, собранные из `fennara-cpp/`.
- `dist/`, сформированные ресурсы webview нативного чата, синхронизированные из `ui/chat/`.
- `runtime/`, синхронизированные вспомогательные скрипты на стороне Godot из исходного каталога `runtime/` в корне репозитория.
- `debugger/`, предназначенные для отладчика ресурсы аддона.
- `VERSION`, маркер версии упакованного аддона.

<a id="rules"></a>
## Правила

- Сохраняйте стабильность относительных путей аддона. Пользовательские проекты получают эту папку как `res://addons/fennara/`.
- Не помещайте сюда ZIP-файлы package preview, ZIP-файлы выпусков, загруженные архивы CEF, журналы или результаты локального тестирования.
- Не редактируйте вручную сформированные файлы webview в `fennara/dist/`, если только вы намеренно не исправляете сформированный результат и затем также не синхронизируете изменение исходника.
- Не редактируйте вручную синхронизированные вспомогательные файлы рантайма в `fennara/runtime/`, не обновив также `runtime/` и не выполнив `node scripts/sync-runtime.mjs`.
- Добавляйте сюда новые полезные нагрузки аддона только в том случае, если они предназначены для копирования в проекты Godot.
