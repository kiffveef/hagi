# Task Management

## 🚨 CRITICAL: TodoWrite ↔ TODO.md Bidirectional Sync

**THE MOST IMPORTANT RULE FOR /st --todo:**

```bash
# EVERY TIME you use TodoWrite tool:
# 1. Use TodoWrite to update task status
# 2. IMMEDIATELY write changes to .claude/TODO.md using Edit tool
# 3. This is BIDIRECTIONAL - both MUST stay synchronized
```

**WHY THIS MATTERS:**
- TodoWrite is session-only memory - lost when session ends
- `.claude/TODO.md` is persistent storage across sessions
- Missing sync = lost progress tracking
- **This rule is violated frequently**

**MANDATORY WORKFLOW:**
```
1. Read .claude/TODO.md (if exists)
2. Use TodoWrite tool to update status
3. **IMMEDIATELY Edit .claude/TODO.md** with same changes
4. Repeat for EVERY TodoWrite call
```

**Example of CORRECT behavior:**
```
# User: /st --todo "Fix authentication bug"

# STEP 1: Read existing TODO.md
[Read .claude/TODO.md]

# STEP 2: Update TodoWrite
[TodoWrite: mark task as in_progress]

# STEP 3: IMMEDIATELY update TODO.md
[Edit .claude/TODO.md: mark same task as in_progress]

# STEP 4: Do work...

# STEP 5: Update TodoWrite when done
[TodoWrite: mark task as completed]

# STEP 6: IMMEDIATELY update TODO.md
[Edit .claude/TODO.md: mark same task as completed]
```

---

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

**CRITICAL REQUIREMENT - NOT OPTIONAL:**

If `.claude/TODO.md` exists in the project, you MUST:

1. ✅ **MUST read TODO.md FIRST** before any TodoWrite operation
2. ✅ **MUST synchronize TodoWrite → TODO.md** after EVERY TodoWrite call
3. ✅ **MUST use Edit tool immediately** - never delay TODO.md updates
4. ✅ **MUST keep both in perfect sync** throughout task execution

**NEVER do:**
- ❌ NEVER use TodoWrite without reading TODO.md first
- ❌ NEVER skip TODO.md update after TodoWrite
- ❌ NEVER batch multiple TodoWrite updates before syncing to TODO.md
- ❌ NEVER assume TODO.md will update automatically

**This is BIDIRECTIONAL synchronization:**
- TodoWrite → TODO.md (after every TodoWrite call)
- TODO.md → TodoWrite (at start of /st --todo session)

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

---

## 🚨 CRITICAL: Memory MCP Auto-Save Rule

**THE RULE THAT PREVENTS DATA LOSS:**

When user says any of these keywords, **ALWAYS save to memory MCP**:
- "保存して" / "save"
- "記録して" / "record"
- "todoを保存" / "save todo"
- "進捗を保存" / "save progress"

**MANDATORY ACTIONS:**

```bash
# When user says "todoを保存して":
1. Edit .claude/TODO.md (update file)
2. IMMEDIATELY call mcp__memory__store_memory (save to memory)
3. BOTH are required, NOT optional
```

**WHY THIS MATTERS:**
- User expects memory MCP storage when saying "保存"
- File edits alone are NOT sufficient
- Memory enables cross-session context retrieval
- `/research` and `/serena` rely on memory data

**WHAT TO SAVE:**

When saving TODO updates to memory:
- Task completion status (completed/partial/pending)
- Implementation details (what was done)
- File changes (which files were modified)
- Commit hashes (for traceability)
- Next steps (remaining work)

**TAGS TO USE:**
```
tags: "project-name,task-name,phase,status"
type: "task-completion" | "task-status" | "project-status"
```

**Example:**
```
User: "todoを保存して"

# STEP 1: Edit TODO.md
[Edit .claude/TODO.md: mark Task 24 as completed]

# STEP 2: IMMEDIATELY save to memory
[mcp__memory__store_memory:
  content: "Task 24: dependency check - completed (2025-10-17)..."
  tags: "hagi,task24,dependency-check,phase4,completed"
  type: "task-completion"
]

# BOTH steps are required. Never skip memory save.
```

**NEVER do:**
- ❌ NEVER edit TODO.md without memory save
- ❌ NEVER assume "保存" means file-only
- ❌ NEVER delay memory save for later
- ❌ NEVER forget to save when user says "保存して"

**ALWAYS do:**
- ✅ ALWAYS save to memory MCP when user says "保存"
- ✅ ALWAYS use both Edit tool AND memory tool
- ✅ ALWAYS tag appropriately (project, task, phase, status)
- ✅ ALWAYS confirm both actions completed
