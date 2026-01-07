# /st - Structured Thinking

Combines Memento and Sequential-Thinking MCP for deep analysis with context accumulation.

## Step 1: Context Retrieval (Memento)

**Project name:** Use current directory name (e.g., `hagi`, `my-app`)

**If Memento enabled:**
1. Use `search_nodes` to search project context
2. Query: `<project-name> <user's topic>` (e.g., "hagi authentication decision")
3. Pass retrieved context to Step 2

**If Memento disabled:**
- Warn: "Memento disabled. Run `hagi mcp enable memory` to accumulate context."
- Proceed to Step 2 without context

## Step 2: Deep Analysis (Sequential-Thinking)

1. Use `mcp__sequential-thinking__sequentialthinking`
2. Include Memory context in analysis
3. Structure problem, generate/verify hypotheses
4. Derive conclusions

**Always execute** - never skip even if past analysis exists.

## Step 3: Save Results (Memento)

**If Memento enabled:**
1. Use `create_entities` to save analysis
2. Entity format:
   ```json
   {
     "entities": [{
       "name": "<topic>",
       "entityType": "st_analysis",
       "observations": [
         "Summary: <summary>",
         "Decision: <key decision with reason>",
         "Rejected: <rejected alternatives>",
         "Project: <dir-name>",
         "Date: <YYYY-MM-DD>"
       ]
     }]
   }
   ```

**If Memento disabled:**
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
| Memento disabled | Warn, run Sequential-Thinking only |
| No Memento results | Execute as new analysis |
| Memento save failed | Warn, output results to screen |
| Sequential-Thinking failed | Error, abort |
