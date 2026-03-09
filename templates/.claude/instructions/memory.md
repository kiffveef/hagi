# Memory

## Memory MCP

When memory MCP is available, proactively record learnings and discoveries during sessions.

### What to Record

- **Patterns**: Problem-solving approaches, implementation patterns, debugging techniques
- **Config/Environment**: Build commands, test execution, environment-specific settings
- **Decisions**: Architecture decisions, technology selection rationale
- **Preferences**: User workflow, code style preferences

### When to Record

- After solving a problem
- When discovering new patterns or techniques
- When learning user preferences or habits
- When making design decisions
- When gaining important debugging insights

### How to Record

Use `create_entities`:

```json
{
  "entities": [{
    "name": "<summary under 50 chars>",
    "entityType": "<type>",
    "observations": [
      "<key>: <value>",
      "Project: <project name>",
      "Date: YYYY-MM-DD"
    ]
  }]
}
```

**entityType list:**

| Type | Use |
|------|-----|
| `thinking_pattern` | Problem-solving approaches |
| `code_pattern` | Implementation patterns, best practices |
| `design_decision` | Architecture/design decisions |
| `debug_insight` | Debugging insights |
| `user_preference` | User preferences/habits |
| `project_config` | Project-specific settings/commands |

### Search

**Priority**: Search memory FIRST, before codebase search, Context7, or WebSearch.
**During skill execution** (`/st`, `/serena`, `/design`): Follow the skill's own search instructions instead.

**Always search at session start:** `project_config` (environments change frequently)

Use `search_nodes` with the user's query. Filter by entityType based on context:

| Trigger | entityType filter |
|---------|-------------------|
| Debugging or investigating errors | `thinking_pattern`, `debug_insight` |
| Implementing a new feature or fix | `code_pattern`, `design_decision` |
| User asks about past decisions or conventions | `user_preference`, `design_decision` |

**Do NOT search for:**

- Mechanical edits (rename, formatting, typo fix)
- Tasks where the user provides complete instructions
- Follow-up actions within the same task
