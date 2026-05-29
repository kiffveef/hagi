# Git Operations

**Check branch BEFORE editing.** Direct commits to main/master are blocked by hook.

```bash
git branch --show-current  # if main: create branch first
```

## Commit Message

```
✨ Add user authentication

- JWT認証を実装
- ログイン/ログアウトエンドポイントを追加
```

| Icon | Use |
|------|-----|
| 🌱 | Initial commit |
| ✨ | New feature |
| 🔄 | Update feature |
| 🐛 | Bug fix |
| 📝 | Documentation |
| ♻️ | Refactoring |
| 🧪 | Tests |
| 🔧 | Configuration |
| 🗑️ | Remove |

## .claude/

Outside git. Edit directly. Use `hagi sync` to sync.
