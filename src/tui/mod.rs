pub mod bottom_bar;
pub mod completion;
pub mod connections;
pub mod editor;
pub mod format;
pub mod layout;
pub mod overlays;
pub mod settings;
pub mod theme_preview;

pub mod syntax;
pub mod table;
pub mod top_bar;
pub mod vertical;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Clear, Paragraph};

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
        render_toast(frame, layout.content, app, msg);
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

fn render_toast(frame: &mut Frame, content_area: Rect, app: &App, msg: &str) {
    let theme = app.theme;
    let icon = toast_icon(msg);
    let display = format!(" {} {} ", icon, msg);
    let width = (display.len() as u16 + 2).min(content_area.width.saturating_sub(4));
    let x = content_area.x + content_area.width.saturating_sub(width) - 1;
    let y = content_area.y + 1;
    let area = Rect::new(x, y, width, 1);

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(Line::from(display))
            .style(Style::new().bg(theme.toast_bg).fg(theme.toast_fg)),
        area,
    );
}

fn toast_icon(msg: &str) -> &'static str {
    let lower = msg.to_lowercase();
    if lower.contains("failed") || lower.contains("error") {
        "✖"
    } else if lower.contains("successful") {
        "✔"
    } else if lower.contains("refreshed") {
        "↻"
    } else if lower.contains("switched") {
        "◆"
    } else if lower.contains("no database") {
        "⚠"
    } else {
        "·"
    }
}
