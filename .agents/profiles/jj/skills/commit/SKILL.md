---
name: commit
description: Create a conventional commit for the current changes.
---

# Commit

Create a commit following project conventions using Jujutsu (jj).

## Instructions

### 1. Review Changes

Run the following commands to understand the current state:

```sh
jj status
```

This shows what files are changed in the working-copy commit.

```sh
jj diff
```

This shows the actual content changes.

```sh
jj log -n 10
```

This shows recent commits so you can match the existing style.

### 2. Draft Commit Message

Follow the conventions in `docs/dev/commits.md`:

```text
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:** feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert

**Rules:**

- If a scope can reasonably apply, it **must** be included
- Multiple scopes are comma-separated
- Imperative mood ("add feature" not "added feature")
- First line under 72 characters
- Append `!` after type/scope for breaking changes
- **Never** reference gest task or artifact IDs in commit messages

**GitHub Issue references:**

If the current work relates to a gest task, check whether the task has `github-issue` metadata:

```sh
cargo run -- task show <id> --json
```

If the `metadata` object contains a `github-issue` key, include a footer referencing that issue
(e.g. `Closes #42`). If there is no `github-issue` metadata, do not add an issue reference.

### 3. Confirm

Present the draft commit message to the user for approval. Do not commit without confirmation.

### 4. Execute

Create the commit using one of these approaches:

**To finalize the current working-copy commit and start a new empty change:**

```sh
jj commit -m "<approved message>"
```

**To describe the current working-copy commit without starting a new change:**

```sh
jj describe -m "<approved message>"
```

Use `jj commit -m` when the work is complete and you want to move on to the next change. Use
`jj describe -m` when you want to set the message but continue working in the same change.

There is no staging area in jj -- all tracked file changes in the working copy are included
automatically. Never use raw `git` commands.
