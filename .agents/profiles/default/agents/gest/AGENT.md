---
name: gest
description: Gest CLI reference for managing artifacts, tasks, and iterations. Use when skills need to create, read, or link entities.
tools: Bash, Read
model: haiku
---

# Gest CLI Agent

Reference agent for interacting with gest -- the project's artifact, task, and iteration store.

As of v0.5.0, gest stores all entity data in a local SQLite database at `<data_dir>/gest.db`
(via libsql). For projects initialized with `gest init --local`, a bidirectional sync layer
also mirrors the database to a `.gest/` directory as JSON and Markdown so the data can be
committed alongside source code — but the database is always the source of truth.

All commands in this reference use `gest` as the invocation prefix so you can exercise
an in-development build; substitute `gest` directly when running against an installed binary.
No special environment variables are required — the project is resolved automatically from the
current working directory.

## Entity Model

- **Artifacts** -- Markdown documents (specs, ADRs, RFCs). Categorized by tag, not by a
  dedicated `type` field (removed in v0.5.0).
- **Tasks** -- Actionable work items with status, links, priority, phase, and assignment.
- **Iterations** -- Execution plans that group tasks into phased, parallelizable work.

IDs are 32-character lowercase alphabetic strings using the `[k-z]` alphabet. Prefix matching
works -- you can use a shorter prefix (typically the first 8 characters) if it's unambiguous.

## Command Aliases

All entity types support these subcommand aliases:

| Command  | Aliases |
|----------|---------|
| `create` | `new`   |
| `list`   | `ls`    |
| `show`   | `view`  |
| `update` | `edit`  |
| `remove` | `rm`    |

Note subcommands also have aliases: `note list` → `ls`, `note show` → `view`.

Top-level aliases: `gest u` → `gest undo`, `gest s` → `gest serve`, `gest a` → `gest artifact`,
`gest t` → `gest task`, `gest i` → `gest iteration`.

## Machine-Readable Output

All mutation commands support `--json` (full JSON output) and `-q`/`--quiet` (print only the
entity short ID on create/mutate). Read commands like `meta get` support `--json` and `--raw`
(bare value, no styling).

## Stdin Piping

When `--description` (tasks), `--body` (artifacts/notes), or `--source` is omitted and stdin is
a pipe, the piped content is used as the body.

## Batch Creation

Use `--batch` on `task create` or `artifact create` to read NDJSON from stdin (one object per
line).

## Artifacts

Artifact categorization is tag-driven. Use tags like `spec`, `adr`, `rfc`, `note` to categorize
and filter with `--tag`.

### Create

The title is a **positional argument**, not a flag. `-t` is `--tag`, not `--title`.

```sh
gest artifact create "<title>" --tag spec --tag "<area>" --source <path>
gest artifact create "<title>" --tag adr --tag "<area>" --body "<inline body>"
```

Options: `-t, --tag <TAG>` (repeatable), `-m, --metadata <JSON>` (JSON object),
`-b, --body <BODY>`, `-s, --source <FILE>`, `-j, --json`, `-q, --quiet`,
`-i, --iteration <ID>` (add to iteration), `--batch` (NDJSON from stdin).

Tags use bare format (no namespace prefixes). Include area tags (`cli`, `config`, `docs`,
`store`, `web`, `ui`) and the artifact category tag (`spec`, `adr`, `rfc`, `note`).

### Show

```sh
gest artifact show <id>          # human-readable
gest artifact show <id> --json   # structured JSON
```

### List

```sh
gest artifact list                 # active only
gest artifact list --all           # include archived
gest artifact list --archived      # only archived
gest artifact list --tag <tag>     # filter by tag (use spec, adr, etc.)
gest artifact list --json          # structured JSON
```

### Update

`-T, --title <TITLE>` (capital T) sets the title on an existing artifact. `-t` on update,
like on create, is `--tag`.

