use crate::app_layout::display_path;
use crate::project_install::{ensure_godot_project, resolve_project_dir};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const RUNTIME_AUTOLOAD_NAME: &str = "_fennara_game_capture";

pub fn run(args: Vec<&str>) -> Result<(), String> {
    let options = PrepareExportOptions::parse(args)?;
    let project_dir = resolve_project_dir(options.project_dir)?;
    ensure_godot_project(&project_dir)?;
    let project_file = project_dir.join("project.godot");
    let source = fs::read_to_string(&project_file)
        .map_err(|error| format!("failed to read {}: {error}", display_path(&project_file)))?;
    let (prepared, removed) = remove_runtime_autoload(&source);

    if removed {
        write_atomic(&project_file, prepared.as_bytes())?;
        println!("Prepared project for export");
        println!("project: {}", display_path(&project_dir));
        println!("removed autoload: {RUNTIME_AUTOLOAD_NAME}");
    } else {
        println!("Project is already prepared for export");
        println!("project: {}", display_path(&project_dir));
    }
    Ok(())
}

fn remove_runtime_autoload(source: &str) -> (String, bool) {
    let mut output = String::with_capacity(source.len());
    let mut in_autoload_section = false;
    let mut removed = false;

    for line in source.split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']);
        let trimmed = content.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_autoload_section = trimmed == "[autoload]";
        }
        if in_autoload_section
            && trimmed
                .split_once('=')
                .is_some_and(|(key, _)| key.trim() == RUNTIME_AUTOLOAD_NAME)
        {
            removed = true;
            continue;
        }
        output.push_str(line);
    }

    (output, removed)
}

fn write_atomic(path: &Path, contents: &[u8]) -> Result<(), String> {
    write_atomic_with_rename(path, contents, &unique_suffix(), |source, destination| {
        fs::rename(source, destination)
    })
}

fn write_atomic_with_rename(
    path: &Path,
    contents: &[u8],
    suffix: &str,
    mut rename: impl FnMut(&Path, &Path) -> std::io::Result<()>,
) -> Result<(), String> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project.godot");
    let staging = path.with_file_name(format!("{file_name}.fennara-{suffix}.tmp"));
    let backup = path.with_file_name(format!("{file_name}.fennara-{suffix}.previous"));
    let mut staging_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&staging)
        .map_err(|error| format!("failed to create {}: {error}", display_path(&staging)))?;
    staging_file
        .write_all(contents)
        .map_err(|error| format!("failed to write {}: {error}", display_path(&staging)))?;
    staging_file
        .sync_all()
        .map_err(|error| format!("failed to flush {}: {error}", display_path(&staging)))?;
    drop(staging_file);

    rename(path, &backup).map_err(|error| {
        let _ = fs::remove_file(&staging);
        format!(
            "failed to stage {} for replacement: {error}",
            display_path(path)
        )
    })?;
    if let Err(error) = rename(&staging, path) {
        let restore_error = rename(&backup, path).err();
        let _ = fs::remove_file(&staging);
        return Err(match restore_error {
            Some(restore) => format!(
                "failed to replace {}: {error}; restoring the original also failed: {restore}",
                display_path(path)
            ),
            None => format!("failed to replace {}: {error}", display_path(path)),
        });
    }
    if let Err(error) = fs::remove_file(&backup) {
        eprintln!(
            "warning: updated {}, but could not remove backup {}: {error}",
            display_path(path),
            display_path(&backup)
        );
    }
    Ok(())
}

fn unique_suffix() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    format!("{}-{nanos}", std::process::id())
}

struct PrepareExportOptions {
    project_dir: Option<PathBuf>,
}

impl PrepareExportOptions {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut project_dir = None;
        let mut index = 0;
        while index < args.len() {
            match args[index] {
                "--project" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "--project requires a path".to_string())?;
                    project_dir = Some(PathBuf::from(value));
                }
                "-h" | "--help" => return Err(print_help()),
                argument => return Err(format!("unknown prepare-export option: {argument}")),
            }
            index += 1;
        }
        Ok(Self { project_dir })
    }
}

fn print_help() -> String {
    println!(
        "\
Prepare a Godot checkout for exporting without the Fennara addon.

Usage:
  fennara prepare-export [--project <path>]

This removes only the _fennara_game_capture entry from project.godot. Run it
before Godot starts when addons/fennara is excluded from a CI checkout."
    );
    String::new()
}

#[cfg(test)]
pub(crate) fn remove_runtime_autoload_for_test(source: &str) -> (String, bool) {
    remove_runtime_autoload(source)
}

#[cfg(test)]
pub(crate) fn write_atomic_for_test(path: &Path, contents: &[u8]) -> Result<(), String> {
    write_atomic(path, contents)
}

#[cfg(test)]
pub(crate) fn write_atomic_rollback_for_test(path: &Path, contents: &[u8]) -> Result<(), String> {
    let mut rename_count = 0;
    write_atomic_with_rename(path, contents, "rollback-test", |source, destination| {
        rename_count += 1;
        if rename_count == 2 {
            Err(std::io::Error::other("injected replacement failure"))
        } else {
            fs::rename(source, destination)
        }
    })
}

#[cfg(test)]
pub(crate) fn parse_options_for_test(args: Vec<&str>) -> Result<Option<PathBuf>, String> {
    PrepareExportOptions::parse(args).map(|options| options.project_dir)
}
