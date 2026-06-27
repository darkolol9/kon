use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, List, ListItem, Paragraph};

use crate::app::App;

// ── Command Palette ──

pub fn render_command_palette(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let candidates = &app.command_palette_candidates;
    if candidates.is_empty() {
        return;
    }

    let max_visible = 12.min(candidates.len());
    let popup_height = max_visible as u16 + 4;
    let popup_width = (area.width as f64 * 0.5).clamp(42.0, 60.0) as u16;

    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 3;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, popup_area);

    let block = Block::bordered()
        .title(" Command Palette ")
        .border_style(theme.completion_border);
    let inner = block.inner(popup_area);
    frame.render_widget(&block, popup_area);

    let [input_area, sep_area, list_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .areas(inner);

    let input_display = format!(" ▸ {}", app.command_palette_input);
    let input_style = if app.command_palette_input.is_empty() {
        Style::new().dim().bg(theme.input_bg)
    } else {
        Style::new().bg(theme.input_bg).fg(theme.input_fg)
    };
    let para = Paragraph::new(Line::from(Span::styled(input_display, input_style))).left_aligned();
    frame.render_widget(para, input_area);

    // Separator
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(popup_width.saturating_sub(4) as usize),
            theme.border_secondary,
        ))),
        sep_area,
    );

    let items_vec: Vec<ListItem> = candidates
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let selected = i == app.command_palette_selection;
            let style = if selected {
                theme.command_palette_selected
            } else {
                Style::new()
            };

            let badge = Span::styled(if selected { " ▶ " } else { "   " }, style);

            ListItem::new(Line::from(vec![
                badge,
                Span::styled(format!("/{:<12}", entry.name), style),
                Span::styled(
                    entry.desc,
                    if selected { style } else { Style::new().dim() },
                ),
            ]))
        })
        .collect();

    let list = List::new(items_vec).highlight_style(theme.command_palette_selected);
    frame.render_widget(list, list_area);

    // Count badge at top right
    let count_text = format!(" {} results ", candidates.len());
    let count_width = count_text.len() as u16;
    let count_area = Rect::new(
        popup_area.x + popup_area.width.saturating_sub(count_width + 1),
        popup_area.y,
        count_width,
        1,
    );
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(count_text, Style::new().dim()))),
        count_area,
    );
}

// ── Help Overlay ──

struct Shortcut {
    key: &'static str,
    desc: &'static str,
}

const SECTIONS: &[(&str, &[Shortcut])] = &[
    (
        "Panel Navigation",
        &[
            Shortcut {
                key: "F1",
                desc: "Switch to Editor panel",
            },
            Shortcut {
                key: "F2",
                desc: "Switch to Connections panel",
            },
            Shortcut {
                key: "F3",
                desc: "Switch to Settings panel",
            },
            Shortcut {
                key: "Alt+←/→",
                desc: "Switch result tab",
            },
        ],
    ),
    (
        "Editor",
        &[
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
                key: "Ctrl+P",
                desc: "Command palette",
            },
            Shortcut {
                key: "Ctrl+?",
                desc: "Show help",
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
                key: "Ctrl+←/→",
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
                desc: "List database tables",
            },
            Shortcut {
                key: "/refresh",
                desc: "Refresh schema cache",
            },
            Shortcut {
                key: "/quit",
                desc: "Exit kon",
            },
        ],
    ),
    (
        "General",
        &[Shortcut {
            key: "Ctrl+C/Q",
            desc: "Quit",
        }],
    ),
];

pub fn render_help_overlay(frame: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme;
    let popup_width = (area.width as f64 * 0.55).clamp(48.0, 64.0) as u16;
    let popup_height = (area.height as f64 * 0.8).clamp(20.0, 28.0) as u16;

    let popup_x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    frame.render_widget(Clear, popup_area);

    let block = Block::bordered()
        .title(" Help ")
        .border_style(theme.completion_border);
    let inner = block.inner(popup_area);
    frame.render_widget(&block, popup_area);

    // Build sections using two-column layout for compactness
    let mut lines = Vec::new();
    for (section_idx, (section_name, shortcuts)) in SECTIONS.iter().enumerate() {
        if section_idx > 0 {
            lines.push(Line::from(""));
        }

        // Section header with underline
        let section_line = format!(" {} ", section_name);
        lines.push(Line::from(Span::styled(
            section_line.clone(),
            theme.help_section,
        )));
        lines.push(Line::from(Span::styled(
            "─".repeat(section_line.len()),
            Style::new().dim(),
        )));

        let max_key_len = shortcuts.iter().map(|s| s.key.len()).max().unwrap_or(0);
        for sc in *shortcuts {
            let padded = format!("{:width$}", sc.key, width = max_key_len);
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(padded, theme.help_key),
                Span::styled(" ─ ", Style::new().dim()),
                Span::styled(sc.desc, theme.help_desc),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!(" {}─── Press any key to close ─── ", " "),
        Style::new().dim(),
    )));

    let para = Paragraph::new(ratatui::text::Text::from(lines)).left_aligned();
    frame.render_widget(para, inner);
}
