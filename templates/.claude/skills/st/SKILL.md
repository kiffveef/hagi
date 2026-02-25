---
name: st
description: Use for complex problems requiring step-by-step analysis. Auto-selects WebSearch/Context7 for research, TaskCreate for multi-step tasks. Checks memory for similar patterns.
memory: project
---

# Structured Thinking

Use Claude's built-in thinking to break down complex problems systematically. Integrates with memory to reuse past problem-solving patterns.

## Usage

```
/st <problem> [--save] [--fresh]
```

## Arguments

- `<problem>`: Problem or task to analyze (required)
- `--save`: Save thinking pattern to memory after completion
- `--fresh`: Skip memory check, start fresh analysis

## Process

0. **Pattern Check** - Search memory for similar problems (unless `--fresh`)
1. **Analyze** - Identify what needs to be solved
2. **Break down** - Divide into logical steps
3. **Plan** - Determine sequence of actions
4. **Execute** - Work through each step
5. **Verify** - Check results
6. **Save** - Optionally save pattern to memory

## Step 0: Pattern Check (Automatic)

**FIRST**, check if similar problems were solved before.

**Instructions:**

1. **IF memento is available AND `--fresh` not provided:**
   - Use `search_nodes` with the problem as query
   - Filter by entityType: `thinking_pattern`
   - Limit: top 3 results

2. **IF patterns found:**
   - Display:
     ```
     Found similar problem-solving patterns:

     1. **<pattern name>** (<date>)
        - Problem: <original problem>
        - Approach: <approach taken>
        - Outcome: <result>
     ```
   - Ask: "Apply this approach or think fresh?"
   - **Wait for response:**
     - "apply/use/yes" → Use cached approach, adapt to current problem
     - "fresh/new/no" → Continue to Step 1

3. **IF no patterns found:**
   - Display: "No similar patterns found. Starting fresh analysis..."
   - Continue to Step 1

4. **IF memento not available:**
   - Skip memory check silently
   - Continue to Step 1

5. **IF `--fresh` flag provided:**
   - Skip memory check
   - Display: "Fresh mode: Skipping pattern check"
   - Continue to Step 1

## Auto Tool Selection

**Research needed** (external info, docs, best practices):
- WebSearch for web information
- Context7 MCP for library documentation

**Multi-step tasks**:
- TaskCreate for progress tracking
- Sync with `.claude/TODO.md` if exists (see @instructions/task-management.md)

**Design documentation needed**:
- Suggest `/design` for capturing design decisions

## Step 6: Save to Memory (--save only)

**Save thinking pattern for future reuse.**

**Trigger:** `--save` flag provided

**IF `--save` not provided:** Skip this step.

**Entity Format (Memento):**

```json
{
  "entities": [{
    "name": "<problem summary - max 50 chars>",
    "entityType": "thinking_pattern",
    "observations": [
      "Problem: <original problem description>",
      "Approach: <key approach/methodology used>",
      "Solution: <outcome/answer summary>",
      "Steps: <number of steps taken>",
      "Tools: <tools used - WebSearch/Context7/TaskCreate/etc>",
      "Project: <current project name>",
      "Date: YYYY-MM-DD"
    ]
  }]
}
```

**Output after save:**
```
Thinking pattern saved to memory.
Next time: `/st <similar problem>` will find this automatically.
```

## Integration with Designs

**IF the problem involves design decisions:**

1. **Check `.claude/designs/`** for related design documents
2. **Suggest `/design`** if decisions should be formally documented:
   ```
   This involves design decisions. Consider documenting with:
     /design "<topic>"
   ```
3. **Reference existing designs** in the analysis if relevant

## Examples

```
/st "How to implement authentication for this API"
/st "Debug failing test in user module" --fresh
/st "Design database schema for multi-tenant app" --save
```

## Output Format

Keep output structured and actionable:

```
## Problem Analysis

<Understanding of the problem>

## Approach

1. <Step 1>
2. <Step 2>
...

## Execution

<Work through each step>

## Result

<Outcome and verification>

---
Pattern saved: No | Yes (for future: `/st <similar>`)
```

## Notes

- Memory integration reduces redundant analysis for similar problems
- Use `--fresh` when you want to approach a problem differently
- Use `--save` for patterns worth reusing across projects
- For formal design documentation, use `/design` skill
