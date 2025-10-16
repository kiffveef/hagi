# Code Style

- **Match project style**: Follow existing code conventions
- **Clear naming**: Names should express purpose
- **Comment "why"**: Explain reasoning, not mechanics

## Formatting
- **Versions**: No spaces - `Rust1.75`, `Python3.12` (not `Rust 1.75`)
- **Parentheses**: Half-width `()` only (not full-width `()`)

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
