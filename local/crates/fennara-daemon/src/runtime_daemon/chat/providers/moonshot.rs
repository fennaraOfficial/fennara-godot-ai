use super::types::{
    AdapterKind, Auth, Capabilities, GenerationDefaults, Limits, ModelDefinition, ModelId,
    ProviderDefinition, ProviderId, RequestDefaults,
};

pub(crate) const PROVIDER_ID: &str = ProviderId::MOONSHOTAI;
pub(crate) const CN_PROVIDER_ID: &str = ProviderId::MOONSHOTAI_CN;
pub(crate) const API_BASE: &str = "https://api.moonshot.ai/v1";
pub(crate) const CN_API_BASE: &str = "https://api.moonshot.cn/v1";
pub(crate) const API_KEY_ENV: &str = "MOONSHOT_API_KEY";

pub(crate) fn provider_definition(api_key: Option<&str>) -> ProviderDefinition {
    provider_definition_for(PROVIDER_ID, "Moonshot AI", API_BASE, api_key)
}

pub(crate) fn cn_provider_definition(api_key: Option<&str>) -> ProviderDefinition {
    provider_definition_for(CN_PROVIDER_ID, "Moonshot AI (China)", CN_API_BASE, api_key)
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
        id: ModelId::new(model_id).expect("Moonshot model id is valid"),
        provider: ProviderId::unchecked(static_provider_id(provider_id)),
        display_name: display_name.unwrap_or_else(|| fallback_display_name(model_id)),
        adapter_model_id: model_id.to_string(),
        capabilities: Capabilities::text_tools(),
        limits: Limits::default(),
        request,
        enabled: true,
    }
}

fn provider_definition_for(
    provider_id: &'static str,
    name: &str,
    api_base: &str,
    api_key: Option<&str>,
) -> ProviderDefinition {
    ProviderDefinition {
        id: ProviderId::unchecked(provider_id),
        name: name.to_string(),
        adapter: AdapterKind::OpenAiCompatibleChat,
        base_url: Some(api_base.to_string()),
        auth: api_key
            .filter(|key| !key.trim().is_empty())
            .map(|key| Auth::InlineBearer {
                value: key.trim().to_string(),
            })
            .unwrap_or_else(|| Auth::Env {
                var: API_KEY_ENV.to_string(),
            }),
        request: RequestDefaults::default(),
        disabled: false,
    }
}

fn static_provider_id(provider_id: &str) -> &'static str {
    match provider_id {
        ProviderId::MOONSHOTAI_CN => ProviderId::MOONSHOTAI_CN,
        _ => ProviderId::MOONSHOTAI,
    }
}

fn fallback_display_name(id: &str) -> String {
    id.split('/').next_back().unwrap_or(id).replace('-', " ")
}
