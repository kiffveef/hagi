# Git Operations

## 🚨 CRITICAL: Read This BEFORE Every File Edit

**THE MOST IMPORTANT RULE - BROKEN REPEATEDLY:**

```bash
# STEP 1: CHECK BRANCH **BEFORE** EDITING ANY FILE
git branch --show-current

# STEP 2: IF ON main, CREATE BRANCH **IMMEDIATELY**
git checkout -b <prefix>/descriptive-name

# STEP 3: ONLY THEN edit files
```

**WHY THIS MATTERS:**
- Editing files on `main` breaks the workflow
- Creates merge conflicts
- Wastes time fixing mistakes
- **This rule is violated in EVERY session**

---

## ⚠️ Critical Rules - MUST Follow

**NEVER do the following:**
- ❌ **NEVER edit files without checking branch first** - **CHECK BRANCH BEFORE EVERY Edit/Write**
- ❌ **NEVER commit to `main` branch directly** - Always create a feature branch first
- ❌ **NEVER start implementation without creating a feature branch** - Code changes require a branch
- ❌ **NEVER merge untested code to `main`** - Test before merging
- ❌ **NEVER use `git add .` blindly** - Review what you're staging
- ❌ **NEVER commit build artifacts or secrets** - Verify .gitignore is properly configured

**MUST do the following (IN THIS ORDER):**
1. ✅ **MUST check current branch FIRST** - `git branch --show-current` **BEFORE ANY Edit/Write**
2. ✅ **MUST create branch if on main** - `git checkout -b <prefix>/xxx` **BEFORE editing**
3. ✅ **MUST then edit files** - Only after steps 1-2
4. ✅ **MUST verify ignored files are in `.gitignore`** - Ensure build artifacts, secrets, etc. are ignored
5. ✅ **MUST review changes before committing** - Use `git status` and `git diff`
6. ✅ **MUST write meaningful commit messages** - Follow project conventions
7. ✅ **MUST update documentation before merging to main** - Docs are part of implementation

## Branch Workflow

### 1. Before Starting Work (MANDATORY FIRST STEP)

**DO THIS BEFORE TOUCHING ANY FILE:**

```bash
# MANDATORY: Check current branch
git branch --show-current

# If on main, CREATE BRANCH IMMEDIATELY
git checkout -b <prefix>/descriptive-name
```

**REMEMBER: This is step 1, not step 2 or 3. FIRST.**

**Branch Naming Convention:**
- `feature/xxx` - New feature development
- `fix/xxx` - Bug fixes
- `refactor/xxx` - Code refactoring
- `docs/xxx` - Documentation-only updates (typos, improvements, new guides)
- `test/xxx` - Test infrastructure improvements (NOT for tests of new features)
- `config/xxx` - Configuration changes
- `perf/xxx` - Performance improvements

**Important:**
- When implementing a new feature in `feature/xxx`, include its tests AND documentation in the same branch
- When fixing a bug in `fix/xxx`, include verification tests AND documentation updates in the same branch
- `docs/xxx` is ONLY for: documentation-only changes (typo fixes, clarifications, new documentation without code changes)
- `test/xxx` is ONLY for: adding tests to existing features, improving test infrastructure, increasing test coverage

**Examples:**
```bash
git checkout -b feature/add-authentication  # includes code + tests + docs
git checkout -b fix/login-error  # includes fix + verification tests + doc updates
git checkout -b docs/fix-readme-typo  # documentation typo fix only
git checkout -b test/improve-test-coverage  # adding tests to existing features
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

**🚨 CRITICAL: NO Claude Code Signature**
- ❌ **NEVER include Claude Code signature** (`🤖 Generated with [Claude Code]...`)
- ❌ **NEVER include Co-Authored-By: Claude** line
- ✅ **Keep commit messages clean and professional**

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

### 🚨 CRITICAL: .claude/ Directory is OUTSIDE Git Workflow

**`.claude/` is outside git workflow. Edit and done. No commit needed.**

**Workflow:**
```
Edit .claude/ → Done (git not involved)
Need sync? → hagi sync
```

**Prohibited:**
- ❌ NEVER `git add .claude/`
- ❌ NEVER `git add --force` on `.claude/`
- ❌ NEVER try to commit `.claude/` files

**Common mistake:**
Edit .claude/ → try to commit → gitignore error → confusion
→ Correct: Don't try to commit. Edit = immediately effective.

**Why:**
- Local config generated by `hagi install`
- Per-developer settings
- Managed separately via `hagi sync`

### Common files to exclude

- Build artifacts and compiled code
- Dependency directories (node_modules/, target/, etc.)
- Environment-specific configuration files
- IDE-specific settings
- Temporary files and caches
- Files containing secrets or credentials
- **`.claude/` directory** (Claude Code local configuration)

**Always Verify:**
```bash
# Check .gitignore before committing
cat .gitignore

# Review what will be committed
git status
git diff --cached
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

### If you accidentally staged files:

```bash
# Unstage specific files
git reset HEAD path/to/file

# Unstage all files
git reset HEAD .

# Verify what's staged
git status
```

## Safety Checks

Before any git operation, ask yourself:
1. Am I on the correct branch?
2. Have I reviewed what I'm about to commit?
3. Is documentation updated?
4. Am I tracking only what should be tracked?
