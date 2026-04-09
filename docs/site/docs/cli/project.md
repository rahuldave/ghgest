# gest project

Show or manage the project registration for the current working directory.

In gest v0.5.0, every tracked project is a row in the `projects` table, keyed on
its root path. `gest init` creates the row; `gest project` commands inspect and
modify it.

## Usage

```text
gest project [OPTIONS] [COMMAND]
```

### Options (bare `gest project`)

When run without a subcommand, `gest project` shows details for the current directory's
project.

| Flag     | Description                                                 |
|----------|-------------------------------------------------------------|
| `--json` | Emit output as JSON (only applies to the default show view) |

## Subcommands

| Command  | Aliases | Description                                                        |
|----------|---------|--------------------------------------------------------------------|
| `attach` |         | Attach the current directory to an existing project as a workspace |
| `detach` |         | Detach the current directory from its project                      |
| `list`   | `ls`    | List all known projects                                            |

Running `gest project` without a subcommand shows the current project.

## project list

List every project recorded in the database.

```text
gest project list [OPTIONS]
```

### Options

| Flag          | Description                              |
|---------------|------------------------------------------|
| `--limit <N>` | Cap the number of items returned         |
| `-j, --json`  | Emit output as JSON                      |
| `-q, --quiet` | Suppress normal output                   |
| `-r, --raw`   | Emit script-friendly plain output        |

### Examples

```sh
gest project list
gest project list --json
gest project list --limit 5
```

## project attach

Attach the current directory to an existing project as a secondary workspace.
Useful when you have multiple checkouts of the same repository (for example, jj
workspaces or git worktrees) and want them all to share the same entity data.

```sh
gest project attach <PROJECT-ID>
```

## project detach

Remove the current directory from its project's workspace list. The project
itself is not deleted — only the workspace association.

```sh
gest project detach
```

## Examples

Show the project for the current directory:

```sh
gest project
```

List every project:

```sh
gest project list
```

Attach a secondary checkout:

```sh
cd ../myapp-workspace-2
gest project attach <project-id>
```
