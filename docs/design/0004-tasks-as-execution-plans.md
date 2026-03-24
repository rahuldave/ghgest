---
id: "0004"
title: Tasks as Freeform Execution Plans
status: active
tags: [model, tasks, agents]
created: 2026-03-26
---

# ADR-0004: Tasks as Freeform Execution Plans

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Tasks carry a freeform `[metadata]` table and `[[links]]` array that agents use for orchestration —
grouping, sequencing, dependency tracking — without gest imposing a schema.

## Context

gest serves two audiences: humans managing a backlog and AI agents coordinating multi-step work. Agents need to express
concepts like "these five tasks can run in parallel," "don't start this until that's done," and "this
task has complexity estimate X." A rigid schema would either be too restrictive (missing fields agents
need) or too sprawling (fields most
users never touch).

The insight is that tasks already have the right shape for execution plans — they just need extensibility points that
agents can use however they want.

## Decision

**Freeform metadata**: every task has a `[metadata]` TOML table. gest stores and serves it without validation —
convention over schema. Agents can put grouping, sequencing, complexity estimates, or anything else they need. The
`meta get` and `meta set` commands provide dot-path access for reading and writing individual keys.

**Links as pointers**: tasks use `[[links]]` with `rel` (relationship type) and `ref` (target path) fields. Relationship
types are freeform strings with conventions: `blocks`, `blocked-by`, `related`, `parent`, `child`. When checking if a
blocker is actually blocking, gest reads the referenced file and checks its status — data lives in one place, links are
just pointers.

**Auto-archive on terminal status**: when a task reaches `done` or `cancelled`, it is automatically moved to the archive
directory. When an archived task's status is set back to `open` or `in-progress`, it is unarchived. There is no manual
`task archive` command — archiving is a side-effect of status, not a user action.

**Orchestration pattern**: an orchestrating agent reads all tasks linked to a spec artifact, inspects their `[metadata]`
for wave grouping and dependencies, follows `[[links]]` to check blocker status, and dispatches parallel work.

**Alternatives considered:**

- *Typed execution plan entity* — a separate `Plan` entity with wave/dependency schema. Adds complexity and rigidity
  without clear benefit over freeform metadata.
- *Validated metadata schema* — gest enforces expected fields. Breaks when agents evolve their conventions faster than
  gest releases.
- *No metadata at all* — agents encode orchestration in artifact bodies. Loses structured access via `meta get/set`.

## Consequences

### Positive

- Agents can evolve orchestration conventions without gest code changes
- `meta get/set` provides structured access without schema enforcement
- Links + status checks give natural dependency resolution
- Auto-archive keeps the active task list clean

### Negative

- No validation means typos in metadata keys go undetected
- Orchestration conventions are implicit — must be documented outside gest
- Freeform links have no referential integrity beyond read-time status checks
