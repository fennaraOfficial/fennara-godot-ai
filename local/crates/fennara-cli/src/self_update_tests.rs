use crate::app_layout::AppLayout;
use crate::self_update::active_release_request;
use std::fs;
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn defaults_to_stable_latest_without_active_identity() {
    let layout = test_layout("stable-default");
    assert_eq!(active_release_request(&layout).unwrap(), "latest");
}

#[test]
fn preserves_the_active_staging_channel() {
    let layout = test_layout("staging-channel");
    fs::create_dir_all(&layout.app_dir).unwrap();
    fs::write(
        &layout.current_manifest_path,
        r#"{"version":"0.3.9-pr.101.2","release_track":"staging","release_channel":"pr-101"}"#,
    )
    .unwrap();

    assert_eq!(active_release_request(&layout).unwrap(), "channel:pr-101");
}

#[test]
fn rejects_malformed_staging_channel_state() {
    let layout = test_layout("invalid-channel");
    fs::create_dir_all(&layout.app_dir).unwrap();
    fs::write(
        &layout.current_manifest_path,
        r#"{"version":"0.3.9-pr.101.2","release_track":"staging","release_channel":"../latest"}"#,
    )
    .unwrap();

    assert!(
        active_release_request(&layout)
            .unwrap_err()
            .contains("pr-<number>")
    );
}

fn test_layout(name: &str) -> TempLayout {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_dir = std::env::temp_dir().join(format!(
        "fennara-self-update-test-{name}-{}-{nonce}",
        std::process::id()
    ));
    TempLayout(AppLayout {
        bin_dir: app_dir.join("bin"),
        versions_dir: app_dir.join("versions"),
        cache_dir: app_dir.join("cache"),
        logs_dir: app_dir.join("logs"),
        operation_logs_dir: app_dir.join("logs/operations"),
        operations_dir: app_dir.join("operations"),
        tools_dir: app_dir.join("tools"),
        webview_dir: app_dir.join("webview"),
        current_manifest_path: app_dir.join("current.json"),
        app_dir,
    })
}

struct TempLayout(AppLayout);

impl Deref for TempLayout {
    type Target = AppLayout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TempLayout {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.app_dir);
    }
}
