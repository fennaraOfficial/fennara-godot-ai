use crate::app_layout::display_path;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn hash_directory(root: &Path) -> Result<String, String> {
    let metadata = fs::symlink_metadata(root)
        .map_err(|error| format!("failed to inspect {}: {error}", display_path(root)))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(format!(
            "refusing to hash non-directory or linked addon path {}",
            display_path(root)
        ));
    }

    let mut files = Vec::new();
    collect_files(root, root, &mut files)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut digest = Sha256::new();
    for (relative, path) in files {
        let bytes = fs::read(&path)
            .map_err(|error| format!("failed to read {}: {error}", display_path(&path)))?;
        digest.update((relative.len() as u64).to_le_bytes());
        digest.update(relative.as_bytes());
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(&bytes);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn collect_files(
    root: &Path,
    current: &Path,
    files: &mut Vec<(String, PathBuf)>,
) -> Result<(), String> {
    for entry in fs::read_dir(current)
        .map_err(|error| format!("failed to read {}: {error}", display_path(current)))?
    {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read an entry in {}: {error}",
                display_path(current)
            )
        })?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {}: {error}", display_path(&path)))?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "refusing to hash linked addon entry {}",
                display_path(&path)
            ));
        }
        if metadata.is_dir() {
            collect_files(root, &path, files)?;
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .map_err(|error| format!("failed to normalize addon path: {error}"))?
                .to_string_lossy()
                .replace('\\', "/");
            files.push((relative, path));
        } else {
            return Err(format!(
                "refusing to hash unsupported addon entry {}",
                display_path(&path)
            ));
        }
    }
    Ok(())
}
