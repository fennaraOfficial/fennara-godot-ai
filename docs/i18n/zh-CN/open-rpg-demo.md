<!-- fennara-i18n: locale=zh-CN source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Open RPG 演示详解

<!-- fennara-doc-nav:start -->
[文档](README.md)

🌐 **语言**

[English](../../open-rpg-demo.md) · **简体中文** · [Español](../es/open-rpg-demo.md) · [Português do Brasil](../pt-BR/open-rpg-demo.md) · [日本語](../ja/open-rpg-demo.md) · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · [Deutsch](../de/open-rpg-demo.md) · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ 由 AI 根据英文原文撰写，欢迎母语者审阅。 [英文原文](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

视频：

https://www.youtube.com/watch?v=0Egu3S-9MM0

此演示在 GDQuest 的开源 Godot 4 Open RPG 项目上测试 Fennara MCP。

演示的重点并不是 AI 从零创建了一个空白项目，而是一个 AI 智能体在现有 Godot RPG 代码库中工作，犯下错误，收到来自 Godot 的反馈，修补实现，然后继续推进。

<a id="project"></a>
## 项目

GDQuest Godot 4 Open RPG：

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## 任务

添加一项成长功能，让玩家战斗角色 Baloo the Bear 在赢得一场现有遭遇战后，解锁一项名为 Tactical Guard 的新战斗能力。

该能力需要：

- 以一个敌人为目标
- 造成适量伤害
- 提高 Baloo 的 Defense
- 解锁后出现在 Baloo 的战斗行动菜单中
- 解锁后显示类似 `Baloo learned Tactical Guard!` 的消息

<a id="what-happened"></a>
## 发生了什么

一个 AI 编程智能体通过 Fennara MCP 连接到实时 Godot 项目，并检查了项目架构。

它使用 Fennara 工具进行：

- 场景树检查
- 节点属性检查
- GDScript 诊断
- 场景验证
- 运行时错误反馈
- 项目和场景检查

第一次实现并没有完美工作，而这正是有价值的部分。

Fennara 返回了来自 Godot 的反馈，智能体修补了损坏的脚本，调整了实现，并继续推进，直到该功能在游戏中正常工作。

<a id="why-this-matters"></a>
## 为什么这很重要

空白演示很容易。现有项目才是 AI 智能体通常会失败的地方。

Fennara 的核心主张是，Godot AI 智能体需要引擎反馈：

- 脚本是否成功解析？
- 场景是否通过验证？
- 运行时是否发出了错误？
- 智能体是否检查了真实的项目结构？
- 智能体能否修补错误，而不是假装任务已经完成？

传统 MCP 为 AI 提供命令。

Fennara 为 AI 提供来自 Godot 的反馈。
