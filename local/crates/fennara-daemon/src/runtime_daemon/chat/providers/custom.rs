use std::collections::{BTreeMap, BTreeSet};

use reqwest::Url;
use reqwest::header::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

use super::types::{
    AdapterKind, Auth, Capabilities, Limits, ModelDefinition, ModelId, ProviderDefinition,
    ProviderId, RequestDefaults,
};

pub(crate) const MAX_CUSTOM_PROVIDERS: usize = 32;
const MAX_MODELS: usize = 100;
const MAX_HEADERS: usize = 32;
const MAX_PROVIDER_ID_LEN: usize = 64;
const MAX_DISPLAY_NAME_LEN: usize = 100;
const MAX_MODEL_ID_LEN: usize = 256;
const MAX_BASE_URL_LEN: usize = 2_048;
const MAX_HEADER_VALUE_LEN: usize = 4_096;
const MAX_API_KEY_LEN: usize = 16_384;

const RESERVED_PROVIDER_IDS: &[&str] = &[
    ProviderId::OPENAI,
    ProviderId::ANTHROPIC,
    ProviderId::OPENROUTER,
    ProviderId::OLLAMA,
    ProviderId::OLLAMA_CLOUD,
    ProviderId::LMSTUDIO,
    ProviderId::DEEPSEEK,
    ProviderId::ZAI,
    ProviderId::MOONSHOTAI,
    ProviderId::MOONSHOTAI_CN,
    ProviderId::KIMI_FOR_CODING,
    ProviderId::MINIMAX,
    ProviderId::MINIMAX_CODING_PLAN,
    ProviderId::MINIMAX_CN,
    ProviderId::MINIMAX_CN_CODING_PLAN,
    ProviderId::NVIDIA,
    ProviderId::LOCAL,
];

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct CustomProviderConfig {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) base_url: String,
    pub(crate) models: Vec<CustomProviderModel>,
    #[serde(default)]
    pub(crate) headers: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct CustomProviderModel {
    pub(crate) id: String,
    pub(crate) name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SaveCustomProviderRequest {
    #[serde(default)]
    pub(crate) update_existing: bool,
    pub(crate) provider_id: String,
    pub(crate) display_name: String,
    pub(crate) base_url: String,
    #[serde(default)]
    pub(crate) api_key: Option<String>,
    #[serde(default)]
    pub(crate) models: Vec<CustomProviderModel>,
    #[serde(default)]
    pub(crate) headers: Vec<CustomProviderHeader>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CustomProviderHeader {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Clone, Debug)]
pub(crate) struct CustomProviderRuntime {
    pub(crate) config: CustomProviderConfig,
    pub(crate) api_key: Option<String>,
}

pub(crate) fn validate_new_provider(
    input: SaveCustomProviderRequest,
) -> Result<(CustomProviderConfig, Option<String>), String> {
    let provider_id = input.provider_id.trim();
    if provider_id.is_empty() {
        return Err("Provider ID is required.".to_string());
    }
    if provider_id.len() > MAX_PROVIDER_ID_LEN || !valid_provider_id(provider_id) {
        return Err(
            "Provider ID must use lowercase letters, numbers, hyphens, or underscores, and start with a letter or number."
                .to_string(),
        );
    }
    if is_reserved_provider_id(provider_id) {
        return Err(format!(
            "Provider ID {provider_id} is already built into Fennara."
        ));
    }

    let display_name = required_bounded(&input.display_name, "Display name", MAX_DISPLAY_NAME_LEN)?;
    let base_url = clean_base_url(&input.base_url)?;
    let models = clean_models(input.models)?;
    let headers = clean_headers(input.headers)?;
    let api_key = input
        .api_key
        .map(|key| key.trim().to_string())
        .filter(|key| !key.is_empty());
    if api_key
        .as_ref()
        .is_some_and(|key| key.len() > MAX_API_KEY_LEN)
    {
        return Err("API key is too long.".to_string());
    }

    Ok((
        CustomProviderConfig {
            id: provider_id.to_string(),
            name: display_name,
            base_url,
            models,
            headers,
        },
        api_key,
    ))
}

pub(crate) fn clean_saved_providers(
    providers: &[CustomProviderConfig],
) -> Vec<CustomProviderConfig> {
    let mut clean = Vec::new();
    let mut seen = BTreeSet::new();
    for provider in providers {
        if clean.len() >= MAX_CUSTOM_PROVIDERS {
            break;
        }
        let provider_id = provider.id.trim().to_string();
        if seen.contains(&provider_id) {
            continue;
        }
        let request = SaveCustomProviderRequest {
            update_existing: false,
            provider_id: provider.id.clone(),
            display_name: provider.name.clone(),
            base_url: provider.base_url.clone(),
            api_key: None,
            models: provider.models.clone(),
            headers: provider
                .headers
                .iter()
                .map(|(name, value)| CustomProviderHeader {
                    name: name.clone(),
                    value: value.clone(),
                })
                .collect(),
        };
        if let Ok((provider, _)) = validate_new_provider(request) {
            seen.insert(provider_id);
            clean.push(provider);
        }
    }
    clean
}

pub(crate) fn provider_definition(runtime: &CustomProviderRuntime) -> ProviderDefinition {
    ProviderDefinition {
        id: ProviderId::new(&runtime.config.id).expect("saved custom provider id is valid"),
        name: runtime.config.name.clone(),
        adapter: AdapterKind::OpenAiCompatibleChat,
        base_url: Some(runtime.config.base_url.clone()),
        auth: runtime
            .api_key
            .as_deref()
            .filter(|key| !key.trim().is_empty())
            .map(|key| Auth::InlineBearer {
                value: key.trim().to_string(),
            })
            .unwrap_or(Auth::None),
        request: RequestDefaults {
            headers: runtime.config.headers.clone(),
            ..RequestDefaults::default()
        },
        disabled: false,
    }
}

pub(crate) fn model_definitions(config: &CustomProviderConfig) -> Vec<ModelDefinition> {
    let provider = ProviderId::new(&config.id).expect("saved custom provider id is valid");
    config
        .models
        .iter()
        .map(|model| ModelDefinition {
            id: ModelId::new(&model.id).expect("saved custom model id is valid"),
            provider: provider.clone(),
            display_name: model.name.clone(),
            adapter_model_id: model.id.clone(),
            capabilities: Capabilities::text_tools(),
            limits: Limits::default(),
            request: RequestDefaults::default(),
            enabled: true,
        })
        .collect()
}

pub(crate) fn split_model_selection<'a>(
    providers: &'a [CustomProviderConfig],
    selection: &'a str,
) -> Option<(&'a str, &'a str)> {
    let (provider_id, model_id) = selection.trim().split_once('/')?;
    providers
        .iter()
        .any(|provider| {
            provider.id == provider_id && provider.models.iter().any(|model| model.id == model_id)
        })
        .then_some((provider_id, model_id.trim()))
        .filter(|(_, model_id)| !model_id.is_empty())
}

