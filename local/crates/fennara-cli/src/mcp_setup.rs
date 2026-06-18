use crate::app_layout::{AppLayout, binary_name, display_path};
use serde_json::{Map, Value, json};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Table, value};

const MCP_STARTUP_TIMEOUT_SEC: i64 = 30;
const MCP_TOOL_TIMEOUT_SEC: i64 = 300;

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
    if targets.claude_code {
        reports.push(configure_claude_code(&mcp_path)?);
    }
    if targets.claude_desktop {
        reports.push(configure_claude_desktop(&mcp_path)?);
    }
    if targets.gemini || targets.antigravity {
        reports.extend(configure_antigravity_gemini(&mcp_path)?);
    }
    if targets.cline {
        reports.extend(configure_cline(&mcp_path)?);
    }
    if targets.cursor {
        reports.push(configure_json_mcp(
            "Cursor",
            cursor_config_path()?,
            cursor_backup_path,
            json_command_entry(&mcp_path),
        )?);
    }
    if targets.vscode {
        reports.push(configure_json_server_config(
            "VS Code",
            vscode_config_path()?,
            vscode_backup_path,
            json_stdio_entry(&mcp_path),
            "servers",
        )?);
    }
    if targets.opencode {
        reports.push(configure_json_server_config(
            "OpenCode",
            opencode_config_path()?,
            opencode_backup_path,
            json_opencode_entry(&mcp_path),
            "mcp",
        )?);
    }
    if targets.windsurf {
        reports.push(configure_json_mcp(
            "Windsurf",
            windsurf_config_path()?,
            windsurf_backup_path,
            json_windsurf_entry(&mcp_path),
        )?);
    }
    if targets.kiro {
        reports.push(configure_json_mcp(
            "Kiro",
            kiro_config_path()?,
            kiro_backup_path,
            json_kiro_entry(&mcp_path),
        )?);
    }
    if targets.codex {
        reports.push(configure_codex(&mcp_path)?);
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
    claude_code: bool,
    claude_desktop: bool,
    gemini: bool,
    antigravity: bool,
    cline: bool,
    cursor: bool,
    vscode: bool,
    opencode: bool,
    windsurf: bool,
    kiro: bool,
    codex: bool,
}

