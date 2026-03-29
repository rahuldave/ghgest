---
name: gest
description: Gest CLI reference for managing artifacts, tasks, and iterations. Use when skills need to create, read, or link entities.
tools: Bash, Read
model: haiku
---

# Gest CLI Agent

Reference agent for interacting with gest -- the project's artifact, task, and iteration store. Gest
stores data outside the repository (XDG data directory). No `gest init` is needed; directories are
created on first write.

All commands use `cargo run --` as the invocation prefix.

## Entity Model

- **Artifacts** -- Markdown documents (specs, ADRs, RFCs). Stored by type.
- **Tasks** -- Actionable work items with status, links, priority, phase, and assignment.
- **Iterations** -- Execution plans that group tasks into phased, parallelizable work.

IDs are 8-character lowercase alphabetic strings. Prefix matching works -- you can use a shorter
prefix if it's unambiguous.

## Artifacts

### Create

```sh
cargo run -- artifact create --title "<title>" --type <kind> --file <path>
cargo run -- artifact create --title "<title>" --type <kind> --body "<inline body>"
```

Types: `spec`, `adr`, `rfc` (freeform string -- use these conventions).

Options: `--tags <comma-separated>`, `--metadata <key=value>`.

Output: `Created artifact <id>` -- extract the ID from this line.

### Show

```sh
cargo run -- artifact show <id>          # human-readable
cargo run -- artifact show --json <id>   # structured JSON
```

### List

```sh
cargo run -- artifact list                     # active only
cargo run -- artifact list --include-archived  # include archived
cargo run -- artifact list --type <kind>       # filter by type
cargo run -- artifact list --tag <tag>         # filter by tag
cargo run -- artifact list --json              # structured JSON
```

### Update

```sh
cargo run -- artifact update <id> --title "<new title>"
cargo run -- artifact update <id> --body "<new body>"
cargo run -- artifact update <id> --type <kind>
```

### Archive

```sh
cargo run -- artifact archive <id>
```

### Tags

```sh
cargo run -- artifact tag <id> <tag>
cargo run -- artifact untag <id> <tag>
```

### Metadata

```sh
cargo run -- artifact meta set <id> <key> <value>
cargo run -- artifact meta get <id> <key>
```

## Tasks

### Create

```sh
cargo run -- task create "<title>" --description "<desc>"
cargo run -- task create "<title>" --status <status> --tags <tags>
cargo run -- task create "<title>" --priority 1 --phase 2 --assigned-to agent-1
```

Options: `--description`, `--status`, `--tags <comma-separated>`, `--metadata <key=value>`,
`--priority <0-4>`, `--phase <number>`, `--assigned-to <actor>`.

Output: `Created task <id>` -- extract the ID from this line.

### Show

```sh
cargo run -- task show <id>          # human-readable
cargo run -- task show --json <id>   # structured JSON
```

### List

```sh
cargo run -- task list                     # active only
cargo run -- task list --all               # include resolved
cargo run -- task list --status <status>   # filter by status
cargo run -- task list --tag <tag>         # filter by tag
cargo run -- task list --json              # structured JSON
```

### Update

```sh
cargo run -- task update <id> --status <status>
cargo run -- task update <id> --title "<new title>"
cargo run -- task update <id> --priority 0
cargo run -- task update <id> --phase 3
cargo run -- task update <id> --assigned-to agent-2
```

Setting status to `done` or `cancelled` automatically archives the task. Setting an archived task's
status to `open` or `in-progress` automatically unarchives it.

### Task Fields

- `priority` -- P0-P4 where P0 is highest priority (optional)
- `phase` -- execution phase number for parallel grouping (optional). Tasks in the same phase are
  safe to run concurrently. Phases execute sequentially.
- `assigned_to` -- actor (human or agent) working on the task (optional)

### Links

Links connect tasks to other tasks or to artifacts.

```sh
cargo run -- task link <task-id> <rel> <target-task-id>              # task-to-task
cargo run -- task link <task-id> <rel> <target-artifact-id> --artifact  # task-to-artifact
```

Valid relation types:

- `blocks` -- this task blocks the target
- `blocked-by` -- this task is blocked by the target
- `relates-to` -- general association
- `child-of` -- this task is a child of the target (e.g., a task that implements a spec)
- `parent-of` -- the target is a child of this task

### Metadata

Freeform key-value pairs.

```sh
cargo run -- task meta set <id> <key> <value>
cargo run -- task meta get <id> <key>
```

### Tags

```sh
cargo run -- task tag <id> <tag>
cargo run -- task untag <id> <tag>
```

## Iterations

Iterations group tasks into an execution plan. They separate "how to execute" from the spec
("what to build").

### Create

```sh
cargo run -- iteration create "<title>"
cargo run -- iteration create "<title>" --description "<desc>" --tags <tags>
```

Output: `Created iteration <id>` -- extract the ID from this line.

### Show

```sh
cargo run -- iteration show <id>          # human-readable
cargo run -- iteration show --json <id>   # structured JSON
```

### List

```sh
cargo run -- iteration list                 # active only
cargo run -- iteration list --all           # include resolved
cargo run -- iteration list --status <s>    # filter: active, completed, failed
cargo run -- iteration list --json          # structured JSON
```

### Update

```sh
cargo run -- iteration update <id> --status completed
cargo run -- iteration update <id> --title "<new title>"
```

Setting status to `completed` or `failed` automatically resolves the iteration.

### Add / Remove Tasks

```sh
cargo run -- iteration add <iteration-id> <task-id>      # add a task
cargo run -- iteration remove <iteration-id> <task-id>   # remove a task
```

### Graph

Visualize the phased execution plan:

```sh
cargo run -- iteration graph <id>          # jj-style tree output
cargo run -- iteration graph --json <id>   # structured JSON
```

### Links

```sh
cargo run -- iteration link <id> <rel> <target-id>              # to a task
cargo run -- iteration link <id> <rel> <target-id> --artifact   # to an artifact
```

### Tags

```sh
cargo run -- iteration tag <id> <tag>
cargo run -- iteration untag <id> <tag>
```

### Metadata

```sh
cargo run -- iteration meta set <id> <key> <value>
cargo run -- iteration meta get <id> <key>
```

## Search

```sh
cargo run -- search "<query>"                     # search all entities
cargo run -- search --json "<query>"              # structured JSON
cargo run -- search --include-archived "<query>"  # include archived
```

## ID Extraction

When creating entities, the output format is:

```text
Created artifact <id>
Created task <id>
Created iteration <id>
```

Extract the last word from the output line to get the ID. This ID is used to reference the entity
in subsequent commands (show, update, link, etc.).

## Workflow Patterns

### Creating an iteration from a spec

```sh
# Create the iteration
cargo run -- iteration create "Sprint: Feature X"
# Link it to the source spec
cargo run -- iteration link <iteration-id> child-of <spec-id> --artifact
# Create tasks with phase and priority
cargo run -- task create "Add parser types" --phase 1 --priority 1
cargo run -- task create "Add CLI flag" --phase 1 --priority 2
cargo run -- task create "Integrate parser" --phase 2 --priority 0
# Link tasks to spec and add to iteration
cargo run -- task link <task-id> child-of <spec-id> --artifact
cargo run -- iteration add <iteration-id> <task-id>
# Set blocking dependencies
cargo run -- task link <task-c> blocked-by <task-a>
# View the execution graph
cargo run -- iteration graph <iteration-id>
```

### Finding tasks linked to a spec

Use `cargo run -- task list --json` and inspect the `links` array for entries referencing the
artifact ID, or use `cargo run -- search "<spec title>"` to find related entities.
