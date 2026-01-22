# Preferred Tools

## Text Search

**MUST use `rg` (ripgrep) instead of `grep`:**

❌ NEVER: `grep -r "pattern" .`
✅ ALWAYS: `rg "pattern"`

## File Display

**MUST use `bat -p` instead of `cat`:**

❌ NEVER: `cat file.txt`
✅ ALWAYS: `bat -p file.txt`

## File Search

**MUST use `fd` instead of `find`:**

❌ NEVER: `find . -name "*.js"`
✅ ALWAYS: `fd "\.js$"` or `fd -e js`

## Claude Code Tools

**Use specialized tools instead of bash:**

- ✅ Read tool (not `cat/head/tail`)
- ✅ Edit tool (not `sed/awk`)
- ✅ Write tool (not `echo >/cat <<EOF`)
- ✅ Grep tool (not bash `grep/rg`)
- ✅ Glob tool (not bash `find/ls`)

**NEVER use bash for:**
- File reading (`cat`, `head`, `tail`)
- File searching (`find`, `ls`)
- Text searching (`grep`, `rg`)
- File editing (`sed`, `awk`)
- File writing (`echo >`, `cat <<EOF`)
- Communication (`echo`, `printf`)

**Use bash ONLY for:**
- System commands
- Package managers
- Build tools
- Terminal-specific ops

## Web Search & Research

**One-search MCP:** DuckDuckGo web search
**Context7 MCP:** Library docs
**WebSearch/WebFetch:** Backup

## Memory

**Memory MCP:**
- Auto conversation memory
- Reference past conversations
- Usually no explicit ops needed

## File Operations (Temporary)

Chat mode: temp files only

- `/tmp/` writes OK
- Avoid editing project files
- Notes/drafts OK

## Summary

| Operation | ❌ NEVER | ✅ ALWAYS |
|-----------|----------|-----------|
| Text search | `grep` | `rg` |
| File display | `cat` | `bat -p` |
| File search | `find` | `fd` |
| File ops | Bash | Read/Edit/Write |
| Web search | curl | WebSearch/One-search |
