use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::{App, AppState, Focus, Panel};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let available = area.width;

    let text: Vec<Span> = match app.state {
        AppState::Executing => vec![
            Span::styled(" ● ", theme.error),
            Span::styled("Executing...", Style::new().fg(theme.bottom_bar_fg)),
        ],
        _ => match (app.active_panel, app.focus) {
            (Panel::Editor, _) if app.help_overlay_active || app.command_palette_active => {
                fit_spans(available, vec![group(" ⎋ Close ", 0, theme)], theme)
            }
            (Panel::Editor, Focus::Input) => fit_spans(
                available,
                vec![
                    group("↵ Execute", 0, theme),
                    group("⌃D Browsers", 1, theme),
                    group("⌃P Palette", 1, theme),
                    group("⌃S Schema", 1, theme),
                    group("⇞ Scroll", 2, theme),
                    group("? Help", 2, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            (Panel::Editor, Focus::Results) => fit_spans(
                available,
                vec![
                    group("⌃V View", 0, theme),
                    group("⌃O Editor", 0, theme),
                    group("⇞ Scroll", 2, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            (Panel::Editor, Focus::SchemaBrowser) => fit_spans(
                available,
                vec![
                    group("↑↓ Nav", 0, theme),
                    group("↵ Insert", 0, theme),
                    group("⎋ Close", 3, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            (Panel::Editor, Focus::DatabaseBrowser) => fit_spans(
                available,
                vec![
                    group("↑↓ Nav", 0, theme),
                    group("↵ Select", 0, theme),
                    group("⎋ Close", 3, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            (Panel::Editor, Focus::HistoryBrowser) => fit_spans(
                available,
                vec![
                    group("↑↓ Nav", 0, theme),
                    group("↵ Paste", 0, theme),
                    group("⎋ Close", 3, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            (Panel::Connections, Focus::ConnectionsList) => {
                if app.confirm_delete.is_some() {
                    vec![
                        Span::styled(" Delete? ", theme.error.add_modifier(Modifier::BOLD)),
                        shortcut("y Yes", theme),
                        shortcut("n No", theme),
                    ]
                } else {
                    fit_spans(
                        available,
                        vec![
                            group("↑↓ Nav", 0, theme),
                            group("↵ Activate", 0, theme),
                            group("n New", 1, theme),
                            group("e Edit", 1, theme),
                            group("d Delete", 1, theme),
                            group("t Test", 1, theme),
                            group("⎋ Back", 3, theme),
                            group("[Ed]", 4, theme),
                            group("[Conn]", 4, theme),
                            group("[Set]", 4, theme),
                        ],
                        theme,
                    )
                }
            }
            (Panel::Connections, Focus::ConnectionForm) => fit_spans(
                available,
                vec![
                    group("⇥ Next", 0, theme),
                    group("↵ Next →", 0, theme),
                    group("⌃E Save", 1, theme),
                    group("⌃T Test", 1, theme),
                    group("⎋ Cancel", 3, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            (Panel::Settings, Focus::SettingsList) => fit_spans(
                available,
                vec![
                    group("↑↓ Nav", 0, theme),
                    group("↵ Apply", 0, theme),
                    group("⎋ Back", 3, theme),
                    group("[Ed]", 4, theme),
                    group("[Conn]", 4, theme),
                    group("[Set]", 4, theme),
                ],
                theme,
            ),
            _ => vec![
                shortcut("[Ed]", theme),
                shortcut("[Conn]", theme),
                shortcut("[Set]", theme),
            ],
        },
    };

    let para = Paragraph::new(Line::from(text))
        .style(Style::new().bg(theme.bottom_bar_bg))
        .left_aligned();
    frame.render_widget(para, area);
}

struct G {
    label: &'static str,
    priority: u8,
}

fn group(label: &'static str, priority: u8, _theme: &crate::theme::Theme) -> G {
    G { label, priority }
}

fn fit_spans(
    available: u16,
    mut groups: Vec<G>,
    theme: &crate::theme::Theme,
) -> Vec<Span<'static>> {
    // Sort by priority (ascending = more important)
    groups.sort_by_key(|g| g.priority);

    let mut spans = Vec::new();
    let mut used: u16 = 0;

    for (i, g) in groups.iter().enumerate() {
        let label_cost = g.label.len() as u16;
        let sep_cost = if i > 0 { 2 } else { 0 }; // " " before each group
        let total = if i > 0 {
            used + sep_cost + label_cost
        } else {
            label_cost
        };

        if total > available.saturating_sub(2) {
            // Not enough room - add ellipsis if we have any spans
            if !spans.is_empty() {
                spans.push(Span::styled(" …", Style::new().dim()));
            }
            break;
        }

        if i > 0 {
            used += sep_cost;
        }

        let group_spans = shortcut(g.label, theme);
        if i > 0 {
            spans.push(Span::styled(" ", Style::new().dim()));
        }
        spans.push(group_spans);
        used = label_cost + if i > 0 { used + 1 } else { label_cost };
        if i > 0 {
            used += 1; // the space separator
        }
    }

    spans
}

fn shortcut(label: &str, theme: &crate::theme::Theme) -> Span<'static> {
    Span::styled(label.to_string(), Style::new().fg(theme.bottom_bar_fg))
}
