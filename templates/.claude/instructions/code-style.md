# Code Style

- **一貫性のあるスタイル**: プロジェクト既存のコードスタイルに従うこと。
- **意味のある命名**: 変数名、関数名、クラス名は、その役割を明確に表現すること。
- **適切なコメント**: コードの「なぜ」を説明するコメントを書くこと。「何を」しているかは、コード自体で明確にすること。

## Version Notation
- **バージョン表記**: ライブラリ・フレームワーク名とバージョン番号の間にスペースを入れないこと
  - ✅ 正しい: `Rust1.75`, `Axum0.7`, `Python3.12`
  - ❌ 間違い: `Rust 1.75`, `Axum 0.7`, `Python 3.12`

## Punctuation
- **括弧の使用**: 括弧を使う場合は必ず半角括弧 `()` を使用すること
  - ✅ 正しい: `Web検索(DuckDuckGo)`, `設定ファイル(mcp.json)`
  - ❌ 間違い: `Web検索(DuckDuckGo)`, `設定ファイル(mcp.json)`

## Comment Style Rules

### ✅ Allowed: One-line Comments

**Short, focused comments on their own line:**

```rust
// Calculate the total price including tax
let total = price * (1.0 + tax_rate);
```

```python
# Validate user input before processing
if not is_valid_email(email):
    return error
```

```typescript
// Initialize the database connection pool
const pool = createPool(config);
```

### ❌ Forbidden: End-of-line Comments

**NEVER place comments at the end of code lines:**

❌ **BAD:**
```rust
let total = price * 1.1;  // Apply 10% tax
```

❌ **BAD:**
```python
result = calculate()  # Get the result
```

❌ **BAD:**
```typescript
const data = fetch(url);  // Fetch data from API
```

✅ **GOOD:**
```rust
// Apply 10% tax
let total = price * 1.1;
```

✅ **GOOD:**
```python
# Get the result
result = calculate()
```

✅ **GOOD:**
```typescript
// Fetch data from API
const data = fetch(url);
```

### Comment Content Guidelines

- **Explain "why", not "what"**: Code should be self-explanatory for "what" it does
- **Keep comments concise**: One line is preferred
- **Update comments when code changes**: Outdated comments are worse than no comments
- **Avoid redundant comments**: Don't state the obvious

**Examples:**

❌ **BAD - States the obvious:**
```rust
// Increment counter by 1
counter += 1;
```

✅ **GOOD - Explains why:**
```rust
// Keep track of retry attempts for rate limiting
counter += 1;
```
