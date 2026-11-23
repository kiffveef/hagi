---
name: review
description: Third-party perspective code review and refactoring suggestions. Use when user wants objective code review, refactoring advice, or quality assessment.
---

# Review

Delegate a code review to the `reviewer` subagent in `review` mode and relay its findings. The main thread does not review inline.

## Philosophy

The review is from a third-party perspective: the reviewer sees the code for the first time and is not told the author's intent.
It questions design decisions instead of accepting "it works", and asks whether a new developer could understand and change the code.

## Usage

```
/review <target> [options]
/review --diff [range]
```

`<target>`: file path, directory, feature name, or description.

| Option | Effect on the brief |
|--------|---------------------|
| `--strict` | Ask for minor issues and nitpicks too, not only bugs and design problems |
| `--focus <area>` | Name the area to prioritize: `security`, `performance`, `readability` or `architecture` |
| `--refactor` | Ask for concrete refactoring code in each suggested fix, and allow a longer report for it |
| `--diff` | Scope is a git diff (default: `git diff HEAD~1`, which includes uncommitted changes; or the given range) instead of named files |

## Steps

1. Resolve the target only as far as needed to name it in the brief: paths (Glob is fine), a feature description, or the diff range for `--diff` (`git diff --stat` at most). Do not read file contents.
2. Launch `reviewer` with a brief containing:
   - Mode `review` and the goal: third-party review of the target, questioning intent and design
   - Scope: the paths, feature description or diff range
   - Focus and options from the table above
   - What is already known and ruled out (facts from the user, not your own assessment of the code)
   - Output shape: findings with `path:line`, ranked Critical / Warnings / Suggestions, plus what was not covered
   Never hand the reviewer your own conclusion about the code.
3. Relay the findings to the user ranked by severity (Critical / Warnings / Suggestions) with `path:line`, and state what the reviewer did not cover. Do not add findings of your own.

## Examples

```
/review src/commands/install.rs
/review src/commands/ --focus architecture
/review "authentication feature" --strict --refactor
/review --diff main..HEAD
```
