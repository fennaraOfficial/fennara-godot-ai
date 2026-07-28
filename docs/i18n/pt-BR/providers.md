<!-- fennara-i18n: locale=pt-BR source=docs/providers.md sha256=47172d638ee7954d827f50c7e8803e4a5bb2552ceb9ff44a780d49b9bf96fac6 -->
<a id="built-in-chat-providers"></a>
# Provedores do chat integrado

<!-- fennara-doc-nav:start -->
[English](../../providers.md) · [简体中文](../zh-CN/providers.md) · [Español](../es/providers.md) · **Português do Brasil** · [日本語](../ja/providers.md) · [한국어](../ko/providers.md) · [Русский](../ru/providers.md) · [Français](../fr/providers.md) · [Deutsch](../de/providers.md) · [Türkçe](../tr/providers.md)

> ℹ️ Tradução redigida por IA a partir do original em inglês. A revisão por falantes nativos é bem-vinda. [Fonte em inglês](../../providers.md)
<!-- fennara-doc-nav:end -->

Conecte um provedor de modelo ao dock de chat do Fennara dentro do Godot.

> [!NOTE]
> Aplicativos MCP externos usam sua própria configuração de modelo. Você não
> precisa conectar um provedor aqui para usar o Fennara a partir do Codex,
> Claude, Cursor ou outro aplicativo MCP. Consulte
> [Aplicativos MCP e chat integrado](chat-vs-mcp.md).

<a id="quick-setup"></a>
## Configuração rápida

1. Abra **Chat Settings > Chat** no dock do Fennara.
2. Selecione **Open providers**.
3. Escolha um provedor na nuvem e informe sua própria chave, ou escolha Ollama
   ou LM Studio para um modelo local.
4. Selecione um modelo.

Você também pode digitar `/provider` e `/model` no compositor.

<a id="provider-reference"></a>
## Referência de provedores

