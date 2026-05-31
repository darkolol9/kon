use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::App;

// ── Command Palette ──

pub fn render_command_palette(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let candidates = &app.command_palette_candidates;
    if candidates.is_empty() {
        return;
    }

    let max_visible = 10.min(candidates.len());
    let popup_height = max_visible as u16 + 4;
    let popup_width = 52;

    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(
        Paragraph::new("").style(Style::new().bg(theme.bg)),
        popup_area,
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Command Palette ")
        .border_style(theme.completion_border);
    let inner = block.inner(popup_area);
    frame.render_widget(&block, popup_area);

    let [input_area, list_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(inner);

    let input_display = format!(" > {}", app.command_palette_input);
    let para = Paragraph::new(Line::from(Span::raw(input_display)))
        .style(Style::new().bg(theme.input_bg).fg(theme.input_fg));
    frame.render_widget(para, input_area);

    let items: Vec<ListItem> = candidates
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == app.command_palette_selection;
            let style = if selected {
                theme.command_palette_selected
            } else {
                Style::new()
            };
            let desc_style = if selected {
                theme.command_palette_selected
            } else {
                Style::new().dim()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" /{}", entry.name), style),
                Span::styled(format!("  {}", entry.desc), desc_style),
            ]))
        })
        .collect();

    let list = List::new(items).highlight_style(theme.command_palette_selected);
    frame.render_widget(list, list_area);
}

// ── Help Overlay ──

struct Shortcut {
    key: &'static str,
    desc: &'static str,
}

const SECTIONS: &[(&str, &[Shortcut])] = &[
    (
        "Navigation",
        &[
            Shortcut {
                key: "Alt+H/L",
                desc: "Switch result tab",
            },
            Shortcut {
                key: "Ctrl+D",
                desc: "Database browser",
            },
            Shortcut {
                key: "Ctrl+S",
                desc: "Toggle schema browser",
            },
            Shortcut {
                key: "Ctrl+H",
                desc: "Toggle history browser",
            },
            Shortcut {
                key: "PgUp/PgDn",
                desc: "Scroll results",
            },
        ],
    ),
    (
        "Input",
        &[
            Shortcut {
                key: "Enter",
                desc: "Execute query",
            },
            Shortcut {
                key: "Up/Down",
                desc: "History navigation",
            },
            Shortcut {
                key: "Left/Right",
                desc: "Move cursor",
            },
            Shortcut {
                key: "Home/End",
                desc: "Jump to start/end",
            },
            Shortcut {
                key: "Tab",
                desc: "Autocomplete",
            },
            Shortcut {
                key: "Esc",
                desc: "Close popup / cancel",
            },
        ],
    ),
    (
        "Results",
        &[
            Shortcut {
                key: "PgUp/PgDn",
                desc: "Scroll results vertically",
            },
            Shortcut {
                key: "Ctrl+V",
                desc: "Toggle table/vertical view",
            },
            Shortcut {
                key: "Ctrl+O",
                desc: "Open last result in editor",
            },
            Shortcut {
                key: "Ctrl+R",
                desc: "Refresh schema cache",
            },
            Shortcut {
                key: "Ctrl+L/R",
                desc: "Horizontal scroll",
            },
        ],
    ),
    (
        "Commands",
        &[
            Shortcut {
                key: "/theme",
                desc: "Change color theme",
            },
            Shortcut {
                key: "/help",
                desc: "Show this help",
            },
            Shortcut {
                key: "/clear",
                desc: "Clear all results",
            },
            Shortcut {
                key: "/tables",
                desc: "SHOW TABLES",
            },
            Shortcut {
                key: "/refresh",
                desc: "Refresh schema",
            },
            Shortcut {
                key: "/quit",
                desc: "Exit",
            },
            Shortcut {
                key: "Ctrl+P",
                desc: "Command palette",
            },
        ],
    ),
    (
        "General",
        &[
            Shortcut {
                key: "Ctrl+?",
                desc: "Show/hide this help",
            },
            Shortcut {
                key: "Ctrl+C/Q",
                desc: "Quit",
            },
        ],
    ),
];

pub fn render_help_overlay(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let popup_width = 56u16.min(area.width.saturating_sub(4));
    let popup_height = 22u16.min(area.height.saturating_sub(2));

    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(
        Paragraph::new("").style(Style::new().bg(theme.bg)),
        popup_area,
    );

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Help — Keyboard Shortcuts ")
        .border_style(theme.completion_border);
    let inner = block.inner(popup_area);
    frame.render_widget(&block, popup_area);

    let mut lines = Vec::new();
    for (section_idx, (section_name, shortcuts)) in SECTIONS.iter().enumerate() {
        if section_idx > 0 {
            lines.push(Line::from(""));
        }
        lines.push(Line::from(Span::styled(
            format!(" {} ", section_name),
            theme.help_section,
        )));
        for sc in *shortcuts {
            lines.push(Line::from(vec![
                Span::styled(format!("   {:<14}", sc.key), theme.help_key),
                Span::styled(sc.desc, theme.help_desc),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Press any key to close ",
        Style::new().dim(),
    )));

    let para = Paragraph::new(ratatui::text::Text::from(lines)).left_aligned();
    frame.render_widget(para, inner);
}
