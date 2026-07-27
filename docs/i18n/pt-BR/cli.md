<!-- fennara-i18n: locale=pt-BR source=docs/cli.md sha256=16441a0d18c69d735854b2f54a905e9d7f5277a8eae9a9c89eced18cfcaca06a -->
<a id="fennara-cli"></a>
# CLI do Fennara

<!-- fennara-doc-nav:start -->
[English](../../cli.md) · [简体中文](../zh-CN/cli.md) · [Español](../es/cli.md) · **Português do Brasil** · [日本語](../ja/cli.md) · [한국어](../ko/cli.md) · [Русский](../ru/cli.md) · [Français](../fr/cli.md) · [Deutsch](../de/cli.md) · [Türkçe](../tr/cli.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../cli.md)
<!-- fennara-doc-nav:end -->

Use a CLI quando preferir o terminal, precisar de diagnósticos ou recuperação,
ou quiser uma instalação automatizada com uma versão exata.

> [!TIP]
> A CLI é o método de instalação recomendado no macOS. Ela evita a notificação
> de segurança do macOS que pode ocorrer quando um ZIP do addon baixado pelo
> navegador é extraído manualmente e sua biblioteca nativa herda a quarentena do Finder.

<a id="common-flow"></a>
## Fluxo comum

```bash
cd path/to/your-godot-project
fennara install
```

Use `fennara doctor` quando precisar inspecionar ou reparar a instalação local.

Use [Configuração](setup.md) para o percurso normal no Godot. Mantenha esta
página como referência dos comandos de terminal.

<a id="install-the-cli"></a>
## Instalar a CLI

Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
```

macOS e Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.sh | sh
```

Se um addon extraído manualmente no macOS já acionar uma notificação para
`libfennara.macos.editor`, feche o Godot e remova a pasta `addons/fennara/`
copiada manualmente antes de executar `fennara install`. Caso contrário, a CLI
preserva um addon completo existente.

Abra um novo terminal se `fennara` não estiver disponível imediatamente e depois
verifique a instalação:

```bash
fennara --version
fennara doctor
```

A CLI é instalada por usuário. Os addons do projeto permanecem dentro de seus
projetos Godot. Inicializadores compartilhados, runtimes versionados, registros
de operações, logs e o CEF do Linux ficam nos dados de aplicativo do Fennara:

```text
Windows: %LOCALAPPDATA%\Fennara
macOS: ~/Library/Application Support/Fennara
Linux: ~/.local/share/fennara
```

<a id="command-summary"></a>
## Resumo dos comandos

| Comando | Finalidade |
| --- | --- |
| `fennara install` | Instalar ou adotar um addon do projeto e seus componentes locais correspondentes |
| `fennara update` | Atualizar um projeto e seus componentes locais |
| `fennara doctor` | Inspecionar ou reparar a instalação local |
| `fennara diagnostics` | Mostrar um relatório sanitizado de uma operação |
| `fennara mcp-setup` | Conectar um aplicativo MCP externo |
| `fennara prepare-export` | Remover o autoload do Fennara antes de uma exportação de CI sem o addon |
| `fennara recover` | Restaurar uma atualização nativa interrompida |
| `fennara self-update` | Atualizar apenas a CLI instalada |

Execute `fennara --help` para ver o resumo dos comandos instalados. Use
`fennara mcp-setup --help` para ver os destinos de aplicativos MCP compatíveis.

<a id="install-a-project"></a>
## Instalar em um projeto

Execute dentro de uma pasta que contenha `project.godot`:

```bash
fennara install
```

Ou identifique explicitamente o projeto:

```bash
fennara install --project path/to/project
```

Sem `--version`, a CLI seleciona o manifesto do lançamento atual. Use um
lançamento exato quando a reprodutibilidade for importante:

```bash
fennara install --project path/to/project --version <version>
```

A instalação tem dois caminhos seguros:

- Se não houver um addon completo, a CLI baixa e verifica o lançamento
  selecionado, instala `addons/fennara`, instala os componentes locais
  correspondentes e grava as orientações de projeto do Fennara.
- Se já houver um addon completo, a CLI lê seu `VERSION`, valida a biblioteca
  da plataforma atual e instala os componentes gerenciados pela CLI dessa
  versão exata. Ela não altera o addon do projeto. Um `--version` explícito
  deve corresponder ao addon existente.

