---
name: doc-code
description: Add documentation comments to all changed code files, dispatching one agent per file.
---

# Doc Code

Add documentation comments to all changed code files in the current diff.

## Instructions

### 1. Identify Changed Files

Run `jj diff --summary` to identify all changed files. Parse the output to extract file paths (each line has a status
letter followed by the file path). Filter the list to only include source code files (e.g., `*.rs`, `*.ts`, `*.py` —
exclude non-code files like `*.md`, `*.toml`, `*.lock`, `*.json`, `*.yaml`).

### 2. Document Each File

For each changed source code file, dispatch an agent to document that file. Launch all agents in parallel.

Each agent should:

1. Read the full file.
2. Run `jj diff <file_path>` to see exactly which lines were added or modified.
3. Add or update documentation comments **only for items touched in the diff** — do not document unchanged code. Items
   include: modules, types (structs, enums, traits), functions, methods, constants, and public fields.
4. Follow the language's idiomatic doc-comment style (e.g., `///` and `//!` for Rust, `/** */` or `//` for TypeScript,
   `"""` docstrings for Python).
5. Keep doc comments concise — one short summary line, optionally followed by a blank line and further detail only when
   the behavior is non-obvious.
6. Do not add redundant comments that just restate the name (e.g., `/// Returns the name` on a method called `name()`).

### 3. Verify

Run `mise lint` to verify that the added documentation does not introduce any lint errors.

### 4. Run Tests

Run `mise test` to confirm nothing is broken.
