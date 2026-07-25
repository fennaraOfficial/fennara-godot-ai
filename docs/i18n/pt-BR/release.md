<!-- fennara-i18n: locale=pt-BR source=docs/release.md sha256=23e966649d82728c9ab9802dd2957bb64c1b13fd3e7ae44bd513835bc063ff77 -->
<a id="release-process"></a>
# Processo de lançamento

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../release.md) · [简体中文](../zh-CN/release.md) · [Español](../es/release.md) · **Português do Brasil** · [日本語](../ja/release.md) · [한국어](../ko/release.md) · [Русский](../ru/release.md) · [Français](../fr/release.md) · [Deutsch](../de/release.md) · [Türkçe](../tr/release.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../release.md)
<!-- fennara-doc-nav:end -->

Os lançamentos são manuais. Não publique a partir de workflows de pull request.

> [!IMPORTANT]
> Execute os lançamentos a partir de `main`, mantenha `VERSION` e a entrada do workflow idênticos e
> decida explicitamente se o lançamento precisa de uma versão mínima da CLI mais alta.

<a id="release-at-a-glance"></a>
## Visão geral do lançamento

| Etapa | Resultado |
| --- | --- |
| Preparar e mesclar a alteração de versão | As fontes de versão do repositório concordam |
| Executar Package Preview | Artefatos com o formato de lançamento são gerados sem publicação |
| Inspecionar a prévia | Arquivos compactados, manifesto, hashes e layout do CEF no Linux são verificados |
| Executar Release a partir de `main` | A tag e o GitHub Release são publicados |
| Fazer teste rápido de instalação e atualização | O fluxo público do usuário é verificado |

<a id="versioning"></a>
## Versionamento

`VERSION` é a fonte da verdade.

As ferramentas de lançamento aceitam valores SemVer. Lançamentos estáveis usam `X.Y.Z`. Candidatos
de staging usam uma pré-versão isolada por pull request, como
`1.2.3-pr.101.2`, em que `pr-101` é o canal de staging e `2` é o número do
candidato nesse canal.

Para incrementar a versão do repositório:

```bash
node scripts/set-version.mjs X.Y.Z
```

O script atualiza:

- `VERSION`
- `godot_demo/addons/fennara/VERSION`
- constantes de versão do plugin
- a versão do pacote do workspace Rust em `local/`
- `local/Cargo.lock`

O addon também contém `addons/fennara/release.json`. A identidade estável é
gravada automaticamente pelo comando normal acima. Um workspace de build de staging
usa as entradas de identidade explícitas:

```bash
node scripts/set-version.mjs 1.2.3-pr.101.2 \
  --track staging \
  --channel pr-101 \
  --source-commit <full-commit-sha>
```

A versão de staging, o canal, o commit de origem e a tag exata do lançamento devem
corresponder. Um addon de pré-lançamento sem essa identidade é rejeitado. Addons estáveis
existentes anteriores ao `release.json` continuam usando a trilha estável por padrão.

Verifique a sincronização da versão:

```bash
node scripts/check-version.mjs
```

<a id="1-prepare-the-release-commit"></a>
## 1. Preparar o commit de lançamento

1. Execute o script de versão.
2. Revise o diff.
3. Execute verificações locais que correspondam à área alterada.
4. Mescle o PR de preparação do lançamento em `main`.

Verificações comuns:

```bash
node scripts/check-version.mjs
cd local
cargo test --locked
```

Para alterações na GDExtension, também faça o build local do addon quando possível:

```bash
cd fennara-cpp
scons platform=windows target=editor
```

<a id="2-run-package-preview"></a>
## 2. Executar Package Preview

Use isto antes da publicação quando o empacotamento tiver mudado ou quando você quiser uma execução de teste.

GitHub:

```text
Actions > Package Preview > Run workflow
```

O workflow gera pacotes para Windows, Linux e macOS e envia artefatos temporários.
Ele não cria tags, GitHub Releases nem `latest`.

Package Preview reproduz as partes não relacionadas à publicação do Release com fidelidade suficiente para
exercitar o empacotamento do lançamento antes da mesclagem:

