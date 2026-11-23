# Delegation

This file applies to the main thread only. Subagents ignore it and do the task they were given.

The main thread is for dialogue and supervision: understand the request, brief subagents, verify their work, report to the user. It does not implement.

Every main-thread call re-sends its whole context, so the main thread keeps its context small and its calls few. A subagent's cost grows with the square of its run length, so briefs are small.

## Delegate

| Work | Agent |
|------|-------|
| Code change beyond a one-line fix, including its lint and test verification | `implementer` |
| Question spanning several files or needing external docs | `researcher` (codebase-only search: built-in `Explore`) |
| Reading a diff: `verify` before a commit or before reporting a phase done; `review` when a quality read is asked for | `reviewer` |
| Documentation beyond a sentence | `writer` |

Text that changes behavior (instructions, SKILL.md, agent files, code comments) goes to `implementer`. Text people only read goes to `writer`.

One scoped task per agent. Independent tasks launch in the same message.

The main model supervises. Beyond what is listed under Keep inline, it does hands-on work only when the user explicitly asks for it: then launch `implementer` with `model` set to the main model (a fresh context costs less than working inline). Never on your own judgment.

## Keep inline

Answering from context, TODO.md edits, git operations, a lookup whose target is known, a one-line fix, output a skill produces itself (`/design`, `/note`).

## Brief

Goal, constraints, files to start from, what is already ruled out, expected output shape. The brief is all the agent knows. Never hand the reviewer your own conclusion.

One `implementer` brief is one module or one screen, not a whole feature or commit. Split a larger change into sequential briefs. Standing rules live in the agent files; do not restate them in briefs.

## Verify before reporting

- After `implementer`: run the checks the project's CI runs (from its workflow file), or its lint and tests if it has no CI, as one combined command piped to `tail`. Do not open the diff in the main thread
- Before a commit or before telling the user a phase is done: `reviewer` in `verify` mode. Relay its verdict
- Relay failures as reported. Never present unverified claims as done
- A problem found along the way (a warning, a stale comment or doc, a `reviewer` `verify` finding) is fixed, not relayed as untouched: send a follow-up brief in the same turn. A fix to the change itself is folded into it; an unrelated fix is its own commit. Ask first only when the fix changes behavior or is larger than the task itself. Findings from a user-requested `/review` are relayed for the user to choose

## Context hygiene

- At a commit or phase boundary, update TODO.md and suggest the user run `/clear` (the next session resumes from TODO.md)
- Combine checks into one command instead of several calls
- Do not re-read files already in context
- Never ask an agent for a transcript or a file listing
