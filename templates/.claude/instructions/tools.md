# Preferred Tools

**IMPORTANT: 以下のツールは必ず置き換えて使用すること。古いツールは使用禁止。**

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

**Reasons:**
- Faster performance
- Respects .gitignore by default
- Better color output
- More intuitive syntax

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

**Reasons:**
- `-p` option for plain text output
- Line numbers included
- Better readability
- Syntax highlighting available

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

**Reasons:**
- Much faster
- Simpler syntax
- Respects .gitignore by default
- Excludes hidden files by default

## Package Execution (Node.js)

**Use `npx` when global installation is not needed:**

✅ **Preferred:**
```bash
npx eslint .
npx prettier --write .
```

**Reasons:**
- Always runs latest version
- No global installation needed
- Avoids version conflicts

## Package Management (Python)

**MUST use `uv` instead of `pip`:**

❌ **NEVER:**
```bash
pip install package
```

✅ **ALWAYS:**
```bash
uv pip install package
```

**Reasons:**
- Faster installation
- Better dependency resolution
- Modern Python package management

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
- Package managers (`cargo`, `npm`, `uv`)
- Git operations
- Build tools
- Terminal-specific operations

## Summary

| Operation | ❌ NEVER Use | ✅ ALWAYS Use |
|-----------|--------------|---------------|
| Text search | `grep` | `rg` |
| File display | `cat` | `bat -p` |
| File search | `find` | `fd` |
| Python packages | `pip` | `uv` |
| File operations | Bash commands | Read/Edit/Write/Grep/Glob tools |
