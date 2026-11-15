# Task Management

**Track tasks in `.claude/TODO.md` only. Edit it directly as work progresses.**
TODO.md persists across sessions via `hagi sync`, independent of built-in task tools.

## Task States

Each state is a TODO.md section. Move items between sections as the state changes.

| State | Section | Rule |
|-------|---------|------|
| pending | Pending | Not started |
| in_progress | In Progress | Move BEFORE starting. Multiple allowed for parallel work |
| completed | Completed | Move immediately when done. Prune at release (history lives in git) |

Each In Progress item notes its branch and remaining steps, so a fresh session can continue without the previous transcript.

## When to Use
Use: 3+ steps, multi-step implementations, user requests todo list.
Skip: single tasks, trivial (<3 steps), conversational requests.

## Format

TODO.md is a task list, not a log. It is imported into every turn, so keep it short.

- One line per task. An In Progress item adds its branch and, when needed, a link to its notes file
- Remaining steps and open decisions are unchecked sub-items (`- [ ]`). Delete a sub-item when it is done
- Never write what git already answers (committed, merged, pushed, files changed) or what happened so far
- Decisions, rejected options, measurements and agent reports go to `.claude/notes/<topic>.md` (not imported). Record a decision or rejection there when the user makes it, so it survives compaction
- Completed items are one line each

```
- (中) Add export command — `feat/export`, notes: `.claude/notes/export.md`
  - [ ] CSV escaping for multi-line cells
  - [ ] Decide: default delimiter
```
