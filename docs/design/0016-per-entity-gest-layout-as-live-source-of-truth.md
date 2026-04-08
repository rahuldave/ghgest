---
id: "0016"
title: "Per-entity .gest/ layout as live source of truth"
status: active
tags: [storage, architecture, sync]
created: 2026-04-08
---

# ADR-0016: Per-entity .gest/ layout as live source of truth

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Reverse the direction set by ADR-0013. Treat `.gest/` as the live, version-controlled source of truth for project
state, with SQLite as a derived local query cache. Replace the current shared-array JSON files with per-entity YAML
files in singular type-named directories so that editing one entity touches exactly one file. Sync everything that
represents shared project state, including tables that ADR-0013 left out. Use soft-delete tombstones for deletion. Keep
transactions/undo local-only.

## Context

ADR-0013 ("Global-Only Storage with Project Identity", 2026-04-06) declared SQLite the sole authoritative store and
reframed `.gest/` as an explicit snapshot format with proposed `gest export` / `gest import` commands. That direction
is wrong for gest: `.gest/` is the canonical, version-controlled representation of project state that collaborators
commit, branch, and merge. SQLite is a local query cache. Auto-sync runs on process start (import) and exit (export).
There will be no `gest export` / `gest import` commands.

The current on-disk layout — single shared JSON files (`tasks.json`, `iterations.json`, `task_notes.json`,
`artifact_notes.json`, `artifacts.json`, `artifacts/index.json`) — is the opposite of merge-conflict resilient. Any
change to any task rewrites the entire `tasks.json`, so two collaborators editing different tasks always conflict on
merge. Worse, several SQLite tables (`tags`, `entity_tags`, `relationships`, `iteration_tasks`, `authors`, `events`)
are not synced to disk at all, so a checkout of `.gest/` is not self-contained.

This ADR establishes the per-entity YAML layout, the embedding boundaries for cross-entity data, the deletion model,
and the local-only scope for transactions/undo.

## Decision

### 1. `.gest/` is the live source of truth

`.gest/` is the canonical, version-controlled representation of project state. SQLite (`gest.db`) is a local query
cache derived from `.gest/`. Auto-sync runs on process start (import) and exit (export). There are no explicit
`gest export` / `gest import` commands.

### 2. Per-entity files in singular type-named directories

Every shared-state entity gets its own file. Directory names are singular and type-named:

```text
.gest/
  project.yaml
  task/<id>.yaml
  task/notes/<note_id>.yaml
  iteration/<id>.yaml
  iteration/notes/<note_id>.yaml
  artifact/<id>.md                (markdown body with YAML frontmatter)
  artifact/notes/<note_id>.yaml
  author/<id>.yaml
  tag/<id>.yaml
  relationship/<id>.yaml
  event/<yyyy-mm>/<event_id>.yaml
```

IDs are 32-character `[k-z]` strings (per ADR-0003), which are filesystem-safe in every common environment.

### 3. Embedding vs. per-edge files

Cross-entity data is stored in one of two ways:

- **Embedded in the parent entity file** when one side clearly "owns" the relationship:
  - **Tags on entities** → `tags: [...]` field inside the parent entity file. Editing tags touches one file.
  - **Iteration membership** (`iteration_tasks`) → embedded in the iteration file as a structured `phases:` field
    listing tasks per phase. Iterations own their phase ordering.

- **Per-edge files** when the relationship is symmetric and neither end owns it:
  - **Relationships** → one file per edge at `relationship/<id>.yaml`. Adding a relationship creates a new file and
    touches no existing files.

### 4. Events are append-only with monthly subdirectory sharding

Events live at `event/<yyyy-mm>/<event_id>.yaml`. Each event is its own file, written once and never modified. Monthly
sharding keeps any single directory manageable as event volume grows. This is conflict-free by construction: two
collaborators recording new events on the same day write different files.

### 5. Artifact bodies are markdown with YAML frontmatter

