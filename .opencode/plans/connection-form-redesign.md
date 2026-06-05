# Connection Form Redesign

## Goal
Rewrite `render_form()` in `src/tui/connections.rs` to use proper ratatui widgets (`Layout`, `Block`, `Paragraph` per field) instead of a single monolithic `Paragraph` with manual `Line`/`Span` concatenation.

## Changes

### File: `src/tui/connections.rs`

**Step 1: Update imports**

Add `Constraint` and `Layout` to the `ratatui::layout` import:

```rust
use ratatui::layout::{Constraint, Layout, Rect};
```

**Step 2: Rewrite `render_form()`**

Replace the current `render_form` function (lines 71-135) with:

```rust
fn render_form(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let block = Block::default()
        .borders(Borders::ALL)
        .title(match app.conn_mode {
            ConnectionMode::Adding => " New Connection ",
            ConnectionMode::Editing(_) => " Edit Connection ",
            _ => " Connection ",
        })
        .border_style(theme.border_primary);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let fields = [
        ("Name", &app.conn_form_name, false),
        ("Host", &app.conn_form_host, false),
        ("Port", &app.conn_form_port, false),
        ("User", &app.conn_form_user, false),
        ("Password", &app.conn_form_password, true),
        ("Database", &app.conn_form_database, false),
    ];

    // 6 field rows + 1 flexible bottom spacer
    let constraints: [Constraint; 7] = [
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Fill(1),
    ];
    let rows = Layout::vertical(&constraints).split(inner);

    for (i, (label, value, is_password)) in fields.iter().enumerate() {
        let is_focused = i == app.conn_form_focus;
        let row = rows[i];

        let [label_area, value_area] = Layout::horizontal([
            Constraint::Length(14),
            Constraint::Fill(1),
        ])
        .areas(row);

        // ── Label ──
        let prefix = if is_focused { "▶ " } else { "  " };
        let label_style = if is_focused {
            theme.sql_focused
        } else {
            Style::new().bold()
        };
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                format!("{}{}:", prefix, label),
                label_style,
            ))),
            label_area,
        );

        // ── Value ──
        let display = if *is_password && !value.is_empty() {
            "·".repeat(value.len())
        } else if value.is_empty() && !is_focused {
            format!("<{}>", label.to_lowercase())
        } else {
            value.to_string()
        };

        let inner_style = if is_focused {
            Style::new().bg(theme.input_bg).fg(theme.input_fg)
        } else if value.is_empty() {
            Style::new().dim()
        } else {
            Style::new()
        };

        let border_style = if is_focused {
            theme.border_primary
        } else {
            theme.border_secondary
        };

        frame.render_widget(
            Paragraph::new(display)
                .style(inner_style)
                .block(Block::default().borders(Borders::BOTTOM).border_style(border_style)),
            value_area,
        );
    }
}
```

## Visual Layout

Each field row is 2 lines tall:

```
┌─── New Connection ──────────────────────────────┐
│                                                   │
│  ▶ Name:      ────────────────────────────────    │  ← focused: primary border + input bg/fg
│                                                   │
│    Host:      ────────────────────────────────    │  ← idle: secondary border, no bg
│                                                   │
│    Port:      ─(3306)─────────────────────────    │  ← empty idle: dim angle-bracket placeholder
│                                                   │
│    User:      ─(root)─────────────────────────    │
│                                                   │
│    Password:  ·························───────    │  ← masked with · chars
│                                                   │
│    Database:  ─(mysql)────────────────────────    │
│                                                   │
│                                                   │  ← Fill(1) spacer
└───────────────────────────────────────────────────┘
```

## What stays the same
- `render_list()` — unchanged
- All `app/connections.rs` logic (form fields, focus index, submit, cancel, navigation)
- All `app/event.rs` key handling
- Bottom bar hints (already show Tab/Enter/Esc for ConnectionForm)
- Password masking with `·`
