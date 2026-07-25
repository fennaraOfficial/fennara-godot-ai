<!-- fennara-i18n: locale=de source=docs/open-rpg-demo.md sha256=e624caff078f8baa85d367191103518527e376606bdb3fa7fc5fbf4d4026752d -->
<a id="open-rpg-demo-breakdown"></a>
# Aufschlüsselung der Open-RPG-Demo

<!-- fennara-doc-nav:start -->
[Dokumentation](README.md)

🌐 **Sprachen**

[English](../../open-rpg-demo.md) · [简体中文](../zh-CN/open-rpg-demo.md) · [Español](../es/open-rpg-demo.md) · [Português do Brasil](../pt-BR/open-rpg-demo.md) · [日本語](../ja/open-rpg-demo.md) · [한국어](../ko/open-rpg-demo.md) · [Русский](../ru/open-rpg-demo.md) · [Français](../fr/open-rpg-demo.md) · **Deutsch** · [Türkçe](../tr/open-rpg-demo.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../open-rpg-demo.md)
<!-- fennara-doc-nav:end -->

Video:

https://www.youtube.com/watch?v=0Egu3S-9MM0

Diese Demo testet Fennara MCP mit dem quelloffenen Godot-4-Projekt Open RPG von GDQuest.

Der Kern der Demo besteht nicht darin, dass eine KI ein leeres Projekt von Grund auf erstellt hat. Entscheidend ist, dass ein KI-Agent innerhalb einer vorhandenen Godot-RPG-Codebasis gearbeitet, Fehler gemacht, Feedback von Godot erhalten, die Implementierung korrigiert und weitergearbeitet hat.

<a id="project"></a>
## Projekt

GDQuest Godot 4 Open RPG:

https://github.com/gdquest-demos/godot-open-rpg

<a id="task"></a>
## Aufgabe

Füge eine Fortschrittsfunktion hinzu, durch die Baloo, der spielbare Bärenkämpfer, nach dem Sieg in einer vorhandenen Begegnung eine neue Kampffähigkeit namens Tactical Guard freischaltet.

Die Fähigkeit musste:

- auf einen Gegner zielen
- mäßigen Schaden verursachen
- Baloos Defense erhöhen
- nach der Freischaltung in Baloos Kampfaktionsmenü erscheinen
- nach der Freischaltung eine Nachricht wie `Baloo learned Tactical Guard!` anzeigen

<a id="what-happened"></a>
## Was geschah

Ein KI-Coding-Agent verband sich über Fennara MCP mit dem laufenden Godot-Projekt und untersuchte die Projektarchitektur.

Er verwendete Fennara-Werkzeuge für:

- Untersuchung des Szenenbaums
- Untersuchung von Knoteneigenschaften
- GDScript-Diagnosen
- Szenenvalidierung
- Feedback zu Laufzeitfehlern
- Projekt- und Szeneninspektion

Die erste Implementierung funktionierte nicht perfekt. Genau das war der nützliche Teil.

Fennara gab Feedback aus Godot zurück, der Agent korrigierte das defekte Skript, passte die Implementierung an und arbeitete weiter, bis die Funktion im Spiel funktionierte.

<a id="why-this-matters"></a>
## Warum das wichtig ist

Leere Demos sind einfach. Bei vorhandenen Projekten scheitern KI-Agenten normalerweise.

Fennaras These lautet, dass Godot-KI-Agenten Feedback von der Engine benötigen:

- Wurde das Skript geparst?
- Wurde die Szene validiert?
- Hat die Laufzeit einen Fehler ausgegeben?
- Hat der Agent die tatsächliche Projektstruktur untersucht?
- Kann der Agent den Fehler korrigieren, anstatt vorzugeben, die Aufgabe sei erledigt?

Traditionelles MCP gibt einer KI Befehle.

Fennara gibt der KI Feedback aus Godot.
