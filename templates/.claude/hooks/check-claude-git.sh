#!/bin/bash
# Claude Code PreToolUse hook to prevent .claude/ git operations
# This hook blocks git add/commit operations that would affect .claude/
# Output uses hookSpecificOutput.permissionDecision (top-level decision/reason is deprecated for PreToolUse)

# jq dependency check
if ! command -v jq &> /dev/null; then
  echo "Warning: jq not found, .claude/ git protection disabled" >&2
  exit 0
fi

# Emit a deny decision with the given reason
deny() {
  cat << EOF
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "$1"
  }
}
EOF
}

input=$(cat)
command=$(echo "$input" | jq -r '.tool_input.command // ""')

# Inspect only the arguments of each `git add`, up to the next separator or line end,
# so text elsewhere in a multi-line command (e.g. a commit message) cannot trigger a false positive
nl=$'\n'
add_re="git[[:space:]]+add([^${nl}&;|]*)(.*)"
rest=$command
while [[ "$rest" =~ $add_re ]]; do
  args=${BASH_REMATCH[1]}
  rest=${BASH_REMATCH[2]}

  # Block git add .claude/ (a path like templates/.claude/ is fine)
  if [[ "$args" =~ [[:space:]](\./)?\.claude(/|[[:space:]]|$) ]]; then
    deny '.claude/ is outside git workflow. Edit = done. No git operation needed.\n\n📖 See: .claude/instructions/git-workflow.md'
    exit 0
  fi

  # Block git add . / -A / --all only if .claude/ is not gitignored
  if [[ "$args" =~ ^[[:space:]]+(\.|-A|--all)[[:space:]]*$ ]]; then
    if [ -d ".claude" ] && ! git check-ignore -q .claude/ 2>/dev/null; then
      deny '.claude/ is not gitignored. git add . would include it.\n\nAdd /.claude/ to .gitignore first, or run: hagi install'
      exit 0
    fi
  fi
done

# Block git commit if .claude/ is staged
if [[ "$command" =~ git[[:space:]]+commit ]]; then
  if git diff --cached --name-only 2>/dev/null | grep -q "^\.claude/"; then
    deny '.claude/ files are staged. Unstage first: git restore --staged .claude/\n\n.claude/ is outside git workflow.'
    exit 0
  fi
fi

exit 0
