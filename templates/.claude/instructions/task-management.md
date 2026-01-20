# Task Management

## TodoWrite ↔ TODO.md Sync

**Every TodoWrite call → IMMEDIATELY Edit .claude/TODO.md (same changes, same message)**

```
1. Read .claude/TODO.md (if exists)
2. TodoWrite → update status
3. Edit .claude/TODO.md → same changes
4. Repeat for every TodoWrite call
```

TodoWrite is session-only. TODO.md persists across sessions.

---

## Task States

| State | Meaning | Rule |
|-------|---------|------|
| pending | Not started | - |
| in_progress | Working now | **Only ONE at a time** |
| completed | Done | Mark immediately when finished |

**Required fields:**
- `content`: Imperative form ("Run tests")
- `activeForm`: Present continuous ("Running tests")

---

## When to Use TodoWrite

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

---

## Memento Save Trigger

When user says: "保存して", "save", "記録して", "record"

**Both actions required:**
1. Edit .claude/TODO.md
2. create_entities to Memento

```
create_entities({
  "entities": [{
    "name": "Task: <name>",
    "entityType": "task_completion",
    "observations": ["Status: completed", "Date: YYYY-MM-DD", "Project: <name>"]
  }]
})
```
