use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, AppState, Focus, Panel};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let text = match app.state {
        AppState::Executing => Line::from(" Executing..."),
        _ => match (app.active_panel, app.focus) {
            (Panel::Editor, Focus::Input) if app.command_palette_active => {
                Line::from(" Type to filter commands  ↑↓ Navigate  Enter Select  Esc Cancel")
            }
            (Panel::Editor, _) if app.help_overlay_active => {
                Line::from(" Press any key to close help")
            }
            (Panel::Editor, Focus::Input) => Line::from(
                " ↵ Execute  ⌃P Palette  ⌃S Schema  ⌃H History  ⇥ Complete  ? Help  ⌃Q Quit  |  F1 Editor  F2 Conn  F3 Settings",
            ),
            (Panel::Editor, Focus::Results) => Line::from(
                " ⌃V Toggle View  ⌃O Open in Editor  ⌃P Palette  ⌃L/R Scroll  ? Help  |  F1 Editor  F2 Conn  F3 Settings",
            ),
            (Panel::Editor, Focus::SchemaBrowser) => Line::from(
                " ↑↓ Navigate  ↵ Insert SELECT  ⎋ Close  |  F1 Editor  F2 Conn  F3 Settings",
            ),
            (Panel::Editor, Focus::HistoryBrowser) => Line::from(
                " ↑↓ Navigate  ↵ Paste to Input  ⎋ Close  |  F1 Editor  F2 Conn  F3 Settings",
            ),
            (Panel::Connections, Focus::ConnectionsList) => {
                if app.confirm_delete.is_some() {
                    Line::from(" Delete this connection?  y Yes  n No")
                } else {
                    Line::from(
                        " ↑↓ Navigate  ↵ Activate  n New  e Edit  d Delete  ⎋ Back  |  F1 Editor  F2 Conn  F3 Settings",
                    )
                }
            }
            (Panel::Connections, Focus::ConnectionForm) => {
                Line::from(" ⇥ Next Field  ↵ Save  ⎋ Cancel  |  F1 Editor  F2 Conn  F3 Settings")
            }
            (Panel::Settings, Focus::SettingsList) => Line::from(
                " ↑↓ Navigate  ↵ Apply Theme  ⎋ Back  |  F1 Editor  F2 Conn  F3 Settings",
            ),
            _ => Line::from(" F1 Editor  F2 Conn  F3 Settings"),
        },
    };

    let para = Paragraph::new(text)
        .style(Style::new().bg(theme.bottom_bar_bg).fg(theme.bottom_bar_fg))
        .left_aligned();

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(theme.border_secondary);
    frame.render_widget(block, area);
    frame.render_widget(para, area);
}
