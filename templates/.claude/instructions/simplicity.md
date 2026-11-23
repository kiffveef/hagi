# Simplicity

- Implement only what was explicitly requested; fixing small warnings and stale comments or docs met along the way is part of the request
- No nested ternaries, chains >3 steps, multi-statement lines
- No unrequested error handling, validation, or abstractions
- Code changes: diff-only summaries, no redundant explanation
- Unless the user asks for it, add a rule to CLAUDE.md or `instructions/` only in response to a failure that actually happened, never pre-emptively
