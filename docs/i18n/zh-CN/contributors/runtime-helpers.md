<!-- fennara-i18n: locale=zh-CN source=runtime/README.md sha256=34a99b8c10136827a2142e78d2517579a3b11f0c2449f668aa667ee728fa5bbf -->
<a id="runtime-helpers"></a>
# 运行时辅助脚本

<!-- fennara-doc-nav:start -->
[English](../../../../runtime/README.md) · **简体中文** · [Español](../../es/contributors/runtime-helpers.md) · [Português do Brasil](../../pt-BR/contributors/runtime-helpers.md) · [日本語](../../ja/contributors/runtime-helpers.md) · [한국어](../../ko/contributors/runtime-helpers.md) · [Русский](../../ru/contributors/runtime-helpers.md) · [Français](../../fr/contributors/runtime-helpers.md) · [Deutsch](../../de/contributors/runtime-helpers.md) · [Türkçe](../../tr/contributors/runtime-helpers.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../../../runtime/README.md)
<!-- fennara-doc-nav:end -->

此文件夹是 `runtime_session` 和 `runtime_script` 使用的 Godot 端运行时辅助脚本源代码。

打包后的插件副本位于：

```text
godot_demo/addons/fennara/runtime/
```

编辑这里的文件后运行：

```bash
node scripts/sync-runtime.mjs
```

在已安装的 Godot 项目中，运行时脚本仍会从 `res://addons/fennara/runtime/` 加载这些辅助文件。请保持辅助功能原始且与项目无关。输入、等待、节点快照、捕获、物理查询和场景生命周期支持都适合放在这里，针对具体游戏的移动、战斗、任务、物品栏或 UI 流程假设则不适合。

截图脚本外观也使用 `image_sheet.gd`。请让它的组合过程保持确定性，并与场景、动画或游戏状态无关。
