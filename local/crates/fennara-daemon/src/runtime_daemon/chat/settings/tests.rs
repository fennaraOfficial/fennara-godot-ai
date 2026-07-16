use std::collections::BTreeMap;

#[cfg(windows)]
use super::replace_settings_file;
use super::{
    ChatSettings, CustomHeaderMigration, migrate_custom_provider_headers,
    migrate_legacy_openrouter_selection, reconcile_custom_provider_models,
};
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

#[test]
fn stored_custom_headers_trigger_legacy_settings_scrub() {
    let mut provider = CustomProviderConfig {
        id: "omniroute".to_string(),
        name: "OmniRoute".to_string(),
        base_url: "https://example.com/v1".to_string(),
        models: vec![CustomProviderModel {
            id: "model".to_string(),
            name: "Model".to_string(),
            context_length: 64_000,
            max_output_tokens: 4_096,
        }],
        headers: BTreeMap::from([
            ("x-api-key".to_string(), "inline-secret".to_string()),
            ("x-router".to_string(), "primary".to_string()),
        ]),
    };
    let stored_headers = BTreeMap::from([("x-api-key".to_string(), "stored-secret".to_string())]);
    let mut saved_headers = None;

    let result = migrate_custom_provider_headers(&mut provider, stored_headers, |_, headers| {
        saved_headers = Some(headers.clone());
        Ok(())
    });

    assert_eq!(result, CustomHeaderMigration::ScrubSettings);
    assert_eq!(
        provider.headers.get("x-api-key").map(String::as_str),
        Some("stored-secret")
    );
    assert_eq!(
        provider.headers.get("x-router").map(String::as_str),
        Some("primary")
    );
    assert_eq!(saved_headers.as_ref(), Some(&provider.headers));

    let settings = ChatSettings {
        custom_providers: vec![provider],
        ..ChatSettings::default()
    };
    let serialized = serde_json::to_string(&settings).unwrap();
    assert!(!serialized.contains("inline-secret"));
    assert!(!serialized.contains("stored-secret"));
    assert!(!serialized.contains("x-api-key"));
    assert!(!serialized.contains("x-router"));
}

#[test]
fn failed_custom_header_migration_keeps_the_inline_value_recoverable() {
    let mut provider = CustomProviderConfig {
        id: "omniroute".to_string(),
        name: "OmniRoute".to_string(),
        base_url: "https://example.com/v1".to_string(),
        models: vec![CustomProviderModel {
            id: "model".to_string(),
            name: "Model".to_string(),
            context_length: 64_000,
            max_output_tokens: 4_096,
        }],
        headers: BTreeMap::from([
            ("x-api-key".to_string(), "inline-secret".to_string()),
            ("x-router".to_string(), "primary".to_string()),
        ]),
    };
    let stored_headers = BTreeMap::from([("x-api-key".to_string(), "stored-secret".to_string())]);

    let result = migrate_custom_provider_headers(&mut provider, stored_headers, |_, _| {
        Err("auth store unavailable".to_string())
    });

    assert_eq!(result, CustomHeaderMigration::Failed);
    assert_eq!(
        provider.headers.get("x-api-key").map(String::as_str),
        Some("inline-secret")
    );
    assert_eq!(
        provider.headers.get("x-router").map(String::as_str),
        Some("primary")
    );
}

#[cfg(windows)]
#[test]
fn failed_settings_replace_preserves_an_existing_backup_without_a_current_file() {
    let dir = std::env::temp_dir().join(format!(
        "fennara-settings-backup-test-{}",
        std::process::id()
    ));
    let path = dir.join("chat_settings.json");
    let backup = path.with_extension("json.previous");
    let missing_temp = dir.join("missing.tmp");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(&backup, b"previous settings").unwrap();

    let result = replace_settings_file(&missing_temp, &path);

    assert!(result.is_err());
    assert_eq!(std::fs::read(&backup).unwrap(), b"previous settings");
    let _ = std::fs::remove_dir_all(dir);
}
