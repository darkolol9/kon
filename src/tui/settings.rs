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
        .border_style(app.theme.border_primary);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<ListItem> = theme_names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let is_current = *name == app.theme.name;
            let selected = i == app.settings_selection;

            let marker = if is_current { " ◉ " } else { " ○ " };

            let style = if selected {
                app.theme.picker_selected
            } else if is_current {
                Style::new().bold()
            } else {
                Style::new()
            };

            let t = theme::from_name(name).unwrap_or(app.theme);
            let sample_spans = t.sample();

            let mut spans = Vec::with_capacity(sample_spans.len() + 3);
            spans.push(Span::styled(marker, style));
            spans.push(Span::styled(format!("{:<16}", name), style));
            spans.push(Span::raw(" "));
            spans.extend(sample_spans);

            ListItem::new(Line::from(spans))
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
