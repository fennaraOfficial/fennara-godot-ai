use crate::app_layout::{AppLayout, binary_name, display_path};
use std::fs;
use std::process::Command;

const CSHARP_LS_VERSION: &str = "0.20.0";

pub fn install() -> Result<(), String> {
    let layout = AppLayout::detect()?;
    layout.ensure_base_dirs()?;

    let tool_dir = layout.tools_dir.join("csharp-ls").join(CSHARP_LS_VERSION);
    let binary_path = tool_dir.join(binary_name("csharp-ls"));

    if binary_path.is_file() {
        println!("C# support is already installed.");
        println!("csharp-ls: {}", display_path(&binary_path));
        return Ok(());
    }

    fs::create_dir_all(&tool_dir)
        .map_err(|err| format!("failed to create {}: {err}", display_path(&tool_dir)))?;

    println!("Installing C# support...");
    println!("tool: csharp-ls {CSHARP_LS_VERSION}");
    println!("target: {}", display_path(&tool_dir));

    let output = Command::new("dotnet")
        .args([
            "tool",
            "install",
            "csharp-ls",
            "--version",
            CSHARP_LS_VERSION,
            "--tool-path",
        ])
        .arg(&tool_dir)
        .output()
        .map_err(|err| format!("failed to run dotnet: {err}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let detail = if !stderr.is_empty() { stderr } else { stdout };
        return Err(format!(
            "failed to install csharp-ls with dotnet tool install: {detail}"
        ));
    }

    if !binary_path.is_file() {
        return Err(format!(
            "dotnet reported success, but csharp-ls was not found at {}",
            display_path(&binary_path)
        ));
    }

    println!("C# support installed.");
    println!("csharp-ls: {}", display_path(&binary_path));
    Ok(())
}