- sincroniza a UI de chat sem etapa de build e o código-fonte dos auxiliares de runtime com o payload do addon
- gera o zip do runtime CEF para Linux
- grava o manifesto gerado do runtime CEF para Linux
- fornece esse manifesto gerado aos builds de pacotes de plataforma
- monta o arquivo compactado do addon para todas as plataformas
- renomeia os pacotes local/addon para os nomes de ativos gerenciados pelo manifesto de lançamento
- valida o ativo do runtime CEF para Linux em relação ao manifesto gerado
- grava `fennara-release-manifest-v<version>.json`
- envia um artefato `fennara-package-preview-release-assets` contendo os
  zips com formato de lançamento e o manifesto

Os artefatos da prévia são úteis para verificar o conteúdo dos zips e o formato do manifesto antes
da publicação. Eles são artefatos do Actions, não ativos públicos de lançamento.

<a id="3-run-release"></a>
## 3. Executar Release

Execute o workflow manual de lançamento a partir de `main`:

```text
Actions > Release > Run workflow
```

Entradas:

```text
version: X.Y.Z
promote_latest: true
```

A entrada `version` deve corresponder a `VERSION`.

O workflow publica:

- `v<version>`
- marca `v<version>` como GitHub Latest quando `promote_latest` é true

O workflow de lançamento prepara o runtime CEF para Linux antes do empacotamento por plataforma.
Ele baixa o SDK mínimo oficial fixado do CEF 139 para Linux, monta o
`fennara-webview-cef-linux-x64-<cef-version>.zip` separado, remove símbolos dos binários ELF
preparados, grava um manifesto habilitado gerado em `local/webview-runtimes/linux-cef.json`
e fornece esse manifesto aos pacotes da CLI. O job de publicação então
valida que os ativos do lançamento incluem o zip CEF exato nomeado pelo
manifesto gerado e que seu SHA-256 corresponde. Ele também grava
`fennara-release-manifest-v<version>.json`, valida cada ativo e hash referenciado
e envia esse manifesto junto com o lançamento.

Workflows de pull request não publicam lançamentos. O workflow Package Preview
cria artefatos de teste com formato de lançamento, incluindo o manifesto e o payload do runtime
CEF para Linux, para que os mantenedores possam fazer testes rápidos do empacotamento antes da mesclagem. Package
Preview não é o canal de lançamento voltado ao usuário.

<a id="release-assets"></a>
## Ativos de lançamento

Cada lançamento deve conter pacotes da CLI/runtime local por plataforma e um pacote compartilhado do addon para todas as plataformas.

| Destino | Ativos |
| --- | --- |
| Windows x86_64 | `fennara-cli-windows-x86_64-v<version>.zip`<br>`fennara-release-local-windows-x86_64-v<version>.zip` |
| Linux x86_64 | `fennara-cli-linux-x86_64-v<version>.zip`<br>`fennara-release-local-linux-x86_64-v<version>.zip`<br>`fennara-webview-cef-linux-x64-<cef-version>.zip` |
| macOS arm64 | `fennara-cli-macos-arm64-v<version>.zip`<br>`fennara-release-local-macos-arm64-v<version>.zip` |
| Todas as plataformas | `fennara-release-addon-v<version>.zip`<br>`fennara-addon-latest.zip`<br>`fennara-release-manifest-v<version>.json` |

Funções dos pacotes:

| Padrão | Função |
| --- | --- |
| `fennara-cli-*` | Payload do script de instalação contendo apenas a CLI `fennara` para uma plataforma |
| `fennara-release-local-*` | Inicializadores de MCP e daemon, além dos binários de runtime versionados para uma plataforma |
| `fennara-release-addon-v*` | Addon versionado para todas as plataformas resolvido por meio do manifesto de lançamento |
| `fennara-addon-latest.zip` | Alias com nome estável do addon para todas as plataformas, usado pela documentação e por downloads manuais |
| `fennara-webview-cef-linux-x64-*` | Runtime CEF compartilhado somente para Linux, instalado uma vez nos dados do aplicativo Fennara |
| `fennara-release-manifest-v*` | Plano de instalação e atualização contendo nomes de ativos, valores SHA-256, primitivas de instalação e runtimes compartilhados |

