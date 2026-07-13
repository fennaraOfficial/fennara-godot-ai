use crate::app_layout::{AppLayout, binary_name};
use crate::release_channel::ChannelPointer;
use crate::release_client::Release;
use crate::release_package::{
    activate_package_at, package_complete, restore_activation_at, shared_runtime_component_key,
    validate_expected_version, validate_legacy_fallback_allowed,
};
use std::fs;
use std::ops::Deref;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn complete_package_requires_launchers_runtimes_and_cached_addon() {
    let layout = test_layout("complete");
    write_complete_package(&layout, "1.2.3");
    assert!(package_complete(&layout, "1.2.3"));
}

#[test]
fn incomplete_runtime_is_not_treated_as_installed() {
    let layout = test_layout("incomplete-runtime");
    write_complete_package(&layout, "1.2.3");
    fs::remove_file(
        layout
            .versions_dir
            .join("1.2.3")
            .join(binary_name("fennara-daemon-runtime")),
    )
    .unwrap();

    assert!(!package_complete(&layout, "1.2.3"));
}

#[test]
fn activation_restore_reinstates_previous_manifest() {
    let layout = test_layout("restore-previous");
    write_complete_package(&layout, "1.2.3");
    fs::write(&layout.current_manifest_path, "{\"version\":\"1.2.2\"}\n").unwrap();
    let previous = fs::read(&layout.current_manifest_path).unwrap();

    let receipt = activate_package_at(&layout, "1.2.3").unwrap();
    assert!(
        fs::read_to_string(&layout.current_manifest_path)
            .unwrap()
            .contains("1.2.3")
    );
    restore_activation_at(&layout, receipt).unwrap();

    assert_eq!(fs::read(&layout.current_manifest_path).unwrap(), previous);
}

#[test]
fn activation_restore_removes_manifest_when_none_existed() {
    let layout = test_layout("restore-none");
    write_complete_package(&layout, "1.2.3");

    let receipt = activate_package_at(&layout, "1.2.3").unwrap();
    assert!(layout.current_manifest_path.is_file());
    restore_activation_at(&layout, receipt).unwrap();

    assert!(!layout.current_manifest_path.exists());
}

#[test]
fn activation_records_staging_identity_for_self_update() {
    let layout = test_layout("staging-identity");
    let version = "1.2.3-pr.101.2";
    write_complete_package(&layout, version);
    fs::write(
        layout
            .versions_dir
            .join(version)
            .join("addon/addons/fennara/release.json"),
        format!(
            r#"{{"schema_version":1,"track":"staging","version":"{version}","release_tag":"v{version}","channel":"pr-101","source_commit":"0123456789abcdef0123456789abcdef01234567"}}"#
        ),
    )
    .unwrap();

    activate_package_at(&layout, version).unwrap();
    let active: serde_json::Value =
        serde_json::from_slice(&fs::read(&layout.current_manifest_path).unwrap()).unwrap();
    assert_eq!(active["release_track"], "staging");
    assert_eq!(active["release_channel"], "pr-101");
    assert_eq!(active["release_tag"], format!("v{version}"));
    assert_eq!(
        active["source_commit"],
        "0123456789abcdef0123456789abcdef01234567"
    );
}

#[test]
fn exact_install_rejects_mismatched_manifest_before_asset_installation() {
    let error = validate_expected_version("v1.2.3", "1.2.4", Some("1.2.3")).unwrap_err();
    assert!(error.contains("release v1.2.3 declares version 1.2.4"));
    assert!(error.contains("addon requires 1.2.3"));
}

#[test]
fn shared_runtime_component_uses_manifest_identifier() {
    let runtime = serde_json::json!({
        "id": "linux-cef",
        "kind": "cef",
        "version": "139.0.0"
    });
    assert_eq!(
        shared_runtime_component_key(&runtime).as_deref(),
        Some("shared_runtime_linux_cef")
    );
}

#[test]
fn channel_release_cannot_use_legacy_installation_without_a_manifest() {
    let release = Release {
        tag: "v0.3.9-pr.101.2".into(),
        assets: serde_json::Value::Array(Vec::new()),
        channel_pointer: Some(ChannelPointer {
            schema_version: 1,
            channel: "pr-101".into(),
            version: "0.3.9-pr.101.2".into(),
            release_tag: "v0.3.9-pr.101.2".into(),
            source_commit: "0123456789abcdef0123456789abcdef01234567".into(),
            release_manifest_sha256: "a".repeat(64),
        }),
    };

    let error = validate_legacy_fallback_allowed(&release).unwrap_err();
    assert!(error.contains("has no release manifest"));
    assert!(error.contains("refusing unverified legacy installation"));
}

#[test]
fn exact_prerelease_cannot_use_legacy_installation_without_a_manifest() {
    let release = Release {
        tag: "v0.3.9-pr.101.2".into(),
        assets: serde_json::Value::Array(Vec::new()),
        channel_pointer: None,
    };

    let error = validate_legacy_fallback_allowed(&release).unwrap_err();
    assert!(error.contains("has no release manifest"));
    assert!(error.contains("refusing unverified legacy installation"));
}

fn write_complete_package(layout: &AppLayout, version: &str) {
    let version_dir = layout.versions_dir.join(version);
    let addon = version_dir.join("addon/addons/fennara");
    fs::create_dir_all(&layout.bin_dir).unwrap();
    fs::create_dir_all(&addon).unwrap();
    for launcher in ["fennara-mcp", "fennara-daemon"] {
        fs::write(layout.bin_dir.join(binary_name(launcher)), "launcher").unwrap();
    }
    for runtime in ["fennara-mcp-runtime", "fennara-daemon-runtime"] {
        fs::write(version_dir.join(binary_name(runtime)), "runtime").unwrap();
    }
    fs::write(addon.join("fennara.gdextension"), "manifest").unwrap();
    fs::write(addon.join("VERSION"), version).unwrap();
}

fn test_layout(name: &str) -> TempLayout {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_dir = std::env::temp_dir().join(format!(
        "fennara-package-test-{name}-{}-{nonce}",
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