| Provedor | Como conectar | Formato do ID do modelo | Observações |
| --- | --- | --- | --- |
| OpenAI | Crie uma chave em [OpenAI API keys](https://platform.openai.com/api-keys). Chave ou variável do Fennara: `OPENAI_API_KEY`. | `openai/<model>` | Usa a API oficial da OpenAI. |
| Anthropic | Crie uma chave em [Claude Console API keys](https://console.anthropic.com/settings/keys). Chave ou variável do Fennara: `ANTHROPIC_API_KEY`. | `anthropic/<model>` | Usa a API Messages oficial da Anthropic. |
| OpenRouter | Crie uma chave em [OpenRouter Keys](https://openrouter.ai/settings/keys). Chave ou variável do Fennara: `OPENROUTER_API_KEY`. | `openrouter/<provider>/<model>` | Usa a API do OpenRouter. |
| Ollama Cloud | Crie uma chave em [Ollama API keys](https://ollama.com/settings/keys). Chave ou variável do Fennara: `OLLAMA_API_KEY`. | `ollama-cloud/<model>` | Usa a API hospedada do Ollama, não o servidor Ollama local. |
| DeepSeek | Crie uma chave em [DeepSeek API keys](https://platform.deepseek.com/api_keys). Chave ou variável do Fennara: `DEEPSEEK_API_KEY`. | `deepseek/<model>` | Usa a API compatível com OpenAI do DeepSeek. |
| Z.AI | Crie uma chave em [Z.AI API keys](https://z.ai/manage-apikey/apikey-list). Chave ou variável do Fennara: `ZHIPU_API_KEY`. | `zai/<model>` | Usa a API compatível com OpenAI da Z.AI. |
| Moonshot AI | Crie uma chave em [Kimi Open Platform API keys](https://platform.kimi.ai/console/api-keys). Chave ou variável do Fennara: `MOONSHOT_API_KEY`. | `moonshotai/<model>` | Usa a API compatível com OpenAI da Moonshot. |
| Moonshot AI (China) | Crie uma chave em [Kimi China Open Platform API keys](https://platform.kimi.com/console/api-keys). Chave ou variável do Fennara: `MOONSHOT_API_KEY`. | `moonshotai-cn/<model>` | Usa a API compatível com OpenAI da Moonshot China. |
| Kimi For Coding | Crie uma chave no [Kimi Code Console](https://www.kimi.com/code/console). Chave ou variável do Fennara: `KIMI_API_KEY`. | `kimi-for-coding/<model>` | Usa a API Messages compatível com Anthropic do Kimi. Exige acesso ao Kimi Code. |
| MiniMax | Crie uma chave pré-paga em [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview), em **API Keys > Create new secret key**. Chave ou variável do Fennara: `MINIMAX_API_KEY`. | `minimax/<model>` | Usa a API Messages compatível com Anthropic do MiniMax em `minimax.io`. |
| MiniMax Token Plan | Use a Subscription Key de [MiniMax API Platform](https://platform.minimax.io/docs/api-reference/api-overview), em **Billing > Token Plan**. Chave ou variável do Fennara: `MINIMAX_API_KEY`. | `minimax-coding-plan/<model>` | As Subscription Keys do Token Plan são separadas das chaves de API pré-pagas. |
| MiniMax (China) | Crie uma chave pré-paga na página de chaves da [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Chave ou variável do Fennara: `MINIMAX_API_KEY`. | `minimax-cn/<model>` | Usa a API Messages compatível com Anthropic da MiniMax China em `minimaxi.com`. |
| MiniMax Token Plan (China) | Use a Subscription Key da página Token Plan da [MiniMax China](https://platform.minimaxi.com/docs/api-reference/api-overview). Chave ou variável do Fennara: `MINIMAX_API_KEY`. | `minimax-cn-coding-plan/<model>` | As Subscription Keys do Token Plan da China são separadas das chaves de API pré-pagas. |
| NVIDIA | Crie uma chave em [build.nvidia.com](https://build.nvidia.com/). Chave ou variável do Fennara: `NVIDIA_API_KEY`. | `nvidia/<publisher>/<model>` | Usa a API NIM hospedada da NVIDIA compatível com OpenAI. |
| Ollama | Execute um servidor Ollama local. Nenhuma chave de API na nuvem é necessária. | `ollama/<local-model>` | O padrão é `http://127.0.0.1:11434`. |
| LM Studio | Inicie o servidor local do LM Studio. Nenhuma chave é necessária por padrão. | `lmstudio/<local-model>` | O padrão é `http://127.0.0.1:1234/v1`. Se o servidor LM Studio exigir autenticação, defina `LMSTUDIO_API_KEY` no ambiente do daemon. |

Provedores na nuvem precisam de sua própria chave de API ou assinatura. Provedores
locais precisam do servidor local em execução com um modelo disponível.

Seleções do OpenRouter sempre usam o formato explícito
`openrouter/<provider>/<model>`. Seleções antigas salvas como
`<provider>/<model>` são migradas uma vez quando as configurações são carregadas,
mas o formato antigo não é usado em novos roteamentos.

O Fennara pode armazenar chaves a partir do seletor de provedores no dock. Chat
Settings inclui o botão **Open providers** para abrir o mesmo seletor. Os nomes
de chave ou variável acima são os mesmos reconhecidos pelo Fennara caso prefira
variáveis de ambiente. As chaves armazenadas ficam nos dados locais do daemon,
fora do projeto Godot.

<a id="custom-openai-compatible-providers"></a>
## Provedores personalizados compatíveis com OpenAI

Escolha **Custom** na parte inferior do seletor de provedores para adicionar um
endpoint compatível com OpenAI, como um roteador local ou um gateway de API interno. Informe:

- um ID exclusivo de provedor em letras minúsculas
- o nome de exibição mostrado no Fennara
- uma URL-base terminando na versão da API, por exemplo `http://localhost:20128/v1`
- uma chave de API opcional
- um ou mais IDs e nomes de exibição de modelos, tamanhos de contexto e limites máximos de tokens de saída
- cabeçalhos de solicitação opcionais

Os IDs dos modelos devem corresponder ao que o endpoint espera. O Fennara os
expõe como `<provider-id>/<model-id>` no seletor, mas envia apenas `<model-id>`
ao provedor. O endpoint deve implementar o formato de solicitação e resposta
por streaming compatível com OpenAI de `/chat/completions`.

Chaves de API e valores de cabeçalhos personalizados usam o armazenamento
protegido de autenticação do daemon. As definições de provedores permanecem em
dados locais gerenciados pelo daemon fora do projeto Godot. Limites precisos de
modelo permitem que o Fennara compacte o histórico antes que uma solicitação
ultrapasse a janela de contexto e mantenha os resumos gerados dentro do limite
de saída. Modelos personalizados salvos antes da existência desses campos são
carregados com padrões de compatibilidade de 64.000 tokens de contexto e 4.096 tokens de saída.

Depois de salvo, o provedor personalizado aparece no seletor com sua quantidade
de modelos. Selecione-o para reabrir o formulário e adicionar ou renomear
modelos. Deixar a chave de API vazia preserva a chave salva, e novos cabeçalhos
informados são combinados por nome com os já salvos.

<a id="where-settings-live"></a>
## Onde ficam as configurações

O Fennara armazena localmente as configurações do chat integrado por meio do daemon, fora do projeto Godot:

- chaves de API dos provedores
- valores de cabeçalhos de provedores personalizados
- definições de provedores personalizados compatíveis com OpenAI
- URLs-base de provedores locais
- modelo selecionado
- esforço de raciocínio
- tempo limite de resposta do provedor
- modo de exibição do chat, incorporado ao Godot ou aberto no navegador do sistema
- histórico do chat

Essas configurações não são gravadas em `res://addons/fennara/` nem compartilhadas com Claude, Codex, Cursor, Gemini ou outros aplicativos MCP externos.

<a id="provider-response-timeout"></a>
## Tempo limite de resposta do provedor

A configuração **Provider response timeout** controla por quanto tempo cada solicitação de modelo pode ser executada no chat integrado. O valor padrão é 120 segundos e são aceitos valores de 30 a 3600 segundos. Aumentar esse valor pode ajudar modelos locais mais lentos ou turnos longos com muitas ferramentas a serem concluídos. O daemon aplica o tempo limite selecionado à solicitação do provedor e cancela a solicitação quando esse limite é atingido.

<a id="chat-display-setting"></a>
## Configuração de exibição do chat

A caixa de diálogo Chat Settings inclui **Open chat in my system browser next time**.

Quando a opção está desativada, o Fennara tenta renderizar o chat integrado
dentro do dock do Godot. Quando está ativada, o dock mostra um botão **Open chat**
e inicia o mesmo chat pelo daemon local em `127.0.0.1`. Isso pode reduzir o uso
de GPU e memória do editor e também serve como alternativa caso a webview nativa não inicie.

A alteração entra em vigor na próxima inicialização do Godot. Ela só muda onde
a interface do chat integrado é exibida. Não altera provedor, modelo, chaves de
API, histórico, configuração de aplicativos MCP nem o modelo usado externamente
por Claude, Codex ou Cursor.

<a id="picker-shortcuts"></a>
## Atalhos dos seletores

Chat Settings, os controles do dock e `/provider` abrem o mesmo seletor de
provedores. Use `/model` ou o controle de modelo do dock para abrir o seletor.

Consulte [Comandos de barra do chat integrado](slash-commands.md) para ver o comportamento da paleta.

<a id="local-providers"></a>
## Provedores locais

Para Ollama:

```bash
ollama serve
ollama pull llama3.1:8b
```

Depois, escolha:

```text
ollama/llama3.1:8b
```

Seleções antigas `local/<model>` ainda são aceitas como aliases de compatibilidade
do Ollama. Prefira o formato explícito `ollama/<model>` para novas configurações.

O Fennara envia o máximo por chamada do Ollama no campo compatível com OpenAI
`max_tokens`, que o Ollama mapeia para sua opção nativa `num_predict`.

Para o LM Studio, inicie o servidor local no LM Studio e escolha um ID de modelo com o formato:

```text
lmstudio/<loaded-model-id>
```

Os formulários de configuração do Ollama e do LM Studio compartilham um limite
máximo configurado de saída por chamada, cujo padrão é 8.192 tokens. Quando um
servidor local informa o tamanho do contexto carregado, o Fennara limita essa
configuração à metade do contexto para preservar espaço para a entrada. O
Fennara envia esse limite efetivo como `max_tokens` e reserva o mesmo valor ao
decidir quando compactar o histórico do chat.

<a id="model-catalog"></a>
## Catálogo de modelos

O daemon mantém um catálogo local de modelos para provedores na nuvem e consulta
os servidores locais sobre os modelos disponíveis. Se um catálogo ou servidor
mudar enquanto o Godot estiver aberto, atualize o seletor de modelos ou reabra
o seletor de provedor ou modelo.

O Fennara verifica recursos básicos do modelo antes de enviar uma solicitação:

- saída de texto é obrigatória
- chamadas de ferramentas são obrigatórias para o uso das ferramentas do Fennara
- entrada de imagens é obrigatória antes que anexos sejam enviados como contexto visual

A entrada de imagens do Ollama ainda não está habilitada no chat do Fennara.
