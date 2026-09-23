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

Each In Progress item notes its branch and next step, so a fresh session can continue without the previous transcript.

## When to Use
Use: 3+ steps, multi-step implementations, user requests todo list.
Skip: single tasks, trivial (<3 steps), conversational requests.
