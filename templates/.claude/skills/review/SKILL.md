---
name: review
description: Third-party perspective code review and refactoring suggestions. Use when user wants objective code review, refactoring advice, or quality assessment.
---

# Review

Perform objective code review from a **third-party perspective** - as if you're seeing this code for the first time with no assumptions about the author's intent.

## Philosophy

- **Don't assume intent is correct** - Question every design decision
- **Challenge "it works"** - Working code isn't necessarily good code
- **Think about the future** - Can a new developer understand and modify this?
- **Be constructively critical** - Point out issues, suggest improvements

## Usage

```
/review <target> [options]
```

## Arguments

- `<target>`: Review target (file path, directory, feature name, or description)

## Options

| Option | Description |
|--------|-------------|
| `--strict` | Strict mode - include minor issues and nitpicks |
| `--focus <area>` | Focus area: `security`, `performance`, `readability`, `architecture` |
| `--refactor` | Provide concrete refactoring code suggestions |
| `--diff` | Review recent git diff instead of specific files |

## Examples

```
/review src/commands/install.rs
/review src/commands/ --focus architecture
/review "authentication feature" --strict --refactor
/review --diff
/review src/utils.rs --focus performance --refactor
```

## Workflow

### Step 0: Identify Target Code

**Determine what to review:**

1. **If file/directory path provided:**
   - Use Glob to find matching files
   - Read file contents with Read tool

2. **If feature name/description provided:**
   - Use Grep to search for relevant code
   - Use serena MCP if available for semantic search

3. **If `--diff` flag provided:**
   - Run `git diff HEAD~1` or `git diff --staged`
   - Review only changed lines with context

**Output:**
```
Review Target:
- Files: N files identified
- Lines: ~N lines to review
- Scope: [file/directory/feature/diff]
```

---

### Step 1: Structural Analysis

**Analyze code structure (use serena if available):**

1. **Identify components:**
   - Functions/methods and their signatures
   - Classes/structs/modules
   - Dependencies and imports
   - Public API surface

2. **Map relationships:**
   - Call graph (who calls whom)
   - Data flow
   - Dependency direction

**Output:**
```
Structure Overview:
- Functions: N (public: N, private: N)
- Modules: N
- External dependencies: N
- Complexity hotspots: [list files with high complexity]
```

---

### Step 2: Best Practices Check (context7)

**If relevant libraries are used, check against official best practices:**

1. **Identify libraries/frameworks** in the code
2. **Use context7** to fetch current best practices
3. **Compare** current implementation against recommendations

**Skip this step if:**
- No external libraries used
- context7 MCP not available

**Output:**
```
Best Practices Check:
- Library: [name] - Following / Deviating
  - [Specific deviation if any]
```

---

### Step 3: Third-Party Perspective Checklist

**Apply the following checklist rigorously:**

#### 3.1 Intent Clarity
- [ ] Is the code's purpose obvious from names and structure?
- [ ] Can you understand "why" without comments?
- [ ] Are there magic numbers or implicit assumptions?
- [ ] Would a newcomer understand this in 5 minutes?

#### 3.2 Responsibility Clarity
- [ ] Does each function/module have a **single responsibility**?
- [ ] Can you describe each function in **one sentence without "and"**?
- [ ] Does the function name match what it actually does?
- [ ] Are there **hidden side effects**? (actions not implied by the name)
- [ ] Are file/module boundaries logical?
- [ ] Is the dependency direction correct? (no circular dependencies)

#### 3.3 Design Validity
- [ ] Is this design actually optimal? What alternatives exist?
- [ ] Is it over-engineered or under-engineered?
- [ ] Is the abstraction level appropriate?
- [ ] Are there unnecessary layers of indirection?

#### 3.4 Maintainability
- [ ] Can your future self understand this in 6 months?
- [ ] Can a new team member modify this safely?
- [ ] Is the impact of changes predictable?
- [ ] Is the code self-documenting?

#### 3.5 Error Handling
- [ ] Are failure cases covered?
- [ ] Are error messages useful and actionable?
- [ ] Are there potential panics/crashes?
- [ ] Is error propagation consistent?

#### 3.6 Testability
- [ ] Is the code unit-testable?
- [ ] Are dependencies injectable/mockable?
- [ ] Are edge cases considered?
- [ ] Is the code deterministic?

#### 3.7 AI-Generated Code Patterns
- [ ] Are there **unnecessary recursion or deep nesting**?
- [ ] Is there **manual implementation** where built-in functions exist?
- [ ] Is the code **overly generic** for its actual use case?
- [ ] Does it follow **project naming conventions** and idioms?
- [ ] Are there **DRY violations** (copy-pasted code)?
- [ ] Is there **excessive abstraction** that adds complexity without benefit?

---

### Step 4: Generate Review Report

**Categorize findings by severity:**

```markdown
## Code Review: {target}

### Overview
- **Files reviewed**: N files
- **Lines analyzed**: ~N lines
- **Overall assessment**: Needs Work / Acceptable / Good

---

### Critical Issues

Issues that **must be fixed** - bugs, security risks, or major design flaws.

1. **{Issue title}** (`path/to/file.rs:45-67`)

   **Problem:**
   [Clear description of what's wrong]

   **Why it matters:**
   [Impact - security risk, bug potential, maintenance nightmare]

   **Suggestion:**
   [How to fix it]

---

### Warnings

Issues that **should be addressed** - code smells, potential problems, or deviations from best practices.

1. **{Warning title}** (`path/to/file.rs:78`)

   **Concern:**
   [What's concerning about this code]

   **Suggestion:**
   [Improvement recommendation]

---

### Suggestions

**Nice to have** improvements - not critical but would improve code quality.

1. **{Suggestion}** (`path/to/file.rs:90`)
   - [Brief improvement suggestion]

---

### Good Practices Found

Acknowledge what's done well:
- {Good practice 1}
- {Good practice 2}

---

### Summary

| Category | Count |
|----------|-------|
| Critical | N |
| Warnings | N |
| Suggestions | N |
| Good practices | N |
```

