# Code Style

- **Match project style**: Follow existing code conventions
- **Clear naming**: Names should express purpose
- **Comment "why"**: Explain reasoning, not mechanics

## Formatting
- **Versions**: No spaces - `Rust1.75`, `Python3.12` (not `Rust 1.75`)
- **Parentheses**: Half-width `()` only (not full-width `()`)

## Comment Rules

- **Own line only**: No end-of-line comments
- **Explain "why", not "what"**: Code should be self-explanatory
- **Keep concise**: One line preferred
- **Stay current**: Outdated comments are worse than none

```rust
// ❌ End-of-line comment
let total = price * 1.1;  // Apply 10% tax

// ✅ Comment on its own line, explains why
// Tax rate required by billing regulation
let total = price * 1.1;
```
