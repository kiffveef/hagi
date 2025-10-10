You are working on a complex problem that requires structured, step-by-step thinking. Use the sequential-thinking MCP tool to break down the problem systematically.

# Instructions

1. **Analyze the problem**: Clearly identify what needs to be solved
2. **Break it down**: Divide the problem into logical steps
3. **Plan your approach**: Determine the best sequence of actions
4. **Execute systematically**: Work through each step methodically
5. **Verify results**: Check that each step produces the expected outcome

# Guidelines

- Use clear, logical reasoning at each step
- Document assumptions and decisions
- Identify dependencies between steps
- Consider edge cases and potential issues
- Revise the plan if new information emerges

# User Options

If the user includes `--search` in their request:
- MUST use WebSearch tool to gather external information when needed
- MUST use Context7 MCP(mcp__context7__resolve-library-id and mcp__context7__get-library-docs) to fetch library documentation when libraries/frameworks are mentioned
- Include search results and documentation in your analysis
- Cite sources when referencing external information

If the user includes `--todo` in their request:
- MUST use TodoWrite tool to track progress through each step
- If `.claude/TODO.md` exists in the project, read it first and synchronize with TodoWrite tool
- Create todos at the beginning with all identified steps
- Update todo status as you progress through the analysis
- Mark each step as completed immediately after finishing it

Apply sequential thinking to analyze and solve the problem presented by the user.
