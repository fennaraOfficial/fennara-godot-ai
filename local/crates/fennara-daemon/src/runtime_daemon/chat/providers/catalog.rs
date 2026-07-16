use std::collections::BTreeMap;

use super::anthropic;
use super::anthropic_providers;
use super::catalog_cache;
use super::deepseek;
use super::lmstudio;
use super::models_dev::OpenRouterCatalog;
use super::moonshot;
use super::nvidia;
use super::ollama;
use super::ollama_cloud;
use super::openai;
use super::openrouter;
use super::types::{
    Limits, ModelDefinition, ModelId, ModelRef, ProviderDefinition, ProviderId, ProviderSettings,
    ResolvedModel,
};
use super::zai;
use crate::runtime_daemon::chat::settings;

#[derive(Clone, Debug, Default)]
pub(crate) struct Catalog {
    providers: BTreeMap<ProviderId, ProviderDefinition>,
    models: BTreeMap<(ProviderId, ModelId), ModelDefinition>,
    local_model_limits: BTreeMap<String, Limits>,
    default_model: Option<ModelRef>,
}

impl Catalog {
    pub(crate) fn from_settings(settings: &ProviderSettings) -> Self {
        let needs_hosted_catalog =
            key_or_env_present(settings.openai_api_key.as_ref(), openai::API_KEY_ENV)
                || key_or_env_present(settings.anthropic_api_key.as_ref(), anthropic::API_KEY_ENV)
                || key_or_env_present(settings.openrouter_api_key.as_ref(), "OPENROUTER_API_KEY")
                || key_or_env_present(settings.ollama_cloud_api_key.as_ref(), "OLLAMA_API_KEY")
                || key_or_env_present(settings.lmstudio_api_key.as_ref(), lmstudio::API_KEY_ENV)
                || key_or_env_present(settings.deepseek_api_key.as_ref(), deepseek::API_KEY_ENV)
                || key_or_env_present(settings.zai_api_key.as_ref(), zai::API_KEY_ENV)
                || key_or_env_present(settings.moonshot_api_key.as_ref(), moonshot::API_KEY_ENV)
                || key_or_env_present(settings.moonshot_cn_api_key.as_ref(), moonshot::API_KEY_ENV)
                || key_or_env_present(
                    settings.kimi_api_key.as_ref(),
                    anthropic_providers::KIMI_API_KEY_ENV,
                )
                || key_or_env_present(
                    settings.minimax_api_key.as_ref(),
                    anthropic_providers::MINIMAX_API_KEY_ENV,
                )
                || key_or_env_present(
                    settings.minimax_coding_plan_api_key.as_ref(),
                    anthropic_providers::MINIMAX_API_KEY_ENV,
                )
                || key_or_env_present(
                    settings.minimax_cn_api_key.as_ref(),
                    anthropic_providers::MINIMAX_API_KEY_ENV,
                )
                || key_or_env_present(
                    settings.minimax_cn_coding_plan_api_key.as_ref(),
                    anthropic_providers::MINIMAX_API_KEY_ENV,
                )
                || key_or_env_present(settings.nvidia_api_key.as_ref(), nvidia::API_KEY_ENV);
        let hosted_catalog = needs_hosted_catalog
            .then(catalog_cache::load_disk_blocking)
            .and_then(Result::ok);
        Self::from_settings_and_openrouter(
            settings,
            hosted_catalog.as_ref().map(|cached| &cached.catalog),
            hosted_catalog.as_ref().map(|cached| &cached.openai),
            hosted_catalog.as_ref().map(|cached| &cached.anthropic),
            hosted_catalog.as_ref().map(|cached| &cached.ollama_cloud),
            hosted_catalog.as_ref().map(|cached| &cached.lmstudio),
            hosted_catalog.as_ref().map(|cached| &cached.deepseek),
            hosted_catalog.as_ref().map(|cached| &cached.zai),
            hosted_catalog.as_ref().map(|cached| &cached.moonshot),
            hosted_catalog.as_ref().map(|cached| &cached.moonshot_cn),
            hosted_catalog
                .as_ref()
                .map(|cached| &cached.kimi_for_coding),
            hosted_catalog.as_ref().map(|cached| &cached.minimax),
            hosted_catalog
                .as_ref()
                .map(|cached| &cached.minimax_coding_plan),
            hosted_catalog.as_ref().map(|cached| &cached.minimax_cn),
            hosted_catalog
                .as_ref()
                .map(|cached| &cached.minimax_cn_coding_plan),
            hosted_catalog.as_ref().map(|cached| &cached.nvidia),
        )
    }

