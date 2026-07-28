use super::types::ProviderId;

pub(crate) fn uses_configured_output_limit(provider_id: &ProviderId) -> bool {
    matches!(
        provider_id.as_str(),
        ProviderId::OLLAMA | ProviderId::LMSTUDIO | ProviderId::LOCAL
    )
}

pub(crate) fn effective_max_output_tokens(configured: u32, context_tokens: Option<u32>) -> u32 {
    context_tokens
        .map(|context| configured.min((context / 2).max(1)))
        .unwrap_or(configured)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_contexts_keep_half_the_window_available_for_input() {
        assert_eq!(effective_max_output_tokens(8_192, Some(4_096)), 2_048);
        assert_eq!(effective_max_output_tokens(8_192, Some(8_192)), 4_096);
        assert_eq!(effective_max_output_tokens(8_192, Some(65_536)), 8_192);
    }

    #[test]
    fn configured_limits_only_apply_to_local_generation_servers() {
        assert!(uses_configured_output_limit(&ProviderId::unchecked(
            ProviderId::OLLAMA
        )));
        assert!(uses_configured_output_limit(&ProviderId::unchecked(
            ProviderId::LMSTUDIO
        )));
        assert!(uses_configured_output_limit(&ProviderId::unchecked(
            ProviderId::LOCAL
        )));
        assert!(!uses_configured_output_limit(&ProviderId::unchecked(
            ProviderId::OPENAI
        )));
    }
}
