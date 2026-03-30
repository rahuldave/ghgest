---
name: code-review
description: Review the current changeset for correctness, safety, style, and test coverage.
---

# Code Review

Review changes for correctness, safety, style, and test coverage.

## Instructions

### 1. Gather the Changeset

Run both commands to see the full picture:

- `git diff` to show unstaged changes
- `git diff --staged` to show staged changes

If a specific commit is provided, run `git diff <commit>~1 <commit>` to diff that commit instead.

### 2. Review

Evaluate the changeset against this checklist:

#### Correctness

- Logic errors, off-by-one mistakes, unhandled edge cases
- Does the code do what it claims?
- Are error conditions handled?

#### Safety

- Resource leaks (file handles, connections, memory)
- Injection risks (SQL, command, template)
- Improper input handling or missing validation at system boundaries

#### Error Handling

- Errors are surfaced clearly, not silently swallowed
- Error messages are actionable and include context
- Failures don't leave the system in an inconsistent state

#### Style

- Follows conventions from `docs/dev/code-style.md`
- Naming is clear and consistent with the codebase
- Code organization matches project structure

#### Test Coverage

- New functionality has corresponding tests
- Edge cases are tested
- Existing tests are not weakened or removed without justification
- CLI-facing changes (`src/cli/`) should have corresponding integration tests in `tests/integration/`. Flag missing
  integration test coverage as a **Warning**.

#### Dependency Hygiene

- No unnecessary new dependencies
- Dependencies are maintained and up to date

### 3. Report Findings

Categorize each finding by severity:

- **Blocking** -- must be fixed (bugs, correctness issues, test failures, security vulnerabilities)
- **Warning** -- should be fixed (style violations, missing edge case tests)
- **Suggestion** -- optional improvements (refactoring ideas, alternative approaches)

For each finding include: file path, line number, description, and suggested fix.

If there are no findings, say so.
