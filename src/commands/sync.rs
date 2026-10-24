use anyhow::{bail, Context, Result};
use colored::*;
use std::io::Write;
use std::process::{Command, Output, Stdio};

use crate::templates;
use crate::utils;

// ============================================================================
// Git Command Helpers
// ============================================================================

/// Run git command in .claude directory, failing with "git <what> failed" on non-zero exit
fn git_in_claude_ok(args: &[&str], what: &str) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .current_dir(utils::CLAUDE_DIR_NAME)
        .status()
        .with_context(|| format!("Failed to run: git {}", args.join(" ")))?;

    if !status.success() {
        bail!("git {} failed", what);
    }

    Ok(())
}

/// Run git command in .claude directory and capture output
fn git_in_claude_output(args: &[&str]) -> Result<Output> {
    Command::new("git")
        .args(args)
        .current_dir(utils::CLAUDE_DIR_NAME)
        .output()
        .with_context(|| format!("Failed to run: git {}", args.join(" ")))
}

/// Clone the remote repository into the project's .claude directory
fn git_clone_claude(remote_url: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["clone", remote_url, utils::CLAUDE_DIR_NAME])
        .status()
        .context("Failed to run git clone")?;

    if !status.success() {
        bail!("git clone failed");
    }

    Ok(())
}

/// Print the pull/push commands used after sync is set up
pub fn print_daily_workflow() {
    println!("Daily workflow:");
    println!("  {} - Pull latest changes", "hagi sync pull".yellow());
    println!("  {} - Push your changes", "hagi sync push".yellow());
    println!();
}

/// Initialize .claude sync with a private Git repository
pub fn sync_init(remote_url: Option<&str>, message: Option<&str>) -> Result<()> {
    let claude_dir = utils::project_claude_dir()?;

    // If .claude/.git exists, already initialized
    if claude_dir.join(".git").exists() {
        bail!(
            ".claude is already a Git repository.\n\
             Use 'hagi sync pull/push' to sync changes."
        );
    }

    // If URL is provided, clone directly (no need for hagi install first)
    if let Some(url) = remote_url {
        return clone_claude_repo(url);
    }

    // For auto-detection, need .claude to exist
    if !claude_dir.exists() {
        bail!(
            ".claude directory not found.\n\
             Please run 'hagi install' first, or specify URL:\n\
             hagi sync init git@github.com:user/project-claude.git"
        );
    }

    // Check if gh CLI is available
    if !utils::command_exists("gh") {
        println!("{}", "gh CLI not found.".yellow());
        println!();
        println!("To clone existing repository:");
        println!("  {}", "hagi sync init git@github.com:user/project-claude.git".cyan());
        println!();
        println!("To install gh CLI:");
        println!("  {}", "https://cli.github.com/".cyan());
        println!();
        bail!("Please specify repository URL or install gh CLI");
    }

    // Check if <project>-claude repo already exists
    let claude_repo_name = utils::claude_repo_name();

    if check_repo_exists(&claude_repo_name) {
        println!("{}", format!("Found existing repository: {}", claude_repo_name).cyan());
        let username = get_github_username()?;
        let remote_url = format!("git@github.com:{}/{}.git", username, claude_repo_name);
        return clone_and_replace_claude(&remote_url);
    }

    // Create new repository
    create_private_repo_interactive().and_then(|remote| init_claude_git_repo(&remote, message))
}

