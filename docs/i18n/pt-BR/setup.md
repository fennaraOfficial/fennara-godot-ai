<!-- fennara-i18n: locale=pt-BR source=docs/setup.md sha256=ab1b11ff7dd3472ab14185e920004b6504fa14eb1c29e7c7b1d7a322780af1dd -->
<a id="setup"></a>
# Configuração

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../setup.md) · [简体中文](../zh-CN/setup.md) · [Español](../es/setup.md) · **Português do Brasil** · [日本語](../ja/setup.md) · [한국어](../ko/setup.md) · [Русский](../ru/setup.md) · [Français](../fr/setup.md) · [Deutsch](../de/setup.md) · [Türkçe](../tr/setup.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../setup.md)
<!-- fennara-doc-nav:end -->

Instale o Fennara, escolha onde deseja conversar e conecte seu projeto Godot.

> [!TIP]
> A maioria dos usuários só precisa adicionar o addon, abrir o dock do Fennara e
> pressionar **Set Up Fennara**. No macOS, use a instalação pela CLI abaixo para
> evitar a notificação de segurança após um ZIP do addon ser baixado manualmente.

<a id="before-you-start"></a>
## Antes de começar

| Requisito | Quando é necessário |
| --- | --- |
| Godot 4.5 ou mais recente | Sempre |
| Windows x86_64, Linux x86_64 ou macOS arm64 | Sempre |
| Um aplicativo de IA compatível com MCP | Apenas para uso MCP externo |
| Uma chave de API na nuvem, Ollama ou LM Studio | Apenas para o chat integrado |
| O SDK .NET disponível como `dotnet` | Apenas para diagnósticos C# e verificação preliminar de runtime |

<a id="install-from-godot"></a>
## Instalar pelo Godot

