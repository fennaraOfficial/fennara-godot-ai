mod adapters;
mod anthropic;
mod anthropic_providers;
mod capability_check;
mod catalog;
pub(crate) mod catalog_cache;
mod context;
pub(crate) mod custom;
mod deepseek;
mod error;
mod lmstudio;
pub(crate) mod models_dev;
mod moonshot;
mod nvidia;
mod ollama;
mod ollama_cloud;
mod openai;
mod openrouter;
mod request;
mod resolver;
mod sse;
mod stream;
mod types;
pub(crate) mod usage;
mod zai;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use serde::Serialize;
use serde_json::{Value, json};

use super::settings::ChatSettings;
use super::{auth, trace::TraceRecorder};
use request::LlmRequest;
use stream::StreamEvent;
use tokio::sync::Mutex;

pub(crate) use error::LlmError;
pub(crate) use request::build_messages;
pub(crate) use stream::FinishReason;
#[allow(unused_imports)]
pub(crate) use types::{
    ChatCompletion, ChatRequest, MalformedToolCall, ProviderId, ProviderSettings, StreamItem,
    ToolCallObservation,
};

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PublicProvider {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) kind: &'static str,
    pub(crate) auth: PublicProviderAuth,
    pub(crate) connected: bool,
    pub(crate) model_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) setup: Option<PublicProviderSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) custom: Option<PublicCustomProvider>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PublicProviderAuth {
    #[serde(rename = "type")]
    pub(crate) kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) env: Option<&'static str>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PublicProviderSetup {
    #[serde(rename = "type")]
    pub(crate) kind: &'static str,
    pub(crate) default_base_url: &'static str,
    pub(crate) base_url: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PublicCustomProvider {
    pub(crate) base_url: String,
    pub(crate) models: Vec<custom::CustomProviderModel>,
    pub(crate) header_count: usize,
}

pub(crate) fn settings_from_chat(settings: &ChatSettings) -> ProviderSettings {
    ProviderSettings {
        openai_api_key: auth::api_key(types::ProviderId::OPENAI),
        anthropic_api_key: auth::api_key(types::ProviderId::ANTHROPIC),
        openrouter_api_key: auth::api_key(types::ProviderId::OPENROUTER),
        ollama_cloud_api_key: auth::api_key(types::ProviderId::OLLAMA_CLOUD),
        lmstudio_api_key: auth::api_key(types::ProviderId::LMSTUDIO),
        deepseek_api_key: auth::api_key(types::ProviderId::DEEPSEEK),
        zai_api_key: auth::api_key(types::ProviderId::ZAI),
        moonshot_api_key: auth::api_key(types::ProviderId::MOONSHOTAI),
        moonshot_cn_api_key: auth::api_key(types::ProviderId::MOONSHOTAI_CN),
        kimi_api_key: auth::api_key(types::ProviderId::KIMI_FOR_CODING),
        minimax_api_key: auth::api_key(types::ProviderId::MINIMAX),
        minimax_coding_plan_api_key: auth::api_key(types::ProviderId::MINIMAX_CODING_PLAN)
            .or_else(|| auth::api_key(types::ProviderId::MINIMAX)),
        minimax_cn_api_key: auth::api_key(types::ProviderId::MINIMAX_CN),
        minimax_cn_coding_plan_api_key: auth::api_key(types::ProviderId::MINIMAX_CN_CODING_PLAN)
            .or_else(|| auth::api_key(types::ProviderId::MINIMAX_CN)),
        nvidia_api_key: auth::api_key(types::ProviderId::NVIDIA),
        custom_providers: settings
            .custom_providers
            .iter()
            .cloned()
            .map(|config| custom::CustomProviderRuntime {
                api_key: auth::api_key(&config.id),
                config,
            })
            .collect(),
        ollama_base_url: settings.ollama_base_url.clone(),
        lmstudio_base_url: settings
            .provider_base_url(types::ProviderId::LMSTUDIO, lmstudio::DEFAULT_BASE_URL),
        custom_models: settings.custom_models.clone(),
        local_model_limits: local_model_limits_from_settings(&settings.local_model_context_lengths),
    }
}