---

### Step 5: Refactoring Suggestions (--refactor only)

**If `--refactor` flag is provided, include concrete code changes:**

```markdown
### Refactoring Suggestions

#### 1. {Refactoring title}

**Location:** `path/to/file.rs:45-67`

**Before:**
```rust
// Current problematic code
fn process_and_validate_and_save(data: &Data) -> Result<()> {
    // 50 lines doing three things
}
```

**After:**
```rust
// Refactored code with single responsibility
fn process(data: &Data) -> ProcessedData {
    // Only processing
}

fn validate(data: &ProcessedData) -> Result<ValidatedData> {
    // Only validation
}

fn save(data: &ValidatedData) -> Result<()> {
    // Only saving
}
```

**Why this is better:**
- Single responsibility per function
- Easier to test each step independently
- Clearer error handling per stage
- More reusable components

---

#### 2. {Next refactoring...}
```

---

## Focus Areas (--focus)

When `--focus <area>` is specified, prioritize that aspect:

### `--focus security`
- Input validation
- Authentication/authorization
- Data sanitization
- Secrets handling
- SQL injection, XSS, etc.

### `--focus performance`
- Algorithm complexity
- Memory allocation
- I/O operations
- Caching opportunities
- Unnecessary computations

### `--focus readability`
- Naming clarity
- Code organization
- Comment quality
- Cognitive complexity
- Consistency

### `--focus architecture`
- Module boundaries
- Dependency direction
- Abstraction levels
- Separation of concerns
- Extensibility

---

## Responsibility Violation Patterns

Common patterns to identify and flag:

### Pattern 1: God Function
```rust
// Does too many things
fn handle_request(req: Request) -> Response {
    validate(req);
    authenticate(req);
    process(req);
    save_to_db(req);
    send_notification(req);
    log(req);
    build_response(req)
}
```

**Flag as:** "Function has 7+ responsibilities"

### Pattern 2: Hidden Side Effects
```rust
// Name suggests read-only, but writes
fn get_user_config() -> Config {
    let config = read_file();
    write_file(config.with_defaults()); // Hidden side effect!
    config
}
```

**Flag as:** "Side effect hidden in getter function"

### Pattern 3: Misleading Names
```rust
// Name doesn't match behavior
fn validate_email(email: &str) -> String {
    email.to_lowercase().trim() // This is normalization, not validation!
}
```

**Flag as:** "Function name doesn't match actual behavior"

### Pattern 4: Circular Dependencies
```
// A depends on B, B depends on A
module_a.rs -> imports module_b
module_b.rs -> imports module_a
```

**Flag as:** "Circular dependency detected"

---

## AI-Generated Code Patterns

Patterns commonly produced by AI that need review:

### Pattern 1: Unnecessary Recursion
```rust
// AI-style: Recursive when loop is simpler
fn sum_recursive(arr: &[i32], idx: usize) -> i32 {
    if idx >= arr.len() { 0 }
    else { arr[idx] + sum_recursive(arr, idx + 1) }
}

// Better: Simple iteration
fn sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
```

### Pattern 2: Manual Implementation of Built-ins
```rust
// AI-style: Manual dictionary counting
let mut counts = HashMap::new();
for item in items {
    *counts.entry(item).or_insert(0) += 1;
}

// Better: Use itertools or built-in
use itertools::Itertools;
let counts = items.into_iter().counts();
```

### Pattern 3: Over-Generic Code
```rust
// AI-style: Generic when not needed
fn process<T: AsRef<str> + Clone + Debug + Send + Sync>(input: T) -> T {
    // Only ever called with &str
}

// Better: Simple and specific
fn process(input: &str) -> String {
    // Clear, simple, sufficient
}
```

### Pattern 4: Deep Nesting
```rust
// AI-style: Nested conditionals
if condition1 {
    if condition2 {
        if condition3 {
            // actual logic buried here
        }
    }
}

// Better: Early returns
if !condition1 { return; }
if !condition2 { return; }
if !condition3 { return; }
// actual logic at top level
```

---

## Error Handling

**If target not found:**
```
Could not find review target: {target}
   - Check the path exists
   - Try a more specific pattern
   - Use --diff to review recent changes
```

**If serena not available:**
- Continue without structural analysis
- Note: "Structural analysis limited (serena not available)"

**If context7 not available:**
- Skip best practices check
- Note: "Skipping library best practices check (context7 not available)"

---

## Output Guidelines

**Keep the review actionable:**
- Specific file and line references
- Clear problem descriptions
- Concrete suggestions
- Code examples where helpful
- No vague criticisms without solutions
- No excessive nitpicking (unless --strict)
- No personal style preferences as "issues"

**Tone:**
- Professional and objective
- Constructive, not destructive
- Assume good intent, question the result
- Focus on code, not coder

---

## Notes

- This command is for **code quality improvement**, not gatekeeping
- Use `--strict` for thorough review before major releases
- Use `--refactor` when you want concrete code suggestions
- Use `--diff` for PR-style incremental reviews
- Combine with `/serena` for deeper pattern analysis
