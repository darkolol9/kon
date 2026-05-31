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
use ratatui::style::Style;
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
            // Completion popup above the input bar
            if app.completion_active && !app.command_palette_active {
                let input_height = 3u16;
                let input_area = ratatui::layout::Rect::new(
                    layout.content.x,
                    layout.content.y + layout.content.height.saturating_sub(input_height),
                    layout.content.width,
                    input_height,
                );
                completion::render(frame, input_area, app);
            }
        }
        Panel::Connections => {
            connections::render(frame, layout.content, app);
        }
        Panel::Settings => {
            settings::render(frame, layout.content, app);
        }
    }

    // Global overlays (drawn last, on top)
    if app.command_palette_active {
        overlays::render_command_palette(frame, layout.content, app);
    }
    if app.help_overlay_active {
        overlays::render_help_overlay(frame, frame.area(), app);
    }
    if app.database_browser_visible {
        overlays::render_database_browser(frame, frame.area(), app);
    }

    // Render bottom sticky bar
    bottom_bar::render(frame, layout.bottom_bar, app);
}
