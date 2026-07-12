use crate::app_layout::{AppLayout, display_path};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

const WAIT_TIMEOUT: Duration = Duration::from_secs(30);
const RETRY_DELAY: Duration = Duration::from_millis(50);
const OWNER_WRITE_GRACE: Duration = Duration::from_secs(10);

pub(super) struct InstallLock {
    path: PathBuf,
}

pub(super) fn acquire(layout: &AppLayout, version: &str) -> Result<InstallLock, String> {
    let lock_dir = layout.versions_dir.join(".install-locks");
    fs::create_dir_all(&lock_dir).map_err(|error| {
        format!(
            "failed to create package lock directory {}: {error}",
            display_path(&lock_dir)
        )
    })?;
    let lock_path = lock_dir.join(format!("{}.lock", lock_name(version)));
    let deadline = Instant::now() + WAIT_TIMEOUT;

    loop {
        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&lock_path)
        {
            Ok(mut file) => {
                let lock = InstallLock {
                    path: lock_path.clone(),
                };
                writeln!(file, "{}", std::process::id()).map_err(|error| {
                    format!(
                        "failed to record package lock owner {}: {error}",
                        display_path(&lock_path)
                    )
                })?;
                file.sync_all().map_err(|error| {
                    format!(
                        "failed to flush package lock {}: {error}",
                        display_path(&lock_path)
                    )
                })?;
                return Ok(lock);
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                if lock_is_stale(&lock_path) {
                    let _ = fs::remove_file(&lock_path);
                    continue;
                }
                if Instant::now() >= deadline {
                    return Err(format!(
                        "timed out waiting for package installation lock {}",
                        display_path(&lock_path)
                    ));
                }
                thread::sleep(RETRY_DELAY);
            }
            Err(error) => {
                return Err(format!(
                    "failed to acquire package installation lock {}: {error}",
                    display_path(&lock_path)
                ));
            }
        }
    }
}

fn lock_is_stale(path: &Path) -> bool {
    let owner = fs::read_to_string(path)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok());
    if let Some(pid) = owner {
        let mut system = System::new();
        system.refresh_processes();
        return system.process(Pid::from_u32(pid)).is_none();
    }

    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|age| age > OWNER_WRITE_GRACE)
}

fn lock_name(version: &str) -> String {
    version
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

impl Drop for InstallLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
