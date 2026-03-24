---
id: "0007"
title: Centralized Theme System Using yansi::Style
status: active
tags: [cli, ui, theme]
created: 2026-03-26
---

# ADR-0007: Centralized Theme System Using yansi::Style

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Introduce a `Theme` struct that maps semantic color token names to `yansi::Style` values, constructed at startup by
merging built-in defaults with user overrides from a `[colors]` config section.

## Context

Without a centralized theme, colors would be hardcoded inline across modules using `yansi::Paint` directly with generic
ANSI color names. This creates two problems:

1. No single source of truth for the color palette — the same intent (e.g., "success") gets expressed as
   `.green().bold()` independently at each callsite.
2. Users cannot customize the color scheme to match their terminal theme or preferences.

Tools like jj solve this with a `[colors]` config section that maps semantic token names to color values.

## Decision

**Theme struct passed through the call chain.** A `Theme` struct holds a `yansi::Style` for each semantic token (log
levels, task statuses, UI elements). It is constructed once during startup by:

1. Starting with hardcoded defaults matching the gest brand palette
2. Merging any user overrides from the `[colors]` config section

Callsites receive the `Theme` (or access it alongside `&Config`) and call a method to get the `yansi::Style` for a given
token, then apply it to their text.

**Brand palette with semantic mappings.** The theme defines a curated color palette (Violet, Azure, Ember, Jade, Rose,
Silver, Pewter) mapped to semantic tokens (`log.error`, `status.done`, `emphasis`, `id_prefix`, etc.). This gives
gest a distinctive visual identity while keeping the mapping user-overridable.

**Config format modeled after jj.** The `[colors]` section uses flat, dot-separated keys. Values can be a hex string
(`"#9448C7"`), a named ANSI color (`"red"`), or an inline table (`{ fg = "#9448C7", bold = true }`).

**Alternatives considered:**

- *Global static theme* — simpler callsite access via a static, but harder to test (global mutable state) and cannot
  vary per-config-load.
- *Keep inline colors, just change values* — doesn't solve user customization and still scatters color decisions across
  files.

## Dependencies

| Dependency | Version | Purpose                      |
|------------|---------|------------------------------|
| yansi      | 1       | Terminal color/style support |

## Consequences

### Positive

- Single source of truth for all color definitions
- Users can fully customize the color scheme via config
- Follows the existing `&Config` threading pattern — no new architectural concepts
- `yansi::Style` is zero-cost when color is disabled (yansi's global condition still applies)

### Negative

- All color callsites must go through Theme instead of using inline Paint calls
- Theme must be accessible wherever colors are emitted, including the logger (which previously had no config dependency)
