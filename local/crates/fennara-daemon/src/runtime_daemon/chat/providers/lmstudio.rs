use reqwest::{Client, RequestBuilder};
use serde_json::Value;
use std::time::Duration;

use super::error::LlmError;
use super::request::LlmRequest;
use super::types::{
    AdapterKind, Auth, Capabilities, GenerationDefaults, Limits, ModelDefinition, ModelId,
    ProviderDefinition, ProviderId, RequestDefaults,
};

pub(crate) const PROVIDER_ID: &str = ProviderId::LMSTUDIO;
pub(crate) const DEFAULT_BASE_URL: &str = "http://127.0.0.1:1234/v1";
pub(crate) const API_KEY_ENV: &str = "LMSTUDIO_API_KEY";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const LOCAL_MODELS_TIMEOUT: Duration = Duration::from_secs(5);

pub(crate) fn provider_definition(base_url: &str, api_key: Option<&str>) -> ProviderDefinition {
    let mut request = RequestDefaults::default();
    request.generation = GenerationDefaults {
        temperature: Some(0.7),
        max_output_tokens: None,
        reasoning_effort: None,
    };

    ProviderDefinition {
        id: ProviderId::unchecked(PROVIDER_ID),
        name: "LM Studio".to_string(),
        adapter: AdapterKind::OpenAiCompatibleChat,
        base_url: Some(v1_base_url(base_url)),
        auth: auth(api_key),
        request,
        disabled: false,
    }
}

pub(crate) fn model_definition(model_id: &str, display_name: Option<String>) -> ModelDefinition {
    ModelDefinition {
        id: ModelId::new(model_id).expect("LM Studio model id is valid"),
        provider: ProviderId::unchecked(PROVIDER_ID),
        display_name: display_name.unwrap_or_else(|| fallback_display_name(model_id)),
        adapter_model_id: model_id.to_string(),
        capabilities: Capabilities::text_tools(),
        limits: Limits::default(),
        request: RequestDefaults::default(),
        enabled: true,
    }
}

pub(crate) fn model_id(model: &str) -> Option<&str> {
    model
        .trim()
        .strip_prefix("lmstudio/")
        .filter(|id| !id.trim().is_empty())
}

pub(crate) async fn fetch_models(
    base_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<Value>, String> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(LOCAL_MODELS_TIMEOUT)
        .build()
        .map_err(|error| format!("Failed to create LM Studio HTTP client: {error}"))?;
    let api_key = api_key_value(api_key);
    let api_key = api_key.as_deref();
    let mut errors = Vec::new();

    match fetch_model_array(
        &client,
        format!("{}/models", native_v1_base_url(base_url)),
        api_key,
        "models",
        "LM Studio native v1 models",
    )
    .await
    {
        Ok(models) => return Ok(models),
        Err(error) => errors.push(error),
    }

    match fetch_model_array(
        &client,
        format!("{}/models", native_v0_base_url(base_url)),
        api_key,
        "data",
        "LM Studio native v0 models",
    )
    .await
    {
        Ok(models) => return Ok(models),
        Err(error) => errors.push(error),
    }

    match fetch_model_array(
        &client,
        format!("{}/models", v1_base_url(base_url)),
        api_key,
        "data",
        "LM Studio OpenAI-compatible models",
    )
    .await
    {
        Ok(models) => Ok(models),
        Err(error) => {
            errors.push(error);
            Err(format!(
                "Failed to fetch LM Studio models: {}",
                errors.join("; ")
            ))
        }
    }
}

pub(crate) fn model_key(model: &Value) -> Option<&str> {
    model
        .get("key")
        .or_else(|| model.get("id"))
        .or_else(|| model.get("model"))
        .and_then(Value::as_str)
}

pub(crate) fn context_tokens_from_model(model: &Value, requested_model: &str) -> Option<u32> {
    loaded_context_tokens(model, requested_model)
        .or_else(|| model.get("max_context_length").and_then(value_as_u32))
        .or_else(|| model.get("context_length").and_then(value_as_u32))
        .or_else(|| {
            model
                .get("model_info")
                .and_then(|info| info.get("context_length"))
                .and_then(value_as_u32)
        })
}

pub(crate) fn v1_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        DEFAULT_BASE_URL.to_string()
    } else if trimmed.ends_with("/v1") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/v1")
    }
}

pub(crate) fn api_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    let clean = ["/api/v1", "/api/v0", "/v1"]
        .into_iter()
        .find_map(|suffix| trimmed.strip_suffix(suffix))
        .unwrap_or(trimmed);
    if clean.is_empty() {
        DEFAULT_BASE_URL
            .trim_end_matches('/')
            .strip_suffix("/v1")
            .unwrap_or(DEFAULT_BASE_URL)
            .to_string()
    } else {
        clean.to_string()
    }
}

