#!/bin/bash
# Claude Code SessionStart hook: hagi compares .claude/hagi-version with its own version
# Stays silent without hagi; the binary is kept current, so the comparison lives in Rust, not here
command -v hagi > /dev/null 2>&1 || exit 0
hagi version-check 2> /dev/null
exit 0
