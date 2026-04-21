---
name: commit
description: Create a conventional commit for the current changes.
---

# Commit

Create a commit following project conventions using git.

## Instructions

### 1. Review Changes

Run the following commands to understand the current state:

```sh
git status
```

This shows staged, unstaged, and untracked files.

```sh
git diff
```

This shows unstaged changes.

```sh
git diff --staged
```

This shows changes that are already staged.

```sh
git log --oneline -10
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
gest task show <id> --json
```

If the `metadata` object contains a `github-issue` key, include a footer referencing that issue (e.g. `Closes #42`). If
there is no `github-issue` metadata, do not add an issue reference.

### 3. Confirm

Present the draft commit message to the user for approval. Do not commit without confirmation.

### 4. Stage Changes

Stage the files to be committed. Prefer adding specific files by name rather than using `git add .` or `git add -A`,
which can accidentally include sensitive files or large binaries:

```sh
git add <file1> <file2> ...
```

### 5. Execute

Create the commit with the approved message:

```sh
git commit -m "<approved message>"
```
