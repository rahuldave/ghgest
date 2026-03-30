# Agent Usage

Gest is designed to work with AI coding agents. Any agent that can run shell commands can use
gest to track tasks, store artifacts, and coordinate execution plans. To see how the gest
project itself uses agents, browse the
[`.agents/` directory](https://github.com/aaronmallen/gest/tree/main/.agents).

## Read the Work Queue

List open tasks to find what needs to be done. Use `--json` for machine-readable output:

```sh
gest task list --json
```

Filter by status, tag, or assignment:

```sh
gest task list --status open --tag api
gest task list --assigned-to agent-1 --json
```

## Track Task Progress

Update a task's status as work progresses:

```sh
gest task update <id> --status in-progress
# ... do the work ...
gest task update <id> --status done
```

Assign a task to the current agent so other agents know it's taken:

```sh
gest task update <id> --assigned-to agent-1
```

## Store Design Documents

Save specs, ADRs, RFCs, and other prose as artifacts. Provide the body inline with `-b` to
avoid opening `$EDITOR`:

```sh
gest artifact create \
  -t "Auth Middleware Design" \
  -b "Token-bucket rate limiting with per-user quotas." \
  -k spec \
  --tags "auth,design"
```

Or import from a file:

```sh
gest artifact create --source design.md --type adr --tags "architecture"
```

Link a task to its source artifact:

```sh
gest task link <task-id> child-of <artifact-id> --artifact
```

## Build Execution Plans

Group related tasks into an iteration with phased execution. Tasks in the same phase can run
in parallel; lower phases execute first:

```sh
# Create tasks with phase assignments
gest task create "Add parser types" --phase 1 --priority 1
gest task create "Add CLI flag" --phase 1 --priority 2
gest task create "Integrate parser" --phase 2 --priority 0

# Set dependencies
gest task link <integrate-id> blocked-by <parser-id>

# Create an iteration and add tasks
gest iteration create "Implement feature X"
gest iteration add <iteration-id> <task-id>

# Visualize the plan
gest iteration graph <iteration-id>
```

## Use JSON Output

Most listing and show commands support `--json` for structured output. This is useful for
agents that need to parse results programmatically:

```sh
gest task show <id> --json
gest task list --json
gest artifact list --json
gest iteration graph <id> --json
gest search "auth" --json
```

## Search Before Creating

Before creating a new task or artifact, search for existing ones to avoid duplicates:

```sh
gest search "rate limiting"
```

Add `--expand` to see full details, or `--json` for machine-readable results:

```sh
gest search "rate limiting" --expand
gest search "rate limiting" --json
```

Resolved tasks and archived artifacts are excluded by default. Pass `--all` to include them:

```sh
gest search "rate limiting" --all
```
