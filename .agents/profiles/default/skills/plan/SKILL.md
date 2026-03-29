---
name: plan
description: "Take a spec and create an implementation plan: tasks, ADRs for large features, and dependency ordering (e.g. /plan <gest-id>)."
args: "<gest-id>"
---

# Plan

Assess a spec and produce an implementation plan.

## Instructions

### 1. Read and Understand the Spec

Read the spec via `cargo run -- artifact show <id>`. Identify:

- Acceptance criteria (these become the basis for tasks)
- Components or subsystems affected
- Whether an architectural decision is involved (warrants an ADR)
- Natural parallelization boundaries

### 2. Scope Assessment

Determine the right approach:

- **Single issue** -- small, focused work that can be completed in one session. Few acceptance
  criteria, one component affected.
- **Multi-issue** -- large work spanning multiple components or requiring parallel effort. Many
  acceptance criteria, clear decomposition boundaries.

Present the assessment to the user and ask which path to take.

### 3. Draft ADR (if needed)

If the work involves an architectural decision (new patterns, significant trade-offs, dependency
choices), invoke `/write-adr` before creating tasks.

### 4. Create Tasks

For **single issue**: invoke `/write-issue` with the spec and acceptance criteria.

For **multi-issue**:

1. Identify the natural breakdown (by component, by layer, by acceptance criteria groups)
2. Determine dependencies between tasks (what must be done first)
3. Invoke `/write-issue` for each task, including dependency references
4. Link each task to the source spec: `cargo run -- task link <task-id> child-of <spec-id> --artifact`
5. For issues that affect CLI behavior (commands, flags, output format), include integration test
   acceptance criteria in the task description. Example: "Integration test verifies
   `gest task create --description` outputs the created task ID."
6. Set execution fields on each task:
   - `cargo run -- task update <task-id> --phase <n>` -- phase number for parallel grouping
   - `cargo run -- task update <task-id> --priority <0-4>` -- priority within the phase
7. Set blocking dependencies: `cargo run -- task link <task-id> blocked-by <other-task-id>`
8. Create an iteration and add all tasks:
   - `cargo run -- iteration create "<plan title>"`
   - `cargo run -- iteration link <iteration-id> child-of <spec-id> --artifact`
   - `cargo run -- iteration add <iteration-id> <task-id>` for each task

### 5. Output

Present the task list with IDs to the user:

- Link to the source spec
- Link to ADR (if created)
- Iteration ID
- Task list with titles, IDs, phases, and dependency ordering
- Parallelization notes (which tasks share a phase and can be worked simultaneously)

### 6. Next Step

For multi-issue plans, print: `invoke /orchestrate <iteration-id> when you're ready`

For single issue plans, print: `invoke /implement <task-id> when you're ready`
