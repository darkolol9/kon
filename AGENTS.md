# AGENTS.md

## Project

A MySQL CLI tool called **kon** — interactive SQL editor + connection manager.

## Scope (from `initial.md`)

- `kon connect` — prompt-based connection setup (host, port, connection name, etc.)
- `kon ls` — list saved connections
- `kon set <pattern>` — set active connection by wildcard name match
- `kon` (no subcommand) — open interactive SQL editor with the active connection
  - formatted table output
  - syntax highlighting
  - autocomplete (SQL syntax + table/column names)

## State

No code written yet. No build system, no dependencies, no framework chosen. This file will need updating once the stack is decided and code exists.
