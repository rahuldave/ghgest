---
id: "0006"
title: UI Components as Display+Write Types
status: active
tags: [cli, ui, architecture]
created: 2026-03-26
---

# ADR-0006: UI Components as Display+Write Types

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

All user-facing CLI output is produced by dedicated types in `src/ui/` that implement `Display` and `Write`. Command
handlers are thin — they fetch data, construct a UI component, and write it to stdout.

## Context

Without a dedicated UI layer, display logic (table rendering, confirmation messages, detail views, colored ID
formatting) would scatter across command handler files and shared utilities. This makes it hard to maintain consistent
output across commands and difficult to test what gets printed — command handlers end up mixing business logic with
formatting concerns.

## Decision

**UI module structure**:

```text
src/ui.rs                      — module root, yansi init, re-exports
src/ui/utils.rs                — pure formatting helpers (format_id, format_status, shortest_unique_prefixes)
src/ui/components.rs           — module root for components, re-exports
src/ui/components/table.rs     — Table component
src/ui/components/detail.rs    — TaskDetail, ArtifactDetail components
src/ui/components/message.rs   — Confirmation messages (Created, Updated, Archived, Tagged, etc.)
src/ui/components/value.rs     — ConfigValue, TomlValue, YamlValue components
```

**Components as types**: each user-facing output becomes a struct that implements both `Display` (for
simple printing) and `Write` (for writing to any `io::Write` target). Command handlers construct the
component with the data and write it.

**Utils are pure**: formatting helpers like `format_id` and `format_status` return styled strings but never print. They
are consumed by components.

**Boundary rule**: anything that writes to stdout/stderr for the user belongs in `ui`. Data transformation stays in the
command handler or model layer. The logger is a separate concern.

**No command handler imports yansi directly**: all color/style application goes through UI components
and utils, which in turn use the Theme (see ADR-0007).

**Alternatives considered:**

- *Template-based rendering* — too heavy for CLI output, adds a dependency for no real benefit.
- *Keep formatting inline* — leads to inconsistent output as the command set grows.
- *Trait-based rendering* (`Renderable` trait on model types) — couples the model layer to display concerns.

## Consequences

### Positive

- Single place to update output format for all commands
- Components are testable in isolation — construct and assert on the `Display` output
- Command handlers are thin and focused on orchestration
- Consistent output across all commands by construction

### Negative

- More types to maintain — each distinct output pattern is a struct
- Indirection between "what the command does" and "what gets printed"