    pub(crate) fn from_settings_and_openrouter(
        settings: &ProviderSettings,
        hosted_openrouter: Option<&OpenRouterCatalog>,
        hosted_openai: Option<&OpenRouterCatalog>,
        hosted_anthropic: Option<&OpenRouterCatalog>,
        hosted_ollama_cloud: Option<&OpenRouterCatalog>,
        hosted_lmstudio: Option<&OpenRouterCatalog>,
        hosted_deepseek: Option<&OpenRouterCatalog>,
        hosted_zai: Option<&OpenRouterCatalog>,
        hosted_moonshot: Option<&OpenRouterCatalog>,
        hosted_moonshot_cn: Option<&OpenRouterCatalog>,
        hosted_kimi_for_coding: Option<&OpenRouterCatalog>,
        hosted_minimax: Option<&OpenRouterCatalog>,
        hosted_minimax_coding_plan: Option<&OpenRouterCatalog>,
        hosted_minimax_cn: Option<&OpenRouterCatalog>,
        hosted_minimax_cn_coding_plan: Option<&OpenRouterCatalog>,
        hosted_nvidia: Option<&OpenRouterCatalog>,
    ) -> Self {
        let mut catalog = Self::default();
        catalog.local_model_limits = settings.local_model_limits.clone();
        catalog.insert_provider(openai::provider_definition(
            settings.openai_api_key.as_deref(),
        ));
        catalog.insert_provider(anthropic::provider_definition(
            settings.anthropic_api_key.as_deref(),
        ));
        catalog.insert_provider(openrouter::provider_definition(
            settings.openrouter_api_key.as_deref(),
        ));
        catalog.insert_provider(ollama_cloud::provider_definition(
            settings.ollama_cloud_api_key.as_deref(),
        ));
        catalog.insert_provider(lmstudio::provider_definition(
            &settings.lmstudio_base_url,
            settings.lmstudio_api_key.as_deref(),
        ));
        catalog.insert_provider(deepseek::provider_definition(
            settings.deepseek_api_key.as_deref(),
        ));
        catalog.insert_provider(zai::provider_definition(settings.zai_api_key.as_deref()));
        catalog.insert_provider(moonshot::provider_definition(
            settings.moonshot_api_key.as_deref(),
        ));
        catalog.insert_provider(moonshot::cn_provider_definition(
            settings.moonshot_cn_api_key.as_deref(),
        ));
        catalog.insert_anthropic_provider(
            ProviderId::KIMI_FOR_CODING,
            settings.kimi_api_key.as_deref(),
        );
        catalog.insert_anthropic_provider(ProviderId::MINIMAX, settings.minimax_api_key.as_deref());
        catalog.insert_anthropic_provider(
            ProviderId::MINIMAX_CODING_PLAN,
            settings.minimax_coding_plan_api_key.as_deref(),
        );
        catalog.insert_anthropic_provider(
            ProviderId::MINIMAX_CN,
            settings.minimax_cn_api_key.as_deref(),
        );
        catalog.insert_anthropic_provider(
            ProviderId::MINIMAX_CN_CODING_PLAN,
            settings.minimax_cn_coding_plan_api_key.as_deref(),
        );
        catalog.insert_provider(nvidia::provider_definition(
            settings.nvidia_api_key.as_deref(),
        ));
        catalog.insert_provider(ollama::provider_definition(&settings.ollama_base_url));
        catalog.insert_provider(local_provider_alias(&settings.ollama_base_url));

        if let Some(hosted_openrouter) = hosted_openrouter {
            for model in &hosted_openrouter.models {
                catalog.insert_model(model.definition.clone());
            }
        } else {
            catalog.insert_model(openrouter::model_definition(
                settings::DEFAULT_MODEL,
                Some("Gemini 3.5 Flash".to_string()),
            ));
            for model in settings::recommended_model_ids()
                .into_iter()
                .filter(|model| *model != settings::DEFAULT_MODEL)
            {
                catalog.insert_model(openrouter::model_definition(model, None));
            }
        }
        catalog.insert_hosted_catalog(hosted_openai);
        catalog.insert_hosted_catalog(hosted_anthropic);
        if let Some(hosted_ollama_cloud) = hosted_ollama_cloud {
            for model in &hosted_ollama_cloud.models {
                catalog.insert_model(model.definition.clone());
            }
        }
        if let Some(hosted_lmstudio) = hosted_lmstudio {
            for model in &hosted_lmstudio.models {
                catalog.insert_model(model.definition.clone());
            }
        }
        if let Some(hosted_deepseek) = hosted_deepseek {
            for model in &hosted_deepseek.models {
                catalog.insert_model(model.definition.clone());
            }
        }
        if let Some(hosted_zai) = hosted_zai {
            for model in &hosted_zai.models {
                catalog.insert_model(model.definition.clone());
            }
        }
        if let Some(hosted_moonshot) = hosted_moonshot {
            for model in &hosted_moonshot.models {
                catalog.insert_model(model.definition.clone());
            }
        }
        if let Some(hosted_moonshot_cn) = hosted_moonshot_cn {
            for model in &hosted_moonshot_cn.models {
                catalog.insert_model(model.definition.clone());
            }
        }
        catalog.insert_hosted_catalog(hosted_kimi_for_coding);
        catalog.insert_hosted_catalog(hosted_minimax);
        catalog.insert_hosted_catalog(hosted_minimax_coding_plan);
        catalog.insert_hosted_catalog(hosted_minimax_cn);
        catalog.insert_hosted_catalog(hosted_minimax_cn_coding_plan);
        catalog.insert_hosted_catalog(hosted_nvidia);
        for model in &settings.custom_models {
            if let Ok(model_ref) = model_ref_from_selection(model, &catalog) {
                catalog.ensure_model_for_ref(&model_ref);
            }
        }
        catalog.default_model = Some(ModelRef::new(
            ProviderId::unchecked(ProviderId::OPENROUTER),
            ModelId::new(settings::DEFAULT_MODEL).expect("default model id is valid"),
        ));
        catalog
    }

