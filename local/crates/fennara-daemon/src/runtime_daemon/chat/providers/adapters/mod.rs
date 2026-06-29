pub(crate) mod anthropic_compatible;
pub(crate) mod openai_compatible;

use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};

use super::error::LlmError;
use super::types::Auth;

pub(crate) fn apply_auth_headers(
    headers: &mut HeaderMap,
    auth: &Auth,
    provider: &str,
) -> Result<(), LlmError> {
    match auth {
        Auth::None => Ok(()),
        Auth::Env { var } => {
            let token = required_env_token(var, provider, "bearer token")?;
            headers.insert(AUTHORIZATION, bearer_header(&token, provider)?);
            Ok(())
        }
        Auth::Bearer { secret_name } => Err(LlmError::Auth {
            provider: provider.to_string(),
            message: format!("Bearer secret {secret_name} is not wired into chat settings yet."),
        }),
        Auth::InlineBearer { value } => {
            headers.insert(AUTHORIZATION, bearer_header(value, provider)?);
            Ok(())
        }
        Auth::EnvHeader { name, var } => {
            let token = required_env_token(var, provider, name)?;
            headers.insert(
                header_name(name, provider)?,
                header_value(&token, provider, name)?,
            );
            Ok(())
        }
        Auth::InlineHeader { name, value } => {
            headers.insert(
                header_name(name, provider)?,
                header_value(value, provider, name)?,
            );
            Ok(())
        }
    }
}

fn required_env_token(var: &str, provider: &str, label: &str) -> Result<String, LlmError> {
    std::env::var(var)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| LlmError::Auth {
            provider: provider.to_string(),
            message: format!("Missing {label} for {provider}."),
        })
}

fn bearer_header(token: &str, provider: &str) -> Result<HeaderValue, LlmError> {
    header_value(
        &format!("Bearer {}", token.trim()),
        provider,
        "Authorization",
    )
}

fn header_name(name: &str, provider: &str) -> Result<HeaderName, LlmError> {
    HeaderName::from_bytes(name.as_bytes()).map_err(|error| LlmError::Config {
        message: format!("Invalid auth header name for {provider}: {name}: {error}"),
    })
}

fn header_value(value: &str, provider: &str, name: &str) -> Result<HeaderValue, LlmError> {
    HeaderValue::from_str(value.trim()).map_err(|_| LlmError::Auth {
        provider: provider.to_string(),
        message: format!("{provider} {name} value contains invalid header characters."),
    })
}
