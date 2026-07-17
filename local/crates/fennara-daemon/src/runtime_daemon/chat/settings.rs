use serde::{Deserialize, Deserializer, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    sync::{
        Mutex, MutexGuard,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::runtime_daemon::permissions::{
    ApprovalMode, approval_mode_options, clean_approval_mode,
};

use super::auth;
use super::providers::{
    self, ProviderId, PublicProvider,
    custom::{self, CustomProviderConfig, SaveCustomProviderRequest},
};

pub(crate) const DEFAULT_MODEL: &str = "openrouter/google/gemini-3.5-flash";
pub(crate) const DEFAULT_OPENROUTER_MODEL_ID: &str = "google/gemini-3.5-flash";
pub(crate) const DEFAULT_REASONING_EFFORT: &str = "medium";
pub(crate) const DEFAULT_OLLAMA_BASE_URL: &str = "http://127.0.0.1:11434";
pub(crate) const DEFAULT_CHAT_SURFACE: &str = "embedded";
pub(crate) const BROWSER_CHAT_SURFACE: &str = "browser";

static SETTINGS_WRITE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
static SETTINGS_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ChatSettings {
    #[serde(default, skip_serializing)]
    pub(crate) openrouter_api_key: Option<String>,
    #[serde(default = "default_ollama_base_url")]
    pub(crate) ollama_base_url: String,
    #[serde(default)]
    pub(crate) provider_base_urls: BTreeMap<String, String>,
    pub(crate) model: String,
    #[serde(default = "default_reasoning_effort")]
    pub(crate) reasoning_effort: String,
    #[serde(default)]
    pub(crate) custom_models: Vec<String>,
    #[serde(default)]
    pub(crate) custom_providers: Vec<CustomProviderConfig>,
    #[serde(default)]
    pub(crate) local_model_context_lengths: BTreeMap<String, u32>,
    #[serde(default = "default_chat_surface")]
    pub(crate) chat_surface: String,
    #[serde(default, deserialize_with = "deserialize_approval_mode")]
    pub(crate) approval_mode: ApprovalMode,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PublicChatSettings {
    pub(crate) has_openrouter_key: bool,
    pub(crate) has_ollama_cloud_key: bool,
    pub(crate) providers: Vec<PublicProvider>,
    pub(crate) ollama_base_url: String,
    pub(crate) provider_base_urls: BTreeMap<String, String>,
    pub(crate) model: String,
    pub(crate) default_model: &'static str,
    pub(crate) reasoning_effort: String,
    pub(crate) reasoning_effort_options: Vec<&'static str>,
    pub(crate) text_model_suggestions: Vec<String>,
    pub(crate) custom_models: Vec<String>,
    pub(crate) local_model_context_lengths: BTreeMap<String, u32>,
    pub(crate) chat_surface: String,
    pub(crate) approval_mode: String,
    pub(crate) approval_mode_options: Vec<serde_json::Value>,
}

impl Default for ChatSettings {
    fn default() -> Self {
        Self {
            openrouter_api_key: None,
            ollama_base_url: DEFAULT_OLLAMA_BASE_URL.to_string(),
            provider_base_urls: default_provider_base_urls(),
            model: DEFAULT_MODEL.to_string(),
            reasoning_effort: DEFAULT_REASONING_EFFORT.to_string(),
            custom_models: Vec::new(),
            custom_providers: Vec::new(),
            local_model_context_lengths: BTreeMap::new(),
            chat_surface: DEFAULT_CHAT_SURFACE.to_string(),
            approval_mode: ApprovalMode::Ask,
        }
    }
}

impl ChatSettings {
    pub(crate) fn public(&self) -> PublicChatSettings {
        let providers = providers::public_provider_registry(self);
        let has_openrouter_key = provider_connected(&providers, ProviderId::OPENROUTER);
        let has_ollama_cloud_key = provider_connected(&providers, ProviderId::OLLAMA_CLOUD);
        PublicChatSettings {
            has_openrouter_key,
            has_ollama_cloud_key,
            providers,
            ollama_base_url: clean_ollama_base_url(&self.ollama_base_url),
            provider_base_urls: clean_provider_base_urls(&self.provider_base_urls),
            model: clean_model(&self.model).unwrap_or_else(|| DEFAULT_MODEL.to_string()),
            default_model: DEFAULT_MODEL,
            reasoning_effort: clean_reasoning_effort(&self.reasoning_effort).to_string(),
            reasoning_effort_options: vec!["low", DEFAULT_REASONING_EFFORT, "high"],
            text_model_suggestions: suggestion_models(
                &self.custom_models,
                &self.custom_providers,
                has_openrouter_key,
            ),
            custom_models: self.custom_models.clone(),
            local_model_context_lengths: self.local_model_context_lengths.clone(),
            chat_surface: clean_chat_surface(&self.chat_surface).to_string(),
            approval_mode: self.approval_mode.as_str().to_string(),
            approval_mode_options: approval_mode_options(),
        }
    }
}

pub(crate) fn recommended_model_ids() -> Vec<&'static str> {
    vec![
        DEFAULT_MODEL,
        "openrouter/qwen/qwen3.7-plus",
        "moonshotai/kimi-k2.7-code",
        "minimax/MiniMax-M3",
        "openai/gpt-5.5",
        "anthropic/claude-opus-4.8",
        "deepseek/deepseek-v4-flash",
        "deepseek/deepseek-v4-pro",
        "openrouter/z-ai/glm-5.2",
    ]
}

fn suggestion_models(
    custom_models: &[String],
    custom_providers: &[CustomProviderConfig],
    has_openrouter_key: bool,
) -> Vec<String> {
    let mut models = if has_openrouter_key {
        recommended_model_ids()
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    for model in custom_models {
        if !has_openrouter_key
            && !model.starts_with("openai/")
            && !model.starts_with("anthropic/")
            && !model.starts_with("ollama/")
            && !model.starts_with("lmstudio/")
            && !model.starts_with("moonshotai/")
            && !model.starts_with("moonshotai-cn/")
            && !model.starts_with("kimi-for-coding/")
            && !model.starts_with("minimax/")
            && !model.starts_with("minimax-coding-plan/")
            && !model.starts_with("minimax-cn/")
            && !model.starts_with("minimax-cn-coding-plan/")
            && !model.starts_with("nvidia/")
            && custom::split_model_selection(custom_providers, model).is_none()
        {
            continue;
        }
        if !models.iter().any(|existing| existing == model) {
            models.push(model.clone());
        }
    }
    models
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SaveSettingsRequest {
    pub(crate) openrouter_api_key: Option<String>,
    pub(crate) ollama_cloud_api_key: Option<String>,
    pub(crate) provider_api_keys: Option<BTreeMap<String, String>>,
    pub(crate) ollama_base_url: Option<String>,
    pub(crate) provider_base_urls: Option<BTreeMap<String, String>>,
    pub(crate) custom_provider: Option<SaveCustomProviderRequest>,
    pub(crate) model: Option<String>,
    pub(crate) reasoning_effort: Option<String>,
    pub(crate) local_model_context_lengths: Option<BTreeMap<String, u32>>,
    pub(crate) chat_surface: Option<String>,
    pub(crate) approval_mode: Option<String>,
}

pub(crate) fn load_settings() -> ChatSettings {
    let _guard = settings_lock();
    load_settings_unlocked().0
}

fn load_settings_unlocked() -> (ChatSettings, bool) {
    let path = settings_path();
    let previous = path.with_extension("json.previous");
    let selected = if path.is_file() {
        &path
    } else if previous.is_file() {
        &previous
    } else {
        return (ChatSettings::default(), true);
    };
    let Ok(raw) = fs::read_to_string(selected) else {
        return (ChatSettings::default(), true);
    };
    let Ok(mut settings) = serde_json::from_str::<ChatSettings>(&raw) else {
        return (ChatSettings::default(), true);
    };
    let legacy_openrouter_key = settings.openrouter_api_key.take();
    let had_legacy_openrouter_key = legacy_openrouter_key.is_some();
    settings.custom_providers = custom::clean_saved_providers(&settings.custom_providers);
    let mut custom_headers_migrated = false;
    let mut custom_headers_migration_failed = false;
    for provider in &mut settings.custom_providers {
        let stored_headers = auth::custom_headers(&provider.id);
        match migrate_custom_provider_headers(provider, stored_headers, auth::save_custom_headers) {
            CustomHeaderMigration::None => {}
            CustomHeaderMigration::ScrubSettings => custom_headers_migrated = true,
            CustomHeaderMigration::Failed => custom_headers_migration_failed = true,
        }
    }
    let clean_model = clean_model(&settings.model).unwrap_or_else(|| DEFAULT_MODEL.to_string());
    settings.model = migrate_legacy_openrouter_selection(&clean_model, &settings.custom_providers);
    let model_migrated = settings.model != clean_model;
    settings.reasoning_effort = clean_reasoning_effort(&settings.reasoning_effort).to_string();
    settings.ollama_base_url = clean_ollama_base_url(&settings.ollama_base_url);
    settings.provider_base_urls = clean_provider_base_urls(&settings.provider_base_urls);
    settings.provider_base_urls.insert(
        ProviderId::OLLAMA.to_string(),
        settings.ollama_base_url.clone(),
    );
    let clean_custom_models = clean_model_list(&settings.custom_models);
    let migrated_custom_models = clean_custom_models
        .iter()
        .map(|model| migrate_legacy_openrouter_selection(model, &settings.custom_providers))
        .collect::<Vec<_>>();
    settings.custom_models = clean_model_list(&migrated_custom_models);
    let custom_models_migrated = settings.custom_models != clean_custom_models;
    settings.local_model_context_lengths =
        clean_local_model_context_lengths(&settings.local_model_context_lengths);
    settings.chat_surface = clean_chat_surface(&settings.chat_surface).to_string();
    settings.approval_mode = clean_approval_mode(settings.approval_mode.as_str());
    if had_legacy_openrouter_key {
        auth::migrate_legacy_api_key(ProviderId::OPENROUTER, legacy_openrouter_key);
    }
    if !custom_headers_migration_failed
        && (had_legacy_openrouter_key
            || model_migrated
            || custom_models_migrated
            || custom_headers_migrated)
    {
        if write_settings_file(&settings).is_ok() && custom_headers_migrated {
            let _ = fs::remove_file(previous);
        }
    }
    (settings, !custom_headers_migration_failed)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CustomHeaderMigration {
    None,
    ScrubSettings,
    Failed,
}

fn migrate_custom_provider_headers<F>(
    provider: &mut CustomProviderConfig,
    stored_headers: BTreeMap<String, String>,
    save_headers: F,
) -> CustomHeaderMigration
where
    F: FnOnce(&str, &BTreeMap<String, String>) -> Result<(), String>,
{
    if provider.headers.is_empty() {
        if stored_headers.is_empty() {
            return CustomHeaderMigration::None;
        }
        provider.headers = stored_headers;
        return CustomHeaderMigration::None;
    }

    let mut merged_headers = provider.headers.clone();
    merged_headers.extend(stored_headers.clone());
    if merged_headers != stored_headers {
        return match save_headers(&provider.id, &merged_headers) {
            Ok(()) => {
                provider.headers = merged_headers;
                CustomHeaderMigration::ScrubSettings
            }
            Err(_) => CustomHeaderMigration::Failed,
        };
    }

    provider.headers = stored_headers;
    CustomHeaderMigration::ScrubSettings
}

pub(crate) fn save_settings(update: SaveSettingsRequest) -> Result<ChatSettings, String> {
    let _guard = settings_lock();
    let (mut settings, custom_headers_ready) = load_settings_unlocked();
    if !custom_headers_ready {
        return Err(
            "Could not move custom provider headers into Fennara's protected auth store."
                .to_string(),
        );
    }
    if let Some(key) = update.openrouter_api_key {
        let trimmed = key.trim();
        if !trimmed.is_empty() {
            auth::save_api_key(ProviderId::OPENROUTER, trimmed)?;
        }
    }
    if let Some(key) = update.ollama_cloud_api_key {
        let trimmed = key.trim();
        if !trimmed.is_empty() {
            auth::save_api_key(ProviderId::OLLAMA_CLOUD, trimmed)?;
        }
    }
    if let Some(provider_api_keys) = update.provider_api_keys {
        save_provider_api_keys(provider_api_keys)?;
    }
    if let Some(base_url) = update.ollama_base_url {
        settings.ollama_base_url = clean_ollama_base_url(&base_url);
        settings.provider_base_urls.insert(
            ProviderId::OLLAMA.to_string(),
            settings.ollama_base_url.clone(),
        );
    }
    if let Some(provider_base_urls) = update.provider_base_urls {
        for (provider, base_url) in provider_base_urls {
            let Some(provider) = super::providers::ProviderId::new(provider) else {
                continue;
            };
            let clean = clean_base_url(&base_url);
            if clean.is_empty() {
                continue;
            }
            if provider.as_str() == ProviderId::OLLAMA {
                settings.ollama_base_url = clean_ollama_base_url(&clean);
            }
            settings
                .provider_base_urls
                .insert(provider.to_string(), clean_base_url(&clean));
        }
    }
    if let Some(custom_provider) = update.custom_provider {
        let update_existing = custom_provider.update_existing;
        if !update_existing && settings.custom_providers.len() >= custom::MAX_CUSTOM_PROVIDERS {
            return Err(format!(
                "Fennara supports at most {} custom providers.",
                custom::MAX_CUSTOM_PROVIDERS
            ));
        }
        let (mut config, api_key) = custom::validate_new_provider(custom_provider)?;
        let existing_index = settings
            .custom_providers
            .iter()
            .position(|provider| provider.id == config.id);
        match (existing_index, update_existing) {
            (Some(_), false) => {
                return Err(format!("Provider ID {} already exists.", config.id));
            }
            (None, true) => {
                return Err(format!("Provider ID {} no longer exists.", config.id));
            }
            (Some(index), true) => {
                let mut headers = settings.custom_providers[index].headers.clone();
                headers.extend(config.headers);
                config.headers = headers;
                settings.custom_providers[index] = config.clone();
                reconcile_custom_provider_models(&mut settings, &config);
            }
            (None, false) => settings.custom_providers.push(config.clone()),
        }
        auth::save_custom_headers(&config.id, &config.headers)?;
        if let Some(api_key) = api_key {
            auth::save_api_key(&config.id, &api_key)?;
        }
    }
    if let Some(model) = update.model {
        settings.model = clean_model(&model).unwrap_or_else(|| DEFAULT_MODEL.to_string());
        remember_custom_model(&mut settings.custom_models, &settings.model);
    }
    if let Some(reasoning_effort) = update.reasoning_effort {
        settings.reasoning_effort = clean_reasoning_effort(&reasoning_effort).to_string();
    }
    if let Some(context_lengths) = update.local_model_context_lengths {
        settings.local_model_context_lengths = clean_local_model_context_lengths(&context_lengths);
    }
    if let Some(chat_surface) = update.chat_surface {
        settings.chat_surface = clean_chat_surface(&chat_surface).to_string();
    }
    if let Some(approval_mode) = update.approval_mode {
        settings.approval_mode = clean_approval_mode(&approval_mode);
    }

    write_settings_file(&settings)?;
    Ok(settings)
}

fn save_provider_api_keys(provider_api_keys: BTreeMap<String, String>) -> Result<(), String> {
    for (provider, key) in provider_api_keys {
        let provider = provider.trim();
        let key = key.trim();
        if provider.is_empty() || key.is_empty() {
            continue;
        }
        auth::save_api_key(provider, key)?;
    }
    Ok(())
}

fn write_settings_file(settings: &ChatSettings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    let raw = serde_json::to_string_pretty(&settings).map_err(|error| error.to_string())?;
    restrict_existing_settings_permissions(&path)?;
    let sequence = SETTINGS_WRITE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = path.with_extension(format!("json.tmp-{}-{sequence}", std::process::id()));
    let result = write_secure_temp_file(&temp, format!("{raw}\n").as_bytes())
        .and_then(|_| replace_settings_file(&temp, &path));
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

fn settings_lock() -> MutexGuard<'static, ()> {
    SETTINGS_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(unix)]
fn restrict_existing_settings_permissions(path: &Path) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = match fs::metadata(path) {
        Ok(metadata) => metadata.permissions(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(format!("failed to inspect {}: {error}", path.display())),
    };
    permissions.set_mode(0o600);
    fs::set_permissions(path, permissions)
        .map_err(|error| format!("failed to protect {}: {error}", path.display()))
}

#[cfg(not(unix))]
fn restrict_existing_settings_permissions(_path: &Path) -> Result<(), String> {
    Ok(())
}

fn write_secure_temp_file(path: &Path, contents: &[u8]) -> Result<(), String> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
    file.write_all(contents)
        .and_then(|_| file.sync_all())
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

#[cfg(not(windows))]
fn replace_settings_file(temp: &Path, path: &Path) -> Result<(), String> {
    fs::rename(temp, path).map_err(|error| {
        format!(
            "failed to replace {} with {}: {error}",
            path.display(),
            temp.display()
        )
    })
}

#[cfg(windows)]
fn replace_settings_file(temp: &Path, path: &Path) -> Result<(), String> {
    let backup = path.with_extension("json.previous");
    let had_current = path.exists();
    if had_current {
        if backup.exists() {
            fs::remove_file(&backup)
                .map_err(|error| format!("failed to remove {}: {error}", backup.display()))?;
        }
        fs::rename(path, &backup).map_err(|error| {
            format!(
                "failed to back up {} as {}: {error}",
                path.display(),
                backup.display()
            )
        })?;
    }
    match fs::rename(temp, path) {
        Ok(()) => {
            let _ = fs::remove_file(backup);
            Ok(())
        }
        Err(error) => {
            if had_current && backup.exists() && !path.exists() {
                let _ = fs::rename(&backup, path);
            }
            Err(format!(
                "failed to replace {} with {}: {error}",
                path.display(),
                temp.display()
            ))
        }
    }
}

fn remember_custom_model(custom_models: &mut Vec<String>, model: &str) {
    if recommended_model_ids()
        .into_iter()
        .any(|recommended| recommended == model)
    {
        return;
    }
    if model == DEFAULT_MODEL || model == "openrouter/auto" {
        return;
    }
    if !model.contains('/') {
        return;
    }
    if !custom_models.iter().any(|existing| existing == model) {
        custom_models.push(model.to_string());
    }
    *custom_models = clean_model_list(custom_models);
}

fn reconcile_custom_provider_models(settings: &mut ChatSettings, provider: &CustomProviderConfig) {
    let prefix = format!("{}/", provider.id);
    let is_configured = |selection: &str| {
        selection
            .strip_prefix(&prefix)
            .is_some_and(|model_id| provider.models.iter().any(|model| model.id == model_id))
    };

    settings
        .custom_models
        .retain(|selection| !selection.starts_with(&prefix) || is_configured(selection));

    if settings.model.starts_with(&prefix) && !is_configured(&settings.model) {
        let replacement = format!("{prefix}{}", provider.models[0].id);
        settings.model = replacement.clone();
        remember_custom_model(&mut settings.custom_models, &replacement);
    }
}

fn clean_model_list(models: &[String]) -> Vec<String> {
    let mut clean = Vec::new();
    for model in models {
        let Some(model) = clean_model(model) else {
            continue;
        };
        if !model.contains('/') {
            continue;
        }
        if !clean.iter().any(|existing| existing == &model) {
            clean.push(model);
        }
    }
    clean
}

// Legacy compatibility only. New selections must always include their provider prefix.
fn migrate_legacy_openrouter_selection(
    model: &str,
    custom_providers: &[CustomProviderConfig],
) -> String {
    let clean = model.trim();
    let explicit_provider = clean.split_once('/').is_some_and(|(provider_id, _)| {
        custom::is_reserved_provider_id(provider_id)
            || custom_providers
                .iter()
                .any(|provider| provider.id == provider_id)
    });
    if explicit_provider {
        clean.to_string()
    } else {
        format!("openrouter/{clean}")
    }
}

fn clean_local_model_context_lengths(
    context_lengths: &BTreeMap<String, u32>,
) -> BTreeMap<String, u32> {
    let mut clean = BTreeMap::new();
    for (model, context_length) in context_lengths {
        if *context_length == 0 {
            continue;
        }
        let Some(model) = clean_model(model) else {
            continue;
        };
        if model.starts_with("ollama/") || model.starts_with("lmstudio/") {
            clean.insert(model, *context_length);
        }
    }
    clean
}

pub(crate) fn clean_model(model: &str) -> Option<String> {
    let trimmed = model.trim();
    if trimmed.is_empty() {
        return None;
    }
    let clean = strip_nitro_variant(trimmed);
    if clean.starts_with("ollama/")
        || clean.starts_with("openai/")
        || clean.starts_with("anthropic/")
        || clean.starts_with("ollama-cloud/")
        || clean.starts_with("lmstudio/")
        || clean.starts_with("deepseek/")
        || clean.starts_with("zai/")
        || clean.starts_with("moonshotai/")
        || clean.starts_with("moonshotai-cn/")
        || clean.starts_with("kimi-for-coding/")
        || clean.starts_with("minimax/")
        || clean.starts_with("minimax-coding-plan/")
        || clean.starts_with("minimax-cn/")
        || clean.starts_with("minimax-cn-coding-plan/")
    {
        return Some(clean.to_string());
    }
    if clean == "openrouter/auto" || clean.starts_with('~') || clean.ends_with("-latest") {
        return Some(DEFAULT_MODEL.to_string());
    }
    Some(clean.to_string())
}

pub(crate) fn custom_model_trace_parts(
    custom_providers: &[CustomProviderConfig],
    model: &str,
) -> Option<(String, String)> {
    custom::split_model_selection(custom_providers, model)
        .map(|(provider_id, model_id)| (provider_id.to_string(), model_id.to_string()))
}

fn strip_nitro_variant(model: &str) -> &str {
    let Some(prefix) = model.get(..model.len().saturating_sub(":nitro".len())) else {
        return model;
    };
    if model[prefix.len()..].eq_ignore_ascii_case(":nitro") {
        prefix
    } else {
        model
    }
}

pub(crate) fn clean_reasoning_effort(effort: &str) -> &'static str {
    match effort.trim().to_ascii_lowercase().as_str() {
        "low" => "low",
        "medium" => DEFAULT_REASONING_EFFORT,
        "high" => "high",
        _ => DEFAULT_REASONING_EFFORT,
    }
}

pub(crate) fn clean_chat_surface(surface: &str) -> &'static str {
    match surface.trim().to_ascii_lowercase().as_str() {
        BROWSER_CHAT_SURFACE => BROWSER_CHAT_SURFACE,
        _ => DEFAULT_CHAT_SURFACE,
    }
}

pub(crate) fn clean_ollama_base_url(base_url: &str) -> String {
    let clean = clean_base_url(base_url);
    if clean.is_empty() {
        DEFAULT_OLLAMA_BASE_URL.to_string()
    } else {
        clean
    }
}

impl ChatSettings {
    pub(crate) fn provider_base_url(&self, provider_id: &str, default_base_url: &str) -> String {
        self.provider_base_urls
            .get(provider_id)
            .map(String::as_str)
            .map(clean_base_url)
            .filter(|url| !url.is_empty())
            .unwrap_or_else(|| default_base_url.to_string())
    }
}

fn clean_provider_base_urls(base_urls: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut clean = default_provider_base_urls();
    for (provider, base_url) in base_urls {
        let Some(provider) = super::providers::ProviderId::new(provider) else {
            continue;
        };
        let base_url = clean_base_url(base_url);
        if base_url.is_empty() {
            continue;
        }
        clean.insert(provider.to_string(), base_url);
    }
    clean
}

fn clean_base_url(base_url: &str) -> String {
    base_url.trim().trim_end_matches('/').to_string()
}

fn default_provider_base_urls() -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            ProviderId::OLLAMA.to_string(),
            DEFAULT_OLLAMA_BASE_URL.to_string(),
        ),
        (
            ProviderId::LMSTUDIO.to_string(),
            super::providers::lmstudio_v1_base_url(""),
        ),
    ])
}

