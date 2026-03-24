---
id: "0008"
title: Custom Logger with Styled Stderr Output
status: active
tags: [cli, logging]
created: 2026-03-26
---

# ADR-0008: Custom Logger with Styled Stderr Output

## Status

![Active](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

gest uses a custom `log` crate backend that writes styled, right-aligned level prefixes to stderr, with log level
determined by a three-source precedence chain: CLI flags > env var > config file.

## Context

gest needs diagnostic output so developers and users can observe internal operations (config loading, store
reads/writes, ID resolution) when troubleshooting. The standard approach of pulling in a full logging framework
(env_logger, tracing-subscriber) brings either too much configuration surface or dependencies that are overkill for a
CLI tool.

## Decision

**Custom `Log` implementation** in `src/logger.rs` — a self-contained module that implements the `log::Log` trait with
a styled stderr backend. No external logging framework dependency beyond the `log` facade.

**Output format**: right-aligned level prefix (5 chars) with distinct theme colors, followed by the message:

```text
TRACE I'm a trace log
DEBUG I'm a debug log
 INFO I'm an info log
 WARN I'm a warn log
ERROR I'm an error log
```

Output goes to stderr, preserving stdout for command output that agents and scripts parse.

**Three-source level resolution** (first set wins):

1. CLI flag: `-v` = info, `-vv` = debug, `-vvv` = trace
2. `$GEST_LOG_LEVEL`: accepts `error`, `warn`, `info`, `debug`, `trace` (case-insensitive)
3. Config key `log.level`: same values

Default when no source is set: `warn`.

**Level semantics** are strictly enforced by convention:

- **error**: unrecoverable failures
- **warn**: recoverable unexpected conditions
- **info**: high-level milestones (config loaded, store opened, command dispatched)
- **debug**: detailed troubleshooting (paths resolved, merge decisions, ID lookups)
- **trace**: fine-grained flow (function entry/exit, intermediate values)

**Alternatives considered:**

- *env_logger* — popular but opinionated about format and configuration. Doesn't support gest's theme system or
  right-aligned prefixes without significant customization.
- *tracing + tracing-subscriber* — powerful but heavy for a CLI tool with no async runtime or span-based diagnostics.
- *No logging, just `--verbose` print statements* — doesn't compose with the `log` ecosystem and scatters debug output
  across the codebase.

## Dependencies

| Dependency | Version | Purpose        |
|------------|---------|----------------|
| log        | 0.4     | Logging facade |

## Consequences

### Positive

- Zero external logging framework dependencies
- Styled output integrates with the theme system (ADR-0007)
- stderr output preserves stdout for machine-parseable command output
- Three-source precedence gives users flexibility without complexity

### Negative

- Custom implementation means maintaining the backend ourselves
- No structured/JSON log output (out of scope, can be added later)
- No log-to-file support (out of scope for a CLI tool)
