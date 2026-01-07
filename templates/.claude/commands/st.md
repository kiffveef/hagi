# /st - Structured Thinking

Combines Memory MCP and Sequential-Thinking MCP for deep analysis with context accumulation.

## Step 1: Context Retrieval (Memory)

**Project name:** Use current directory name (e.g., `hagi`, `my-app`)

**If Memory MCP enabled:**
1. Use `mcp__memory__retrieve_memory` to search project context
2. Query: `<project-name> <user's topic>` (e.g., "hagi authentication decision")
3. Pass retrieved context to Step 2

**If Memory MCP disabled:**
- Warn: "Memory MCP disabled. Run `hagi mcp enable memory` to accumulate context."
- Proceed to Step 2 without context

## Step 2: Deep Analysis (Sequential-Thinking)

1. Use `mcp__sequential-thinking__sequentialthinking`
2. Include Memory context in analysis
3. Structure problem, generate/verify hypotheses
4. Derive conclusions

**Always execute** - never skip even if past analysis exists.

## Step 3: Save Results (Memory)

**If Memory MCP enabled:**
1. Use `mcp__memory__store_memory` to save analysis
2. Content (max ~500 words): summary, decisions with reasons, rejected alternatives
3. Tags: `<dir-name>,st-analysis,<topic>,<YYYY-MM-DD>`
4. Type: `structured-analysis`

**If Memory MCP disabled:**
- Skip (warning already shown in Step 1)

## Options

### --search
Fetch external info for analysis:
- WebSearch for latest information
- Context7 for library documentation

### --todo
Integrate with task management:
- Read `.claude/TODO.md`
- Use TodoWrite to manage task status
- Update TODO.md after analysis
- **CRITICAL**: Always sync TODO.md after TodoWrite

## Error Handling

| Situation | Action |
|-----------|--------|
| Memory MCP disabled | Warn, run Sequential-Thinking only |
| No Memory results | Execute as new analysis |
| Memory save failed | Warn, output results to screen |
| Sequential-Thinking failed | Error, abort |
