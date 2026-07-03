use super::error::LlmError;
use super::request::LlmRequest;
use super::types::Limits;
use serde_json::Value;

const TOKEN_CHAR_APPROX: usize = 4;
const DEFAULT_RESERVED_BUFFER: u32 = 2_000;
const MESSAGE_STRUCTURAL_TOKEN_ESTIMATE: usize = 8;
const TOOL_STRUCTURAL_TOKEN_ESTIMATE: usize = 16;
const IMAGE_BASE_TOKEN_ESTIMATE: usize = 1_024;
const IMAGE_TOKEN_PER_KIB_ESTIMATE: usize = 2;
const IMAGE_MAX_TOKEN_ESTIMATE: usize = 8_192;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ContextDecision {
    Send {
        estimated_input_tokens: u32,
        usable_input_tokens: Option<u32>,
    },
    TooLarge {
        estimated_input_tokens: u32,
        usable_input_tokens: u32,
    },
}

pub(crate) fn preflight(request: &LlmRequest) -> Result<ContextDecision, LlmError> {
    let estimated = estimate_request_tokens(request);
    let usable = usable_input_tokens(
        &request.model.model.limits,
        request.model.request.generation.max_output_tokens,
    );
    let Some(usable) = usable else {
        return Ok(ContextDecision::Send {
            estimated_input_tokens: estimated,
            usable_input_tokens: None,
        });
    };
    if estimated > usable {
        return Err(LlmError::ContextOverflow {
            provider: request.model.provider.id.to_string(),
            message: format!(
                "This chat is estimated at {estimated} input tokens, which exceeds the selected model's usable input budget of {usable} tokens."
            ),
        });
    }
    Ok(ContextDecision::Send {
        estimated_input_tokens: estimated,
        usable_input_tokens: Some(usable),
    })
}

pub(crate) fn estimate_request_tokens(request: &LlmRequest) -> u32 {
    let message_tokens = request
        .messages
        .iter()
        .map(estimate_message_tokens)
        .sum::<usize>();
    let tool_chars = serde_json::to_string(&request.tools)
        .map(|value| value.len())
        .unwrap_or_default();
    let tool_tokens = tool_chars / TOKEN_CHAR_APPROX
        + request
            .tools
            .len()
            .saturating_mul(TOOL_STRUCTURAL_TOKEN_ESTIMATE);
    (message_tokens + tool_tokens).max(1).min(u32::MAX as usize) as u32
}

pub(crate) fn request_usable_input_tokens(request: &LlmRequest) -> Option<u32> {
    usable_input_tokens(
        &request.model.model.limits,
        request.model.request.generation.max_output_tokens,
    )
}

fn estimate_message_tokens(message: &Value) -> usize {
    let Some(object) = message.as_object() else {
        return estimate_value_text_tokens(message);
    };
    let role_tokens = object
        .get("role")
        .and_then(Value::as_str)
        .map(text_tokens)
        .unwrap_or_default();
    let content_tokens = object
        .get("content")
        .map(estimate_content_tokens)
        .unwrap_or_default();
    let tool_calls_tokens = object
        .get("tool_calls")
        .map(estimate_value_text_tokens)
        .unwrap_or_default();
    let tool_call_id_tokens = object
        .get("tool_call_id")
        .and_then(Value::as_str)
        .map(text_tokens)
        .unwrap_or_default();
    let name_tokens = object
        .get("name")
        .and_then(Value::as_str)
        .map(text_tokens)
        .unwrap_or_default();

    MESSAGE_STRUCTURAL_TOKEN_ESTIMATE
        + role_tokens
        + content_tokens
        + tool_calls_tokens
        + tool_call_id_tokens
        + name_tokens
}

fn estimate_content_tokens(content: &Value) -> usize {
    match content {
        Value::String(text) => text_tokens(text),
        Value::Array(parts) => parts.iter().map(estimate_content_part_tokens).sum(),
        _ => estimate_value_text_tokens(content),
    }
}

