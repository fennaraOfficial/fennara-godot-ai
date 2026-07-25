<!-- fennara-i18n: locale=pt-BR source=docs/telemetry.md sha256=925414507b4bfef9d6b7f207125bc0df953c8392e168f3ae20be78cf79c58d6a -->
<a id="anonymous-telemetry"></a>
# Telemetria anônima

<!-- fennara-doc-nav:start -->
[Documentação](README.md)

🌐 **Idiomas**

[English](../../telemetry.md) · [简体中文](../zh-CN/telemetry.md) · [Español](../es/telemetry.md) · **Português do Brasil** · [日本語](../ja/telemetry.md) · [한국어](../ko/telemetry.md) · [Русский](../ru/telemetry.md) · [Français](../fr/telemetry.md) · [Deutsch](../de/telemetry.md) · [Türkçe](../tr/telemetry.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../telemetry.md)
<!-- fennara-doc-nav:end -->

O Fennara envia no máximo um pequeno evento anônimo de atividade por dia UTC.
O evento só é enviado depois que um editor Godot compatível se conecta ao daemon
local. Ele ajuda os mantenedores a medir instalações ativas, o uso das plataformas
compatíveis e a adoção de versões.

A telemetria é ativada por padrão. Abra **Chat Settings > Chat > Anonymous
telemetry** para desativá-la. Ambientes headless e automatizados podem definir:

```text
FENNARA_DISABLE_TELEMETRY=true
DO_NOT_TRACK=1
```

Uma variável de ambiente tem precedência sobre a preferência salva na interface.
Desativar a telemetria interrompe eventos futuros e exclui a identidade local de
telemetria e o estado do último envio. Reativá-la cria uma nova identidade
aleatória quando o Godot se conectar novamente.

<a id="event-contents"></a>
## Conteúdo do evento

O evento `fennara_active_installation` contém apenas:

| Campo | Finalidade |
| --- | --- |
| `schema_version` | Versão do pequeno contrato do payload de telemetria |
| `event` | Nome fixo do evento |
| `installation_id` | UUID aleatório gerado localmente, não derivado de hardware nem de contas |
| `fennara_version` | Versão do daemon em execução |
| `godot_version` | Versão numérica do Godot, como `4.6.3` |
| `platform` | `windows`, `macos` ou `linux` |
| `architecture` | `x86_64` ou `aarch64` |

O Fennara não envia nomes ou caminhos de projetos, informações de contas, prompts,
mensagens de chat, chaves de provedores, nomes de modelos, nomes de ferramentas,
argumentos ou resultados de ferramentas, logs, capturas de tela, conteúdo de
cenas, nomes de arquivos nem texto de erros.

<a id="storage-and-transport"></a>
## Armazenamento e transporte

O daemon armazena sua identidade aleatória e o último dia UTC enviado no
diretório compartilhado de dados de aplicativo do Fennara:

```text
Fennara/
  telemetry/
    state.json
```

O daemon envia o evento por HTTPS para
`https://fennara.io/api/telemetry`. O receptor valida uma lista exata de campos
permitidos e substitui o UUID bruto da instalação por um HMAC do lado do servidor
antes de encaminhar o evento ao PostHog. Perfis de pessoas e geolocalização por
IP no PostHog ficam desativados para esse evento.

O receptor da Vercel necessariamente observa metadados normais de rede enquanto
processa a solicitação HTTPS. Esses metadados não são copiados para o payload do
evento no PostHog.

<a id="delivery-behavior"></a>
## Comportamento do envio

A telemetria funciona fora dos caminhos de chamadas de ferramentas do Godot:

- Uma fila limitada aceita sinais de atividade sem esperar.
- Um único worker em segundo plano reutiliza um cliente HTTP.
- As solicitações têm um tempo limite curto.
- Uma fila cheia, um problema no sistema de arquivos, uma falha de rede ou uma
  rejeição do servidor são tolerados silenciosamente e nunca fazem uma ferramenta
  do Fennara falhar.
- O dia UTC só é registrado depois que o servidor aceita um evento, portanto uma
  conexão posterior do Godot pode tentar novamente um envio que falhou.
- O encerramento aguarda brevemente e depois cancela o worker de telemetria em vez
  de atrasar o daemon.

Uma instalação corresponde a um UUID aleatório persistido. Usar o Fennara em dois
computadores conta como duas instalações. Limpar os dados de aplicativo do Fennara
ou desativar e depois reativar a telemetria cria uma nova identidade.

As instalações ativas mensais são contadas como identidades anônimas distintas
que enviaram pelo menos um evento `fennara_active_installation` durante o mês civil.
