use super::{
    current_linux_platform_arch, ensure_linux_cef_manifest, runtime_complete, runtime_repairable,
};
use crate::app_layout::AppLayout;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn repairs_release_marker_missing_native_runtime_identity() {
    let root = TestDir::new("repair-runtime-marker");
    let version = "139.0.28+chromium-139.0.7258.139";
    let layout = test_layout(&root);
    let platform_arch = current_linux_platform_arch();
    let runtime_dir = layout.linux_cef_runtime_dir(platform_arch, version);
    fs::create_dir_all(&runtime_dir).unwrap();
    fs::write(runtime_dir.join("libcef.so"), "cef").unwrap();

    let manifest = release_manifest(version, platform_arch);
    fs::write(
        runtime_dir.join("fennara-cef-runtime.json"),
        format!("{}\n", serde_json::to_string_pretty(&manifest).unwrap()),
    )
    .unwrap();

    assert!(runtime_repairable(&runtime_dir, &manifest));
    assert!(!runtime_complete(&runtime_dir, &manifest));

    let message = ensure_linux_cef_manifest(&layout, &manifest, None)
        .unwrap()
        .unwrap();

    let marker: Value =
        serde_json::from_slice(&fs::read(runtime_dir.join("fennara-cef-runtime.json")).unwrap())
            .unwrap();
    assert_eq!(marker["kind"], "cef");
    assert_eq!(marker["runtime"], "cef");
    assert_eq!(marker["platform"], "linux");
    assert!(runtime_complete(&runtime_dir, &manifest));
    assert!(message.contains("marker repaired"));

    let current: Value = serde_json::from_slice(
        &fs::read(runtime_dir.parent().unwrap().join("current.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(current["runtime"], "cef");
    assert_eq!(current["dir"], version);
}

#[test]
fn leaves_complete_runtime_marker_unchanged() {
    let root = TestDir::new("complete-runtime-marker");
    let version = "139.0.28+chromium-139.0.7258.139";
    let layout = test_layout(&root);
    let platform_arch = current_linux_platform_arch();
    let runtime_dir = layout.linux_cef_runtime_dir(platform_arch, version);
    fs::create_dir_all(&runtime_dir).unwrap();
    fs::write(runtime_dir.join("libcef.so"), "cef").unwrap();

    let manifest = release_manifest(version, platform_arch);
    let mut complete_marker = manifest.clone();
    complete_marker["runtime"] = json!("cef");
    complete_marker["platform"] = json!("linux");
    let marker_bytes = format!(
        "{}\n",
        serde_json::to_string_pretty(&complete_marker).unwrap()
    );
    let marker_path = runtime_dir.join("fennara-cef-runtime.json");
    fs::write(&marker_path, &marker_bytes).unwrap();

    assert!(runtime_complete(&runtime_dir, &manifest));

    let message = ensure_linux_cef_manifest(&layout, &manifest, None)
        .unwrap()
        .unwrap();

    assert!(message.contains("is installed"));
    assert_eq!(fs::read_to_string(marker_path).unwrap(), marker_bytes);
}

fn release_manifest(version: &str, platform_arch: &str) -> Value {
    json!({
        "id": "linux-cef",
        "kind": "cef",
        "schema_version": 1,
        "platform": "linux",
        "arch": "x86_64",
        "platform_arch": platform_arch,
        "version": version,
        "enabled": true,
        "required_files": ["libcef.so"],
        "archive": {
            "format": "zip",
            "name": "fennara-webview-cef-linux-x64.zip",
            "sha256": "abc123"
        }
    })
}

fn test_layout(root: &Path) -> AppLayout {
    let app_dir = root.join("app");
    AppLayout {
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
    }
}

struct TestDir(PathBuf);

impl TestDir {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../temp")
            .join(format!(
                "webview-runtime-test-{name}-{}-{nonce}",
                std::process::id()
            ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }
}

impl std::ops::Deref for TestDir {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
