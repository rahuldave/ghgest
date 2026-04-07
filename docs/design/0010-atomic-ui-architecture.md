---
id: "0010"
title: "Atomic UI Architecture"
status: superseded
supersedes: "0006"
superseded_by: "0015"
tags: [cli, ui, architecture]
created: 2026-03-30
---

# ADR-0010: Atomic UI Architecture

## Status

[![Superseded][superseded-badge]][0015]

Supersedes [ADR-0006: UI Components as Display+Write Types](0006-ui-components-as-display-types.md).

## Summary

All user-facing CLI output is produced by `Display` types organized into three composition layers — atoms, composites,
and views — plus a layout module for spatial arrangement. Command handlers construct a view and write it to stdout.

## Context

ADR-0006 established that all CLI output should flow through dedicated `Display`+`Write` types in `src/ui/`, keeping
command handlers thin. As the command set grew, the flat `src/ui/components/` directory accumulated types at very
different levels of abstraction: small reusable primitives (ID formatting, status badges) sat alongside full detail
views
and confirmation messages. This made it unclear which types were meant to be composed into others and which were
top-level outputs, leading to inconsistent composition patterns across commands.

## Decision

The UI module is organized into four layers with a strict composition direction: atoms are consumed by composites,
composites are consumed by views, and layout primitives can be used at any level.

### Atoms (`src/ui/atoms/`)

Smallest reusable display primitives. Each atom renders a single visual concept and accepts theme tokens for styling.

```text
badge.rs      — styled inline label (e.g. status badge)
icon.rs       — single-character semantic icons
id.rs         — formatted entity ID with prefix highlighting
label.rs      — key portion of a key-value pair
separator.rs  — horizontal or inline dividers
tag.rs        — tag chip rendering
title.rs      — section/entity title with styling
value.rs      — value portion of a key-value pair
```

### Composites (`src/ui/composites/`)

Mid-level types that compose atoms into meaningful UI blocks. A composite renders a logical unit of output (a detail
card, a list row, a confirmation message) but does not own the full screen layout.

```text
artifact_detail.rs      — full artifact detail card
artifact_list_row.rs    — single row in an artifact list
banner.rs               — application banner / header
empty_list.rs           — placeholder when a list has no items
error_message.rs        — styled error output
grouped_list.rs         — list with group headings
indicators.rs           — progress/status indicators
iteration_detail.rs     — full iteration detail card
iteration_graph.rs      — iteration dependency graph
iteration_list_row.rs   — single row in an iteration list
search_result.rs        — single search result entry
success_message.rs      — styled confirmation output
task_detail.rs          — full task detail card
task_list_row.rs        — single row in a task list
```

### Views (`src/ui/views/`)

Top-level output types that command handlers construct and write. A view composes composites (and occasionally atoms)
into the complete output for a command. Each view module groups related views for one command noun.

```text
artifact.rs   — create, detail, list, update, tag, untag, archive views
iteration.rs  — create, detail, list, graph, add, remove, link views
log.rs        — logger configuration views
search.rs     — search results views
system.rs     — init, config, version, self-update, generate views
task.rs       — create, detail, list, update, tag, untag, link views
```

### Layout (`src/ui/layout.rs`)

Spatial arrangement primitives — `Column` (vertical stacking) and `Row` (horizontal packing with terminal-width-aware
truncation). Used at any layer to arrange child elements.

### What changed from ADR-0006

| Aspect            | ADR-0006                       | This ADR                                         |
|-------------------|--------------------------------|--------------------------------------------------|
| Module structure  | Flat `src/ui/components/`      | `atoms/`, `composites/`, `views/`, `layout/`     |
| Composition model | Implicit, all types peers      | Explicit three-tier: atom -> composite -> view   |
| Layout primitives | Inline in components           | Dedicated `layout` module                        |
| Command interface | Handler constructs a component | Handler constructs a view                        |

Core principles from ADR-0006 are preserved:

- All output flows through `Display` types in `src/ui/`
- Command handlers are thin — fetch data, construct a view, write it
- No command handler imports `yansi` directly; all styling goes through the Theme (ADR-0007)
- Utils remain pure formatting helpers consumed by atoms and composites

**Alternatives considered:**

- *Keep the flat component directory* — simple but does not scale; unclear which types are building blocks vs top-level
  outputs.
- *Full atomic design (atoms, molecules, organisms, templates, pages)* — five tiers is overkill for a CLI; three tiers
  plus layout captures the meaningful boundaries without unnecessary granularity.

## Consequences

### Positive

- Clear composition direction makes it obvious where new UI types belong
- Views provide a stable interface for command handlers while internals can be refactored freely
- Atoms are small and independently testable
- Layout primitives are reusable across all layers

### Negative

- More modules and files than the flat structure
- Three-tier naming requires contributors to understand the distinction between atoms, composites, and views

[0015]: https://github.com/aaronmallen/gest/blob/main/docs/design/0015-atoms-molecules-views-ui.md
[superseded-badge]:
  https://img.shields.io/badge/0015-black?style=for-the-badge&label=Superseded&labelColor=orange
