pub mod bottom_bar;
pub mod completion;
pub mod connections;
pub mod editor;
pub mod format;
pub mod layout;
pub mod overlays;
pub mod settings;
pub mod syntax;
pub mod table;
pub mod top_bar;
pub mod vertical;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use crate::app::{App, Panel};
use layout::AppLayout;

pub fn render(frame: &mut Frame, app: &App) {
    let layout = AppLayout::new(frame.area());

    // Full-screen background
    frame.render_widget(
        Paragraph::new("").style(Style::new().bg(app.theme.bg)),
        frame.area(),
    );

    // Render top sticky bar
    top_bar::render(frame, layout.top_bar, app);

    // Render active panel content
    match app.active_panel {
        Panel::Editor => {
            editor::render(frame, layout.content, app);
        }
        Panel::Connections => {
            connections::render(frame, layout.content, app);
        }
        Panel::Settings => {
            settings::render(frame, layout.content, app);
        }
    }

    // Render toast notification
    if let Some((msg, _instant)) = &app.toast {
        render_toast(frame, layout.content, msg);
    }

    // Global overlays (drawn last, on top)
    if app.command_palette_active {
        overlays::render_command_palette(frame, layout.content, app);
    }
    if app.help_overlay_active {
        overlays::render_help_overlay(frame, frame.area(), app);
    }

    // Render bottom sticky bar
    bottom_bar::render(frame, layout.bottom_bar, app);
}

fn render_toast(frame: &mut Frame, content_area: Rect, msg: &str) {
    let width = (msg.len() as u16 + 4).min(content_area.width.saturating_sub(4));
    let x = content_area.x + content_area.width.saturating_sub(width) - 1;
    let y = content_area.y + 1;
    let area = Rect::new(x, y, width, 1);

    frame.render_widget(
        Paragraph::new(Line::from(format!(" {} ", msg))).style(Style::new()),
        area,
    );
}
