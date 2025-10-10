# Task Management

## TodoWrite Tool Usage

**When to use TodoWrite tool:**
- Complex tasks with 3 or more distinct steps
- Multi-step implementation tasks
- User explicitly requests todo list
- User provides multiple tasks

**When NOT to use:**
- Single, straightforward tasks
- Trivial tasks completable in <3 steps
- Purely conversational requests

## Task States

Use these states to track progress:
- `pending`: Task not yet started
- `in_progress`: Currently working (ONE task at a time)
- `completed`: Task finished successfully

## Task Management Rules

**MUST follow:**
- ✅ **MUST provide both `content` and `activeForm` for each task**
  - `content`: Imperative form ("Run tests")
  - `activeForm`: Present continuous form ("Running tests")
- ✅ **MUST update task status in real-time as you work**
- ✅ **MUST mark tasks completed IMMEDIATELY after finishing**
- ✅ **MUST have exactly ONE task in_progress at any time**
- ✅ **MUST complete current task before starting new one**

**NEVER do:**
- ❌ NEVER batch multiple completions
- ❌ NEVER leave tasks in_progress when done
- ❌ NEVER have multiple tasks in_progress simultaneously

## .claude/TODO.md Integration

If `.claude/TODO.md` exists in the project:
- Read it first before creating TodoWrite tasks
- Synchronize TodoWrite with TODO.md content
- Keep both in sync during task execution

## Example Usage

```
User: "Add authentication feature with tests"

Assistant creates todos:
[
  {"content": "Design authentication system", "status": "in_progress", "activeForm": "Designing authentication"},
  {"content": "Implement authentication logic", "status": "pending", "activeForm": "Implementing authentication"},
  {"content": "Write tests for authentication", "status": "pending", "activeForm": "Writing tests"},
  {"content": "Update documentation", "status": "pending", "activeForm": "Updating documentation"}
]

After completing design:
- Mark "Design authentication system" as completed
- Mark "Implement authentication logic" as in_progress
```

## Task Completion Criteria

**ONLY mark task as completed when:**
- ✅ Task is fully accomplished
- ✅ All tests pass
- ✅ Implementation is complete
- ✅ No errors or blockers

**Keep as in_progress if:**
- ❌ Tests are failing
- ❌ Implementation is partial
- ❌ Unresolved errors exist
- ❌ Necessary files/dependencies not found
