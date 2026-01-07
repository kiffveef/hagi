use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{templates, utils};

// ============================================================================
// Common Helpers
// ============================================================================

/// Print dry run mode header if enabled
fn print_dry_run_header(dry_run: bool) {
    if dry_run {
        println!("{}", "[DRY RUN MODE]".yellow().bold());
    }
}

/// Print dry run mode footer if enabled
fn print_dry_run_footer(dry_run: bool) {
    if dry_run {
        println!("{}", "\nDry run completed. No files were modified.".yellow());
    }
}

/// Create directory with dry_run support
fn ensure_directory(path: &Path, dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{} {}", "Would create:".yellow(), path.display());
    } else {
        utils::ensure_dir(path)?;
    }
    Ok(())
}

// ============================================================================
// Global Install
// ============================================================================

/// Install global configuration to ~/.claude/
pub fn install_global(dry_run: bool) -> Result<()> {
    print_dry_run_header(dry_run);
    println!("{}", "Installing global configuration...".green());

    print_dependency_warnings();

    let claude_dir = utils::claude_dir()?;
    ensure_directory(&claude_dir, dry_run)?;

    install_mcp_config(&claude_dir, dry_run)?;
    install_settings(&claude_dir, dry_run)?;

    if dry_run {
        print_dry_run_footer(dry_run);
    } else {
        println!("{}", "\n✅ Global configuration installed successfully!".green().bold());
        println!("\nNext steps:");
        println!("  1. Restart Claude Code to load the new configuration");
        println!("  2. Run 'hagi install' in your project directory");
    }

    Ok(())
}

/// Check and print dependency warnings
fn print_dependency_warnings() {
    let warnings = check_dependencies();
    if !warnings.is_empty() {
        println!("\n{}", "⚠ Warning: Missing dependencies".yellow().bold());
        for warning in &warnings {
            println!("\n{}", warning);
        }
        println!();
    }
}

// ============================================================================
// Project Install
// ============================================================================

/// Install project-specific configuration to .claude/
pub fn install_project(dry_run: bool, skip_paths: &[String]) -> Result<()> {
    print_dry_run_header(dry_run);
    println!("{}", "Installing project configuration...".green());

    // Step 1: Ensure git repository exists
    ensure_git_repository(dry_run)?;

    // Step 2: Print skip list info
    print_skip_list(skip_paths);

    // Step 3: Setup directories
    let project_dir = env::current_dir().context("Failed to get current directory")?;
    let claude_dir = project_dir.join(".claude");
    ensure_directory(&claude_dir, dry_run)?;

    // Step 4: Copy templates
    templates::copy_all_templates_with_skip(&claude_dir, dry_run, skip_paths)?;

    // Step 5: Setup Claude Code hooks
    setup_claude_hooks(&claude_dir, dry_run)?;

    // Step 6: Update project configuration
    update_project_gitignore(&project_dir, dry_run)?;
    install_git_hooks(&project_dir, dry_run)?;

    // Step 7: Print completion message
    print_project_completion(dry_run)?;

    Ok(())
}

/// Ensure current directory is a git repository, initialize if not
fn ensure_git_repository(dry_run: bool) -> Result<()> {
    if !is_git_repository() {
        println!("\n{}", "⚠ Not a git repository. Initializing git...".yellow());
        initialize_git_repository(dry_run)?;
        println!();
    }
    Ok(())
}

/// Print skip list if not empty
fn print_skip_list(skip_paths: &[String]) {
    if !skip_paths.is_empty() {
        println!("\n{}", "Skipping the following paths:".yellow());
        for path in skip_paths {
            println!("  - {}", path);
        }
        println!();
    }
}

/// Print project installation completion message
fn print_project_completion(dry_run: bool) -> Result<()> {
    if dry_run {
        print_dry_run_footer(dry_run);
    } else {
        println!("{}", "\n✅ Project configuration installed successfully!".green().bold());
        println!("\nNext steps:");
        println!("  1. Review .claude/CLAUDE.md for project guidelines");
        println!("  2. Customize .claude/instructions/ as needed");
        println!("  3. Enable additional MCP servers with 'hagi mcp enable <name>'");
        print_claude_sync_notice()?;
    }
    Ok(())
}


// ============================================================================
// Chat Install
// ============================================================================

/// Install chat mode configuration to ~/.chat/
pub fn install_chat(dry_run: bool) -> Result<()> {
    print_dry_run_header(dry_run);
    println!("{}", "Installing chat mode configuration...".green());

    let home_dir = utils::home_dir()?;
    let chat_dir = home_dir.join(".chat");
    ensure_directory(&chat_dir, dry_run)?;

    templates::copy_chat_templates(&chat_dir, dry_run)?;

    if dry_run {
        print_dry_run_footer(dry_run);
    } else {
        println!("{}", "\n✅ Chat mode configuration installed successfully!".green().bold());
        println!("\nUsage:");
        println!("  cd ~/.chat && claude");
        println!("\nCustomize:");
        println!("  Edit ~/.chat/CLAUDE.md to personalize your chat experience");
    }

    Ok(())
}