    pub(crate) fn provider(&self, id: &ProviderId) -> Option<&ProviderDefinition> {
        self.providers.get(id)
    }

    pub(crate) fn resolve(
        &self,
        model_ref: &ModelRef,
    ) -> Result<ResolvedModel, super::error::LlmError> {
        let provider = self
            .providers
            .get(&model_ref.provider)
            .cloned()
            .ok_or_else(|| super::error::LlmError::ProviderNotFound {
                provider: model_ref.provider.to_string(),
            })?;
        if provider.disabled {
            return Err(super::error::LlmError::ProviderApi {
                provider: provider.id.to_string(),
                status: None,
                message: format!("{} is disabled.", provider.name),
                retryable: false,
            });
        }

        let mut model = self
            .models
            .get(&(model_ref.provider.clone(), model_ref.model.clone()))
            .cloned()
            .unwrap_or_else(|| dynamic_model(&model_ref.provider, &model_ref.model));
        if let Some(limits) = self.local_model_limits.get(&model_ref.canonical()) {
            model.limits.merge_defined(limits);
        }
        if !model.enabled {
            return Err(super::error::LlmError::ModelNotFound {
                provider: provider.id.to_string(),
                model: model.id.to_string(),
            });
        }

        Ok(resolve_model(provider, model, model_ref.clone()))
    }

    pub(crate) fn default_model(&self) -> Option<&ModelRef> {
        self.default_model.as_ref()
    }

    fn insert_provider(&mut self, provider: ProviderDefinition) {
        self.providers.insert(provider.id.clone(), provider);
    }

    fn insert_model(&mut self, model: ModelDefinition) {
        self.models
            .insert((model.provider.clone(), model.id.clone()), model);
    }

    fn insert_anthropic_provider(&mut self, provider_id: &str, api_key: Option<&str>) {
        if let Some(provider) = anthropic_providers::provider_definition(provider_id, api_key) {
            self.insert_provider(provider);
        }
    }

