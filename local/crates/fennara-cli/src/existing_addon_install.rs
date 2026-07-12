use crate::app_layout::{AppLayout, display_path};
use crate::daemon_setup::{self, ReadyState};
use crate::operation::{self, FailureClass, Phase};
use crate::project_addon::ExistingAddon;
use crate::project_guidance;
use crate::release_package;
use crate::webview_prereq;
use std::path::Path;

pub fn run(
    project_dir: &Path,
    addon: ExistingAddon,
    requested_version: Option<&str>,
) -> Result<(), String> {
    install_with(project_dir, addon, requested_version, &mut RealDependencies)
}

trait InstallDependencies {
    type Activation;

    fn check_daemon(&mut self, version: &str) -> Result<(), String>;
    fn install_package(&mut self, version: &str) -> Result<String, String>;
    fn activate_package(&mut self, version: &str) -> Result<Self::Activation, String>;
    fn ensure_daemon(&mut self, version: &str) -> Result<ReadyState, String>;
    fn restore_activation(&mut self, activation: Self::Activation) -> Result<(), String>;
    fn check_webview(&mut self) -> Result<(), String>;
}

struct RealDependencies;

impl InstallDependencies for RealDependencies {
    type Activation = release_package::ActivationReceipt;

    fn check_daemon(&mut self, version: &str) -> Result<(), String> {
        daemon_setup::check_compatibility(version).map(|_| ())
    }

    fn install_package(&mut self, version: &str) -> Result<String, String> {
        release_package::stage_exact_package(version).map(|package| package.version)
    }

    fn activate_package(&mut self, version: &str) -> Result<Self::Activation, String> {
        release_package::activate_package(version)
    }

    fn ensure_daemon(&mut self, version: &str) -> Result<ReadyState, String> {
        let layout = AppLayout::detect()?;
        daemon_setup::ensure_running(&layout, version)
    }

    fn restore_activation(&mut self, activation: Self::Activation) -> Result<(), String> {
        release_package::restore_activation(activation)
    }

    fn check_webview(&mut self) -> Result<(), String> {
        webview_prereq::warn_for_current_platform()
    }
}

fn install_with(
    project_dir: &Path,
    addon: ExistingAddon,
    requested_version: Option<&str>,
    dependencies: &mut impl InstallDependencies,
) -> Result<(), String> {
    if let Some(requested) = requested_version
        && requested != addon.version
    {
        return Err(operation::failure(
            FailureClass::ProjectInvalid,
            format!(
                "the existing project addon is version {}, but --version requested {requested}; remove --version to install the matching local components",
                addon.version
            ),
        ));
    }

    println!("Found an existing Fennara addon");
    println!("addon version: {}", addon.version);
    println!("addon: keeping the existing project files unchanged");
    println!("package: resolving exact release {}", addon.version);
    operation::set_requested_version(&addon.version)?;
    operation::set_component("addon", &addon.version)?;
    dependencies
        .check_daemon(&addon.version)
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    let package_version = dependencies.install_package(&addon.version)?;
    if package_version != addon.version {
        return Err(operation::failure(
            FailureClass::ManifestInvalid,
            format!(
                "release resolution returned version {}, but the existing addon requires {}",
                package_version, addon.version
            ),
        ));
    }
    dependencies
        .check_daemon(&addon.version)
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    operation::phase(
        Phase::Validating,
        "Checking project and platform prerequisites",
    )?;
    dependencies
        .check_webview()
        .map_err(|error| operation::failure(FailureClass::ValidationFailed, error))?;
    println!("guidance: writing project AGENTS.md");
    project_guidance::write_project_files(project_dir)
        .map_err(|error| operation::failure(FailureClass::StageFilesystem, error))?;

    let activation = dependencies.activate_package(&addon.version)?;
    let daemon_state = match dependencies.ensure_daemon(&addon.version) {
        Ok(state) => state,
        Err(error) => {
            if let Err(rollback_error) = dependencies.restore_activation(activation) {
                return Err(operation::failure(
                    FailureClass::RollbackFailed,
                    format!(
                        "daemon validation failed: {error}; restoring the previous runtime selection also failed: {rollback_error}"
                    ),
                ));
            }
            return Err(operation::failure(FailureClass::ValidationFailed, error));
        }
    };
    match daemon_state {
        ReadyState::AlreadyRunning => {
            println!("daemon: version {} is already running", addon.version)
        }
        ReadyState::Started => println!("daemon: started version {}", addon.version),
    }

    println!("Installed matching Fennara components");
    println!("version: {}", addon.version);
    println!("project: {}", display_path(project_dir));
    println!("addon: existing files were not replaced");
    println!("daemon: connection confirmed");
    Ok(())
}

#[cfg(test)]
mod tests;
