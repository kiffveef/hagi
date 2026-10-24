use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

use crate::templates;
use crate::utils::{self, VersionCheck};

/// Backend of the SessionStart hook: emit a hook payload when .claude/ and this binary differ in minor version
///
/// Prints nothing when there is nothing to act on, so a missing or odd `hagi-version` never disturbs a session.
pub fn version_check() -> Result<()> {
    // Claude Code sets CLAUDE_PROJECT_DIR for hooks; fall back to cwd for manual runs
    let project_dir = match env::var_os("CLAUDE_PROJECT_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir().context("Failed to get current directory")?,
    };
    let version_file = project_dir
        .join(utils::CLAUDE_DIR_NAME)
        .join(templates::HAGI_VERSION_FILE);

    let Ok(recorded) = std::fs::read_to_string(&version_file) else {
        return Ok(());
    };
    let recorded = recorded.trim();
    let current = env!("CARGO_PKG_VERSION");

    let check = utils::check_version(recorded, current);
    let Some(fix) = check.fix() else {
        return Ok(());
    };
    let verb = match check {
        VersionCheck::TemplatesBehind => "追従",
        _ => "更新",
    };

    let payload = serde_json::json!({
        "systemMessage": format!(
            "⚠ .claude/ は hagi {recorded} で配置されましたが、現在の hagi は {current} です。{fix} で{verb}できます。"
        ),
        "hookSpecificOutput": {
            "hookEventName": "SessionStart",
            "additionalContext": format!(
                ".claude/ was set up by hagi {recorded} but hagi {current} is installed. Suggest running '{fix}' before other work."
            ),
        },
    });
    println!("{}", payload);
    Ok(())
}