pub(crate) fn is_reserved_provider_id(provider_id: &str) -> bool {
    RESERVED_PROVIDER_IDS.contains(&provider_id)
}

fn valid_provider_id(provider_id: &str) -> bool {
    provider_id
        .bytes()
        .next()
        .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && provider_id.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'_')
        })
}

fn clean_base_url(value: &str) -> Result<String, String> {
    let value = required_bounded(value, "Base URL", MAX_BASE_URL_LEN)?;
    let url = Url::parse(&value).map_err(|_| "Base URL must be a valid URL.".to_string())?;
    if !matches!(url.scheme(), "http" | "https") || !url.has_host() {
        return Err("Base URL must start with http:// or https:// and include a host.".to_string());
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err("Base URL must not contain embedded credentials.".to_string());
    }
    if url.query().is_some() || url.fragment().is_some() {
        return Err("Base URL must not contain a query string or fragment.".to_string());
    }
    Ok(value.trim_end_matches('/').to_string())
}

fn clean_models(models: Vec<CustomProviderModel>) -> Result<Vec<CustomProviderModel>, String> {
    if models.is_empty() {
        return Err("Add at least one model.".to_string());
    }
    if models.len() > MAX_MODELS {
        return Err(format!(
            "A custom provider can contain at most {MAX_MODELS} models."
        ));
    }
    let mut clean = Vec::with_capacity(models.len());
    let mut seen = BTreeSet::new();
    for model in models {
        let id = required_bounded(&model.id, "Model ID", MAX_MODEL_ID_LEN)?;
        let name = required_bounded(&model.name, "Model display name", MAX_DISPLAY_NAME_LEN)?;
        if !seen.insert(id.clone()) {
            return Err(format!("Model ID {id} is duplicated."));
        }
        clean.push(CustomProviderModel { id, name });
    }
    Ok(clean)
}

