use crate::app_layout::display_path;
use crate::operation;

pub fn run(args: Vec<&str>) -> Result<(), String> {
    let mut operation_id = None;
    let mut json_only = false;
    let mut index = 0;
    while index < args.len() {
        match args[index] {
            "--operation" => {
                index += 1;
                operation_id = Some(value_arg(&args, index, "--operation")?);
            }
            arg if arg.starts_with("--operation=") => {
                operation_id = Some(arg.trim_start_matches("--operation="));
            }
            "--json" => json_only = true,
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            other => return Err(format!("unknown diagnostics option: {other}")),
        }
        index += 1;
    }

    let (state, state_path, log_path) = operation::diagnostics(operation_id)?;
    if !json_only {
        println!("Fennara operation diagnostics");
        println!(
            "operation: {}",
            state
                .get("operation_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown")
        );
        println!(
            "phase: {}",
            state
                .get("phase")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown")
        );
        println!("state: {}", display_path(&state_path));
        println!("events: {}", display_path(&log_path));
        println!("sanitized report:");
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&state)
            .map_err(|err| format!("failed to serialize diagnostics: {err}"))?
    );
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
Show a sanitized Fennara installation or update operation report.

Usage:
  fennara diagnostics
  fennara diagnostics --operation <operation-id>
  fennara diagnostics --operation <operation-id> --json
"
    );
}
