use crate::app_layout::display_path;
use crate::operation::{self, FailureClass, Phase};
use crate::project_addon;
use crate::project_guidance;
use crate::project_install;
use crate::release_package;
use crate::self_update::{self, StartResult};
use crate::update_stage;
use crate::webview_prereq;
use std::path::PathBuf;

pub fn run(args: Vec<&str>) -> Result<(), String> {
    operation::phase(Phase::Checking, "Validating project update request")?;
    let options = UpdateOptions::parse(args)?;
    let project_dir = project_install::resolve_project_dir(options.project_dir.clone())
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    project_install::ensure_godot_project(&project_dir)
        .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?;
    println!("Updating Fennara");
    println!("project: {}", display_path(&project_dir));
    println!("requested version: {}", options.version);

    if !project_install::has_fennara_addon(&project_dir) {
        return Err(operation::failure(
            FailureClass::ProjectInvalid,
            format!(
                "This Godot project does not have Fennara installed yet. Run `fennara install` from {} first.",
                display_path(&project_dir)
            ),
        ));
    }
    let project_version = if options.prepare {
        Some(
            project_addon::inspect(&project_dir)
                .map_err(|error| operation::failure(FailureClass::ProjectInvalid, error))?
                .ok_or_else(|| {
                    operation::failure(
                        FailureClass::ProjectInvalid,
                        "The project does not contain a complete Fennara addon to update.",
                    )
                })?
                .version,
        )
    } else {
        project_install::project_addon_version(&project_dir)
    };

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
        println!("guidance: refreshing AGENTS.md and addons/fennara/ai/guidelines.md");
        project_guidance::write(&project_dir)?;
        println!("Fennara is already up to date.");
        println!("version: {}", package.version);
        println!("project: {}", display_path(&project_dir));
        println!("guidance: refreshed AGENTS.md and addons/fennara/ai/guidelines.md");
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

struct UpdateOptions {
    version: String,
    project_dir: Option<PathBuf>,
    no_self_update: bool,
    prepare: bool,
}

impl UpdateOptions {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut version = "latest".to_string();
        let mut project_dir = None;
        let mut no_self_update = false;
        let mut prepare = false;
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
        args
    }
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

    #[test]
    fn continuation_args_resume_without_self_update() {
        let options = UpdateOptions {
            version: "1.2.3".to_string(),
            project_dir: Some(PathBuf::from("demo-project")),
            no_self_update: false,
            prepare: false,
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
}
