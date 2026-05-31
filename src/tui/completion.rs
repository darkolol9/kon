use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::App;

pub fn render(frame: &mut Frame, input_area: Rect, app: &App) {
    let theme = app.theme;
    let candidates = app.completion.candidates();
    if candidates.is_empty() {
        return;
    }

    let max_visible = 8.min(candidates.len());
    let popup_height = max_visible as u16 + 2;
    let popup_width = 48;

    let popup_x = input_area.x + 6;
    let popup_y = input_area.y.saturating_sub(popup_height + 1);

    let available_width = input_area.width.saturating_sub(6);
    let actual_width = popup_width.min(available_width);

    let popup_area = Rect::new(popup_x, popup_y, actual_width, popup_height);

    let items: Vec<ListItem> = candidates
        .iter()
        .enumerate()
        .map(|(i, cand)| {
            let (prefix, prefix_color) = match cand.kind {
                "keyword" => (" K ", theme.completion_kw),
                "table" => (" T ", theme.completion_table),
                "column" => (" C ", theme.completion_column),
                "function" => (" F ", theme.completion_fn),
                "command" => (" M ", theme.completion_command),
                _ => (" ? ", theme.bottom_bar_fg),
            };

            let badge = Span::styled(
                format!(" {} ", prefix.trim()),
                Style::new().fg(theme.bg).bg(prefix_color),
            );

            let suffix = cand
                .table
                .as_ref()
                .map(|t| format!(" ({})", t))
                .unwrap_or_default();

            let selected = i == app.completion.selection();

            let display =
                if selected && cand.display.len() + suffix.len() + 3 > actual_width as usize {
                    let max_d = actual_width as usize - suffix.len() - 4;
                    if max_d > 3 {
                        let mut t: String =
                            cand.display.chars().take(max_d.saturating_sub(3)).collect();
                        t.push_str("...");
                        t
                    } else {
                        cand.display.clone()
                    }
                } else {
                    cand.display.clone()
                };

            let text_style = if selected {
                theme.completion_selected
            } else {
                Style::new()
            };

            ListItem::new(Line::from(vec![
                badge,
                Span::raw(" "),
                Span::styled(display, text_style),
                Span::styled(suffix, Style::new().dim()),
            ]))
        })
        .collect();

    let title = format!(
        " Completions ({}/{}) ",
        app.completion.selection() + 1,
        candidates.len()
    );

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(theme.completion_border),
        )
        .highlight_style(theme.completion_selected);

    frame.render_widget(
        Paragraph::new("").style(Style::new().bg(theme.bg)),
        popup_area,
    );
    frame.render_widget(list, popup_area);
}
