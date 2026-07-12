use crate::app_layout::{arch_name, display_path, platform_name};
use crate::operation;
use crate::project_addon;
use crate::project_install;
use crate::release_package::InstalledPackage;
use serde_json::json;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const RECEIPT_SCHEMA_VERSION: u64 = 1;

pub struct StagedUpdate {
    pub root: PathBuf,
    pub receipt_path: PathBuf,
    pub version: String,
}

pub fn prepare(
    project_dir: &Path,
    current_version: &str,
    package: &InstalledPackage,
    operation_id: &str,
) -> Result<StagedUpdate, String> {
    operation::validate_id(operation_id)?;
    let source = project_addon::validate(&package.addon_dir)?;
    if source.version != package.version {
        return Err(format!(
            "staged addon version {} did not match package version {}",
            source.version, package.version
        ));
    }

    let staging_parent = project_dir.join(".godot").join("fennara-update");
    let final_root = staging_parent.join(operation_id);
    let preparing_root = staging_parent.join(format!("{operation_id}.preparing"));
    project_install::ensure_target_within_project(project_dir, &staging_parent)?;
    project_install::ensure_target_within_project(project_dir, &final_root)?;
    project_install::ensure_target_within_project(project_dir, &preparing_root)?;
    fs::create_dir_all(&staging_parent).map_err(|error| {
        format!(
            "failed to create update staging directory {}: {error}",
            display_path(&staging_parent)
        )
    })?;
    if final_root.exists() {
        return Err(format!(
            "update staging directory already exists at {}",
            display_path(&final_root)
        ));
    }
    if preparing_root.exists() {
        fs::remove_dir_all(&preparing_root).map_err(|error| {
            format!(
                "failed to clean incomplete update staging directory {}: {error}",
                display_path(&preparing_root)
            )
        })?;
    }

    let mut cleanup = PreparingCleanup::new(preparing_root.clone());
    let staged_addon = preparing_root.join("addon");
    copy_dir_without_links(&package.addon_dir, &staged_addon)?;
    let staged = project_addon::validate(&staged_addon)?;
    if staged.version != package.version {
        return Err(format!(
            "copied addon version {} did not match package version {}",
            staged.version, package.version
        ));
    }

    let receipt_path = preparing_root.join("receipt.json");
    write_receipt(
        &receipt_path,
        operation_id,
        current_version,
        &package.version,
    )?;
    fs::rename(&preparing_root, &final_root).map_err(|error| {
        format!(
            "failed to finalize update staging directory {}: {error}",
            display_path(&final_root)
        )
    })?;
    cleanup.disarm();

    Ok(StagedUpdate {
        receipt_path: final_root.join("receipt.json"),
        root: final_root,
        version: package.version.clone(),
    })
}

fn write_receipt(
    path: &Path,
    operation_id: &str,
    current_version: &str,
    target_version: &str,
) -> Result<(), String> {
    let receipt = json!({
        "schema_version": RECEIPT_SCHEMA_VERSION,
        "operation_id": operation_id,
        "state": "ready_to_close",
        "from_version": current_version,
        "to_version": target_version,
        "platform": platform_name(),
        "architecture": arch_name(),
        "addon": "addon",
    });
    let mut bytes = serde_json::to_vec_pretty(&receipt)
        .map_err(|error| format!("failed to serialize update staging receipt: {error}"))?;
    bytes.push(b'\n');
    let mut file = File::create(path)
        .map_err(|error| format!("failed to create {}: {error}", display_path(path)))?;
    file.write_all(&bytes)
        .map_err(|error| format!("failed to write {}: {error}", display_path(path)))?;
    file.sync_all()
        .map_err(|error| format!("failed to flush {}: {error}", display_path(path)))
}

