# AI Coding Agents

This project supports multiple AI coding assistants running in a sandboxed Docker container with full access to the
project toolchain. Agents can edit code, run tests, invoke skills, and interact with GitHub -- without access to your
host system.

## Supported Assistants

| Assistant   | Task                      | Sandbox mode                     |
|-------------|---------------------------|----------------------------------|
| Claude Code | `mise run agent:claude`   | `--dangerously-skip-permissions` |
| Codex CLI   | `mise run agent:codex`    | `--yolo`                         |
| Gemini CLI  | `mise run agent:gemini`   | `--yolo`                         |
| OpenCode    | `mise run agent:opencode` | permission rules in config       |

## Quick Start

```sh
# 1. Run the interactive setup (choose assistants, VCS, task runner)
mise run dev:setup:agents

# 2. Launch an agent in a sandbox
mise run agent:claude
```

The first run builds a Docker image with the full project toolchain (Rust, cargo tools, linters, formatters).
This takes several minutes but only happens once -- subsequent runs use the cached image.

## First Run Authentication

On first launch inside the container, you need to authenticate two things:

1. **The AI assistant** -- each harness prompts for login on first use (e.g. Claude Code shows `/login`). Credentials
   are stored in a persistent Docker volume and reused across sessions.
2. **gh CLI** -- the container automatically detects if gh is unauthenticated and runs `gh auth login` before launching
   the assistant. This also persists across sessions.

## Setup

The setup task (`mise run dev:setup:agents`) is an interactive wizard that configures:

- **Which assistants** to enable (Claude Code, Codex CLI, Gemini CLI, OpenCode)
- **Which VCS** you use (jj, git, git-butler)
- **Which task runner** you use (mise, sh)

The setup script wires each assistant's config directory (`.claude/`, `.codex/`, `.gemini/`, `.opencode/`) with
symlinks to the assembled agents and skills from the profile system.

To reconfigure, run the setup task again -- it detects existing configuration and offers to reconfigure.

## Profile System

Agents and skills are organized into profiles under `.agents/profiles/`. Three profiles are merged at setup time:

1. **`default/`** -- tool-agnostic agents, skills, and hooks shared by all configurations
2. **VCS profile** (`jj/`, `git/`, `git-butler/`) -- provides the `vcs-expert` agent
3. **Task runner profile** (`mise/`, `sh/`) -- provides the `task-runner` agent

Later profiles override earlier ones. This means a VCS or task runner profile can replace a default agent if needed.

### Directory Structure

```text
.agents/
  profiles/                        # Source of truth (tracked in git)
    default/
      agents/                      # code-reviewer, dependency-auditor, doc-writer, test-runner
      skills/                      # brainstorm, commit, format, implement-issue, plan, etc.
      hooks/                       # session-start.sh, pre-edit.sh, pre-commit.sh
    jj/agents/vcs-expert/          # Jujutsu VCS commands
    git/agents/vcs-expert/         # Git VCS commands
    git-butler/agents/vcs-expert/  # Git Butler VCS commands
    mise/agents/task-runner/       # mise run <task> commands
    sh/agents/task-runner/         # ./tasks/<task> direct execution
  docker/                          # Container infrastructure
    generate.sh                    # Generates Dockerfile from .agents/.config
    entrypoint.sh                  # Container entrypoint (user setup, symlinks, auth)
    run.sh                         # Shared runner for all agent tasks
```

### Delegation Pattern

Skills and agents in the `default/` profile never reference specific tools directly. Instead:

- VCS operations delegate to the **vcs-expert** agent
- Task execution delegates to the **task-runner** agent

This keeps the default profile portable across VCS and task runner choices.

## Docker Sandbox

Each `mise run agent:<name>` task runs the assistant inside a Docker container with:

- The project mounted read-write at `/workspace`
- A persistent named volume (`gest-agent-home`) for credentials and caches
- The full project toolchain installed (Rust, clippy, rustfmt, cargo tools, gh, linters, formatters)
- A non-root user matching your host UID/GID for correct file ownership

### Image Generation

The Dockerfile is generated from `.agents/.config` by `.agents/docker/generate.sh`. It conditionally includes layers
based on your configuration (which VCS, which assistants, whether mise is the task runner). The generator only
regenerates when the config file is newer than the existing Dockerfile.

### Environment Variables

API keys and tokens set on the host are forwarded into the container if present:

| Variable                  | Used by     |
|---------------------------|-------------|
| `ANTHROPIC_API_KEY`       | Claude Code |
| `CLAUDE_CODE_OAUTH_TOKEN` | Claude Code |
| `OPENAI_API_KEY`          | Codex CLI   |
| `GEMINI_API_KEY`          | Gemini CLI  |
| `GOOGLE_API_KEY`          | Gemini CLI  |
| `GH_TOKEN`                | gh CLI      |

If none are set, the assistant prompts for interactive login on first run.

## Adding a New Profile

To add support for a new VCS or task runner:

1. Create a directory under `.agents/profiles/<name>/`
2. Add an `agents/` subdirectory with the appropriate agent (`vcs-expert/` or `task-runner/`)
3. Write a `SKILL.md` file describing the agent's capabilities and available commands
4. If the tool needs to be installed in the Docker image, update `.agents/docker/generate.sh` with a new layer

## Troubleshooting

**Image build fails with "linker cc not found"** -- the base image needs `build-essential` for compiling cargo tools.
This is included by default but if you modify the Dockerfile generator, ensure it stays.

**gh CLI not authenticated** -- the entrypoint runs `gh auth login` automatically if gh is not authenticated. If this
fails, you can run `! gh auth login -h github.com` from inside the assistant.

**Stale Docker image after config change** -- delete the generated Dockerfile and rebuild:

```sh
rm .agents/docker/Dockerfile
mise run agent:claude
```

**Reset all persisted credentials** -- remove the named volume:

```sh
docker volume rm gest-agent-home
```
