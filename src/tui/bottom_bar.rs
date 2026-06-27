use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, AppState, Focus, Panel};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let available = area.width;

    let left_text: Vec<Span> = match app.state {
        AppState::Executing => vec![Span::styled(" ● Executing...", theme.error)],
        _ => match (app.active_panel, app.focus) {
            (Panel::Editor, _) if app.help_overlay_active || app.command_palette_active => {
                vec![shortcut("⎋ Close", theme)]
            }
            (Panel::Editor, Focus::Input) => {
                let mut v = vec![
                    shortcut("↵ Run", theme),
                    sep(),
                    shortcut("^D DB", theme),
                    shortcut("^P Pal", theme),
                    shortcut("^S Sch", theme),
                    sep(),
                    shortcut("^? Help", theme),
                ];
                if app.query_blocks.len() > 1 {
                    v.push(sep());
                    v.push(shortcut("< > Tab", theme));
                }
                v.push(sep());
                v.push(shortcut("F1 Ed", theme));
                v.push(shortcut("F2 Conn", theme));
                v.push(shortcut("F3 Set", theme));
                v
            }
            (Panel::Editor, Focus::Results) => {
                let mut v = vec![
                    shortcut("^V View", theme),
                    shortcut("^O Edit", theme),
                    sep(),
                    shortcut("⇞ Scr", theme),
                ];
                if app.query_blocks.len() > 1 {
                    v.push(sep());
                    v.push(shortcut("< > Tab", theme));
                }
                v.push(sep());
                v.push(shortcut("F1 Ed", theme));
                v.push(shortcut("F2 Conn", theme));
                v.push(shortcut("F3 Set", theme));
                v
            }
            (Panel::Editor, Focus::SchemaBrowser) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Insert", theme),
                sep(),
                shortcut("⎋ Close", theme),
            ],
            (Panel::Editor, Focus::DatabaseBrowser) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Select", theme),
                sep(),
                shortcut("⎋ Close", theme),
            ],
            (Panel::Editor, Focus::HistoryBrowser) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Paste", theme),
                sep(),
                shortcut("⎋ Close", theme),
            ],
            (Panel::Connections, Focus::ConnectionsList) => {
                if app.confirm_delete.is_some() {
                    vec![
                        Span::styled(" Delete? ", theme.error).bold(),
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
                        shortcut("t Test", theme),
                        sep(),
                        shortcut("⎋ Back", theme),
                    ]
                }
            }
            (Panel::Connections, Focus::ConnectionForm) => vec![
                shortcut("⇥ Next", theme),
                shortcut("↵ Next", theme),
                sep(),
                shortcut("⌃E Save", theme),
                shortcut("⌃T Test", theme),
                sep(),
                shortcut("⎋ Cancel", theme),
            ],
            (Panel::Settings, Focus::SettingsList) => vec![
                shortcut("↑↓ Nav", theme),
                shortcut("↵ Apply", theme),
                sep(),
                shortcut("⎋ Back", theme),
            ],
            _ => vec![],
        },
    };

    // Right-aligned info
    let right_text = right_info(app);

    // Build the full line
    let left_len: u16 = left_text
        .iter()
        .map(|s| s.content.len() as u16 + 1)
        .sum::<u16>();
    let right_len: u16 = right_text
        .iter()
        .map(|s| s.content.len() as u16 + 1)
        .sum::<u16>();
    let padding = available.saturating_sub(left_len + right_len);

    let mut spans = left_text;
    if padding > 0 {
        spans.push(Span::raw(" ".repeat(padding as usize)));
    }
    spans.extend(right_text);

    let para = Paragraph::new(Line::from(spans))
        .style(Style::new().bg(theme.bottom_bar_bg))
        .left_aligned();
    frame.render_widget(para, area);
}

fn sep() -> Span<'static> {
    Span::styled(" │ ", Style::new().dim())
}

fn right_info(app: &App) -> Vec<Span<'static>> {
    let total_ms: u128 = app
        .query_blocks
        .iter()
        .filter_map(|b| b.result.as_ref().map(|r| r.execution_time_ms))
        .sum();
    let block_count = app.query_blocks.len();
    vec![
        Span::styled(format!(" {} blocks ", block_count), Style::new().dim()),
        Span::styled(
            if total_ms > 0 {
                format!(" {}ms ", total_ms)
            } else {
                String::new()
            },
            Style::new().dim(),
        ),
    ]
}

fn shortcut(label: &str, theme: &crate::theme::Theme) -> Span<'static> {
    Span::styled(label.to_string(), Style::new().fg(theme.bottom_bar_fg))
}
