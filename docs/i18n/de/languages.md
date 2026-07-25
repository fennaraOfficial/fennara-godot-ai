<!-- fennara-i18n: locale=de source=docs/languages.md sha256=29ca1071b436e0ff29fa5d18d9e2b09cbe64749513ea7f4e1e6471569fcb6456 -->
<a id="languages-and-translation-status"></a>
# Sprachen und Übersetzungsstatus

<!-- fennara-doc-nav:start -->
[English](../../languages.md) · [简体中文](../zh-CN/languages.md) · [Español](../es/languages.md) · [Português do Brasil](../pt-BR/languages.md) · [日本語](../ja/languages.md) · [한국어](../ko/languages.md) · [Русский](../ru/languages.md) · [Français](../fr/languages.md) · **Deutsch** · [Türkçe](../tr/languages.md)

> ℹ️ Diese Übersetzung wurde von einer KI anhand der englischen Quelle verfasst. Eine Prüfung durch Muttersprachler ist willkommen. [Englische Quelle](../../languages.md)
<!-- fennara-doc-nav:end -->

Englisch ist die kanonische Dokumentationsquelle. Fennara stellt außerdem vollständige,
von KI verfasste Übersetzungen in neun Sprachen bereit. Jede übersetzte Seite verlinkt auf ihre
aktuelle englische Quelle und bittet Muttersprachler um Prüfung.

| Sprache | Dokumentation | Abdeckung | Prüfstatus |
| --- | --- | --- | --- |
| English | [Englische Dokumentation](../../README.md) | 30/30 | Kanonisch |
| 简体中文 | [Chinesische Dokumentation](../zh-CN/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| Español | [Spanische Dokumentation](../es/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| Português do Brasil | [Portugiesische Dokumentation](../pt-BR/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| 日本語 | [Japanische Dokumentation](../ja/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| 한국어 | [Koreanische Dokumentation](../ko/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| Русский | [Russische Dokumentation](../ru/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| Français | [Französische Dokumentation](../fr/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| Deutsch | [Deutsche Dokumentation](README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |
| Türkçe | [Türkische Dokumentation](../tr/README.md) | 30/30 | Prüfung durch Muttersprachler erbeten |

<a id="what-is-translated"></a>
## Was übersetzt wird

Der übersetzte Satz enthält die Haupt-README, jede Seite direkt unter
`docs/`, `CONTRIBUTING.md`, `CONTEXT.md`, `SECURITY.md` und die sechs
für Mitwirkende bestimmten READMEs der Subsysteme.

Rechtstexte, Hinweise Dritter, Issue-Vorlagen, interne Agentenanweisungen,
generierte Projektanweisungen, Test-Fixtures und Herstellerdokumentation bleiben in ihrer
maßgeblichen Form. Generierte oder verhaltenstragende Dateien sind keine
eigenständigen Übersetzungsquellen.

<a id="freshness-and-validation"></a>
## Aktualität und Validierung

Jede übersetzte Seite zeichnet ihren kanonischen Quellpfad und den Quellhash auf.
Die Navigation wird aus einem einzigen Sprachmanifest erzeugt, und stabile englische
Anker-Aliasse sorgen dafür, dass Deep Links auch bei übersetzten Überschriften funktionieren.

Ausführen:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

Diese Werkzeuge übersetzen keine Prosa. Sie pflegen ausschließlich Navigationsmetadaten und
prüfen Abdeckung, Aktualität, Markdown-Struktur, Befehle, Links, Anker, Codeblöcke,
Tabellen und URLs. Korrekturen von Muttersprachlern sind über normale Pull Requests willkommen.

Die normale Synchronisierung behält vorhandene Quellenhashes bei. Eine Änderung
an englischer Prosa lässt ihre Übersetzungen deshalb veraltet, bis sie direkt
aktualisiert werden. Bestätige nach der Prüfung aller neun Übersetzungen einer
geänderten englischen Seite ausschließlich diese Quelle:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

CI führt die Navigationssynchronisierung vor der Strukturvalidierung im
Prüfmodus aus. Dabei wird außerdem überprüft, dass jeder stabile englische Anker
mit der entsprechenden übersetzten Überschrift verbunden bleibt.
