use super::{RuntimeSessionStartRequest, launch_command::godot_runtime_arguments};
use serde_json::json;
use std::{ffi::OsString, path::Path};

#[test]
fn omits_user_argument_separator_when_no_user_args_are_supplied() {
    let arguments = godot_runtime_arguments(Path::new("project-root"), "res://Main.tscn", &[]);

    assert_eq!(
        arguments,
        [
            "--windowed",
            "--debug",
            "--ignore-error-breaks",
            "--path",
            "project-root",
            "--scene",
            "res://Main.tscn",
        ]
        .map(OsString::from)
    );
}

#[test]
fn appends_user_args_unchanged_after_godot_separator() {
    let user_args = vec![
        "--no-steam".to_string(),
        "--open".to_string(),
        r"D:\cases\bug.ciallo".to_string(),
        String::new(),
    ];

    let arguments =
        godot_runtime_arguments(Path::new("project-root"), "res://Main.tscn", &user_args);

    assert_eq!(
        &arguments[7..],
        ["--", "--no-steam", "--open", r"D:\cases\bug.ciallo", ""].map(OsString::from)
    );
}

#[test]
fn start_request_defaults_missing_user_args_and_rejects_non_strings() {
    let base = json!({
        "executable": "godot",
        "working_directory": "project-root",
        "scene_path": "res://Main.tscn",
        "artifact_dir": "artifacts"
    });
    let request: RuntimeSessionStartRequest =
        serde_json::from_value(base.clone()).expect("user_args should be optional");
    assert!(request.user_args.is_empty());

    let mut invalid = base;
    invalid["user_args"] = json!(["--valid", 42]);
    assert!(serde_json::from_value::<RuntimeSessionStartRequest>(invalid).is_err());
}
