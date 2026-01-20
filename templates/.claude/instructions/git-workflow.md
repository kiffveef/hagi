# Git Operations

## Hook-Enforced Rules (automatic)

`hagi install` installs git hooks that enforce:

| Rule | Hook | Action |
|------|------|--------|
| No direct commits to main/master | pre-commit | Blocks commit |
| No .claude/ in git | pre-commit + PreToolUse | Blocks staging |
| No Claude signatures | commit-msg | Blocks commit |

Commits are blocked automatically. But **check branch BEFORE editing** (hooks can't prevent edits on wrong branch).

---

## Branch Workflow

### Before Editing Files

```bash
git branch --show-current
# If on main: git checkout -b <prefix>/descriptive-name
```

**Branch Prefixes:**
- `feature/` - New feature (include tests + docs)
- `fix/` - Bug fix (include verification tests)
- `refactor/` - Code refactoring
- `docs/` - Documentation-only changes
- `test/` - Test infrastructure only
- `config/` - Configuration changes

### During Development

```bash
git status && git diff
git add path/to/file1 path/to/file2
git commit -m "verb: brief description"
```

---

## Commit Message Format

**Line 1**: Icon + English summary (max 50 chars)
**Line 2**: Empty
**Line 3+**: Japanese bullet points (optional)

### Icon Prefixes

| Icon | Prefix | Use |
|------|--------|-----|
| 🌱 | first | Initial commit |
| ✨ | add | New feature |
| 🔄 | update | Update feature |
| 🐛 | fix | Bug fix |
| 📝 | docs | Documentation |
| ♻️ | refactor | Refactoring |
| 🧪 | test | Tests |
| 🔧 | config | Configuration |

### Example

```
✨ Add user authentication

- JWT トークンベースの認証を実装
- ログイン/ログアウトエンドポイントを追加
```

---

## .claude/ Directory

**.claude/ is outside git workflow.**

- Edit .claude/ files directly (no commit needed)
- Use `hagi sync` for cross-machine sync
- Never `git add .claude/` (blocked by hook)
