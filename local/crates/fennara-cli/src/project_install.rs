use crate::app_layout::display_path;
use crate::daemon_setup;
use crate::existing_addon_install;
use crate::operation::{self, FailureClass, Phase};
use crate::project_addon;
use crate::project_guidance;
use crate::release_identity::{ReleaseIdentity, ReleaseTrack};
use crate::release_package;
use crate::webview_prereq;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: Vec<&str>) -> Result<(), String> {
    operation::phase(Phase::Checking, "Validating project installation request")?;
    let options = InstallOptions::parse(args)?;
    let project_dir = resolve_project_dir(options.project_dir)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    ensure_godot_project(&project_dir)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    println!("Installing Fennara");
    println!("project: {}", display_path(&project_dir));

    let addon_dir = project_addon_dir(&project_dir);
    if let Some(existing) = project_addon::inspect(&project_dir)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?
    {
        if options.source_dir.is_some() {
            return Err(operation::failure(
                FailureClass::ProjectInvalid,
                "--source cannot be used when adopting an existing project addon",
            ));
        }
        validate_requested_channel(
            &project_addon_dir(&project_dir),
            &existing.version,
            options.channel.as_deref(),
        )?;
        let layout = crate::app_layout::AppLayout::detect()?;
        prepare_version_switch(&layout, &project_dir, &existing.version)?;
        return existing_addon_install::run(&project_dir, existing, options.version.as_deref());
    }
    if addon_dir.exists() {
        println!(
            "addon: replacing incomplete installation at {}",
            display_path(&addon_dir)
        );
    }
    let (version, source) = match options.source_dir {
        Some(path) => {
            if options.channel.is_some() {
                return Err(operation::failure(
                    FailureClass::ProjectInvalid,
                    "--channel cannot be combined with a local --source addon",
                ));
            }
            println!("package: using local addon source {}", display_path(&path));
            ("local".to_string(), path)
        }
        None => {
            let requested_version = options.version.clone().unwrap_or_else(|| {
                options
                    .channel
                    .as_ref()
                    .map(|channel| format!("channel:{channel}"))
                    .unwrap_or_else(|| "latest".to_string())
            });
            println!("requested version: {requested_version}");
            let resolved = release_package::resolve_package(&requested_version)?;
            validate_resolved_channel(
                resolved.identity(),
                resolved.version(),
                options.channel.as_deref(),
            )?;
            let layout = crate::app_layout::AppLayout::detect()?;
            prepare_version_switch(&layout, &project_dir, resolved.version())?;
            let package = release_package::ensure_resolved_package(resolved)?;
            (package.version, package.addon_dir)
        }
    };
    operation::phase(Phase::Staging, "Installing the project addon")?;
    println!("addon: copying from {}", display_path(&source));
    install_addon(&project_dir, &source)?;
    println!("guidance: writing AGENTS.md and addons/fennara/ai/guidelines.md");
    project_guidance::write(&project_dir)?;
    operation::phase(Phase::Validating, "Checking platform prerequisites")?;
    println!("Installed Fennara");
    println!("version: {version}");
    println!("project: {}", display_path(&project_dir));
    println!("guidance: wrote AGENTS.md and addons/fennara/ai/guidelines.md");
    webview_prereq::warn_for_current_platform()?;
    println!("next: run `fennara update` inside this project when a new release is available");
    Ok(())
}

fn prepare_version_switch(
    layout: &crate::app_layout::AppLayout,
    project_dir: &Path,
    target_version: &str,
) -> Result<(), String> {
    if active_version(layout).as_deref() == Some(target_version) {
        return Ok(());
    }
    daemon_setup::ensure_switch_available(layout, Some(project_dir))
        .and_then(|()| daemon_setup::shutdown_if_running(layout))
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))
}

fn active_version(layout: &crate::app_layout::AppLayout) -> Option<String> {
    crate::app_layout::read_current_manifest(&layout.current_manifest_path)
        .ok()
        .flatten()
        .and_then(|manifest| {
            manifest
                .get("version")
                .and_then(serde_json::Value::as_str)
                .map(str::to_string)
        })
}

pub fn resolve_project_dir(project_dir: Option<PathBuf>) -> Result<PathBuf, String> {
    match project_dir {
        Some(path) => Ok(path),
        None => env::current_dir().map_err(|err| {
            format!("failed to read the current directory; pass --project instead: {err}")
        }),
    }
}

pub fn is_godot_project(project_dir: &Path) -> bool {
    project_dir.join("project.godot").is_file()
}

pub fn ensure_godot_project(project_dir: &Path) -> Result<(), String> {
    if is_godot_project(project_dir) {
        Ok(())
    } else {
        Err(format!(
            "{} is not a Godot project. Run this inside a folder with project.godot or pass --project <path>.",
            display_path(project_dir)
        ))
    }
}

