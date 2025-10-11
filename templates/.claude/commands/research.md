Comprehensive research using one-search + context7 + mem0-mcp integration.

# /research Command

Perform comprehensive research on a topic by combining:
1. **one-search**: Web search for practical examples and community insights
2. **context7**: Official documentation for accurate specifications
3. **Integration**: Synthesize information from both sources
4. **mem0-mcp**: Save key learnings to long-term memory (optional)

## Usage

```
/research <topic> [--no-save]
```

## Arguments

- `<topic>`: The topic to research (required)
- `--no-save`: Skip saving to memory (optional)

## Examples

```
/research Rust async programming
/research "Next.js 14 App Router"
/research "Axum 0.7 CORS configuration" --no-save
```

## Workflow

### Step 0: Memory Check (Automatic)

**FIRST**, check if this topic was researched before.

**Instructions for Claude:**

1. **IF mem0-mcp is available:**
   - Use `search_memories` tool with the topic
   - Search tags and content for relevant matches

2. **IF memories found:**
   - Display: "✅ Found previous research from {date}"
   - Show the saved information with sources
   - Ask user: "Use this information or update with latest?"
   - **Wait for user response:**
     - User says "use", "ok", "yes" → **END here, use cached info**
     - User says "update", "refresh", "latest" → **Continue to Step 1**
     - User provides no clear response → **Assume update, continue to Step 1**

3. **IF no memories found:**
   - Display: "ℹ️ No previous research found. Starting new research..."
   - Continue to Step 1

4. **IF mem0-mcp not available:**
   - Display: "ℹ️ Memory not available. Proceeding with fresh research."
   - Continue to Step 1

5. **IF `--force` flag provided:**
   - Skip memory check entirely
   - Display: "🔄 Force mode: Skipping memory check"
   - Continue to Step 1

6. **IF `--memory-only` flag provided:**
   - ONLY search memory, do NOT proceed to Step 1
   - If found: display and END
   - If not found: display "No memory found" and END

### Step 1: Web Search (one-search)

Search the web for practical examples, tutorials, and community insights.

**Instructions for Claude:**
- Use `one_search` tool if available
- Search for: tutorials, blog posts, Stack Overflow discussions
- Focus on: best practices, common patterns, real-world examples

**Example queries:**
- "{topic} tutorial"
- "{topic} best practices"
- "{topic} examples"

### Step 2: Official Documentation (context7)

Retrieve official documentation for accurate specifications.

**Instructions for Claude:**
- Use `resolve-library-id` to find relevant libraries
- Use `get-library-docs` to fetch documentation
- Focus on: API specifications, official examples, version-specific info

**Example:**
```
1. resolve-library-id: "rust async"
2. get-library-docs: /rust-lang/rust
```

### Step 3: Integration & Analysis

Synthesize information from both sources:

**Combine:**
- ✅ Official specifications (context7)
- ✅ Practical examples (one-search)
- ✅ Community best practices (one-search)
- ✅ Version-specific details (context7)

**Provide:**
- Clear explanation of the topic
- Code examples with explanations
- Best practices and common pitfalls
- Links to official documentation
- Links to useful articles/tutorials

### Step 3b: Current Codebase Integration (serena) [Optional]

If serena MCP is available and the topic is relevant to the current project, integrate findings with the current codebase.

**Instructions for Claude:**

1. **Check if serena is available:**
   - Look for serena MCP in available tools
   - Skip this step if serena is not available

2. **Analyze current codebase:**
   - Use serena to search for relevant code in current project
   - Find similar patterns or related implementations
   - Identify potential integration points

3. **Provide integration suggestions:**
   - How to apply the research findings to current code
   - Refactoring opportunities based on best practices found
   - Code examples adapted to current project structure
   - Migration steps if applicable

**Example scenarios:**
- Researching "Rust async programming" → Find async code in current project and suggest improvements
- Researching "Next.js 14 App Router" → Locate current routing code and propose migration path
- Researching "Axum 0.7 CORS" → Find existing CORS configuration and suggest updates

