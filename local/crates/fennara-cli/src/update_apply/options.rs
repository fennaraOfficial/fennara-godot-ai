use crate::operation;
use std::path::PathBuf;

pub(super) struct ApplyOptions {
    pub project_dir: PathBuf,
    pub operation_id: String,
    pub wait_for_pid: u32,
    pub godot_executable: PathBuf,
}

impl ApplyOptions {
    pub(super) fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut project_dir = None;
        let mut operation_id = None;
        let mut wait_for_pid = None;
        let mut godot_executable = None;
        let mut index = 0;
        while index < args.len() {
            match args[index] {
                "--project" => {
                    index += 1;
                    project_dir = Some(PathBuf::from(value(&args, index, "--project")?));
                }
                "--resume-operation" => {
                    index += 1;
                    operation_id = Some(value(&args, index, "--resume-operation")?.to_string());
                }
                "--wait-for-pid" => {
                    index += 1;
                    wait_for_pid = Some(parse_pid(value(&args, index, "--wait-for-pid")?)?);
                }
                "--godot-executable" => {
                    index += 1;
                    godot_executable =
                        Some(PathBuf::from(value(&args, index, "--godot-executable")?));
                }
                other => return Err(format!("unknown update completion option: {other}")),
            }
            index += 1;
        }
        let operation_id =
            operation_id.ok_or_else(|| "--resume-operation is required".to_string())?;
        operation::validate_id(&operation_id)?;
        Ok(Self {
            project_dir: project_dir.ok_or_else(|| "--project is required".to_string())?,
            operation_id,
            wait_for_pid: wait_for_pid.ok_or_else(|| "--wait-for-pid is required".to_string())?,
            godot_executable: godot_executable
                .ok_or_else(|| "--godot-executable is required".to_string())?,
        })
    }
}

fn value<'a>(args: &'a [&str], index: usize, option: &str) -> Result<&'a str, String> {
    args.get(index)
        .copied()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn parse_pid(value: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .ok()
        .filter(|pid| *pid > 0)
        .ok_or_else(|| format!("invalid process ID: {value}"))
}
