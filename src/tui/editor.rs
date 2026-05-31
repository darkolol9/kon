use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Paragraph};

use crate::app::{App, AppState, ViewMode};
use crate::theme::Theme;
use crate::tui::{completion, syntax, table, vertical};

pub fn render(frame: &mut Frame, content_area: Rect, app: &App) {
    let theme = app.theme;

    let editor_area = if app.db_browser_visible {
        let panel_width = 22u16.min(content_area.width.saturating_sub(4));
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

    let input_height = 3u16;
    let results_area = Rect::new(
        editor_area.x,
        editor_area.y,
        editor_area.width,
        editor_area.height.saturating_sub(input_height),
    );
    let input_area = Rect::new(
        editor_area.x,
        editor_area.y + results_area.height,
        editor_area.width,
        input_height,
    );

    render_results(frame, results_area, app, theme);
    render_input(frame, input_area, app, theme);

    if app.completion_active && !app.command_palette_active {
        completion::render(frame, input_area, app);
    }
}

fn render_database_panel(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let block = Block::bordered()
        .title(" Databases ")
        .border_style(theme.schema_browser_border);
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
    let bottom = inner.y + inner.height;

    for (y, (i, db)) in (inner.y..).zip(app.db_browser_databases.iter().enumerate()) {
        if y >= bottom {
            break;
        }
        let selected = i == app.db_browser_selection;
        let is_current = db == current_db;

        let prefix = if is_current { " ◉ " } else { "   " };
        let text = format!("{}{}", prefix, db);

        let style = if selected {
            theme.completion_selected
        } else if is_current {
            Style::new().fg(theme.completion_kw).bold()
        } else {
            Style::new()
        };

        frame.render_widget(
            Paragraph::new(Line::from(text)).style(style),
            Rect::new(inner.x, y, inner.width, 1),
        );
    }

    if app.db_browser_databases.is_empty() && !app.db_browser_fetching {
        let para = Paragraph::new(Line::from(" No databases")).style(Style::new().dim());
        frame.render_widget(para, inner);
    }
}

fn render_results(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let block = Block::bordered()
        .title(" Results ")
        .border_style(theme.border_primary);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.query_blocks.is_empty() {
        let text = Text::from(Line::from(
            "  Welcome to kon! Type a SQL query and press Enter to execute.",
        ));
        let para = Paragraph::new(text)
            .style(Style::new().dim())
            .left_aligned();
        frame.render_widget(para, inner);
        return;
    }

    let total = app.query_blocks.len();
    let scroll_blocks = app.scroll.min(total.saturating_sub(1));
    let start = total.saturating_sub(1).saturating_sub(scroll_blocks);

    let mut y = inner.y;
    let bottom = inner.y + inner.height;

    for idx in start..total {
        if y >= bottom {
            break;
        }

        let qb = &app.query_blocks[idx];
        let focused = idx == app.active_block;

        let header = if focused {
            Line::from(vec![
                Span::styled("▶", Style::new().fg(theme.completion_kw)),
                Span::raw(" "),
                Span::styled(qb.sql.as_str(), theme.sql_focused),
            ])
        } else {
            Line::from(vec![
                Span::raw("  "),
                Span::styled(qb.sql.as_str(), theme.sql_unfocused),
            ])
        };
        frame.render_widget(
            Paragraph::new(header),
            Rect::new(inner.x, y, inner.width, 1),
        );
        y += 1;
        if y >= bottom {
            continue;
        }

        let remaining = bottom - y;

        if let Some(err) = &qb.error {
            let err_line = Line::from(vec![
                Span::styled("✖ ", theme.error),
                Span::styled(err.as_str(), theme.error),
            ]);
            frame.render_widget(
                Paragraph::new(err_line),
                Rect::new(inner.x, y, inner.width, 1),
            );
            y += 1;
        } else if let Some(result) = &qb.result {
            if result.columns.is_empty() {
                let summary = format!("{} row(s) affected", result.rows_affected);
                frame.render_widget(
                    Paragraph::new(Line::from(vec![
                        Span::styled("✔ ", theme.summary),
                        Span::styled(summary, theme.summary),
                    ])),
                    Rect::new(inner.x, y, inner.width, 1),
                );
                y += 1;
                if y < bottom {
                    frame.render_widget(
                        Paragraph::new(
                            Line::from(format!("  ({} ms)", result.execution_time_ms)).dim(),
                        ),
                        Rect::new(inner.x, y, inner.width, 1),
                    );
                    y += 1;
                }
            } else if matches!(qb.view_mode, ViewMode::Table) {
                let table_area = Rect::new(inner.x, y, inner.width, remaining);
                let row_offset = app.block_row_scroll.get(idx).copied().unwrap_or(0);
                table::render_table(
                    frame,
                    table_area,
                    result,
                    theme,
                    focused,
                    app.scroll_x as u16,
                    row_offset,
                );

                let visible_rows = (result.rows.len() as u16).min(remaining.saturating_sub(1));
                let summary_y = y + 1 + visible_rows;
                if summary_y < bottom {
                    let summary = format!(
                        "{} rows in set ({} ms)",
                        result.rows_affected, result.execution_time_ms
                    );
                    frame.render_widget(
                        Paragraph::new(Line::from(summary).style(theme.summary)),
                        Rect::new(inner.x, summary_y, inner.width, 1),
                    );
                    y = summary_y + 1;
                } else {
                    y = bottom;
                }
            } else {
                let row_offset = app.block_row_scroll.get(idx).copied().unwrap_or(0);
                let lines = vertical::render_vertical_lines(result, theme, row_offset);
                frame.render_widget(
                    Paragraph::new(Text::from(lines)).left_aligned(),
                    Rect::new(inner.x, y, inner.width, remaining),
                );
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
        Line::from(vec![
            Span::styled(" ● ", theme.error),
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
        std::iter::once(Span::raw(&prefix))
            .chain(input_line.spans.iter().cloned())
            .collect::<Vec<_>>(),
    );

    let inner_style = Style::new().bg(theme.input_bg).fg(theme.input_fg);
    let block = Block::bordered()
        .title(app.conn_name.as_str())
        .border_style(theme.border_primary);
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let para = Paragraph::new(Text::from(full_line))
        .style(inner_style)
        .left_aligned();
    frame.render_widget(para, inner);

    let cursor_x = inner.x + prefix.len() as u16 + app.cursor as u16;
    let cursor_y = inner.y;
    frame.set_cursor_position((cursor_x.min(inner.right().saturating_sub(1)), cursor_y));
}