fn clean_headers(headers: Vec<CustomProviderHeader>) -> Result<BTreeMap<String, String>, String> {
    if headers.len() > MAX_HEADERS {
        return Err(format!(
            "A custom provider can contain at most {MAX_HEADERS} headers."
        ));
    }
    let mut clean = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for header in headers {
        let name = header.name.trim();
        let value = header.value.trim();
        if name.is_empty() && value.is_empty() {
            continue;
        }
        if name.is_empty() || value.is_empty() {
            return Err("Custom headers require both a name and value.".to_string());
        }
        let parsed_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| format!("Header name {name} is invalid."))?;
        HeaderValue::from_str(value).map_err(|_| format!("Header {name} has an invalid value."))?;
        if value.len() > MAX_HEADER_VALUE_LEN {
            return Err(format!("Header {name} value is too long."));
        }
        let normalized_name = parsed_name.as_str().to_string();
        if !seen.insert(normalized_name.clone()) {
            return Err(format!("Header {name} is duplicated."));
        }
        clean.insert(normalized_name, value.to_string());
    }
    Ok(clean)
}

fn required_bounded(value: &str, label: &str, max_len: usize) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("{label} is required."));
    }
    if value.len() > max_len {
        return Err(format!("{label} is too long."));
    }
    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn omniroute_request() -> SaveCustomProviderRequest {
        SaveCustomProviderRequest {
            update_existing: false,
            provider_id: "omniroute".to_string(),
            display_name: "OmniRoute".to_string(),
            base_url: "http://localhost:20128/v1/".to_string(),
            api_key: Some(" secret ".to_string()),
            models: vec![CustomProviderModel {
                id: "zai/glm-5".to_string(),
                name: "GLM 5".to_string(),
            }],
            headers: vec![CustomProviderHeader {
                name: "X-Router".to_string(),
                value: "primary".to_string(),
            }],
        }
    }

    #[test]
    fn validates_omniroute_and_builds_openai_compatible_definitions() {
        let (config, api_key) = validate_new_provider(omniroute_request()).unwrap();
        let runtime = CustomProviderRuntime {
            config: config.clone(),
            api_key,
        };
        let provider = provider_definition(&runtime);
        let models = model_definitions(&config);

        assert_eq!(config.base_url, "http://localhost:20128/v1");
        assert_eq!(provider.adapter, AdapterKind::OpenAiCompatibleChat);
        assert_eq!(
            provider.auth,
            Auth::InlineBearer {
                value: "secret".to_string()
            }
        );
        assert_eq!(
            provider.request.headers.get("x-router").map(String::as_str),
            Some("primary")
        );
        assert_eq!(models[0].adapter_model_id, "zai/glm-5");
        assert!(models[0].capabilities.tools);
    }

    #[test]
    fn rejects_reserved_provider_ids_and_duplicate_models_or_headers() {
        let mut reserved = omniroute_request();
        reserved.provider_id = ProviderId::OPENAI.to_string();
        assert!(
            validate_new_provider(reserved)
                .unwrap_err()
                .contains("built into Fennara")
        );

        let mut duplicate_model = omniroute_request();
        duplicate_model
            .models
            .push(duplicate_model.models[0].clone());
        assert!(
            validate_new_provider(duplicate_model)
                .unwrap_err()
                .contains("duplicated")
        );

        let mut duplicate_header = omniroute_request();
        duplicate_header.headers.push(CustomProviderHeader {
            name: "x-router".to_string(),
            value: "fallback".to_string(),
        });
        assert!(
            validate_new_provider(duplicate_header)
                .unwrap_err()
                .contains("duplicated")
        );
    }

    #[test]
    fn rejects_base_urls_with_queries_or_fragments() {
        let mut query = omniroute_request();
        query.base_url = "https://example.com/v1?token=secret".to_string();
        assert!(
            validate_new_provider(query)
                .unwrap_err()
                .contains("query string or fragment")
        );

        let mut fragment = omniroute_request();
        fragment.base_url = "https://example.com/v1#api".to_string();
        assert!(
            validate_new_provider(fragment)
                .unwrap_err()
                .contains("query string or fragment")
        );
    }

    #[test]
    fn splits_custom_selection_without_losing_slashes_in_the_model_id() {
        let (config, _) = validate_new_provider(omniroute_request()).unwrap();

        assert_eq!(
            split_model_selection(&[config], "omniroute/zai/glm-5"),
            Some(("omniroute", "zai/glm-5"))
        );
    }
}
