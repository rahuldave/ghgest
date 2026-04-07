---
name: write-spec
description: "Draft a product spec and store it as a gest artifact (e.g. /write-spec \"user authentication\")."
args: "<topic and approach>"
---

# Write Spec

Draft a product spec and store it as a gest artifact.

## Instructions

### 1. Draft the Spec

Write a spec using this structure. Omit any section that doesn't apply.

```markdown
# Spec: <Title>

## Problem Statement
What problem does this solve? Who is affected?

## Proposed Solution
How it works from the user's perspective. Focus on behavior, not implementation.

## Scope

### In Scope
- ...

### Out of Scope
- ...

## Acceptance Criteria
- [ ] Measurable outcome 1
- [ ] Measurable outcome 2

## Open Questions
- Anything still unresolved

## References
- Related specs, ADRs, issues
```

### 2. Review with User

Present the draft and iterate based on feedback. Keep it concise -- a spec should be short enough to read in a couple of
minutes.

### 3. Save

Pipe the spec content via stdin. When `--body` is omitted and stdin is piped, gest reads the body automatically. Use
`-q`
to get the bare artifact ID:

```sh
cat <<'EOF' | cargo run -- artifact create "<title>" \
  --tag spec --tag "<area>" -q
<spec content here>
EOF
```

Title is a positional argument in v0.5.0. Categorization is tag-driven: include the `spec`
tag plus the relevant area tag(s) from `docs/process/labels.md`.

The `-q` flag prints only the artifact ID, ready for downstream use.

### 4. Next Step

Print: `invoke /plan <id> when you're ready for the next step`
