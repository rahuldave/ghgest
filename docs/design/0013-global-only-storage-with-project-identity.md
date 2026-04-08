---
id: "0013"
title: "Global-Only Storage with Project Identity"
status: active
tags: [storage, architecture, database]
created: 2026-04-06
supersedes: ["0001", "0012"]
---

# ADR-0013: Global-Only Storage with Project Identity

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

Supersedes [ADR-0001] (Flat Files Over SQLite) and [ADR-0012] (SQLite for the
Event Store).

## Summary

Use a single global SQLite-backed store as gest's source of truth for all
project and operational data. Identify projects by an explicit ID committed to
the repository, and treat `.gest/` files as an automatically synchronized
on-disk projection of the database rather than a live storage mode.

## Context

ADR-0001 made in-repo flat files the source of truth for entity data, while
ADR-0012 introduced SQLite only for operational state. That split breaks down
once the same project is used from multiple worktrees or other parallel
checkouts.

The problem is not simply that SQLite is more capable than flat files. The
actual issue is identity and coordination:

- A project cannot be identified by its current working directory when one
  logical project may exist in multiple worktrees.
- Keeping state inside each checkout as the source of truth creates multiple
  uncoordinated copies of the same project's data.

The decision from [discussion #28][Discussion #28] is to make the global store
authoritative, anchor projects with an explicit committed ID, and keep `.gest/`
as an automatically synchronized projection of the database so that project
state still travels with the repository.

## Decision

Adopt a single SQLite-backed global store for all gest data:

- **Authoritative store** — all entity and operational data lives in the global
  database at `<data_dir>/gest.db`.
- **Project identity** — each repository carries a stable project ID in
  `gest.toml`; gest uses that ID, not the checkout path, to resolve project
  state.
- **No local live mode** — `.gest/` directories are no longer treated as an
  authoritative storage backend; the database remains the source of truth.
- **Automatic synchronization** — gest reads `.gest/` on process start
  (importing into the database) and writes on exit (exporting from the
  database). There are no explicit `gest export` / `gest import` commands.
- **Unified persistence model** — the event store exception from ADR-0012 is
  removed; entity and operational data share the same storage backend.

### Why SQLite?

SQLite gives gest one embedded database for both entity and operational data.
The primary reason for this ADR is not a specific driver choice; it is
establishing one authoritative store that is independent of checkout location.

## Consequences

### Positive

- One source of truth for a project across worktrees and other parallel
  checkouts
- Stable project identity that is independent of checkout path
- A single storage model for entity and operational data
- `.gest/` continues to ride alongside the code via automatic sync, so project
  state remains version-controlled with the repository
- SQLite provides transactional, queryable storage for the unified data model

### Negative

- Additional database dependency and operational complexity compared to a
  flat-file-only model
- Migration system must be maintained as schema evolves
- Existing local-mode workflows need a migration path into the global store

### Migration Path

- v0.5.0 moves all authoritative state into the global database
- Existing `.gest/` directories are imported into the global store during
  migration
- The old local live storage path (`store::artifact`, `store::task`, etc.) is
  removed entirely
- `.gest/` output is automatically refreshed by gest on process start and exit

## References

- [ADR-0001]: Flat Files Over SQLite for Storage (superseded)
- [ADR-0012]: SQLite for the Event Store (superseded)
- [Discussion #28]: Global-Only Storage with Project Identity

[ADR-0001]: https://github.com/aaronmallen/gest/blob/main/docs/design/0001-flat-files-over-sqlite.md
[ADR-0012]: https://github.com/aaronmallen/gest/blob/main/docs/design/0012-sqlite-for-the-event-store.md
[Discussion #28]: https://github.com/aaronmallen/gest/discussions/28