```sh
gest artifact update <id> -T "<new title>"
gest artifact update <id> --body "<new body>"
gest artifact update <id> --edit        # open $EDITOR on the current body
gest artifact update <id> --tag <tag>   # replace all tags
```

### Archive

```sh
gest artifact archive <id>
```

### Tags

```sh
gest artifact tag <id> <tag>
gest artifact untag <id> <tag>
```

### Notes

```sh
gest artifact note add <artifact-id> --body "<text>"
gest artifact note add <artifact-id> --agent <name> --body "<text>"
gest artifact note list <artifact-id>
gest artifact note show <artifact-id> <note-id>
gest artifact note update <artifact-id> <note-id> --body "<new text>"
gest artifact note delete <artifact-id> <note-id>
```

### Metadata

```sh
gest artifact meta set <id> <key> <value>
gest artifact meta get <id> <key>
```

## Tasks

### Create

Title is a positional argument.

```sh
gest task create "<title>" --description "<desc>" --tag enhancement --tag cli
gest task create "<title>" --status open --tag "<area>"
gest task create "<title>" --priority 1 --phase 2 --assigned-to agent-1
```

Options: `--description`, `--status`, `-t, --tag <TAG>` (repeatable),
`-m, --metadata <JSON>`, `--priority <0-4>`, `--phase <number>`, `--assigned-to <actor>`,
`-j, --json`, `-q`, `-i, --iteration <id>`, `-l, --link <rel>:<target_id>` (repeatable),
`--batch` (NDJSON from stdin).

Tags use bare format (no namespace prefixes). See `docs/process/labels.md` for the tag
vocabulary.

### Show

```sh
gest task show <id>          # human-readable
gest task show <id> --json   # structured JSON
```

### List

```sh
gest task list                           # active only
gest task list --all                     # include resolved
gest task list --status <status>         # filter by status
gest task list --tag <tag>               # filter by tag
gest task list --assigned-to <name>      # filter by assignee
gest task list --json                    # structured JSON
```

### Update

`-T, --title` (capital T) is the title flag; `-t` is `--tag`.

```sh
gest task update <id> --status <status>
gest task update <id> -T "<new title>"
gest task update <id> --priority 0
gest task update <id> --phase 3
gest task update <id> --assigned-to agent-2
```

Setting status to `done` or `cancelled` resolves the task; setting it back to `open` or
`in-progress` reopens it.

### Shortcuts

```sh
gest task complete <id>                     # mark done
gest task cancel <id>                       # mark cancelled
gest task claim <id> --agent <name>        # claim for an agent (set to in-progress)
gest task block <id> <target-id>            # task blocks target task
gest task block <id> <target-id> --artifact # task blocks artifact
```

### Task Fields

- `priority` -- P0-P4 where P0 is highest priority (optional)
- `phase` -- execution phase number for parallel grouping (optional). Tasks in the same phase
  are safe to run concurrently. Phases execute sequentially.
- `assigned_to` -- actor (human or agent) working on the task (optional)

### Links

Links connect tasks to other tasks or to artifacts.

```sh
gest task link <task-id> <rel> <target-task-id>               # task-to-task
gest task link <task-id> <rel> <target-artifact-id> --artifact # task-to-artifact
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
gest task meta set <id> <key> <value>
gest task meta get <id> <key>
```

### Notes

Notes are timestamped, attributed entries for recording decisions, progress updates, and
observations.

```sh
# Add a note (human author from git config)
gest task note add <task-id> --body "<text>"
# Add an agent-attributed note
gest task note add <task-id> --agent <name> --body "<text>"
# List notes
gest task note list <task-id>
gest task note list <task-id> --json
# Show a single note
gest task note show <task-id> <note-id>
# Update a note
gest task note update <task-id> <note-id> --body "<new text>"
# Delete a note
gest task note delete <task-id> <note-id>
```