fn estimate_content_part_tokens(part: &Value) -> usize {
    let Some(object) = part.as_object() else {
        return estimate_value_text_tokens(part);
    };
    match object.get("type").and_then(Value::as_str) {
        Some("text") => {
            object
                .get("text")
                .and_then(Value::as_str)
                .map(text_tokens)
                .unwrap_or_default()
                + MESSAGE_STRUCTURAL_TOKEN_ESTIMATE
        }
        Some("image_url") => object
            .get("image_url")
            .and_then(|image_url| image_url.get("url"))
            .and_then(Value::as_str)
            .map(estimate_image_url_tokens)
            .unwrap_or(IMAGE_BASE_TOKEN_ESTIMATE),
        Some("image") => object
            .get("source")
            .map(estimate_anthropic_image_source_tokens)
            .unwrap_or(IMAGE_BASE_TOKEN_ESTIMATE),
        _ => estimate_value_text_tokens(part),
    }
}

fn estimate_anthropic_image_source_tokens(source: &Value) -> usize {
    let data = source
        .get("data")
        .and_then(Value::as_str)
        .unwrap_or_default();
    estimate_image_base64_tokens(data)
}

fn estimate_image_url_tokens(url: &str) -> usize {
    let base64 = url
        .split_once(',')
        .map(|(_, data)| data)
        .unwrap_or(url)
        .trim();
    estimate_image_base64_tokens(base64)
}

fn estimate_image_base64_tokens(base64: &str) -> usize {
    let approx_bytes = base64.trim().len().saturating_mul(3) / 4;
    let kib = approx_bytes.div_ceil(1024);
    IMAGE_BASE_TOKEN_ESTIMATE
        .saturating_add(kib.saturating_mul(IMAGE_TOKEN_PER_KIB_ESTIMATE))
        .min(IMAGE_MAX_TOKEN_ESTIMATE)
}

fn estimate_value_text_tokens(value: &Value) -> usize {
    serde_json::to_string(value)
        .map(|value| text_tokens(&value))
        .unwrap_or_default()
}

fn text_tokens(text: &str) -> usize {
    text.len().max(1).div_ceil(TOKEN_CHAR_APPROX)
}

