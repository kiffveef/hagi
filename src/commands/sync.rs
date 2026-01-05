use anyhow::{bail, Context, Result};
use colored::*;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Initialize .claude sync with a private Git repository
pub fn sync_init(remote_url: Option<&str>) -> Result<()> {
    let claude_dir = Path::new(".claude");

    if !claude_dir.exists() {
        bail!(
            ".claude directory not found.\n\
             Please run 'hagi install' first to create .claude/ directory."
        );
    }

    let remote = if let Some(url) = remote_url {
        url.to_string()
    } else {
        create_private_repo_interactive()?
    };

    init_claude_git_repo(&remote)?;

    Ok(())
}

/// Pull latest changes from remote
pub fn sync_pull() -> Result<()> {
    let claude_dir = Path::new(".claude");

    if !claude_dir.exists() {
        bail!(".claude directory not found");
    }

    if !claude_dir.join(".git").exists() {
        bail!(
            ".claude is not a Git repository.\n\
             Run 'hagi sync init' first to initialize sync."
        );
    }

    println!("{}", "Pulling latest .claude changes...".green());

    let status = Command::new("git")
        .args(["pull", "--rebase"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git pull")?;

    if !status.success() {
        bail!("git pull failed");
    }

    println!("{}", "✅ Pulled latest .claude config".green().bold());

    Ok(())
}

/// Push changes to remote
pub fn sync_push(message: Option<&str>) -> Result<()> {
    let claude_dir = Path::new(".claude");

    if !claude_dir.exists() {
        bail!(".claude directory not found");
    }

    if !claude_dir.join(".git").exists() {
        bail!(
            ".claude is not a Git repository.\n\
             Run 'hagi sync init' first to initialize sync."
        );
    }

    println!("{}", "Pushing .claude changes...".green());

    let status = Command::new("git")
        .args(["add", "."])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git add")?;

    if !status.success() {
        bail!("git add failed");
    }

    let commit_msg = message.unwrap_or("Update .claude config");
    let status = Command::new("git")
        .args(["commit", "-m", commit_msg])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git commit")?;

    if !status.success() {
        println!("{}", "⚠ Nothing to commit".yellow());
        return Ok(());
    }

    let status = Command::new("git")
        .args(["push"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git push")?;

    if !status.success() {
        bail!("git push failed");
    }

    println!("{}", "✅ Pushed .claude changes".green().bold());

    Ok(())
}

/// Show sync status
pub fn sync_status() -> Result<()> {
    let claude_dir = Path::new(".claude");

    if !claude_dir.exists() {
        bail!(".claude directory not found");
    }

    if !claude_dir.join(".git").exists() {
        println!("{}", "❌ .claude is not a Git repository".red());
        println!();
        println!("Run 'hagi sync init' to initialize sync.");
        return Ok(());
    }

    println!("{}", "📊 .claude sync status:".cyan().bold());
    println!();

    let status = Command::new("git")
        .args(["status"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git status")?;

    if !status.success() {
        bail!("git status failed");
    }

    Ok(())
}

/// Create private repository interactively using gh CLI
fn create_private_repo_interactive() -> Result<String> {
    if !command_exists("gh") {
        print_manual_setup_instructions();
        bail!("Please install gh CLI or specify remote URL manually");
    }

    let repo_name = get_repository_name();
    let claude_repo_name = format!("{}-claude", repo_name);

    println!();
    println!("{}", "📦 Creating private repository".cyan().bold());
    println!();
    println!("  Repository name: {}", claude_repo_name.yellow());
    println!("  Visibility: {}", "Private".green());
    println!();
    print!("Proceed? [Y/n]: ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "n" {
        print_manual_setup_instructions();
        bail!("Cancelled");
    }

    println!();
    println!("{}", "Creating repository on GitHub...".green());

    let output = Command::new("gh")
        .args(["repo", "create", &claude_repo_name, "--private"])
        .output()
        .context("Failed to run gh repo create")?;

    if !output.status.success() {
        eprintln!("{}", "❌ Failed to create repository".red());
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        print_manual_setup_instructions();
        bail!("gh repo create failed");
    }

    let username = get_github_username()?;
    let remote_url = format!("git@github.com:{}/{}.git", username, claude_repo_name);

    println!("{}", format!("✅ Created private repository: {}", claude_repo_name).green());

    Ok(remote_url)
}

/// Initialize .claude as Git repository and push to remote
fn init_claude_git_repo(remote_url: &str) -> Result<()> {
    let claude_dir = Path::new(".claude");

    if claude_dir.join(".git").exists() {
        bail!(
            ".claude is already a Git repository.\n\
             Use 'hagi sync pull/push' to sync changes."
        );
    }

    println!();
    println!("{}", "Initializing .claude as Git repository...".green());

    let status = Command::new("git")
        .args(["init"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git init")?;

    if !status.success() {
        bail!("git init failed");
    }

    println!("{}", "✅ Initialized Git repository".green());

    let gitignore_path = claude_dir.join(".gitignore");
    let gitignore_content = "# Backup files (hagi auto-generated)\n*.backup.*\n";
    std::fs::write(&gitignore_path, gitignore_content)
        .context("Failed to create .gitignore")?;

    println!("{}", "✅ Created .gitignore (excludes backup files)".green());

    let status = Command::new("git")
        .args(["remote", "add", "origin", remote_url])
        .current_dir(claude_dir)
        .status()
        .context("Failed to add remote")?;

    if !status.success() {
        bail!("git remote add failed");
    }

    println!("{}", format!("✅ Added remote: {}", remote_url).green());

    let status = Command::new("git")
        .args(["add", "."])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git add")?;

    if !status.success() {
        bail!("git add failed");
    }

    let status = Command::new("git")
        .args(["commit", "-m", "🌱 first: Initial .claude config"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git commit")?;

    if !status.success() {
        bail!("git commit failed");
    }

    println!("{}", "✅ Created initial commit".green());

    let status = Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to rename branch")?;

    if !status.success() {
        bail!("git branch failed");
    }

    let status = Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .current_dir(claude_dir)
        .status()
        .context("Failed to run git push")?;

    if !status.success() {
        bail!("git push failed");
    }

    println!("{}", "✅ Pushed to remote".green().bold());
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!("{}", "  ✅ .claude sync initialized successfully!".cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!();
    println!("On other machines:");
    println!("  1. Clone your project repository");
    println!("  2. Run: {}", "hagi sync pull".yellow());
    println!();
    println!("Daily workflow:");
    println!("  {} - Pull latest changes", "hagi sync pull".yellow());
    println!("  {} - Push your changes", "hagi sync push".yellow());
    println!();

    Ok(())
}

/// Print manual setup instructions
fn print_manual_setup_instructions() {
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow());
    println!("{}", "  📋 Manual Setup Instructions".yellow().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow());
    println!();
    println!("1. Create private repository on GitHub:");
    println!("   {}", "https://github.com/new".cyan());
    println!();
    println!("   Repository name: {}", format!("{}-claude", get_repository_name()).cyan());
    println!("   Visibility: {}", "Private".green());
    println!();
    println!("2. Initialize sync:");
    println!("   {}", format!("hagi sync init git@github.com:<username>/{}-claude.git", get_repository_name()).yellow());
    println!();
    println!("💡 Tip: Install GitHub CLI for automatic setup:");
    println!("   {}", "https://cli.github.com/".cyan());
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow());
    println!();
}

/// Get GitHub username via gh CLI
fn get_github_username() -> Result<String> {
    let output = Command::new("gh")
        .args(["api", "user", "--jq", ".login"])
        .output()
        .context("Failed to get GitHub username")?;

    if !output.status.success() {
        bail!("Failed to get GitHub username via gh CLI");
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// Get repository name from git remote or current directory
fn get_repository_name() -> String {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout);
            if let Some(name) = url.split('/').last() {
                return name.trim_end_matches(".git\n").trim().to_string();
            }
        }
    }

    env::current_dir()
        .ok()
        .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "myproject".to_string())
}

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