    fn insert_hosted_catalog(&mut self, catalog: Option<&OpenRouterCatalog>) {
        if let Some(catalog) = catalog {
            for model in &catalog.models {
                self.insert_model(model.definition.clone());
            }
        }
    }

    fn ensure_model_for_ref(&mut self, model_ref: &ModelRef) {
        if self
            .models
            .contains_key(&(model_ref.provider.clone(), model_ref.model.clone()))
        {
            return;
        }
        self.insert_model(dynamic_model(&model_ref.provider, &model_ref.model));
    }
}

pub(crate) fn model_ref_from_selection(
    model: &str,
    catalog: &Catalog,
) -> Result<ModelRef, super::error::LlmError> {
    let clean = model.trim();
    if clean.is_empty() {
        return catalog
            .default_model()
            .cloned()
            .ok_or_else(|| super::error::LlmError::Config {
                message: "No default chat model is configured.".to_string(),
            });
    }

    if let Ok(parsed) = ModelRef::parse(clean) {
        if catalog.provider(&parsed.provider).is_some() {
            return Ok(parsed);
        }
    }

    ModelId::new(clean)
        .map(|model| ModelRef::new(ProviderId::unchecked(ProviderId::OPENROUTER), model))
        .ok_or_else(|| super::error::LlmError::Config {
            message: "Model id is empty.".to_string(),
        })
}

fn resolve_model(
    provider: ProviderDefinition,
    model: ModelDefinition,
    reference: ModelRef,
) -> ResolvedModel {
    let request = provider.request.merged(&model.request);
    ResolvedModel {
        reference,
        provider,
        model,
        request,
    }
}

fn dynamic_model(provider_id: &ProviderId, model_id: &ModelId) -> ModelDefinition {
    match provider_id.as_str() {
        ProviderId::OPENAI => openai::model_definition(model_id.as_str(), None),
        ProviderId::ANTHROPIC => anthropic::model_definition(model_id.as_str(), None),
        ProviderId::OPENROUTER => openrouter::model_definition(model_id.as_str(), None),
        ProviderId::OLLAMA => ollama::model_definition(model_id.as_str(), None),
        ProviderId::OLLAMA_CLOUD => ollama_cloud::model_definition(model_id.as_str(), None),
        ProviderId::LMSTUDIO => lmstudio::model_definition(model_id.as_str(), None),
        ProviderId::DEEPSEEK => deepseek::model_definition(model_id.as_str(), None),
        ProviderId::ZAI => zai::model_definition(model_id.as_str(), None),
        ProviderId::NVIDIA => nvidia::model_definition(model_id.as_str(), None),
        ProviderId::MOONSHOTAI | ProviderId::MOONSHOTAI_CN => {
            moonshot::model_definition(provider_id.as_str(), model_id.as_str(), None)
        }
        provider if anthropic_providers::is_anthropic_provider(provider) => {
            anthropic_providers::model_definition(provider, model_id.as_str(), None)
        }
        ProviderId::LOCAL => {
            let mut model = ollama::model_definition(model_id.as_str(), None);
            model.provider = ProviderId::unchecked(ProviderId::LOCAL);
            model
        }
        _ => ModelDefinition {
            id: model_id.clone(),
            provider: provider_id.clone(),
            display_name: model_id.to_string(),
            adapter_model_id: model_id.to_string(),
            capabilities: super::types::Capabilities::text_tools(),
            limits: super::types::Limits::default(),
            request: super::types::RequestDefaults::default(),
            enabled: true,
        },
    }
}

fn local_provider_alias(base_url: &str) -> ProviderDefinition {
    let mut provider = ollama::provider_definition(base_url);
    provider.id = ProviderId::unchecked(ProviderId::LOCAL);
    provider.name = "Local OpenAI-compatible".to_string();
    provider
}

