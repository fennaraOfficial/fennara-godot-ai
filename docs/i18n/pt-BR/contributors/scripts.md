<!-- fennara-i18n: locale=pt-BR source=scripts/README.md sha256=57f0afc86f3a2f7e6e9f5f912884ccad08769c06d34bf55592b230681de36d31 -->
<a id="scripts"></a>
# Scripts

<!-- fennara-doc-nav:start -->
[English](../../../../scripts/README.md) · [简体中文](../../zh-CN/contributors/scripts.md) · [Español](../../es/contributors/scripts.md) · **Português do Brasil** · [日本語](../../ja/contributors/scripts.md) · [한국어](../../ko/contributors/scripts.md) · [Русский](../../ru/contributors/scripts.md) · [Français](../../fr/contributors/scripts.md) · [Deutsch](../../de/contributors/scripts.md) · [Türkçe](../../tr/contributors/scripts.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../../../scripts/README.md)
<!-- fennara-doc-nav:end -->

Este diretório contém a automação do repositório compartilhada pelo
desenvolvimento local, package preview e fluxos de lançamento.

Os scripts devem ser pequenos, determinísticos e seguros para execução a partir
da raiz do repositório, a menos que o texto de ajuda diga o contrário. Eles não
devem gravar estado específico do usuário fora do repositório.

<a id="version-scripts"></a>
## Scripts de versão

- `set-version.mjs`: atualiza o `VERSION` do repositório, o `VERSION` do addon, os metadados do workspace Rust local, as versões dos pacotes no lockfile e a constante de versão do plugin C++.
- `check-version.mjs`: verifica se esses arquivos versionados continuam sincronizados.

Execute `check-version.mjs` na CI e antes do empacotamento de um lançamento. Use
`set-version.mjs` somente ao alterar intencionalmente a versão do Fennara.

<a id="packaging-scripts"></a>
## Scripts de empacotamento

- `package-preview.mjs`: sincroniza payloads de addon commitados e então monta arquivos de preview por plataforma depois que a GDExtension e os binários Rust locais já tiverem sido compilados.
- `package-addon-all.mjs`: combina partes do addon de cada plataforma no arquivo final do addon para todas as plataformas.
- `release-policy.mjs`: define a versão mínima da CLI publicada compatível com cada faixa de lançamentos.
- `write-release-manifest.mjs`: grava `fennara-release-manifest-v<version>.json` a partir dos recursos de lançamento e valida cada SHA-256 referenciado.

Os dois scripts usam `.package-preview/` como staging temporário e gravam os
arquivos zip na pasta `dist/` na raiz do repositório. Essas saídas são ignoradas
e não devem ser commitadas.

Os scripts de empacotamento devem manter pequeno o payload do addon. Em especial,
arquivos do runtime CEF do Linux, como `libcef.so` e `fennara_cef_helper`, não
devem ser incluídos em `fennara-addon-*`. O CEF é instalado uma vez no diretório
compartilhado de dados de aplicativo do usuário.

<a id="staging-release-scripts"></a>
## Scripts de lançamento de staging

- `write-staging-candidate.mjs`: cria a identidade exata de pré-lançamento para um pull request e um commit de origem fixado.
- `validate-staging-build.mjs`: verifica partes do addon, arquivos de plataforma, o addon montado, o manifesto de lançamento e o CEF do Linux antes da publicação.
- `smoke-public-release.mjs`: baixa cada candidato publicado por sua URL de navegador não autenticada e verifica os hashes confiáveis dos recursos e do manifesto antes do avanço do canal.
- `write-staging-pointer.mjs`: grava o pequeno ponteiro por PR depois de calcular o hash do manifesto de lançamento exato.
- `check-staging-channel-advance.mjs`: rejeita movimentos retroativos ou conflitantes do canal.
- `validate-staging-publish-bundle.mjs`: revalida o pacote final de artefatos sem executar o código candidato.
- `verify-published-assets.mjs`: compara os nomes e valores SHA-256 esperados com os recursos baixados do GitHub Release.

Esses scripts dão suporte a `.github/workflows/staging-release.yml`. Jobs de
compilação de candidatos são executados sem credenciais de lançamento. Somente
o job final confiável pode publicar, e ele avança a referência Git específica
do canal depois que o lançamento exato é baixado e verificado.

<a id="linux-cef-scripts"></a>
## Scripts de CEF para Linux

