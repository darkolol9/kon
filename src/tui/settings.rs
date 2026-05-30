use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::App;
use crate::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme_names = theme::theme_names();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Settings — Theme ")
        .border_style(app.theme.completion_border);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = theme_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let is_current = *name == app.theme.name;
            let selected = i == app.settings_selection;

            let marker = if is_current { " ◉ " } else { " ○ " };
            let display = format!("{}{}", marker, name);

            let style = if selected {
                app.theme.picker_selected
            } else if is_current {
                Style::new().bold()
            } else {
                Style::new()
            };

            ListItem::new(Line::from(Span::styled(display, style)))
        })
        .collect();

    if items.is_empty() {
        let para = Paragraph::new(" (no themes available) ")
            .style(Style::new().dim())
            .left_aligned();
        frame.render_widget(para, inner);
        return;
    }

    let list = List::new(items).highlight_style(app.theme.picker_selected);
    frame.render_widget(list, inner);
}
