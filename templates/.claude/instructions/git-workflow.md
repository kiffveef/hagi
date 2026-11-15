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

## Commit Granularity

One commit per logical change, not per round of feedback. This section overrides the default of committing only on request.

- Commit a logical change once it is verified (lint and tests pass, `reviewer` verdict relayed). Do not wait to be asked, and do not batch unrelated changes into one commit
- A later fix to a change that is committed but not pushed is folded into that commit, never added as a new one. Not pushed means `git branch -r --contains <sha>` prints nothing; if it prints a branch, make a new commit instead
- Fold into HEAD with `git commit --amend`; into an older commit with `git commit --fixup <sha>` then `git rebase --autosquash --autostash <sha>~1`
- If the fold conflicts, `git rebase --abort` and ask. Never push unless asked
- Report each commit or fold in one line (sha, subject)

## .claude/

Outside git. Edit directly. Use `hagi sync` to sync.
