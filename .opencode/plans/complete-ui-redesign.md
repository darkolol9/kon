# Complete UI Redesign

## Root problems

1. **Editor panel looks bad** — results are concatenated into a single Paragraph via string manipulation instead of using ratatui's proper `Table` widget
2. **Input bar is cramped** — 2-line area with border eating half of it, no room for the prompt
3. **`tui/table.rs` does manual box-drawing with unicode chars** — fragile, doesn't use `ratatui::widgets::Table`
4. **No visual separation** between query blocks, results, and input area
5. **Top/bottom bars are plain Paragraph lines** — no visual depth

---

## Plan: Rewrite 5 files

### 1. `src/tui/table.rs` — Use ratatui `Table` widget

**Before:** Manual Paragraph line rendering with `├┼┤`, `┌┐└┘`, manual column width calc.

**After:** Use `ratatui::widgets::Table` + `Row` + `Cell`.

```
use ratatui::layout::Constraint;
use ratatui::widgets::{Cell, Row, Table};
```

- Compute column widths based on content length: each column gets `Constraint::Min(min_width)`. If total < available width, distribute remaining with `Constraint::Length`.
- Header row: `Row::new(columns).style(theme.table_header_focused)`
- Data rows: `Row::new(cells).style(...)` where cells are `Cell::from(value).style(...)` for null/numeric/string.
- Alternate row bg: apply `.style()` to every other row (or let `Table` handle it).
- `Table::new(rows, widths).header(header).column_spacing(2).highlight_style(...)`
- Render via `frame.render_widget(table, area)` — no manual Paragraph.

**Return type:** Change from `Vec<Line<'static>>` to `(Table<'static>, Rect)` — the table widget + the area it occupies. Or, simplify: have `render_table(frame, area, result, theme)` render directly into the frame.

Delete dead code: `format_result_as_text()`, the old `render()` function.

### 2. `src/tui/vertical.rs` — Keep mostly the same

The vertical view is inherently line-oriented. Keep Paragraph-based rendering but improve styling:
- Use a `Block::bordered().title("...")` around each row's section
- No need to change to Table widget here

Or, keep as-is with minor style improvements.

### 3. `src/tui/editor.rs` — Complete rewrite

**Before:** Splits content into results area + 2-line input area. Results are rendered as one giant Paragraph. Input has `Borders::TOP` with `input_height=2`.

**After:**

```
Layout within content area:
  [results area]           — Constraint::Fill(1)
  [input area]             — Constraint::Length(3)
```

**Results area rendering:**
- If no query blocks: show welcome message centered
- Iterate `app.query_blocks`, for each:
  - Render a "query header" line: `▶ SELECT * FROM users` (focused style) or `  SELECT * FROM users` (unfocused)
  - Render the result using `table::render_table()` or `vertical::render_vertical_lines()`
  - If error: render error line
  - Add spacing between blocks
- Wrap everything in a `Block::bordered()` or use borderless with subtle separators

**Input area:**
```
┌─ myconn ─────────────────────────────────┐
│ myconn> SELECT * FROM [cursor]           │
└──────────────────────────────────────────┘
```
- 3 lines: top border (with title = connection name), input text, bottom border
- Or: `Block::bordered().title(app.conn_name).title_alignment(...)`
- Inside: Paragraph with `prefix + input_text`, left-aligned
- Cursor: `frame.set_cursor_position(...)` on the second row of the area

Actually simpler: just use 3 lines:
- Line 1: `Block::bordered().title(app.conn_name)` — this creates top edge
- Content inside the block: 1 line for text
- So `input_height = 3` gives: top border (1) + text (1) + bottom border (1)

Since Block::bordered() has both top and bottom, the actual text renders inside. With height=3:
- Line 0: top border
- Line 1: text content
- Line 2: bottom border

The cursor should be on line 1 of the inner area.

### 4. `src/tui/top_bar.rs` — Use `Block` for visual depth

**Before:** Plain Paragraph background fill with colored Spans.

**After:** Use a `Block::default()` with background set — or keep Paragraph but add a bottom border line for visual separation from content.
- Keep the active/inactive tab styling
- Add `Block::default().borders(Borders::BOTTOM).border_style(theme.border_secondary)` 
- This gives a clean horizontal line separating the top bar from content

