use crate::prepare_export::{
    parse_options_for_test, remove_runtime_autoload_for_test, write_atomic_for_test,
    write_atomic_rollback_for_test,
};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(name: &str) -> Self {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!(
            "fennara-prepare-export-{name}-{}-{suffix}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory should be created");
        Self(path)
    }

    fn join(&self, path: &str) -> PathBuf {
        self.0.join(path)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn removes_only_the_fennara_runtime_autoload() {
    let source = "\
[application]\n\
config/name=\"Game\"\n\
\n\
[autoload]\n\
Other=\"*res://other.gd\"\n\
_fennara_game_capture=\"*res://addons/fennara/runtime/game_capture_helper.gd\"\n\
\n\
[rendering]\n\
renderer/rendering_method=\"gl_compatibility\"\n";
    let expected = "\
[application]\n\
config/name=\"Game\"\n\
\n\
[autoload]\n\
Other=\"*res://other.gd\"\n\
\n\
[rendering]\n\
renderer/rendering_method=\"gl_compatibility\"\n";

    let (prepared, removed) = remove_runtime_autoload_for_test(source);

    assert!(removed);
    assert_eq!(prepared, expected);
}

#[test]
fn preserves_crlf_and_similarly_named_settings() {
    let source = "\
[autoload]\r\n\
_fennara_game_capture_extra=\"res://keep.gd\"\r\n\
_fennara_game_capture = \"*res://addons/fennara/runtime/game_capture_helper.gd\"\r\n\
[application]\r\n\
_fennara_game_capture=\"not-an-autoload\"\r\n";
    let expected = "\
[autoload]\r\n\
_fennara_game_capture_extra=\"res://keep.gd\"\r\n\
[application]\r\n\
_fennara_game_capture=\"not-an-autoload\"\r\n";

    let (prepared, removed) = remove_runtime_autoload_for_test(source);

    assert!(removed);
    assert_eq!(prepared, expected);
}

#[test]
fn leaves_an_already_prepared_project_unchanged() {
    let source = "[application]\nconfig/name=\"Game\"\n";

    let (prepared, removed) = remove_runtime_autoload_for_test(source);

    assert!(!removed);
    assert_eq!(prepared, source);
}

#[test]
fn atomic_write_replaces_the_original_file() {
    let directory = TestDirectory::new("atomic-success");
    let project_file = directory.join("project.godot");
    fs::write(&project_file, b"original").expect("original should be written");

    write_atomic_for_test(&project_file, b"prepared").expect("replacement should succeed");

    assert_eq!(
        fs::read(&project_file).expect("project should remain readable"),
        b"prepared"
    );
    assert_eq!(
        fs::read_dir(&directory.0)
            .expect("directory should remain readable")
            .count(),
        1,
        "staging and backup files should be removed"
    );
}

#[test]
fn atomic_write_restores_the_original_when_replacement_fails() {
    let directory = TestDirectory::new("atomic-rollback");
    let project_file = directory.join("project.godot");
    fs::write(&project_file, b"original").expect("original should be written");

    let error = write_atomic_rollback_for_test(&project_file, b"prepared")
        .expect_err("injected replacement failure should be returned");

    assert!(error.contains("failed to replace"));
    assert_eq!(
        fs::read(&project_file).expect("original should be restored"),
        b"original"
    );
    assert_eq!(
        fs::read_dir(&directory.0)
            .expect("directory should remain readable")
            .count(),
        1,
        "staging and backup files should be removed"
    );
}

#[test]
fn options_reject_project_without_a_path() {
    assert_eq!(
        parse_options_for_test(vec!["--project"]).unwrap_err(),
        "--project requires a path"
    );
}

#[test]
fn options_reject_unknown_flags() {
    assert_eq!(
        parse_options_for_test(vec!["--unknown"]).unwrap_err(),
        "unknown prepare-export option: --unknown"
    );
}

#[test]
fn options_accept_a_project_path() {
    assert_eq!(
        parse_options_for_test(vec!["--project", "game"]).unwrap(),
        Some(PathBuf::from("game"))
    );
}
