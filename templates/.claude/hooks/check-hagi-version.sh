#!/bin/bash
# Claude Code SessionStart hook: warn when .claude/ was applied by a different hagi minor version
# Patch differences are ignored; hagi bumps minor for Claude Code follow-ups and CLI changes.
# Every early exit is 0 so a broken environment never blocks a session.

project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
version_file="$project_dir/.claude/hagi-version"

[ -f "$version_file" ] || exit 0
command -v hagi > /dev/null 2>&1 || exit 0

# "X.Y" or "X.Y.<patch or prerelease>"; anything else is unknown and must not reach the JSON below
version_re='^[0-9]+\.[0-9]+(\.[0-9A-Za-z.+-]*)?$'

read -r recorded < "$version_file"
recorded=${recorded%$'\r'}
[[ "$recorded" =~ $version_re ]] || exit 0

# clap prints "hagi X.Y.Z"; take the first version-looking token so future suffixes don't break parsing
current=$(hagi --version 2>/dev/null)
[[ "$current" =~ [0-9]+\.[0-9]+(\.[0-9A-Za-z.+-]*)? ]] || exit 0
current=${BASH_REMATCH[0]}

IFS=. read -r rec_major rec_minor _ <<< "$recorded"
IFS=. read -r cur_major cur_minor _ <<< "$current"
# 10# keeps a zero-padded component from being read as octal
rec_major=$((10#$rec_major)); rec_minor=$((10#$rec_minor))
cur_major=$((10#$cur_major)); cur_minor=$((10#$cur_minor))

if (( cur_major > rec_major || (cur_major == rec_major && cur_minor > rec_minor) )); then
  fix="hagi install --refresh"
  verb="追従"
elif (( cur_major < rec_major || (cur_major == rec_major && cur_minor < rec_minor) )); then
  fix="hagi update"
  verb="更新"
else
  exit 0
fi

cat << EOF
{
  "systemMessage": "⚠ .claude/ は hagi $recorded で配置されましたが、現在の hagi は $current です。$fix で${verb}できます。",
  "hookSpecificOutput": {
    "hookEventName": "SessionStart",
    "additionalContext": ".claude/ was set up by hagi $recorded but hagi $current is installed. Suggest running '$fix' before other work."
  }
}
EOF
