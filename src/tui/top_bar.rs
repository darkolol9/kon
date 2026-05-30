use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, Panel};
use crate::theme::Theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let editor_tab = panel_tab(" Editor ", Panel::Editor, app.active_panel, theme);
    let conn_tab = panel_tab(" Connections ", Panel::Connections, app.active_panel, theme);
    let settings_tab = panel_tab(" Settings ", Panel::Settings, app.active_panel, theme);

    let conn_name = format!(" kon · {} ", app.conn_name);

    let line = Line::from(vec![
        editor_tab,
        Span::raw(" "),
        conn_tab,
        Span::raw(" "),
        settings_tab,
        Span::raw(
            " ".repeat(
                area.width
                    .saturating_sub(35)
                    .saturating_sub(conn_name.len() as u16) as usize,
            ),
        ),
        Span::styled(conn_name, Style::new().dim()),
    ]);

    let para = Paragraph::new(line)
        .style(Style::new().bg(theme.top_bar_bg))
        .left_aligned();

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(theme.border_secondary);
    frame.render_widget(block, area);
    frame.render_widget(para, area);
}

fn panel_tab<'a>(label: &'a str, panel: Panel, active: Panel, theme: &Theme) -> Span<'a> {
    if panel == active {
        Span::styled(label, theme.top_bar_active)
    } else {
        Span::styled(label, theme.top_bar_inactive)
    }
}
