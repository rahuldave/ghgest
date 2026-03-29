#!/usr/bin/env bash
# Session-start hook — injects git-specific VCS context.

set -euo pipefail

read -r -d '' CONTEXT << 'CONTEXT_EOF' || true
## VCS Context

This project is configured to use **git** for version control. VCS commands are inline in each skill —
do not delegate to a separate agent. Always use standard `git` commands. Do not use `jj` or
`git-butler` commands.
CONTEXT_EOF

printf '{"hookSpecificOutput": {"additionalContext": "%s"}}' \
  "$(printf '%s' "$CONTEXT" | sed 's/\\/\\\\/g; s/"/\\"/g; s/$/\\n/g' | tr -d '\n')"
