#!/usr/bin/env bash
# Session-start hook — injects jj-specific VCS context.

set -euo pipefail

read -r -d '' CONTEXT << 'CONTEXT_EOF' || true
## VCS Context

This project is configured to use **Jujutsu (jj)** for version control. This is a colocated jj/git
repository. VCS commands are inline in each skill — do not delegate to a separate agent. Always use
`jj` commands exclusively. Never run raw `git` commands for write operations.
CONTEXT_EOF

printf '{"hookSpecificOutput": {"additionalContext": "%s"}}' \
  "$(printf '%s' "$CONTEXT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/$/\\n/g' | tr -d '\n')"