**Output format:**
```markdown
### Current Project Integration (serena)

**Relevant code found:**
- `src/handlers.rs:45` - Current async handler implementation
- `src/routes.rs:23` - Route definitions

**Suggested improvements:**
1. Apply best practice from research: [specific recommendation]
2. Refactor code at `src/handlers.rs:45` using pattern from [source]
3. Add missing error handling based on community recommendations

**Migration steps:**
1. Update dependency versions
2. Refactor handlers to use new pattern
3. Test thoroughly
```

### Step 4: Memory Storage (mcp-memory-service)

Save or update key learnings in long-term memory.

**Instructions for Claude:**

**Determine save/update mode:**
- IF Step 0 found existing memory AND user chose "update" → **UPDATE mode**
- IF Step 0 found no memory → **NEW SAVE mode**
- IF `--no-save` flag provided → **SKIP saving entirely**

**UPDATE mode:**
1. Use `update_memory` tool (if available)
2. OR delete old memory + save new one
3. Display: "✅ Memory updated with latest information"

**NEW SAVE mode:**
1. Use `save_memory` or `add_memory` tool
2. Display: "✅ Research saved to long-term memory"

**SKIP mode:**
- Display: "ℹ️ Skipped memory storage (--no-save)"

**Requirements for saving:**
- ONLY save if mem0-mcp is available
- MUST include verification information

**Memory Format:**
```json
{
  "content": "Key learnings about {topic}",
  "sources": [
    "https://official-docs-url",
    "https://helpful-article-url"
  ],
  "type": "research_topic",
  "verified": "YYYY-MM-DD",
  "tags": ["tag1", "tag2", "tag3"]
}
```

**Important:**
- Include official documentation URLs
- Add verification date
- Use relevant tags for future retrieval
- Keep content concise but comprehensive

## Output Format

Present the research results in the following structure:

```markdown
## {Topic} Research Results

### Official Documentation (context7)

[Key points from official docs]
- Specification details
- Official examples
- Version information

**Source:** [Official docs URL]

### Practical Insights (one-search)

[Key points from web search]
- Real-world examples
- Best practices
- Common pitfalls

**Sources:**
- [Article 1 URL]
- [Article 2 URL]

### Code Examples

[Practical code examples with explanations]

### Best Practices

- Best practice 1
- Best practice 2
- ...

### Current Project Integration (serena) [If Available]

**Relevant code found:**
- `path/to/file.rs:line` - Description of relevant code

**Suggested improvements:**
1. Improvement 1 with specific code location
2. Improvement 2 with refactoring suggestion

**Migration steps:**
1. Step 1
2. Step 2
3. Step 3

### Memory Status

**If NEW SAVE:**
✅ Research saved to long-term memory
- Tags: [tag1, tag2, tag3]
- Date: {today's date}
- Next time: `/research {topic}` will find this automatically

**If UPDATE:**
✅ Memory updated with latest information
- Previous: {old date}
- Updated: {today's date}

**If SKIPPED:**
ℹ️ Memory storage skipped (--no-save or mem0-mcp unavailable)
```

## Error Handling

**If one-search is not available:**
- Continue with context7 only
- Notify user: "⚠️ one-search not available, using context7 only"

**If context7 is not available:**
- Continue with one-search only
- Notify user: "⚠️ context7 not available, using web search only"

**If mem0-mcp is not available:**
- Skip memory storage
- Notify user: "ℹ️ mem0-mcp not available, skipping memory storage"

**If no MCP servers are available:**
- Provide research based on built-in knowledge
- Warn user about potentially outdated information

## Notes

- This command leverages multiple MCP servers for comprehensive research
- Official documentation (context7) takes precedence for accuracy
- Web search (one-search) provides practical context and real-world usage
- Memory storage (mem0-mcp) enables knowledge reuse across projects
- Use `--no-save` when researching temporary or experimental topics
