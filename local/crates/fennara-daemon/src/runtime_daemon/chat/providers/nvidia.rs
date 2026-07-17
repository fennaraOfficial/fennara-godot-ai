use super::types::{
    AdapterKind, Auth, Capabilities, GenerationDefaults, Limits, ModelDefinition, ModelId,
    ProviderDefinition, ProviderId, RequestDefaults,
};

pub(crate) const PROVIDER_ID: &str = ProviderId::NVIDIA;
pub(crate) const API_BASE: &str = "https://integrate.api.nvidia.com/v1";
pub(crate) const API_KEY_ENV: &str = "NVIDIA_API_KEY";

pub(crate) fn provider_definition(api_key: Option<&str>) -> ProviderDefinition {
    ProviderDefinition {
        id: ProviderId::unchecked(PROVIDER_ID),
        name: "NVIDIA".to_string(),
        adapter: AdapterKind::OpenAiCompatibleChat,
        base_url: Some(API_BASE.to_string()),
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

pub(crate) fn model_definition(model_id: &str, display_name: Option<String>) -> ModelDefinition {
    let request = RequestDefaults {
        generation: GenerationDefaults {
            temperature: Some(0.7),
            max_output_tokens: None,
            reasoning_effort: None,
        },
        ..RequestDefaults::default()
    };

    ModelDefinition {
        id: ModelId::new(model_id).expect("NVIDIA model id is valid"),
        provider: ProviderId::unchecked(PROVIDER_ID),
        display_name: display_name.unwrap_or_else(|| fallback_display_name(model_id)),
        adapter_model_id: model_id.to_string(),
        capabilities: Capabilities::text_tools(),
        limits: Limits::default(),
        request,
        enabled: true,
    }
}

fn fallback_display_name(id: &str) -> String {
    id.split('/').next_back().unwrap_or(id).replace('-', " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_uses_hosted_nim_chat_endpoint_and_bearer_auth() {
        let provider = provider_definition(Some(" nvapi-test "));

        assert_eq!(provider.id.as_str(), ProviderId::NVIDIA);
        assert_eq!(provider.base_url.as_deref(), Some(API_BASE));
        assert_eq!(provider.adapter, AdapterKind::OpenAiCompatibleChat);
        assert_eq!(
            provider.auth,
            Auth::InlineBearer {
                value: "nvapi-test".to_string()
            }
        );
    }

    #[test]
    fn model_keeps_publisher_namespace_for_nvidia_api() {
        let model = model_definition("meta/llama-3.3-70b-instruct", None);

        assert_eq!(model.provider.as_str(), ProviderId::NVIDIA);
        assert_eq!(model.id.as_str(), "meta/llama-3.3-70b-instruct");
        assert_eq!(model.adapter_model_id, "meta/llama-3.3-70b-instruct");
    }
}
