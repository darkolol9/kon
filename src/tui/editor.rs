use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, List, Paragraph};

use crate::app::{App, AppState, ViewMode};
use crate::theme::Theme;
use crate::tui::{completion, syntax, table, vertical};

pub fn render(frame: &mut Frame, content_area: Rect, app: &App) {
    let theme = app.theme;

    let editor_area = if app.db_browser_visible {
        let panel_width = 28u16.min(content_area.width.saturating_sub(4));
        let side_area = Rect::new(
            content_area.x,
            content_area.y,
            panel_width,
            content_area.height,
        );
        let right_area = Rect::new(
            content_area.x + panel_width,
            content_area.y,
            content_area.width.saturating_sub(panel_width),
            content_area.height,
        );
        render_database_panel(frame, side_area, app, theme);
        right_area
    } else {
        content_area
    };

    let input_height = 1u16;
    let results_area = Rect::new(
        editor_area.x,
        editor_area.y,
        editor_area.width,
        editor_area.height.saturating_sub(input_height + 1),
    );
    let sep_area = Rect::new(
        editor_area.x,
        editor_area.y + results_area.height,
        editor_area.width,
        1,
    );
    let input_area = Rect::new(
        editor_area.x,
        editor_area.y + results_area.height + 1,
        editor_area.width,
        input_height,
    );

    render_results(frame, results_area, app, theme);
    render_separator(frame, sep_area, theme);
    render_input(frame, input_area, app, theme);

    if app.completion_active && !app.command_palette_active {
        completion::render(frame, input_area, app);
    }
}

fn render_separator(frame: &mut Frame, area: Rect, theme: &Theme) {
    let sep = Line::from(Span::styled(
        "─".repeat(area.width as usize),
        theme.border_secondary,
    ));
    frame.render_widget(Paragraph::new(sep).style(Style::new().bg(theme.bg)), area);
}

fn render_database_panel(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let block = Block::bordered()
        .title(" Databases ")
        .border_style(theme.border_primary);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.db_browser_fetching {
        let para = Paragraph::new(Line::from(" Loading...")).style(Style::new().dim());
        frame.render_widget(para, inner);
        return;
    }

    if let Some(err) = &app.db_browser_error {
        let para = Paragraph::new(Line::from(vec![
            Span::raw(" Error: "),
            Span::styled(err.as_str(), theme.error),
        ]));
        frame.render_widget(para, inner);
        return;
    }

    let current_db = app.conn_name.split(" > ").last().unwrap_or("");

    let items: Vec<Line> = app
        .db_browser_databases
        .iter()
        .enumerate()
        .map(|(i, db)| {
            let selected = i == app.db_browser_selection;
            let is_current = db == current_db;
            let prefix = if is_current { " ◉ " } else { "   " };
            let style = if selected {
                theme.completion_selected
            } else if is_current {
                Style::new().fg(theme.completion_kw).bold()
            } else {
                Style::new().fg(theme.input_fg)
            };
            Line::from(Span::styled(format!("{}{}", prefix, db), style))
        })
        .collect();

    let list = List::new(items).highlight_style(theme.completion_selected);
    frame.render_widget(list, inner);
}

