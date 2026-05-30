# AGENTS.md

## Project

**kon** — a MySQL CLI tool with an interactive TUI editor (single binary).

## Stack

- Rust edition 2024 (requires 1.85+), tokio async, ratatui 0.30 + crossterm 0.29, sqlx 0.9 (MySQL), sqlparser 0.62, clap 4
- `Clear` is at `ratatui::widgets::Clear` (re-exported from `ratatui_widgets`)

## Commands

| Command | What it does |
|---------|-------------|
| `cargo check` | Type-check only |
| `cargo clippy -- -D warnings` | Lint (CI enforces this) |
| `cargo fmt --check` | Format check |
| `cargo test` | Run tests (none exist yet) |
| `cargo build --release` | Production build |
| `./build.sh` | Build + install to `$KON_BIN_DIR` (default `~/.cargo/bin`) |

CI runs: `check` → `fmt` → `clippy` → `test` in parallel on push/PR to main.

Release: push tag `v*` → builds 3 targets (linux-musl, macos, windows) → creates GitHub release with `install.sh`.

## Architecture

```
src/main.rs          — entrypoint, dispatch to cmd_* functions
src/cli.rs           — clap CLI definition (connect/ls/set)
src/config.rs        — TOML config at ~/.config/kon/config.toml, wildcard matching
src/db.rs            — sqlx MySqlPool, row value conversion with type fallbacks
src/repl.rs          — App struct, event loop, key handling, open_in_editor
src/repl/ui.rs       — ratatui rendering (tables, completion popup, status bar)
src/repl/completion.rs — context-aware autocomplete engine (sqlparser tokenizer)
src/repl/syntax.rs   — SQL syntax highlighting tokenizer
```

## Gotchas

- `open_in_editor()` manually does `LeaveAlternateScreen`/`disable_raw_mode` → editor → `enable_raw_mode`/`EnterAlternateScreen` + `terminal.clear()`. When adding editor support, always pass `terminal: &mut DefaultTerminal`.
- `scroll` in `App` is offset-from-bottom (larger = older content). Scroll direction: `scroll_page_older` increments, `scroll_page_newer` decrements.
- `row_values()` in `db.rs` has explicit `try_get` fallbacks: `String` → `i64` → `i32` → `u64` → `u32` → `f64` → `f32` → `"?"`. Add new type mappings when encountering `?` output.
- Completion context is determined by tokenizing input backwards from cursor via `sqlparser`. The `Context` enum is: `Keyword`/`Table`/`Column`/`Global`/`None`. Functions are a separate category matched alongside column context. Subsequence matching (no external fuzzy lib).
- `PgUp`/`PgDn` scroll results by 1 block (not a full page).
- `\G` suffix (MySQL convention) triggers vertical view mode; `\g` is equivalent. Handled in `strip_trailing_g()`.
- Key bindings not in README: `Ctrl+O` = open last block in `$EDITOR`, `Ctrl+V` = toggle table/vertical view, `Ctrl+R` = refresh schema, `Alt+Up/Down` = navigate between query blocks, `Ctrl+Left/Right` = horizontal scroll.
- All SQL goes through `sqlx::raw_sql(AssertSqlSafe(...))` — MySQL-only, no query builder.
- Passwords stored in plaintext in config file. No encryption.
- No tests exist anywhere in the codebase.
