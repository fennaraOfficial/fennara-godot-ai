use std::collections::BTreeMap;

use super::{ChatSettings, migrate_legacy_openrouter_selection, reconcile_custom_provider_models};
use crate::runtime_daemon::chat::providers::custom::{CustomProviderConfig, CustomProviderModel};

#[test]
fn provider_edit_replaces_a_removed_selected_model() {
    let mut settings = ChatSettings {
        model: "omniroute/removed/model".to_string(),
        custom_models: vec![
            "omniroute/removed/model".to_string(),
            "openai/gpt-5.5".to_string(),
        ],
        ..ChatSettings::default()
    };
    let provider = CustomProviderConfig {
        id: "omniroute".to_string(),
        name: "OmniRoute".to_string(),
        base_url: "http://localhost:20128/v1".to_string(),
        models: vec![CustomProviderModel {
            id: "replacement/model".to_string(),
            name: "Replacement".to_string(),
            context_length: 64_000,
            max_output_tokens: 4_096,
        }],
        headers: BTreeMap::new(),
    };

    reconcile_custom_provider_models(&mut settings, &provider);

    assert_eq!(settings.model, "omniroute/replacement/model");
    assert_eq!(
        settings.custom_models,
        vec!["openai/gpt-5.5", "omniroute/replacement/model"]
    );
}

#[test]
fn legacy_openrouter_models_migrate_to_an_explicit_provider_prefix() {
    assert_eq!(
        migrate_legacy_openrouter_selection("google/gemini-3.5-flash", &[]),
        "openrouter/google/gemini-3.5-flash"
    );
    assert_eq!(
        migrate_legacy_openrouter_selection("openrouter/google/gemini-3.5-flash", &[]),
        "openrouter/google/gemini-3.5-flash"
    );
}

#[test]
fn legacy_migration_preserves_a_custom_provider_namespace() {
    let provider = CustomProviderConfig {
        id: "google".to_string(),
        name: "Custom Google".to_string(),
        base_url: "https://example.com/v1".to_string(),
        models: vec![CustomProviderModel {
            id: "gemini-3.5-flash".to_string(),
            name: "Custom Gemini".to_string(),
            context_length: 64_000,
            max_output_tokens: 4_096,
        }],
        headers: BTreeMap::new(),
    };

    assert_eq!(
        migrate_legacy_openrouter_selection("google/gemini-3.5-flash", &[provider]),
        "google/gemini-3.5-flash"
    );
}
