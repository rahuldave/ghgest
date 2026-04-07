---
id: "0002"
title: Zero-Config Discovery with Fallback Chain
status: superseded
superseded_by: "0009"
tags: [config, discovery]
created: 2026-03-26
---

# ADR-0002: Zero-Config Discovery with Fallback Chain

## Status

[![Superseded][superseded-badge]][0009]

## Summary

gest works out of the box with zero configuration. The data directory is discovered automatically via a walk-up search
for `.gest/`, with fallbacks to a hashed external directory. Config files are purely optional overrides.

## Context

A hierarchical config discovery approach — scanning dozens of candidate filenames and merging from `$HOME` to CWD —
would make the config system hard to reason about and impossible to explain in one sentence. Most users just want to run
`gest task create "foo"` and have it work.

gest needs to support two modes: **in-repo** (artifacts committed alongside code) and **external** (artifacts stored
outside the repo). The discovery mechanism must choose the right mode automatically while allowing explicit overrides.

## Decision

**Data directory discovery** follows a strict fallback chain:

1. Walk up from CWD looking for a `.gest/` directory → in-repo mode
2. Walk up from CWD looking for a git root (`.git/`) → external mode at `~/.local/share/gest/<sha256-of-git-root>/`
3. No git root → external mode at `~/.local/share/gest/<sha256-of-cwd>/`

The data directory is created automatically on first write. `gest init` explicitly creates `.gest/` for in-repo mode.

**Config file discovery** is layered with simple precedence:

- **Global**: `$XDG_CONFIG_HOME/gest/config.{toml,json,yaml,yml}`
- **Project**: `.gest/config.{toml,json,yaml,yml}` (in-repo) or `.gest.{toml,json,yaml,yml}` at project root (external)
- **Env vars**: `GEST_DATA_DIR` and `GEST_CONFIG` override everything

Merge order: global config → project config → env vars. Config files can be TOML, JSON, or YAML — since `serde_json`
and `yaml_serde` are already required for other features, supporting all three is free.

**Path expansion** is applied at read time: `~`, `$VAR`, and `${VAR}` are expanded in data directory paths from config
or env vars. `config set` stores raw values as-is to preserve portability.

**Alternatives considered:**

- *Hierarchical merge from `$HOME` to CWD* — too complex, hard to debug which config wins.
- *Require explicit init before use* — friction for the common case where external mode is fine.
- *Single config format* — TOML-only would be simpler but forces a format choice on users who prefer YAML/JSON.

## Dependencies

| Dependency | Version | Purpose                                |
|------------|---------|----------------------------------------|
| dir_spec   | 0.5     | XDG base directory resolution          |
| sha2       | 0.11    | Hashing project root for external path |
| typed-env  | 0.3     | Typed environment variable access      |

## Consequences

### Positive

- Zero friction for new users — `gest task create "foo"` just works
- In-repo and external modes coexist without configuration
- Config is predictable — at most three sources with clear precedence
- Path expansion supports use cases like Obsidian vaults and NAS mounts

### Negative

- Walk-up search adds startup latency (negligible in practice)
- SHA-256 hashing of project root means moving a repo creates a new external data dir
- Supporting three config formats means three code paths to maintain

[0009]: https://github.com/aaronmallen/gest/blob/main/docs/design/0009-zero-config-discovery-toml-only.md
[superseded-badge]:
  https://img.shields.io/badge/0009-black?style=for-the-badge&label=Superseded&labelColor=orange
