use crate::app_layout::{arch_name, display_path, platform_name};
use crate::project_install::project_addon_dir;
use crate::release_identity::ReleaseIdentity;
use crate::release_manifest::compare_versions;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub struct ExistingAddon {
    pub version: String,
}

pub fn inspect(project_dir: &Path) -> Result<Option<ExistingAddon>, String> {
    let addon_dir = project_addon_dir(project_dir);
    let manifest_path = addon_dir.join("fennara.gdextension");
    if !manifest_path.is_file() {
        return Ok(None);
    }

    validate(&addon_dir).map(Some)
}

pub fn validate(addon_dir: &Path) -> Result<ExistingAddon, String> {
    let manifest_path = addon_dir.join("fennara.gdextension");
    if !manifest_path.is_file() {
        return Err(format!(
            "the Fennara addon is missing its extension manifest at {}",
            display_path(&manifest_path)
        ));
    }

    let version_path = addon_dir.join("VERSION");
    let version = fs::read_to_string(&version_path)
        .map_err(|error| {
            format!(
                "the existing Fennara addon is missing a readable VERSION file at {}: {error}",
                display_path(&version_path)
            )
        })?
        .trim()
        .to_string();
    if version.is_empty() || compare_versions(&version, &version).is_none() {
        return Err(format!(
            "the existing Fennara addon has an invalid VERSION value {version:?} at {}",
            display_path(&version_path)
        ));
    }
    ReleaseIdentity::load(addon_dir, &version)?;

    let library_path = current_library_path(addon_dir, &manifest_path)?;
    if !library_path.is_file() {
        return Err(format!(
            "the existing Fennara addon is missing its {} {} editor library at {}",
            platform_name(),
            arch_name(),
            display_path(&library_path)
        ));
    }

    let guidelines_path = addon_dir.join("ai").join("guidelines.md");
    if !guidelines_path.is_file() {
        return Err(format!(
            "the existing Fennara addon is missing its guidance file at {}",
            display_path(&guidelines_path)
        ));
    }

    Ok(ExistingAddon { version })
}

fn current_library_path(addon_dir: &Path, manifest_path: &Path) -> Result<PathBuf, String> {
    let raw = fs::read_to_string(manifest_path).map_err(|error| {
        format!(
            "failed to read existing addon manifest {}: {error}",
            display_path(manifest_path)
        )
    })?;
    let mut in_libraries = false;
    for line in raw.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_libraries = line == "[libraries]";
            continue;
        }
        if !in_libraries || line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if !library_key_matches(key.trim()) {
            continue;
        }
        let resource_path = value.trim().trim_matches(['"', '\'']);
        let relative = resource_path
            .strip_prefix("res://addons/fennara/")
            .ok_or_else(|| {
                format!(
                    "existing addon library path {resource_path:?} is outside res://addons/fennara/"
                )
            })?;
        let relative = Path::new(relative);
        if relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(format!(
                "existing addon library path {resource_path:?} is not a safe relative path"
            ));
        }
        return Ok(addon_dir.join(relative));
    }

    Err(format!(
        "existing addon manifest {} has no editor library for {} {}",
        display_path(manifest_path),
        platform_name(),
        arch_name()
    ))
}

fn library_key_matches(key: &str) -> bool {
    let parts: Vec<_> = key.split('.').collect();
    if !parts.contains(&platform_name()) || !parts.contains(&"editor") {
        return false;
    }
    let known_architectures = ["x86_64", "x86_32", "arm64", "aarch64"];
    let selected_architecture = parts.iter().find(|part| known_architectures.contains(part));
    selected_architecture.is_none_or(|selected| architecture_matches(selected))
}

fn architecture_matches(selected: &str) -> bool {
    selected == arch_name()
        || matches!(
            (selected, arch_name()),
            ("aarch64", "arm64") | ("arm64", "aarch64")
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn inspects_matching_existing_addon() {
        let root = test_root("valid");
        let project = root.join("project");
        let addon = write_addon(&project, Some("1.2.3"), true);

        let existing = inspect(&project).unwrap().unwrap();
        assert_eq!(existing.version, "1.2.3");
        assert!(addon.join("fennara.gdextension").is_file());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_existing_addon_without_version() {
        let root = test_root("missing-version");
        let project = root.join("project");
        write_addon(&project, None, true);

        let error = inspect(&project).unwrap_err();
        assert!(error.contains("missing a readable VERSION"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_addon_identity_that_does_not_match_version() {
        let root = test_root("identity-version-mismatch");
        let project = root.join("project");
        let addon = write_addon(&project, Some("1.2.3"), true);
        fs::write(
            addon.join("release.json"),
            serde_json::to_vec(&serde_json::json!({
                "schema_version": 1,
                "track": "stable",
                "version": "1.2.4",
                "release_tag": "v1.2.4"
            }))
            .unwrap(),
        )
        .unwrap();

        let error = inspect(&project).unwrap_err();
        assert!(error.contains("does not match VERSION"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_existing_addon_without_current_library() {
        let root = test_root("missing-library");
        let project = root.join("project");
        write_addon(&project, Some("1.2.3"), false);

        let error = inspect(&project).unwrap_err();
        assert!(error.contains("missing its"));
        assert!(error.contains("editor library"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_directory_in_place_of_current_library() {
        let root = test_root("library-directory");
        let project = root.join("project");
        let addon = write_addon(&project, Some("1.2.3"), false);
        fs::create_dir_all(addon.join("bin/fennara-test-library")).unwrap();

        let error = inspect(&project).unwrap_err();
        assert!(error.contains("missing its"));
        assert!(error.contains("editor library"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_existing_addon_without_guidance() {
        let root = test_root("missing-guidance");
        let project = root.join("project");
        let addon = write_addon(&project, Some("1.2.3"), true);
        fs::remove_file(addon.join("ai/guidelines.md")).unwrap();

        let error = inspect(&project).unwrap_err();
        assert!(error.contains("missing its guidance file"));
        fs::remove_dir_all(root).unwrap();
    }

    fn write_addon(project: &Path, version: Option<&str>, include_library: bool) -> PathBuf {
        let addon = project_addon_dir(project);
        let library_relative = PathBuf::from("bin").join("fennara-test-library");
        fs::create_dir_all(addon.join("bin")).unwrap();
        fs::create_dir_all(addon.join("ai")).unwrap();
        let key = format!("{}.editor.{}", platform_name(), arch_name());
        fs::write(
            addon.join("fennara.gdextension"),
            format!(
                "[configuration]\nentry_symbol = \"fennara_entry\"\n\n[libraries]\n{key} = \"res://addons/fennara/{}\"\n",
                library_relative.display().to_string().replace('\\', "/")
            ),
        )
        .unwrap();
        if let Some(version) = version {
            fs::write(addon.join("VERSION"), version).unwrap();
        }
        if include_library {
            fs::write(addon.join(library_relative), "library").unwrap();
        }
        fs::write(addon.join("ai/guidelines.md"), "store guidance\n").unwrap();
        addon
    }

    fn test_root(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "fennara-existing-addon-{name}-{}-{nonce}",
            std::process::id()
        ))
    }
}
