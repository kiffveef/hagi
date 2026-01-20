use anyhow::{Context, Result};
use colored::*;
use std::io::{self, Write};
use std::process::Command;

/// Update hagi to the latest version
pub fn update() -> Result<()> {
    println!("{}", "Updating hagi...".green().bold());
    println!();

    // Show current version
    println!("{} {}", "Current version:".green(), env!("CARGO_PKG_VERSION"));
    println!();

    // Confirm before updating
    print!("{}", "Do you want to update hagi to the latest version? [Y/n]: ".yellow());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim().to_lowercase();
    if !input.is_empty() && input != "y" && input != "yes" {
        println!("{}", "Update cancelled.".yellow());
        return Ok(());
    }

    println!();
    println!("{}", "Fetching latest version from GitHub...".green());

    // Run cargo install
    let status = Command::new("cargo")
        .args([
            "install",
            "--git",
            "https://github.com/kiffveef/hagi",
            "hagi",
            "--force",
        ])
        .status()
        .context("Failed to run cargo install")?;

    println!();

    if status.success() {
        println!("{}", "✅ hagi updated successfully!".green().bold());
        println!();
        println!("{}", "New version:".green());

        // Show new version
        let output = Command::new("hagi")
            .arg("--version")
            .output()
            .context("Failed to get version")?;

        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("  {}", version.trim());
        }
    } else {
        println!("{}", "❌ Update failed.".red().bold());
        println!();
        println!("{}", "Troubleshooting:".yellow());
        println!("  1. Check your internet connection");
        println!("  2. Verify cargo is installed: cargo --version");
        println!("  3. Try manual installation:");
        println!("     cargo install --git https://github.com/kiffveef/hagi hagi --force");
    }

    Ok(())
}