Nas instalações de um lançamento, a CLI primeiro resolve a solicitação para
uma versão exata, atualiza a CLI do Fennara instalada quando esse lançamento
oferece uma versão mais nova e depois continua a instalação com a CLI
substituta. Instalações locais com `--source` não acessam o serviço de
lançamentos nem fazem atualização automática.

<a id="prepare-an-addon-free-ci-export"></a>
## Preparar uma exportação de CI sem o addon

Se `addons/fennara/` estiver excluído de um checkout de CI, remova o autoload
persistente de runtime do Fennara antes de iniciar o Godot:

```bash
fennara prepare-export --project path/to/project
godot --headless --path path/to/project --export-release "Preset"
```

O comando altera somente a entrada `_fennara_game_capture` em `project.godot`.
Ele preserva os demais autoloads e configurações e pode ser executado novamente
com segurança. Essa etapa precisa ocorrer antes de iniciar o Godot porque a
inicialização do projeto valida os caminhos de autoload antes que plugins do
editor ou de exportação possam ser executados. Como alternativa, a CI pode
instalar o addon do Fennara antes de iniciar o Godot.

<a id="update-a-project"></a>
## Atualizar um projeto

Para uma atualização normal pelo terminal, feche o Godot desse projeto e execute:

```bash
fennara update --project path/to/project
```

Sem `--version`, a CLI lê a identidade do addon instalado. Addons estáveis
resolvem o Latest release do GitHub, enquanto addons de staging resolvem apenas
seu canal `pr-<number>`. O seletor é imediatamente fixado em uma versão exata,
inclusive durante a substituição automática da CLI. Em seguida, a CLI verifica
os recursos do lançamento, atualiza o addon e os componentes locais versionados,
atualiza as orientações do projeto e verifica o pré-requisito de webview da
plataforma. Use `--version <version>` para selecionar explicitamente um lançamento exato.

`--no-self-update` destina-se à automação controlada ou à continuação depois
que a CLI já foi substituída. Não o use para contornar o requisito de versão
mínima da CLI de um lançamento.

