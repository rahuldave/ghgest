---
id: "0014"
title: "Atoms/Molecules/Views UI Architecture"
status: active
supersedes: "0010"
tags: [cli, ui, architecture]
created: 2026-04-06
---

# ADR-0014: Atoms/Molecules/Views UI Architecture

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

Supersedes [ADR-0010: Atomic UI Architecture](0010-atomic-ui-architecture.md).

## Summary

Reorganize the UI module into three strict composition layers — **atoms**,
**molecules**, and **views** — replacing the atoms/composites/layout/views
structure from ADR-0010. Each layer has a single purpose and a clear
composition direction: views use molecules, molecules use atoms, atoms use
nothing. Theme tokens live in a single consolidated `style` module instead of
the old `theming` submodule.

## Context

ADR-0010 established a four-layer structure (atoms / composites / layout /
views) with the goal of making composition explicit. In practice, the
`composites/` layer accumulated both small reusable widgets (status badges,
field lists) and large command-specific renderers (task detail, iteration
graph). The `layout/` module sat awkwardly beside them — layout primitives
were supposed to be usable at any level, but this leaked layout concerns into
what should have been presentation types.

Several pain points emerged:

- **Blurred boundaries** — "is this thing a composite or a view?" came up in
  almost every code review. The distinction between "reusable widget" and
  "command-specific renderer" is an axis the old model didn't encode.
- **Layout split across modules** — terminal width detection lived in
  `layout`, but composites and views also needed it. Callers ended up
  reaching into `layout` from every other module.
- **Theme ceremony** — every atom/composite/view took a `&Theme` parameter,
  threading it through constructors. Most call sites pulled from the same
  global.

The Atomic Design vocabulary (atoms → molecules → organisms/views) is an
industry-standard framing that captures the "reusable widget" vs
"command-specific renderer" distinction directly: molecules are the reusable
widgets, views are the page-level outputs.

## Decision

Adopt a three-layer structure under `src/ui/components/`:

### Atoms (`src/ui/components/atoms/`)

Smallest reusable display primitives. Each atom renders a single visual
concept with a theme-token style attached.

Current atoms: `badge`, `block`, `column`, `icon`, `id`, `indent`, `label`,
`separator`, `tag`, `title`, `value`.

Atoms import from nothing inside `ui::components` — only `style` and stdlib.

### Molecules (`src/ui/components/molecules/`)

Mid-level composable widgets that combine atoms into small, reusable display
units. A molecule is something multiple views would reach for — not
something built for a single command.

Current molecules: `banner`, `empty_list`, `error_message`, `field_list`,
`grid`, `grouped_list`, `indicators`, `row`, `status_badge`, `success_message`.

Molecules import from `atoms` and `style` only.

### Views (`src/ui/components/views/`)

Page-level renderers, one per CLI command output shape. Views assemble
molecules (and atoms where needed) into the final output a user sees. Each
`view` corresponds to one or more commands.

Current views: `artifact_detail`, `artifact_list`, `artifact_list_row`,
`iteration_detail`, `iteration_graph`, `iteration_list`,
`iteration_list_row`, `task_detail`, `task_list`, `task_list_row`,
`project_list_row`, `search_result`, `search_results`, `note_change`,
`state_change`, `tag_change`, `meta_get`, `meta_set`, `undo`, `update`,
`create`, `link`.

### Supporting modules

- **`ui/style.rs`** — consolidated theme tokens and color palette resolution,
  replacing the multi-file `ui/theming/` module. Provides `style::global()`
  that returns the theme built from the user's color config.
- **`ui/markdown.rs`** — standalone markdown renderer that pulls theme and
  terminal width from globals, taking only the markdown string as input.
- **`ui/json.rs`** — JSON output formatter for `--json` flags.

## Consequences

### Positive

- **One obvious home for each concept** — "is it used by multiple views?" is
  an easier question than "is it a composite?". Molecules are the reusable
  layer; views are command-specific.
- **Cleaner imports** — atoms can't reach up to views; views can't reach
  sideways into other views. The dependency graph is a strict tree.
- **Theme ceremony eliminated** — components pull from `style::global()`
  instead of threading `&Theme` through every constructor.
- **Terminal detection colocated** — `markdown::render(text)` detects width
  internally, removing the need for a separate `layout` module.
- **Familiar vocabulary** — Atomic Design is a well-known framing, making
  the structure immediately understandable to new contributors.

### Negative

- **No "organism" layer** — the full three-tier Atomic Design model has
  organisms between molecules and views. We collapsed that into "molecules
  or views" because the distinction was underused. If future growth needs
  it, adding an `organisms/` tier is non-breaking.
- **Global theme access** — `style::global()` is an implicit dependency that
  tests must set up (or rely on defaults). In practice this is fine because
  `Display` impls render the same strings regardless of color state.
- **Larger migration cost** — every command handler and output path in the
  codebase had to be updated. This was a one-time cost, absorbed as part of
  the v0.5.0 rewrite.

## References

- [ADR-0010]: Atomic UI Architecture (superseded)
- [ADR-0006]: UI Components as Display+Write Types (superseded by ADR-0010)

[ADR-0010]: https://github.com/aaronmallen/gest/blob/main/docs/design/0010-atomic-ui-architecture.md
[ADR-0006]: https://github.com/aaronmallen/gest/blob/main/docs/design/0006-ui-components-as-display-types.md