A GDExtension do addon para macOS atualmente não é autenticada pela Apple. Downloads pelo navegador
e extração manual pelo Finder podem propagar metadados de quarentena e acionar a
notificação de verificação do macOS. A documentação de instalação voltada ao usuário deve
recomendar `fennara install` no macOS, explicar a limitação do ZIP manual e
orientar os usuários afetados a remover o addon copiado manualmente antes de reinstalá-lo
pela CLI. A validação de lançamento não considera que apenas criar o ZIP seja
assinatura ou autenticação no macOS.

O prefixo `fennara-release-local-*` impede que CLIs mais antigas contornem silenciosamente
o caminho de pacotes gerenciado pelo manifesto.

<a id="release-manifest"></a>
## Manifesto de lançamento

A partir da versão 0.3.0, `fennara install` e `fennara update` preferem o manifesto de
lançamento sempre que o lançamento publicar um. O manifesto registra:

- `schema_version`
- `version`
- `minimum_cli_version`
- primitivas de instalação compatíveis
- ativos da CLI e do runtime local por plataforma com hashes SHA-256
- o ativo compartilhado do addon com SHA-256
- ativos de runtime compartilhado específicos de plataforma, atualmente o CEF para Linux

`scripts/release-policy.mjs` é a fonte da verdade para
`minimum_cli_version`. O gravador do manifesto seleciona a política depois de validar
a identidade do lançamento, portanto Stable, Package Preview e Staging não podem escolher
valores independentes. Alterações normais no layout dos pacotes ou nos nomes dos ativos devem ser
tratadas pelos dados do manifesto, não pela alteração da CLI externa. Aumente a política quando
um lançamento exigir uma transferência de atualização, esquema de manifesto, primitiva de instalação,
comportamento de autoatualização ou outro recurso da CLI mais recente que uma CLI publicada
mais antiga não consiga executar com segurança.

Quando a CLI for antiga demais, `fennara update` deve usar a entrada
`assets.cli` por plataforma do manifesto para atualizar primeiro a CLI instalada e depois retomar
a atualização dos pacotes com `--no-self-update`. Se a autoatualização não estiver disponível para
esse lançamento ou local de instalação, ela deve falhar antes de instalar os pacotes e
exibir uma instrução clara para executar novamente `install.sh` ou `install.ps1`.

A identidade opcional de lançamento adicionada ao esquema 1 do manifesto não exige um
aumento da versão mínima da CLI. Clientes mais antigos do esquema 1 ignoram campos desconhecidos, enquanto
clientes que reconhecem staging validam a identidade quando ela está presente. Um futuro
lançamento que dependa de ativação consciente de canal ou transferência do atualizador deve
reavaliar a versão mínima da CLI antes da publicação.

<a id="staging-identity-and-discovery-contract"></a>
## Contrato de identidade e descoberta de staging

Os canais de staging são isolados por pull request:

| Valor | Exemplo do PR 101 |
| --- | --- |
| Canal | `pr-101` |
| Versão candidata | `1.2.3-pr.101.2` |
| Lançamento exato | `v1.2.3-pr.101.2` |
| Ref do canal | `fennara-staging/pr-101` |
| Arquivo de ponteiro | `fennara-staging-channel-pr-101.json` |

A ref do Git por canal contém apenas um pequeno arquivo de ponteiro para um
lançamento versionado exato. Os binários de lançamento nunca ficam sob a ref móvel do canal. A
CLI pode resolver esse ponteiro com a solicitação interna de versão
`channel:pr-101` e então continua usando apenas a versão exata.

Portanto, os PRs 101 e 125 usam tags de lançamento e ativos de ponteiro diferentes.
Atualizar um canal não pode redirecionar os testadores do outro canal. Publicar
um canal nunca altera a designação estável GitHub Latest nem o canal de outro
pull request.

<a id="staging-candidate-workflow"></a>
## Workflow de candidato de staging

O workflow manual **Staging Release** gera um candidato a partir do head atual
de um pull request aberto. Execute-o a partir de `main` e forneça:

