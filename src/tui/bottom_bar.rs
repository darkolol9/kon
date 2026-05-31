use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, AppState, Focus, Panel};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;

    let text: Vec<Span> = match app.state {
        AppState::Executing => vec![
            Span::styled(" ● ", theme.error),
            Span::styled("Executing...", Style::new().fg(theme.bottom_bar_fg)),
        ],
        _ => match (app.active_panel, app.focus) {
            (Panel::Editor, _) if app.help_overlay_active || app.command_palette_active => {
                vec![Span::styled(
                    " ⎋ Close ",
                    Style::new().fg(theme.bottom_bar_fg),
                )]
            }
            (Panel::Editor, Focus::Input) => vec![
                shortcut("↵ Execute", theme),
                sep(),
                shortcut("⌃D Browsers", theme),
                shortcut("⌃P Palette", theme),
                shortcut("⌃S Schema", theme),
                sep(),
                shortcut("⇞ Scroll", theme),
                shortcut("? Help", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            (Panel::Editor, Focus::Results) => vec![
                shortcut("⌃V View", theme),
                shortcut("⌃O Editor", theme),
                sep(),
                shortcut("⇞ Scroll", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            (Panel::Editor, Focus::SchemaBrowser) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Insert", theme),
                sep(),
                shortcut("⎋ Close", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            (Panel::Editor, Focus::DatabaseBrowser) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Select", theme),
                sep(),
                shortcut("⎋ Close", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            (Panel::Editor, Focus::HistoryBrowser) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Paste", theme),
                sep(),
                shortcut("⎋ Close", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            (Panel::Connections, Focus::ConnectionsList) => {
                if app.confirm_delete.is_some() {
                    vec![
                        Span::styled(
                            " Delete? ",
                            theme.error.add_modifier(ratatui::style::Modifier::BOLD),
                        ),
                        shortcut("y Yes", theme),
                        shortcut("n No", theme),
                    ]
                } else {
                    vec![
                        shortcut("↑↓ Nav", theme),
                        shortcut("↵ Activate", theme),
                        sep(),
                        shortcut("n New", theme),
                        shortcut("e Edit", theme),
                        shortcut("d Delete", theme),
                        sep(),
                        shortcut("⎋ Back", theme),
                        sep(),
                        shortcut("F1 Ed", theme),
                        shortcut("F2 Conn", theme),
                        shortcut("F3 Set", theme),
                    ]
                }
            }
            (Panel::Connections, Focus::ConnectionForm) => vec![
                shortcut("⇥ Next", theme),
                shortcut("↵ Save", theme),
                shortcut("⎋ Cancel", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            (Panel::Settings, Focus::SettingsList) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Apply", theme),
                sep(),
                shortcut("⎋ Back", theme),
                sep(),
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
            _ => vec![
                shortcut("F1 Ed", theme),
                shortcut("F2 Conn", theme),
                shortcut("F3 Set", theme),
            ],
        },
    };

    let para = Paragraph::new(Line::from(text))
        .style(Style::new().bg(theme.bottom_bar_bg))
        .left_aligned();
    frame.render_widget(para, area);
}

fn sep() -> Span<'static> {
    Span::styled(" │ ", Style::new().dim())
}

fn shortcut(label: &str, theme: &crate::theme::Theme) -> Span<'static> {
    Span::styled(label.to_string(), Style::new().fg(theme.bottom_bar_fg))
}