- `prepare-linux-cef-sdk.mjs`: baixa e extrai o SDK oficial fixado do CEF Linux x64 usado para compilar a ponte CEF do Linux.
- `prepare-linux-cef-runtime.mjs`: prepara o zip separado do runtime CEF do Linux, valida os arquivos obrigatórios, remove símbolos dos binários ELF preparados no Linux e pode gravar o manifesto gerado `local/webview-runtimes/linux-cef.json` para o empacotamento do lançamento.
- `check-linux-cef-runtime-release.mjs`: valida que os recursos de lançamento contenham o zip do runtime CEF nomeado pelo manifesto habilitado e que seu SHA-256 corresponda.
- `cef/linux/fennara_cef_helper.cpp`: pequeno código-fonte do processo auxiliar CEF usado ao compilar o auxiliar de runtime a partir do SDK do CEF.

Os scripts do CEF trabalham apenas com arquivos copiados para staging. Eles não
devem alterar a árvore baixada ou de origem do SDK do CEF.

<a id="development-tests"></a>
## Testes de desenvolvimento

- `test-run-scene-edit-script-inspect.mjs`: cria um projeto de smoke test ignorado do Godot em `temp/` e verifica a inspeção de `PackedScene` importada, proteções de contexto somente leitura, falha por fonte ausente e comportamento sem salvamento com uma GDExtension de editor compilada.

<a id="documentation-localization"></a>
## Localização da documentação

- `sync-doc-navigation.mjs`: adiciona hashes de origem, âncoras estáveis e o seletor compacto de idioma para a mesma página sem traduzir a prosa.
- `check-doc-i18n.mjs`: valida a cobertura completa das localidades, atualização da fonte, navegação, âncoras, estrutura Markdown, código protegido, URLs e links.
- `doc-i18n-lib.mjs`: é responsável pelo manifesto de localidades compartilhado, normalização das fontes, renderização da navegação e auxiliares estruturais.

Execute:

```bash
node scripts/sync-doc-navigation.mjs
node scripts/check-doc-i18n.mjs
```

A localidade e o conjunto de documentos são declarados em
`docs/i18n/languages.json`. O inglês continua canônico. A prosa traduzida deve
ser escrita a partir da fonte em inglês, e não gerada por esses scripts.

A sincronização normal atualiza a navegação e as âncoras estáveis, mas preserva
os hashes de origem existentes. Depois de atualizar diretamente todas as nove
traduções de uma página em inglês alterada, atualize deliberadamente somente
essa fonte:

```bash
node scripts/sync-doc-navigation.mjs --accept-source docs/cli.md
```

A opção pode ser repetida para várias fontes revisadas. Não aceite uma fonte
cuja prosa traduzida ainda não foi atualizada. A CI executa
`sync-doc-navigation.mjs --check` antes do validador completo de traduções.

<a id="ui-sync"></a>
## Sincronização da interface

- `sync-chat-ui.mjs`: copia `ui/chat/` para `godot_demo/addons/fennara/dist/`.

`godot_demo/addons/fennara/dist/` é intencionalmente commitado porque os zips
de addon lançados devem conter a webview de chat compilada. Faça alterações em
`ui/chat/`, execute o script de sincronização e commite juntos os recursos-fonte
e os recursos gerados do addon.

<a id="runtime-sync"></a>
## Sincronização do runtime

- `sync-runtime.mjs`: copia `runtime/` para `godot_demo/addons/fennara/runtime/`.

`godot_demo/addons/fennara/runtime/` é intencionalmente commitado porque os zips
de addon lançados devem conter os scripts auxiliares de runtime do lado do Godot.
Faça alterações em `runtime/`, execute o script de sincronização e commite juntos
os recursos-fonte e os recursos gerados do addon.

<a id="guidance-sync"></a>
## Sincronização das orientações

- `sync-guidance.mjs`: copia as orientações compactas e páginas de conhecimento sob demanda de `local/templates/` para `godot_demo/addons/fennara/ai/`, correspondendo aos arquivos que `fennara install` e `fennara update` gravam nos projetos dos usuários.

`godot_demo/addons/fennara/ai/` é intencionalmente commitado porque o addon de
demonstração espelha a estrutura de um addon instalado. Faça alterações em
`local/templates/`, execute o script de sincronização e commite juntos os
recursos-fonte e as orientações geradas do addon.

<a id="boundaries"></a>
## Limites

- Os scripts podem criar saídas em `.package-preview/` e no `dist/` da raiz.
- Os scripts podem atualizar payloads gerados e commitados apenas quando esse for seu trabalho explícito, como `sync-chat-ui.mjs`, `sync-runtime.mjs`, `sync-guidance.mjs` ou `set-version.mjs`.
- Os scripts não devem gravar cache do editor Godot, instalações locais nos dados de aplicativo, artefatos de lançamento baixados nem saída de testes em máquinas virtuais dentro de pastas de código-fonte rastreadas.
