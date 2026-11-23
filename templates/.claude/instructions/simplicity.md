# Simplicity

- Implement only what was explicitly requested
- No nested ternaries, chains >3 steps, multi-statement lines
- No unrequested error handling, validation, or abstractions
- Code changes: diff-only summaries, no redundant explanation
- Unless the user asks for it, add a rule to CLAUDE.md or `instructions/` only in response to a failure that actually happened, never pre-emptively