| Entrada | Significado |
| --- | --- |
| `pull_request` | Pull request aberto para gerar |
| `base_version` | Versão estável planejada no formato `X.Y.Z` |
| `candidate` | Número crescente do candidato para este pull request |
| `source_commit` | SHA completo opcional que ainda deve ser o head do pull request |
| `publish` | Desativado para validação somente de artefatos, ativado para publicar o candidato |

O workflow congela o SHA do head do pull request antes de qualquer build de plataforma. Os
jobs de Windows, Linux e macOS fazem checkout desse commit exato com permissões somente leitura,
sem credenciais Git persistidas, sem credenciais de lançamento e sem
capacidade de salvar caches compartilhados de dependências. Eles podem restaurar caches compatíveis de
SCons/godot-cpp e Cargo gravados por workflows confiáveis da branch padrão.
Staging usa a ação de cache somente para restauração, portanto o código candidato pode consumir
resultados de build confiáveis, mas não pode substituir nem envenenar caches de execuções posteriores.
O código candidato pode produzir artefatos de build, mas não pode publicar um GitHub
Release.

Scripts confiáveis do repositório então validam a identidade do candidato, o inventário exato do
arquivo compactado, o conteúdo do addon, o layout dos pacotes de plataforma, o manifesto de lançamento e cada
valor SHA-256. A publicação permanece desabilitada, a menos que `publish` seja explicitamente
selecionado.

Quando a publicação está habilitada, o job final confiável:

1. Revalida os artefatos do candidato como dados.
2. Cria um rascunho, envia cada ativo e o publica como o pré-lançamento exato
   `v<exact-version>` sem alterar o GitHub Latest.
3. Baixa os ativos publicados e compara seus nomes e hashes.
4. Rejeita uma alteração de canal regressiva ou conflitante.
5. Atualiza por último a pequena ref de ponteiro `fennara-staging/pr-<number>` por meio de uma
   gravação condicional pela API de Conteúdo do GitHub.
6. Baixa o ponteiro ativo e verifica seu conteúdo exato.

As execuções de um pull request são serializadas. Pull requests diferentes usam
grupos de concorrência, tags de lançamento e refs de ponteiro separados. Repetir o mesmo
candidato verifica o lançamento exato existente em vez de misturar arquivos nele.
O workflow nunca cria, envia para nem promove o GitHub Latest estável.

A publicação estável não usa uma tag ou um lançamento literal `latest`. O workflow Release
cria o lançamento exato `v<version>` como rascunho, verifica byte a byte os ativos
enviados, publica-o como um lançamento mutável e marca esse lançamento exato
como GitHub Latest quando `promote_latest` é true. Os instaladores e a descoberta da CLI estável
resolvem o endpoint da API de Latest Release do GitHub.

Lançamentos estáveis e de staging são mutáveis enquanto a imutabilidade de lançamentos do repositório estiver
desabilitada. Ambos os workflows verificam os metadados do lançamento e os bytes dos ativos baixados
antes de concluir a publicação ou avançar um canal de staging. A publicação dos ativos
usa o `GITHUB_TOKEN` com escopo do job e acesso de gravação a conteúdo.

A política de lançamento atualmente exige a CLI `0.3.12` para manifestos estáveis e a
CLI `0.3.8` para manifestos de staging. A descoberta estável não resolve mais a
tag `latest` desativada. A versão estável `0.3.12` exige a validação de atualização corrigida,
a verificação prévia de troca de versão e o tratamento do diário de operações no Windows. Um candidato
de staging como `0.3.12-pr.123.1` tem precedência inferior à versão estável `0.3.12` no
SemVer, portanto seu mínimo deve permanecer abaixo da
versão candidata para que a configuração inicial instale a CLI candidata. Não
altere nenhum dos mínimos com base apenas na compatibilidade do esquema do manifesto.

O zip compartilhado do addon contém todos os binários GDExtension gerados e referenciados por `godot_demo/addons/fennara/fennara.gdextension`. O Godot carrega a biblioteca correspondente ao sistema operacional do usuário e ignora as outras.