> [!IMPORTANT]
> Se estiver atualizando a partir do Fennara v0.3.8 ou anterior, reinstale a CLI
> uma vez com o comando de instalação da plataforma em [Configuração](setup.md#instalar-pelo-terminal-recomendado-no-macos)
> antes de executar `fennara update`. Essas CLIs consultam uma tag de lançamento
> desativada e não conseguem descobrir os lançamentos atuais. Reinstalar a CLI
> não remove o addon nem as configurações do projeto.

> [!IMPORTANT]
> No macOS, reinstale a CLI uma vez antes de atualizar a partir do Fennara v0.3.11.
> Essa CLI rejeita o pacote de framework existente antes de chegar à
> autoatualização. A reinstalação substitui apenas a CLI e preserva o addon e
> as configurações do projeto.

<a id="prepare-while-godot-is-open"></a>
### Preparar enquanto o Godot está aberto

O botão de atualização dentro do editor usa o formato de preparação:

```bash
fennara update --prepare --project path/to/project
```

A preparação baixa, verifica e coloca o addon em staging de forma durável. Ela
não fecha o Godot, não substitui o addon ativo, não troca o manifesto de runtime
ativo nem reinicia o daemon. O dock do Godot observa o recibo da operação e
pede confirmação ao usuário antes de iniciar a etapa destacada de fechar,
substituir, reabrir e validar. O dock passa a versão exata que já descobriu,
portanto o movimento de um ponteiro não pode alterar uma atualização em andamento.

O Fennara oferece suporte a uma única versão ativa do runtime compartilhado.
A ativação é bloqueada se outro editor Godot com Fennara permanecer conectado
ao daemon compartilhado. Feche o outro editor e tente novamente. A versão local
anterior e o ponteiro do runtime continuam disponíveis para recuperação sem
acesso à rede.

`--prepare` é uma primitiva de baixo nível para a integração com o Godot.
Usuários do terminal normalmente usam `fennara update` com o Godot já fechado.

<a id="recover-an-interrupted-update"></a>
## Recuperar uma atualização interrompida

Se o addon atualizado não conseguir carregar o suficiente para mostrar o painel
de recuperação, feche o Godot e execute:

```bash
fennara recover --project path/to/project
```

A CLI restaura apenas operações em um estado recuperável. Ela restaura o addon
anterior, os inicializadores compartilhados e o manifesto de runtime ativo, e
então tenta reabrir o executável do Godot registrado. Selecione uma transação
específica quando o suporte fornecer o ID da operação:

```bash
fennara recover --project path/to/project --operation <operation-id>
```

Operações concluídas, apenas preparadas ou já revertidas são rejeitadas.

<a id="inspect-health-and-failures"></a>
## Inspecionar integridade e falhas

`doctor` relata a plataforma detectada, a estrutura dos dados de aplicativo, a
versão ativa, os inicializadores, os runtimes, o estado do daemon e o
pré-requisito da webview:

```bash
fennara doctor
```

Se ele relatar um daemon ou runtime MCP em execução mais antigo que `current.json`,
reinicie o Godot ou o aplicativo MCP afetado para que ele inicie o runtime selecionado.

Use `--repair` para recriar diretórios-base ausentes nos dados de aplicativo.
No Linux, ele também limpa perfis obsoletos de processos CEF e repara o marcador
do runtime atual quando já houver um runtime gerenciado completo instalado:

```bash
fennara doctor --repair
```

Operações de instalação, atualização, recuperação e autoatualização gravam
estado e eventos duráveis. Mostre o relatório sanitizado mais recente com:

```bash
fennara diagnostics
```

Para uma operação anterior ou saída legível por máquina:

```bash
fennara diagnostics --operation <operation-id>
fennara diagnostics --operation <operation-id> --json
```

Os relatórios incluem códigos de erro estáveis, fases, versões de componentes,
nomes dos recursos selecionados e resultados da verificação de hashes. Eles
ocultam caminhos de projeto, diretório pessoal e dados de aplicativo do Fennara,
credenciais, bearer tokens e consultas de URLs. Não incluem mensagens de chat,
chaves de provedores nem conteúdo de arquivos do projeto.

<a id="configure-an-external-mcp-app"></a>
## Configurar um aplicativo MCP externo

O dock de chat do Godot expõe esses comandos em **Chat Settings > MCP Apps**.
O botão Set Up pede ao daemon local que invoque a CLI instalada, portanto os
fluxos do dock e do terminal usam a mesma implementação de configuração e backup.

Execute `fennara mcp-setup --help` para escolher um destino compatível. Reinicie
o aplicativo MCP depois de alterar sua configuração. Esse comando conecta um
aplicativo externo ao servidor MCP do Fennara. Ele não seleciona o provedor de
modelo usado pelo dock de chat integrado do Godot. [Configuração de MCP](mcp-setup.md)
define a lista de destinos, os locais de configuração e os exemplos de configuração manual.

<a id="update-only-the-cli"></a>
## Atualizar apenas a CLI

Atualizações normais de projeto cuidam automaticamente da autoatualização da CLI.
Para atualizar apenas a CLI instalada:

```bash
fennara self-update
fennara self-update --version <version>
```

Sem `--version`, a autoatualização preserva a faixa da instalação ativa: stable
usa o Latest release do GitHub, e staging usa apenas seu canal de PR registrado.

Staging nunca passa automaticamente para stable. Para sair de staging de forma
deliberada, feche o Godot e execute `fennara update --version <stable-version> --project <path>`.
Esse lançamento stable exato é validado antes que a versão ativa compartilhada mude.

Use esse comando quando o suporte solicitar ou quando uma atualização do projeto
informar que a CLI instalada é antiga demais para continuar com segurança.

<a id="automation-guidance"></a>
## Orientações para automação

- Passe `--project` em vez de depender do diretório atual.
- Fixe `--version` quando uma compilação precisar ser reproduzível.
- Preserve o ID da operação e o caminho do log impressos em caso de falha.
- Use `fennara diagnostics --operation <id> --json` para relatórios estruturados.
- Não edite `current.json`, diretórios de versões, recibos de atualização ou
  pastas de addons em staging manualmente.
- Não execute uma atualização normal que substitua o addon enquanto o projeto
  estiver aberto no Godot. Use o fluxo de atualização dentro do editor ou feche
  primeiro o Godot.
