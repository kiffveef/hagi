#!/bin/bash
# Claude Code PreToolUse hook to prevent .claude/ git operations
# This hook blocks git add/commit operations that would affect .claude/

# jq dependency check
if ! command -v jq &> /dev/null; then
  echo "Warning: jq not found, .claude/ git protection disabled" >&2
  exit 0
fi

input=$(cat)
command=$(echo "$input" | jq -r '.tool_input.command // ""')

# Block git add .claude/ (but allow templates/.claude/)
# Extract just the git add portion (before && or ;)
add_part=$(echo "$command" | sed 's/&&.*//' | sed 's/;.*//')
if [[ "$add_part" =~ git[[:space:]]+add && "$add_part" =~ [[:space:]]\.claude ]]; then
  cat << 'EOF'
{
  "decision": "block",
  "reason": ".claude/ is outside git workflow. Edit = done. No git operation needed.\n\n📖 See: .claude/instructions/git-workflow.md"
}
EOF
  exit 0
fi

# Block git add . / git add -A / git add --all only if .claude/ is not gitignored
if [[ "$command" =~ git[[:space:]]+add[[:space:]]+(\.|-A|--all)[[:space:]]*$ ]]; then
  if [ -d ".claude" ] && ! git check-ignore -q .claude/ 2>/dev/null; then
    cat << 'EOF'
{
  "decision": "block",
  "reason": ".claude/ is not gitignored. git add . would include it.\n\nAdd /.claude/ to .gitignore first, or run: hagi install"
}
EOF
    exit 0
  fi
fi

# Block git commit if .claude/ is staged
if [[ "$command" =~ git[[:space:]]+commit ]]; then
  if git diff --cached --name-only 2>/dev/null | grep -q "^\.claude/"; then
    cat << 'EOF'
{
  "decision": "block",
  "reason": ".claude/ files are staged. Unstage first: git restore --staged .claude/\n\n.claude/ is outside git workflow."
}
EOF
    exit 0
  fi
fi

exit 0
