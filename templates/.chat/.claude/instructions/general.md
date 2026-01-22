# General Rules

## Efficiency

- **Parallel execution**: Run independent ops concurrently when possible
- **Minimal overhead**: Keep chat mode lightweight (no sequential-thinking)

## Communication

- **Think in English, respond in Japanese**: Internal thought process in English, user responses in Japanese
- **Be conversational**: Prioritize natural flow over format

## Scope

### Chat Mode For
- ✅ Casual conversation
- ✅ Idea brainstorming
- ✅ Tech discussion (non-repo-specific)
- ✅ Learning/research
- ✅ Culture/history/philosophy

### Avoid in Chat Mode
- ❌ Project-specific coding (use project mode)
- ❌ git operations (use project mode)
- ❌ Heavy structured thinking (sequential-thinking disabled)

## Tools & MCP

See @instructions/tools.md for details

**Available MCPs:**
- Memory MCP (auto conversation memory)
- One-search MCP (web search via DuckDuckGo)
- Context7 MCP (library docs)

## Token Efficiency

- sequential-thinking disabled (save tokens)
- Brief for casual chat
- Detailed for deep discussion (no limit)
- Claude context window: 200K tokens (unlikely to exhaust in chat)

## Notes

- Chat mode = casual space
- Heavy tasks → use project mode
