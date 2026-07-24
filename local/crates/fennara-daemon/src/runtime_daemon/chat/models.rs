use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::Duration;

use super::auth;
use super::providers;
use super::providers::ProviderId;
use super::providers::catalog_cache;
use super::providers::models_dev::{OpenRouterCatalog, OpenRouterCatalogModel};
use super::settings::{self, ChatSettings};

const LOCAL_MODELS_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ModelCatalog {
    pub(crate) models: Vec<ModelInfo>,
    pub(crate) recommended_ids: Vec<&'static str>,
    pub(crate) live: bool,
    pub(crate) error: Option<String>,
    pub(crate) catalog_status: CatalogStatus,
    pub(crate) ollama_status: OllamaStatus,
    pub(crate) local_provider_statuses: BTreeMap<String, LocalProviderStatus>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct CatalogStatus {
    pub(crate) provider: &'static str,
    pub(crate) state: &'static str,
    pub(crate) source_url: Option<String>,
    pub(crate) fetched_at_ms: Option<u128>,
    pub(crate) age_ms: Option<u128>,
    pub(crate) using_stale: bool,
    pub(crate) openrouter_model_count: usize,
    pub(crate) last_error: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct OllamaStatus {
    pub(crate) state: &'static str,
    pub(crate) base_url: String,
    pub(crate) model_count: usize,
    pub(crate) error: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct LocalProviderStatus {
    pub(crate) state: &'static str,
    pub(crate) base_url: String,
    pub(crate) model_count: usize,
    pub(crate) error: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ModelInfo {
    pub(crate) id: String,
    pub(crate) display_name: String,
    pub(crate) provider_id: String,
    pub(crate) provider: String,
    pub(crate) source: &'static str,
    pub(crate) recommended: bool,
    pub(crate) custom: bool,
    pub(crate) verified: bool,
    pub(crate) latest_alias: bool,
    pub(crate) canonical_slug: Option<String>,
    pub(crate) context_length: Option<u64>,
    pub(crate) max_output_tokens: Option<u64>,
    pub(crate) input_cost_per_million: Option<f64>,
    pub(crate) output_cost_per_million: Option<f64>,
    pub(crate) cache_read_cost_per_million: Option<f64>,
    pub(crate) cache_write_cost_per_million: Option<f64>,
    pub(crate) tokens_per_second: Option<f64>,
    pub(crate) modalities: Vec<String>,
    pub(crate) supports_tools: bool,
    pub(crate) supports_reasoning: bool,
    pub(crate) supported_reasoning_efforts: Vec<String>,
    pub(crate) description: Option<String>,
}

pub(crate) async fn list_models(settings: &ChatSettings, refresh_local: bool) -> ModelCatalog {
    let recommended_ids = settings::recommended_model_ids();
    catalog_cache::spawn_refresh_if_stale();
    let has_openai_key = auth::has_api_key(ProviderId::OPENAI)
        || std::env::var("OPENAI_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_anthropic_key = auth::has_api_key(ProviderId::ANTHROPIC)
        || std::env::var("ANTHROPIC_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_saved_openrouter_key = auth::has_api_key(ProviderId::OPENROUTER)
        || std::env::var("OPENROUTER_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_ollama_cloud_key = auth::has_api_key(ProviderId::OLLAMA_CLOUD)
        || std::env::var("OLLAMA_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_zai_key = auth::has_api_key(ProviderId::ZAI)
        || std::env::var("ZHIPU_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_deepseek_key = auth::has_api_key(ProviderId::DEEPSEEK)
        || std::env::var("DEEPSEEK_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_moonshot_key = auth::has_api_key(ProviderId::MOONSHOTAI)
        || std::env::var("MOONSHOT_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_moonshot_cn_key = auth::has_api_key(ProviderId::MOONSHOTAI_CN)
        || std::env::var("MOONSHOT_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_kimi_key = auth::has_api_key(ProviderId::KIMI_FOR_CODING)
        || std::env::var("KIMI_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_minimax_key = auth::has_api_key(ProviderId::MINIMAX)
        || std::env::var("MINIMAX_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_minimax_coding_plan_key = auth::has_api_key(ProviderId::MINIMAX_CODING_PLAN)
        || has_minimax_key
        || std::env::var("MINIMAX_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_minimax_cn_key = auth::has_api_key(ProviderId::MINIMAX_CN)
        || std::env::var("MINIMAX_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_minimax_cn_coding_plan_key = auth::has_api_key(ProviderId::MINIMAX_CN_CODING_PLAN)
        || has_minimax_cn_key
        || std::env::var("MINIMAX_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let has_nvidia_key = auth::has_api_key(ProviderId::NVIDIA)
        || std::env::var("NVIDIA_API_KEY")
            .ok()
            .is_some_and(|key| !key.trim().is_empty());
    let cached_catalog = catalog_cache::load_disk().await;
    let catalog_status = catalog_status(&cached_catalog);
    let openrouter_error = cached_catalog.as_ref().err().cloned();
    let needs_hosted_catalog = has_saved_openrouter_key
        || has_openai_key
        || has_anthropic_key
        || has_ollama_cloud_key
        || has_deepseek_key
        || has_zai_key
        || has_moonshot_key
        || has_moonshot_cn_key
        || has_kimi_key
        || has_minimax_key
        || has_minimax_coding_plan_key
        || has_minimax_cn_key
        || has_minimax_cn_coding_plan_key
        || has_nvidia_key;
    let mut models = Vec::new();
    if has_saved_openrouter_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_openrouter_catalog_models(
                &mut models,
                &cached_catalog.catalog,
                &recommended_ids,
            );
        }
    }
    if has_openai_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.openai,
                "OpenAI",
                ProviderId::OPENAI,
            );
        }
    }
    if has_anthropic_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.anthropic,
                "Anthropic",
                ProviderId::ANTHROPIC,
            );
        }
    }
    if has_ollama_cloud_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.ollama_cloud,
                "Ollama Cloud",
                ProviderId::OLLAMA_CLOUD,
            );
        }
    }
    if has_zai_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(&mut models, &cached_catalog.zai, "Z.AI", ProviderId::ZAI);
        }
    }
    if has_deepseek_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.deepseek,
                "DeepSeek",
                ProviderId::DEEPSEEK,
            );
        }
    }
    if has_moonshot_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.moonshot,
                "Moonshot AI",
                ProviderId::MOONSHOTAI,
            );
        }
    }
    if has_moonshot_cn_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.moonshot_cn,
                "Moonshot AI (China)",
                ProviderId::MOONSHOTAI_CN,
            );
        }
    }
    if has_kimi_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.kimi_for_coding,
                "Kimi For Coding",
                ProviderId::KIMI_FOR_CODING,
            );
        }
    }
    if has_minimax_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.minimax,
                "MiniMax (minimax.io)",
                ProviderId::MINIMAX,
            );
        }
    }
    if has_minimax_coding_plan_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.minimax_coding_plan,
                "MiniMax Token Plan (minimax.io)",
                ProviderId::MINIMAX_CODING_PLAN,
            );
        }
    }
    if has_minimax_cn_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.minimax_cn,
                "MiniMax (minimaxi.com)",
                ProviderId::MINIMAX_CN,
            );
        }
    }
    if has_minimax_cn_coding_plan_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.minimax_cn_coding_plan,
                "MiniMax Token Plan (minimaxi.com)",
                ProviderId::MINIMAX_CN_CODING_PLAN,
            );
        }
    }
    if has_nvidia_key {
        if let Ok(cached_catalog) = &cached_catalog {
            append_hosted_catalog_models(
                &mut models,
                &cached_catalog.nvidia,
                "NVIDIA",
                ProviderId::NVIDIA,
            );
        }
    }
    for provider in &settings.custom_providers {
        append_custom_provider_models(&mut models, provider);
    }

    let lmstudio_base_url = settings.provider_base_url(
        ProviderId::LMSTUDIO,
        providers::lmstudio_v1_base_url("").as_str(),
    );
    let (ollama_status, lmstudio_status) = if refresh_local {
        let ollama_result = tokio::time::timeout(
            LOCAL_MODELS_TIMEOUT,
            fetch_ollama_models(&settings.ollama_base_url),
        )
        .await
        .unwrap_or_else(|_| Err("Ollama models request timed out.".to_string()));
        let ollama_status = match ollama_result {
            Ok(ollama_models) => {
                append_ollama_models(&mut models, &ollama_models);
                OllamaStatus {
                    state: if ollama_models.is_empty() {
                        "empty"
                    } else {
                        "ready"
                    },
                    base_url: settings.ollama_base_url.clone(),
                    model_count: ollama_models.len(),
                    error: None,
                }
            }
            Err(error) => OllamaStatus {
                state: "offline",
                base_url: settings.ollama_base_url.clone(),
                model_count: 0,
                error: Some(error),
            },
        };

        let lmstudio_result = tokio::time::timeout(
            LOCAL_MODELS_TIMEOUT,
            fetch_lmstudio_models(&lmstudio_base_url),
        )
        .await
        .unwrap_or_else(|_| Err("LM Studio models request timed out.".to_string()));
        let lmstudio_status = match lmstudio_result {
            Ok(lmstudio_models) => {
                let catalog = cached_catalog.as_ref().ok().map(|cached| &cached.lmstudio);
                append_lmstudio_models(&mut models, &lmstudio_models, catalog);
                LocalProviderStatus {
                    state: if lmstudio_models.is_empty() {
                        "empty"
                    } else {
                        "ready"
                    },
                    base_url: lmstudio_base_url.clone(),
                    model_count: lmstudio_models.len(),
                    error: None,
                }
            }
            Err(error) => LocalProviderStatus {
                state: "offline",
                base_url: lmstudio_base_url.clone(),
                model_count: 0,
                error: Some(error),
            },
        };
        (ollama_status, lmstudio_status)
    } else {
        (
            OllamaStatus {
                state: "unknown",
                base_url: settings.ollama_base_url.clone(),
                model_count: 0,
                error: None,
            },
            LocalProviderStatus {
                state: "unknown",
                base_url: lmstudio_base_url.clone(),
                model_count: 0,
                error: None,
            },
        )
    };

    let mut local_provider_statuses = BTreeMap::new();
    local_provider_statuses.insert(
        ProviderId::OLLAMA.to_string(),
        LocalProviderStatus {
            state: ollama_status.state,
            base_url: ollama_status.base_url.clone(),
            model_count: ollama_status.model_count,
            error: ollama_status.error.clone(),
        },
    );
    local_provider_statuses.insert(ProviderId::LMSTUDIO.to_string(), lmstudio_status);

    let ollama_live = matches!(ollama_status.state, "ready" | "empty");
    let local_live = local_provider_statuses
        .values()
        .any(|status| matches!(status.state, "ready" | "empty"));
    let custom_live = !settings.custom_providers.is_empty();
    let live = openrouter_error.is_none() || ollama_live || local_live || custom_live;
    ModelCatalog {
        models,
        recommended_ids,
        live,
        error: if needs_hosted_catalog {
            openrouter_error
        } else {
            None
        },
        catalog_status,
        ollama_status,
        local_provider_statuses,
    }
}

