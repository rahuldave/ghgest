---
id: "0005"
title: CLI Command Structure and Output Conventions
status: active
tags: [cli, ux]
created: 2026-03-26
---

# ADR-0005: CLI Command Structure and Output Conventions

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

gest follows strict CLI conventions: a noun-verb command tree, no silent success, Markdown-formatted output for humans,
and `--json` for machines.

## Context

gest serves two audiences — humans in a terminal and AI agents calling commands programmatically. The
CLI needs consistent conventions that work for both without special-casing either. This decision establishes the shared
contract that all commands follow.

## Decision

**Noun-verb command tree**: top-level nouns (`task`, `artifact`, `config`) with verb subcommands (`create`, `show`,
`list`, `update`, `tag`, `untag`, `archive`, `meta get`, `meta set`). Plus `init` and `search` at the root.

**No silent success**: every mutative command confirms the action on stdout (e.g., `Created task abcdefgh`, `Archived
artifact ijklmnop`). Read commands show the data. This is non-negotiable — agents and humans both need confirmation.

**Flag conventions**:

- Every flag gets a short form unless there's a conflict within the same command
- Long flags use `--kebab-case`
- Repeatable flags for multi-value input: `-m foo=bar -m baz=qux`
- `--json` / `-j` switches output to JSON for machine consumption
- Boolean flags are bare switches, not `--flag/--no-flag` pairs

**Output formatting**:

- `show` commands render Markdown-style detail views: `# Title`, `**Field:** value` pairs, `## Sections`
- `list` commands render Markdown tables with entity-appropriate columns
- Confirmation messages use short-form IDs (first 8 characters)

**Artifact body input** follows a fallback chain: `--file` → `--body` → `$VISUAL`/`$EDITOR` (when stdin is a TTY) →
raw stdin. Title is extracted from the first `#` heading in the body unless `--title` is provided explicitly.

**Alternatives considered:**

- *Verb-noun ordering* (`create task`) — less discoverable, tab completion on the noun is more useful.
- *Silent success with `--verbose`* — agents would need `--verbose` on every call, adding friction.
- *Structured output by default* — JSON-first output alienates human users who are the primary audience.

## Dependencies

| Dependency | Version | Purpose           |
|------------|---------|-------------------|
| clap       | 4       | Argument parsing  |
| tempfile   | 3       | Editor temp files |

## Consequences

### Positive

- Predictable command structure — learn one noun's verbs, know them all
- Agents can parse `--json` output; humans read the default Markdown-style output
- No silent success means scripts can always verify an action occurred
- Editor integration provides a familiar `git commit`-style experience

### Negative

- Every new entity type must implement the full verb set (create, show, list, update, tag, etc.)
- Markdown table output can be wide for entities with many columns
- `--json` must be maintained as a parallel output path for every read command