fn copy_dir_without_links(source: &Path, target: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(source)
        .map_err(|error| format!("failed to inspect {}: {error}", display_path(source)))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "refusing to stage non-directory or linked addon path {}",
            display_path(source)
        ));
    }
    fs::create_dir_all(target)
        .map_err(|error| format!("failed to create {}: {error}", display_path(target)))?;
    for entry in fs::read_dir(source)
        .map_err(|error| format!("failed to read {}: {error}", display_path(source)))?
    {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read an entry in {}: {error}",
                display_path(source)
            )
        })?;
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let metadata = fs::symlink_metadata(&source_path).map_err(|error| {
            format!("failed to inspect {}: {error}", display_path(&source_path))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "refusing to stage linked addon entry {}",
                display_path(&source_path)
            ));
        }
        if metadata.is_dir() {
            copy_dir_without_links(&source_path, &target_path)?;
        } else if metadata.is_file() {
            fs::copy(&source_path, &target_path).map_err(|error| {
                format!(
                    "failed to copy {} to {}: {error}",
                    display_path(&source_path),
                    display_path(&target_path)
                )
            })?;
        } else {
            return Err(format!(
                "refusing to stage unsupported addon entry {}",
                display_path(&source_path)
            ));
        }
    }
    Ok(())
}

struct PreparingCleanup {
    path: PathBuf,
    armed: bool,
}

impl PreparingCleanup {
    fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for PreparingCleanup {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_layout::{arch_name, platform_name};
    use std::ops::Deref;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn stages_valid_addon_without_touching_active_addon() {
        let root = TestRoot::new("success");
        let project = root.join("project");
        let source = root.join("package");
        write_project(&project);
        write_addon(&project.join("addons/fennara"), "1.0.0");
        write_addon(&source, "1.1.0");
        let active_before = fs::read(project.join("addons/fennara/VERSION")).unwrap();
        let package = InstalledPackage {
            version: "1.1.0".to_string(),
            addon_dir: source,
        };

        let staged = prepare(&project, "1.0.0", &package, "update-123-test").unwrap();

        assert_eq!(staged.version, "1.1.0");
        assert!(staged.root.join("addon/fennara.gdextension").is_file());
        assert_eq!(
            fs::read(project.join("addons/fennara/VERSION")).unwrap(),
            active_before
        );
        let receipt: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(staged.receipt_path).unwrap()).unwrap();
        assert_eq!(receipt["state"], "ready_to_close");
        assert_eq!(receipt["from_version"], "1.0.0");
        assert_eq!(receipt["to_version"], "1.1.0");
    }

    #[test]
    fn failed_validation_leaves_no_staging_directory() {
        let root = TestRoot::new("invalid");
        let project = root.join("project");
        let source = root.join("package");
        write_project(&project);
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("VERSION"), "1.1.0\n").unwrap();
        let package = InstalledPackage {
            version: "1.1.0".to_string(),
            addon_dir: source,
        };

        assert!(prepare(&project, "1.0.0", &package, "update-456-test").is_err());
        assert!(
            !project
                .join(".godot/fennara-update/update-456-test")
                .exists()
        );
        assert!(
            !project
                .join(".godot/fennara-update/update-456-test.preparing")
                .exists()
        );
    }

    fn write_project(project: &Path) {
        fs::create_dir_all(project).unwrap();
        fs::write(project.join("project.godot"), "[application]\n").unwrap();
    }

    fn write_addon(addon: &Path, version: &str) {
        fs::create_dir_all(addon.join("bin")).unwrap();
        fs::create_dir_all(addon.join("ai")).unwrap();
        fs::write(addon.join("VERSION"), format!("{version}\n")).unwrap();
        fs::write(addon.join("ai/guidelines.md"), "guidance\n").unwrap();
        fs::write(addon.join("bin/fennara-test-library"), "library").unwrap();
        fs::write(
            addon.join("fennara.gdextension"),
            format!(
                "[libraries]\n{}.editor.{} = \"res://addons/fennara/bin/fennara-test-library\"\n",
                platform_name(),
                arch_name()
            ),
        )
        .unwrap();
    }

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(name: &str) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            Self(std::env::temp_dir().join(format!(
                "fennara-update-stage-{name}-{}-{nonce}",
                std::process::id()
            )))
        }
    }

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
}
