---
id: "0014"
title: "libsql for Remote Database Support"
status: active
tags: [storage, architecture, database]
created: 2026-04-07
---

# ADR-0014: libsql for Remote Database Support

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Use `libsql` instead of `rusqlite` as the SQLite driver so gest can keep one
driver model for both the embedded local database and future remote database
connections.

## Context

ADR-0013 establishes the storage architecture: one authoritative global SQLite
database keyed by an explicit project identity, with `.gest/` files treated as
an import/export format rather than a live storage mode.

That ADR intentionally does not choose a specific SQLite driver. At the driver
level, `rusqlite` works well for local embedded storage, but it stops at the
boundary of a local SQLite file. Supporting remote databases on top of that
would require a separate connection model, separate error handling, and likely
separate repository behavior for local versus remote execution.

We want remote database support to be an extension of the same storage model,
not a second storage stack.

## Decision

Switch the SQLite driver from `rusqlite` to `libsql`.

- **Local mode** — gest continues to use a local SQLite database at
  `<data_dir>/gest.db`.
- **Remote mode** — gest can connect to a remote database through the same
  driver family rather than introducing a second database stack.
- **Shared storage layer** — repository, migration, and query code target one
  database API across local and remote modes.
- **No architectural change** — ADR-0013's global-only storage model and
  project identity rules remain in effect.

## Dependencies

| Dependency | Version | Purpose                                        |
|------------|---------|------------------------------------------------|
| libsql     | 0.9     | SQLite driver for local and remote connections |

Removed dependencies: `rusqlite`.

## Consequences

### Positive

- One database driver model for local and remote storage
- A clear path to remote database support without redesigning the repository
  layer around a second backend
- Better alignment between today's embedded database usage and future remote
  deployment needs

### Negative

- Adds dependency on `libsql` and its connection model
- Couples remote support to the capabilities and semantics of the `libsql`
  ecosystem
- Existing `rusqlite`-specific code must be migrated to the new API

## Future Work

- Define which remote database configurations gest will support first
- Specify authentication, configuration, and failure semantics for remote
  connections
- Validate migration and transaction behavior in both local and remote modes

## References

- [ADR-0013]: Global-Only Storage with Project Identity

[ADR-0013]: https://github.com/aaronmallen/gest/blob/main/docs/design/0013-global-only-storage-with-project-identity.md
