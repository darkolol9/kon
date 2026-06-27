use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

use crate::app::{App, ConnectionMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    match app.conn_mode {
        ConnectionMode::Browse => render_list(frame, area, app),
        ConnectionMode::Adding | ConnectionMode::Editing(_) => render_wizard(frame, area, app),
    }
}

fn render_list(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let block = Block::bordered()
        .title(" Connections ")
        .border_style(theme.border_primary);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let list = app.config.list_connections();
    if list.is_empty() {
        let para = Paragraph::new(" (no connections saved) ")
            .style(Style::new().dim())
            .left_aligned();
        frame.render_widget(para, inner);
        return;
    }

    let active_name = app.config.active_connection.as_deref();

    let items: Vec<ListItem> = list
        .iter()
        .enumerate()
        .map(|(idx, (name, conn))| {
            let is_active = active_name == Some(name);
            let is_selected = idx == app.connection_selection;

            let marker = if is_active { " ◉ " } else { " ○ " };
            let display = format!(
                "{}{}  {}@{}:{}/{}",
                marker, name, conn.user, conn.host, conn.port, conn.database
            );

            let style = if is_active {
                Style::new().bold()
            } else {
                Style::new()
            };

            let item_style = if is_selected {
                theme.completion_selected
            } else {
                style
            };

            ListItem::new(Line::from(Span::styled(display, item_style)))
        })
        .collect();

    let list_widget = List::new(items).highlight_style(theme.completion_selected);
    frame.render_widget(list_widget, inner);
}

fn render_wizard(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let title = match app.conn_mode {
        ConnectionMode::Adding => " New Connection ",
        ConnectionMode::Editing(_) => " Edit Connection ",
        _ => " Connection ",
    };

    let popup_width = 52u16.min(area.width.saturating_sub(4));
    let popup_height = 16u16.min(area.height.saturating_sub(2));

    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(
        Paragraph::new("").style(Style::new().bg(theme.bg)),
        popup_area,
    );

    let block = Block::bordered()
        .title(title)
        .border_style(theme.border_primary);
    let inner = block.inner(popup_area);
    frame.render_widget(&block, popup_area);

    let [top_area, fields_area, hint_area, actions_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    // ── Progress / step indicator ──
    render_progress(frame, top_area, app, theme);

    // ── Fields for current step ──
    render_wizard_fields(frame, fields_area, app, theme);

    // ── Hint text ──
    let hint = wizard_hint(app.conn_wizard_step, app.conn_form_focus);
    let hint_para =
        Paragraph::new(Line::from(Span::styled(hint, Style::new().dim()))).left_aligned();
    frame.render_widget(hint_para, hint_area);

    // ── Actions ──
    render_actions(frame, actions_area, app, theme);
}

fn render_progress(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let steps = ["Connection Info", "Credentials", "Database"];
    let mut spans = Vec::new();

    for (i, name) in steps.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        let filled = i <= app.conn_wizard_step;
        let is_current = i == app.conn_wizard_step;
        let dot = if filled { "●" } else { "○" };
        let style = if is_current {
            theme.sql_focused
        } else if filled {
            theme.summary
        } else {
            Style::new().dim()
        };
        spans.push(Span::styled(format!(" {} {} ", dot, name), style));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)).left_aligned(), area);
}

fn wizard_fields_for_step(step: usize) -> Vec<(&'static str, &'static str)> {
    match step {
        0 => vec![
            ("Name", "A memorable name for this connection"),
            (
                "Host",
                "MySQL server address (e.g. localhost or 192.168.1.1)",
            ),
            ("Port", "Port number (default: 3306)"),
        ],
        1 => vec![
            ("User", "MySQL username"),
            ("Password", "Password for the MySQL user"),
        ],
        2 => vec![("Database", "Default database to use (e.g. mysql)")],
        _ => vec![],
    }
}

fn field_value(app: &App, step: usize, focus: usize) -> &str {
    let abs = App::wizard_absolute_field(step, focus);
    match abs {
        0 => &app.conn_form_name,
        1 => &app.conn_form_host,
        2 => &app.conn_form_port,
        3 => &app.conn_form_user,
        4 => &app.conn_form_password,
        5 => &app.conn_form_database,
        _ => "",
    }
}

fn render_wizard_fields(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let step = app.conn_wizard_step;
    let fields = wizard_fields_for_step(step);

    let row_height = 2u16;
    let total_height = fields.len() as u16 * row_height;
    let field_area = Rect::new(area.x, area.y, area.width, total_height.min(area.height));

    let constraints: Vec<Constraint> = fields
        .iter()
        .map(|_| Constraint::Length(row_height))
        .collect();
    let rows = Layout::vertical(constraints).split(field_area);

    for (i, (label, _hint)) in fields.iter().enumerate() {
        let is_focused = i == app.conn_form_focus;
        let row = rows[i];

        let [label_area, value_area] =
            Layout::horizontal([Constraint::Length(12), Constraint::Fill(1)]).areas(row);

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

        let value = field_value(app, step, i);
        let (display, cursor_offset) = if *label == "Password" && !value.is_empty() {
            ("·".repeat(value.len()), value.len())
        } else if value.is_empty() && !is_focused {
            (format!("<{}>", label.to_lowercase()), 0)
        } else {
            (value.to_string(), app.cursor.min(value.len()))
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

        let para = Paragraph::new(display).style(inner_style).block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(border_style),
        );
        frame.render_widget(para, value_area);

        if is_focused {
            let cursor_x = value_area.x + cursor_offset as u16;
            let cursor_y = value_area.y;
            frame.set_cursor_position((
                cursor_x.min(value_area.right().saturating_sub(1)),
                cursor_y,
            ));
        }
    }
}

fn render_actions(frame: &mut Frame, area: Rect, app: &App, theme: &crate::theme::Theme) {
    let step = app.conn_wizard_step;
    let mut spans: Vec<Span> = Vec::new();

    // Back button (not on first step)
    if step > 0 {
        if !spans.is_empty() {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            "[< Back]",
            Style::new().fg(theme.bottom_bar_fg),
        ));
    }

    // Next / Save
    if !spans.is_empty() {
        spans.push(Span::raw("  "));
    }
    if step < 2 {
        spans.push(Span::styled("[Next >]", theme.sql_focused));
    } else {
        spans.push(Span::styled(
            "[Save]",
            Style::new()
                .bold()
                .fg(theme.summary.fg.unwrap_or(theme.bottom_bar_fg)),
        ));
    }

    // Cancel
    spans.push(Span::raw("  "));
    spans.push(Span::styled(
        "[Cancel]",
        Style::new().fg(theme.bottom_bar_fg),
    ));

    // Test
    spans.push(Span::raw("  "));
    spans.push(Span::styled("[Test]", Style::new().fg(theme.bottom_bar_fg)));

    // Key hints
    let next_label = if step < 2 { "↵ Next" } else { "↵ Save" };
    let hint_text = format!("  ⇥ Tab {} ⎋ Cancel", next_label);
    spans.push(Span::raw(hint_text));

    frame.render_widget(Paragraph::new(Line::from(spans)).right_aligned(), area);
}

fn wizard_hint(step: usize, focus: usize) -> &'static str {
    let fields = wizard_fields_for_step(step);
    fields.get(focus).map(|(_, h)| *h).unwrap_or("")
}
