---
id: "0009"
title: "Zero-Config Discovery with Fallback Chain (TOML Only)"
status: active
supersedes: "0002"
tags: [config, discovery]
created: 2026-03-29
---

# ADR-0009: Zero-Config Discovery with Fallback Chain (TOML Only)

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

Supersedes [ADR-0002: Zero-Config Discovery with Fallback Chain](0002-zero-config-discovery.md).

## Summary

gest works out of the box with zero configuration. The data directory is discovered automatically via a walk-up search
for `.gest/`, with fallbacks to a hashed external directory. Config files are purely optional overrides and use TOML
exclusively.

## Context

ADR-0002 established the zero-config discovery model and proposed supporting TOML, JSON, and YAML config formats. In
practice, supporting three formats adds maintenance burden (three parsing paths, three sets of edge cases) for minimal
user benefit. TOML is the standard configuration format in the Rust ecosystem, and the `serde_json`/`yaml_serde`
dependencies cited in ADR-0002 as justification for multi-format support are no longer required by other features in the
rewrite.

## Decision

This ADR preserves the data directory discovery and config layering from ADR-0002 with one change: **config files use
TOML only**.

### Data directory discovery

Follows a strict fallback chain:

1. `$GEST_DATA_DIR` environment variable (must be an absolute path to an existing directory)
2. `storage.data_dir` from the config file (must be an absolute path to an existing directory)
3. Walk up from CWD looking for a `.gest/` directory
4. `$XDG_DATA_HOME/gest/<sha256-of-cwd>/`

The data directory is created automatically on first write. `gest init` explicitly creates `.gest/` for in-repo mode.

### Config file discovery

Layered with simple precedence:

- **Global**: `$XDG_CONFIG_HOME/gest/config.toml`
- **Project**: `.gest/config.toml` (in-repo) or `.gest.toml` at project root (external)
- **Env vars**: `GEST_DATA_DIR` and `GEST_CONFIG` override everything

Merge order: global config → project config → env vars.

### Path expansion

Applied at read time: `~`, `$VAR`, and `${VAR}` are expanded in data directory paths from config or env vars.
`config set` stores raw values as-is to preserve portability.

### What changed from ADR-0002

| Aspect        | ADR-0002                          | This ADR         |
|---------------|-----------------------------------|------------------|
| Config format | TOML, JSON, YAML                  | TOML only        |
| Config files  | `config.{toml,json,yaml,yml}`     | `config.toml`    |

All other decisions from ADR-0002 remain unchanged.

## Dependencies

| Dependency | Version | Purpose                                |
|------------|---------|----------------------------------------|
| dir_spec   | 0.5     | XDG base directory resolution          |
| sha2       | 0.11    | Hashing project root for external path |
| toml       | 0.8     | Config file parsing                    |
| typed-env  | 0.3     | Typed environment variable access      |

## Consequences

### Positive

- Zero friction for new users — `gest task create "foo"` just works
- In-repo and external modes coexist without configuration
- Config is predictable — at most three sources with clear precedence
- Single config format eliminates parsing ambiguity and reduces maintenance surface
- TOML aligns with Rust ecosystem conventions (`Cargo.toml`, `rustfmt.toml`, etc.)

### Negative

- Walk-up search adds startup latency (negligible in practice)
- SHA-256 hashing of CWD means moving a project creates a new external data dir
- Users who prefer JSON or YAML must use TOML for gest config