pub fn has_fennara_addon(project_dir: &Path) -> bool {
    project_addon_dir(project_dir)
        .join("fennara.gdextension")
        .is_file()
}

pub fn install_addon(project_dir: &Path, source: &Path) -> Result<(), String> {
    ensure_godot_project(project_dir)?;
    ensure_addon_source(source)?;

    let target = project_addon_dir(project_dir);
    ensure_target_within_project(project_dir, &target)?;
    if target.exists() {
        fs::remove_dir_all(&target).map_err(|err| {
            format!(
                "failed to remove existing addon at {}: {err}",
                display_path(&target)
            )
        })?;
    }
    copy_dir(source, &target)
}

pub fn project_addon_dir(project_dir: &Path) -> PathBuf {
    project_dir.join("addons").join("fennara")
}

pub(crate) fn ensure_target_within_project(
    project_dir: &Path,
    target: &Path,
) -> Result<(), String> {
    let project_root = fs::canonicalize(project_dir).map_err(|err| {
        format!(
            "failed to resolve project path {}: {err}",
            display_path(project_dir)
        )
    })?;
    let mut existing = target;
    while fs::symlink_metadata(existing).is_err() {
        existing = existing.parent().ok_or_else(|| {
            format!(
                "failed to find an existing parent for addon path {}",
                display_path(target)
            )
        })?;
    }
    let resolved = fs::canonicalize(existing).map_err(|err| {
        format!(
            "failed to resolve addon path {}: {err}",
            display_path(existing)
        )
    })?;
    if resolved.starts_with(&project_root) {
        Ok(())
    } else {
        Err(format!(
            "refusing to install Fennara because {} resolves outside the selected project {}",
            display_path(target),
            display_path(&project_root)
        ))
    }
}

struct InstallOptions {
    project_dir: Option<PathBuf>,
    source_dir: Option<PathBuf>,
    version: Option<String>,
    channel: Option<String>,
}

impl InstallOptions {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut project_dir = None;
        let mut source_dir = None;
        let mut version = None;
        let mut channel = None;
        let mut index = 0;

        while index < args.len() {
            match args[index] {
                "--project" => {
                    index += 1;
                    project_dir = Some(PathBuf::from(value_arg(&args, index, "--project")?));
                }
                arg if arg.starts_with("--project=") => {
                    project_dir = Some(PathBuf::from(arg.trim_start_matches("--project=")));
                }
                "--source" => {
                    index += 1;
                    source_dir = Some(PathBuf::from(value_arg(&args, index, "--source")?));
                }
                arg if arg.starts_with("--source=") => {
                    source_dir = Some(PathBuf::from(arg.trim_start_matches("--source=")));
                }
                "--version" => {
                    index += 1;
                    version = Some(value_arg(&args, index, "--version")?.to_string());
                }
                arg if arg.starts_with("--version=") => {
                    version = Some(arg.trim_start_matches("--version=").to_string());
                }
                "--channel" => {
                    index += 1;
                    channel = Some(value_arg(&args, index, "--channel")?.to_string());
                }
                arg if arg.starts_with("--channel=") => {
                    channel = Some(arg.trim_start_matches("--channel=").to_string());
                }
                "--operation-id" => {
                    index += 1;
                    value_arg(&args, index, "--operation-id")?;
                }
                arg if arg.starts_with("--operation-id=") => {
                    if arg.trim_start_matches("--operation-id=").is_empty() {
                        return Err("--operation-id requires a value".to_string());
                    }
                }
                "-h" | "--help" => {
                    print_help();
                    return Err("".to_string());
                }
                other => return Err(format!("unknown install option: {other}")),
            }
            index += 1;
        }

        Ok(Self {
            project_dir,
            source_dir,
            version,
            channel,
        })
    }
}

fn validate_requested_channel(
    addon_dir: &Path,
    version: &str,
    requested_channel: Option<&str>,
) -> Result<(), String> {
    let identity = ReleaseIdentity::load(addon_dir, version)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    validate_resolved_channel(Some(&identity), version, requested_channel)
}

fn validate_resolved_channel(
    identity: Option<&ReleaseIdentity>,
    version: &str,
    requested_channel: Option<&str>,
) -> Result<(), String> {
    validate_channel_selection(identity, requested_channel)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    operation::set_component(
        "release_track",
        match identity.map(|value| &value.track) {
            Some(ReleaseTrack::Staging) => "staging",
            Some(ReleaseTrack::Stable) | None => "stable",
        },
    )?;
    operation::set_requested_version(version)?;
    operation::set_component("activation_reason", "project_install")?;
    if let Some(channel) = identity.and_then(|value| value.channel.as_deref()) {
        operation::set_component("release_channel", channel)?;
    }
    if let Some(source_commit) = identity.and_then(|value| value.source_commit.as_deref()) {
        operation::set_component("source_commit", source_commit)?;
    }
    Ok(())
}