fn key_or_env_present(key: Option<&String>, env_var: &str) -> bool {
    key.is_some_and(|key| !key.trim().is_empty())
        || std::env::var(env_var)
            .ok()
            .is_some_and(|key| !key.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::super::types::AdapterKind;
    use super::*;

    fn test_catalog() -> Catalog {
        Catalog::from_settings(&ProviderSettings {
            openai_api_key: None,
            anthropic_api_key: None,
            openrouter_api_key: None,
            ollama_cloud_api_key: None,
            lmstudio_api_key: None,
            deepseek_api_key: None,
            zai_api_key: None,
            moonshot_api_key: None,
            moonshot_cn_api_key: None,
            kimi_api_key: None,
            minimax_api_key: None,
            minimax_coding_plan_api_key: None,
            minimax_cn_api_key: None,
            minimax_cn_coding_plan_api_key: None,
            nvidia_api_key: None,
            ollama_base_url: "http://127.0.0.1:11434".to_string(),
            lmstudio_base_url: lmstudio::DEFAULT_BASE_URL.to_string(),
            custom_models: Vec::new(),
            local_model_limits: BTreeMap::new(),
        })
    }

    #[test]
    fn canonical_model_ref_uses_provider_segment() {
        let catalog = test_catalog();
        let model_ref =
            model_ref_from_selection("openrouter/google/gemini-3.5-flash", &catalog).unwrap();

        assert_eq!(model_ref.provider.as_str(), "openrouter");
        assert_eq!(model_ref.model.as_str(), "google/gemini-3.5-flash");
    }

    #[test]
    fn legacy_openrouter_model_ids_still_resolve() {
        let catalog = test_catalog();
        let model_ref = model_ref_from_selection("google/gemini-3.5-flash", &catalog).unwrap();

        assert_eq!(model_ref.provider.as_str(), "openrouter");
        assert_eq!(model_ref.model.as_str(), "google/gemini-3.5-flash");
    }

    #[test]
    fn local_model_limits_apply_to_dynamic_models() {
        let mut local_model_limits = BTreeMap::new();
        local_model_limits.insert(
            "ollama/llama3.1:8b".to_string(),
            Limits {
                context_tokens: Some(8192),
                input_tokens: None,
                output_tokens: None,
            },
        );
        local_model_limits.insert(
            "lmstudio/google/gemma-4-26b-a4b".to_string(),
            Limits {
                context_tokens: Some(4096),
                input_tokens: None,
                output_tokens: None,
            },
        );
        let catalog = Catalog::from_settings(&ProviderSettings {
            openai_api_key: None,
            anthropic_api_key: None,
            openrouter_api_key: None,
            ollama_cloud_api_key: None,
            lmstudio_api_key: None,
            deepseek_api_key: None,
            zai_api_key: None,
            moonshot_api_key: None,
            moonshot_cn_api_key: None,
            kimi_api_key: None,
            minimax_api_key: None,
            minimax_coding_plan_api_key: None,
            minimax_cn_api_key: None,
            minimax_cn_coding_plan_api_key: None,
            nvidia_api_key: None,
            ollama_base_url: "http://127.0.0.1:11434".to_string(),
            lmstudio_base_url: lmstudio::DEFAULT_BASE_URL.to_string(),
            custom_models: Vec::new(),
            local_model_limits,
        });

        let ollama_ref = model_ref_from_selection("ollama/llama3.1:8b", &catalog).unwrap();
        let lmstudio_ref =
            model_ref_from_selection("lmstudio/google/gemma-4-26b-a4b", &catalog).unwrap();

        assert_eq!(
            catalog
                .resolve(&ollama_ref)
                .unwrap()
                .model
                .limits
                .context_tokens,
            Some(8192)
        );
        assert_eq!(
            catalog
                .resolve(&lmstudio_ref)
                .unwrap()
                .model
                .limits
                .context_tokens,
            Some(4096)
        );
    }

    #[test]
    fn moonshot_model_refs_resolve_to_native_provider() {
        let catalog = test_catalog();
        let model_ref = model_ref_from_selection("moonshotai/kimi-k2.7-code", &catalog).unwrap();
        let resolved = catalog.resolve(&model_ref).unwrap();

        assert_eq!(model_ref.provider.as_str(), ProviderId::MOONSHOTAI);
        assert_eq!(model_ref.model.as_str(), "kimi-k2.7-code");
        assert_eq!(resolved.model.adapter_model_id, "kimi-k2.7-code");
        assert_eq!(
            resolved.provider.base_url.as_deref(),
            Some(moonshot::API_BASE)
        );
    }

    #[test]
    fn nvidia_model_refs_resolve_to_hosted_nim_provider() {
        let catalog = test_catalog();
        let model_ref =
            model_ref_from_selection("nvidia/meta/llama-3.3-70b-instruct", &catalog).unwrap();
        let resolved = catalog.resolve(&model_ref).unwrap();

        assert_eq!(model_ref.provider.as_str(), ProviderId::NVIDIA);
        assert_eq!(model_ref.model.as_str(), "meta/llama-3.3-70b-instruct");
        assert_eq!(
            resolved.model.adapter_model_id,
            "meta/llama-3.3-70b-instruct"
        );
        assert_eq!(
            resolved.provider.base_url.as_deref(),
            Some(nvidia::API_BASE)
        );
    }

    #[test]
    fn official_openai_and_anthropic_refs_resolve_to_native_providers() {
        let catalog = test_catalog();
        let openai_ref = model_ref_from_selection("openai/gpt-5.1", &catalog).unwrap();
        let openai = catalog.resolve(&openai_ref).unwrap();

        assert_eq!(openai_ref.provider.as_str(), ProviderId::OPENAI);
        assert_eq!(openai.model.adapter_model_id, "gpt-5.1");
        assert_eq!(openai.provider.adapter, AdapterKind::OpenAiCompatibleChat);
        assert_eq!(openai.provider.base_url.as_deref(), Some(openai::API_BASE));

        let anthropic_ref =
            model_ref_from_selection("anthropic/claude-sonnet-4.5", &catalog).unwrap();
        let anthropic_resolved = catalog.resolve(&anthropic_ref).unwrap();

        assert_eq!(anthropic_ref.provider.as_str(), ProviderId::ANTHROPIC);
        assert_eq!(
            anthropic_resolved.model.adapter_model_id,
            "claude-sonnet-4.5"
        );
        assert_eq!(
            anthropic_resolved.provider.adapter,
            AdapterKind::AnthropicCompatibleMessages
        );
        assert_eq!(
            anthropic_resolved.provider.base_url.as_deref(),
            Some(anthropic::API_BASE)
        );
    }

    #[test]
    fn anthropic_provider_model_refs_resolve_to_native_provider() {
        let catalog = test_catalog();
        let kimi_ref = model_ref_from_selection("kimi-for-coding/k2p7", &catalog).unwrap();
        let kimi = catalog.resolve(&kimi_ref).unwrap();

        assert_eq!(kimi_ref.provider.as_str(), ProviderId::KIMI_FOR_CODING);
        assert_eq!(kimi.model.adapter_model_id, "k2p7");
        assert_eq!(
            kimi.provider.base_url.as_deref(),
            Some(anthropic_providers::KIMI_API_BASE)
        );

        for (selection, provider_id, base_url) in [
            (
                "minimax/MiniMax-M3",
                ProviderId::MINIMAX,
                anthropic_providers::MINIMAX_API_BASE,
            ),
            (
                "minimax-coding-plan/MiniMax-M3",
                ProviderId::MINIMAX_CODING_PLAN,
                anthropic_providers::MINIMAX_API_BASE,
            ),
            (
                "minimax-cn/MiniMax-M3",
                ProviderId::MINIMAX_CN,
                anthropic_providers::MINIMAX_CN_API_BASE,
            ),
            (
                "minimax-cn-coding-plan/MiniMax-M3",
                ProviderId::MINIMAX_CN_CODING_PLAN,
                anthropic_providers::MINIMAX_CN_API_BASE,
            ),
        ] {
            let model_ref = model_ref_from_selection(selection, &catalog).unwrap();
            let resolved = catalog.resolve(&model_ref).unwrap();
            assert_eq!(model_ref.provider.as_str(), provider_id);
            assert_eq!(resolved.model.adapter_model_id, "MiniMax-M3");
            assert_eq!(
                resolved.provider.adapter,
                AdapterKind::AnthropicCompatibleMessages
            );
            assert_eq!(resolved.provider.base_url.as_deref(), Some(base_url));
        }
    }
}