Author resolution: `--agent <name>` sets `author_type: agent`. Without `--agent`, author comes
from `git config user.name` / `user.email` with `author_type: human`. Notes appear in
`task show` output and `task show --json`.

### Tags

```sh
gest task tag <id> <tag>
gest task untag <id> <tag>
```

## Iterations

Iterations group tasks into an execution plan. They separate "how to execute" from the spec
("what to build").

### Create

```sh
gest iteration create "<title>"
gest iteration create "<title>" --description "<desc>" --tag <tag>
```

Output: `created iteration  <id>` -- the last whitespace-delimited token is the ID.

### Show

```sh
gest iteration show <id>
gest iteration show <id> --json
```

### List

```sh
gest iteration list                    # active only
gest iteration list --all              # include resolved
gest iteration list --status <s>       # filter: active, completed, cancelled
gest iteration list --json
```

### Update / Complete / Cancel

```sh
gest iteration update <id> --status completed
gest iteration update <id> -T "<new title>"
gest iteration complete <id>              # shortcut: mark completed
gest iteration cancel <id>                # shortcut: mark cancelled
gest iteration reopen <id>                # shortcut: move back to active
```

### Add / Remove Tasks

```sh
gest iteration add <iteration-id> <task-id>
gest iteration remove <iteration-id> <task-id>
```

### Graph

Visualize the phased execution plan:

```sh
gest iteration graph <id>          # jj-style tree output
gest iteration graph <id> --json   # structured JSON
```

### Links

```sh
gest iteration link <id> <rel> <target-id>              # to a task
gest iteration link <id> <rel> <target-id> --artifact   # to an artifact
```

### Tags

```sh
gest iteration tag <id> <tag>
gest iteration untag <id> <tag>
```

### Metadata

```sh
gest iteration meta set <id> <key> <value>
gest iteration meta get <id> <key>
```

### Orchestration

These commands support multi-agent execution of iteration phases.

#### Status

Check iteration progress (active phase, task counts, assignees):

```sh
gest iteration status <id>
gest iteration status <id> --json
```

JSON output includes: `active_phase`, `total_phases`, `phase_progress` (`done`/`total`),
`blocked`, `in_progress`, `assignees`, `overall_progress` (`done`/`total`).

#### Next

Find and optionally claim the next available task in the iteration:

```sh
gest iteration next <id>                          # show next task
gest iteration next <id> --claim --agent <name>   # atomically claim it
gest iteration next <id> -q                       # print only the short ID
gest iteration next <id> --json                   # structured JSON
```