Os payloads do runtime de webview CEF para Linux são separados do arquivo compactado do addon. O
empacotamento de lançamento gera o manifesto habilitado do runtime e incorpora esses dados em
`fennara-release-manifest-v<version>.json`. A CLI instala o payload CEF correspondente
uma vez no diretório de dados do aplicativo Fennara do usuário:

```text
webview/cef/linux-x64/<cef-version>/
```

Não coloque `libcef.so`, executáveis auxiliares do CEF, recursos do CEF nem pacotes de localidade
dentro de `fennara-addon-*`. Package Preview gera um artefato CEF separado para
testes e grava o mesmo tipo de manifesto de runtime gerado usado por Release,
mas a publicação do lançamento continua sendo a única fonte voltada ao usuário para os ativos de lançamento.

Os builds da GDExtension para Linux também precisam do código-fonte do wrapper oficial do SDK CEF, mas não
dos arquivos de runtime CEF no addon. A CI executa:

```bash
node scripts/prepare-linux-cef-sdk.mjs
```

e passa o diretório extraído como `FENNARA_CEF_ROOT` ao SCons. O SCons usa
`FENNARA_CEF_ROOT/libcef_dll/` para gerar a pequena biblioteca do addon
`libfennara_linux_cef_bridge.so` com o wrapper C++ fixado do CEF 139. O
download do SDK tem versão e hash verificados porque o código-fonte gerado do
wrapper deve corresponder à ABI do runtime CEF. A ponte é empacotada com o
addon; `libcef.so`, recursos, pacotes de localidade e `fennara_cef_helper` permanecem no
runtime CEF compartilhado separado.

Os scripts de empacotamento falham se arquivos do runtime CEF forem encontrados dentro do arquivo compactado do addon.
O nome do ativo de runtime deve ser:

```text
fennara-webview-cef-linux-x64-<cef-version>.zip
```

O zip deve ser extraído com os arquivos obrigatórios em sua raiz:

```text
libcef.so
fennara_cef_helper
icudtl.dat
resources.pak
chrome_100_percent.pak
chrome_200_percent.pak
v8_context_snapshot.bin
locales/en-US.pak
```

Arquivos opcionais do runtime CEF, como `chrome-sandbox`, `libEGL.so`,
`libGLESv2.so`, `libvk_swiftshader.so`, `libvulkan.so.1`,
`vk_swiftshader_icd.json`, `snapshot_blob.bin` e `locales/*.pak` adicionais
devem ser incluídos quando estiverem presentes na distribuição CEF selecionada.

Para montar manualmente o zip de runtime a partir de uma árvore de binários CEF selecionada pelo mantenedor:

```bash
node scripts/prepare-linux-cef-runtime.mjs \
  --cef-root /path/to/cef_binary_<version>_linux64_minimal \
  --version <cef-version> \
  --out-dir dist/cef-runtime
```

No Linux, o script gera `fennara_cef_helper` a partir de
`scripts/cef/linux/fennara_cef_helper.cpp` usando os cabeçalhos oficiais do CEF em
`fennara-cpp/vendor/cef/`. Em outro sistema operacional, primeiro gere esse auxiliar no Linux e
passe `--helper /path/to/fennara_cef_helper`. Use `--dry-run` para inspecionar os
arquivos selecionados antes de gravar o zip.

Depois que o script imprimir o SHA-256, atualize
`local/webview-runtimes/linux-cef.json`:

```json
{
  "version": "<cef-version>",
  "enabled": true,
  "archive": {
    "format": "zip",
    "name": "fennara-webview-cef-linux-x64-<cef-version>.zip",
    "url": null,
    "sha256": "<sha256>"
  }
}
```

Para lançamentos normais, o workflow grava automaticamente o manifesto do runtime CEF
para Linux com `--write-manifest`, e então `scripts/write-release-manifest.mjs`
copia os campos do runtime para `fennara-release-manifest-v<version>.json`. Não
habilite manualmente o manifesto substituto versionado, a menos que esteja
depurando intencionalmente um caminho manual de ativo de runtime ou um comportamento de fallback legado. Se os
dados gerados do manifesto apontarem para um ativo ausente ou cujo SHA-256 não
corresponda, o workflow Release e `fennara install` / `fennara update` no Linux falham
com clareza.

