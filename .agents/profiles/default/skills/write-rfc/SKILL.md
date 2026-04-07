---
name: write-rfc
description: "Draft a Request for Comments and store it as a gest artifact (e.g. /write-rfc \"plugin system\")."
args: "<proposal topic>"
---

# Write RFC

Draft an RFC and store it as a gest artifact. RFCs gather feedback before committing to an approach -- use them for
cross-cutting concerns, public API changes, or proposals that benefit from discussion.

## Instructions

### 1. Draft the RFC

Use the template from `docs/process/writing-rfcs.md`. Key sections:

- **Summary** -- one paragraph explaining the proposal
- **Motivation** -- why are we doing this?
- **Goals / Non-Goals** -- what this does and doesn't aim to address
- **Proposed Design** -- detailed explanation with API examples where helpful
- **Alternatives Considered** -- other designs and why they weren't chosen
- **Unresolved Questions** -- things to answer before or during implementation

### 2. Review with User

Present the draft and iterate. An RFC should be detailed enough for someone unfamiliar with the problem to evaluate the
proposal.

### 3. Save

Pipe the RFC content via stdin. When `--body` is omitted and stdin is piped, gest reads the body automatically. Use `-q`
to get the bare artifact ID:

```sh
cat <<'EOF' | cargo run -- artifact create "<title>" \
  --tag rfc --tag "<area>" -q
<rfc content here>
EOF
```

Title is a positional argument in v0.5.0. Categorization is tag-driven: include the `rfc`
tag plus the relevant area tag(s) from `docs/process/labels.md`.

The `-q` flag prints only the artifact ID, ready for downstream use.

### 4. Next Step

Print: `share the RFC for feedback, then invoke /plan <id> when ready`
