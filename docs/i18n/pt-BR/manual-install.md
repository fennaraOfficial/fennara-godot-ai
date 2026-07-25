<!-- fennara-i18n: locale=pt-BR source=docs/manual-install.md sha256=3337708611e93975c41085834cec8564108e26bbaa89e7cdc4bd6e824adcf31c -->
<a id="manual-install"></a>
# Instalação manual

<!-- fennara-doc-nav:start -->
[English](../../manual-install.md) · [简体中文](../zh-CN/manual-install.md) · [Español](../es/manual-install.md) · **Português do Brasil** · [日本語](../ja/manual-install.md) · [한국어](../ko/manual-install.md) · [Русский](../ru/manual-install.md) · [Français](../fr/manual-install.md) · [Deutsch](../de/manual-install.md) · [Türkçe](../tr/manual-install.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../manual-install.md)
<!-- fennara-doc-nav:end -->

Use esta página somente quando precisar montar o Fennara sem o fluxo de
configuração do Godot ou `fennara install`.

> [!TIP]
> No Windows e no Linux, a maioria dos usuários deve adicionar `addons/fennara`
> ao projeto, abrir o dock do Fennara e pressionar **Set Up Fennara**. No macOS,
> use a CLI. Consulte [Configuração](setup.md).

> [!IMPORTANT]
> A instalação manual do ZIP do addon não é recomendada no macOS. O addon contém
> uma biblioteca nativa que atualmente não é notarizada pela Apple, e o download
> pelo navegador seguido da extração pelo Finder pode fazer o macOS informar que
> não consegue verificar se `libfennara.macos.editor` está livre de malware.
> Use a [instalação pela CLI](setup.md#instalar-pelo-terminal-recomendado-no-macos)
> para evitar essa notificação. Se ela já aparecer, feche o Godot, remova a pasta
> `addons/fennara/` copiada manualmente e execute `fennara install`.

A instalação manual tem quatro partes: a CLI, o addon do projeto, o pacote de
runtime local compartilhado e a configuração opcional do aplicativo MCP.

<a id="1-download-release-files"></a>
## 1. Baixar os arquivos do lançamento

Abra o lançamento mais recente no GitHub:

https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest

Baixe o manifesto de lançamento, os arquivos da sua plataforma e o zip compartilhado do addon.

| Finalidade | Recurso |
| --- | --- |
| Plano de lançamento e valores SHA-256 | `fennara-release-manifest-v<version>.json` |
| CLI Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip` |
| Runtime local Windows x86_64 | `fennara-release-local-windows-x86_64-v<version>.zip` |
| CLI Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip` |
| Runtime local Linux x86_64 | `fennara-release-local-linux-x86_64-v<version>.zip` |
| Webview incorporada Linux x86_64 | `fennara-webview-cef-linux-x64-<cef-version>.zip` |
| CLI macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip` |
| Runtime local macOS arm64 | `fennara-release-local-macos-arm64-v<version>.zip` |
| Addon versionado para todas as plataformas | `fennara-release-addon-v<version>.zip` |

O lançamento também inclui este alias de nome estável do addon para documentação e downloads manuais:

```text
fennara-addon-latest.zip
```

O manifesto registra o SHA-256 esperado do runtime local, do addon e dos recursos
de runtime compartilhados. Use-o como fonte oficial ao verificar downloads manuais.

<a id="2-install-the-cli"></a>
## 2. Instalar a CLI

Extraia o zip `fennara-cli`.

Adicione seu diretório `bin` ao PATH ou copie o binário `fennara` para uma de suas pastas existentes no PATH.

Verifique:

```bash
fennara --version
fennara doctor
```

<a id="3-install-the-godot-addon"></a>
## 3. Instalar o addon Godot

Extraia o zip `fennara-addon`.

Copie:

```text
addons/fennara
```

para seu projeto Godot, de modo que o projeto contenha:

```text
addons/fennara/fennara.gdextension
```

<a id="4-install-the-local-runtime-package"></a>
## 4. Instalar o pacote de runtime local

Normalmente, a CLI gerencia isso para você. A configuração manual do runtime só é necessária se você estiver evitando `fennara install`.

Pastas de dados padrão do Fennara:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

A estrutura esperada é:

```text
Fennara/
  bin/
    fennara-mcp
    fennara-daemon
  current.json
  versions/
    <version>/
      fennara-mcp-runtime
      fennara-daemon-runtime
      addon/
        addons/
          fennara/
  webview/
    cef/
      linux-x64/
        <cef-version>/
```

No Windows, os binários usam `.exe`.

`current.json` aponta os binários inicializadores para a versão ativa do runtime.
Os comandos normais `fennara install` e `fennara update` criam esse arquivo automaticamente.

O chat incorporado no Linux usa o local compartilhado de runtime
`webview/cef/linux-x64/<cef-version>/`. Execuções normais de `fennara install`
ou `fennara update` instalam automaticamente o runtime CEF gerenciado pelo
lançamento a partir do manifesto e do recurso do lançamento. Se estiver
instalando tudo manualmente, extraia
`fennara-webview-cef-linux-x64-<cef-version>.zip` nesse local compartilhado e
grave o marcador correspondente `webview/cef/linux-x64/current.json`. Mantenha
esse payload fora do addon do projeto Godot. `addons/fennara` não deve conter
`libcef.so` nem outros arquivos de runtime CEF.

Esse payload CEF serve apenas para o chat incorporado no Linux. Os usuários
podem escolher **Open chat in my system browser next time** em Chat Settings
para exibir o mesmo chat integrado pelo daemon local no navegador do sistema,
em vez de usar a webview incorporada ao Godot.

A estrutura final do CEF no Linux deve ser:

```text
~/.local/share/fennara/
  webview/
    cef/
      linux-x64/
        current.json
        <cef-version>/
          fennara-cef-runtime.json
          libcef.so
          fennara_cef_helper
          icudtl.dat
          resources.pak
          locales/
            en-US.pak
```

`webview/cef/linux-x64/current.json` deve ser:

```json
{
  "runtime": "cef",
  "platform": "linux",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "dir": "<cef-version>"
}
```

`webview/cef/linux-x64/<cef-version>/fennara-cef-runtime.json` deve ser o
manifesto de lançamento correspondente do recurso CEF, por exemplo:

```json
{
  "schema_version": 1,
  "runtime": "cef",
  "platform": "linux",
  "arch": "x86_64",
  "platform_arch": "linux-x64",
  "version": "<cef-version>",
  "enabled": true,
  "layout": "webview/cef/linux-x64/<cef-version> with webview/cef/linux-x64/current.json pointing at the selected version",
  "required_files": [
    "libcef.so",
    "fennara_cef_helper",
    "icudtl.dat",
    "resources.pak",
    "chrome_100_percent.pak",
    "chrome_200_percent.pak",
    "v8_context_snapshot.bin",
    "locales/en-US.pak"
  ],
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Não coloque estado gravável do navegador dentro do diretório da versão do CEF.
O uso normal grava perfis e logs por editor nas raízes de cache e logs dos dados
de aplicativo, enquanto o payload de runtime permanece compartilhado e somente leitura.

<a id="5-configure-your-mcp-app"></a>
## 5. Configurar seu aplicativo MCP

Depois de instalar o pacote de runtime local, configure seu aplicativo MCP:

```bash
fennara mcp-setup --claude
```

Outros destinos:

```bash
fennara mcp-setup --help
```

Reinicie o aplicativo MCP após a configuração.

Se o aplicativo não estiver listado ou se você estiver editando manualmente a
configuração MCP como parte desta instalação, consulte
[Configuração de MCP](mcp-setup.md) para ver o caminho estável do inicializador
e exemplos em JSON e TOML.

Isso apenas conecta o aplicativo MCP externo às ferramentas Godot do Fennara.
Não configura o provedor de modelo do dock de chat integrado. Configure o dock
dentro do Godot se quiser usar o chat integrado ou consulte
[Aplicativos MCP e chat integrado](chat-vs-mcp.md).

<a id="6-verify"></a>
## 6. Verificar

Abra o projeto Godot e pergunte ao aplicativo MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Se o caminho estiver correto, a instalação manual está funcionando.

<a id="recommended-shortcut"></a>
## Atalho recomendado

Mesmo que instale a CLI manualmente, você pode deixá-la instalar o addon e o pacote de runtime local:

```bash
cd path/to/your-godot-project
fennara install
```

A CLI também grava orientações de projeto para agentes de programação com IA:

```text
AGENTS.md
addons/fennara/ai/
```

O diretório de IA contém orientações compactas sempre lidas, um índice e páginas
especializadas carregadas apenas quando relevantes. Um ZIP de addon copiado
manualmente pode incluir esse diretório empacotado, mas não cria nem atualiza o
`AGENTS.md` na raiz do projeto. Use `fennara install` e `fennara update` quando
o Fennara precisar gerenciar e atualizar todas as orientações do projeto.
