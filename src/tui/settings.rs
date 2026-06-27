use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph};

use crate::app::App;
use crate::theme;
use crate::tui::theme_preview;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme_names = theme::theme_names();

    let block = Block::bordered()
        .title(" Settings — Theme ")
        .border_style(app.theme.border_primary);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.width < 60 || inner.height < 8 {
        // Too narrow — render list only
        render_list(frame, inner, app, &theme_names);
        return;
    }

    let [list_area, preview_area] =
        Layout::horizontal([Constraint::Length(34), Constraint::Fill(1)])
            .spacing(1)
            .areas(inner);

    render_list(frame, list_area, app, &theme_names);

    // Preview the hovered theme
    if let Some(preview_theme) = theme::ALL_THEMES.get(app.settings_selection).copied() {
        let preview_block = Block::bordered()
            .title(" Preview ")
            .border_style(preview_theme.border_secondary);
        let p_inner = preview_block.inner(preview_area);
        frame.render_widget(preview_block, preview_area);
        theme_preview::render_preview(frame, p_inner, preview_theme);
    } else {
        let para = Paragraph::new(" (select a theme) ")
            .style(Style::new().dim())
            .left_aligned();
        frame.render_widget(para, preview_area);
    }
}

fn render_list(frame: &mut Frame, area: Rect, app: &App, theme_names: &[&'static str]) {
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
                Style::new().fg(app.theme.input_fg).bold()
            } else {
                Style::new().fg(app.theme.input_fg)
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
        frame.render_widget(para, area);
        return;
    }

    let list = List::new(items).highlight_style(app.theme.picker_selected);
    frame.render_widget(list, area);
}
