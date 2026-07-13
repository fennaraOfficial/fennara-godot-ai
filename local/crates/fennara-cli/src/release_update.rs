use crate::app_layout::display_path;
use crate::daemon_setup;
use crate::operation::{self, FailureClass, Phase};
use crate::project_addon;
use crate::project_guidance;
use crate::project_install;
use crate::release_client;
use crate::release_identity::{ReleaseIdentity, ReleaseSelector, ReleaseTrack};
use crate::release_package;
use crate::self_update::{self, StartResult};
use crate::update_stage;
use crate::webview_prereq;
use std::path::PathBuf;
use sysinfo::{Pid, System};

pub fn run(args: Vec<&str>) -> Result<(), String> {
    operation::phase(Phase::Checking, "Validating project update request")?;
    let mut options = UpdateOptions::parse(args)?;
    let project_dir = project_install::resolve_project_dir(options.project_dir.clone())
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    project_install::ensure_godot_project(&project_dir)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    println!("Updating Fennara");
    println!("project: {}", display_path(&project_dir));

    if !project_install::has_fennara_addon(&project_dir) {
        return Err(operation::failure(
            FailureClass::ProjectInvalid,
            format!(
                "This Godot project does not have Fennara installed yet. Run `fennara install` from {} first.",
                display_path(&project_dir)
            ),
        ));
    }
    let existing = project_addon::inspect(&project_dir)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?
        .ok_or_else(|| {
            operation::failure(
                FailureClass::ProjectInvalid,
                "The project does not contain a complete Fennara addon to update.",
            )
        })?;
    let project_version = Some(existing.version.clone());
    let identity = ReleaseIdentity::load(
        &project_install::project_addon_dir(&project_dir),
        &existing.version,
    )
    .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    if options.version.is_empty() {
        options.version = match identity.track {
            ReleaseTrack::Stable => "latest".to_string(),
            ReleaseTrack::Staging => format!(
                "channel:{}",
                identity.channel.as_deref().ok_or_else(|| {
                    operation::failure(
                        FailureClass::ProjectInvalid,
                        "the staging addon is missing its release channel",
                    )
                })?
            ),
        };
    }
    options.version = resolve_exact_target(&options.version)?;
    if options.version != existing.version {
        let layout = crate::app_layout::AppLayout::detect()?;
        daemon_setup::ensure_switch_available(
            &layout,
            options.prepare.then_some(project_dir.as_path()),
        )
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    }
    operation::set_requested_version(&options.version)?;
    println!("requested version: {}", options.version);
    operation::set_component(
        "release_track",
        match identity.track {
            ReleaseTrack::Stable => "stable",
            ReleaseTrack::Staging => "staging",
        },
    )?;
    operation::set_component("activation_reason", "project_update")?;
    if let Some(channel) = identity.channel.as_deref() {
        operation::set_component("release_channel", channel)?;
    }
    if let Some(source_commit) = identity.source_commit.as_deref() {
        operation::set_component("source_commit", source_commit)?;
    }

    if !options.no_self_update {
        println!("self-update: checking installed CLI");
        match self_update::start(&options.version, options.continuation_args())? {
            StartResult::Started => return Ok(()),
            StartResult::AlreadyCurrent => println!("self-update: CLI is current"),
            StartResult::Skipped(reason) => println!("warning: {reason}"),
        }
    } else {
        println!("self-update: skipped by --no-self-update");
    }

    println!("package: resolving update package");
    let package = if options.prepare {
        release_package::prepare_package(&options.version)?
    } else {
        release_package::ensure_package(&options.version)?
    };
    if project_version.as_deref() == Some(package.version.as_str()) {
        if !options.prepare {
            println!("guidance: refreshing AGENTS.md and addons/fennara/ai/guidelines.md");
            project_guidance::write(&project_dir)?;
        }
        println!("Fennara is already up to date.");
        println!("version: {}", package.version);
        println!("project: {}", display_path(&project_dir));
        if !options.prepare {
            println!("guidance: refreshed AGENTS.md and addons/fennara/ai/guidelines.md");
        }
        webview_prereq::warn_for_current_platform()?;
        return Ok(());
    }

    if options.prepare {
        operation::phase(
            Phase::Staging,
            "Copying the verified addon into project update staging",
        )?;
        let operation_id = operation::current_id()
            .ok_or_else(|| "update staging requires an operation ID".to_string())?;
        let staged = update_stage::prepare(
            &project_dir,
            project_version.as_deref().unwrap_or("unknown"),
            &package,
            &operation_id,
            observed_godot_process(options.godot_pid, options.godot_executable.as_deref())?,
        )
        .map_err(|error| operation::failure(FailureClass::StageFilesystem, error))?;
        operation::phase(
            Phase::ReadyToClose,
            "The verified update is staged and the active installation is unchanged",
        )?;
        operation::defer_completion()?;
        println!("Fennara {} is ready to install.", staged.version);
        println!("staging: {}", display_path(&staged.root));
        println!("receipt: {}", display_path(&staged.receipt_path));
        println!("The active addon and runtime have not been changed.");
        return Ok(());
    }

    operation::phase(Phase::Staging, "Installing the updated project addon")?;
    println!("addon: copying from {}", display_path(&package.addon_dir));
    project_install::install_addon(&project_dir, &package.addon_dir)
        .map_err(|error| operation::failure(FailureClass::StageFilesystem, error))?;
    println!("guidance: refreshing AGENTS.md and addons/fennara/ai/guidelines.md");
    project_guidance::write(&project_dir)
        .map_err(|error| operation::failure(FailureClass::StageFilesystem, error))?;
    operation::phase(Phase::Validating, "Checking the updated installation")?;
    println!("Updated Fennara");
    println!(
        "from: {}",
        project_version.unwrap_or_else(|| "unknown".to_string())
    );
    println!("to: {}", package.version);
    println!("project: {}", display_path(&project_dir));
    println!("guidance: refreshed AGENTS.md and addons/fennara/ai/guidelines.md");
    webview_prereq::warn_for_current_platform()?;
    Ok(())
}