> [!IMPORTANT]
> No macOS, o addon de lançamento contém uma biblioteca nativa que atualmente
> não é notarizada pela Apple. Baixar o ZIP pelo navegador e extraí-lo
> manualmente pode fazer o macOS informar que não consegue verificar se
> `libfennara.macos.editor` está livre de malware. Use
> [Instalar pelo terminal](#instalar-pelo-terminal-recomendado-no-macos) para evitar essa notificação.

1. Baixe `fennara-addon-latest.zip` do
   [lançamento mais recente](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest)
   e copie `addons/fennara/` para seu projeto.
2. Abra o projeto e selecione o dock do Fennara.
3. Pressione **Set Up Fennara**.

O Fennara instala os componentes locais correspondentes e conecta o projeto
aberto. Se um daemon compartilhado antigo estiver ocioso, a configuração o
interrompe antes de ativar a versão correspondente. Uma troca de versão exige
zero projetos conectados. O projeto em configuração normalmente fica
desconectado enquanto as versões são diferentes. Se a configuração relatar um
projeto conectado, feche todos os outros editores com Fennara e tente novamente.
Se permanecer uma conexão obsoleta do projeto atual, feche e reabra este editor
e tente novamente. Se a configuração falhar, o dock oferece **Retry**,
**Copy Report** e **Open Logs**. Os relatórios copiados são sanitizados e não
incluem chaves de API, conteúdo do chat nem arquivos do projeto.

> [!NOTE]
> O addon permanece no projeto. A CLI, o daemon, o servidor MCP, os logs e o
> runtime compartilhado do navegador ficam nos dados de aplicativo do Fennara fora do projeto.

<a id="install-from-the-terminal-recommended-on-macos"></a>
## Instalar pelo terminal (recomendado no macOS)

A CLI instala o mesmo addon e é o método recomendado no macOS. Ela evita o
caminho de quarentena do navegador e Finder que causa a notificação descrita acima.

Instale a CLI no Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Ou no macOS e Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Depois, execute o Fennara dentro do projeto:

```bash
cd path/to/your-godot-project
fennara install
```

Se já extraiu o addon manualmente no macOS e vê a notificação, feche o Godot e
remova a pasta `addons/fennara/` copiada manualmente antes de executar
`fennara install`. Isso importa porque a CLI preserva um addon completo existente.

Se o projeto já contiver um addon Fennara completo, a CLI o mantém e instala os
componentes locais correspondentes. Caso contrário, também instala o addon do
lançamento atual. Consulte a [referência de instalação da CLI](cli.md#instalar-em-um-projeto)
para fixação de versão e automação.

<a id="choose-how-you-use-fennara"></a>
## Escolher como usar o Fennara

| Caminho | Conta do modelo | Configuração |
| --- | --- | --- |
| Chat integrado | Um provedor conectado em Fennara Chat Settings | [Conectar um provedor](#conectar-o-chat-integrado) |
| Aplicativo MCP externo | A conta ou assinatura do próprio aplicativo | [Conectar um aplicativo MCP](#conectar-um-aplicativo-mcp) |
| Ambos | Cada caminho mantém suas próprias configurações de modelo | Conclua as duas seções |

<a id="connect-the-built-in-chat"></a>
### Conectar o chat integrado

1. Abra **Chat Settings > Chat**.
2. Selecione **Open providers**.
3. Conecte um provedor na nuvem com sua própria chave ou um servidor Ollama ou LM Studio local.
4. Escolha um modelo.

Consulte [Provedores do chat integrado](providers.md) para ver provedores,
chaves, URLs de servidores locais e IDs de modelos. Use `/provider` e `/model`
para as mesmas ações no compositor.

O chat incorporado usa a webview da plataforma:

| Plataforma | Webview |
| --- | --- |
| Windows | Microsoft Edge WebView2 Runtime |
| macOS | WKWebView/WebKit do sistema |
| Linux | Runtime CEF compartilhado gerenciado pelo Fennara |

`fennara install`, `fennara update` e `fennara doctor` verificam esses
pré-requisitos. As ferramentas MCP continuam funcionando se o chat incorporado opcional não iniciar.

Para usar o navegador do sistema, ative **Open chat in my system browser next
time** em Chat Settings e reinicie o Godot. Isso só altera onde o chat aparece.
O provedor, o histórico e a conexão com o projeto permanecem iguais.

Para anexar código à próxima mensagem do chat, selecione o código no editor de
scripts do Godot, abra o menu de contexto e escolha **Add to Chat**.

<a id="connect-an-mcp-app"></a>
### Conectar um aplicativo MCP

Abra **Chat Settings > MCP Apps**, encontre seu aplicativo e pressione **Set Up**.
Reinicie o aplicativo para que ele carregue o Fennara.

Você também pode conectar pelo terminal:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Se o aplicativo não estiver listado, consulte [Configuração de MCP](mcp-setup.md)
para ver todos os destinos e formatos de configuração manual.

Aplicativos MCP externos usam suas próprias contas de modelo. O chat integrado
usa o provedor selecionado em Fennara Chat Settings. Consulte
[Aplicativos MCP e chat integrado](chat-vs-mcp.md) para entender a diferença.

<a id="verify-the-connection"></a>
## Verificar a conexão

Abra o projeto Godot e pergunte ao aplicativo MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Se ele relatar o projeto errado, selecione o destino MCP correto no dock do Fennara.

<a id="update-fennara"></a>
## Atualizar o Fennara

Quando o dock mostrar **Update**, pressione-o e siga as instruções. O Fennara
baixa e verifica a atualização antes de pedir para fechar o Godot. Ele reabre o
mesmo projeto depois da instalação e mantém a versão anterior funcional até
que a atualização seja validada.

Para atualizar pelo terminal, feche o Godot e execute:

```bash
cd path/to/your-godot-project
fennara update
```

> [!IMPORTANT]
> Se estiver atualizando a partir do Fennara v0.3.8 ou anterior, reinstale a CLI
> uma vez com o comando da plataforma acima antes de executar `fennara update`.
> Essas CLIs consultam uma tag desativada e não descobrem os lançamentos atuais.
> Reinstalar a CLI muda as atualizações futuras para o endpoint Latest Release
> do GitHub sem remover o addon nem as configurações do projeto.

> [!IMPORTANT]
> No macOS, reinstale a CLI uma vez antes de atualizar a partir do Fennara v0.3.11.
> Essa CLI rejeita o pacote de framework existente antes de chegar à
> autoatualização. A reinstalação substitui somente a CLI e preserva o addon e as configurações.

Se a validação falhar, use **Restore Previous Version**, **Open Logs** ou
**Copy Report** no dock. Consulte a [referência de atualização da CLI](cli.md#atualizar-um-projeto)
para versões exatas, preparação e recuperação de atualizações interrompidas.

<a id="troubleshooting"></a>
## Solução de problemas

<a id="an-install-or-update-failed"></a>
### Uma instalação ou atualização falhou

Copie o relatório sanitizado do dock ou mostre o mais recente no terminal:

```bash
fennara diagnostics
```

Consulte [Diagnósticos da CLI](cli.md#inspecionar-integridade-e-falhas) para IDs
de operações, saída JSON, campos registrados e garantias de ocultação.

<a id="fennara-is-not-found"></a>
### `fennara` não foi encontrado

Abra um novo terminal e execute:

```bash
fennara doctor
```

Se o comando ainda não estiver disponível, adicione o diretório `bin` do Fennara
ao PATH. A [página de instalação da CLI](cli.md#instalar-a-cli) lista os caminhos das plataformas.

<a id="windows-binaries-fail-before-starting"></a>
### Os binários do Windows falham antes de iniciar

Se um binário do Fennara relatar uma DLL `VCRUNTIME` ou `MSVCP` ausente, código
de saída `-1073741515` ou `0xc0000135`, instale o Microsoft Visual C++ Redistributable 2015-2022 x64:

```text
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

Isso só é necessário em máquinas Windows sem essas DLLs de runtime da Microsoft.

<a id="a-release-requires-a-newer-cli"></a>
### Um lançamento exige uma CLI mais recente

Se a autoatualização da CLI não conseguir instalar a versão necessária, execute
novamente o script em [Instalar a CLI](cli.md#instalar-a-cli) e tente o comando novamente.

<a id="the-addon-is-not-visible-in-godot"></a>
### O addon não aparece no Godot

Confirme que este arquivo existe e reabra o projeto:

```text
addons/fennara/fennara.gdextension
```

<a id="fennarastatus-shows-the-wrong-project"></a>
### `fennara_status` mostra o projeto errado

Abra o projeto desejado e selecione-o com o controle de destino MCP no dock do Fennara.

<a id="c-diagnostics-are-missing"></a>
### Os diagnósticos C# estão ausentes

Confirme que o projeto contém um único `.csproj`, `.sln` ou `.slnx` claro e execute:

```bash
dotnet --version
```

Para estruturas de runtime do navegador, recuperação manual e detalhes de
implementação, consulte [Arquitetura](architecture.md),
[Instalação manual](manual-install.md) e [Perguntas frequentes](faq.md).
