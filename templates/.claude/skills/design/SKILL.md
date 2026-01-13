---
name: design
description: Document design decisions and specifications in .claude/designs/. Use when planning features, recording architectural decisions, or documenting implementation approaches.
---

# Design Documentation

Create and maintain design documents in `.claude/designs/` to preserve design intent, architectural decisions, and implementation specifications.

## Usage

```
/design <topic> [--memory]
```

## Arguments

- `<topic>`: Design topic or feature name (required)
- `--memory`: Also save to memory for cross-project reference

## Examples

```
/design "authentication flow"
/design "error handling strategy" --memory
```

## Workflow

### Step 1: Check Existing & Determine Mode

**Check if design document already exists.**

**Instructions:**

1. **Search for existing design:**
   - Use Glob: `.claude/designs/*<topic>*.md`
   - Match flexibly (kebab-case, partial match)

2. **IF design exists:**
   - Display: "Updating: .claude/designs/<filename>"
   - Read existing file for context
   - Mode: **UPDATE** (preserve structure, update content)

3. **IF no design exists:**
   - Display: "Creating: .claude/designs/<topic-kebab-case>.md"
   - Mode: **CREATE** (use template)

### Step 2: Gather Requirements

**Collect information for the design document.**

**Instructions:**

1. **Understand the context:**
   - What problem does this solve?
   - What are the constraints?
   - What alternatives were considered?

2. **Ask clarifying questions if needed:**
   - Use AskUserQuestion for ambiguous requirements
   - Confirm key decisions before documenting

3. **If memory is available, check for related patterns:**
   - Search memory for similar designs from other projects
   - Display relevant findings if any

### Step 3: Create/Update Design Document

**Write the design document to `.claude/designs/`.**

**File naming:**
- Convert topic to kebab-case: "authentication flow" → `authentication-flow.md`
- Use descriptive names: `mcp-defaults.md`, `error-handling.md`

**Document Template:**

```markdown
# <Topic>

**Status:** Draft | Approved | Implemented
**Created:** YYYY-MM-DD
**Updated:** YYYY-MM-DD

## Purpose

[What problem does this solve?]

## Background

[Context and motivation]

## Design

### Approach

[How will we solve it?]

### Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| ... | ... | ... |

### Alternatives Considered

[What other options were evaluated and why they were rejected]

## Implementation

### Components

[Key components or modules involved]

### Changes Required

[List of files/areas that need modification]

## Open Questions

[Unresolved items, if any]

## References

[Related documents, issues, or external resources]
```

**Instructions:**

1. **Create file path:** `.claude/designs/<topic-kebab-case>.md`
2. **Write document** using the template above
3. **Display confirmation:** "Design saved to .claude/designs/<filename>"

### Step 4: Memory Storage (Optional)

**IF `--memory` flag provided, save to memory for cross-project access.**

**Instructions:**

1. **Check if memento is available**
2. **IF available, save with:**

```json
{
  "entities": [{
    "name": "<topic>",
    "entityType": "design_decision",
    "observations": [
      "Purpose: <brief purpose>",
      "Approach: <key approach>",
      "Key decision: <most important decision>",
      "Project: <current project name>",
      "File: .claude/designs/<filename>",
      "Date: YYYY-MM-DD"
    ]
  }]
}
```

3. **Display confirmation:**
   ```
   Design also saved to memory for cross-project reference.
   Find later with: /st "<related problem>"
   ```

## Output Format

**After creating/updating design:**

```
Design Document: .claude/designs/<filename>

Summary:
- Purpose: <brief purpose>
- Approach: <key approach>
- Status: Draft

Next steps:
1. Review and refine the design
2. Get approval if needed
3. Start implementation

Tip: Use `--memory` to save for cross-project reference.
```

## When to Use This Skill

**Use /design for:**
- New feature planning before implementation
- Recording architectural decisions (ADRs)
- Documenting "why" behind complex implementations
- Capturing design intent that might be lost over time

**Don't use for:**
- Task tracking (use TODO.md instead)
- Code documentation (use inline comments/docstrings)
- API documentation (use dedicated tools)

## Integration with Other Skills

| Skill | Integration |
|-------|-------------|
| `/st` | Design decisions can be referenced during problem-solving |
| `/serena` | Code patterns can be documented as design patterns |

## Notes

- Design documents are stored in `.claude/designs/` (git-trackable)
- Use `--memory` for patterns you want to reuse across projects
- Keep designs concise but complete enough to understand intent later
- Update status as design progresses: Draft → Approved → Implemented