fn resolve_exact_target(request: &str) -> Result<String, String> {
    if matches!(
        ReleaseSelector::from_version_request(request)?,
        ReleaseSelector::ExactVersion(_)
    ) {
        return Ok(request.to_string());
    }
    let release = release_client::fetch_release(request)?;
    if let Some(pointer) = release.channel_pointer.as_ref() {
        operation::set_component("release_channel", &pointer.channel)?;
        operation::set_component("source_commit", &pointer.source_commit)?;
        return Ok(pointer.version.clone());
    }
    exact_version_from_stable_release(&release)
}

fn exact_version_from_stable_release(release: &release_client::Release) -> Result<String, String> {
    let manifest = release.manifest_asset().ok_or_else(|| {
        operation::failure(
            FailureClass::ManifestInvalid,
            format!(
                "release {} does not contain a versioned release manifest",
                release.tag
            ),
        )
    })?;
    let exact = manifest
        .name
        .strip_prefix("fennara-release-manifest-v")
        .and_then(|value| value.strip_suffix(".json"))
        .ok_or_else(|| {
            operation::failure(
                FailureClass::ManifestInvalid,
                "stable latest contains an invalid release manifest asset name",
            )
        })?;
    match ReleaseSelector::exact(exact)? {
        ReleaseSelector::ExactVersion(version) => Ok(version),
        _ => unreachable!(),
    }
}

struct UpdateOptions {
    version: String,
    project_dir: Option<PathBuf>,
    no_self_update: bool,
    prepare: bool,
    godot_pid: Option<u32>,
    godot_executable: Option<PathBuf>,
}

impl UpdateOptions {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut version = String::new();
        let mut project_dir = None;
        let mut no_self_update = false;
        let mut prepare = false;
        let mut godot_pid = None;
        let mut godot_executable = None;
        let mut index = 0;

        while index < args.len() {
            match args[index] {
                "--version" => {
                    index += 1;
                    version = value_arg(&args, index, "--version")?.to_string();
                }
                arg if arg.starts_with("--version=") => {
                    version = arg.trim_start_matches("--version=").to_string();
                }
                "--project" => {
                    index += 1;
                    project_dir = Some(PathBuf::from(value_arg(&args, index, "--project")?));
                }
                arg if arg.starts_with("--project=") => {
                    project_dir = Some(PathBuf::from(arg.trim_start_matches("--project=")));
                }
                "--no-self-update" => {
                    no_self_update = true;
                }
                "--prepare" => {
                    prepare = true;
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
                "--godot-pid" => {
                    index += 1;
                    godot_pid = Some(parse_pid(value_arg(&args, index, "--godot-pid")?)?);
                }
                arg if arg.starts_with("--godot-pid=") => {
                    godot_pid = Some(parse_pid(arg.trim_start_matches("--godot-pid="))?);
                }
                "--godot-executable" => {
                    index += 1;
                    godot_executable = Some(PathBuf::from(value_arg(
                        &args,
                        index,
                        "--godot-executable",
                    )?));
                }
                arg if arg.starts_with("--godot-executable=") => {
                    godot_executable =
                        Some(PathBuf::from(arg.trim_start_matches("--godot-executable=")));
                }
                "-h" | "--help" => {
                    print_help();
                    return Err("".to_string());
                }
                other => return Err(format!("unknown update option: {other}")),
            }
            index += 1;
        }

        Ok(Self {
            version,
            project_dir,
            no_self_update,
            prepare,
            godot_pid,
            godot_executable,
        })
    }

