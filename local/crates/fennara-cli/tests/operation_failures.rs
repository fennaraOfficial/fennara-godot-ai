use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn failed_install_has_stable_code_and_sanitized_diagnostics() {
    let root = test_root("invalid-project");
    let project = root.join("private-project");
    fs::create_dir_all(&project).unwrap();

    let output = run(&root, ["install", "--project", path_arg(&project)]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[FEN-INSTALL-PROJECT-INVALID]"));
    assert!(stderr.contains("operation:"));
    assert!(stderr.contains("operation log:"));

    let diagnostics = run(&root, ["diagnostics", "--json"]);
    assert!(diagnostics.status.success());
    let report = String::from_utf8_lossy(&diagnostics.stdout);
    assert!(report.contains("FEN-INSTALL-PROJECT-INVALID"));
    assert!(report.contains("<project>"));
    assert!(!report.contains(&project.display().to_string()));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn operation_log_initialization_failure_stops_before_installation() {
    let root = test_root("blocked-log");
    let project = root.join("project");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("project.godot"), "[application]\n").unwrap();

    let app_dir = app_dir(&root);
    fs::create_dir_all(app_dir.parent().unwrap()).unwrap();
    fs::write(&app_dir, "blocks app-data directory creation").unwrap();

    let output = run(&root, ["install", "--project", path_arg(&project)]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[FEN-OPERATION-LOG-INIT]"));
    assert!(!project.join("addons").exists());
    fs::remove_dir_all(root).unwrap();
}

fn run<const N: usize>(root: &Path, args: [&str; N]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_fennara"));
    command.args(args).current_dir(root);
    if cfg!(target_os = "windows") {
        command.env("LOCALAPPDATA", root);
    } else {
        command.env("HOME", root);
    }
    command.output().unwrap()
}

fn app_dir(root: &Path) -> PathBuf {
    if cfg!(target_os = "windows") {
        root.join("Fennara")
    } else if cfg!(target_os = "macos") {
        root.join("Library")
            .join("Application Support")
            .join("Fennara")
    } else {
        root.join(".local").join("share").join("fennara")
    }
}

fn test_root(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "fennara-cli-test-{name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn path_arg(path: &Path) -> &str {
    path.to_str().unwrap()
}
