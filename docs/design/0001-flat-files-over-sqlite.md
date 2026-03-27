---
id: "0001"
title: Flat Files Over SQLite for Storage
status: active
tags: [storage, architecture]
created: 2026-03-26
---

# ADR-0001: Flat Files Over SQLite for Storage

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Store tasks as TOML files and artifacts as Markdown with YAML frontmatter in a flat directory structure.

## Context

gest needs a storage layer that satisfies three constraints:

1. **Committability** — Users who check artifacts into version control (`.gest/` in-repo mode) need human-readable
   diffs and clean merges. Binary formats like SQLite produce opaque blobs that defeat this goal.
2. **Concurrent access** — Multi-agent workflows where several agents read and write different entities simultaneously
   need to avoid lock contention. Flat files are naturally concurrent when agents work on different entities.
3. **Greppability** — Flat files are searchable with standard Unix tools, which aligns with gest's philosophy of
   staying close to the filesystem.

## Decision

All data is stored as flat files in a two-directory structure:

```text
<data_dir>/
  tasks/
    <id>.toml
    archive/
      <id>.toml
  artifacts/
    <id>.md
    archive/
      <id>.md
```

**Tasks** use TOML — a natural fit for structured key-value data with typed fields (status, tags, links, metadata).

**Artifacts** use Markdown with YAML frontmatter — the universally expected format for documents with metadata. The body
is everything after the closing `---`.

**Search** reads files in parallel using rayon for in-memory matching. No database, no full-text search index. Files are
naturally greppable.

**Archive** directories hold archived items. `list` skips them by default; `list --all` includes them.
`show` reaches any item by ID regardless of archive status.

**Alternatives considered:**

- *SQLite* — better query performance at scale, but binary format conflicts with committability and concurrent
  access goals.
- *Single JSON/TOML file* — simpler implementation, but concurrent writes require file-level locking
  and diffs are noisy.
- *SQLite + export* — SQLite as primary with a `dump` command for diffable output. Adds complexity without solving
  concurrent access.

## Dependencies

| Dependency | Version | Purpose                            |
|------------|---------|------------------------------------|
| toml       | 1       | Task serialization/deserialization |
| yaml_serde | 0.10    | Artifact frontmatter parsing       |
| rayon      | 1       | Parallel file search               |

## Consequences

### Positive

- Files are committable, diffable, and greppable with standard tools
- No database lock contention during concurrent agent access
- Zero runtime dependencies beyond the filesystem
- Users can manually inspect and edit files if needed

### Negative

- No indexed queries — search is O(n) across all files
- No referential integrity enforcement — links are pointers validated at read time
- Large stores (thousands of files) may see slower list/search performance than SQLite
