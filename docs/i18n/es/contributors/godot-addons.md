<!-- fennara-i18n: locale=es source=godot_demo/addons/README.md sha256=6c9aba0ace26f56a1db6e1a00a27db4dfdc2c8b756eb8679e7caaf22fd15643a -->
<a id="godot-addons"></a>
# Addons de Godot

<!-- fennara-doc-nav:start -->
[English](../../../../godot_demo/addons/README.md) · [简体中文](../../zh-CN/contributors/godot-addons.md) · **Español** · [Português do Brasil](../../pt-BR/contributors/godot-addons.md) · [日本語](../../ja/contributors/godot-addons.md) · [한국어](../../ko/contributors/godot-addons.md) · [Русский](../../ru/contributors/godot-addons.md) · [Français](../../fr/contributors/godot-addons.md) · [Deutsch](../../de/contributors/godot-addons.md) · [Türkçe](../../tr/contributors/godot-addons.md)

> ℹ️ Traducción redactada por IA a partir del original en inglés. Se agradece la revisión de hablantes nativos. [Fuente en inglés](../../../../godot_demo/addons/README.md)
<!-- fennara-doc-nav:end -->

Este directorio reproduce la forma que Godot espera dentro de un proyecto:

```text
res://addons/
  fennara/
```

Mantener el paquete bajo `godot_demo/addons/` permite que los scripts de
empaquetado y pruebas lo copien sin cambiar las rutas.

<a id="current-addon"></a>
## Addon actual

`fennara/` es el addon instalable. Contiene:

- `fennara.gdextension`, punto de entrada de la extensión nativa.
- `bin/`, binarios de editor compilados desde `fennara-cpp/`.
- `dist/`, recursos generados del chat sincronizados desde `ui/chat/`.
- `runtime/`, auxiliares sincronizados desde la fuente `runtime/`.
- `debugger/`, recursos del addon para el depurador.
- `VERSION`, marcador de versión empaquetada.

<a id="rules"></a>
## Reglas

- Mantén estables las rutas relativas. Los proyectos reciben la carpeta como `res://addons/fennara/`.
- No coloques aquí ZIP de prueba o publicación, archivos CEF, registros ni resultados locales.
- No edites manualmente `fennara/dist/` sin sincronizar también el cambio fuente.
- No edites `fennara/runtime/` sin actualizar `runtime/` y ejecutar `node scripts/sync-runtime.mjs`.
- Añade paquetes nuevos solo si deben copiarse en los proyectos.
