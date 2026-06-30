use super::types::{
    AdapterKind, Auth, Capabilities, GenerationDefaults, Limits, ModelDefinition, ModelId,
    ProviderDefinition, ProviderId, RequestDefaults,
};

pub(crate) const KIMI_PROVIDER_ID: &str = ProviderId::KIMI_FOR_CODING;
pub(crate) const KIMI_API_BASE: &str = "https://api.kimi.com/coding/v1";
pub(crate) const KIMI_API_KEY_ENV: &str = "KIMI_API_KEY";

pub(crate) const MINIMAX_PROVIDER_ID: &str = ProviderId::MINIMAX;
pub(crate) const MINIMAX_CODING_PLAN_PROVIDER_ID: &str = ProviderId::MINIMAX_CODING_PLAN;
pub(crate) const MINIMAX_CN_PROVIDER_ID: &str = ProviderId::MINIMAX_CN;
pub(crate) const MINIMAX_CN_CODING_PLAN_PROVIDER_ID: &str = ProviderId::MINIMAX_CN_CODING_PLAN;
pub(crate) const MINIMAX_API_BASE: &str = "https://api.minimax.io/anthropic/v1";
pub(crate) const MINIMAX_CN_API_BASE: &str = "https://api.minimaxi.com/anthropic/v1";
pub(crate) const MINIMAX_API_KEY_ENV: &str = "MINIMAX_API_KEY";

#[derive(Clone, Copy, Debug)]
pub(crate) struct AnthropicProviderSpec {
    pub(crate) id: &'static str,
    pub(crate) name: &'static str,
    pub(crate) api_base: &'static str,
    pub(crate) api_key_env: &'static str,
}

pub(crate) const SPECS: &[AnthropicProviderSpec] = &[
    AnthropicProviderSpec {
        id: KIMI_PROVIDER_ID,
        name: "Kimi For Coding",
        api_base: KIMI_API_BASE,
        api_key_env: KIMI_API_KEY_ENV,
    },
    AnthropicProviderSpec {
        id: MINIMAX_PROVIDER_ID,
        name: "MiniMax (minimax.io)",
        api_base: MINIMAX_API_BASE,
        api_key_env: MINIMAX_API_KEY_ENV,
    },
    AnthropicProviderSpec {
        id: MINIMAX_CODING_PLAN_PROVIDER_ID,
        name: "MiniMax Token Plan (minimax.io)",
        api_base: MINIMAX_API_BASE,
        api_key_env: MINIMAX_API_KEY_ENV,
    },
    AnthropicProviderSpec {
        id: MINIMAX_CN_PROVIDER_ID,
        name: "MiniMax (minimaxi.com)",
        api_base: MINIMAX_CN_API_BASE,
        api_key_env: MINIMAX_API_KEY_ENV,
    },
    AnthropicProviderSpec {
        id: MINIMAX_CN_CODING_PLAN_PROVIDER_ID,
        name: "MiniMax Token Plan (minimaxi.com)",
        api_base: MINIMAX_CN_API_BASE,
        api_key_env: MINIMAX_API_KEY_ENV,
    },
];

pub(crate) fn spec(provider_id: &str) -> Option<&'static AnthropicProviderSpec> {
    SPECS.iter().find(|spec| spec.id == provider_id)
}

pub(crate) fn provider_definition(
    provider_id: &str,
    api_key: Option<&str>,
) -> Option<ProviderDefinition> {
    let spec = spec(provider_id)?;
    Some(ProviderDefinition {
        id: ProviderId::unchecked(spec.id),
        name: spec.name.to_string(),
        adapter: AdapterKind::AnthropicCompatibleMessages,
        base_url: Some(spec.api_base.to_string()),
        auth: api_key
            .filter(|key| !key.trim().is_empty())
            .map(|key| Auth::InlineBearer {
                value: key.trim().to_string(),
            })
            .unwrap_or_else(|| Auth::Env {
                var: spec.api_key_env.to_string(),
            }),
        request: RequestDefaults::default(),
        disabled: false,
    })
}

pub(crate) fn model_definition(
    provider_id: &str,
    model_id: &str,
    display_name: Option<String>,
) -> ModelDefinition {
    let mut request = RequestDefaults::default();
    request.generation = GenerationDefaults {
        temperature: Some(0.7),
        max_output_tokens: None,
        reasoning_effort: None,
    };

    ModelDefinition {
        id: ModelId::new(model_id).expect("Anthropic-compatible model id is valid"),
        provider: ProviderId::unchecked(static_provider_id(provider_id)),
        display_name: display_name.unwrap_or_else(|| fallback_display_name(model_id)),
        adapter_model_id: model_id.to_string(),
        capabilities: Capabilities::text_tools(),
        limits: Limits::default(),
        request,
        enabled: true,
    }
}

pub(crate) fn is_anthropic_provider(provider_id: &str) -> bool {
    spec(provider_id).is_some()
}

fn static_provider_id(provider_id: &str) -> &'static str {
    spec(provider_id)
        .map(|spec| spec.id)
        .unwrap_or(ProviderId::KIMI_FOR_CODING)
}

fn fallback_display_name(id: &str) -> String {
    id.split('/').next_back().unwrap_or(id).replace('-', " ")
}
