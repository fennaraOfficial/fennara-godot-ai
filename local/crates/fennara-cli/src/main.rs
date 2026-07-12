mod app_layout;
mod diagnostics;
mod doctor;
mod mcp_setup;
mod operation;
mod project_guidance;
mod project_install;
mod release_client;
#[cfg(test)]
mod release_client_tests;
mod release_manifest;
mod release_package;
mod release_update;
mod self_update;
mod webview_prereq;
mod webview_runtime;

use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let status = match run_tracked(env::args().skip(1).collect()) {
        Ok(()) => 0,
        Err(error) if error.is_empty() => 0,
        Err(error) => {
            eprintln!("error: {error}");
            1
        }
    };
    std::process::exit(status);
}

fn run_tracked(args: Vec<String>) -> Result<(), String> {
    let kind = match args.first().map(String::as_str) {
        Some("install") => Some(operation::OperationKind::Install),
        Some("update") => Some(operation::OperationKind::Update),
        Some("self-update") | Some(self_update::COMPLETE_COMMAND) => {
            Some(operation::OperationKind::SelfUpdate)
        }
        _ => None,
    };
    let is_help = args
        .iter()
        .skip(1)
        .any(|arg| arg == "-h" || arg == "--help");
    if let Some(kind) = kind.filter(|_| !is_help) {
        operation::begin(kind, &args).map_err(|error| {
            format!(
                "[FEN-OPERATION-LOG-INIT] could not initialize durable operation logging: {error}"
            )
        })?;
    }

    match run(args) {
        Ok(()) => {
            operation::finish_success().map_err(|error| {
                format!(
                    "[FEN-OPERATION-LOG-WRITE] operation completed but its final state could not be recorded: {error}"
                )
            })
        }
        Err(error) if error.is_empty() => Err(error),
        Err(error) => {
            match operation::finish_failure(&error) {
                Ok(Some((code, operation_id, log_path))) => Err(format!(
                    "[{code}] {error}\noperation: {operation_id}\noperation log: {}",
                    app_layout::display_path(&log_path)
                )),
                Ok(None) => Err(error),
                Err(journal_error) => Err(format!(
                    "[FEN-OPERATION-LOG-WRITE] {error}\nfailed to record the operation failure: {journal_error}"
                )),
            }
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    match args.first().map(String::as_str) {
        None | Some("-h") | Some("--help") | Some("help") => {
            print_help();
            Ok(())
        }
        Some("-V") | Some("--version") | Some("version") => {
            println!("fennara {VERSION}");
            Ok(())
        }
        Some("doctor") => doctor::run(args.iter().skip(1).map(String::as_str).collect()),
        Some("diagnostics") => diagnostics::run(args.iter().skip(1).map(String::as_str).collect()),
        Some("install") => project_install::run(args.iter().skip(1).map(String::as_str).collect()),
        Some("mcp-setup") => mcp_setup::run(args.iter().skip(1).map(String::as_str).collect()),
        Some("update") => release_update::run(args.iter().skip(1).map(String::as_str).collect()),
        Some("self-update") => self_update::run(args.iter().skip(1).map(String::as_str).collect()),
        Some(self_update::COMPLETE_COMMAND) => {
            self_update::complete(args.iter().skip(1).cloned().collect())
        }
        Some(command) => Err(format!("unknown command: {command}")),
    }
}

fn print_help() {
    println!(
        "\
Fennara CLI {VERSION}

Usage:
  fennara doctor [--repair]
  fennara diagnostics [--operation <operation-id>] [--json]
  fennara install [--project <path>] [--version <version>]
  fennara mcp-setup <target flags>
  fennara update [--version <version>] [--project <path>] [--no-self-update]
  fennara self-update [--version <version>]
  fennara --version
  fennara --help

Commands:
  doctor     Inspect the local Fennara install layout
  diagnostics
             Show a sanitized install or update operation report
  install    Set up Fennara in a Godot project
  mcp-setup  Configure an MCP app to launch Fennara
  update     Update the CLI, local package, addon, and project guidance
  self-update
             Update only the installed Fennara CLI

Options:
  --repair   Create missing base app-data directories during doctor
"
    );
}
