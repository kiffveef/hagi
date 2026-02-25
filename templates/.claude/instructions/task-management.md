# Task Management

## Task API ↔ TODO.md Sync

**Every TaskCreate/TaskUpdate call → IMMEDIATELY Edit .claude/TODO.md (same changes)**

```
1. Read .claude/TODO.md (if exists)
2. TaskCreate/TaskUpdate → manage tasks
3. Edit .claude/TODO.md → reflect same changes
```

Task API is session-only. TODO.md persists across sessions.

---

## Tools

| Tool | Purpose |
|------|---------|
| `TaskCreate` | Create new task (subject, description, activeForm) |
| `TaskUpdate` | Update status, set dependencies, delete |
| `TaskGet` | Get task details before updating |
| `TaskList` | List all tasks and find next work |

---

## Task States

| State | Meaning | Rule |
|-------|---------|------|
| pending | Not started | - |
| in_progress | Working now | Mark BEFORE starting work. **Multiple allowed** for parallel work |
| completed | Done | Mark immediately when finished |

**Required fields on create:**
- `subject`: Imperative form ("Run tests")
- `activeForm`: Present continuous ("Running tests")
- `description`: What needs to be done

---

## Dependencies

Use `addBlockedBy` and `addBlocks` to express task ordering:

```
TaskUpdate({ taskId: "2", addBlockedBy: ["1"] })
```

---

## When to Use Tasks

**Use for:**
- Tasks with 3+ steps
- Multi-step implementations
- User requests todo list

**Skip for:**
- Single straightforward tasks
- Trivial tasks (<3 steps)
- Conversational requests

---

## Completion Criteria

**Mark completed only when:**
- Task fully accomplished
- Tests pass
- No errors or blockers

**Keep in_progress if:**
- Tests failing
- Implementation partial
- Errors unresolved
