---
id: "0003"
title: jj-Style Reverse Hex Change IDs
status: active
tags: [model, id]
created: 2026-03-26
---

# ADR-0003: jj-Style Reverse Hex Change IDs

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

Entity IDs are 16 cryptographically random bytes encoded as 32 lowercase alphabetic characters using jj's reverse hex
alphabet, with shortest-unique-prefix highlighting in list views and prefix resolution for user input.

## Context

A simple approach would be 8-character lowercase alpha IDs (`[a-z]`, 26^8 ≈ 208B possibilities) generated from pure RNG.
While the collision probability is low for small datasets, there is no structural collision protection — a collision is
theoretically possible after just 2 IDs. For a tool managing user data, this is uncomfortable.

jj (Jujutsu VCS) solves this elegantly with "reverse hex" change IDs: random bytes encoded using a 16-letter alphabet
(`zyxwvutsrqponmlk`) that produces all-alphabetic strings visually distinct from hex hashes.

## Decision

**ID generation**: 16 cryptographically random bytes (128 bits of entropy) via the `rand` crate, raising the
birthday-problem collision threshold to ~2^64 IDs.

**Encoding**: reverse hex alphabet where `z=0, y=1, ..., k=15`. Each byte produces two characters, yielding a 32-char
string using only `k` through `z`.

**Display rules**:

- **List views** (artifact list, task list, search): first 8 characters with shortest-unique-prefix highlighting
- **Detail views** (artifact show, task show): full 32-character ID
- **Confirmation messages** (create, update, archive, etc.): first 8 characters

**User interaction**: prefix matching — users type as few characters as needed to uniquely identify an entity. The CLI
resolves prefixes by scanning filenames.

**Filenames** use the full 32-character ID as the stem (e.g., `zyxwvutsrqponmlkzyxwvutsrqponmlk.toml`).

**Alternatives considered:**

- *UUIDs* — standard but long (36 chars with hyphens), not aesthetically suited for a CLI tool.
- *Nanoid* — shorter, but the mixed alphanumeric output is visually noisy.
- *Sequential integers* — simple but leak ordering information and conflict across concurrent writers.
- *Keep 8-char alpha* — works in practice but no collision guarantees at scale.

## Consequences

### Positive

- 128-bit entropy makes collisions practically impossible
- All-alphabetic IDs are visually distinct from git hashes and UUIDs
- Prefix resolution preserves the short-ID UX
- Shortest-unique-prefix highlighting helps users pick the minimal prefix

### Negative

- 32-character filenames are longer than shorter alternatives (e.g., 8-character stems)
- The reverse hex alphabet is unfamiliar — `zyxwvuts` looks like gibberish until you know the convention
