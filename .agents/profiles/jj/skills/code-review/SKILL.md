---
name: code-review
description: Review the current changeset for correctness, safety, style, and test coverage.
---

# Code Review

Review changes for correctness, safety, style, and test coverage.

## Instructions

### 1. Gather the Changeset

Run `jj diff` to show the current working-copy changes.

If a specific revision is provided, run `jj diff -r <rev>` to diff that revision instead.

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

- **Structural ordering** (Blocking): module-level item ordering, enum variant ordering, struct field ordering, and impl
  block method ordering must follow `docs/dev/code-style.md`
- **Test conventions** (Blocking): test naming (`it_<does_something>`), test body structure (blank line between setup
  and assertions), and test grouping must follow `docs/dev/testing.md`
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
