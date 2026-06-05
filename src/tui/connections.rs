use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
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
        ("Name", &app.conn_form_name, false),
        ("Host", &app.conn_form_host, false),
        ("Port", &app.conn_form_port, false),
        ("User", &app.conn_form_user, false),
        ("Password", &app.conn_form_password, true),
        ("Database", &app.conn_form_database, false),
    ];

    let constraints: [Constraint; 7] = [
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Fill(1),
    ];
    let rows = Layout::vertical(constraints).split(inner);

    for (i, (label, value, is_password)) in fields.iter().enumerate() {
        let is_focused = i == app.conn_form_focus;
        let row = rows[i];

        let [label_area, value_area] =
            Layout::horizontal([Constraint::Length(14), Constraint::Fill(1)]).areas(row);

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
            Paragraph::new(display).style(inner_style).block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(border_style),
            ),
            value_area,
        );
    }
}
