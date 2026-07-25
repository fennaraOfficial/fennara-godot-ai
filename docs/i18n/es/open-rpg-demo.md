<!-- fennara-i18n: locale=es source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Análisis de la demostración Open RPG

<!-- fennara-doc-nav:start -->
[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · **Español** · [Português do Brasil](../pt-BR/open-rpg-demo.md) · [日本語](../ja/open-rpg-demo.md) · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · [Deutsch](../de/open-rpg-demo.md) · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

Video:

https://www.youtube.com/watch?v=0Egu3S-9MM0

Esta demostración prueba Fennara MCP en Open RPG, el proyecto de código abierto para Godot 4 de GDQuest.

Lo importante no es que una IA creara un proyecto vacío desde cero. Lo importante es que un agente de IA trabajó dentro de una base de código existente de un RPG para Godot, cometió errores, recibió información de Godot, corrigió la implementación y continuó.

<a id="project"></a>
## Proyecto

Godot 4 Open RPG de GDQuest:

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## Tarea

Añadir una función de progresión que permita a Baloo, el oso combatiente del jugador, desbloquear una nueva habilidad de combate llamada Tactical Guard después de ganar un encuentro existente.

La habilidad debía:

- tener como objetivo a un enemigo
- infligir daño moderado
- aumentar la defensa de Baloo
- aparecer en el menú de acciones de combate de Baloo después de desbloquearse
- mostrar un mensaje como `Baloo learned Tactical Guard!` después de desbloquearse

<a id="what-happened"></a>
## Qué ocurrió

Un agente de programación con IA se conectó al proyecto de Godot activo mediante Fennara MCP e inspeccionó su arquitectura.

Utilizó herramientas de Fennara para:

- inspeccionar el árbol de escenas
- inspeccionar las propiedades de los nodos
- obtener diagnósticos de GDScript
- validar escenas
- obtener información sobre errores de ejecución
- inspeccionar el proyecto y sus escenas

La primera implementación no funcionó a la perfección. Esa fue la parte útil.

Fennara devolvió información de Godot. El agente corrigió el script roto, ajustó la implementación y continuó hasta que la función se pudo utilizar en el juego.

<a id="why-this-matters"></a>
## Por qué es importante

Las demostraciones con proyectos vacíos son fáciles. Los proyectos existentes son donde los agentes de IA suelen fallar.

La tesis de Fennara es que los agentes de IA para Godot necesitan información del motor:

- ¿Se pudo analizar el script?
- ¿Se validó la escena?
- ¿El runtime emitió algún error?
- ¿El agente inspeccionó la estructura real del proyecto?
- ¿Puede el agente corregir el error en lugar de fingir que la tarea está terminada?

Un MCP tradicional proporciona comandos a una IA.

Fennara proporciona a la IA información procedente de Godot.
