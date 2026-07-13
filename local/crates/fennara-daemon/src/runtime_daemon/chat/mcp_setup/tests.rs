use super::{target_flag, try_setup_guard};

#[test]
fn public_targets_map_to_cli_flags() {
    assert_eq!(target_flag("claude"), Some("--claude"));
    assert_eq!(target_flag("gemini"), Some("--gemini"));
    assert_eq!(target_flag("codex"), Some("--codex"));
    assert_eq!(target_flag("vscode"), Some("--vscode"));
}

#[test]
fn internal_or_unknown_targets_are_rejected() {
    assert_eq!(target_flag("claude-code"), None);
    assert_eq!(target_flag("claude-desktop"), None);
    assert_eq!(target_flag("antigravity"), None);
    assert_eq!(target_flag("unknown"), None);
}

#[test]
fn concurrent_setup_is_rejected_instead_of_queued() {
    let guard = try_setup_guard().expect("first setup should acquire the lock");
    assert_eq!(
        try_setup_guard().unwrap_err(),
        "Another Fennara MCP setup is already running. Try again shortly."
    );
    drop(guard);
    assert!(try_setup_guard().is_ok());
}
