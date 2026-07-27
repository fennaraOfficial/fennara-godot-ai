<!-- fennara-i18n: locale=pt-BR source=README.md sha256=bb9720891f1a14c9d6ae542665829e5a6d736f56c0b4afd6160890b8efba398a -->
<a id="fennara-godot-ai"></a>
# Fennara Godot AI

<!-- fennara-doc-nav:start -->
[English](README.md) · [简体中文](README.zh-CN.md) · [Español](README.es.md) · **Português do Brasil** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Русский](README.ru.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Türkçe](README.tr.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](README.md)
<!-- fennara-doc-nav:end -->

[![Discord](https://img.shields.io/badge/Discord-Join%20Fennara-5865F2?logo=discord&logoColor=white)](https://discord.com/invite/3fF4ft9PTk)
[![Demos](https://img.shields.io/badge/Demos-See%20all-red?logo=youtube&logoColor=white)](docs/i18n/pt-BR/demos.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

Usado por desenvolvedores e equipes Godot, incluindo a [Somni Game Studios](https://somnigamestudios.com/).

O Fennara oferece aos assistentes de IA uma conexão ativa com o Godot. Use-o em aplicativos compatíveis com MCP, como Codex, Claude, Cursor, Gemini e Antigravity, ou no dock de chat opcional dentro do editor.

Os agentes podem inspecionar cenas, verificar scripts, capturar telas, ler erros de execução e validar alterações dentro do editor, em vez de apenas deduzir o estado do projeto a partir dos arquivos.

<table>
  <tr>
    <td width="46%">
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">
        <img src="https://i.ytimg.com/vi/2vSYP7GyA5U/hqdefault.jpg" alt="Comparação do Fennara com outros MCPs para Godot" width="100%" />
      </a>
    </td>
    <td>
      <strong>Assista à demonstração em destaque</strong><br />
      Comparação do Fennara com outros MCPs para Godot.<br />
      <a href="https://www.youtube.com/watch?v=2vSYP7GyA5U">Reproduzir este vídeo</a><br />
      <a href="docs/i18n/pt-BR/demos.md">Ver todos os vídeos de demonstração</a>
    </td>
  </tr>
</table>

<a id="what-it-does"></a>
## O que ele faz

- expõe ferramentas com conhecimento do Godot a aplicativos externos de IA por meio de MCP
- adiciona um dock de chat local opcional dentro do editor Godot
- retorna feedback real do Godot: árvores de cena, diagnósticos, capturas de tela, logs de execução e resultados de validação
- mantém o agente responsável perante o editor aberto, e não apenas o sistema de arquivos

Aplicativos MCP externos e o chat integrado usam configurações de modelo separadas. Consulte [Aplicativos MCP e chat integrado](docs/i18n/pt-BR/chat-vs-mcp.md) e [Provedores do chat integrado](docs/i18n/pt-BR/providers.md).

<a id="requirements"></a>
## Requisitos

- Godot 4.5 ou mais recente.
- Um sistema operacional desktop compatível: Windows x86_64, Linux x86_64 ou macOS arm64.
- Um aplicativo de programação compatível com MCP apenas se você quiser usar o Fennara a partir do Claude, Codex, Cursor, Gemini, Antigravity ou outro aplicativo externo de IA.
- Um provedor de chat apenas se você quiser usar o dock de chat integrado do Fennara. Pode ser uma chave de provedor na nuvem ou um provedor local, como Ollama ou LM Studio.

Para ver o passo a passo completo da instalação, consulte [Configuração](docs/i18n/pt-BR/setup.md).

<a id="what-setup-adds"></a>
## O que a configuração adiciona

- o addon Fennara mantido em `res://addons/fennara/`
- uma pequena CLI `fennara` instalada nos dados de aplicativo do Fennara
- um servidor MCP local usado por aplicativos de programação com IA
- um daemon local que conecta solicitações de MCP e chat ao editor Godot aberto
- orientações de projeto geradas para agentes de IA

O dock de chat integrado usa a webview da plataforma: Microsoft Edge WebView2 no Windows, WKWebView/WebKit no macOS e um runtime CEF compartilhado gerenciado pelo Fennara no Linux. As ferramentas MCP continuam funcionando se o dock de chat opcional não puder iniciar.

<a id="install"></a>
## Instalação

No Windows e no Linux, escolha a instalação pelo addon ou pela CLI. No macOS,
use a instalação pela CLI abaixo se quiser evitar a notificação de segurança do
macOS que pode aparecer depois de baixar e extrair manualmente o arquivo ZIP do addon.

<a id="add-the-addon-to-your-project"></a>
### Adicionar o addon ao projeto

- Abra o [lançamento mais recente](https://github.com/fennaraOfficial/fennara-godot-ai/releases/latest), baixe `fennara-addon-latest.zip` e extraia a pasta `addons/fennara/` para o seu projeto.

Abra o projeto, selecione o dock do Fennara e pressione **Set Up Fennara**.

O Fennara é uma dependência do editor, não uma dependência do runtime do jogo.
Durante a exportação, o plugin do editor remove seu autoload de runtime do
projeto exportado e ignora `res://addons/fennara/` e `res://.fennara/`. O
projeto do editor é restaurado depois que a exportação termina. Se um checkout
de CI excluir o addon por meio do `.gitignore`, execute
`fennara prepare-export --project path/to/project` antes de iniciar o Godot ou
instale o addon nesse checkout. O Godot valida os caminhos de autoload antes
que os plugins de exportação possam ser executados, portanto essa preparação
precisa ocorrer primeiro.

> **macOS:** O addon do lançamento contém uma biblioteca nativa que atualmente
> não é notarizada pela Apple. Se você baixar o ZIP do addon pelo navegador e
> extraí-lo manualmente, o macOS poderá informar que não consegue verificar se
> `libfennara.macos.editor` está livre de malware. Para evitar essa notificação,
> use a instalação pela CLI abaixo. Se a notificação já aparecer, feche o Godot,
> remova a pasta `addons/fennara/` copiada manualmente e instale o Fennara com a CLI.

<a id="install-with-the-cli-recommended-on-macos"></a>
### Instalar com a CLI (recomendado no macOS)

A CLI instala o mesmo addon Fennara. Esse é o método de instalação recomendado
no macOS, pois evita o caminho de quarentena do navegador e do Finder que causa
a notificação descrita acima.

Instale a CLI no Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

Ou no macOS e Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Depois, execute o Fennara a partir do seu projeto Godot:

```bash
cd path/to/your-godot-project
fennara install
```

Consulte [Configuração](docs/i18n/pt-BR/setup.md) para solucionar problemas e
[CLI do Fennara](docs/i18n/pt-BR/cli.md) para ver a referência completa de comandos.

<a id="set-up-a-provider-or-connect-an-mcp-app"></a>
## Configurar um provedor ou conectar um aplicativo MCP

<a id="built-in-chat"></a>
### Chat integrado

Abra **Chat Settings > Chat**, selecione **Open providers** e conecte um provedor.
O Fennara usa sua própria chave para provedores na nuvem (BYOK). Você também pode
usar um servidor Ollama ou LM Studio local. Consulte a [lista de provedores compatíveis](docs/i18n/pt-BR/providers.md).

<a id="mcp-apps"></a>
### Aplicativos MCP

Abra **Chat Settings > MCP Apps**, encontre seu aplicativo e pressione **Set Up**.

Você também pode conectar um aplicativo pelo terminal:

```bash
fennara mcp-setup --codex
fennara mcp-setup --help
```

Se o seu aplicativo MCP não estiver listado em Chat Settings, consulte
[Configuração de MCP](docs/i18n/pt-BR/mcp-setup.md) para ver a lista completa de
aplicativos e as instruções de configuração manual.

<a id="update"></a>
## Atualização

Quando o dock do Fennara mostrar **Update**, pressione-o e siga as instruções.

> **Atualização a partir do Fennara v0.3.8 ou anterior:** Reinstale a CLI uma vez
> com o comando de instalação da plataforma acima antes de executar `fennara update`.
> Essas versões da CLI resolvem uma tag de lançamento desativada e não conseguem
> descobrir os lançamentos atuais. Reinstalar a CLI faz com que as atualizações
> futuras usem o endpoint Latest Release do GitHub e não remove o addon nem as
> configurações existentes do projeto.

> **Usuários do macOS atualizando a partir do Fennara v0.3.11:** Reinstalem a CLI
> uma vez com o comando de instalação do macOS acima antes de atualizar. A CLI
> v0.3.11 rejeita o pacote de framework existente do macOS antes que possa se
> autoatualizar. A reinstalação substitui apenas a CLI. Ela não remove o addon
> nem as configurações do projeto.

Para atualizar pelo terminal, feche o Godot e execute:

```bash
cd path/to/your-godot-project
fennara update
```

Consulte [Atualizar o Fennara](docs/i18n/pt-BR/setup.md#atualizar-o-fennara) para recuperação e diagnósticos.

<a id="tools"></a>
## Ferramentas

O Fennara expõe um pequeno conjunto de ferramentas com conhecimento do Godot:

- gravar ou atualizar arquivos do projeto e retornar diagnósticos
- executar scripts pontuais de edição de cenas
- inspecionar árvores de cena, nós, recursos e classes do Godot
- validar cenas
- capturar telas
- iniciar sessões de execução e ler logs de execução
- executar pequenos scripts de runtime em uma cena ativa

O objetivo não é substituir as ferramentas normais de arquivo de um agente. O Fennara oferece o ciclo de feedback do Godot que estava faltando.

<a id="privacy"></a>
## Privacidade

O Fennara envia no máximo um evento anônimo de instalação ativa por dia UTC
depois que o Godot se conecta. Ele contém um UUID aleatório da instalação, as
versões do Fennara e do Godot, o sistema operacional e a arquitetura da CPU.
Não contém dados ou caminhos do projeto, prompts, atividade de ferramentas,
logs, capturas de tela nem informações de contas.

A telemetria pode ser desativada em **Chat Settings > Chat > Anonymous telemetry**,
com `FENNARA_DISABLE_TELEMETRY=true` ou com `DO_NOT_TRACK=1`. Consulte
[Telemetria anônima](docs/i18n/pt-BR/telemetry.md) para ver o payload completo,
o armazenamento, o transporte e o contrato de desativação.

<a id="demos"></a>
## Demonstrações

Assista a uma demonstração prática do Fennara:

[![This Godot Plugin Revolutionizes AI Game Development Forever](https://i.ytimg.com/vi/pijlHyiOnz4/hqdefault.jpg)](https://www.youtube.com/watch?v=pijlHyiOnz4&t=22s)

Mais vídeos:

- [I Gave Codex an AI Game Image and It Built This in Godot](https://www.youtube.com/watch?v=ztbH6zBhxMc)
- [Fennara MCP Builds a Katamari-Style Godot Game](https://www.youtube.com/watch?v=8y2Ub8pgNSs)
- [This Godot Plugin Transforms AI Game Development Forever](https://www.youtube.com/watch?v=wKln8248y2M)

Consulte [Demonstrações](docs/i18n/pt-BR/demos.md) para mais vídeos do canal do Fennara.

<a id="star-history"></a>
## Histórico de estrelas
<a href="https://github.com/fennaraOfficial/fennara-godot-ai/stargazers">
  <img alt="Gráfico do histórico de estrelas" src="https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/star-history/star-history.svg" width="700">
</a>

<a id="documentation"></a>
## Documentação

| Comece por... | Quando precisar de... |
| --- | --- |
| [Página inicial da documentação](docs/i18n/pt-BR/README.md) | Todos os guias e páginas de referência |
| [Configuração](docs/i18n/pt-BR/setup.md) | Instalação, atualizações e solução de problemas |
| [Provedores de chat](docs/i18n/pt-BR/providers.md) | Modelos e chaves do chat integrado |
| [Configuração de MCP](docs/i18n/pt-BR/mcp-setup.md) | Codex, Claude, Cursor e outros aplicativos MCP |
| [Ferramentas](docs/i18n/pt-BR/tools.md) | O feedback do Godot disponível aos agentes |
| [Telemetria anônima](docs/i18n/pt-BR/telemetry.md) | Dados coletados, comportamento de envio e controles de desativação |
| [Como contribuir](docs/i18n/pt-BR/CONTRIBUTING.md) | Orientações para desenvolvimento e pull requests |

<a id="community"></a>
## Comunidade

Perguntas, ajuda com configuração e feedback inicial são bem-vindos no Discord:

https://discord.com/invite/3fF4ft9PTk

<a id="license"></a>
## Licença

Consulte [LICENSE.md](LICENSE.md).