impl Targets {
    fn parse(args: Vec<&str>) -> Result<Self, String> {
        let mut targets = Self {
            claude: false,
            claude_code: false,
            claude_desktop: false,
            gemini: false,
            antigravity: false,
            cline: false,
            cursor: false,
            vscode: false,
            opencode: false,
            windsurf: false,
            kiro: false,
            codex: false,
        };

        for arg in args {
            match arg {
                "--claude" => targets.claude = true,
                "--claude-code" => targets.claude_code = true,
                "--claude-desktop" => targets.claude_desktop = true,
                "--gemini" => targets.gemini = true,
                "--antigravity" => targets.antigravity = true,
                "--cline" => targets.cline = true,
                "--cursor" => targets.cursor = true,
                "--vscode" => targets.vscode = true,
                "--opencode" => targets.opencode = true,
                "--windsurf" => targets.windsurf = true,
                "--kiro" => targets.kiro = true,
                "--codex" => targets.codex = true,
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
        self.claude
            || self.claude_code
            || self.claude_desktop
            || self.gemini
            || self.antigravity
            || self.cline
            || self.cursor
            || self.vscode
            || self.opencode
            || self.windsurf
            || self.kiro
            || self.codex
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
  fennara mcp-setup --claude-code
  fennara mcp-setup --claude-desktop
  fennara mcp-setup --gemini
  fennara mcp-setup --antigravity
  fennara mcp-setup --cline
  fennara mcp-setup --cursor
  fennara mcp-setup --vscode
  fennara mcp-setup --opencode
  fennara mcp-setup --windsurf
  fennara mcp-setup --kiro
  fennara mcp-setup --codex
"
    );
}

fn configure_claude(mcp_path: &Path) -> Result<Vec<ConfigReport>, String> {
    let mut reports = vec![configure_claude_code(mcp_path)?];

    match configure_claude_desktop(mcp_path) {
        Ok(report) => reports.push(report),
        Err(error) => println!("Claude Desktop skipped: {error}"),
    }

    Ok(reports)
}

fn configure_claude_code(mcp_path: &Path) -> Result<ConfigReport, String> {
    configure_json_mcp(
        "Claude Code",
        claude_code_config_path()?,
        claude_code_backup_path,
        json_command_args_env_entry(mcp_path),
    )
}

fn configure_claude_desktop(mcp_path: &Path) -> Result<ConfigReport, String> {
    configure_json_mcp(
        "Claude Desktop",
        claude_desktop_config_path()?,
        claude_desktop_backup_path,
        json_stdio_entry(mcp_path),
    )
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

fn configure_cline(mcp_path: &Path) -> Result<Vec<ConfigReport>, String> {
    let mut config_paths = detected_cline_config_paths()?;
    if config_paths.is_empty() {
        config_paths.push(cline_cli_config_path()?);
    }

    let mut reports = Vec::new();
    for path in config_paths {
        reports.push(configure_json_mcp(
            "Cline",
            path,
            cline_backup_path,
            json_cline_entry(mcp_path),
        )?);
    }
    Ok(reports)
}

fn configure_json_mcp(
    app_name: &str,
    config_path: PathBuf,
    backup_path_fn: fn(&Path) -> PathBuf,
    server_entry: Value,
) -> Result<ConfigReport, String> {
    configure_json_server_config(
        app_name,
        config_path,
        backup_path_fn,
        server_entry,
        "mcpServers",
    )
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

fn json_cline_entry(mcp_path: &Path) -> Value {
    json!({
        "command": display_path(mcp_path),
        "args": [],
        "env": {},
        "timeout": MCP_TOOL_TIMEOUT_SEC
    })
}

fn json_stdio_entry(mcp_path: &Path) -> Value {
    json!({
        "type": "stdio",
        "command": display_path(mcp_path),
        "args": [],
        "env": {}
    })
}

fn json_opencode_entry(mcp_path: &Path) -> Value {
    json!({
        "type": "local",
        "command": [display_path(mcp_path)],
        "enabled": true,
        "timeout": MCP_TOOL_TIMEOUT_SEC * 1000
    })
}

fn json_windsurf_entry(mcp_path: &Path) -> Value {
    json!({
        "command": display_path(mcp_path),
        "args": [],
        "env": {},
        "disabled": false,
        "alwaysAllow": []
    })
}

fn json_kiro_entry(mcp_path: &Path) -> Value {
    json!({
        "command": display_path(mcp_path),
        "args": [],
        "env": {},
        "disabled": false,
        "timeout": MCP_TOOL_TIMEOUT_SEC * 1000
    })
}

fn parse_json_config(raw: &str, config_path: &Path) -> Result<Value, String> {
    serde_json::from_str(raw)
        .or_else(|_| json5::from_str(raw))
        .map_err(|err| format!("failed to parse {}: {err}", display_path(config_path)))
}

fn configure_codex(mcp_path: &Path) -> Result<ConfigReport, String> {
    let config_path = codex_config_path()?;
    let existed_before = config_path.is_file();
    let had_fennara_before = if existed_before {
        toml_config_contains_fennara(&config_path)?
    } else {
        false
    };

    let mut document = if existed_before {
        let raw = fs::read_to_string(&config_path)
            .map_err(|err| format!("failed to read {}: {err}", display_path(&config_path)))?;
        if raw.trim().is_empty() {
            DocumentMut::new()
        } else {
            raw.parse::<DocumentMut>()
                .map_err(|err| format!("failed to parse {}: {err}", display_path(&config_path)))?
        }
    } else {
        DocumentMut::new()
    };

    upsert_codex_fennara_entry(&mut document, mcp_path)?;

    let parent_dir = config_path
        .parent()
        .ok_or_else(|| "failed to resolve Codex config directory".to_string())?;
    fs::create_dir_all(parent_dir)
        .map_err(|err| format!("failed to create {}: {err}", display_path(parent_dir)))?;

    let backup_path = codex_backup_path(&config_path);
    if existed_before {
        fs::copy(&config_path, &backup_path).map_err(|err| {
            format!(
                "failed to back up {} to {}: {err}",
                display_path(&config_path),
                display_path(&backup_path)
            )
        })?;
    } else {
        fs::write(&backup_path, "").map_err(|err| {
            format!(
                "failed to create backup {}: {err}",
                display_path(&backup_path)
            )
        })?;
    }

    fs::write(&config_path, document.to_string())
        .map_err(|err| format!("failed to write {}: {err}", display_path(&config_path)))?;

    Ok(ConfigReport {
        app_name: "Codex".to_string(),
        config_path,
        backup_path,
        existed_before,
        had_fennara_before,
    })
}

fn toml_config_contains_fennara(config_path: &Path) -> Result<bool, String> {
    let raw = fs::read_to_string(config_path)
        .map_err(|err| format!("failed to read {}: {err}", display_path(config_path)))?;
    if raw.trim().is_empty() {
        return Ok(false);
    }

    let document = raw
        .parse::<DocumentMut>()
        .map_err(|err| format!("failed to parse {}: {err}", display_path(config_path)))?;
    Ok(document
        .get("mcp_servers")
        .and_then(Item::as_table_like)
        .is_some_and(|servers| servers.contains_key("fennara")))
}

fn upsert_codex_fennara_entry(document: &mut DocumentMut, mcp_path: &Path) -> Result<(), String> {
    ensure_table_item(&mut document["mcp_servers"], "Codex mcp_servers")?;
    let servers = document["mcp_servers"]
        .as_table_mut()
        .ok_or_else(|| "Codex mcp_servers must be a TOML table".to_string())?;

    if !servers.contains_key("fennara") {
        servers["fennara"] = Item::Table(Table::new());
    }

    let fennara = servers["fennara"]
        .as_table_mut()
        .ok_or_else(|| "Codex mcp_servers.fennara must be a TOML table".to_string())?;
    fennara["command"] = value(display_path(mcp_path));
    fennara["startup_timeout_sec"] = value(MCP_STARTUP_TIMEOUT_SEC);
    fennara["tool_timeout_sec"] = value(MCP_TOOL_TIMEOUT_SEC);
    Ok(())
}

fn ensure_table_item(item: &mut Item, label: &str) -> Result<(), String> {
    if item.is_none() {
        *item = Item::Table(Table::new());
    }
    if item.is_table() {
        Ok(())
    } else {
        Err(format!("{label} must be a TOML table"))
    }
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

fn cline_cli_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| {
            path.join(".cline")
                .join("data")
                .join("settings")
                .join("cline_mcp_settings.json")
        })
        .ok_or_else(|| "home directory is not available".to_string())
}

fn cline_extension_config_path(app_name: &str) -> Result<PathBuf, String> {
    Ok(roaming_app_data_dir()?
        .join(app_name)
        .join("User")
        .join("globalStorage")
        .join("saoudrizwan.claude-dev")
        .join("settings")
        .join("cline_mcp_settings.json"))
}

fn detected_cline_config_paths() -> Result<Vec<PathBuf>, String> {
    let mut config_paths = Vec::new();
    add_existing_unique_path(&mut config_paths, cline_cli_config_path()?);
    for app_name in ["Cursor", "Antigravity", "Code"] {
        add_existing_unique_path(&mut config_paths, cline_extension_config_path(app_name)?);
    }
    Ok(config_paths)
}

fn add_existing_unique_path(paths: &mut Vec<PathBuf>, path: PathBuf) {
    if path.is_file() && !paths.iter().any(|existing| existing == &path) {
        paths.push(path);
    }
}

fn cline_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("cline_mcp_settings.backup.json")
}

fn codex_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".codex").join("config.toml"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn codex_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("config.backup.toml")
}

fn cursor_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".cursor").join("mcp.json"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn cursor_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("mcp.backup.json")
}

