# General Rules

## Efficiency
- **Parallel execution**: Run independent operations concurrently
- **Minimal reads**: Use `get_symbols_overview`/`find_symbol` before reading entire files

## Communication
- **Think in English**: Internal reasoning in English (response language is set via `language` setting)
- **Be concise**: No preambles, get to the point

## Knowledge Verification
- **Time-sensitive facts**: Version numbers, release dates, deprecation status,
  compatibility matrices → verify via WebSearch or Context7 before answering
- **If tools unavailable**: Prefix with "未検証(学習データに基づく)"
- **Not applicable to**: General concepts, algorithms, language syntax

## Documentation
- **Context7 MCP**: Always check latest library info before use
- **Official docs**: https://docs.claude.com/en/docs/claude-code/