A CLI deve publicar atualizações do runtime CEF para Linux atomicamente: extrair e validar
em um diretório de staging, gravar o marcador do runtime somente depois que os arquivos obrigatórios
estiverem presentes, então publicar o diretório de versão e atualizar `current.json` com uma
renomeação de arquivo temporário. O marcador instalado `fennara-cef-runtime.json` deve identificar
o contrato do carregador nativo com `"runtime": "cef"`. A instalação e a atualização corrigem um
marcador legado correspondente que contenha apenas `"kind": "cef"` sem baixar
o payload CEF novamente. Editores em execução continuam usando o runtime que já
carregaram.

A CLI incorpora os modelos gerados de orientações do projeto a partir de `local/templates/`.
Quando o empacotamento do lançamento gera a CLI, esses modelos são compilados no binário junto com o restante do código da CLI.

<a id="what-latest-means"></a>
## O que `latest` significa

O ponteiro Latest Release do GitHub seleciona o lançamento versionado usado pelos fluxos normais
de instalação e atualização. A Fennara não cria nem move uma tag literal `latest`.

- `install.ps1` e `install.sh` buscam o ativo mais recente da CLI por padrão.
- `fennara update` busca o manifesto de lançamento pelo endpoint Latest Release do GitHub por padrão, autoatualiza a CLI instalada quando necessário e então resolve os ativos de runtime local/addon/compartilhado a partir dele.
- Atualizações no editor preparam ativos verificados antes do encerramento, verificam novamente o resumo completo do addon preparado antes da substituição, mantêm o addon anterior, os inicializadores e o manifesto do runtime até que a validação de ativação seja bem-sucedida e exigem o handshake da GDExtension reaberta antes de excluir os dados de reversão.
- `fennara install` busca o manifesto de lançamento pelo endpoint Latest Release do GitHub por padrão e então resolve os ativos de runtime local/addon/compartilhado a partir dele.
- A verificação de atualizações do plugin Godot compara com o lançamento mais recente do GitHub.

Use `promote_latest: false` somente ao publicar uma versão que não deva se tornar a instalação padrão do usuário.

Os instaladores e downloads de lançamento devem exibir os metadados do lançamento e as etapas de
download, extração, instalação e verificação dos ativos. As requisições de rede devem usar
limites de tempo definidos para que travamentos do GitHub/CDN falhem com um diagnóstico, em vez de
parecerem congelados. No Windows, `install.ps1` deve verificar o código de saída da
verificação da CLI antes de exibir sucesso. O código de saída `-1073741515` (`0xC0000135`) significa que o executável da CLI
foi gravado, mas o Windows não conseguiu iniciá-lo porque uma DLL obrigatória está ausente;
oriente o usuário a instalar o Microsoft Visual C++ Redistributable 2015-2022 x64 e
então executar novamente `fennara --version`, `fennara doctor` e `fennara install`.
URL de download: `https://aka.ms/vs/17/release/vc_redist.x64.exe`.

<a id="smoke-test-after-release"></a>
## Teste rápido após o lançamento

No Windows:

```powershell
irm https://raw.githubusercontent.com/fennaraOfficial/fennara-godot-ai/main/install.ps1 | iex
fennara --version
fennara doctor
```

Em um projeto Godot:

```bash
cd path/to/your-godot-project
fennara install
fennara mcp-setup --claude
```

Verifique se o projeto recebeu:

```text
AGENTS.md
addons/fennara/ai/
```

Abra o projeto no Godot e então peça ao aplicativo MCP:

```text
Use Fennara MCP to run fennara_status and tell me which Godot project is connected.
```

Teste de atualização:

```bash
cd path/to/your-godot-project
fennara update
fennara self-update
```

<a id="rules"></a>
## Regras

- O workflow Release é executado somente a partir de `main`.
- A entrada de versão do lançamento deve corresponder a `VERSION`.
- Workflows de pull request podem gerar e enviar artefatos de teste, mas não devem publicar lançamentos.
- Mantenha o lançamento destinado ao usuário comum designado como GitHub Latest.
- Não regrave tags de lançamento publicadas, a menos que os mantenedores decidam intencionalmente substituir um lançamento com defeito.