fn render_results(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    if app.query_blocks.is_empty() {
        let text = Text::from(Line::from(vec![
            Span::raw("  "),
            Span::styled("◆", theme.summary),
            Span::raw(" Type a SQL query and press "),
            Span::styled("Enter", theme.syntax_keyword),
            Span::raw(" to execute."),
        ]));
        let para = Paragraph::new(text)
            .style(Style::new().dim())
            .left_aligned();
        frame.render_widget(para, area);
        return;
    }

    let total = app.query_blocks.len();
    let scroll_blocks = app.scroll.min(total.saturating_sub(1));
    let start = total.saturating_sub(1).saturating_sub(scroll_blocks);

    let mut y = area.y;
    let bottom = area.y + area.height;

    for idx in start..total {
        if y >= bottom {
            break;
        }

        let qb = &app.query_blocks[idx];
        let focused = idx == app.active_block;

        // Block header with number badge
        let header = if focused {
            Line::from(vec![
                Span::styled(" ", Style::new()),
                Span::styled(
                    format!("[{}]", idx + 1),
                    Style::new().fg(theme.bg).bg(theme.completion_kw),
                ),
                Span::raw(" "),
                Span::styled(qb.sql.as_str(), theme.sql_focused),
            ])
        } else {
            Line::from(vec![
                Span::styled(" ", Style::new()),
                Span::styled(
                    format!("[{}]", idx + 1),
                    Style::new().fg(theme.border_secondary.fg.unwrap_or(theme.bottom_bar_fg)),
                ),
                Span::raw(" "),
                Span::styled(qb.sql.as_str(), theme.sql_unfocused),
            ])
        };
        frame.render_widget(Paragraph::new(header), Rect::new(area.x, y, area.width, 1));
        y += 1;
        if y >= bottom {
            continue;
        }

        let remaining = bottom - y;

        if let Some(err) = &qb.error {
            // Error card
            let err_block = Block::bordered().title(" Error ").border_style(theme.error);
            let err_inner = err_block.inner(Rect::new(area.x, y, area.width, remaining.min(3)));
            frame.render_widget(
                err_block,
                Rect::new(area.x, y, area.width, remaining.min(3)),
            );
            let err_line = Line::from(vec![
                Span::styled("  ", theme.error),
                Span::styled(err.as_str(), theme.error),
            ]);
            frame.render_widget(Paragraph::new(err_line), err_inner);
            y += remaining.min(3);
        } else if let Some(result) = &qb.result {
            if result.columns.is_empty() {
                // DML result
                let summary = format!("✔ {} row(s) affected", result.rows_affected);
                frame.render_widget(
                    Paragraph::new(Line::from(vec![
                        Span::styled("   ", Style::new()),
                        Span::styled(summary, theme.summary),
                    ])),
                    Rect::new(area.x, y, area.width, 1),
                );
                y += 1;
                if y < bottom {
                    frame.render_widget(
                        Paragraph::new(Line::from(vec![
                            Span::styled("   ", Style::new()),
                            Span::styled(
                                format!("({} ms)", result.execution_time_ms),
                                Style::new().dim(),
                            ),
                        ])),
                        Rect::new(area.x, y, area.width, 1),
                    );
                    y += 1;
                }
            } else if matches!(qb.view_mode, ViewMode::Table) {
                let use_scrollbar = result.rows.len() > remaining as usize;
                let table_width = if use_scrollbar {
                    area.width.saturating_sub(1)
                } else {
                    area.width
                };
                let table_area = Rect::new(area.x, y, table_width, remaining);
                let row_offset = qb.block_row_scroll;
                table::render_table(
                    frame,
                    table_area,
                    result,
                    theme,
                    focused,
                    app.scroll_x as u16,
                    row_offset,
                );
                if use_scrollbar {
                    render_scrollbar(
                        frame,
                        Rect::new(area.x + table_width, y, 1, remaining),
                        result.rows.len(),
                        remaining as usize,
                        row_offset,
                        theme.scrollbar_thumb,
                        theme.scrollbar_track,
                    );
                }

                let visible_rows = (result.rows.len() as u16).min(remaining.saturating_sub(1));
                let summary_y = y + 1 + visible_rows;
                if summary_y < bottom {
                    let summary = format!(
                        "{} row(s) in set ({} ms)",
                        result.rows_affected, result.execution_time_ms
                    );
                    frame.render_widget(
                        Paragraph::new(Line::from(summary).style(theme.summary)),
                        Rect::new(area.x, summary_y, area.width, 1),
                    );
                    y = summary_y + 1;
                } else {
                    y = bottom;
                }
            } else {
                let use_scrollbar = result.rows.len() > remaining as usize;
                let vert_width = if use_scrollbar {
                    area.width.saturating_sub(1)
                } else {
                    area.width
                };
                let row_offset = qb.block_row_scroll;
                let lines = vertical::render_vertical_lines(result, theme, row_offset);
                frame.render_widget(
                    Paragraph::new(Text::from(lines)).left_aligned(),
                    Rect::new(area.x, y, vert_width, remaining),
                );
                if use_scrollbar {
                    render_scrollbar(
                        frame,
                        Rect::new(area.x + vert_width, y, 1, remaining),
                        result.rows.len(),
                        remaining as usize,
                        row_offset,
                        theme.scrollbar_thumb,
                        theme.scrollbar_track,
                    );
                }
                y = bottom;
            }
        }

        if y < bottom {
            y += 1;
        }
    }
}

fn render_input(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let prefix = format!(" {}> ", app.conn_name);

    let input_line = if matches!(app.state, AppState::Executing) {
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let idx = (app.spinner_tick as usize) % spinner_chars.len();
        Line::from(vec![
            Span::styled(format!(" {} ", spinner_chars[idx]), theme.error),
            Span::styled("Executing...", Style::new().dim()),
        ])
    } else {
        let tokens = syntax::highlight(&app.input, theme);
        let mut spans: Vec<Span> = Vec::with_capacity(tokens.len() + 1);
        let mut pos = 0;
        for ht in &tokens {
            if ht.start > pos {
                spans.push(Span::raw(&app.input[pos..ht.start]));
            }
            let text = &app.input[ht.start..ht.end.min(app.input.len())];
            spans.push(Span::styled(text.to_string(), ht.style));
            pos = ht.end.min(app.input.len());
        }
        if pos < app.input.len() {
            spans.push(Span::raw(&app.input[pos..]));
        }
        if spans.is_empty() && !app.input.is_empty() {
            spans.push(Span::raw(&app.input));
        }
        Line::from(spans)
    };

    let full_line = Line::from(
        std::iter::once(Span::styled(&prefix, Style::new().fg(theme.completion_kw)))
            .chain(input_line.spans.iter().cloned())
            .collect::<Vec<_>>(),
    );

    let inner_style = Style::new().bg(theme.input_bg).fg(theme.input_fg);

    let para = Paragraph::new(Text::from(full_line))
        .style(inner_style)
        .left_aligned();
    frame.render_widget(para, area);

    let cursor_x = area.x + prefix.len() as u16 + app.cursor as u16;
    let cursor_y = area.y;
    frame.set_cursor_position((cursor_x.min(area.right().saturating_sub(1)), cursor_y));
}

fn render_scrollbar(
    frame: &mut Frame,
    area: Rect,
    total: usize,
    visible: usize,
    offset: usize,
    thumb_style: Style,
    track_style: Style,
) {
    if total <= visible || area.width == 0 || area.height == 0 {
        return;
    }
    let track_h = area.height as usize;
    let thumb_h = ((visible as f64 / total as f64) * track_h as f64)
        .ceil()
        .max(1.0) as usize;
    let max_offset = total.saturating_sub(visible);
    let thumb_y = if max_offset > 0 {
        ((offset as f64 / max_offset as f64) * (track_h - thumb_h) as f64) as usize
    } else {
        0
    };

    for i in 0..track_h {
        let style = if i >= thumb_y && i < thumb_y + thumb_h {
            thumb_style
        } else {
            track_style
        };
        frame.render_widget(
            Paragraph::new("▐").style(style),
            Rect::new(area.x, area.y + i as u16, 1, 1),
        );
    }
}