pub(crate) async fn refresh_model_catalog(force: bool) -> Result<CatalogStatus, String> {
    let refreshed = catalog_cache::refresh(force).await;
    let status = catalog_status(&refreshed);
    refreshed.map(|_| status)
}

pub(crate) fn spawn_catalog_refresh_if_needed() {
    catalog_cache::spawn_refresh_if_stale();
}

fn catalog_status(
    cached_catalog: &Result<catalog_cache::CachedOpenRouterCatalog, String>,
) -> CatalogStatus {
    match cached_catalog {
        Ok(cached) => CatalogStatus {
            provider: "openrouter",
            state: if cached.stale { "stale" } else { "ready" },
            source_url: Some(cached.meta.source_url.clone()),
            fetched_at_ms: Some(cached.meta.fetched_at_ms),
            age_ms: cached.meta.age_ms(),
            using_stale: cached.stale,
            openrouter_model_count: cached.catalog.models.len(),
            last_error: None,
        },
        Err(error) => CatalogStatus {
            provider: "openrouter",
            state: "empty",
            source_url: Some(catalog_cache::DEFAULT_MODELS_DEV_URL.to_string()),
            fetched_at_ms: None,
            age_ms: None,
            using_stale: false,
            openrouter_model_count: 0,
            last_error: Some(error.clone()),
        },
    }
}