fn default_reasoning_effort() -> String {
    DEFAULT_REASONING_EFFORT.to_string()
}

fn default_chat_surface() -> String {
    DEFAULT_CHAT_SURFACE.to_string()
}

fn deserialize_approval_mode<'de, D>(deserializer: D) -> Result<ApprovalMode, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value
        .as_deref()
        .map(clean_approval_mode)
        .unwrap_or(ApprovalMode::Ask))
}

fn default_ollama_base_url() -> String {
    DEFAULT_OLLAMA_BASE_URL.to_string()
}

fn provider_connected(providers: &[PublicProvider], provider_id: &str) -> bool {
    providers
        .iter()
        .any(|provider| provider.id == provider_id && provider.connected)
}

fn settings_path() -> PathBuf {
    app_dir().join("chat_settings.json")
}

pub(crate) fn app_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Some(path) = env::var_os("LOCALAPPDATA") {
            return PathBuf::from(path).join("Fennara");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(path) = home_dir() {
            return path
                .join("Library")
                .join("Application Support")
                .join("Fennara");
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(path) = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
        {
            return path.join("fennara");
        }
        if let Some(path) = home_dir() {
            return path.join(".local").join("share").join("fennara");
        }
    }

    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(any(target_os = "macos", all(unix, not(target_os = "macos"))))]
fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

#[cfg(test)]
mod tests;