fn vscode_config_path() -> Result<PathBuf, String> {
    Ok(roaming_app_data_dir()?
        .join("Code")
        .join("User")
        .join("mcp.json"))
}

fn vscode_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("mcp.backup.json")
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

fn opencode_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".config").join("opencode").join("opencode.json"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn opencode_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("opencode.backup.json")
}

fn windsurf_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| {
            path.join(".codeium")
                .join("windsurf")
                .join("mcp_config.json")
        })
        .ok_or_else(|| "home directory is not available".to_string())
}

fn windsurf_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("mcp_config.backup.json")
}

fn kiro_config_path() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".kiro").join("settings").join("mcp.json"))
        .ok_or_else(|| "home directory is not available".to_string())
}

fn kiro_backup_path(config_path: &Path) -> PathBuf {
    config_path.with_file_name("mcp.backup.json")
}

#[cfg(target_os = "windows")]
fn roaming_app_data_dir() -> Result<PathBuf, String> {
    env::var_os("APPDATA")
        .map(PathBuf::from)
        .ok_or_else(|| "APPDATA is not set".to_string())
}

#[cfg(target_os = "macos")]
fn roaming_app_data_dir() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join("Library").join("Application Support"))
        .ok_or_else(|| "home directory is not available".to_string())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn roaming_app_data_dir() -> Result<PathBuf, String> {
    home_dir()
        .map(|path| path.join(".config"))
        .ok_or_else(|| "home directory is not available".to_string())
}