pub(crate) fn public_provider_registry(settings: &ChatSettings) -> Vec<PublicProvider> {
    let mut providers = vec![
        api_key_provider(
            openai::provider_definition(None),
            "cloud",
            openai::API_KEY_ENV,
        ),
        api_key_provider(
            anthropic::provider_definition(None),
            "cloud",
            anthropic::API_KEY_ENV,
        ),
        api_key_provider(
            openrouter::provider_definition(None),
            "cloud",
            "OPENROUTER_API_KEY",
        ),
        local_provider(
            ollama::provider_definition(&settings.ollama_base_url),
            "local",
            super::settings::DEFAULT_OLLAMA_BASE_URL,
            settings.ollama_base_url.clone(),
        ),
        local_provider(
            lmstudio::provider_definition(
                &settings
                    .provider_base_url(types::ProviderId::LMSTUDIO, lmstudio::DEFAULT_BASE_URL),
                None,
            ),
            "local",
            lmstudio::DEFAULT_BASE_URL,
            settings.provider_base_url(types::ProviderId::LMSTUDIO, lmstudio::DEFAULT_BASE_URL),
        ),
        api_key_provider(
            ollama_cloud::provider_definition(None),
            "cloud",
            "OLLAMA_API_KEY",
        ),
        api_key_provider(
            deepseek::provider_definition(None),
            "cloud",
            deepseek::API_KEY_ENV,
        ),
        api_key_provider(zai::provider_definition(None), "cloud", zai::API_KEY_ENV),
        api_key_provider(
            moonshot::provider_definition(None),
            "cloud",
            moonshot::API_KEY_ENV,
        ),
        api_key_provider(
            moonshot::cn_provider_definition(None),
            "cloud",
            moonshot::API_KEY_ENV,
        ),
        anthropic_api_key_provider(types::ProviderId::KIMI_FOR_CODING),
        anthropic_api_key_provider(types::ProviderId::MINIMAX),
        anthropic_api_key_provider(types::ProviderId::MINIMAX_CODING_PLAN),
        anthropic_api_key_provider(types::ProviderId::MINIMAX_CN),
        anthropic_api_key_provider(types::ProviderId::MINIMAX_CN_CODING_PLAN),
        api_key_provider(
            nvidia::provider_definition(None),
            "cloud",
            nvidia::API_KEY_ENV,
        ),
    ];
    providers.extend(settings.custom_providers.iter().map(custom_public_provider));
    providers
}

fn custom_public_provider(config: &custom::CustomProviderConfig) -> PublicProvider {
    PublicProvider {
        id: config.id.clone(),
        name: config.name.clone(),
        kind: "custom",
        auth: PublicProviderAuth {
            kind: "optional_api_key",
            env: None,
        },
        connected: true,
        model_prefix: format!("{}/", config.id),
        setup: None,
        custom: Some(PublicCustomProvider {
            base_url: config.base_url.clone(),
            models: config.models.clone(),
            header_count: config.headers.len(),
        }),
    }
}

pub(crate) fn provider_has_api_key(provider_id: &str, env_var: &str) -> bool {
    auth::has_api_key(provider_id)
        || minimax_alias_has_api_key(provider_id)
        || minimax_cn_alias_has_api_key(provider_id)
        || std::env::var(env_var)
            .ok()
            .is_some_and(|key| !key.trim().is_empty())
}

fn minimax_alias_has_api_key(provider_id: &str) -> bool {
    matches!(provider_id, types::ProviderId::MINIMAX_CODING_PLAN)
        && auth::has_api_key(types::ProviderId::MINIMAX)
}

fn minimax_cn_alias_has_api_key(provider_id: &str) -> bool {
    matches!(provider_id, types::ProviderId::MINIMAX_CN_CODING_PLAN)
        && auth::has_api_key(types::ProviderId::MINIMAX_CN)
}

fn api_key_provider(
    provider: types::ProviderDefinition,
    kind: &'static str,
    env_var: &'static str,
) -> PublicProvider {
    let provider_id = provider.id.to_string();
    PublicProvider {
        id: provider_id.clone(),
        name: provider.name,
        kind,
        auth: PublicProviderAuth {
            kind: "api_key",
            env: Some(env_var),
        },
        connected: provider_has_api_key(&provider_id, env_var),
        model_prefix: format!("{provider_id}/"),
        setup: None,
        custom: None,
    }
}

fn anthropic_api_key_provider(provider_id: &'static str) -> PublicProvider {
    let provider = anthropic_providers::provider_definition(provider_id, None)
        .expect("Anthropic-compatible provider id is registered");
    let env_var = anthropic_providers::spec(provider_id)
        .expect("Anthropic-compatible provider id is registered")
        .api_key_env;
    let connected = provider_has_api_key(provider_id, env_var)
        || minimax_alias_has_api_key(provider_id)
        || minimax_cn_alias_has_api_key(provider_id);
    let provider_id = provider.id.to_string();
    PublicProvider {
        id: provider_id.clone(),
        name: provider.name,
        kind: "cloud",
        auth: PublicProviderAuth {
            kind: "api_key",
            env: Some(env_var),
        },
        connected,
        model_prefix: format!("{provider_id}/"),
        setup: None,
        custom: None,
    }
}