    fn continuation_args(&self) -> Vec<String> {
        let mut args = vec![
            "update".to_string(),
            "--no-self-update".to_string(),
            "--version".to_string(),
            self.version.clone(),
        ];
        if let Some(project_dir) = &self.project_dir {
            args.push("--project".to_string());
            args.push(project_dir.display().to_string());
        }
        if self.prepare {
            args.push("--prepare".to_string());
        }
        if let Some(pid) = self.godot_pid {
            args.push("--godot-pid".to_string());
            args.push(pid.to_string());
        }
        if let Some(executable) = &self.godot_executable {
            args.push("--godot-executable".to_string());
            args.push(executable.display().to_string());
        }
        args
    }
}

fn parse_pid(value: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .ok()
        .filter(|pid| *pid > 0)
        .ok_or_else(|| format!("invalid Godot process ID: {value}"))
}

fn observed_godot_process(
    pid: Option<u32>,
    executable: Option<&std::path::Path>,
) -> Result<Option<(u32, u64, &std::path::Path)>, String> {
    match (pid, executable) {
        (None, None) => Ok(None),
        (Some(pid), Some(executable)) => {
            let mut system = System::new();
            system.refresh_processes();
            let process = system.process(Pid::from_u32(pid)).ok_or_else(|| {
                format!("Godot process {pid} exited before update preparation completed")
            })?;
            if let Some(actual) = process.exe()
                && canonical_or_original(actual) != canonical_or_original(executable)
            {
                return Err(format!(
                    "process {pid} does not match the selected Godot executable"
                ));
            }
            Ok(Some((pid, process.start_time(), executable)))
        }
        _ => Err("--godot-pid and --godot-executable must be provided together".to_string()),
    }
}

fn canonical_or_original(path: &std::path::Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn value_arg<'a>(args: &'a [&str], index: usize, option: &str) -> Result<&'a str, String> {
    args.get(index)
        .copied()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn print_help() {
    println!(
        "\
Update an existing Fennara project setup.

Usage:
  fennara update
  fennara update --project <path>
  fennara update --version 0.2.8 --project <path>
  fennara update --no-self-update
  fennara update --prepare --project <path>
"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn continuation_args_resume_without_self_update() {
        let options = UpdateOptions {
            version: "1.2.3".to_string(),
            project_dir: Some(PathBuf::from("demo-project")),
            no_self_update: false,
            prepare: false,
            godot_pid: None,
            godot_executable: None,
        };

        assert_eq!(
            options.continuation_args(),
            vec![
                "update".to_string(),
                "--no-self-update".to_string(),
                "--version".to_string(),
                "1.2.3".to_string(),
                "--project".to_string(),
                PathBuf::from("demo-project").display().to_string(),
            ]
        );
    }

    #[test]
    fn parse_accepts_no_self_update() {
        let options = UpdateOptions::parse(vec!["--no-self-update", "--version", "1.2.3"])
            .expect("parse update options");

        assert!(options.no_self_update);
        assert_eq!(options.version, "1.2.3");
    }

    #[test]
    fn continuation_preserves_prepare_mode() {
        let options = UpdateOptions {
            version: "1.2.3".to_string(),
            project_dir: Some(PathBuf::from("demo-project")),
            no_self_update: false,
            prepare: true,
            godot_pid: None,
            godot_executable: None,
        };

        assert!(
            options
                .continuation_args()
                .contains(&"--prepare".to_string())
        );
    }

    #[test]
    fn parse_accepts_prepare_and_operation_id() {
        let options =
            UpdateOptions::parse(vec!["--prepare", "--operation-id", "update-123-godot-456"])
                .unwrap();

        assert!(options.prepare);
    }

    #[test]
    fn stable_latest_version_comes_from_the_versioned_manifest_asset() {
        let release = release_client::Release {
            tag: "latest".to_string(),
            assets: json!([{
                "name": "fennara-release-manifest-v0.3.9.json",
                "browser_download_url": "https://example.invalid/manifest"
            }]),
            channel_pointer: None,
        };
        assert_eq!(
            exact_version_from_stable_release(&release).unwrap(),
            "0.3.9"
        );
    }
}
