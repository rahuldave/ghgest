# gest artifact

Create, update, list, and manage artifacts. Artifacts store documents such as specs, ADRs,
RFCs, and notes alongside your project.

## Usage

```text
gest artifact <COMMAND> [OPTIONS]
```

## Subcommands

| Command                        | Aliases | Description                          |
|--------------------------------|---------|--------------------------------------|
| [`archive`](#artifact-archive) |         | Archive an artifact                  |
| [`create`](#artifact-create)   | `new`   | Create a new artifact                |
| [`list`](#artifact-list)       | `ls`    | List artifacts with optional filters |
| [`show`](#artifact-show)       | `view`  | Display an artifact's full details   |
| [`update`](#artifact-update)   | `edit`  | Update an artifact's fields          |
| [`tag`](#artifact-tag)         |         | Add tags to an artifact              |
| [`untag`](#artifact-untag)     |         | Remove tags from an artifact         |
| [`meta`](#artifact-meta)       |         | Read or write metadata fields        |

---

## artifact archive

Move an artifact to the archive by setting its `archived_at` timestamp.

```text
gest artifact archive [OPTIONS] <ID>
```

### Arguments

| Argument | Description                  |
|----------|------------------------------|
| `<ID>`   | Artifact ID or unique prefix |

### Options

| Flag          | Description                |
|---------------|----------------------------|
| `-j, --json`  | Output as JSON             |
| `-q, --quiet` | Print only the artifact ID |

### Examples

```sh
gest artifact archive abc123
```

---

## artifact create

Create a new artifact from inline text, a source file, an editor, or stdin.

When `--body` and `--source` are both omitted and stdin is a terminal, `$EDITOR` opens for
interactive editing. When stdin is a pipe, the piped content is used as the body.

```text
gest artifact create [OPTIONS]
```

### Options

| Flag                              | Description                                                       |
|-----------------------------------|-------------------------------------------------------------------|
| `--batch`                         | Read NDJSON from stdin (one artifact per line)                    |
| `-b, --body <BODY>`               | Body content as an inline string (skips editor and stdin)         |
| `-i, --iteration <ITERATION>`     | Add the artifact to an iteration (ID or prefix)                   |
| `-j, --json`                      | Output the created artifact as JSON                               |
| `-k, --type <KIND>`               | Artifact type (e.g. `spec`, `adr`, `rfc`, `note`)                 |
| `-m, --metadata <METADATA>`       | Key=value metadata pairs (repeatable)                             |
| `-q, --quiet`                     | Print only the artifact ID                                        |
| `-s, --source <SOURCE>`           | Read body content from a file path                                |
| `--tag <TAG>`                     | Tag (repeatable, or comma-separated)                              |
| `-t, --title <TITLE>`             | Artifact title (auto-extracted from first `#` heading if omitted) |

### Examples

```sh
# Create from inline body
gest artifact create -t "Auth Spec" -k spec -b "## Overview\nAuth flow details..."

# Create from a file
gest artifact create -k adr -s docs/decisions/001-storage.md

# Create interactively (opens $EDITOR)
gest artifact create -k rfc --tag "backend,v2"

# Add to an iteration
gest artifact create -t "Sprint 3 Notes" -k note -i iter123

# Pipe body from stdin
echo "# My Spec\nDetails..." | gest artifact create -k spec

# Batch-create artifacts from NDJSON
cat artifacts.ndjson | gest artifact create --batch

# Machine-readable output
gest artifact create -t "Quick note" -k note --json
gest artifact create -t "Quick note" -k note -q
```

---

## artifact list

List artifacts, optionally filtered by type, tag, or archive status.

```text
gest artifact list [OPTIONS]
```

### Options

| Flag                | Description                                         |
|---------------------|-----------------------------------------------------|
| `-a, --all`         | Include archived artifacts alongside active ones    |
| `--archived`        | Show only archived artifacts                        |
| `-j, --json`        | Output as JSON                                      |
| `-k, --type <KIND>` | Filter by artifact type (e.g. `spec`, `adr`, `rfc`) |
| `--tag <TAG>`       | Filter by tag                                       |

### Examples

```sh
# List active artifacts
gest artifact list

# Filter by type
gest artifact list -k spec

# Include archived
gest artifact list --all

# JSON output for scripting
gest artifact list --json
```

---

## artifact show

Display an artifact's full details and rendered body.

```text
gest artifact show [OPTIONS] <ID>
```

### Arguments

| Argument | Description                  |
|----------|------------------------------|
| `<ID>`   | Artifact ID or unique prefix |

### Options

| Flag         | Description                                |
|--------------|--------------------------------------------|
| `-j, --json` | Output as JSON instead of formatted detail |

### Examples

```sh
gest artifact show abc123
gest artifact show abc123 --json
```

---

## artifact update

Update an artifact's title, body, type, or tags.

```text
gest artifact update [OPTIONS] <ID>
```

### Arguments

| Argument | Description                  |
|----------|------------------------------|
| `<ID>`   | Artifact ID or unique prefix |

### Options

| Flag                  | Description                                       |
|-----------------------|---------------------------------------------------|
| `-b, --body <BODY>`   | Replace the body content                          |
| `-j, --json`          | Output as JSON                                    |
| `-k, --type <KIND>`   | Artifact type (e.g. `spec`, `adr`, `rfc`, `note`) |
| `-q, --quiet`         | Print only the artifact ID                        |
| `--tag <TAG>`         | Replace all tags (repeatable, or comma-separated) |
| `-t, --title <TITLE>` | New title                                         |

### Examples

```sh
gest artifact update abc123 -t "Updated Title"
gest artifact update abc123 -k adr --tag "approved,backend"

# Machine-readable output
gest artifact update abc123 -t "New Title" --json
```

---

## artifact tag

Add tags to an artifact, deduplicating with any existing tags.

```text
gest artifact tag [OPTIONS] <ID> [TAGS]...
```

### Arguments

| Argument    | Description                            |
|-------------|----------------------------------------|
| `<ID>`      | Artifact ID or unique prefix           |
| `[TAGS]...` | Tags to add (space or comma-separated) |

### Options

| Flag          | Description                               |
|---------------|-------------------------------------------|
| `-j, --json`  | Output the artifact as JSON after tagging |
| `-q, --quiet` | Output only the artifact ID               |

### Examples

```sh
gest artifact tag abc123 approved reviewed
gest artifact tag abc123 approved,reviewed
```

---

## artifact untag

Remove tags from an artifact.

```text
gest artifact untag [OPTIONS] <ID> [TAGS]...
```

### Arguments

| Argument    | Description                               |
|-------------|-------------------------------------------|
| `<ID>`      | Artifact ID or unique prefix              |
| `[TAGS]...` | Tags to remove (space or comma-separated) |

### Options

| Flag          | Description                                 |
|---------------|---------------------------------------------|
| `-j, --json`  | Output the artifact as JSON after untagging |
| `-q, --quiet` | Output only the artifact ID                 |

### Examples

```sh
gest artifact untag abc123 draft
```

---

## artifact meta

Read or write artifact metadata fields. Metadata uses dot-delimited key paths for nested values.

```text
gest artifact meta <COMMAND>
```

### meta get

Retrieve a single metadata value.

```text
gest artifact meta get [OPTIONS] <ID> <PATH>
```

| Argument | Description                                 |
|----------|---------------------------------------------|
| `<ID>`   | Artifact ID or unique prefix                |
| `<PATH>` | Dot-delimited key path (e.g. `outer.inner`) |

| Flag     | Description                           |
|----------|---------------------------------------|
| `--json` | Output as a JSON object               |
| `--raw`  | Output the bare value with no styling |

### meta set

Set a metadata value. Strings, numbers, booleans, and null are auto-detected.

```text
gest artifact meta set [OPTIONS] <ID> <PATH> <VALUE>
```

| Argument  | Description                                    |
|-----------|------------------------------------------------|
| `<ID>`    | Artifact ID or unique prefix                   |
| `<PATH>`  | Dot-delimited key path (e.g. `config.timeout`) |
| `<VALUE>` | Value to set                                   |

| Flag          | Description              |
|---------------|--------------------------|
| `-j, --json`  | Output as JSON           |
| `-q, --quiet` | Print only the entity ID |

### Examples

```sh
# Set a metadata field
gest artifact meta set abc123 status "approved"

# Read it back
gest artifact meta get abc123 status

# JSON output
gest artifact meta get abc123 status --json

# Raw value (no styling)
gest artifact meta get abc123 status --raw
```
