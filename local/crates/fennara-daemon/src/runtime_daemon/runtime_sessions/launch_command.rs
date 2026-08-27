use std::{ffi::OsString, path::Path};

pub(super) fn godot_runtime_arguments(
    working_directory: &Path,
    scene_path: &str,
    user_args: &[String],
) -> Vec<OsString> {
    let mut arguments = vec![
        OsString::from("--windowed"),
        OsString::from("--debug"),
        OsString::from("--ignore-error-breaks"),
        OsString::from("--path"),
        working_directory.as_os_str().to_owned(),
        OsString::from("--scene"),
        OsString::from(scene_path),
    ];
    if !user_args.is_empty() {
        arguments.push(OsString::from("--"));
        arguments.extend(user_args.iter().map(OsString::from));
    }
    arguments
}