fn usable_input_tokens(limits: &Limits, requested_output: Option<u32>) -> Option<u32> {
    let context = limits.context_tokens?;
    let output = requested_output.unwrap_or(DEFAULT_RESERVED_BUFFER);
    let reserved = DEFAULT_RESERVED_BUFFER.max(output);
    if let Some(input) = limits.input_tokens {
        return Some(input.saturating_sub(reserved));
    }
    Some(context.saturating_sub(reserved))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime_daemon::chat::providers::catalog::Catalog;
    use crate::runtime_daemon::chat::providers::request::LlmRequest;
    use crate::runtime_daemon::chat::providers::types::{
        ChatRequest, ProviderSettings, RequestDefaults,
    };
    use serde_json::json;

    #[test]
    fn known_limits_can_block_request_before_provider_call() {
        let settings = ProviderSettings {
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
            ollama_base_url: "http://127.0.0.1:11434".to_string(),
            lmstudio_base_url: "http://127.0.0.1:1234/v1".to_string(),
            custom_models: Vec::new(),
            local_model_limits: std::collections::BTreeMap::new(),
        };
        let catalog = Catalog::from_settings(&settings);
        let model_ref = super::super::catalog::model_ref_from_selection(
            "openrouter/google/gemini-3.5-flash",
            &catalog,
        )
        .unwrap();
        let mut resolved = catalog.resolve(&model_ref).unwrap();
        resolved.model.limits.context_tokens = Some(100);
        resolved.model.limits.output_tokens = Some(50);
        resolved.request = RequestDefaults::default();
        let request = LlmRequest {
            model: resolved,
            messages: vec![json!({ "role": "user", "content": "x".repeat(2000) })],
            tools: Vec::new(),
        };

        assert!(matches!(
            preflight(&request),
            Err(LlmError::ContextOverflow { .. })
        ));
    }

    #[test]
    fn unknown_limits_allow_provider_to_decide() {
        let settings = ProviderSettings {
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
            ollama_base_url: "http://127.0.0.1:11434".to_string(),
            lmstudio_base_url: "http://127.0.0.1:1234/v1".to_string(),
            custom_models: Vec::new(),
            local_model_limits: std::collections::BTreeMap::new(),
        };
        let request = LlmRequest::from_chat(
            &settings,
            &ChatRequest {
                model: "ollama/llama3.1:8b".to_string(),
                reasoning_effort: "medium".to_string(),
                messages: vec![json!({ "role": "user", "content": "hello" })],
                tools: Vec::new(),
                max_output_tokens: None,
            },
        )
        .unwrap();

        assert!(matches!(
            preflight(&request).unwrap(),
            ContextDecision::Send {
                usable_input_tokens: None,
                ..
            }
        ));
    }

    #[test]
    fn catalog_output_limit_does_not_reserve_the_whole_context() {
        let limits = Limits {
            context_tokens: Some(1_048_576),
            input_tokens: None,
            output_tokens: Some(1_048_576),
        };

        assert_eq!(
            usable_input_tokens(&limits, None),
            Some(1_048_576 - DEFAULT_RESERVED_BUFFER)
        );
    }

    #[test]
    fn historical_image_placeholders_do_not_inflate_context_estimate() {
        let settings = ProviderSettings {
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
            ollama_base_url: "http://127.0.0.1:11434".to_string(),
            lmstudio_base_url: "http://127.0.0.1:1234/v1".to_string(),
            custom_models: Vec::new(),
            local_model_limits: std::collections::BTreeMap::new(),
        };
        let catalog = Catalog::from_settings(&settings);
        let model_ref = super::super::catalog::model_ref_from_selection(
            "openrouter/google/gemini-3.5-flash",
            &catalog,
        )
        .unwrap();
        let mut resolved = catalog.resolve(&model_ref).unwrap();
        resolved.request = RequestDefaults::default();
        let placeholder_request = LlmRequest {
            model: resolved.clone(),
            messages: vec![json!({
                "role": "user",
                "content": [
                    { "type": "text", "text": "old screenshot" },
                    { "type": "text", "text": "[Attached image/png: old.png]" }
                ]
            })],
            tools: Vec::new(),
        };
        let current_image_request = LlmRequest {
            model: resolved,
            messages: vec![json!({
                "role": "user",
                "content": [
                    { "type": "text", "text": "old screenshot" },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/png;base64,{}", "a".repeat(320_000))
                        }
                    }
                ]
            })],
            tools: Vec::new(),
        };

        assert!(estimate_request_tokens(&placeholder_request) < 100);
        assert!(estimate_request_tokens(&current_image_request) < 5_000);
    }

    #[test]
    fn current_image_base64_uses_image_estimate_not_text_estimate() {
        let settings = ProviderSettings {
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
            ollama_base_url: "http://127.0.0.1:11434".to_string(),
            lmstudio_base_url: "http://127.0.0.1:1234/v1".to_string(),
            custom_models: Vec::new(),
            local_model_limits: std::collections::BTreeMap::new(),
        };
        let catalog = Catalog::from_settings(&settings);
        let model_ref = super::super::catalog::model_ref_from_selection(
            "openrouter/google/gemini-3.5-flash",
            &catalog,
        )
        .unwrap();
        let mut resolved = catalog.resolve(&model_ref).unwrap();
        resolved.request = RequestDefaults::default();
        let image_base64_len = 935_248;
        let request = LlmRequest {
            model: resolved,
            messages: vec![json!({
                "role": "user",
                "content": [
                    { "type": "text", "text": "can u understand this" },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/png;base64,{}", "a".repeat(image_base64_len))
                        }
                    }
                ]
            })],
            tools: Vec::new(),
        };

        let estimated = estimate_request_tokens(&request);
        assert!(estimated > 1_000);
        assert!(
            estimated < 10_000,
            "image payload was counted like text: {estimated}"
        );
    }
}
