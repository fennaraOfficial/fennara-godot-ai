use crate::app_layout::{AppLayout, binary_name, display_path};
use serde_json::{Map, Value, json};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(args: Vec<&str>) -> Result<(), String> {
    let targets = Targets::parse(args)?;
    if !targets.any() {
        print_help();
        return Ok(());
    }

    let layout = AppLayout::detect()?;
    let mcp_path = layout.bin_dir.join(binary_name("fennara-mcp"));
    if !mcp_path.is_file() {
        return Err(format!(
            "Fennara MCP runtime is not installed yet. Run `fennara install` inside a Godot project first. Missing: {}",
            display_path(&mcp_path)
        ));
    }

    let mut reports = Vec::new();
    if targets.claude {
        reports.extend(configure_claude(&mcp_path)?);
    }
    if targets.gemini || targets.antigravity {
        reports.extend(configure_antigravity_gemini(&mcp_path)?);
    }

    println!("Fennara MCP setup complete.");
    for report in reports {
        println!(
            "- {}: {}",
            report.app_name,
            if report.had_fennara_before {
                "updated existing entry"
            } else if report.existed_before {
                "added entry"
            } else {
                "created config"
            }
        );
        println!("  config: {}", display_path(&report.config_path));
        println!("  backup: {}", display_path(&report.backup_path));
    }
    println!("Restart the selected MCP app so it reloads Fennara.");
    Ok(())
}

struct Targets {
    claude: bool,
    gemini: bool,
    antigravity: bool,
}

impl Targets {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut targets = Self {
            claude: false,
            gemini: false,
            antigravity: false,
        };

        for arg in args {
            match arg {
                "--claude" => targets.claude = true,
                "--gemini" => targets.gemini = true,
                "--antigravity" => targets.antigravity = true,
                "-h" | "--help" => {
                    print_help();
                    return Err("".to_string());
                }
                other => return Err(format!("unknown mcp-setup option: {other}")),
            }
        }

        Ok(targets)
    }

    fn any(&self) -> bool {
        self.claude || self.gemini || self.antigravity
    }
}

struct ConfigReport {
    app_name: String,
    config_path: PathBuf,
    backup_path: PathBuf,
    existed_before: bool,
    had_fennara_before: bool,
}

fn print_help() {
    println!(
        "\
Configure MCP apps to run Fennara.

Usage:
  fennara mcp-setup --claude
  fennara mcp-setup --gemini
  fennara mcp-setup --antigravity
"
    );
}

fn configure_claude(mcp_path: &Path) -> Result<Vec<ConfigReport>, String> {
    let code_path = claude_code_config_path()?;
    let mut reports = vec![configure_json_server_config(
        "Claude Code",
        code_path,
        claude_code_backup_path,
        json_command_args_env_entry(mcp_path),
        "mcpServers",
    )?];

    match claude_desktop_config_path() {
        Ok(path) => {
            reports.push(configure_json_server_config(
                "Claude Desktop",
                path,
                claude_desktop_backup_path,
                json_claude_desktop_stdio_entry(mcp_path),
                "mcpServers",
            )?);
        }
        Err(error) => {
            println!("Claude Desktop skipped: {error}");
        }
    }

    Ok(reports)
}

fn configure_antigravity_gemini(mcp_path: &Path) -> Result<Vec<ConfigReport>, String> {
    let targets = [
        (
            "Antigravity shared",
            antigravity_shared_config_path()?,
            antigravity_backup_path as fn(&Path) -> PathBuf,
        ),
        (
            "Antigravity IDE",
            antigravity_ide_config_path()?,
            antigravity_backup_path as fn(&Path) -> PathBuf,
        ),
        (
            "Gemini CLI",
            gemini_cli_config_path()?,
            gemini_cli_backup_path as fn(&Path) -> PathBuf,
        ),
    ];
    let mut reports = Vec::new();
    for (name, path, backup_path_fn) in targets {
        reports.push(configure_json_server_config(
            name,
            path,
            backup_path_fn,
            json_command_entry(mcp_path),
            "mcpServers",
        )?);
    }
    Ok(reports)
}

