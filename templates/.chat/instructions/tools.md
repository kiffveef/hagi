# Preferred Tools

## Text Search

**MUST use `rg` (ripgrep) instead of `grep`:**

❌ **NEVER:**
```bash
grep -r "pattern" .
```

✅ **ALWAYS:**
```bash
rg "pattern"
```

## File Display

**MUST use `bat -p` instead of `cat`:**

❌ **NEVER:**
```bash
cat file.txt
```

✅ **ALWAYS:**
```bash
bat -p file.txt
```

## File Search

**MUST use `fd` instead of `find`:**

❌ **NEVER:**
```bash
find . -name "*.js"
```

✅ **ALWAYS:**
```bash
fd "\.js$"
# or
fd -e js
```

## Claude Code Tools

**Use specialized tools instead of bash commands:**

- ✅ **Read tool** instead of `cat/head/tail`
- ✅ **Edit tool** instead of `sed/awk`
- ✅ **Write tool** instead of `echo >/cat <<EOF`
- ✅ **Grep tool** instead of `grep/rg` in bash
- ✅ **Glob tool** instead of `find/ls` in bash

**NEVER use bash for:**
- File reading (`cat`, `head`, `tail`)
- File searching (`find`, `ls`)
- Text searching (`grep`, `rg`)
- File editing (`sed`, `awk`)
- File writing (`echo >`, `cat <<EOF`)
- Communication (`echo`, `printf`)

**Use bash ONLY for:**
- Actual system commands
- Package managers (when needed)
- Build tools (when needed)
- Terminal-specific operations

## Web Search & Research

**One-search MCP:**
- DuckDuckGo検索
- Web検索が必要な時に使用

**Context7 MCP:**
- ライブラリドキュメント検索
- 技術文書が必要な時に使用

**WebSearch/WebFetch:**
- バックアップとして使用可能

## Memory

**Memory MCP:**
- 会話内容を自動記憶
- 過去の会話を参照可能
- 明示的な操作は通常不要

## File Operations (Temporary)

chatモードでは一時ファイル操作のみ:

- `/tmp/` 以下への書き込みはOK
- プロジェクトファイルの編集は避ける
- メモ・下書きなどの一時ファイル作成は可

## Summary

| Operation | ❌ NEVER Use | ✅ ALWAYS Use |
|-----------|--------------|---------------|
| Text search | `grep` | `rg` |
| File display | `cat` | `bat -p` |
| File search | `find` | `fd` |
| File operations | Bash commands | Read/Edit/Write tools |
| Web search | Direct curl | WebSearch/One-search MCP |
