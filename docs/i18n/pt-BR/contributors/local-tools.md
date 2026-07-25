<!-- fennara-i18n: locale=pt-BR source=local/README.md sha256=a7dee6dc27d357ae479c13a0f950aa2664f2e7548f09f7623bbff0e07a49ad50 -->
<a id="fennara-local-tools"></a>
# Ferramentas locais do Fennara

<!-- fennara-doc-nav:start -->
[Documentação](../README.md)

🌐 **Idiomas**

[English](../../../../local/README.md) · [简体中文](../../zh-CN/contributors/local-tools.md) · [Español](../../es/contributors/local-tools.md) · **Português do Brasil** · [日本語](../../ja/contributors/local-tools.md) · [한국어](../../ko/contributors/local-tools.md) · [Русский](../../ru/contributors/local-tools.md) · [Français](../../fr/contributors/local-tools.md) · [Deutsch](../../de/contributors/local-tools.md) · [Türkçe](../../tr/contributors/local-tools.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../../local/README.md)
<!-- fennara-doc-nav:end -->

Esta pasta contém componentes locais nativos do Fennara.

<a id="daemon"></a>
## Daemon

`crates/fennara-daemon` executa o daemon local do Fennara em:

```text
http://127.0.0.1:41287
```

Endpoints:

- `GET /health`: integridade do daemon.
- `GET /status`: status do daemon e metadados dos plugins Godot conectados.
- `POST /tools/call`: encaminha uma chamada de ferramenta ao plugin Godot conectado e aguarda um resultado.
- `WS /godot/ws`: ponte local do plugin Godot. O plugin envia uma mensagem `hello` depois de se conectar.

Binário de desenvolvimento:

```text
local/target/debug/fennara-daemon.exe
```

<a id="mcp-server"></a>
## Servidor MCP

`crates/fennara-mcp` é o servidor MCP local. Ele fala JSON-RPC via stdio para que clientes MCP possam iniciá-lo como processo local.

`fennara-mcp` incorpora, durante a compilação, os esquemas selecionados voltados
ao MCP de `local/schemas/tools/` e encaminha essas chamadas de ferramentas ao
daemon local. Ele não precisa de um serviço externo de esquemas em tempo de
execução. O chat integrado seleciona um conjunto relacionado, mas diferente,
de ferramentas no mesmo diretório de esquemas.

`fennara install` também grava no projeto Godot orientações geradas a partir de
`local/templates/`:

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

Compilação:

```powershell
cd local
cargo build
```

No Windows, se um terminal ainda não tiver atualizado o PATH do Rust:

```powershell
cd local
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
```

Binário de desenvolvimento:

```text
local/target/debug/fennara-mcp.exe
```

Ferramentas atuais:

- `fennara_status`: verifica se o servidor MCP está instalado e acessível, e então relata o status do daemon e da ponte do Godot quando o daemon está em execução.
- Ferramentas de projeto Godot, como `write_or_update_file`, `run_scene_edit_script`,
  `get_scene_tree`, `script_diagnostics` e `screenshot_scene`, são encaminhadas
  ao daemon, que as encaminha ao plugin Godot conectado.

Futuro caminho instalado do usuário no Windows:

```text
%LOCALAPPDATA%\Fennara\bin\fennara-mcp.exe
```
