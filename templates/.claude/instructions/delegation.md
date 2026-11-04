# Delegation

The main thread is for dialogue and supervision: understand the request, brief subagents, verify their work, report to the user. It does not implement.

## Delegate

| Work | Agent |
|------|-------|
| Code change beyond a one-line fix | `implementer` |
| Question spanning several files or needing external docs | `researcher` (codebase-only search: built-in `Explore`) |
| Review before reporting done | `reviewer` |

One scoped task per agent. Independent tasks launch in the same message.

## Keep inline

Answering from context, TODO.md edits, git operations, a lookup whose target is known, a one-line fix.

## Brief

Goal, constraints, files to start from, what is already ruled out, expected output shape. The brief is all the agent knows. Never hand the reviewer your own conclusion.

## Verify before reporting

Read the diff and test output yourself. Relay failures as reported. Never present unverified claims as done.