fn local_provider(
    provider: types::ProviderDefinition,
    kind: &'static str,
    default_base_url: &'static str,
    base_url: String,
) -> PublicProvider {
    let provider_id = provider.id.to_string();
    PublicProvider {
        id: provider_id.clone(),
        name: provider.name,
        kind,
        auth: PublicProviderAuth {
            kind: "none",
            env: None,
        },
        connected: true,
        model_prefix: format!("{provider_id}/"),
        setup: Some(PublicProviderSetup {
            kind: "base_url",
            default_base_url,
            base_url,
        }),
        custom: None,
    }
}

pub(crate) fn missing_auth_for_model(settings: &ChatSettings, model: &str) -> Option<LlmError> {
    if custom::split_model_selection(&settings.custom_providers, model).is_some() {
        return None;
    }
    let (provider_id, provider_name, env_var) = auth_provider_for_model(model)?;
    (!provider_has_api_key(provider_id, env_var)).then(|| LlmError::Auth {
        provider: provider_id.to_string(),
        message: format!("Add your {provider_name} API key first."),
    })
}

fn auth_provider_for_model(model: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match selected_provider_for_model(model)? {
        types::ProviderId::OPENAI => {
            Some((types::ProviderId::OPENAI, "OpenAI", openai::API_KEY_ENV))
        }
        types::ProviderId::ANTHROPIC => Some((
            types::ProviderId::ANTHROPIC,
            "Anthropic",
            anthropic::API_KEY_ENV,
        )),
        types::ProviderId::OPENROUTER => Some((
            types::ProviderId::OPENROUTER,
            "OpenRouter",
            "OPENROUTER_API_KEY",
        )),
        types::ProviderId::OLLAMA_CLOUD => Some((
            types::ProviderId::OLLAMA_CLOUD,
            "Ollama Cloud",
            "OLLAMA_API_KEY",
        )),
        types::ProviderId::DEEPSEEK => Some((
            types::ProviderId::DEEPSEEK,
            "DeepSeek",
            deepseek::API_KEY_ENV,
        )),
        types::ProviderId::ZAI => Some((types::ProviderId::ZAI, "Z.AI", zai::API_KEY_ENV)),
        types::ProviderId::MOONSHOTAI => Some((
            types::ProviderId::MOONSHOTAI,
            "Moonshot AI",
            moonshot::API_KEY_ENV,
        )),
        types::ProviderId::MOONSHOTAI_CN => Some((
            types::ProviderId::MOONSHOTAI_CN,
            "Moonshot AI (China)",
            moonshot::API_KEY_ENV,
        )),
        types::ProviderId::NVIDIA => {
            Some((types::ProviderId::NVIDIA, "NVIDIA", nvidia::API_KEY_ENV))
        }
        provider if anthropic_providers::is_anthropic_provider(provider) => {
            let spec = anthropic_providers::spec(provider)?;
            Some((spec.id, spec.name, spec.api_key_env))
        }
        _ => None,
    }
}

fn selected_provider_for_model(model: &str) -> Option<&'static str> {
    let clean = model.trim();
    [
        types::ProviderId::OPENAI,
        types::ProviderId::ANTHROPIC,
        types::ProviderId::OPENROUTER,
        types::ProviderId::OLLAMA_CLOUD,
        types::ProviderId::OLLAMA,
        types::ProviderId::LMSTUDIO,
        types::ProviderId::DEEPSEEK,
        types::ProviderId::ZAI,
        types::ProviderId::MOONSHOTAI,
        types::ProviderId::MOONSHOTAI_CN,
        types::ProviderId::KIMI_FOR_CODING,
        types::ProviderId::MINIMAX_CN_CODING_PLAN,
        types::ProviderId::MINIMAX_CODING_PLAN,
        types::ProviderId::MINIMAX_CN,
        types::ProviderId::MINIMAX,
        types::ProviderId::NVIDIA,
    ]
    .into_iter()
    .find(|provider| has_provider_prefix(clean, provider))
}

fn has_provider_prefix(model: &str, provider: &str) -> bool {
    model
        .strip_prefix(provider)
        .is_some_and(|rest| rest.starts_with('/'))
}

