---
name: orchestrate
description: "Take an iteration and execute it: dispatch /implement agents per phase (e.g. /orchestrate <iteration-id>)."
args: "<iteration-id>"
---

# Orchestrate

Execute an iteration by dispatching implementation agents phase by phase.

## Instructions

### 1. Read the Iteration

Retrieve the iteration via `cargo run -- iteration show --json <id>`. Then read each task in the
iteration via `cargo run -- task show --json <task-id>`.

Visualize the execution plan: `cargo run -- iteration graph <id>`.

Extract:

- Task list grouped by `phase` field
- Blocking dependencies (`blocked-by` links)
- Priority ordering within each phase

### 2. Build Phases

Group tasks by their `phase` field:

- **Phase 1** -- tasks with `phase: 1` (run in parallel -- they are conflict-safe)
- **Phase 2** -- tasks with `phase: 2`
- **Phase N** -- and so on

Present the phase plan to the user for confirmation before proceeding.

### 3. Execute Phases

For each phase:

1. Set `assigned_to` on each task: `cargo run -- task update <task-id> --assigned-to <agent-name>`
2. Dispatch `/implement <task-id>` for each task in the phase
3. Wait for all agents in the phase to complete
4. Report results to the user (successes, failures, tasks needing attention)

Only proceed to the next phase after the user confirms the current phase's results.

### 4. Clean Up

After all phases complete:

1. Check for failed tasks -- any task still `in-progress` represents a failure. Report these to the
   user with their IDs and titles.
2. Update the iteration status:
   - If **all tasks** completed successfully (`done`):
     `cargo run -- iteration update <iteration-id> --status completed`
   - If **any tasks** remain `in-progress`:
     `cargo run -- iteration update <iteration-id> --status failed`
     Flag this to the user and list the incomplete tasks.
3. Present a summary of all implemented tasks, including successes and failures
