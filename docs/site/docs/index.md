---
slug: /
title: gest
---

# gest

Parallel execution for AI-assisted development.

Decompose agent-generated work into phased iterations, dispatch independent
tasks concurrently, and browse everything through a built-in web dashboard.

- **⚡ Parallel Execution** — Group tasks into phased iterations with
  dependency tracking. Tasks in the same phase run concurrently across
  workspaces.
- **🖥️ Web Dashboard** — Browse tasks, artifacts, and iterations in a built-in
  web UI. Inspect status at a glance, view kanban boards, search across
  everything, and read rendered Markdown.
- **📄 Artifacts & Specs** — Store specs, ADRs, RFCs, and design documents as
  versioned Markdown with YAML frontmatter.
- **🤖 Agent-Native** — Every command supports `--json` output. Agents read
  the work queue, claim tasks, update status, and store artifacts — all
  through the CLI.
- **🗂️ SQLite-First, Git-Friendly** — Entity data lives in a single SQLite
  database. Opt into a `.gest/` sync mirror (YAML + Markdown) to commit your
  data alongside your code.
- **🌐 Global or Local** — Projects share one SQLite database at
  `~/.local/share/gest/gest.db`. Add `--local` on init to materialize a
  `.gest/` mirror inside the repo.

Head to [Installation](getting-started/installation.md) to get started.
