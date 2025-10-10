# Git Operations

## ⚠️ Critical Rules - MUST Follow

**NEVER do the following:**
- ❌ **NEVER commit to `main` branch directly** - Always create a feature branch first
- ❌ **NEVER track `.claude/` directory in git** - Project root `.claude/` should never be tracked
- ❌ **NEVER start implementation without creating a feature branch** - Code changes require a branch
- ❌ **NEVER merge untested code to `main`** - Test before merging
- ❌ **NEVER use `git add .` blindly** - Review what you're staging
- ❌ **NEVER commit without checking current branch** - Verify branch before committing

**MUST do the following:**
- ✅ **MUST create a feature branch before any code changes** - Use `git checkout -b feature/xxx`
- ✅ **MUST check current branch before editing files** - Run `git branch --show-current`
- ✅ **MUST verify `.claude/` is in `.gitignore`** - Ensure setup files are ignored
- ✅ **MUST review changes before committing** - Use `git status` and `git diff`
- ✅ **MUST write meaningful commit messages** - Follow project conventions
- ✅ **MUST update documentation before merging to main** - Docs are part of implementation

## Branch Workflow

### 1. Before Starting Work

```bash
# Check current branch
git branch --show-current

# If on main, create feature branch
git checkout -b feature/descriptive-name
```

### 2. During Development

```bash
# Check what will be staged
git status
git diff

# Stage specific files (NOT git add .)
git add path/to/file1 path/to/file2

# Commit with meaningful message
git commit -m "verb: brief description"
```

### 3. Before Merging to Main

```bash
# Ensure documentation is updated
# Update README.md, docs/usage.md, etc. as needed

# Final check
git status
git log --oneline -3

# Switch to main and merge
git checkout main
git merge feature/descriptive-name
```

## Commit Message Format

**Line 1**: Icon + English summary (within 50 characters)
**Line 2**: Empty line
**Line 3+**: Japanese bullet points with brief work descriptions (if needed)

### Icon Prefixes

- 🌱 `first`: Initial commit
- ✨ `add`: Add new feature
- 🔄 `update`: Update existing feature
- 🐛 `fix`: Fix bug
- 🚧 `wip`: Work in progress
- 🐤 `tweak`: Small adjustment
- 📝 `docs`: Documentation
- 🎨 `style`: Code style/formatting
- ♻️ `refactor`: Refactor code
- 🧪 `test`: Add or update tests
- 🔧 `config`: Configuration changes
- 🗑️ `remove`: Remove files or code

### Example

```
✨ Add user authentication feature

- JWT トークンベースの認証を実装
- ログイン/ログアウトエンドポイントを追加
- Redis によるセッション管理を導入
```

## What NOT to Track in Git

**Project Root Files (NEVER track):**
- `/.claude/` - Setup-generated configuration
- `/mcp.json` - MCP configuration
- `/settings.json` - Claude settings
- `/settings.local.json` - Local settings
- `/.serena/` - Serena cache directory
- `.env*` - Environment variables

**Always Verify:**
```bash
# Check .gitignore includes these patterns
cat .gitignore | grep -E "\.claude|mcp.json|settings"
```

## Recovery from Mistakes

### If you accidentally committed to main:

```bash
# Create a branch from current state
git checkout -b feature/emergency-branch

# Reset main to previous state
git checkout main
git reset --hard origin/main

# Continue work on feature branch
git checkout feature/emergency-branch
```

### If you accidentally staged `.claude/`:

```bash
# Unstage the directory
git reset HEAD .claude/

# Verify .gitignore
echo "/.claude/" >> .gitignore
```

## Safety Checks

Before any git operation, ask yourself:
1. Am I on the correct branch?
2. Have I reviewed what I'm about to commit?
3. Is documentation updated?
4. Am I tracking only what should be tracked?
