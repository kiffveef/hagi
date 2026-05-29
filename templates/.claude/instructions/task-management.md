# Task Management

**On every TaskCreate/TaskUpdate: immediately update `.claude/TODO.md` with the same changes.**
Task API is session-only. TODO.md persists across sessions.

## Task States

| State | Rule |
|-------|------|
| pending | Not started |
| in_progress | Mark BEFORE starting. Multiple allowed for parallel work |
| completed | Mark immediately when done |

## When to Use
Use: 3+ steps, multi-step implementations, user requests todo list.
Skip: single tasks, trivial (<3 steps), conversational requests.
