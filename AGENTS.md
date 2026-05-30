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
src/
├── main.rs              — entrypoint, dispatch to cmd_* functions
├── cli.rs               — clap CLI definition (connect/ls/set)
├── cmd.rs               — Command registry (name, aliases, description, _category) + resolve()/all_names()
├── config.rs            — TOML config at ~/.config/kon/config.toml, wildcard matching
├── db.rs                — sqlx MySqlPool, row value conversion with type fallbacks
├── theme.rs             — Theme struct, 7 LazyLock presets (default/dracula/nord/monokai/light/tokyo-night/catppuccin)
│
├── app/                 — Application state + logic
│   ├── mod.rs           — App struct, QueryBlock, new()
│   ├── state.rs         — Enums: Panel, Focus, ViewMode, AppState, ConnectionMode, PaletteEntry
│   ├── input.rs         — impl App: insert_char, delete, cursor movement, history, scroll
│   ├── editor.rs        — impl App: query execution, command handling, push_block, open_in_editor
│   ├── connections.rs   — impl App: connection CRUD, inline form management
│   ├── settings.rs      — impl App: set_theme, select_theme
│   ├── completion.rs    — CompletionEngine: schema fetching, tokenizer-based context detection, subsequence matching
│   └── event.rs         — run() event loop, 9 focus-specific key handlers (input, results, schema, history, palette, connections list, connection form, settings, help)
│
└── tui/                 — Rendering layer (ratatui)
    ├── mod.rs           — render(): splits layout, dispatches to active panel
    ├── layout.rs        — AppLayout: [TopBar=1] [Content=Fill] [BottomBar=1] via Layout::vertical
    ├── top_bar.rs       — Sticky panel tabs: [Editor] [Connections] [Settings] + connection name
    ├── bottom_bar.rs    — Context-sensitive shortcut bar (dynamic by Panel/Focus)
    ├── editor.rs        — Editor panel: input line (syntax-highlighted) + results (table/vertical blocks + scrollbar)
    ├── table.rs         — SQL table rendering (Paragraph-based with column width calc, scrollbar)
    ├── vertical.rs      — Vertical result rendering (col : val per row)
    ├── connections.rs   — Connections list + inline add/edit form
    ├── settings.rs      — Theme picker list
    ├── syntax.rs        — SQL syntax highlighting tokenizer (sqlparser)
    ├── completion.rs    — Completion popup overlay
    ├── overlays.rs      — Command palette + help overlay
    └── format.rs        — Placeholder for text export helpers
```

### 3 panels system (exclusive tabs, `Alt+1/2/3` to switch)
- **Editor** — SQL input + results (existing workflow)
- **Connections** — Browse, add (`n`), edit (`e`), delete (`d`), activate (`Enter`) connections inline
- **Settings** — Theme picker with live preview

## Gotchas

### Editor screen management
`open_in_editor()` manually does `LeaveAlternateScreen`/`disable_raw_mode` → editor → `enable_raw_mode`/`EnterAlternateScreen` + `terminal.clear()`. When adding editor support, always pass `terminal: &mut DefaultTerminal`.

### Scroll semantics
`scroll` in `App` is offset-from-bottom (larger = older content). Scroll direction: `scroll_page_older` increments, `scroll_page_newer` decrements.

### Row value type fallbacks
`row_values()` in `db.rs` has explicit `try_get` fallbacks: `String` → `i64` → `i32` → `u64` → `u32` → `f64` → `f32` → `"?"`. Add new type mappings when encountering `?` output.

### Completion engine
Context is determined by tokenizing input backwards from cursor via `sqlparser`. The `Context` enum is: `Keyword`/`Table`/`Column`/`Global`/`None`. Functions are a separate category matched alongside column context. Subsequence matching (no external fuzzy lib).

When input starts with `/`, completion short-circuits to command name matching (from `cmd::all_names()`). No SQL completion runs for command lines.

### QueryResult: is_query field
`QueryResult` has `is_query: bool` (`#[allow(dead_code)]`) — set by `db.execute()` based on whether SQL is a SELECT/SHOW/DESC query vs DML. Not used in rendering but must be set when constructing manually (e.g. `/help` command).

### Command system
`/` commands are defined in `cmd.rs` as a static `COMMANDS` array. Each entry has name, aliases, description, category.

**Adding a new command is 3 steps:**
1. Add an entry to `COMMANDS` in `cmd.rs`
2. Add a match arm in `handle_command()` in `app/editor.rs`
3. Write a handler method on `App` (in `app/editor.rs`)

Handler methods that need DB access must be `async` (e.g. `/tables` calls `self.db.execute()`). Handlers use `self.push_block()` to create result blocks and manage `view_modes` / `active_block`.

### Theme system
`Theme` struct in `theme.rs` has ~30 color/style fields. All UI and syntax colors are driven by `app.theme`. To add a field, update the struct + all 7 LazyLock presets.

`completion_command` controls the color of `/` command entries in the completion popup (the ` M ` prefix).

Theme change persists to config on selection via `set_theme()`.

### Scroll granularity
`PgUp`/`PgDn` scroll results by 1 block (not a full page).

### View modes
`\G` suffix (MySQL convention) triggers vertical view mode; `\g` is equivalent. Handled in `strip_trailing_g()`. `Ctrl+V` toggles between table/vertical on the focused block.

### Key bindings
Panel navigation: `Alt+1` Editor, `Alt+2` Connections, `Alt+3` Settings
Editor: `Ctrl+O` open last block in `$EDITOR`, `Ctrl+V` toggle table/vertical, `Ctrl+R` refresh schema, `Alt+Left/Right` navigate query blocks, `Ctrl+Left/Right` horizontal scroll, `Ctrl+S` schema browser, `Ctrl+H` history, `Ctrl+P` command palette, `Ctrl+?` help.
Connections: `↑↓` navigate, `Enter` activate, `n` new, `e` edit, `d` delete, `Esc` back.
Settings: `↑↓` navigate, `Enter` apply theme, `Esc` back.

### SQL execution
All SQL goes through `sqlx::raw_sql(AssertSqlSafe(...))` — MySQL-only, no query builder.

### Security
Passwords stored in plaintext in config file. No encryption.

### Tests
No tests exist anywhere in the codebase.
