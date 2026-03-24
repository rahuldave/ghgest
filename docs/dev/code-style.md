# Code Style

This document describes the formatting and organization conventions used in Gest. Most formatting is handled
automatically by tools -- you generally just need to run `mise run format` before committing.

The code organization rules are the main things to keep in mind when writing new code. Project-specific rules will be
added to this document as the project matures.

## Running Formatters and Linters

```bash
mise run format       # Format all files
mise run lint         # Lint all files
```

## General Principles

These principles apply across all file types in the project:

| Principle            | Convention                                                    |
|----------------------|---------------------------------------------------------------|
| Maximum line width   | 120 characters                                                |
| Indentation          | 2 spaces (no tabs)                                            |
| Trailing whitespace  | None                                                          |
| Final newline        | All files end with a single newline                           |
| Import/include order | Alphabetical, grouped by origin (stdlib, external, local)     |
| Declaration ordering | Alphabetical within visibility groups (public before private) |

These conventions are enforced by `.editorconfig` and the project's linting tools.

## Code Organization

### Module-Level Ordering

Order items within a module by:

1. **Constants**: All constant declarations first
2. **Type groups**: Each type definition immediately followed by its implementation blocks
3. **Free functions**: Any standalone helper functions after all type groups

Type groups are ordered **alphabetically** by type name, with **public types before private types** (each visibility
group sorted alphabetically).

### Enumeration Variants

Enumeration variants should be ordered **alphabetically**.

### Struct/Record Fields

Struct or record fields should be ordered **alphabetically**, unless field order is semantically significant (e.g.,
positional arguments in a CLI framework).

### Implementation Block Ordering

Order functions and methods within implementation blocks by:

1. **Static vs Instance**: Static/associated functions first, then instance methods
2. **Visibility**: Public items first, then private items
3. **Alphabetical**: Within each group, sort alphabetically

In test modules, fall back to purely alphabetical ordering when the static/instance/public/private structure doesn't
apply. See [testing] for test-specific conventions.

[testing]: https://github.com/aaronmallen/gest/blob/main/docs/dev/testing.md