fn append_openrouter_catalog_models(
    models: &mut Vec<ModelInfo>,
    catalog: &OpenRouterCatalog,
    recommended_ids: &[&'static str],
) {
    for model in &catalog.models {
        let id = model.definition.id.as_str();
        let prefixed_id = openrouter_model_id(id);
        models.push(openrouter_catalog_model_info(
            model,
            recommended_ids.contains(&prefixed_id.as_str()),
        ));
    }
}

fn append_hosted_catalog_models(
    models: &mut Vec<ModelInfo>,
    catalog: &OpenRouterCatalog,
    provider_label: &str,
    provider_id: &str,
) {
    for catalog_model in &catalog.models {
        let mut model = openrouter_catalog_model_info(catalog_model, false);
        model.id = format!("{provider_id}/{}", catalog_model.definition.id);
        model.provider_id = provider_id.to_string();
        model.provider = provider_label.to_string();
        model.canonical_slug = Some(catalog_model.definition.id.to_string());
        models.push(model);
    }
}

fn append_custom_provider_models(
    models: &mut Vec<ModelInfo>,
    provider: &providers::custom::CustomProviderConfig,
) {
    for configured_model in &provider.models {
        let id = format!("{}/{}", provider.id, configured_model.id);
        models.push(ModelInfo {
            id,
            display_name: configured_model.name.clone(),
            provider_id: provider.id.clone(),
            provider: provider.name.clone(),
            source: "custom",
            recommended: false,
            custom: true,
            verified: false,
            latest_alias: false,
            canonical_slug: Some(configured_model.id.clone()),
            context_length: Some(u64::from(configured_model.context_length)),
            max_output_tokens: Some(u64::from(configured_model.max_output_tokens)),
            input_cost_per_million: None,
            output_cost_per_million: None,
            cache_read_cost_per_million: None,
            cache_write_cost_per_million: None,
            tokens_per_second: None,
            modalities: vec!["in:text".to_string(), "out:text".to_string()],
            supports_tools: true,
            supports_reasoning: false,
            supported_reasoning_efforts: Vec::new(),
            description: Some("Custom OpenAI-compatible provider model.".to_string()),
        });
    }
}

fn openrouter_catalog_model_info(
    catalog_model: &OpenRouterCatalogModel,
    recommended: bool,
) -> ModelInfo {
    let definition = &catalog_model.definition;
    let mut modalities = definition
        .capabilities
        .input
        .iter()
        .map(|modality| format!("in:{modality}"))
        .chain(
            definition
                .capabilities
                .output
                .iter()
                .map(|modality| format!("out:{modality}")),
        )
        .collect::<Vec<_>>();
    modalities.sort();
    modalities.dedup();
    ModelInfo {
        id: openrouter_model_id(definition.id.as_str()),
        display_name: definition.display_name.clone(),
        provider_id: ProviderId::OPENROUTER.to_string(),
        provider: "OpenRouter".to_string(),
        source: "catalog",
        recommended,
        custom: false,
        verified: true,
        latest_alias: false,
        canonical_slug: Some(definition.id.to_string()),
        context_length: definition.limits.context_tokens.map(u64::from),
        max_output_tokens: definition.limits.output_tokens.map(u64::from),
        input_cost_per_million: catalog_model.input_cost_per_million,
        output_cost_per_million: catalog_model.output_cost_per_million,
        cache_read_cost_per_million: catalog_model.cache_read_cost_per_million,
        cache_write_cost_per_million: catalog_model.cache_write_cost_per_million,
        tokens_per_second: None,
        modalities,
        supports_tools: definition.capabilities.tools,
        supports_reasoning: definition.capabilities.reasoning,
        supported_reasoning_efforts: if definition.capabilities.reasoning {
            vec!["low".to_string(), "medium".to_string(), "high".to_string()]
        } else {
            Vec::new()
        },
        description: catalog_model.family.as_ref().map(|family| {
            if catalog_model.status == "beta" {
                format!("{family} family. Beta model.")
            } else {
                format!("{family} family.")
            }
        }),
    }
}

fn openrouter_model_id(model_id: &str) -> String {
    let clean = model_id.trim();
    if clean.starts_with("openrouter/") {
        clean.to_string()
    } else {
        format!("openrouter/{clean}")
    }
}

async fn fetch_ollama_models(base_url: &str) -> Result<Vec<Value>, String> {
    providers::fetch_ollama_models(base_url).await
}

async fn fetch_lmstudio_models(base_url: &str) -> Result<Vec<Value>, String> {
    let api_key = auth::api_key(ProviderId::LMSTUDIO);
    providers::fetch_lmstudio_models(base_url, api_key.as_deref()).await
}

fn append_ollama_models(models: &mut Vec<ModelInfo>, raw_models: &[Value]) {
    for raw in raw_models {
        let Some(local_id) = raw
            .get("name")
            .or_else(|| raw.get("model"))
            .and_then(Value::as_str)
        else {
            continue;
        };
        let id = format!("ollama/{local_id}");
        if models.iter().any(|model| model.id == id) {
            continue;
        }
        models.push(ollama_model_info(&id, Some(raw), true));
    }
}

fn append_lmstudio_models(
    models: &mut Vec<ModelInfo>,
    raw_models: &[Value],
    catalog: Option<&OpenRouterCatalog>,
) {
    for raw in raw_models {
        let Some(local_id) = raw
            .get("key")
            .or_else(|| raw.get("id"))
            .or_else(|| raw.get("model"))
            .and_then(Value::as_str)
        else {
            continue;
        };
        let id = format!("lmstudio/{local_id}");
        if models.iter().any(|model| model.id == id) {
            continue;
        }
        models.push(lmstudio_model_info(
            &id,
            Some(raw),
            catalog.and_then(|catalog| catalog.model(local_id)),
        ));
    }
}

fn ollama_model_info(id: &str, raw: Option<&Value>, verified: bool) -> ModelInfo {
    let local_id = providers::ollama_model_id(id).unwrap_or(id);
    let capabilities = string_array(raw.and_then(|model| model.get("capabilities")));
    let mut modalities = vec!["in:text".to_string(), "out:text".to_string()];
    if capabilities.iter().any(|capability| capability == "vision") {
        modalities.push("in:image".to_string());
    }
    ModelInfo {
        id: id.to_string(),
        display_name: fallback_display_name(local_id),
        provider_id: ProviderId::OLLAMA.to_string(),
        provider: "Ollama".to_string(),
        source: "local",
        recommended: false,
        custom: false,
        verified,
        latest_alias: false,
        canonical_slug: raw
            .and_then(|model| model.get("name").or_else(|| model.get("model")))
            .and_then(Value::as_str)
            .map(ToString::to_string),
        context_length: raw
            .and_then(providers::ollama_context_tokens)
            .map(u64::from),
        max_output_tokens: None,
        input_cost_per_million: Some(0.0),
        output_cost_per_million: Some(0.0),
        cache_read_cost_per_million: Some(0.0),
        cache_write_cost_per_million: Some(0.0),
        tokens_per_second: None,
        modalities,
        supports_tools: capabilities.iter().any(|capability| capability == "tools"),
        supports_reasoning: false,
        supported_reasoning_efforts: Vec::new(),
        description: raw.and_then(ollama_description),
    }
}

fn lmstudio_model_info(
    id: &str,
    raw: Option<&Value>,
    catalog_model: Option<&OpenRouterCatalogModel>,
) -> ModelInfo {
    let local_id = providers::lmstudio_model_id(id).unwrap_or(id);
    if let Some(catalog_model) = catalog_model {
        let mut model = openrouter_catalog_model_info(catalog_model, false);
        model.id = id.to_string();
        model.provider_id = ProviderId::LMSTUDIO.to_string();
        model.provider = "LM Studio".to_string();
        model.source = "local";
        model.verified = true;
        model.canonical_slug = Some(local_id.to_string());
        if let Some(context_length) = raw
            .and_then(|model| providers::lmstudio_context_tokens(model, local_id))
            .map(u64::from)
        {
            model.context_length = Some(context_length);
        }
        return model;
    }

    ModelInfo {
        id: id.to_string(),
        display_name: raw
            .and_then(|model| {
                model
                    .get("display_name")
                    .or_else(|| model.get("key"))
                    .or_else(|| model.get("id"))
                    .or_else(|| model.get("model"))
            })
            .and_then(Value::as_str)
            .map(fallback_display_name)
            .unwrap_or_else(|| fallback_display_name(local_id)),
        provider_id: ProviderId::LMSTUDIO.to_string(),
        provider: "LM Studio".to_string(),
        source: "local",
        recommended: false,
        custom: false,
        verified: true,
        latest_alias: false,
        canonical_slug: Some(local_id.to_string()),
        context_length: raw
            .and_then(|model| providers::lmstudio_context_tokens(model, local_id))
            .map(u64::from),
        max_output_tokens: None,
        input_cost_per_million: Some(0.0),
        output_cost_per_million: Some(0.0),
        cache_read_cost_per_million: Some(0.0),
        cache_write_cost_per_million: Some(0.0),
        tokens_per_second: None,
        modalities: vec!["in:text".to_string(), "out:text".to_string()],
        supports_tools: true,
        supports_reasoning: false,
        supported_reasoning_efforts: Vec::new(),
        description: raw
            .and_then(|model| model.get("description"))
            .and_then(Value::as_str)
            .map(|description| description.trim().to_string())
            .or_else(|| Some("Loaded from the local LM Studio server.".to_string())),
    }
}

fn ollama_description(model: &Value) -> Option<String> {
    let details = model.get("details")?;
    let size = details.get("parameter_size").and_then(Value::as_str)?;
    let quant = details.get("quantization_level").and_then(Value::as_str);
    Some(match quant {
        Some(quant) => format!("{size} local model, {quant}."),
        None => format!("{size} local model."),
    })
}

pub(crate) fn model_supports_text_chat(model: &Value) -> bool {
    let architecture = model.get("architecture");
    if architecture
        .and_then(|value| value.get("modality"))
        .and_then(Value::as_str)
        .is_some_and(|modality| modality.to_ascii_lowercase().contains("text->text"))
    {
        return true;
    }

    let has_text_input = string_array_contains(
        architecture.and_then(|value| value.get("input_modalities")),
        "text",
    ) || string_array_contains(model.get("input_modalities"), "text");
    let has_text_output = string_array_contains(
        architecture.and_then(|value| value.get("output_modalities")),
        "text",
    ) || string_array_contains(model.get("output_modalities"), "text");
    has_text_input && has_text_output
}

pub(crate) fn model_supports_image_chat(model: &Value) -> bool {
    if !model_supports_text_chat(model) {
        return false;
    }
    let architecture = model.get("architecture");
    string_array_contains(
        architecture.and_then(|value| value.get("input_modalities")),
        "image",
    ) || string_array_contains(model.get("input_modalities"), "image")
        || architecture
            .and_then(|value| value.get("modality"))
            .and_then(Value::as_str)
            .is_some_and(|modality| modality.to_ascii_lowercase().contains("image"))
}

fn fallback_display_name(id: &str) -> String {
    id.trim_start_matches('~')
        .split('/')
        .next_back()
        .unwrap_or(id)
        .replace('-', " ")
        .replace("latest", "Latest")
}

fn string_array(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn string_array_contains(value: Option<&Value>, needle: &str) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(needle)))
}

