use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::{App, ConnectionMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    match app.conn_mode {
        ConnectionMode::Browse => render_list(frame, area, app),
        ConnectionMode::Adding | ConnectionMode::Editing(_) => render_form(frame, area, app),
    }
}

fn render_list(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let block = Block::default()
        .borders(Borders::ALL)
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
        ("Name", &app.conn_form_name),
        ("Host", &app.conn_form_host),
        ("Port", &app.conn_form_port),
        ("User", &app.conn_form_user),
        ("Password", &app.conn_form_password),
        ("Database", &app.conn_form_database),
    ];

    let mut lines = Vec::new();
    for (i, (label, value)) in fields.iter().enumerate() {
        let is_focused = i == app.conn_form_focus;
        let prefix = if is_focused { "▶ " } else { "  " };

        let label_style = if is_focused {
            theme.sql_focused
        } else {
            Style::new().bold()
        };

        let value_display = if value.is_empty() && !is_focused {
            " (empty) ".to_string()
        } else {
            let masked = *label == "Password" && !value.is_empty();
            if masked {
                "·".repeat(value.len())
            } else {
                value.to_string()
            }
        };

        let value_style = if is_focused {
            Style::new().bg(theme.input_bg).fg(theme.input_fg)
        } else if value.is_empty() {
            Style::new().dim()
        } else {
            Style::new()
        };

        lines.push(Line::from(vec![
            Span::raw(prefix),
            Span::styled(format!("{:<10}", label), label_style),
            Span::raw(" "),
            Span::styled(value_display, value_style),
        ]));
    }

    let para = Paragraph::new(ratatui::text::Text::from(lines)).left_aligned();
    frame.render_widget(para, inner);
}
