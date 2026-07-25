<!-- fennara-i18n: locale=tr source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
<a id="fennara-local-tools"></a>
# Fennara Yerel Araçları

<!-- fennara-doc-nav:start -->
[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · [Português do Brasil](../../pt-BR/contributors/local-tools.md) · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · **Türkçe**

> ℹ️ Bu çeviri İngilizce kaynak temel alınarak yapay zeka tarafından yazılmıştır. Ana dil konuşurlarının incelemesi memnuniyetle karşılanır. [İngilizce kaynak](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Bu klasör yerel Fennara bileşenlerini içerir.

<a id="daemon"></a>
## Daemon

`crates/fennara-daemon`, yerel Fennara daemon'unu şu adreste çalıştırır:

```text
http://127.0.0.1:41287
```

Uç noktalar:

- `GET /health`: daemon durumu.
- `GET /status`: daemon durumu ve bağlı Godot eklentisi meta verileri.
- `POST /tools/call`: bir araç çağrısını bağlı Godot eklentisine iletir ve araç sonucunu bekler.
- `WS /godot/ws`: yerel Godot eklentisi köprüsü. Eklenti bağlandıktan sonra bir `hello` mesajı gönderir.

Geliştirme ikilisi:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## MCP Sunucusu

`crates/fennara-mcp` yerel MCP sunucusudur. MCP istemcilerinin yerel bir işlem olarak başlatabilmesi için stdio üzerinden JSON-RPC konuşur.

`fennara-mcp`, seçtiği MCP'ye yönelik şemaları derleme zamanında `local/schemas/tools/` içinden gömer ve bu araç çağrılarını yerel daemon'a iletir. Çalışma zamanında harici bir şema hizmetine ihtiyaç duymaz. Yerleşik sohbet aynı şema dizininden ilişkili, ancak farklı bir araç kümesi seçer.

`fennara install` ayrıca `local/templates/` içinden oluşturulan proje yönergelerini Godot projesine yazar:

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

Derleme:

```powershell
cd local
cargo build
```

Windows'ta bir terminal Rust PATH değerini henüz yenilemediyse:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Geliştirme ikilisi:

```text
local/target/debug/fennara-mcp.exe
```

Geçerli araçlar:

- `fennara_status`: MCP sunucusunun kurulu ve erişilebilir olduğunu doğrular, ardından daemon çalışıyorsa daemon ve Godot köprüsü durumunu bildirir.
- `write_or_update_file`, `run_scene_edit_script`, `get_scene_tree`, `script_diagnostics` ve `screenshot_scene` gibi Godot proje araçları daemon'a, oradan da bağlı Godot eklentisine iletilir.

Windows'ta daha sonra kurulan kullanıcı yolu:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
