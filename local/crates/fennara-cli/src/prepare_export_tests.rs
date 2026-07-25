use crate::prepare_export::remove_runtime_autoload_for_test;

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
