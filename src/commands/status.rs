use anyhow::Result;
use colored::*;
use std::path::Path;

use crate::templates;
use crate::utils;

/// Show installation status
pub fn status() -> Result<()> {
    println!("{}", "Checking hagi installation status...".green().bold());
    println!();

    // Check global configuration
    check_global_configuration()?;
    println!();

    // Check project configuration
    check_project_configuration()?;
    println!();

    // Check MCP servers
    check_mcp_servers()?;

    Ok(())
}

/// Check global configuration status
fn check_global_configuration() -> Result<()> {
    println!("{}", "[Global Configuration]".cyan().bold());

    let claude_dir = match utils::claude_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("{} ~/.claude/ not found", "✗".red());
            return Ok(());
        }
    };

    // Only settings.json is installed globally; MCP config is per-project
    let settings_json = claude_dir.join(templates::SETTINGS_JSON);
    let display_name = format!("~/{}/{}", utils::CLAUDE_DIR_NAME, templates::SETTINGS_JSON);
    if settings_json.exists() {
        println!("{} {} - {}", "✓".green(), display_name.bold(), "installed".dimmed());
    } else {
        println!("{} {} - {}", "✗".red(), display_name.bold(), "not found".dimmed());
    }

    Ok(())
}

/// Compare the hagi version recorded at install time with the running binary (major.minor only)
fn print_version_status(claude_dir: &Path) {
    let name = templates::HAGI_VERSION_FILE;
    let Ok(recorded) = std::fs::read_to_string(claude_dir.join(name)) else {
        println!(
            "  {} .claude/{} - {}",
            "✗".red(),
            name,
            "not found (run 'hagi install --refresh')".dimmed()
        );
        return;
    };
    let recorded = recorded.trim();
    let current = env!("CARGO_PKG_VERSION");

    let check = utils::check_version(recorded, current);
    match check.fix() {
        Some(cmd) => println!(
            "  {} installed by hagi {} (current {}) - run {}",
            "⚠".yellow(),
            recorded,
            current,
            cmd.yellow()
        ),
        None if check == utils::VersionCheck::UpToDate => {
            println!("  {} installed by hagi {}", "✓".green(), recorded);
        }
        None => println!(
            "  {} .claude/{} holds '{}' - run {}",
            "⚠".yellow(),
            name,
            recorded,
            "hagi install --refresh".yellow()
        ),
    }
}

/// Check project configuration status
fn check_project_configuration() -> Result<()> {
    println!("{}", "[Project Configuration]".cyan().bold());

    let claude_dir = utils::project_claude_dir()?;

    if !claude_dir.exists() {
        println!("{} {} - {}", "✗".red(), ".claude/".bold(), "not found".dimmed());
        println!("\nRun {} to install project configuration", "hagi install".yellow());
        return Ok(());
    }

    println!("{} {} - {}", "✓".green(), ".claude/".bold(), "installed".dimmed());

    // Check key template files
    for name in [templates::CLAUDE_MD, templates::MCP_JSON, templates::SETTINGS_LOCAL_JSON] {
        if claude_dir.join(name).exists() {
            println!("  {} .claude/{}", "✓".green(), name);
        } else {
            println!("  {} .claude/{} - {}", "✗".red(), name, "not found".dimmed());
        }
    }

    print_version_status(&claude_dir);

    // Check template directories
    for dir_name in ["instructions", "skills"] {
        let dir = claude_dir.join(dir_name);
        if dir.is_dir() {
            let count = std::fs::read_dir(&dir)
                .map(|entries| entries.filter_map(Result::ok).count())
                .unwrap_or(0);
            println!("  {} .claude/{}/ ({} files)", "✓".green(), dir_name, count);
        } else {
            println!("  {} .claude/{}/ - {}", "✗".red(), dir_name, "not found".dimmed());
        }
    }

    Ok(())
}

/// Show the project's MCP servers and whether each is enabled
fn check_mcp_servers() -> Result<()> {
    println!("{}", "[MCP Servers]".cyan().bold());

    let mcp_path = utils::project_claude_dir()?.join(templates::MCP_JSON);
    if !mcp_path.exists() {
        println!("{} No MCP configuration found", "✗".red());
        println!("\nRun {} to install MCP configuration", "hagi install".yellow());
        return Ok(());
    }

    let config = utils::read_json_file(&mcp_path)?;
    let Some(servers) = config.get("mcpServers").and_then(|v| v.as_object()) else {
        println!("{} .claude/{} has no mcpServers section", "✗".red(), templates::MCP_JSON);
        return Ok(());
    };

    for (name, server_config) in servers {
        let disabled = server_config
            .get("disabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let status = if disabled { "disabled".dimmed() } else { "enabled".green() };
        println!("  {} {} [{}]", "✓".dimmed(), name.cyan(), status);
    }

    Ok(())
}