#[cfg(test)]
mod tests {
    use super::super::providers::custom::{CustomProviderConfig, CustomProviderModel};
    use super::super::providers::models_dev::{parse_moonshot_catalog, parse_nvidia_catalog};
    use super::*;
    use serde_json::json;

    #[test]
    fn appends_moonshot_catalog_models_with_native_prefix() {
        let raw = br#"{
            "moonshotai": {
                "id": "moonshotai",
                "models": {
                    "kimi-k2.7-code": {
                        "id": "kimi-k2.7-code",
                        "name": "Kimi K2.7 Code",
                        "tool_call": true,
                        "reasoning": false,
                        "temperature": true,
                        "limit": { "context": 262144, "output": 32768 },
                        "modalities": { "input": ["text"], "output": ["text"] },
                        "cost": { "input": 0.6, "output": 2.5 }
                    }
                }
            }
        }"#;
        let catalog = parse_moonshot_catalog(raw).unwrap();
        let mut models = Vec::new();

        append_hosted_catalog_models(&mut models, &catalog, "Moonshot AI", ProviderId::MOONSHOTAI);

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "moonshotai/kimi-k2.7-code");
        assert_eq!(models[0].provider_id, ProviderId::MOONSHOTAI);
        assert_eq!(models[0].provider, "Moonshot AI");
        assert_eq!(models[0].canonical_slug.as_deref(), Some("kimi-k2.7-code"));
    }

    #[test]
    fn appends_minimax_catalog_models_with_native_prefix() {
        let raw = br#"{
            "minimax": {
                "id": "minimax",
                "models": {
                    "MiniMax-M3": {
                        "id": "MiniMax-M3",
                        "name": "MiniMax-M3",
                        "tool_call": true,
                        "reasoning": false,
                        "temperature": true,
                        "limit": { "context": 1000000, "output": 128000 },
                        "modalities": { "input": ["text", "image"], "output": ["text"] },
                        "cost": { "input": 1.0, "output": 8.0 }
                    }
                }
            }
        }"#;
        let catalog = super::super::providers::models_dev::parse_minimax_catalog(raw).unwrap();
        let mut models = Vec::new();

        append_hosted_catalog_models(
            &mut models,
            &catalog,
            "MiniMax (minimax.io)",
            ProviderId::MINIMAX,
        );

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "minimax/MiniMax-M3");
        assert_eq!(models[0].provider_id, ProviderId::MINIMAX);
        assert_eq!(models[0].provider, "MiniMax (minimax.io)");
        assert_eq!(models[0].canonical_slug.as_deref(), Some("MiniMax-M3"));
    }

    #[test]
    fn appends_nvidia_catalog_models_with_native_prefix() {
        let raw = br#"{
            "nvidia": {
                "id": "nvidia",
                "models": {
                    "meta/llama-3.3-70b-instruct": {
                        "id": "meta/llama-3.3-70b-instruct",
                        "name": "Llama 3.3 70B Instruct",
                        "tool_call": true,
                        "reasoning": false,
                        "temperature": true,
                        "limit": { "context": 131072, "output": 32768 },
                        "modalities": { "input": ["text"], "output": ["text"] }
                    }
                }
            }
        }"#;
        let catalog = parse_nvidia_catalog(raw).unwrap();
        let mut models = Vec::new();

        append_hosted_catalog_models(&mut models, &catalog, "NVIDIA", ProviderId::NVIDIA);

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "nvidia/meta/llama-3.3-70b-instruct");
        assert_eq!(models[0].provider_id, ProviderId::NVIDIA);
        assert_eq!(models[0].provider, "NVIDIA");
        assert_eq!(
            models[0].canonical_slug.as_deref(),
            Some("meta/llama-3.3-70b-instruct")
        );
    }

    #[test]
    fn appends_custom_provider_models_with_provider_prefix() {
        let provider = CustomProviderConfig {
            id: "omniroute".to_string(),
            name: "OmniRoute".to_string(),
            base_url: "http://localhost:20128/v1".to_string(),
            models: vec![CustomProviderModel {
                id: "zai/glm-5".to_string(),
                name: "GLM 5".to_string(),
                context_length: 131_072,
                max_output_tokens: 8_192,
            }],
            headers: BTreeMap::new(),
        };
        let mut models = Vec::new();

        append_custom_provider_models(&mut models, &provider);

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "omniroute/zai/glm-5");
        assert_eq!(models[0].provider_id, "omniroute");
        assert_eq!(models[0].provider, "OmniRoute");
        assert_eq!(models[0].display_name, "GLM 5");
        assert!(models[0].supports_tools);
        assert_eq!(models[0].context_length, Some(131_072));
        assert_eq!(models[0].max_output_tokens, Some(8_192));
    }

    #[test]
    fn ollama_model_info_prefers_running_context_length() {
        let raw = json!({
            "name": "gemma4",
            "details": {
                "parameter_size": "8.0B",
                "quantization_level": "Q4_K_M",
                "context_length": 262144
            },
            "context_length": 4096,
            "capabilities": ["completion", "tools"]
        });

        let model = ollama_model_info("ollama/gemma4", Some(&raw), true);

        assert_eq!(model.context_length, Some(4096));
    }

    #[test]
    fn selected_uncatalogued_lmstudio_model_remains_local() {
        let settings: ChatSettings = serde_json::from_value(json!({
            "model": "lmstudio/qwen/qwen3.6-27b",
            "custom_models": ["lmstudio/qwen/qwen3.6-27b"]
        }))
        .unwrap();
        let raw = json!({
            "type": "llm",
            "key": "qwen/qwen3.6-27b",
            "display_name": "Qwen3.6 27B",
            "loaded_instances": [],
            "max_context_length": 262144
        });
        let mut models = Vec::new();

        append_lmstudio_models(&mut models, &[raw], None);

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, settings.model);
        assert_eq!(models[0].source, "local");
        assert!(!models[0].custom);
        assert!(
            serde_json::to_value(settings)
                .unwrap()
                .get("custom_models")
                .is_none()
        );
    }

    #[test]
    fn lmstudio_model_info_prefers_loaded_instance_context_length() {
        let raw = json!({
            "type": "llm",
            "key": "google/gemma-4-26b-a4b",
            "display_name": "Gemma 4 26B A4B",
            "loaded_instances": [
                {
                    "id": "google/gemma-4-26b-a4b",
                    "config": {
                        "context_length": 4096
                    }
                }
            ],
            "max_context_length": 262144
        });
        let mut models = Vec::new();

        append_lmstudio_models(&mut models, &[raw], None);

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "lmstudio/google/gemma-4-26b-a4b");
        assert_eq!(models[0].context_length, Some(4096));
    }

    #[test]
    fn lmstudio_model_info_uses_max_context_when_unloaded() {
        let raw = json!({
            "type": "llm",
            "key": "deepseek-r1",
            "display_name": "DeepSeek R1",
            "loaded_instances": [],
            "max_context_length": 131072
        });
        let mut models = Vec::new();

        append_lmstudio_models(&mut models, &[raw], None);

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].context_length, Some(131072));
    }
}