Artifact bodies stay as `artifact/<id>.md`. Their metadata (title, tags, timestamps, etc.) lives in YAML frontmatter at
the top of the file. The current `artifacts/index.json` aggregate is dropped — its data folds into per-artifact
frontmatter.

### 6. YAML format and serialization

All structured entity files use YAML, serialized with the **`yaml_serde` crate** (the successor to the deprecated
`serde_yaml`). YAML schemas are hand-designed (via wrapper structs with explicit field ordering) for human-readable
diffs, not raw serde-default dumps.

### 7. Deletion via soft-delete tombstones

Each entity wrapper struct has a top-level `deleted_at: Option<DateTime<Utc>>` field. When an entity is deleted in
SQLite, the writer rewrites its file with `deleted_at` set; the file stays in `.gest/`. The reader treats files with
`deleted_at` set as deleted (hard-deletes the corresponding SQLite row on import).

Rationale:

- **Conflict-clean.** Two collaborators deleting the same entity merge cleanly (both set `deleted_at`). A
  delete-vs-modify produces a real conflict that should be surfaced.
- **No shared tombstone manifest.** A central `tombstones.yaml` would itself be a merge-conflict hot spot — exactly
  what this ADR avoids.
- **Audit trail.** `git log` over the entity file shows when and by whom it was deleted.
- **Garbage collection** of long-tombstoned files is a separate, deferrable concern.

The alternative considered — file removal with a separate tombstone manifest — was rejected because the manifest
reintroduces a shared write surface.

### 8. Transactions and `transaction_events` are local-only

Transactions and `transaction_events` represent local undo state. They are **not** synced to `.gest/`. Each
collaborator maintains their own undo history. Synchronizing undo across collaborators has no clear semantics (who can
undo whose action? what does "undo" mean across a merge?) and would create coordination problems with no upside.

Events (`events`), in contrast, are synced — they are the audit log of project state changes and belong to the
project, not to any single collaborator.

### 9. `sync_digests` keys by repo-relative path

The `sync_digests` table that caches file content digests is restructured to key by repo-relative path (e.g.
`task/abc.yaml`) rather than absolute filesystem path. This makes the digest cache portable across checkouts and
resilient to layout changes.

## Consequences

### Positive

- Editing one entity touches exactly one file. Conflicts are scoped to genuinely concurrent edits of the same entity.
- `.gest/` is self-contained: a fresh checkout can fully reconstruct project state without any external coordination.
- Adding/removing relationships, events, and entities never modifies existing files, so those operations are
  conflict-free.
- Human-readable YAML diffs in PRs, with hand-tuned field ordering.
- Deletions propagate cleanly across collaborators via tombstones.

### Negative

- Many more files in `.gest/` (one per entity instead of one per type). `git status` and directory listings are
  noisier.
- The `event/` directory grows unboundedly; monthly sharding helps but doesn't eliminate the trend.
- Tombstone files accumulate; garbage collection deferred to a follow-up.
- Schema evolution is harder than with a single binary store: changes to YAML schemas must remain backward-readable,
  or migrations must rewrite every entity file.

### Neutral

- Auto-sync stays. The user-visible behavior is unchanged: gest reads `.gest/` on start, writes on exit. The on-disk
  format changes underneath.

## References

- Spec tyyrtkxq — Merge-conflict-resilient local mode storage layout
- [ADR-0013] — Global-Only Storage with Project Identity
- [ADR-0003] — jj-Style Reverse Hex Change IDs (defines the `[k-z]` ID alphabet relied on for filesystem-safe
  filenames)
- [ADR-0001] — Flat Files Over SQLite for Storage (superseded by ADR-0013)

[ADR-0001]: https://github.com/aaronmallen/gest/blob/main/docs/design/0001-flat-files-over-sqlite.md
[ADR-0003]: https://github.com/aaronmallen/gest/blob/main/docs/design/0003-reverse-hex-change-ids.md
[ADR-0013]: https://github.com/aaronmallen/gest/blob/main/docs/design/0013-global-only-storage-with-project-identity.md