// ============================================================================
// Template Installation Helpers
// ============================================================================

/// Maximum characters to show in error messages for template preview
const TEMPLATE_PREVIEW_LENGTH: usize = 200;

/// Install JSON configuration from embedded template
///
/// # Arguments
/// * `claude_dir` - Target directory for the config file
/// * `template_name` - Name of the embedded template file
/// * `target_name` - Name of the output file (can differ from template_name)
/// * `expand_env` - Whether to expand environment variables in the template
/// * `dry_run` - If true, only print what would be done
fn install_json_template(
    claude_dir: &PathBuf,
    template_name: &str,
    target_name: &str,
    expand_env: bool,
    dry_run: bool,
) -> Result<()> {
    let target = claude_dir.join(target_name);
    let template_str = templates::get_template(template_name)
        .with_context(|| format!("Failed to get embedded {} template", template_name))?;

    let mut template_content: serde_json::Value = serde_json::from_str(template_str)
        .with_context(|| {
            let preview = if template_str.len() > TEMPLATE_PREVIEW_LENGTH {
                &template_str[..TEMPLATE_PREVIEW_LENGTH]
            } else {
                template_str
            };
            format!(
                "Failed to parse {} template. Template may be corrupted.\nFirst {} chars: {}",
                template_name, TEMPLATE_PREVIEW_LENGTH, preview
            )
        })?;

    if expand_env {
        utils::expand_json_env_vars(&mut template_content).with_context(|| {
            format!(
                "Failed to expand environment variables in {} template.\n\
                 This may be due to missing HOME or other required environment variables.",
                template_name
            )
        })?;
    }

    if dry_run {
        let action = if target.exists() { "Would merge into" } else { "Would create" };
        println!("{} {}", action.yellow(), target.display());

        let env_note = if expand_env { " (with environment variables expanded)" } else { "" };
        if template_name == target_name {
            println!("  Template: embedded {}{}", template_name, env_note);
        } else {
            println!("  Template: embedded {} → {}{}", template_name, target_name, env_note);
        }
    } else {
        utils::merge_json_file(&target, &template_content)
            .with_context(|| format!("Failed to install {} to {}", target_name, target.display()))?;
    }

    Ok(())
}

/// Install MCP configuration from embedded template
fn install_mcp_config(claude_dir: &PathBuf, dry_run: bool) -> Result<()> {
    install_json_template(claude_dir, "mcp.json", "mcp.json", true, dry_run)
}

/// Install settings configuration from embedded template (rename settings.local.json → settings.json)
fn install_settings(claude_dir: &PathBuf, dry_run: bool) -> Result<()> {
    install_json_template(claude_dir, "settings.local.json", "settings.json", false, dry_run)
}

// ============================================================================
// Git & Project Configuration Helpers
// ============================================================================

/// Update .gitignore in project root
fn update_project_gitignore(project_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let entries = vec![
        "/.claude/",
        "/.serena/",
    ];

    if dry_run {
        println!("{} {}", "Would update:".yellow(), project_dir.join(".gitignore").display());
        for entry in &entries {
            println!("  {} {}", "Would add:".yellow(), entry);
        }
    } else {
        utils::update_gitignore(project_dir, &entries)?;
    }

    Ok(())
}

/// Check if current directory is a git repository
fn is_git_repository() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Initialize git repository in current directory
fn initialize_git_repository(dry_run: bool) -> Result<()> {
    if dry_run {
        println!("{}", "  Would run: git init".yellow());
        println!("{}", "  Would run: git commit --allow-empty -m \"🌱 init\"".yellow());
        return Ok(());
    }

    // Initialize git repository
    let status = Command::new("git")
        .arg("init")
        .status()
        .context("Failed to run git init")?;

    if !status.success() {
        anyhow::bail!("git init failed");
    }

    println!("{}", "  ✅ Git repository initialized".green());

    // Create empty initial commit
    let commit_status = Command::new("git")
        .args(["commit", "--allow-empty", "-m", "🌱 init"])
        .status()
        .context("Failed to run git commit")?;

    if !commit_status.success() {
        anyhow::bail!("git commit failed");
    }

    println!("{}", "  ✅ Initial commit created".green());
    Ok(())
}

