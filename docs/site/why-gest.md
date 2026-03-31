# Why gest?

AI agents are good at generating tasks. They decompose a feature request into dozens of subtasks,
write specs, and propose execution plans. The hard part is turning that output into parallel
workstreams without losing track of what depends on what.

Gest solves this by giving you a lightweight task and artifact store that lives in your repo as
plain files. No database, no server, no accounts -- just TOML and Markdown that version-control
alongside your code.

## The problem

A typical AI-assisted workflow looks like this:

1. You describe a feature.
2. An agent produces a plan with 15-20 tasks.
3. You execute them one at a time, in one context window, hoping nothing gets lost.

This breaks down quickly. Sequential execution is slow. A single context window can't hold the
full plan plus the code it's modifying. And if the agent loses context mid-way, you're back to
manually piecing together what was done and what remains.

What you need is a way to decompose work into phases, track dependencies, and dispatch
independent tasks in parallel -- without adding infrastructure overhead.

## The workflow

Gest structures work into three layers: **artifacts** capture the design, **tasks** capture
the work, and **iterations** organize tasks into a phased execution plan.

Here's a concrete example. You're adding a new CLI command with several moving parts:

```sh
# Capture the design as an artifact
gest artifact create \
  --title "Add export command" \
  --type spec \
  --body "Export tasks and artifacts to JSON or CSV for external tooling."

# Create tasks with phase assignments
gest task create "Add export data types" --phase 1 --priority 0
gest task create "Add CSV formatter" --phase 1 --priority 1
gest task create "Add JSON formatter" --phase 1 --priority 2
gest task create "Wire up CLI command" --phase 2 --priority 0
gest task create "Add integration tests" --phase 3 --priority 0
```

Phase 1 tasks have no dependencies on each other -- the data types, CSV formatter, and JSON
formatter can all be built concurrently. Phase 2 wires them together. Phase 3 validates the
result.

```sh
# Set dependencies
gest task link <wire-up-id> blocked-by <data-types-id>
gest task link <wire-up-id> blocked-by <csv-id>
gest task link <wire-up-id> blocked-by <json-id>

# Group into an iteration
gest iteration create "Implement export command"
gest iteration add <iteration-id> <task-id>  # repeat for each task

# Visualize the plan
gest iteration graph <iteration-id>
```

The iteration graph shows which tasks can run in parallel and which must wait. Agents or
developers can pick up independent tasks concurrently, each in its own workspace.

## What makes it work

The parallelization story depends on three design choices:

**Plain files.** Tasks are TOML. Artifacts are Markdown with YAML frontmatter. There is no
database to synchronize, no migrations to run, no server to keep alive. Each file is an
independent unit keyed by a unique ID.

**VCS-native.** The `.gest/` directory is just another directory in your repo. When you create
a branch or workspace for parallel work, the task state comes along automatically. Merges work
through your normal VCS workflow -- because files are keyed by unique IDs, merge conflicts are
rare and only happen when two workers edit the same task concurrently.

**Zero infrastructure.** There is nothing to install beyond the `gest` binary. No Docker
containers, no cloud services, no configuration ceremony. This makes it practical to spin up
parallel workspaces on the fly -- each one gets a complete copy of the task state for free.

These properties are not features for their own sake. They are what make lightweight parallel
execution viable. A system that requires a central database or network coordination adds
friction to every workspace you create. Plain files eliminate that friction.

## Fits your existing stack

Gest is not a replacement for GitHub Issues, Jira, or your team's project tracker. It operates
at a different level -- in-flight development work that moves too fast for a team-facing tool.

Use gest for decomposition and parallel execution during development. When a task needs broader
visibility -- a bug report, a feature request, a cross-team dependency -- promote it to your
team tracker. Every list and search command supports `--json` for scripting this handoff.

```sh
# Example: find all open tasks tagged "blocker" and export for triage
gest task list --tag blocker --json
```

Gest handles the fast, local, developer-side loop. Your team tracker handles coordination and
visibility. They complement each other.
