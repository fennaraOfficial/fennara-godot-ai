use std::fs;
use std::ops::Deref;
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
}

#[test]
fn existing_addon_without_version_is_rejected_without_replacement() {
    let root = test_root("existing-addon-missing-version");
    let project = root.join("project");
    let addon = project.join("addons").join("fennara");
    fs::create_dir_all(&addon).unwrap();
    fs::write(project.join("project.godot"), "[application]\n").unwrap();
    let manifest = b"[configuration]\nentry_symbol = \"fennara_entry\"\n";
    fs::write(addon.join("fennara.gdextension"), manifest).unwrap();

    let output = run(&root, ["install", "--project", path_arg(&project)]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[FEN-INSTALL-PROJECT-INVALID]"));
    assert!(stderr.contains("missing a readable VERSION"));
    assert_eq!(
        fs::read(addon.join("fennara.gdextension")).unwrap(),
        manifest
    );
}

#[test]
fn explicit_version_mismatch_is_rejected_before_download() {
    let root = test_root("existing-addon-version-mismatch");
    let project = root.join("project");
    let addon = project.join("addons").join("fennara");
    let library = addon.join("bin").join("fennara-test-library");
    fs::create_dir_all(library.parent().unwrap()).unwrap();
    fs::write(project.join("project.godot"), "[application]\n").unwrap();
    fs::write(addon.join("VERSION"), "1.2.3\n").unwrap();
    fs::write(&library, "store library").unwrap();
    fs::write(
        addon.join("fennara.gdextension"),
        format!(
            "[libraries]\n{}.editor.{} = \"res://addons/fennara/bin/fennara-test-library\"\n",
            std::env::consts::OS,
            fennara_arch()
        ),
    )
    .unwrap();

    let output = run(
        &root,
        [
            "install",
            "--project",
            path_arg(&project),
            "--version",
            "1.2.4",
        ],
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[FEN-INSTALL-PROJECT-INVALID]"));
    assert!(stderr.contains("existing project addon is version 1.2.3"));
    assert_eq!(fs::read_to_string(library).unwrap(), "store library");
}

#[test]
fn update_without_addon_has_project_invalid_code() {
    let root = test_root("update-missing-addon");
    let project = root.join("project");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("project.godot"), "[application]\n").unwrap();

    let output = run(
        &root,
        [
            "update",
            "--no-self-update",
            "--project",
            path_arg(&project),
        ],
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("[FEN-UPDATE-PROJECT-INVALID]"));
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

fn test_root(name: &str) -> TestRoot {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    TestRoot(std::env::temp_dir().join(format!(
        "fennara-cli-test-{name}-{}-{timestamp}",
        std::process::id()
    )))
}

fn path_arg(path: &Path) -> &str {
    path.to_str().unwrap()
}

fn fennara_arch() -> &'static str {
    if std::env::consts::ARCH == "aarch64" {
        "arm64"
    } else {
        std::env::consts::ARCH
    }
}

struct TestRoot(PathBuf);

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