/// Check if a repository exists on GitHub (caller ensures gh is installed)
fn check_repo_exists(repo_name: &str) -> bool {
    Command::new("gh")
        .args(["repo", "view", repo_name])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Clone existing repo and replace .claude directory
fn clone_and_replace_claude(remote_url: &str) -> Result<()> {
    let claude_dir = utils::project_claude_dir()?;

    println!("{}", "Cloning existing .claude repository...".green());

    // Backup current .claude to .claude.backup
    let backup_dir = claude_dir.with_file_name(format!("{}.backup", utils::CLAUDE_DIR_NAME));
    if backup_dir.exists() {
        std::fs::remove_dir_all(&backup_dir).context("Failed to remove old backup")?;
    }
    std::fs::rename(&claude_dir, &backup_dir).context("Failed to backup .claude")?;

    // Clone the repository
    if let Err(e) = git_clone_claude(remote_url) {
        // Restore backup on failure
        std::fs::rename(&backup_dir, &claude_dir).ok();
        return Err(e);
    }

    // An empty remote clones "successfully" but has no commits (unborn HEAD).
    // Deleting the backup here would destroy the local config, so restore it
    // and keep the cloned .git for the first push.
    if !claude_rev_exists("HEAD")? {
        std::fs::rename(claude_dir.join(".git"), backup_dir.join(".git"))
            .context("Failed to move cloned .git into backup")?;
        std::fs::remove_dir_all(&claude_dir).context("Failed to remove empty clone")?;
        std::fs::rename(&backup_dir, &claude_dir).context("Failed to restore .claude backup")?;

        println!("{}", "⚠ Remote repository is empty. Kept local .claude files.".yellow());
        println!();
        println!("Run {} to make the first push.", "hagi sync push".yellow());
        println!();
        return Ok(());
    }

    // Remove backup on success
    std::fs::remove_dir_all(&backup_dir).ok();

    println!("{}", "✅ Cloned .claude repository".green().bold());
    println!();
    print_daily_workflow();

    Ok(())
}

/// Pull latest changes from remote
pub fn sync_pull() -> Result<()> {
    ensure_claude_git_repo()?;

    println!("{}", "Pulling latest .claude changes...".green());

    let branch = current_claude_branch()?;
    git_in_claude_ok(&["pull", "origin", &branch, "--rebase"], "pull")?;

    println!("{}", "✅ Pulled latest .claude config".green().bold());

    Ok(())
}

/// Ensure .claude directory exists and is a git repository
fn ensure_claude_git_repo() -> Result<()> {
    let claude_dir = utils::project_claude_dir()?;

    if !claude_dir.exists() {
        bail!(".claude directory not found");
    }

    if !claude_dir.join(".git").exists() {
        bail!(
            ".claude is not a Git repository.\n\
             Run 'hagi sync init' first to initialize sync."
        );
    }

    Ok(())
}

/// Stage all files, force-adding ones the parent repo's .gitignore may exclude
fn stage_claude_files() -> Result<()> {
    git_in_claude_ok(&["add", "."], "add")?;

    // Add individually: one missing pathspec aborts a combined add entirely
    let _ = git_in_claude_output(&["add", "--force", "--", templates::CLAUDE_MD]);
    let _ = git_in_claude_output(&["add", "--force", "--", templates::TODO_MD]);

    Ok(())
}

/// Resolve the current branch name of the .claude repository
fn current_claude_branch() -> Result<String> {
    let output = git_in_claude_output(&["symbolic-ref", "--short", "HEAD"])?;
    if !output.status.success() {
        bail!("Failed to resolve current branch in .claude (detached HEAD?)");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check whether a revision exists in the .claude repository
fn claude_rev_exists(rev: &str) -> Result<bool> {
    let output = git_in_claude_output(&["rev-parse", "--verify", "--quiet", rev])?;
    Ok(output.status.success())
}

/// Push changes to remote
pub fn sync_push(message: Option<&str>) -> Result<()> {
    ensure_claude_git_repo()?;

    println!("{}", "Pushing .claude changes...".green());

    stage_claude_files()?;

    // Distinguish "nothing staged" from a real commit failure (missing user.name, hook, ...)
    let staged = git_in_claude_output(&["status", "--porcelain"])?;
    if staged.stdout.is_empty() {
        println!("{}", "⚠ Nothing to commit".yellow());
    } else {
        let commit_msg = message.unwrap_or("Update .claude config");
        git_in_claude_ok(&["commit", "-m", commit_msg], "commit")?;
    }

    // Unborn HEAD: fresh clone of an empty remote with no local files yet
    if !claude_rev_exists("HEAD")? {
        bail!("No commits in .claude yet. Add files and run 'hagi sync push' again.");
    }

    let branch = current_claude_branch()?;
    let remote_ref = format!("origin/{}", branch);

    // The remote branch is absent before the first push (e.g. cloned from an empty repo)
    if claude_rev_exists(&remote_ref)? {
        let range = format!("{}..HEAD", remote_ref);
        let output = git_in_claude_output(&["rev-list", "--count", &range])?;
        if !output.status.success() {
            bail!("git rev-list failed");
        }
        let ahead_count: u32 = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .context("Failed to parse commit count from git rev-list")?;

        if ahead_count == 0 {
            println!("{}", "✅ Already up to date with remote".green());
            return Ok(());
        }

        println!("{}", format!("Pushing {} commit(s)...", ahead_count).green());
        git_in_claude_ok(&["push"], "push")?;
    } else {
        println!("{}", format!("First push: setting upstream to {}...", remote_ref).green());
        git_in_claude_ok(&["push", "-u", "origin", &branch], "push")?;
    }

    println!("{}", "✅ Pushed .claude changes".green().bold());

    Ok(())
}

/// Show sync status
pub fn sync_status() -> Result<()> {
    let claude_dir = utils::project_claude_dir()?;

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

    git_in_claude_ok(&["status"], "status")
}

/// Create private repository interactively using gh CLI
fn create_private_repo_interactive() -> Result<String> {
    if !utils::command_exists("gh") {
        print_manual_setup_instructions();
        bail!("Please install gh CLI or specify remote URL manually");
    }

    let claude_repo_name = utils::claude_repo_name();

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

/// Initialize .claude as Git repository and push to remote (caller ensures .claude/.git is absent)
fn init_claude_git_repo(remote_url: &str, message: Option<&str>) -> Result<()> {
    let claude_dir = utils::project_claude_dir()?;

    println!();
    println!("{}", "Initializing .claude as Git repository...".green());

    git_in_claude_ok(&["init"], "init")?;

    println!("{}", "✅ Initialized Git repository".green());

    let gitignore_path = claude_dir.join(".gitignore");
    let gitignore_content = "# Backup files (hagi auto-generated)\n*.backup.*\n";
    std::fs::write(&gitignore_path, gitignore_content)
        .context("Failed to create .gitignore")?;

    println!("{}", "✅ Created .gitignore (excludes backup files)".green());

    git_in_claude_ok(&["remote", "add", "origin", remote_url], "remote add")?;

    println!("{}", format!("✅ Added remote: {}", remote_url).green());

    stage_claude_files()?;

    let commit_msg = message.unwrap_or("🌱 first: Initial .claude config");
    git_in_claude_ok(&["commit", "-m", commit_msg], "commit")?;

    println!("{}", "✅ Created initial commit".green());

    git_in_claude_ok(&["branch", "-M", "main"], "branch")?;
    git_in_claude_ok(&["push", "-u", "origin", "main"], "push")?;

    println!("{}", "✅ Pushed to remote".green().bold());
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!("{}", "  ✅ .claude sync initialized successfully!".cyan().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan());
    println!();
    println!("On other machines:");
    println!("  1. Clone your project repository");
    println!(
        "  2. Run: {}",
        format!("git clone {} {}", remote_url, utils::CLAUDE_DIR_NAME).yellow()
    );
    println!();
    print_daily_workflow();

    Ok(())
}

/// Print manual setup instructions
fn print_manual_setup_instructions() {
    let claude_repo_name = utils::claude_repo_name();

    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow());
    println!("{}", "  📋 Manual Setup Instructions".yellow().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".yellow());
    println!();
    println!("1. Create private repository on GitHub:");
    println!("   {}", "https://github.com/new".cyan());
    println!();
    println!("   Repository name: {}", claude_repo_name.cyan());
    println!("   Visibility: {}", "Private".green());
    println!();
    println!("2. Initialize sync:");
    println!(
        "   {}",
        format!("hagi sync init git@github.com:<username>/{}.git", claude_repo_name).yellow()
    );
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

/// Clone .claude repository from URL (handles both fresh clone and replace)
fn clone_claude_repo(remote_url: &str) -> Result<()> {
    // If .claude exists, use backup/replace logic
    if utils::project_claude_dir()?.exists() {
        return clone_and_replace_claude(remote_url);
    }

    // Fresh clone - .claude doesn't exist
    println!("{}", "Cloning .claude repository...".green());

    git_clone_claude(remote_url)?;

    println!("{}", "✅ Cloned .claude repository".green().bold());
    println!();
    print_daily_workflow();

    Ok(())
}
