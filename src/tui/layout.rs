use ratatui::layout::{Constraint, Layout, Rect};

/// The three vertical sections of the UI:
///   [0] top bar (1 row)
///   [1] main content (fills remaining)
///   [2] bottom bar (1 row)
pub struct AppLayout {
    pub top_bar: Rect,
    pub content: Rect,
    pub bottom_bar: Rect,
}

impl AppLayout {
    pub fn new(area: Rect) -> Self {
        let [top_bar, content, bottom_bar] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .spacing(0)
        .areas(area);

        Self {
            top_bar,
            content,
            bottom_bar,
        }
    }
}
