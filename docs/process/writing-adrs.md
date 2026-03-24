# Architecture Decision Records (ADRs)

## Lifecycle

In our workflow, ADRs follow a specific path:

1. **Drafted** during planning -- stored in gest as `adr` artifacts
2. **Published** as a GitHub Discussion for team review -- see [Publishing]
3. **Committed** during implementation -- copied into `docs/design/` as one of the first commits

## When to Write an ADR

Write an ADR when making decisions that:

- Affect the overall structure or architecture of the codebase
- Establish patterns or conventions that other code should follow
- Have long-term implications that future contributors need to understand
- Represent a significant trade-off between competing concerns

Unlike RFCs which gather feedback before committing to an approach, ADRs document decisions that have already been made.

## ADR Structure

Each ADR describes:

- **Context**: The circumstances and forces at play when the decision was made
- **Decision**: The change or approach that was chosen
- **Consequences**: The resulting effects, both positive and negative

## Status Lifecycle

| Status                           | Meaning                                                |
|----------------------------------|--------------------------------------------------------|
| ![Active][badge-active]          | Currently enforced                                     |
| ![Superseded][badge-superseded]  | Replaced by another ADR (update `superseded-by` field) |
| ![Deprecated][badge-deprecated]  | No longer followed, kept for historical reference      |

## ID Assignment

ADR IDs are **not** assigned during drafting. Drafts use `id: draft` and `# ADR-DRAFT: Title`. The next sequential ID is
assigned at approval time by checking existing ADRs in `docs/design/`.

## Using the Template

The template below shows all possible sections and frontmatter fields. **Omit any section or frontmatter field that does
not apply to the decision.** For example, if the decision introduces no new dependencies, omit the Dependencies section
entirely. Only add `superseded-by` when the ADR is actually superseded. Do not include empty or placeholder sections.

## Template

```markdown
---
id: draft
title: ADR Title
status: active
tags: []
created: YYYY-MM-DD
superseded-by:
---

# ADR-DRAFT: Title

## Status

![Static Badge](https://img.shields.io/badge/Active-green?style=for-the-badge)

## Summary

One paragraph explaining the decision.

## Context

Why is this decision needed? What problem does it solve?

## Decision

What we're going to do. Technical details, syntax, semantics, etc.

## Dependencies

New dependencies introduced by this decision.

| Dependency | Version | Purpose |
|------------|---------|---------|
| -          | -       | -       |

## Consequences

### Positive

- ...

### Negative

- ...

## Open Questions

- Question 1?

## Future Work

Things explicitly out of scope, for future ADRs.

## References

- Related ADRs, discussions, external resources
```

[badge-active]: https://img.shields.io/badge/Active-green?style=for-the-badge
[badge-deprecated]: https://img.shields.io/badge/Deprecated-red?style=for-the-badge
[badge-superseded]: https://img.shields.io/badge/XXXX--Title-black?style=for-the-badge&label=Superseded&labelColor=orange
[Publishing]: https://github.com/aaronmallen/gest/blob/main/docs/process/publishing.md