Exits with code **75** (`EX_TEMPFAIL`) when no tasks remain — an idle signal, not an error. Every
other failure maps to a specific sysexits code (see the [Exit Codes](#exit-codes) section below).

Options: `--claim` (set task to in-progress), `--agent <name>` (set assigned_to).

#### Advance

Move to the next phase once the current phase is complete:

```sh
gest iteration advance <id>           # advance if phase is done
gest iteration advance <id> --force   # advance past stuck tasks
```

## Projects

Projects are rows in the `projects` table, keyed on root path. `gest init` creates the row;
the `project` command inspects and manages it.

```sh
gest project              # show the current project
gest project list         # list every known project
gest project attach <id>  # attach the current directory as a workspace
gest project detach       # detach the current directory
```

## Migrate

Import legacy v0.4.x flat-file data into the current SQLite database:

```sh
gest migrate --from v0.4                 # auto-discover .gest/
gest migrate --from v0.4 --path ~/old    # explicit path
```

## Cross-Entity Tagging

Tag, untag, and list tags across all entity types without knowing the entity type in advance.

### Add / Remove Tags

```sh
gest tag add <id> <tags...>      # add tags (space-separated)
gest tag remove <id> <tags...>   # remove tags
```

The ID prefix is resolved across tasks, artifacts, and iterations. If the prefix matches
multiple entity types, an error is returned with disambiguation guidance.

### List Tags

```sh
gest tag list                    # all tags
gest tag list --task             # only task tags
gest tag list --artifact         # only artifact tags
gest tag list --iteration        # only iteration tags
```

Flags can be combined.

## Undo

Reverse the most recent mutating command(s) by replaying a database transaction log in
reverse. Every mutating CLI command is wrapped in a database transaction whose row-level
changes are captured in the `transactions` and `transaction_events` tables. Non-mutating
commands (show, list, search) are not recorded.

```sh
gest undo       # undo last command
gest undo 3     # undo last 3 commands
```

Undo applies the inverse of each recorded change: inserts become deletes, updates restore the
captured before-row, and deletes re-insert the captured row. The undo command itself is not
recorded in the log, so repeated calls walk backwards through history.

## Search

Cross-entity search with filter prefixes (`is:`, `tag:`, `status:`) and free text. The
`type:` filter was removed in v0.5.0 -- use `tag:<category>` instead.

```sh
gest search "<query>"                  # search all entities
gest search "<query>" --json           # structured JSON
gest search "<query>" --all            # include archived/resolved
gest search "is:task tag:urgent"       # combine filters
gest search "is:artifact tag:spec"     # artifacts tagged spec (replaces type:spec)
```

## ID Extraction

When creating entities, the output format is:

```text
  ✓  created artifact  <id>
  ✓  created task  <id>
  ✓  created iteration  <id>
```

Extract the last whitespace-delimited token from the "created ..." line to get the short ID,
or pass `-q` to get just the short ID. Use `--json` and parse `.id` for the full 32-character
ID.

## Workflow Patterns

### Creating an iteration from a spec

```sh
# Create the iteration
iter_id=$(gest iteration create "Sprint: Feature X" -q)
# Link it to the source spec
gest iteration link $iter_id child-of <spec-id> --artifact
# Create tasks with phase, priority, and tags
a=$(gest task create "Add parser types" --phase 1 --priority 1 --tag enhancement --tag store -q)
b=$(gest task create "Add CLI flag" --phase 1 --priority 2 --tag enhancement --tag cli -q)
c=$(gest task create "Integrate parser" --phase 2 --priority 0 --tag enhancement --tag cli -q)
# Link tasks to spec and add to iteration
for t in $a $b $c; do
  gest task link $t child-of <spec-id> --artifact
  gest iteration add $iter_id $t
done
# Set blocking dependencies
gest task link $c blocked-by $a
# View the execution graph
gest iteration graph $iter_id
```

### Finding tasks linked to a spec

Use `gest task list --json` and inspect the `links` array for entries referencing the
artifact ID, or use `gest search "<spec title>"` to find related entities.

## Exit Codes

Gest follows the BSD `sysexits.h` convention. Every CLI failure maps to one of these codes, so
scripts and agents can branch on `$?` alone without parsing stderr:

| Code | Name            | Meaning                                         |
|------|-----------------|-------------------------------------------------|
| 0    | —               | Success                                         |
| 64   | `EX_USAGE`      | Command-line usage error                        |
| 65   | `EX_DATAERR`    | User-supplied data was malformed                |
| 66   | `EX_NOINPUT`    | Referenced entity was not found                 |
| 69   | `EX_UNAVAILABLE`| Resource not in the required state              |
| 70   | `EX_SOFTWARE`   | Internal software error                         |
| 74   | `EX_IOERR`      | Filesystem or database I/O error                |
| 75   | `EX_TEMPFAIL`   | Try again later (e.g., `iteration next` idle)   |
| 78   | `EX_CONFIG`     | Configuration or setup error                    |

The authoritative contract lives in ADR `prsooyor` (_Exit Code Contract for the gest CLI_).

Typical scripting pattern for `iteration next`:

```sh
gest iteration next <id> --claim --agent my-agent --json
status=$?
if [ $status -eq 75 ]; then
  echo "no work available -- idle"
elif [ $status -ne 0 ]; then
  echo "error"
fi
```