### 5. `src/tui/bottom_bar.rs` — Use `Block` for visual depth

**After:** Add `Block::default().borders(Borders::TOP).border_style(theme.border_secondary)` to create a separating line above the shortcut text.

---

## File-by-file changes

### `src/tui/table.rs` (~310 lines → ~120 lines)
- Delete: `truncate()`, `is_numeric()`, `detect_numeric_cols()`, `compute_col_widths()`, `format_value()`, `render()`, `format_result_as_text()`, `render_table_lines()`
- New function: `pub fn render_table(frame: &mut Frame, area: Rect, result: &QueryResult, theme: &Theme, focused: bool)`
- Inside: build `Table` widget with `Row`/`Cell`, compute `Constraint` array, render via `frame.render_widget`

Column width strategy:
```
for each column i:
  measure header width = col_name.len()
  measure max data width = max(row[i].len()) among all rows
  col_width[i] = max(header_width, max_data_width).min(MAX_COL_WIDTH)
  constraint[i] = Constraint::Min(col_width[i])
```

### `src/tui/editor.rs` (~166 lines → ~200 lines)
- Change `input_height: 1` → `input_height: 3`
- Rewrite `render()` to:
  1. Use `Layout::vertical([Fill(1), Length(3)])` to split content into results + input
  2. Results area: render as a scrollable set of blocks (query header + table result)
  3. Each query block: Paragraph header + Table below
- Rewrite `render_input()` to use `Block::bordered().title(conn_name)` with 3-line area
- Set cursor inside the inner content area (line 1 of the 3)

### `src/tui/top_bar.rs` (~47 lines)
- Add `Block::default().borders(Borders::BOTTOM).border_style(...)` wrapper

### `src/tui/bottom_bar.rs` (~56 lines)
- Add `Block::default().borders(Borders::TOP).border_style(...)` wrapper

---

## Visual flow (ASCII)

```
┌─ Editor ── Connections ── Settings ── kon · mydb ───────┐
│                                                          │  ← top_bar with bottom border
│ ┌──────────────────────────────────────────────────────┐ │
│ │ ▶ SELECT * FROM users                                 │ │
│ │ ┌id─┬─name──┬─email───────────┬─created_at─────────┐ │ │
│ │ │ 1 │ Alice │ alice@example.. │ 2024-01-15 10:30   │ │ │
│ │ │ 2 │ Bob   │ bob@example.com │ 2024-01-20 14:22   │ │ │
│ │ └────┴───────┴─────────────────┴────────────────────┘ │ │
│ │                                                      │ │
│ │ ▶ SELECT * FROM orders                               │ │
│ │ ┌id─┬─customer─┬─total─┬─status────┬─date─────────┐ │ │
│ │ │ 1 │ Alice    │ 42.50 │ pending    │ 2024-02-01  │ │ │
│ │ └────┴──────────┴───────┴───────────┴──────────────┘ │ │
│ │                                                      │ │
│ │ 2 rows in set (12 ms)                                │ │
│ │                                                      │ │
│ ├─ mydb ───────────────────────────────────────────────┤ │
│ │ mydb> SELECT * FROM [cursor]                         │ │
│ └──────────────────────────────────────────────────────┘ │
│                                                          │
├─ ↵ Execute  ⌃P Palette  ...  |  Alt+1 Editor  Alt+2 Con ┤ ← bottom_bar with top border
└──────────────────────────────────────────────────────────┘
```

---

## Error handling

- Table has no rows → show `Query OK, N row(s) affected` as Paragraph
- Error → show `ERROR: ...` in red as Paragraph below the query header
- No results yet → centered welcome text

## Implementation steps

1. Rewrite `tui/table.rs` to use `ratatui::widgets::Table`
2. Rewrite `tui/editor.rs` for proper 3-line input + Table-based results
3. Update `tui/top_bar.rs` with bottom border
4. Update `tui/bottom_bar.rs` with top border
5. `cargo check && cargo clippy -- -D warnings && cargo fmt --check`