// ============================================================================
// Dependency Checking
// ============================================================================

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check dependencies and show warnings
fn check_dependencies() -> Vec<String> {
    let mut warnings = Vec::new();

    // Node.js check
    if !command_exists("node") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -\n    sudo apt-get install -y nodejs",
            "Node.js not found".red(),
            "Required for: context7, sequential-thinking, one-search",
            "Install"
        ));
    }

    // uv check
    if !command_exists("uv") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    curl -LsSf https://astral.sh/uv/install.sh | sh",
            "uv not found".red(),
            "Required for: memento MCP",
            "Install"
        ));
    }

    // Python3 check
    if !command_exists("python3") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    sudo apt-get update\n    sudo apt-get install -y python3 python3-pip",
            "Python3 not found".red(),
            "Required for: memento MCP setup",
            "Install"
        ));
    }

    // Git check
    if !command_exists("git") {
        warnings.push(format!(
            "{}\n  {}\n\n  {}:\n    sudo apt-get install -y git",
            "Git not found".red(),
            "Required for: memento MCP",
            "Install"
        ));
    }

    warnings
}

/// Make a file executable (Unix only)
#[cfg(unix)]
fn make_executable(path: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

/// Write a git hook file and make it executable
fn install_hook(hooks_dir: &std::path::Path, name: &str, content: &str) -> Result<()> {
    let hook_path = hooks_dir.join(name);
    std::fs::write(&hook_path, content)
        .with_context(|| format!("Failed to write {} hook", name))?;

    #[cfg(unix)]
    make_executable(&hook_path)
        .with_context(|| format!("Failed to make {} executable", name))?;

    Ok(())
}


/// Make Claude Code hook script executable and print summary
fn setup_claude_hooks(claude_dir: &PathBuf, dry_run: bool) -> Result<()> {
    println!("\n{}", "Claude Code hooks...".green());

    let hook_script = claude_dir.join("hooks").join("check-claude-git.sh");

    if dry_run {
        println!("{} PreToolUse hook for .claude/ git protection", "Would configure:".yellow());
    } else {
        #[cfg(unix)]
        if hook_script.exists() {
            make_executable(&hook_script)
                .with_context(|| "Failed to make Claude hook script executable")?;
        }

        println!("  ✅ {}", "Claude Code hooks configured".green());
        println!("     {}", "- PreToolUse: Blocks .claude/ git operations".dimmed());
    }

    Ok(())
}

/// Install git hooks to .git/hooks/
fn install_git_hooks(project_dir: &PathBuf, dry_run: bool) -> Result<()> {
    let git_hooks_dir = project_dir.join(".git").join("hooks");

    if !git_hooks_dir.exists() {
        let msg = if dry_run { "would skip" } else { "skipping" };
        println!("\n{} .git/hooks directory not found, {} git hooks installation", "⚠".yellow(), msg);
        return Ok(());
    }

    println!("\n{}", "Installing git hooks...".green());

    let hooks = [
        ("pre-commit", include_str!("../../templates/git-hooks/pre-commit")),
        ("commit-msg", include_str!("../../templates/git-hooks/commit-msg")),
    ];

    if dry_run {
        for (name, _) in &hooks {
            println!("{} {}", "Would install:".yellow(), git_hooks_dir.join(name).display());
        }
    } else {
        for (name, content) in &hooks {
            install_hook(&git_hooks_dir, name, content)?;
        }

        println!("  ✅ {}", "Git hooks installed".green());
        println!("     {}", "- pre-commit: Prevents direct commits to main/master".dimmed());
        println!("     {}", "- commit-msg: Blocks Claude Code signatures".dimmed());
    }

    Ok(())
}

/// Print .claude sync notice for multi-machine workflow
fn print_claude_sync_notice() -> Result<()> {
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!("{}", "  📦 Multiple Machine Setup".cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!();
    println!("If you work on multiple machines, you can sync your .claude/ directory");
    println!("across machines using a private Git repository.");
    println!();

    let repo_name = get_repository_name();
    let claude_repo_name = format!("{}-claude", repo_name);

    if command_exists("gh") {
        println!("{}", "✨ GitHub CLI detected - Auto setup available:".green().bold());
        println!();
        println!("  {}", "hagi sync init".yellow());
        println!("  → Creates private repository: {}", claude_repo_name.cyan());
        println!("  → Initializes .claude/ as Git repository");
        println!("  → Pushes to GitHub automatically");
        println!();
    } else {
        println!("{}", "📋 Manual setup required:".yellow().bold());
        println!();
        println!("  1. Create private repository on GitHub:");
        println!("     {}", format!("https://github.com/new?name={}&visibility=private", claude_repo_name).cyan());
        println!();
        println!("  2. Initialize sync:");
        println!("     {}", format!("hagi sync init git@github.com:<username>/{}.git", claude_repo_name).yellow());
        println!();
        println!("{}", "💡 Tip: Install GitHub CLI for automatic setup:".dimmed());
        println!("     {}", "https://cli.github.com/".cyan().dimmed());
        println!();
    }

    println!("{}", "Daily workflow:".bold());
    println!("  {} - Pull latest changes from other machines", "hagi sync pull".yellow());
    println!("  {} - Push your changes to sync", "hagi sync push".yellow());
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());

    Ok(())
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
