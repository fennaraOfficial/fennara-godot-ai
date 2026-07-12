use super::*;
use crate::release_package;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn restores_both_launchers_after_partial_activation() {
    let root = TestRoot::new("partial");
    let layout = layout(&root);
    let transaction = root.join("transaction");
    fs::create_dir_all(&layout.bin_dir).unwrap();
    fs::create_dir_all(layout.versions_dir.join("1.1.0/staged-launchers")).unwrap();
    fs::write(layout.bin_dir.join(binary_name("fennara-mcp")), b"old-mcp").unwrap();
    fs::write(
        layout.bin_dir.join(binary_name("fennara-daemon")),
        b"old-daemon",
    )
    .unwrap();
    fs::write(
        layout
            .versions_dir
            .join("1.1.0/staged-launchers")
            .join(binary_name("fennara-mcp")),
        b"new-mcp",
    )
    .unwrap();

    snapshot(&layout, &transaction).unwrap();
    assert!(release_package::activate_staged_launchers_at(&layout, "1.1.0").is_err());
    restore(&layout, &transaction).unwrap();

    assert_eq!(
        fs::read(layout.bin_dir.join(binary_name("fennara-mcp"))).unwrap(),
        b"old-mcp"
    );
    assert_eq!(
        fs::read(layout.bin_dir.join(binary_name("fennara-daemon"))).unwrap(),
        b"old-daemon"
    );
}

#[test]
fn repeated_launcher_restore_is_idempotent() {
    let root = TestRoot::new("repeated");
    let layout = layout(&root);
    let transaction = root.join("transaction");
    fs::create_dir_all(&layout.bin_dir).unwrap();
    fs::write(layout.bin_dir.join(binary_name("fennara-mcp")), b"old-mcp").unwrap();

    snapshot(&layout, &transaction).unwrap();
    fs::write(layout.bin_dir.join(binary_name("fennara-mcp")), b"new-mcp").unwrap();
    fs::write(
        layout.bin_dir.join(binary_name("fennara-daemon")),
        b"new-daemon",
    )
    .unwrap();

    restore(&layout, &transaction).unwrap();
    restore(&layout, &transaction).unwrap();

    assert_eq!(
        fs::read(layout.bin_dir.join(binary_name("fennara-mcp"))).unwrap(),
        b"old-mcp"
    );
    assert!(!layout.bin_dir.join(binary_name("fennara-daemon")).exists());
}

fn layout(root: &Path) -> AppLayout {
    AppLayout {
        app_dir: root.join("app"),
        bin_dir: root.join("app/bin"),
        versions_dir: root.join("app/versions"),
        cache_dir: root.join("app/cache"),
        logs_dir: root.join("app/logs"),
        operation_logs_dir: root.join("app/logs/operations"),
        operations_dir: root.join("app/operations"),
        tools_dir: root.join("app/tools"),
        webview_dir: root.join("app/webview"),
        current_manifest_path: root.join("app/current.json"),
    }
}

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self(std::env::temp_dir().join(format!(
            "fennara-update-launchers-{name}-{}-{nonce}",
            std::process::id()
        )))
    }
}

impl Deref for TestRoot {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
