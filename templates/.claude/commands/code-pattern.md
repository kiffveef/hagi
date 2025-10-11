Search past coding patterns using serena + mcp-memory-service integration.

# /code-pattern Command

Find similar code patterns from past projects and current codebase by combining:
1. **serena**: Semantic search in current codebase
2. **mcp-memory-service**: Search past patterns from long-term memory
3. **Pattern matching**: Identify similarities and suggest best implementations

## Usage

```
/code-pattern <description> [--current-only] [--memory-only]
```

## Arguments

- `<description>`: Description of the pattern to search (required)
- `--current-only`: Search only in current codebase (serena only)
- `--memory-only`: Search only in past patterns (memory only)

## Examples

```
/code-pattern error handling in async functions
/code-pattern "REST API pagination implementation"
/code-pattern authentication middleware --current-only
/code-pattern "database connection pooling" --memory-only
```

## Workflow

### Step 1: Current Codebase Search (serena)

Search for relevant patterns in the current project.

**Instructions for Claude:**

1. **Check if serena is available:**
   - Look for serena MCP in available tools
   - If not available AND `--current-only` flag: display error and stop
   - If not available: skip to Step 2

2. **Search current codebase:**
   - Use serena to find code matching the description
   - Look for:
     - Function/method implementations
     - Class definitions
     - Design patterns
     - Similar logic flows

3. **Analyze findings:**
   - Identify common patterns
   - Note implementation approaches
   - Extract reusable code snippets

**Example serena search:**
- Query: "error handling in async functions"
- Results: List of async functions with error handling in current project

### Step 2: Past Pattern Search (mcp-memory-service)

Search for similar patterns in long-term memory.

**Instructions for Claude:**

1. **Check if mcp-memory-service is available:**
   - Look for memory tools in available MCPs
   - If not available AND `--memory-only` flag: display error and stop
   - If not available: skip to Step 3

2. **Search memory:**
   - Use `search_memories` with the pattern description
   - Search for:
     - `type: "code_pattern"`
     - `type: "implementation"`
     - `type: "best_practice"`
   - Filter by relevant tags

3. **Retrieve pattern details:**
   - Get implementation examples
   - Get best practices notes
   - Get lessons learned

**Memory search query:**
```
search_memories(
  query="error handling in async functions",
  tags=["error-handling", "async", "pattern"]
)
```

### Step 3: Pattern Analysis & Comparison

Compare and analyze patterns from both sources.

**Compare:**
- Current implementation (serena) vs. Past implementations (memory)
- Identify strengths and weaknesses of each approach
- Find common patterns across projects

**Provide:**
- Side-by-side comparison of implementations
- Best practices from past projects
- Suggested improvements for current code
- Code examples from both sources

### Step 4: Recommendations

Provide actionable recommendations based on pattern analysis.

**Include:**
1. **Best approach**: Recommended implementation pattern
2. **Current code improvements**: Specific refactoring suggestions
3. **Reusable components**: Code that can be extracted/reused
4. **Anti-patterns to avoid**: Common mistakes found in past projects
5. **Next steps**: Concrete actions to improve code quality

## Output Format

Present the pattern analysis in the following structure:

```markdown
## {Pattern Description} - Pattern Analysis

### Current Project Patterns (serena)

**Found {N} instances:**

1. **`path/to/file.rs:45-67`** - Error handling with Result<T, E>
   ```rust
   // Code snippet
   ```
   - Strengths: Type-safe, explicit
   - Weaknesses: Verbose error conversion

2. **`path/to/other.rs:123-145`** - Error handling with anyhow
   ```rust
   // Code snippet
   ```
   - Strengths: Concise, context propagation
   - Weaknesses: Less type information

### Past Project Patterns (mcp-memory-service)

**Found {M} patterns in memory:**

1. **Project: web-api-v2** (2024-08-15)
   - Pattern: Custom error types with thiserror
   - Use case: REST API error responses
   - Outcome: Improved error messages for clients
   ```rust
   // Code snippet from memory
   ```

2. **Project: data-processor** (2024-07-20)
   - Pattern: Error context with backtrace
   - Use case: Debugging complex async workflows
   - Outcome: Faster debugging in production
   ```rust
   // Code snippet from memory
   ```

### Pattern Comparison

| Aspect | Current (serena) | Past (memory) | Recommendation |
|--------|------------------|---------------|----------------|
| Type safety | High (Result) | High (thiserror) | ✅ Use thiserror |
| Ergonomics | Medium (anyhow) | High (custom types) | ⚠️ Consider custom types |
| Debugging | Basic | Advanced (backtrace) | ✅ Add backtrace |

### Recommendations

#### 1. Best Approach
Use custom error types with `thiserror` for public APIs, `anyhow` for internal code.

**Example implementation:**
```rust
// Recommended pattern combining best of both
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

#### 2. Current Code Improvements

**Refactor `src/handlers.rs:45-67`:**
```rust
// Before (current code)
async fn handler() -> Result<(), Box<dyn Error>> {
    // ...
}

// After (recommended)
async fn handler() -> Result<(), ApiError> {
    // ...
}
```

**Benefits:**
- Better error messages for API clients
- Type-safe error handling
- Easier debugging with context

#### 3. Reusable Components

Extract error handling utilities:
- `src/error.rs` - Custom error types
- `src/middleware/error_handler.rs` - Error response formatter

#### 4. Anti-patterns to Avoid

❌ **Don't:**
- Use `unwrap()` in async code
- Ignore error context
- Return generic `Box<dyn Error>` from public APIs

✅ **Do:**
- Use `?` operator with custom error types
- Add context with `.context()` or `.with_context()`
- Define specific error variants for different failure modes

#### 5. Next Steps

1. Create `src/error.rs` with custom error types
2. Refactor `src/handlers.rs` to use new types
3. Add error handling middleware
4. Update tests to verify error cases
5. Save this pattern to memory for future projects

### Save to Memory

**Save this pattern analysis:**
```json
{
  "type": "code_pattern",
  "topic": "async error handling in Rust",
  "best_practice": "Use thiserror for public APIs, anyhow for internal code",
  "project": "current-project",
  "date": "2025-10-11",
  "tags": ["rust", "async", "error-handling", "pattern", "best-practice"]
}
```

✅ Pattern analysis saved to long-term memory for future reference.
```

## Error Handling

**If serena is not available:**
- Display: "⚠️ serena MCP not available, searching memory only"
- Continue with Step 2 only

**If mcp-memory-service is not available:**
- Display: "⚠️ Memory not available, searching current codebase only"
- Continue with Step 1 only

**If both are unavailable:**
- Display: "❌ Both serena and memory MCPs are unavailable"
- Suggest enabling at least one MCP with `hagi mcp enable serena` or `hagi mcp enable memory`

**If `--current-only` but serena unavailable:**
- Display error and stop

**If `--memory-only` but memory unavailable:**
- Display error and stop

## Notes

- This command leverages both short-term (serena) and long-term (memory) knowledge
- Pattern analysis helps maintain consistency across projects
- Saved patterns become increasingly valuable over time
- Use regularly when implementing common patterns (error handling, auth, pagination, etc.)
- Patterns are automatically tagged for easy retrieval

## Best Use Cases

1. **Before implementing a feature**: Check if you've done something similar
2. **During code review**: Validate patterns against past best practices
3. **Refactoring**: Find better implementations from past projects
4. **Learning**: Understand how you solved similar problems before
5. **Consistency**: Maintain similar patterns across projects
