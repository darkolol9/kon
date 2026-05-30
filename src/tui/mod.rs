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

use crate::app::{App, Panel};
use layout::AppLayout;

pub fn render(frame: &mut Frame, app: &App) {
    let layout = AppLayout::new(frame.area());

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