fn native_v1_base_url(base_url: &str) -> String {
    format!("{}/api/v1", api_base_url(base_url))
}

fn native_v0_base_url(base_url: &str) -> String {
    format!("{}/api/v0", api_base_url(base_url))
}

async fn fetch_model_array(
    client: &Client,
    url: String,
    api_key: Option<&str>,
    array_key: &str,
    label: &str,
) -> Result<Vec<Value>, String> {
    let response = with_auth(client.get(&url), api_key)
        .send()
        .await
        .map_err(|error| format!("{label} request failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!("{label} request failed: {}", response.status()));
    }
    let body: Value = response
        .json()
        .await
        .map_err(|error| format!("{label} response was invalid: {error}"))?;
    Ok(body
        .get(array_key)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

fn with_auth(request: RequestBuilder, api_key: Option<&str>) -> RequestBuilder {
    if let Some(api_key) = api_key {
        request.bearer_auth(api_key)
    } else {
        request
    }
}

fn api_key_value(api_key: Option<&str>) -> Option<String> {
    api_key
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .map(ToString::to_string)
        .or_else(|| {
            std::env::var(API_KEY_ENV)
                .ok()
                .map(|key| key.trim().to_string())
                .filter(|key| !key.is_empty())
        })
}

fn loaded_context_tokens(model: &Value, requested_model: &str) -> Option<u32> {
    let instances = model.get("loaded_instances").and_then(Value::as_array)?;
    let mut fallback = None;
    for instance in instances {
        let Some(context_length) = instance
            .get("config")
            .and_then(|config| config.get("context_length"))
            .and_then(value_as_u32)
        else {
            continue;
        };
        if fallback.is_none() {
            fallback = Some(context_length);
        }
        if instance
            .get("id")
            .and_then(Value::as_str)
            .is_some_and(|id| id == requested_model)
        {
            return Some(context_length);
        }
    }
    fallback
}

fn value_as_u32(value: &Value) -> Option<u32> {
    let value = value.as_u64()?;
    u32::try_from(value).ok().filter(|value| *value > 0)
}

pub(crate) async fn validate_request(request: &LlmRequest) -> Result<(), LlmError> {
    let client = Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|error| LlmError::ProviderInit {
            provider: PROVIDER_ID.to_string(),
            message: format!("Failed to create LM Studio HTTP client: {error}"),
        })?;

    let base_url = request
        .model
        .provider
        .base_url
        .as_deref()
        .unwrap_or(DEFAULT_BASE_URL);
    validate_model_available(&client, base_url, &request.model.model.adapter_model_id).await
}

async fn validate_model_available(
    client: &Client,
    base_url: &str,
    model: &str,
) -> Result<(), LlmError> {
    let response = client
        .get(format!("{}/models", v1_base_url(base_url)))
        .send()
        .await
        .map_err(|error| LlmError::from_reqwest(PROVIDER_ID, "Failed to reach LM Studio", error))?;
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(LlmError::from_http_response(PROVIDER_ID, status, &text));
    }
    let body = response
        .json::<Value>()
        .await
        .map_err(|error| LlmError::InvalidProviderOutput {
            provider: PROVIDER_ID.to_string(),
            message: format!("LM Studio models response was invalid: {error}"),
            raw: None,
        })?;
    let models = body
        .get("data")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if models.is_empty() {
        return Err(LlmError::ProviderApi {
            provider: PROVIDER_ID.to_string(),
            status: None,
            message: "LM Studio is running but no models are loaded.".to_string(),
            retryable: false,
        });
    }
    if models.iter().any(|entry| {
        entry
            .get("id")
            .or_else(|| entry.get("model"))
            .and_then(Value::as_str)
            .is_some_and(|id| id == model)
    }) {
        return Ok(());
    }
    Err(LlmError::ModelNotFound {
        provider: PROVIDER_ID.to_string(),
        model: model.to_string(),
    })
}

fn auth(api_key: Option<&str>) -> Auth {
    if let Some(key) = api_key.map(str::trim).filter(|key| !key.is_empty()) {
        return Auth::InlineBearer {
            value: key.to_string(),
        };
    }
    if std::env::var(API_KEY_ENV)
        .ok()
        .is_some_and(|key| !key.trim().is_empty())
    {
        return Auth::Env {
            var: API_KEY_ENV.to_string(),
        };
    }
    Auth::None
}

fn fallback_display_name(id: &str) -> String {
    id.split('/').next_back().unwrap_or(id).replace('-', " ")
}