pub(crate) async fn stream_chat<F, Fut>(
    settings: &ProviderSettings,
    request: &ChatRequest,
    trace: Option<TraceRecorder>,
    on_item: F,
) -> Result<ChatCompletion, LlmError>
where
    F: FnMut(StreamItem) -> Fut + Send,
    Fut: std::future::Future<Output = Result<bool, LlmError>> + Send,
{
    let llm_request = LlmRequest::from_chat(settings, request)?;
    capability_check::preflight(&llm_request)?;
    let _ = context::preflight(&llm_request)?;
    validate_provider_request(&llm_request).await?;

    let accumulator = Arc::new(Mutex::new(StreamAccumulator::default()));
    let on_item = Arc::new(Mutex::new(on_item));
    let adapter = llm_request.model.provider.adapter.clone();
    let mut completion = match adapter {
        types::AdapterKind::OpenAiCompatibleChat => {
            adapters::openai_compatible::stream_chat(&llm_request, trace, {
                let accumulator = Arc::clone(&accumulator);
                let on_item = Arc::clone(&on_item);
                move |event| {
                    let accumulator = Arc::clone(&accumulator);
                    let on_item = Arc::clone(&on_item);
                    async move {
                        let items = {
                            let mut accumulator = accumulator.lock().await;
                            accumulator.items_for_event(event)?
                        };
                        for item in items {
                            let mut on_item = on_item.lock().await;
                            if !on_item(item).await? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                }
            })
            .await?
        }
        types::AdapterKind::AnthropicCompatibleMessages => {
            adapters::anthropic_compatible::stream_chat(&llm_request, trace, {
                let accumulator = Arc::clone(&accumulator);
                let on_item = Arc::clone(&on_item);
                move |event| {
                    let accumulator = Arc::clone(&accumulator);
                    let on_item = Arc::clone(&on_item);
                    async move {
                        let items = {
                            let mut accumulator = accumulator.lock().await;
                            accumulator.items_for_event(event)?
                        };
                        for item in items {
                            let mut on_item = on_item.lock().await;
                            if !on_item(item).await? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                }
            })
            .await?
        }
    };

    let accumulator = accumulator.lock().await;
    if let Some(reason) = accumulator.finish_reason.clone() {
        completion.finish_reason = reason;
    }
    completion.tool_call_observation.observed = completion
        .tool_call_observation
        .observed
        .max(accumulator.observed_tool_calls);
    Ok(completion)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ChatContextEstimate {
    pub(crate) estimated_input_tokens: u32,
    pub(crate) usable_input_tokens: Option<u32>,
    pub(crate) raw_context_tokens: Option<u32>,
    pub(crate) max_output_tokens: Option<u32>,
}

pub(crate) fn estimate_chat_context(
    settings: &ProviderSettings,
    request: &ChatRequest,
) -> Result<ChatContextEstimate, LlmError> {
    let llm_request = LlmRequest::from_chat(settings, request)?;
    Ok(ChatContextEstimate {
        estimated_input_tokens: context::estimate_request_tokens(&llm_request),
        usable_input_tokens: context::request_usable_input_tokens(&llm_request),
        raw_context_tokens: llm_request.model.model.limits.context_tokens,
        max_output_tokens: llm_request.model.model.limits.output_tokens,
    })
}

pub(crate) fn model_context_estimate(
    settings: &ProviderSettings,
    model: &str,
    reasoning_effort: &str,
) -> Result<ChatContextEstimate, LlmError> {
    estimate_chat_context(
        settings,
        &ChatRequest {
            model: model.to_string(),
            reasoning_effort: reasoning_effort.to_string(),
            messages: vec![json!({ "role": "user", "content": "" })],
            tools: Vec::new(),
            max_output_tokens: None,
        },
    )
}

pub(crate) fn selected_model_supports_image_input(
    settings: &ProviderSettings,
    model: &str,
    reasoning_effort: &str,
) -> bool {
    let request = ChatRequest {
        model: model.to_string(),
        reasoning_effort: reasoning_effort.to_string(),
        messages: vec![json!({ "role": "user", "content": "" })],
        tools: Vec::new(),
        max_output_tokens: None,
    };
    LlmRequest::from_chat(settings, &request)
        .map(|request| {
            request
                .model
                .model
                .capabilities
                .input
                .iter()
                .any(|value| value == "image")
        })
        .unwrap_or(false)
}

#[derive(Default)]
struct StreamAccumulator {
    text: String,
    reasoning: String,
    emit_lens: HashMap<String, usize>,
    finish_reason: Option<FinishReason>,
    observed_tool_calls: usize,
}

impl StreamAccumulator {
    fn items_for_event(&mut self, event: StreamEvent) -> Result<Vec<StreamItem>, LlmError> {
        let mut items = Vec::new();
        match event {
            StreamEvent::TextDelta { text: delta, .. } => {
                self.text.push_str(&delta);
                if self.text.len().saturating_sub(self.emit_len("__text__")) >= 24 {
                    self.emit_lens
                        .insert("__text__".to_string(), self.text.len());
                    items.push(StreamItem::Text {
                        content: self.text.clone(),
                        done: false,
                    });
                }
            }
            StreamEvent::ReasoningDelta {
                text: reasoning_delta,
                ..
            } => {
                if reasoning_delta.starts_with(self.reasoning.as_str()) {
                    self.reasoning = reasoning_delta;
                } else {
                    self.reasoning.push_str(&reasoning_delta);
                }
                if self
                    .reasoning
                    .len()
                    .saturating_sub(self.emit_len("__reasoning__"))
                    >= 48
                {
                    self.emit_lens
                        .insert("__reasoning__".to_string(), self.reasoning.len());
                    items.push(StreamItem::Reasoning {
                        content: self.reasoning.clone(),
                        done: false,
                    });
                }
            }
            StreamEvent::ToolCallDelta {
                id,
                name,
                arguments,
            } => {
                self.observed_tool_calls = self.observed_tool_calls.saturating_add(1);
                let last_len = self.emit_len(&id);
                if arguments.len().saturating_sub(last_len) >= 24 || !name.is_empty() {
                    self.emit_lens.insert(id.clone(), arguments.len());
                    items.push(StreamItem::FunctionCall {
                        id,
                        name,
                        arguments,
                        done: false,
                    });
                }
            }
            StreamEvent::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                self.observed_tool_calls = self.observed_tool_calls.saturating_add(1);
                items.push(StreamItem::FunctionCall {
                    id,
                    name,
                    arguments,
                    done: true,
                });
            }
            StreamEvent::ToolCallMalformed {
                id,
                name,
                arguments,
                message,
                ..
            } => {
                self.observed_tool_calls = self.observed_tool_calls.saturating_add(1);
                items.push(StreamItem::FunctionCallError {
                    id,
                    name,
                    arguments,
                    message,
                });
            }
            StreamEvent::Status { message } => {
                items.push(StreamItem::Status { message });
            }
            StreamEvent::Usage(usage) => {
                if let Some(raw) = usage.raw {
                    items.push(StreamItem::Usage(raw));
                }
            }
            StreamEvent::ProviderError(error) => return Err(error),
            StreamEvent::Finish { reason, usage } => {
                self.finish_reason = Some(reason);
                if !self.reasoning.is_empty() {
                    items.push(StreamItem::Reasoning {
                        content: self.reasoning.clone(),
                        done: true,
                    });
                }
                if !self.text.is_empty() {
                    items.push(StreamItem::Text {
                        content: self.text.clone(),
                        done: true,
                    });
                }
                if let Some(usage) = usage.and_then(|usage| usage.raw) {
                    items.push(StreamItem::Usage(usage));
                }
            }
            StreamEvent::StepStart { .. } => {}
        }
        Ok(items)
    }

    fn emit_len(&self, key: &str) -> usize {
        *self.emit_lens.get(key).unwrap_or(&0)
    }
}

async fn validate_provider_request(request: &LlmRequest) -> Result<(), LlmError> {
    match request.model.provider.id.as_str() {
        types::ProviderId::OPENAI | types::ProviderId::ANTHROPIC => Ok(()),
        types::ProviderId::OPENROUTER => openrouter::validate_request(request).await,
        types::ProviderId::OLLAMA_CLOUD => Ok(()),
        types::ProviderId::LMSTUDIO => lmstudio::validate_request(request).await,
        types::ProviderId::DEEPSEEK => Ok(()),
        types::ProviderId::ZAI => Ok(()),
        types::ProviderId::MOONSHOTAI | types::ProviderId::MOONSHOTAI_CN => Ok(()),
        provider if anthropic_providers::is_anthropic_provider(provider) => Ok(()),
        types::ProviderId::OLLAMA | types::ProviderId::LOCAL => {
            ollama::validate_request(request).await
        }
        _ => Ok(()),
    }
}

pub(crate) fn ollama_model_id(model: &str) -> Option<&str> {
    ollama::model_id(model)
}

pub(crate) fn lmstudio_model_id(model: &str) -> Option<&str> {
    lmstudio::model_id(model)
}

pub(crate) async fn hydrate_selected_local_model_limits(
    settings: &mut ProviderSettings,
    model: &str,
) -> Result<(), String> {
    match selected_local_provider(model) {
        Some(types::ProviderId::OLLAMA) | Some(types::ProviderId::LOCAL) => {
            hydrate_ollama_model_limits(settings).await
        }
        Some(types::ProviderId::LMSTUDIO) => hydrate_lmstudio_model_limits(settings).await,
        _ => Ok(()),
    }
}

pub(crate) async fn fetch_ollama_models(base_url: &str) -> Result<Vec<Value>, String> {
    ollama::fetch_models(base_url).await
}

pub(crate) async fn fetch_lmstudio_models(
    base_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<Value>, String> {
    lmstudio::fetch_models(base_url, api_key).await
}

pub(crate) fn ollama_context_tokens(model: &Value) -> Option<u32> {
    ollama::context_tokens_from_model(model)
}

pub(crate) fn lmstudio_context_tokens(model: &Value, model_id: &str) -> Option<u32> {
    lmstudio::context_tokens_from_model(model, model_id)
}

pub(crate) fn lmstudio_v1_base_url(base_url: &str) -> String {
    lmstudio::v1_base_url(base_url)
}

fn selected_local_provider(model: &str) -> Option<&'static str> {
    let clean = model.trim();
    if clean.strip_prefix("ollama/").is_some() {
        Some(types::ProviderId::OLLAMA)
    } else if clean.strip_prefix("local/").is_some() {
        Some(types::ProviderId::LOCAL)
    } else if clean.strip_prefix("lmstudio/").is_some() {
        Some(types::ProviderId::LMSTUDIO)
    } else {
        None
    }
}

async fn hydrate_ollama_model_limits(settings: &mut ProviderSettings) -> Result<(), String> {
    let models = ollama::fetch_models(&settings.ollama_base_url).await?;
    for model in &models {
        let Some(model_id) = ollama::model_name(model) else {
            continue;
        };
        let Some(context_tokens) = ollama::context_tokens_from_model(model) else {
            continue;
        };
        insert_local_context_limit(
            &mut settings.local_model_limits,
            types::ProviderId::OLLAMA,
            model_id,
            context_tokens,
        );
        insert_local_context_limit(
            &mut settings.local_model_limits,
            types::ProviderId::LOCAL,
            model_id,
            context_tokens,
        );
    }
    Ok(())
}

async fn hydrate_lmstudio_model_limits(settings: &mut ProviderSettings) -> Result<(), String> {
    let models = lmstudio::fetch_models(
        &settings.lmstudio_base_url,
        settings.lmstudio_api_key.as_deref(),
    )
    .await?;
    for model in &models {
        let Some(model_id) = lmstudio::model_key(model) else {
            continue;
        };
        let Some(context_tokens) = lmstudio::context_tokens_from_model(model, model_id) else {
            continue;
        };
        insert_local_context_limit(
            &mut settings.local_model_limits,
            types::ProviderId::LMSTUDIO,
            model_id,
            context_tokens,
        );
    }
    Ok(())
}

fn insert_local_context_limit(
    limits: &mut BTreeMap<String, types::Limits>,
    provider_id: &str,
    model_id: &str,
    context_tokens: u32,
) {
    let mut model_limits = types::Limits::default();
    model_limits.context_tokens = Some(context_tokens);
    insert_limit_alias(limits, provider_id, model_id, &model_limits);
    if let Some(stripped) = model_id.strip_suffix(":latest") {
        insert_limit_alias(limits, provider_id, stripped, &model_limits);
    } else if !model_id.contains(':') {
        insert_limit_alias(
            limits,
            provider_id,
            &format!("{model_id}:latest"),
            &model_limits,
        );
    }
}

fn insert_limit_alias(
    limits: &mut BTreeMap<String, types::Limits>,
    provider_id: &str,
    model_id: &str,
    model_limits: &types::Limits,
) {
    limits.insert(format!("{provider_id}/{model_id}"), model_limits.clone());
}

fn local_model_limits_from_settings(
    context_lengths: &BTreeMap<String, u32>,
) -> BTreeMap<String, types::Limits> {
    let mut limits = BTreeMap::new();
    for (model, context_tokens) in context_lengths {
        let Some((provider_id, model_id)) = model.split_once('/') else {
            continue;
        };
        if provider_id != types::ProviderId::OLLAMA && provider_id != types::ProviderId::LMSTUDIO {
            continue;
        }
        insert_local_context_limit(&mut limits, provider_id, model_id, *context_tokens);
    }
    limits
}

pub(crate) fn pricing_for_model(
    settings: &ProviderSettings,
    model: &str,
    context_tokens: u64,
) -> Option<models_dev::CostRates> {
    let catalog = catalog::Catalog::from_settings(settings);
    let model_ref = catalog::model_ref_from_selection(model, &catalog).ok()?;
    let cached = catalog_cache::load_disk_blocking().ok()?;
    let provider_catalog = match model_ref.provider.as_str() {
        types::ProviderId::OPENROUTER => &cached.catalog,
        types::ProviderId::OPENAI => &cached.openai,
        types::ProviderId::ANTHROPIC => &cached.anthropic,
        types::ProviderId::OLLAMA_CLOUD => &cached.ollama_cloud,
        types::ProviderId::LMSTUDIO => &cached.lmstudio,
        types::ProviderId::DEEPSEEK => &cached.deepseek,
        types::ProviderId::ZAI => &cached.zai,
        types::ProviderId::MOONSHOTAI => &cached.moonshot,
        types::ProviderId::MOONSHOTAI_CN => &cached.moonshot_cn,
        types::ProviderId::KIMI_FOR_CODING => &cached.kimi_for_coding,
        types::ProviderId::MINIMAX => &cached.minimax,
        types::ProviderId::MINIMAX_CODING_PLAN => &cached.minimax_coding_plan,
        types::ProviderId::MINIMAX_CN => &cached.minimax_cn,
        types::ProviderId::MINIMAX_CN_CODING_PLAN => &cached.minimax_cn_coding_plan,
        types::ProviderId::NVIDIA => &cached.nvidia,
        _ => return None,
    };
    provider_catalog
        .model(model_ref.model.as_str())
        .map(|model| model.pricing_for_context(context_tokens))
}

#[allow(dead_code)]
pub(crate) fn parse_model_ref(model: &str) -> Result<String, LlmError> {
    let catalog = catalog::Catalog::from_settings(&ProviderSettings {
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
        custom_providers: Vec::new(),
        ollama_base_url: super::settings::DEFAULT_OLLAMA_BASE_URL.to_string(),
        lmstudio_base_url: lmstudio::DEFAULT_BASE_URL.to_string(),
        custom_models: Vec::new(),
        local_model_limits: BTreeMap::new(),
    });
    catalog::model_ref_from_selection(model, &catalog).map(|model_ref| model_ref.canonical())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn custom_provider_config() -> custom::CustomProviderConfig {
        custom::CustomProviderConfig {
            id: "omniroute".to_string(),
            name: "OmniRoute".to_string(),
            base_url: "http://localhost:20128/v1".to_string(),
            models: vec![custom::CustomProviderModel {
                id: "zai/glm-5".to_string(),
                name: "GLM 5".to_string(),
                context_length: 131_072,
                max_output_tokens: 8_192,
            }],
            headers: BTreeMap::new(),
        }
    }

    #[test]
    fn custom_providers_are_connected_and_allow_optional_auth() {
        let mut settings = ChatSettings::default();
        settings.custom_providers.push(custom_provider_config());

        let provider = public_provider_registry(&settings)
            .into_iter()
            .find(|provider| provider.id == "omniroute")
            .unwrap();

        assert_eq!(provider.name, "OmniRoute");
        assert_eq!(provider.kind, "custom");
        assert_eq!(provider.auth.kind, "optional_api_key");
        assert!(provider.connected);
        let custom = provider.custom.unwrap();
        assert_eq!(custom.base_url, "http://localhost:20128/v1");
        assert_eq!(custom.models[0].id, "zai/glm-5");
        assert_eq!(custom.header_count, 0);
        assert!(missing_auth_for_model(&settings, "omniroute/zai/glm-5").is_none());
    }

    #[test]
    fn openrouter_vendor_namespaces_are_not_provider_prefixes() {
        assert_eq!(selected_provider_for_model("google/gemini-3.5-flash"), None);
        assert_eq!(
            selected_provider_for_model("openai/gpt-5.1"),
            Some(types::ProviderId::OPENAI)
        );
        assert_eq!(
            selected_provider_for_model("anthropic/claude-sonnet-4.5"),
            Some(types::ProviderId::ANTHROPIC)
        );
        assert_eq!(
            selected_provider_for_model("openrouter/google/gemini-3.5-flash"),
            Some(types::ProviderId::OPENROUTER)
        );
        assert_eq!(
            selected_provider_for_model("openrouter/openai/gpt-5.5"),
            Some(types::ProviderId::OPENROUTER)
        );
        assert_eq!(
            selected_provider_for_model("ollama/llama3.2"),
            Some(types::ProviderId::OLLAMA)
        );
        assert_eq!(
            selected_provider_for_model("moonshotai/kimi-k2.7-code"),
            Some(types::ProviderId::MOONSHOTAI)
        );
        assert_eq!(
            selected_provider_for_model("moonshotai-cn/kimi-k2.7-code"),
            Some(types::ProviderId::MOONSHOTAI_CN)
        );
        assert_eq!(
            selected_provider_for_model("openrouter/moonshotai/kimi-k2.7-code"),
            Some(types::ProviderId::OPENROUTER)
        );
        assert_eq!(
            selected_provider_for_model("minimax/MiniMax-M3"),
            Some(types::ProviderId::MINIMAX)
        );
        assert_eq!(
            selected_provider_for_model("minimax-coding-plan/MiniMax-M3"),
            Some(types::ProviderId::MINIMAX_CODING_PLAN)
        );
        assert_eq!(
            selected_provider_for_model("minimax-cn/MiniMax-M3"),
            Some(types::ProviderId::MINIMAX_CN)
        );
        assert_eq!(
            selected_provider_for_model("minimax-cn-coding-plan/MiniMax-M3"),
            Some(types::ProviderId::MINIMAX_CN_CODING_PLAN)
        );
        assert_eq!(
            selected_provider_for_model("kimi-for-coding/k2p7"),
            Some(types::ProviderId::KIMI_FOR_CODING)
        );
        assert_eq!(
            selected_provider_for_model("nvidia/meta/llama-3.3-70b-instruct"),
            Some(types::ProviderId::NVIDIA)
        );
    }

    #[test]
    fn local_context_overrides_seed_provider_limits() {
        let overrides = BTreeMap::from([
            ("ollama/gemma4".to_string(), 8192),
            ("lmstudio/google/gemma-4-26b-a4b".to_string(), 4096),
            ("openrouter/google/gemini-3.5-flash".to_string(), 1234),
        ]);

        let limits = local_model_limits_from_settings(&overrides);

        assert_eq!(
            limits
                .get("ollama/gemma4")
                .and_then(|limits| limits.context_tokens),
            Some(8192)
        );
        assert_eq!(
            limits
                .get("ollama/gemma4:latest")
                .and_then(|limits| limits.context_tokens),
            Some(8192)
        );
        assert_eq!(
            limits
                .get("lmstudio/google/gemma-4-26b-a4b")
                .and_then(|limits| limits.context_tokens),
            Some(4096)
        );
        assert!(!limits.contains_key("openrouter/google/gemini-3.5-flash"));
    }

    #[test]
    fn moonshot_auth_uses_moonshot_key() {
        assert_eq!(
            auth_provider_for_model("openai/gpt-5.1"),
            Some((types::ProviderId::OPENAI, "OpenAI", openai::API_KEY_ENV))
        );
        assert_eq!(
            auth_provider_for_model("anthropic/claude-sonnet-4.5"),
            Some((
                types::ProviderId::ANTHROPIC,
                "Anthropic",
                anthropic::API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("moonshotai/kimi-k2.7-code"),
            Some((
                types::ProviderId::MOONSHOTAI,
                "Moonshot AI",
                moonshot::API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("moonshotai-cn/kimi-k2.7-code"),
            Some((
                types::ProviderId::MOONSHOTAI_CN,
                "Moonshot AI (China)",
                moonshot::API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("kimi-for-coding/k2p7"),
            Some((
                types::ProviderId::KIMI_FOR_CODING,
                "Kimi For Coding",
                anthropic_providers::KIMI_API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("minimax/MiniMax-M3"),
            Some((
                types::ProviderId::MINIMAX,
                "MiniMax (minimax.io)",
                anthropic_providers::MINIMAX_API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("minimax-coding-plan/MiniMax-M3"),
            Some((
                types::ProviderId::MINIMAX_CODING_PLAN,
                "MiniMax Token Plan (minimax.io)",
                anthropic_providers::MINIMAX_API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("minimax-cn/MiniMax-M3"),
            Some((
                types::ProviderId::MINIMAX_CN,
                "MiniMax (minimaxi.com)",
                anthropic_providers::MINIMAX_API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("minimax-cn-coding-plan/MiniMax-M3"),
            Some((
                types::ProviderId::MINIMAX_CN_CODING_PLAN,
                "MiniMax Token Plan (minimaxi.com)",
                anthropic_providers::MINIMAX_API_KEY_ENV
            ))
        );
        assert_eq!(
            auth_provider_for_model("nvidia/meta/llama-3.3-70b-instruct"),
            Some((types::ProviderId::NVIDIA, "NVIDIA", nvidia::API_KEY_ENV))
        );
    }
}