pub(crate) fn validate_channel_selection(
    identity: Option<&ReleaseIdentity>,
    requested_channel: Option<&str>,
) -> Result<(), String> {
    if matches!(
        identity.map(|value| &value.track),
        Some(ReleaseTrack::Staging)
    ) && identity
        .and_then(|value| value.channel.as_deref())
        .is_none()
    {
        return Err("staging addon release identity is missing its channel".into());
    }
    let Some(channel) = requested_channel else {
        return Ok(());
    };
    let actual = identity
        .and_then(|value| value.channel.as_deref())
        .ok_or_else(|| {
            format!("--channel requested {channel}, but the addon is on the stable track")
        })?;
    if !matches!(
        identity.map(|value| &value.track),
        Some(ReleaseTrack::Staging)
    ) || actual != channel
    {
        return Err(format!(
            "--channel requested {channel}, but the addon belongs to {actual}"
        ));
    }
    Ok(())
}

fn value_arg<'a>(args: &'a [&str], index: usize, option: &str) -> Result<&'a str, String> {
    args.get(index)
        .copied()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn print_help() {
    println!(
        "\
Install Fennara into a Godot project.

If a complete addon already exists, install keeps it and sets up its exact matching local components.

Usage:
  fennara install
  fennara install --project <path>
  fennara install --version 0.2.8 --project <path>
  fennara install --version 0.3.9-pr.101.2 --channel pr-101 --project <path>
"
    );
}

fn ensure_addon_source(source: &Path) -> Result<(), String> {
    if source.join("fennara.gdextension").is_file() {
        Ok(())
    } else {
        Err(format!(
            "{} is not a Fennara addon folder; expected fennara.gdextension inside it",
            display_path(source)
        ))
    }
}

fn copy_dir(source: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target)
        .map_err(|err| format!("failed to create {}: {err}", display_path(target)))?;

    for entry in fs::read_dir(source)
        .map_err(|err| format!("failed to read {}: {err}", display_path(source)))?
    {
        let entry = entry
            .map_err(|err| format!("failed to read an entry in {}: {err}", display_path(source)))?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &target_path)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|err| format!("failed to create {}: {err}", display_path(parent)))?;
            }
            fs::copy(&source_path, &target_path).map_err(|err| {
                format!(
                    "failed to copy {} to {}: {err}",
                    display_path(&source_path),
                    display_path(&target_path)
                )
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock")
            .as_nanos();
        env::temp_dir().join(format!(
            "fennara-project-install-{name}-{}-{nonce}",
            std::process::id()
        ))
    }

    #[test]
    fn incomplete_addon_is_replaced_by_install() {
        let root = test_dir("incomplete-addon");
        let project = root.join("project");
        let source = root.join("source");
        let addon = project_addon_dir(&project);

        fs::create_dir_all(&addon).expect("create incomplete addon");
        fs::write(project.join("project.godot"), "[application]\n").expect("write project file");
        fs::write(addon.join("partial.txt"), "stale").expect("write partial addon file");
        fs::create_dir_all(&source).expect("create addon source");
        fs::write(source.join("fennara.gdextension"), "[configuration]\n")
            .expect("write addon manifest");

        assert!(!has_fennara_addon(&project));
        install_addon(&project, &source).expect("replace incomplete addon");
        assert!(has_fennara_addon(&project));
        assert!(!addon.join("partial.txt").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn addon_parent_symlink_cannot_escape_project() {
        use std::os::unix::fs::symlink;

        let root = test_dir("symlink-escape");
        let project = root.join("project");
        let outside_addons = root.join("outside-addons");
        let outside_addon = outside_addons.join("fennara");
        let source = root.join("source");

        fs::create_dir_all(&project).expect("create project");
        fs::write(project.join("project.godot"), "[application]\n").expect("write project file");
        fs::create_dir_all(&outside_addon).expect("create outside addon");
        fs::write(outside_addon.join("keep.txt"), "keep").expect("write outside marker");
        symlink(&outside_addons, project.join("addons")).expect("link addons outside project");
        fs::create_dir_all(&source).expect("create addon source");
        fs::write(source.join("fennara.gdextension"), "[configuration]\n")
            .expect("write addon manifest");

        let error = install_addon(&project, &source).expect_err("reject escaped addon path");
        assert!(error.contains("outside the selected project"));
        assert!(outside_addon.join("keep.txt").is_file());

        let _ = fs::remove_dir_all(root);
    }
}