fn configure_json_server_config(
    app_name: &str,
    config_path: PathBuf,
    backup_path_fn: fn(&Path) -> PathBuf,
    server_entry: Value,
    servers_key: &str,
) -> Result<ConfigReport, String> {
    let existed_before = config_path.is_file();
    let mut root = if existed_before {
        let raw = fs::read_to_string(&config_path)
            .map_err(|err| format!("failed to read {}: {err}", display_path(&config_path)))?;
        if raw.trim().is_empty() {
            json!({})
        } else {
            parse_json_config(&raw, &config_path)?
        }
    } else {
        json!({})
    };

    let root_object = root
        .as_object_mut()
        .ok_or_else(|| format!("{app_name} config must be a JSON object"))?;
    let servers = root_object
        .entry(servers_key.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    let servers_object = servers
        .as_object_mut()
        .ok_or_else(|| format!("{app_name} {servers_key} must be a JSON object"))?;
    let had_fennara_before = servers_object.contains_key("fennara");
    servers_object.insert("fennara".to_string(), server_entry);

    let parent_dir = config_path
        .parent()
        .ok_or_else(|| format!("failed to resolve {app_name} config directory"))?;
    fs::create_dir_all(parent_dir)
        .map_err(|err| format!("failed to create {}: {err}", display_path(parent_dir)))?;

    let backup_path = backup_path_fn(&config_path);
    if existed_before {
        fs::copy(&config_path, &backup_path).map_err(|err| {
            format!(
                "failed to back up {} to {}: {err}",
                display_path(&config_path),
                display_path(&backup_path)
            )
        })?;
    } else {
        fs::write(&backup_path, "{}\n").map_err(|err| {
            format!(
                "failed to create backup {}: {err}",
                display_path(&backup_path)
            )
        })?;
    }

    let serialized = serde_json::to_string_pretty(&root)
        .map_err(|err| format!("failed to serialize {app_name} config: {err}"))?;
    fs::write(&config_path, format!("{serialized}\n"))
        .map_err(|err| format!("failed to write {}: {err}", display_path(&config_path)))?;

    Ok(ConfigReport {
        app_name: app_name.to_string(),
        config_path,
        backup_path,
        existed_before,
        had_fennara_before,
    })
}

fn json_command_entry(mcp_path: &Path) -> Value {
    json!({
        "command": display_path(mcp_path)
    })
}

fn json_command_args_env_entry(mcp_path: &Path) -> Value {
    json!({
        "command": display_path(mcp_path),
        "args": [],
        "env": {}
    })
}

fn json_claude_desktop_stdio_entry(mcp_path: &Path) -> Value {
    json!({
        "type": "stdio",
        "command": display_path(mcp_path),
        "args": [],
        "env": {}
    })
}

fn parse_json_config(raw: &str, config_path: &Path) -> Result<Value, String> {
    serde_json::from_str(raw)
        .or_else(|_| json5::from_str(raw))
        .map_err(|err| format!("failed to parse {}: {err}", display_path(config_path)))
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

fn antigravity_shared_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".gemini").join("config").join("mcp_config.json"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn antigravity_ide_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| {
            path.join(".gemini")
                .join("antigravity")
                .join("mcp_config.json")
        })
        .ok_or_else(|| "home directory is not available".to_string())
}

fn gemini_cli_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".gemini").join("settings.json"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn antigravity_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("mcp_config.backup.json")
}

fn gemini_cli_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("settings.backup.json")
}

fn claude_code_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".claude.json"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn claude_code_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name(".claude.backup.json")
}

#[cfg(target_os = "windows")]
fn claude_desktop_config_path() -> Result<PathBuf, String> {
    env::var_os("APPDATA")
        .map(PathBuf::from)
        .map(|path| path.join("Claude").join("claude_desktop_config.json"))
        .ok_or_else(|| "APPDATA is not set".to_string())
}

#[cfg(target_os = "macos")]
fn claude_desktop_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| {
            path.join("Library")
                .join("Application Support")
                .join("Claude")
                .join("claude_desktop_config.json")
        })
        .ok_or_else(|| "home directory is not available".to_string())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn claude_desktop_config_path() -> Result<PathBuf, String> {
    Err("Claude Desktop MCP config is documented for macOS and Windows only.".to_string())
}

fn claude_desktop_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("claude_desktop_config.backup.json")
}
