# Labels

Labels are used to categorize and track issues and pull requests. Each label belongs to a color-coded category
that indicates its purpose at a glance.

## Color Scheme

| Swatch                                                           | Category      | Description                                   |
|------------------------------------------------------------------|---------------|-----------------------------------------------|
| ![bug](https://img.shields.io/badge/-b60205?color=b60205)        | Bug           | Reserved exclusively for bugs                 |
| ![priority](https://img.shields.io/badge/-d93f0b?color=d93f0b)   | Priority      | Issue priority level                          |
| ![status](https://img.shields.io/badge/-fbca04?color=fbca04)     | Status        | Current workflow status (max: 1 per issue)    |
| ![type](https://img.shields.io/badge/-6aa654?color=6aa654)       | Type          | Kind of work being done (max: 1 per issue)    |
| ![area](https://img.shields.io/badge/-1d76db?color=1d76db)       | Area          | Affected component or area of the codebase    |
| ![discussion](https://img.shields.io/badge/-5319e7?color=5319e7) | Discussion    | Discussion or exploration type                |
| ![semver](https://img.shields.io/badge/-f9d0c4?color=f9d0c4)     | Semver Impact | Semantic versioning impact of the change      |
| ![meta](https://img.shields.io/badge/-333333?color=333333)       | Meta          | Additional context or contributor info        |
| ![resolution](https://img.shields.io/badge/-cccccc?color=cccccc) | Resolution    | How the issue was resolved (max: 1 per issue) |

## Rules

- An issue may only have **one** Type label (green)
- An issue may only have **one** Status label (yellow)
- An issue may only have **one** Resolution label (light gray)

## Categories

### Priority (`d93f0b`)

Priority labels indicate urgency and help with sprint planning. Every issue should eventually receive a priority
label during triage. Multiple priority labels on a single issue are discouraged but not enforced.

| Label | Description                 |
|-------|-----------------------------|
| `p0`  | Critical -- drop everything |
| `p1`  | High -- address this sprint |
| `p2`  | Medium -- address soon      |
| `p3`  | Low -- when time permits    |
| `p4`  | Minimal -- nice to have     |

### Status (`fbca04`)

Status labels track where an issue is in the workflow. An issue must have at most **one** status label at any time.
Update the status label as work progresses rather than stacking them.

| Label         | Description                                   |
|---------------|-----------------------------------------------|
| `triage`      | Needs review and categorization               |
| `in progress` | Actively being worked on                      |
| `blocked`     | Waiting on an external dependency or decision |

### Type (`6aa654`)

Type labels describe the kind of work an issue represents. An issue must have at most **one** type label.

| Label          | Description                                 |
|----------------|---------------------------------------------|
| `bug`          | Something isn't working                     |
| `enhancement`  | New feature or improvement                  |
| `epic`         | A large feature composed of multiple issues |
| `chore`        | Maintenance or housekeeping task            |
| `optimization` | Performance or efficiency improvement       |
| `release`      | Release preparation and versioning          |

> **PR-only:** The `fix` label should only be applied to pull requests, not issues.

### Area (`1d76db`)

Area labels identify which part of the codebase is affected. An issue should have at least one area label. An issue may
have multiple area labels when changes span several components. Area labels are expected to evolve as the project grows
-- check the repository's label list for current values.

### Discussion (`5319e7`)

Discussion labels are used for issues that are exploratory or conversational rather than actionable work items.
They may eventually produce actionable issues but exist primarily to gather input.

| Label        | Description                               |
|--------------|-------------------------------------------|
| `discussion` | Open-ended discussion topic               |
| `question`   | Request for information or clarification  |
| `rfc`        | Request for comments on a proposed change |
| `spec`       | Product specification                     |
| `adr`        | Architecture Decision Record              |
| `spike`      | Research or exploration task              |

### Semver Impact (`f9d0c4`) -- PRs only

Semver labels indicate the semantic versioning impact of a pull request. They help with release planning and changelog
generation. These labels should only be applied to PRs, not issues.

| Label          | Description                                                 |
|----------------|-------------------------------------------------------------|
| `major change` | Breaking change requiring a major version bump              |
| `minor change` | Backwards-compatible feature requiring a minor version bump |
| `patch change` | Backwards-compatible fix requiring a patch version bump     |

### Meta (`333333`)

Meta labels provide additional context about an issue. They can be combined freely with other categories.

| Label              | Description                                         |
|--------------------|-----------------------------------------------------|
| `dependabot`       | Automated dependency update from Dependabot         |
| `good first issue` | Good for newcomers                                  |
| `help wanted`      | Extra attention is needed                           |
| `regression`       | Previously working functionality that is now broken |
| `security`         | Security-related issue or vulnerability             |

### Resolution (`cccccc`)

Resolution labels indicate why an issue was closed without completing the requested work. An issue must have at
most **one** resolution label. Issues closed as completed do not need a resolution label.

| Label       | Description                |
|-------------|----------------------------|
| `duplicate` | This issue already exists  |
| `invalid`   | This issue is not valid    |
| `on hold`   | Postponed indefinitely     |
| `wontfix`   | This will not be worked on |

## Gest Tag Convention

Gest tasks, artifacts, and iterations use **bare tags** — the same vocabulary as GitHub labels but without namespace
prefixes. Do not use `area:cli` or `type:bug`; use `cli` and `bug` directly.

### Known Area Tags

| Tag       | Component                          |
|-----------|------------------------------------|
| `cli`     | CLI commands and argument parsing  |
| `config`  | Configuration loading and settings |
| `docs`    | Documentation and process guides   |
| `model`   | Domain model types and IDs         |
| `server`  | Web dashboard and API handlers     |
| `storage` | Store layer and file persistence   |
| `ui`      | Terminal UI composites and theming |

Area tags may be combined with type tags (`enhancement`, `bug`, `chore`) and priority tags (`p0`–`p4`) in a single
comma-separated `--tags` value.
