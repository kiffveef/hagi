Intelligent code analysis combining serena + memory - automatic pattern integration.

# /serena Command

Analyze code with serena while **automatically** checking past patterns from memory.
No need to run `/code-pattern` separately - this command does both.

## Usage

```
/serena <query> [--skip-memory] [--save-pattern]
```

## Arguments

- `<query>`: Code pattern or feature to search for (required)
- `--skip-memory`: Skip memory search, use serena only (faster)
- `--save-pattern`: Save analysis results to memory for future reference

## Examples

```
/serena "error handling in async functions"
/serena "REST API pagination implementation"
/serena "authentication middleware" --skip-memory
/serena "database connection pooling" --save-pattern
```

## Workflow (Automatic Integration)

**The entire workflow runs automatically - you just provide the query.**

### Step 0: Check Memory First (Automatic)

**Check for related past patterns in memory:**

1. **If memento MCP is available:**
   - Use `search_nodes` to find related patterns
   - Query: the user's query
   - Filter by entityType: `code_pattern`, `implementation`, `best_practice`
   - Limit: top 3 most relevant results

2. **If patterns found, display:**
   ```
   📚 Past Patterns Found:

   1. **[Project Name]** ([Date])
      - Pattern: [Brief description]
      - Use case: [Context]
      - Outcome: [Result]

   2. **[Project Name]** ([Date])
      - Pattern: [Brief description]
      - Use case: [Context]
      - Outcome: [Result]
   ```

3. **If no patterns found:**
   ```
   ℹ️ No past patterns found in memory. This will be the first record!
   ```

4. **If memento MCP not available:**
   ```
   ⚠️ Memory not available - searching current codebase only
   ```

**Important:** Skip this step if `--skip-memory` flag is provided.

---

### Step 1: Analyze Current Code (Serena)

**Search current codebase with serena:**

1. **If serena MCP is available:**
   - Use serena to find code matching the query
   - Look for:
     - Function/method implementations
     - Class definitions
     - Design patterns
     - Similar logic flows

2. **Display findings:**
   ```
   🔍 Current Codebase Analysis:

   **Found [N] instances:**

   1. **`path/to/file.rs:45-67`** - [Brief description]
      ```rust
      // Key code snippet (5-10 lines max)
      ```
      - Approach: [Implementation approach]
      - Notes: [Any notable aspects]

   2. **`path/to/other.rs:123-145`** - [Brief description]
      ```rust
      // Key code snippet
      ```
      - Approach: [Implementation approach]
      - Notes: [Any notable aspects]
   ```

3. **If serena MCP not available:**
   ```
   ❌ Serena not available. Please enable with:
      hagi mcp enable serena
   ```

**Important:** This is the ONLY step if `--skip-memory` is provided.

---

### Step 2: Integrated Recommendations

**Compare current code with past patterns (if available):**

1. **If both memory and serena results exist:**

   **Comparison:**
   ```
   💡 Integrated Analysis:

   ✅ **Good Patterns in Use:**
   - [Pattern from current code that matches past best practices]
   - [Another good pattern]

   ⚠️ **Differences from Past Best Practices:**
   - Current: [Approach in current code]
   - Past: [Approach in past project]
   - Suggestion: [Why past approach might be better]

   💡 **Suggested Improvements:**
   1. [Specific actionable improvement]
   2. [Another improvement]
   3. [Reference to past implementation that worked well]
   ```

2. **If only serena results (no memory):**

   **Analysis:**
   ```
   📊 Current Code Analysis:

   **Patterns Identified:**
   - [Pattern 1 in current code]
   - [Pattern 2 in current code]

   **Suggested Improvements:**
   - [Improvement based on general best practices]
   - [Another improvement]

   💾 Tip: Use `--save-pattern` to save this analysis for future projects!
   ```

3. **Provide actionable recommendations:**
   - Reference specific files and line numbers
   - Show concrete code examples
   - Explain "why" not just "what"

---

### Step 3: Optional Save to Memory

**If `--save-pattern` flag is provided:**

1. **Ask user for confirmation:**
   ```
   💾 Save this pattern to memory?

   This will save:
   - Query: "[user's query]"
   - Current implementation approaches
   - Recommended best practices
   - Date: [today's date]

   Use `create_entities` to save? (y/n)
   ```

2. **If user confirms, save with Memento entity structure:**
   ```json
   {
     "entities": [{
       "name": "[user's query]",
       "entityType": "code_pattern",
       "observations": [
         "Implementation: [summary of current approaches]",
         "Best practice: [recommended approach]",
         "Project: [current project name if detectable]",
         "Date: [YYYY-MM-DD]",
         "Tags: [relevant], [tags], [from], [query]"
       ]
     }]
   }
   ```

3. **Confirm save:**
   ```
   ✅ Pattern saved to memory for future reference!
   ```

**If flag not provided:**
```
ℹ️ To save this analysis for future projects, run:
   /serena "[query]" --save-pattern
```

## Output Format

Keep output **concise and actionable**:

- ✅ Show key findings only (not exhaustive lists)
- ✅ Limit code snippets to 5-10 lines
- ✅ Focus on differences and improvements
- ✅ Provide specific file:line references
- ❌ Don't repeat full code listings
- ❌ Don't show extensive comparison tables

**Example output structure:**
```
📚 Past Patterns: [Brief summary]
🔍 Current Code: [Key findings]
💡 Recommendations: [3-5 actionable items]
```

## Error Handling

**If serena is not available:**
- Display error message
- Suggest: `hagi mcp enable serena`
- Stop execution (serena is required)

**If memory is not available:**
- Display warning
- Continue with serena-only analysis
- Suggest enabling memory for future use

**If both are unavailable:**
- Display error
- Suggest enabling at least serena
- Stop execution

## Notes

- This command **replaces the need for manual `/code-pattern` execution**
- Memory check is automatic (unless `--skip-memory`)
- Use `--skip-memory` for faster results when you don't need historical context
- Use `--save-pattern` when you find a good implementation worth remembering
- Patterns accumulate over time, making this command more valuable with use

## Best Use Cases

1. **Before implementing a feature**: Check current codebase and past patterns
2. **During code review**: Validate against past best practices
3. **Refactoring**: Find better implementations from past projects
4. **Learning**: Understand how similar problems were solved before
5. **Consistency**: Maintain similar patterns across projects

## Comparison with `/code-pattern`

| Feature | `/serena` | `/code-pattern` |
|---------|-----------|-----------------|
| Automatic memory check | ✅ Yes | ❌ No (manual) |
| Primary focus | Current code + context | Pattern comparison |
| Output length | Concise | Detailed |
| Use case | Quick analysis | Deep pattern study |
| Recommendation | ✅ **Use this for most cases** | Use for in-depth analysis |

**In most cases, use `/serena` instead of `/code-pattern`.**
